use crate::{
    machine_utils::{add_tape_mov_stay_fir, table_lookup},
    transitions::{self, SingleChar},
    StateMachine, StateMachineBuilder, TapeMovement,
};

use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Dfa {
    transition_table: Vec<u16>,
    accept_states: HashSet<u16>,
    states: u16,
    chars: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DfaBuilder {
    accept_states: HashSet<u16>,
    states: u16,
    chars: u16,
    building_layers: Vec<Option<u16>>,
}

impl DfaBuilder {
    pub fn new(base: Dfa) -> Self {
        let Dfa {
            transition_table,
            accept_states,
            states,
            chars,
        } = base;
        Self {
            accept_states,
            states,
            chars,
            building_layers: transition_table.into_iter().map(Some).collect(),
        }
    }

    fn swap_state(&mut self, first: u16, second: u16) {
        debug_assert!(first < self.states);
        debug_assert!(second < self.states);
        if first == second {
            return;
        }

        // Make first < second
        if first > second {
            return self.swap_state(second, first);
        }
        debug_assert!(first < second);

        // Indexes to the start of the tables for first and second
        let first_start = table_lookup(first as usize, 0, self.chars as usize);
        let second_start = table_lookup(second as usize, 0, self.chars as usize);

        // Swap the slices in the transition table
        let (first_extra, second_extra) = self.building_layers.split_at_mut(second_start);
        let first_table = &mut first_extra[first_start..first_start + self.chars as usize];
        let second_table = &mut second_extra[..self.chars as usize];
        first_table.swap_with_slice(second_table);

        for state in self.building_layers.iter_mut().flatten() {
            *state = match *state {
                state if state == first => second,
                state if state == second => first,
                other => other,
            }
        }

        let first_is_accept = self.accept_states.remove(&first);
        let second_is_accept = self.accept_states.remove(&second);

        if first_is_accept {
            self.accept_states.insert(second);
        }
        if second_is_accept {
            self.accept_states.insert(first);
        }
    }
}

impl StateMachineBuilder for DfaBuilder {
    type Machine = Dfa;
    type Trasition = SingleChar;

    //TDOD: figure out error type
    type Error = ();

    fn add_state(&mut self) -> u16 {
        self.building_layers
            .append(&mut vec![None; self.chars as usize].into());
        self.states += 1;
        debug_assert!(self.building_layers.len() == (self.chars as usize) * (self.states as usize));

        self.states - 1
    }

    fn remove_state(&mut self, state: u16) -> Result<Option<u16>, ()> {
        if state >= self.states || self.states == 1 {
            return Err(());
        }

        self.swap_state(state, self.states - 1);
        self.states -= 1;
        self.building_layers
            .drain((self.states * self.chars) as usize..);

        for t in self
            .building_layers
            .iter_mut()
            .filter(|&&mut transition| transition == Some(self.states))
        {
            *t = None;
        }

        self.accept_states.remove(&self.states);

        debug_assert!(self.building_layers.len() == (self.chars as usize) * (self.states as usize));
        if self.states == state {
            Ok(None)
        } else {
            Ok(Some(self.states))
        }
    }

    fn set_transition(&mut self, transition: SingleChar) -> Result<(), Self::Error> {
        let SingleChar { start, end, char } = transition;
        if start >= self.states || end >= self.states {
            return Err(());
        }

        self.building_layers[table_lookup(start as usize, self.chars as usize, char as usize)] =
            Some(end);

        Ok(())
    }

    fn set_start_state(&mut self, new_start_state: u16) -> Result<(), Self::Error> {
        if new_start_state < self.states {
            return Err(());
        }
        self.swap_state(0, new_start_state);
        Ok(())
    }

    fn add_char(&mut self) {
        let chunks = self.building_layers.chunks_exact(self.chars as usize);
        debug_assert!(chunks.remainder().len() == 0);
        debug_assert!(chunks.len() == self.states as usize);

        self.building_layers = chunks
            .map(|chunk| chunk.into_iter().chain([None].iter()))
            .flatten()
            .copied()
            .collect();
        self.chars += 1;
        debug_assert!(self.building_layers.len() == (self.chars * self.states) as usize);
    }

    fn remove_char(&mut self, char: u16) -> Result<(), ()> {
        if self.chars == 1 {
            return Err(());
        }
        if char >= self.chars {
            return Err(());
        }
        let chunks = self.building_layers.chunks_exact(self.chars as usize);
        debug_assert!(chunks.remainder().len() == 0);
        debug_assert!(chunks.len() == self.states as usize);

        self.building_layers = chunks
            .map(|chunk| {
                let (left, right) = chunk.split_at(char as usize);
                left.into_iter().chain(right.into_iter().skip(1))
            })
            .flatten()
            .copied()
            .collect();
        self.chars -= 1;
        debug_assert!(self.building_layers.len() == (self.chars * self.states) as usize);
        Ok(())
    }

    fn add_accept_state(&mut self, state: u16) -> Result<bool, Self::Error> {
        if state >= self.states {
            return Err(());
        }

        return Ok(self.accept_states.insert(state));
    }

    fn remove_accept_state(&mut self, state: u16) -> Result<bool, Self::Error> {
        if state >= self.states {
            return Err(());
        }

        return Ok(self.accept_states.remove(&state));
    }
}

impl From<Dfa> for DfaBuilder {
    fn from(value: Dfa) -> Self {
        DfaBuilder {
            accept_states: value.accept_states,
            states: value.states,
            chars: value.chars,
            building_layers: value.transition_table.into_iter().map(Some).collect(),
        }
    }
}

//TODO: create
impl TryFrom<DfaBuilder> for Dfa {
    type Error = ();

    fn try_from(value: DfaBuilder) -> Result<Self, Self::Error> {
        Ok(Dfa::build(
            dbg!(value
                .building_layers
                .into_iter()
                .collect::<Option<Vec<u16>>>()
                .ok_or(()))?,
            dbg!(value.accept_states),
            dbg!(value.states),
            dbg!(value.chars),
        )?)
    }
}

impl Dfa {
    pub fn build(
        transition_table: Vec<u16>,
        accept_states: HashSet<u16>,
        states: u16,
        chars: u16,
    ) -> Result<Dfa, ()> {
        if transition_table.len() != (states * chars) as usize {
            return Err(());
        }
        if transition_table
            .iter()
            .chain(accept_states.iter())
            .any(|&item| item >= states)
        {
            return Err(());
        }

        Ok(Dfa {
            transition_table,
            accept_states,
            states,
            chars,
        })
    }

    fn states<'a>(&'a self, input: &'a [u16]) -> impl Iterator<Item = u16> + 'a {
        Some(0).into_iter().chain(input.iter().scan(0, |state, &c| {
            *state = self.next_state(*state, c);
            Some(*state)
        }))
    }

    fn next_state(&self, cur_state: u16, cur_char: u16) -> u16 {
        self.transition_table
            [table_lookup(cur_state as usize, cur_char as usize, self.chars as usize)]
    }
}

impl StateMachine for Dfa {
    fn accepts_validated(&self, input: &[u16]) -> bool {
        self.accept_states.contains(
            &self
                .states(input)
                .last()
                .expect("The first state will always be visited no matter the input"),
        )
    }

    fn trace_states_validated(&self, input: &[u16]) -> Vec<(u16, Vec<TapeMovement>)> {
        add_tape_mov_stay_fir(self.states(input).collect(), TapeMovement::Right(None))
    }

    fn states(&self) -> u16 {
        self.states
    }

    fn chars(&self) -> u16 {
        self.chars
    }
}

#[cfg(test)]
mod dfa_tests {
    mod dfa_builder_tests;
    mod dfa_machine_tests;
}
