use crate::{
    machine_utils::{add_tape_mov_stay_fir, table_lookup},
    transitions::SingleChar,
    StateMachine, StateMachineBuilder, TapeMovement,
};
use std::{cmp::max, collections::HashSet};

#[derive(Debug, Clone)]
pub struct Nfa {
    transition_table: Vec<HashSet<u16>>,
    accept_states: HashSet<u16>,
    //TODO:
    states: u16,
    chars: u16,
}

//TODO: create
#[derive(Debug, Clone)]
pub struct NfaBuilder {
    transition_table: Vec<HashSet<u16>>,
    accept_states: HashSet<u16>,
    max_state: u16,
    max_char: u16,
}

impl StateMachineBuilder for NfaBuilder {
    type Trasition = SingleChar;

    type Machine = Nfa;

    type Error = ();

    fn add_state(&mut self) -> u16 {
        todo!()
    }

    fn remove_state(&mut self, state: u16) -> Result<Option<u16>, Self::Error> {
        todo!()
    }

    fn set_transition(&mut self, transition: Self::Trasition) -> Result<(), Self::Error> {
        todo!()
    }

    fn set_start_state(&mut self, new_start_state: u16) -> Result<(), Self::Error> {
        todo!()
    }

    fn add_accept_state(&mut self, state: u16) -> Result<bool, Self::Error> {
        todo!()
    }

    fn remove_accept_state(&mut self, state: u16) -> Result<bool, Self::Error> {
        todo!()
    }

    fn add_char(&mut self) {
        todo!()
    }

    fn remove_char(&mut self, char: u16) -> Result<Option<u16>, Self::Error> {
        todo!()
    }
}

//TODO:
impl From<Nfa> for NfaBuilder {
    fn from(value: Nfa) -> Self {
        todo!()
    }
}
impl TryFrom<NfaBuilder> for Nfa {
    type Error = ();

    fn try_from(value: NfaBuilder) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl Nfa {
    pub fn build(
        transition_table: Vec<HashSet<u16>>,
        accept_states: HashSet<u16>,
        states: u16,
        chars: u16,
    ) -> Result<Nfa, ()> {
        if states == 0 || chars == 0 {
            return Err(());
        }
        if transition_table.len() != (states * chars) as usize {
            return Err(());
        }

        if transition_table
            .iter()
            .flatten()
            .chain(accept_states.iter())
            .any(|&item| item >= states)
        {
            return Err(());
        }

        Ok(Nfa {
            transition_table,
            accept_states,
            states,
            chars,
        })
    }

    fn search_for_state_path(
        &self,
        cannot_accept: &mut Vec<bool>,
        input: &[u16],
        state_trace: &mut Vec<u16>,
        max_state_trace_len: &mut usize,
        target_len: Option<usize>,
    ) -> bool {
        let cur_char_index = state_trace.len() - 1;
        let cur_state = *state_trace.last().expect(
            "state_trace should not be empty as it should always contain 0, the start state",
        );
        let accept_table_index = table_lookup(cur_state as usize, cur_char_index, input.len() + 1);
        debug_assert_eq!(
            cannot_accept.len(),
            (input.len() + 1) * self.states as usize
        );
        debug_assert!(cur_char_index <= input.len());
        debug_assert!(target_len.unwrap_or(0) <= input.len() + 1);

        // If we have been in this combinations of state and current character then we know that we
        // will not be able to succeed by continuing to search from here
        if cannot_accept[accept_table_index] {
            return false;
        }

        // Here we search for a win return condition which depends on what are target length is. If
        // is None that means we will on stop searching once we find a path which accpets the
        // input. If it is Some we will stop once our then is the given length
        match target_len {
            Some(n) => {
                if n == cur_char_index + 1 {
                    println!("1");
                    return true;
                }
            }
            None => {
                if cur_char_index == input.len() {
                    let last_state_is_accpet_state = self.accept_states.contains(&cur_state);
                    cannot_accept[accept_table_index] = last_state_is_accpet_state;
                    return last_state_is_accpet_state;
                }
            }
        }

        let transition_table_index = table_lookup(
            cur_state as usize,
            input[cur_char_index] as usize,
            self.chars as usize,
        );
        for next_state in self.transition_table[transition_table_index]
            .iter()
            .copied()
        {
            state_trace.push(next_state);
            *max_state_trace_len = max(*max_state_trace_len, state_trace.len());
            if self.search_for_state_path(
                cannot_accept,
                input,
                state_trace,
                max_state_trace_len,
                target_len,
            ) {
                return true;
            }
            state_trace.pop();
        }

        cannot_accept[accept_table_index] = true;
        false
    }

    pub fn accept_states(&self) -> &HashSet<u16> {
        &self.accept_states
    }

    pub fn transition_table(&self) -> &[HashSet<u16>] {
        &self.transition_table
    }
}

impl StateMachine for Nfa {
    fn accepts_validated(&self, input: &[u16]) -> bool {
        let mut cur_states = HashSet::from([0]);
        let mut next_states = HashSet::new();

        for c in input {
            for state in cur_states {
                next_states.extend(
                    &self.transition_table
                        [table_lookup(state as usize, *c as usize, self.chars as usize)],
                )
            }
            cur_states = next_states;
            next_states = HashSet::new()
        }
        return cur_states.iter().any(|s| self.accept_states.contains(s));
    }

