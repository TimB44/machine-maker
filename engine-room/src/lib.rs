use machine_utils::validate_input;

pub mod dfa;
pub mod e_nfa;
pub mod multi_tm;
pub mod nfa;
pub mod pda;
pub mod stay_tm;
pub mod tm;
pub mod transitions;

mod machine_utils;

/// # State Machine
///
/// This trait defines the essential methods for a state machine. A state machine
/// can determine if it accepts a given input and can also trace the states it
/// passes through during the input processing.
///
/// For DFA and NFA, the `state_trace` method will always return movements
/// of `Right(None)` as these state machines only read the tape once without
/// modifying it.
///
pub trait StateMachine {
    /// Checks if the state machine accepts the given input.
    fn accepts(&self, input: &[u16]) -> Result<bool, ()> {
        validate_input(input, self.chars())?;
        Ok(self.accepts_validated(input))
    }
    /// Checks if the state machine accepts the given input.
    fn accepts_validated(&self, input: &[u16]) -> bool;

    /// Traces the states and movements of the state machine for the given input.
    fn trace_states(&self, input: &[u16]) -> Result<Vec<(u16, Vec<TapeMovement>)>, ()> {
        validate_input(input, self.chars())?;
        Ok(self.trace_states_validated(input))
    }
    /// Traces the states and movements of the state machine for the given input.
    fn trace_states_validated(&self, input: &[u16]) -> Vec<(u16, Vec<TapeMovement>)>;

    /// Returns the largest state in this machine
    fn states(&self) -> u16;

    /// Returns the largest input in this machine
    fn chars(&self) -> u16;
}

pub trait StateMachineBuilder
where
    Self: From<Self::Machine>,
    Self::Machine: TryFrom<Self>,
{
    type Trasition;
    type Machine;
    type Error;

    fn add_state(&mut self) -> u16;
    fn remove_state(&mut self, state: u16) -> Result<Option<u16>, Self::Error>;

    fn set_transition(&mut self, transition: Self::Trasition) -> Result<(), Self::Error>;

    fn set_start_state(&mut self, new_start_state: u16) -> Result<(), Self::Error>;

    fn add_accept_state(&mut self, state: u16) -> Result<bool, Self::Error>;
    fn remove_accept_state(&mut self, state: u16) -> Result<bool, Self::Error>;

    fn add_char(&mut self);
    fn remove_char(&mut self, char: u16) -> Result<Self::Error, ()>;
}

/// # Tape Movement
///
/// This enum defines the possible movements of a tape in a general state machine.
/// The tape can move left, right, or stay in place. Additionally, with each
/// movement, an optional symbol can be written to the tape. If no symbol is
/// provided, the tape position will be updated without writing a symbol.
///
/// For DFA and NFA, the movement will always be `Right(None)` as these
/// state machines only interact with the tape by reading it once.
///
/// # Examples
///
/// ```
/// use engine_room::TapeMovement;
///
/// // Move the tape to the right and write the symbol 1
/// let move_right = TapeMovement::Right(Some(1));
///
/// // Move the tape to the left without writing any symbol
/// let move_left = TapeMovement::Left(None);
///
/// // Keep the tape in the current position and write the symbol 0
/// let stay_put = TapeMovement::Stay(Some(0));
///
/// // Move the tape to the right without writing any symbol
/// let move_right_no_write = TapeMovement::Right(None);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TapeMovement {
    Right(Option<u16>),
    Left(Option<u16>),
    Stay(Option<u16>),
}

pub trait LogicalEdge {
    fn to_string(&self) -> String;
    fn input_count() -> usize;
    fn input_labels() -> &'static Vec<&'static str>;
}
