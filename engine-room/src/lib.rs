use dfa::Dfa;
use e_nfa::EpsilonNfa;
use machine_utils::validate_input;
use nfa::Nfa;

pub mod dfa;
pub mod e_nfa;
pub mod multi_tm;
pub mod nfa;
pub mod pda;
pub mod stay_tm;
pub mod tm;

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
        validate_input(input, self.max_input())?;
        Ok(self.accepts_validated(input))
    }
    /// Checks if the state machine accepts the given input.
    fn accepts_validated(&self, input: &[u16]) -> bool;

    /// Traces the states and movements of the state machine for the given input.
    fn trace_states(&self, input: &[u16]) -> Result<Vec<(u16, Vec<TapeMovement>)>, ()> {
        validate_input(input, self.max_input())?;
        Ok(self.trace_states_validated(input))
    }
    /// Traces the states and movements of the state machine for the given input.
    fn trace_states_validated(&self, input: &[u16]) -> Vec<(u16, Vec<TapeMovement>)>;

    /// Returns the largest state in this machine
    fn max_state(&self) -> u16;

    /// Returns the largest input in this machine
    fn max_input(&self) -> u16;
}

#[derive(Debug, Clone)]
pub enum Machine {
    Dfa(Dfa),
    Nfa(Nfa),
    EpsilonNfa(EpsilonNfa),
}

impl StateMachine for Machine {
    fn accepts_validated(&self, input: &[u16]) -> bool {
        match self {
            Machine::Dfa(dfa) => dfa.accepts_validated(input),
            Machine::Nfa(nfa) => nfa.accepts_validated(input),
            Machine::EpsilonNfa(enfa) => enfa.accepts_validated(input),
        }
    }

    fn trace_states_validated(&self, input: &[u16]) -> Vec<(u16, Vec<TapeMovement>)> {
        match self {
            Machine::Dfa(dfa) => dfa.trace_states_validated(input),
            Machine::Nfa(nfa) => nfa.trace_states_validated(input),
            Machine::EpsilonNfa(enfa) => enfa.trace_states_validated(input),
        }
    }

    fn max_state(&self) -> u16 {
        match self {
            Machine::Dfa(dfa) => dfa.max_state(),
            Machine::Nfa(nfa) => nfa.max_state(),
            Machine::EpsilonNfa(enfa) => enfa.max_state(),
        }
    }

    fn max_input(&self) -> u16 {
        match self {
            Machine::Dfa(dfa) => dfa.max_input(),
            Machine::Nfa(nfa) => nfa.max_input(),
            Machine::EpsilonNfa(enfa) => enfa.max_input(),
        }
    }
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