    fn trace_states_validated(&self, input: &[u16]) -> Vec<(u16, Vec<TapeMovement>)> {
        if input.len() == 0 {
            return vec![(0, vec![TapeMovement::Stay(None)])];
        }
        let mut max_len = 0;
        let mut state_trace = vec![0];

        let mut dp = vec![false; (input.len() + 1) * self.states as usize];
        if !self.search_for_state_path(&mut dp, input, &mut state_trace, &mut max_len, None) {
            dp.fill(false);
            debug_assert_eq!(state_trace, vec![0]);
            debug_assert!(self.search_for_state_path(
                &mut dp,
                input,
                &mut state_trace,
                &mut 1,
                Some(max_len)
            ))
        }

        let mut trace = add_tape_mov_stay_fir(state_trace, TapeMovement::Right(None));
        trace[0].1[0] = TapeMovement::Stay(None);
        trace
    }

    fn states(&self) -> u16 {
        self.states
    }

    fn chars(&self) -> u16 {
        self.chars
    }
}

#[cfg(test)]
mod nfa_tests {

    use crate::machine_utils::add_tape_mov_stay_fir;
    use crate::{StateMachine, TapeMovement};

    use super::Nfa;
    use std::collections::HashSet;

    #[test]
    fn build_nfa_small() {
        assert!(Nfa::build(vec![HashSet::new()], HashSet::new(), 1, 1).is_ok());
    }

    #[test]
    fn build_nfa_two_states() {
        assert!(Nfa::build(vec![HashSet::new(), HashSet::new()], HashSet::new(), 2, 1).is_ok());
        assert!(Nfa::build(vec![HashSet::new(), HashSet::new()], HashSet::new(), 1, 2).is_ok());
    }

    #[test]
    fn build_nfa_medium() {
        assert!(Nfa::build(
            vec![
                HashSet::from([1]),
                HashSet::from([1]),
                HashSet::from([2]),
                HashSet::from([2]),
                HashSet::from([3]),
                HashSet::from([3]),
                HashSet::from([0]),
                HashSet::from([0])
            ],
            HashSet::from([3]),
            4,
            2
        )
        .is_ok());
    }

    #[test]
    fn build_nfa_large() {
        assert!(Nfa::build(
            (0..100).map(|n| HashSet::from([(n + 1) % 10])).collect(),
            HashSet::from([3]),
            10,
            10
        )
        .is_ok());
    }
    #[test]
    fn build_nfa_small_wrong_dims() {
        assert!(Nfa::build(vec![HashSet::new()], HashSet::new(), 1, 2).is_err());
        assert!(Nfa::build(vec![HashSet::new()], HashSet::new(), 1, 3).is_err());
        assert!(Nfa::build(vec![HashSet::new()], HashSet::new(), 2, 1).is_err());
        assert!(Nfa::build(vec![HashSet::new()], HashSet::new(), 2, 2).is_err());
        assert!(Nfa::build(vec![HashSet::new()], HashSet::new(), 3, 1).is_err());

        assert!(Nfa::build(vec![], HashSet::new(), 1, 1).is_err());
        assert!(Nfa::build(vec![HashSet::new(), HashSet::new()], HashSet::new(), 1, 1).is_err());
        assert!(Nfa::build(
            vec![
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
                HashSet::new()
            ],
            HashSet::new(),
            1,
            1
        )
        .is_err());
    }

    #[test]
    fn build_correct_nfa_two_states() {
        assert!(Nfa::build(vec![HashSet::new(), HashSet::new()], HashSet::new(), 2, 1).is_ok());
        assert!(Nfa::build(vec![HashSet::new(), HashSet::new()], HashSet::new(), 1, 2).is_ok());
    }

    #[test]
    fn build_nfa_medium_wrong_dims() {
        assert!(Nfa::build(
            vec![
                HashSet::from([1]),
                HashSet::from([1]),
                HashSet::from([2]),
                HashSet::from([2]),
                HashSet::from([2]),
                HashSet::from([2]),
                HashSet::from([0]),
                HashSet::from([0])
            ],
            HashSet::from([3]),
            3,
            2
        )
        .is_err());

        assert!(Nfa::build(
            vec![
                HashSet::from([1]),
                HashSet::from([1]),
                HashSet::from([2]),
                HashSet::from([2]),
                HashSet::from([3]),
                HashSet::from([3]),
                HashSet::from([0]),
                HashSet::from([0])
            ],
            HashSet::from([3]),
            4,
            3
        )
        .is_err());

        assert!(Nfa::build(
            vec![
                HashSet::from([1]),
                HashSet::from([1]),
                HashSet::from([2]),
                HashSet::from([2]),
                HashSet::from([3]),
                HashSet::from([3]),
                HashSet::from([0]),
                HashSet::from([0]),
                HashSet::from([0])
            ],
            HashSet::from([3]),
            4,
            2
        )
        .is_err());

        assert!(Nfa::build(
            vec![
                HashSet::from([1]),
                HashSet::from([1]),
                HashSet::from([2]),
                HashSet::from([2]),
                HashSet::from([3]),
                HashSet::from([3]),
                HashSet::from([0]),
            ],
            HashSet::from([3]),
            4,
            2
        )
        .is_err());
    }

    #[test]
    fn build_nfa_large_wrong_dims() {
        assert!(Nfa::build(
            (0..100).map(|n| HashSet::from([(n + 1) % 10])).collect(),
            HashSet::from([3]),
            9,
            10
        )
        .is_err());

        assert!(Nfa::build(
            (0..100).map(|n| HashSet::from([(n + 1) % 10])).collect(),
            HashSet::from([3]),
            10,
            9
        )
        .is_err());

        assert!(Nfa::build(
            (0..100).map(|n| HashSet::from([(n + 1) % 10])).collect(),
            HashSet::from([3]),
            9,
            9
        )
        .is_err());

        assert!(Nfa::build(
            (0..100).map(|n| HashSet::from([(n + 1) % 10])).collect(),
            HashSet::from([3]),
            11,
            10
        )
        .is_err());

        assert!(Nfa::build(
            (0..100).map(|n| HashSet::from([(n + 1) % 10])).collect(),
            HashSet::from([3]),
            10,
            11
        )
        .is_err());
    }

    #[test]
    fn states_in_transition_table_to_high_fails() {
        assert!(Nfa::build(vec![HashSet::from([0])], HashSet::new(), 1, 1).is_ok());
        assert!(Nfa::build(vec![HashSet::from([1])], HashSet::new(), 1, 1).is_err());
        assert!(Nfa::build(
            vec![HashSet::from([1]), HashSet::from([1])],
            HashSet::new(),
            1,
            2
        )
        .is_err());

        assert!(Nfa::build(
            vec![
                HashSet::from([0]),
                HashSet::from([1]),
                HashSet::from([2]),
                HashSet::from([3])
            ],
            HashSet::new(),
            4,
            1
        )
        .is_ok());

        assert!(Nfa::build(
            vec![
                HashSet::from([1]),
                HashSet::from([2]),
                HashSet::from([3]),
                HashSet::from([4])
            ],
            HashSet::new(),
            4,
            1
        )
        .is_err());

        assert!(Nfa::build(
            vec![
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
                HashSet::from([1234])
            ],
            HashSet::new(),
            4,
            1
        )
        .is_err());
    }

    #[test]
    fn states_in_accept_set_to_high_fails() {
        assert!(Nfa::build(
            vec![
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
                HashSet::new()
            ],
            HashSet::from([1]),
            2,
            2
        )
        .is_ok());

        assert!(Nfa::build(
            vec![
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
                HashSet::new()
            ],
            HashSet::from([2]),
            2,
            2
        )
        .is_err());

        assert!(Nfa::build(
            (0..100).map(|n| HashSet::from([(n + 1) % 10])).collect(),
            HashSet::from([9]),
            10,
            10
        )
        .is_ok());

        assert!(Nfa::build(
            (0..100).map(|n| HashSet::from([(n + 1) % 10])).collect(),
            HashSet::from([9, 10]),
            10,
            10
        )
        .is_err());

        assert!(Nfa::build(
            (0..100).map(|n| HashSet::from([(n + 1) % 10])).collect(),
            HashSet::from([11]),
            10,
            10
        )
        .is_err());

        assert!(Nfa::build(
            (0..100).map(|n| HashSet::from([(n + 1) % 10])).collect(),
            HashSet::from_iter(100..1000),
            10,
            10
        )
        .is_err());

        assert!(Nfa::build(
            (0..100).map(|n| HashSet::from([(n + 1) % 10])).collect(),
            HashSet::from_iter(0..20),
            20,
            5
        )
        .is_ok());

        assert!(Nfa::build(
            (0..100).map(|n| HashSet::from([(n + 1) % 10])).collect(),
            HashSet::from([20]),
            20,
            5
        )
        .is_err());

        assert!(Nfa::build(
            (0..100).map(|n| HashSet::from([(n + 1) % 10])).collect(),
            HashSet::from_iter((0..20).map(|n| n * 7)),
            20,
            5
        )
        .is_err());
    }

    #[test]
    fn to_large_chars_errors() {
        let nfa = Nfa::build(
            (0..100).map(|n| HashSet::from([(n + 1) % 10])).collect(),
            HashSet::from([9]),
            10,
            10,
        )
        .unwrap();

        assert!(nfa.accepts(&[1, 2, 3, 4, 5, 6, 7, 8, 9]).is_ok());
        assert!(nfa.accepts(&[10]).is_err());
        assert!(nfa.accepts(&[1, 2, 3, 4, 10]).is_err());
        assert!(nfa.accepts(&[124, 1523, 325, 123]).is_err());
    }

    #[test]
    fn ends_in_one_accepts() {
        let nfa = Nfa::build(
            vec![
                HashSet::from([0]),
                HashSet::from([0, 1]),
                HashSet::new(),
                HashSet::new(),
            ],
            HashSet::from([1]),
            2,
            2,
        )
        .unwrap();

        assert!(nfa.accepts(&[0, 0, 0, 0, 1]).unwrap());
        assert!(nfa.accepts(&[1]).unwrap());
        assert!(nfa.accepts(&[[0].repeat(1000), vec![1]].concat()).unwrap());

        assert!(!nfa.accepts(&[0, 1, 0]).unwrap());
        assert!(!nfa.accepts(&[0, 0]).unwrap());
        assert!(!nfa.accepts(&[1, 1, 1, 1, 1, 0]).unwrap());
        assert!(!nfa.accepts(&[[1].repeat(1000), vec![0]].concat()).unwrap());
    }
    #[test]
    fn valid_input_empty() {
        let nfa = Nfa::build(
            vec![
                HashSet::from([1]),
                HashSet::from([1]),
                HashSet::from([1]),
                HashSet::from([1]),
            ],
            HashSet::from([0]),
            2,
            2,
        )
        .unwrap();
        assert!(nfa.accepts(&[]).is_ok());
        assert_eq!(
            nfa.trace_states(&[]).unwrap(),
            vec![(0, vec![TapeMovement::Stay(None)])]
        );
    }

    #[test]
    fn ends_in_one_accepts_path() {
        let nfa = Nfa::build(
            vec![
                HashSet::from([0]),
                HashSet::from([0, 1]),
                HashSet::new(),
                HashSet::new(),
            ],
            HashSet::from([1]),
            2,
            2,
        )
        .unwrap();

        assert_eq!(
            nfa.trace_states(&[1, 1, 0]).unwrap(),
            add_tape_mov_stay_fir(vec![0, 0, 0, 0], TapeMovement::Right(None))
        );
        assert_eq!(
            nfa.trace_states(&[0, 0, 0, 0, 1]).unwrap(),
            add_tape_mov_stay_fir(vec![0, 0, 0, 0, 0, 1], TapeMovement::Right(None))
        );
        assert_eq!(
            nfa.trace_states(&[1]).unwrap(),
            add_tape_mov_stay_fir(vec![0, 1], TapeMovement::Right(None))
        );
        assert_eq!(
            nfa.trace_states(&[[0].repeat(1000), vec![1]].concat())
                .unwrap(),
            add_tape_mov_stay_fir(
                [[0].repeat(1001), vec![1]].concat(),
                TapeMovement::Right(None)
            )
        );

        assert_eq!(
            nfa.trace_states(&[0, 1, 0]).unwrap(),
            add_tape_mov_stay_fir(vec![0, 0, 0, 0], TapeMovement::Right(None))
        );
        assert_eq!(
            nfa.trace_states(&[0, 0]).unwrap(),
            add_tape_mov_stay_fir(vec![0, 0, 0], TapeMovement::Right(None))
        );
        assert_eq!(
            nfa.trace_states(&[1, 1, 1, 1, 1, 0]).unwrap(),
            add_tape_mov_stay_fir(vec![0, 0, 0, 0, 0, 0, 0], TapeMovement::Right(None))
        );
        assert_eq!(
            nfa.trace_states(&[[1].repeat(1000), vec![0]].concat())
                .unwrap(),
            add_tape_mov_stay_fir([0].repeat(1002), TapeMovement::Right(None))
        );
        assert_eq!(
            nfa.trace_states(&[0, 1, 1]).unwrap(),
            add_tape_mov_stay_fir(vec![0, 0, 0, 1], TapeMovement::Right(None))
        );
    }

    // Test NFA which accepts 0*(1* ∪ 2*)01
    #[test]
    fn accepts_0_star_1_star_or_star_2_star_concat_01() {
        let nfa = Nfa::build(
            vec![
                // State 0
                HashSet::from([0, 3]),
                HashSet::from([1]),
                HashSet::from([2]),
                // State 1
                HashSet::from([3]),
                HashSet::from([1]),
                HashSet::new(),
                // State 2
                HashSet::from([3]),
                HashSet::new(),
                HashSet::from([2]),
                // State 3
                HashSet::new(),
                HashSet::from([4]),
                HashSet::new(),
                // State 4
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
            ],
            HashSet::from([4]),
            5,
            3,
        )
        .unwrap();

        // Test cases for `accepts`
        assert!(nfa.accepts(&[0, 0, 0, 1, 0, 1]).unwrap());
        assert_eq!(
            nfa.trace_states(&[0, 0, 0, 1, 0, 1]).unwrap(),
            add_tape_mov_stay_fir(vec![0, 0, 0, 0, 1, 3, 4], TapeMovement::Right(None))
        );

        assert!(nfa.accepts(&[0, 0, 2, 0, 1]).unwrap());
        assert_eq!(
            nfa.trace_states(&[0, 0, 2, 0, 1]).unwrap(),
            add_tape_mov_stay_fir(vec![0, 0, 0, 2, 3, 4], TapeMovement::Right(None))
        );

        assert!(nfa.accepts(&[1, 0, 1]).unwrap());
        assert_eq!(
            nfa.trace_states(&[1, 0, 1]).unwrap(),
            add_tape_mov_stay_fir(vec![0, 1, 3, 4], TapeMovement::Right(None))
        );

        assert!(nfa.accepts(&[2, 0, 1]).unwrap());
        assert_eq!(
            nfa.trace_states(&[2, 0, 1]).unwrap(),
            add_tape_mov_stay_fir(vec![0, 2, 3, 4], TapeMovement::Right(None))
        );

        assert!(nfa.accepts(&[0, 2, 0, 1]).unwrap());
        assert_eq!(
            nfa.trace_states(&[0, 2, 0, 1]).unwrap(),
            add_tape_mov_stay_fir(vec![0, 0, 2, 3, 4], TapeMovement::Right(None))
        );
        assert!(nfa.accepts(&[0, 0, 0, 2, 2, 0, 1]).unwrap());
        assert_eq!(
            nfa.trace_states(&[0, 0, 0, 2, 2, 0, 1]).unwrap(),
            add_tape_mov_stay_fir(vec![0, 0, 0, 0, 2, 2, 3, 4], TapeMovement::Right(None))
        );

        assert!(nfa.accepts(&[0, 1, 1, 0, 1]).unwrap());
        assert_eq!(
            nfa.trace_states(&[0, 1, 1, 0, 1]).unwrap(),
            add_tape_mov_stay_fir(vec![0, 0, 1, 1, 3, 4], TapeMovement::Right(None))
        );

        assert!(nfa.accepts(&[0, 0, 0, 1, 1, 0, 1]).unwrap());
        assert_eq!(
            nfa.trace_states(&[0, 0, 0, 1, 1, 0, 1]).unwrap(),
            add_tape_mov_stay_fir(vec![0, 0, 0, 0, 1, 1, 3, 4], TapeMovement::Right(None))
        );
    }

    #[test]
    fn state_trace_takes_longer_path() {
        // This nfa accepts (012 ∪ 0123)
        let nfa = Nfa::build(
            vec![
                //State 0: Start
                HashSet::from([1, 4]),
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
                // State 1: 0
                HashSet::new(),
                HashSet::from([2]),
                HashSet::new(),
                HashSet::new(),
                //State 2: 01
                HashSet::new(),
                HashSet::new(),
                HashSet::from([3]),
                HashSet::new(),
                //State 3: 012 accepet
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
                //State 4: 0
                HashSet::new(),
                HashSet::from([5]),
                HashSet::new(),
                HashSet::new(),
                //State 5: 01
                HashSet::new(),
                HashSet::new(),
                HashSet::from([6]),
                HashSet::new(),
                //State 6: 012
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
                HashSet::from([7]),
                //State 7: 0123 accept
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
            ],
            HashSet::from([3, 7]),
            8,
            4,
        )
        .unwrap();
        assert!(nfa.accepts(&[0, 1, 2]).unwrap());
        assert_eq!(
            nfa.trace_states(&[0, 1, 2]).unwrap(),
            add_tape_mov_stay_fir(vec![0, 1, 2, 3], TapeMovement::Right(None))
        );
        assert!(nfa.accepts(&[0, 1, 2, 3]).unwrap());
        assert_eq!(
            nfa.trace_states(&[0, 1, 2, 3]).unwrap(),
            add_tape_mov_stay_fir(vec![0, 4, 5, 6, 7], TapeMovement::Right(None))
        );
        assert!(!nfa.accepts(&[0, 1, 2, 3, 3]).unwrap());
        assert_eq!(
            nfa.trace_states(&[0, 1, 2, 3, 3]).unwrap(),
            add_tape_mov_stay_fir(vec![0, 4, 5, 6, 7], TapeMovement::Right(None))
        );
    }

    #[test]
    // Accepts (0* U 1*)(0* U 1*)
    fn accepts_regex_union_contacat_star() {
        let table = vec![
            // State 0:
            HashSet::from([1, 3]),
            HashSet::from([2, 4]),
            // State 1: 0*
            HashSet::from([1, 3]),
            HashSet::from([4]),
            // State 2: 1*
            HashSet::from([3]),
            HashSet::from([2, 4]),
            // State 3: 0*
            HashSet::from([3]),
            HashSet::from([]),
            // State 4: 1*
            HashSet::from([]),
            HashSet::from([4]),
        ];

        let nfa = Nfa::build(table, HashSet::from([0, 3, 4]), 5, 2).unwrap();
        assert!(nfa.accepts(&[]).unwrap());
        assert_eq!(
            nfa.trace_states(&[]).unwrap(),
            vec![(0, vec![TapeMovement::Stay(None)])]
        );
        assert!(nfa.accepts(&[0]).unwrap());
        assert_eq!(
            nfa.trace_states(&[0]).unwrap(),
            add_tape_mov_stay_fir(vec![0, 3], TapeMovement::Right(None))
        );
        assert!(nfa.accepts(&[1]).unwrap());
        assert_eq!(
            nfa.trace_states(&[1]).unwrap(),
            add_tape_mov_stay_fir(vec![0, 4], TapeMovement::Right(None))
        );

        assert!(nfa.accepts(&[0, 1]).unwrap());
        assert_eq!(
            nfa.trace_states(&[0, 1]).unwrap(),
            add_tape_mov_stay_fir(vec![0, 1, 4], TapeMovement::Right(None))
        );
        assert!(nfa.accepts(&[1, 0]).unwrap());
        assert_eq!(
            nfa.trace_states(&[1, 0]).unwrap(),
            add_tape_mov_stay_fir(vec![0, 2, 3], TapeMovement::Right(None))
        );

        assert!(!nfa.accepts(&[0, 1, 0]).unwrap());
        assert_eq!(
            nfa.trace_states(&[0, 1, 0]).unwrap(),
            add_tape_mov_stay_fir(vec![0, 1, 4], TapeMovement::Right(None))
        );

        assert!(!nfa.accepts(&[1, 0, 1]).unwrap());
        assert_eq!(
            nfa.trace_states(&[1, 0, 1]).unwrap(),
            add_tape_mov_stay_fir(vec![0, 2, 3], TapeMovement::Right(None))
        );
    }
}
