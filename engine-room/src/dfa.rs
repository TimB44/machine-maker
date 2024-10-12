use crate::{
    machine_utils::{add_tape_mov_stay_fir, table_lookup},
    BuildError, StateMachine, TapeMovement,
};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Dfa {
    transition_table: Vec<u16>,
    accept_states: HashSet<u16>,
    max_state: u16,
    max_char: u16,
}
//TODO: create
#[derive(Debug, Clone)]
pub struct DfaBuilder {}

//TODO: create
impl From<Dfa> for DfaBuilder {
    fn from(value: Dfa) -> Self {
        todo!()
    }
}

//TODO: create
impl TryInto<Dfa> for DfaBuilder {
    type Error = BuildError;

    fn try_into(self) -> Result<Dfa, Self::Error> {
        todo!()
    }
}

impl Dfa {
    pub fn build(
        transition_table: Vec<u16>,
        accept_states: HashSet<u16>,
        max_state: u16,
        max_char: u16,
    ) -> Result<Dfa, ()> {
        if transition_table.len() != ((max_state + 1) * (max_char + 1)) as usize {
            return Err(());
        }
        if transition_table
            .iter()
            .chain(accept_states.iter())
            .any(|item| item > &max_state)
        {
            return Err(());
        }

        Ok(Dfa {
            transition_table,
            accept_states,
            max_state,
            max_char,
        })
    }

    fn states<'a>(&'a self, input: &'a [u16]) -> impl Iterator<Item = u16> + 'a {
        let mut state = 0;
        Some(0).into_iter().chain(input.iter().map(move |c| {
            state = self.next_state(state, *c);
            state
        }))
    }

    fn next_state(&self, cur_state: u16, cur_char: u16) -> u16 {
        self.transition_table[table_lookup(
            cur_state as usize,
            cur_char as usize,
            self.max_char as usize,
        )]
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

    fn max_state(&self) -> u16 {
        self.max_state
    }

    fn max_input(&self) -> u16 {
        self.max_char
    }
}

#[cfg(test)]
mod dfa_tests {

    use crate::{dfa::Dfa, machine_utils::add_tape_mov_stay_fir, StateMachine, TapeMovement};
    use std::collections::HashSet;

    #[test]
    fn wrong_length_errors() {
        let tf_long = vec![0, 1, 1, 0, 1];
        let tf_correct = vec![0, 1, 1, 0];
        let tf_short = vec![0, 1, 1];
        assert!(Dfa::build(tf_long, HashSet::from([1]), 1, 1).is_err());
        assert!(Dfa::build(tf_short, HashSet::from([1]), 1, 1).is_err());
        assert!(Dfa::build(tf_correct, HashSet::from([1]), 1, 1).is_ok());
    }

    #[test]
    fn wrong_length_uneven_errors() {
        let tf_long = vec![0; 11];
        let tf_short = vec![0; 8];
        let tf_correct = vec![0; 10];
        assert!(Dfa::build(tf_long, HashSet::from([1]), 1, 4).is_err());
        assert!(Dfa::build(tf_short, HashSet::from([1]), 1, 4).is_err());
        assert!(Dfa::build(tf_correct, HashSet::from([1]), 1, 4).is_ok());
    }

    #[test]
    fn invalid_accept_states() {
        let tf = vec![0; 20];
        assert!(Dfa::build(tf.clone(), HashSet::from([5]), 4, 3).is_err());
        assert!(Dfa::build(tf.clone(), HashSet::from([4, 5]), 4, 3).is_err());
        assert!(Dfa::build(tf.clone(), HashSet::from([500]), 4, 3).is_err());
        assert!(Dfa::build(tf.clone(), HashSet::from([0, 1, 2, 3, 4]), 4, 3).is_ok());
    }

    #[test]
    fn to_large_state_errors() {
        let tf_correct = vec![0, 1, 2, 2, 1, 1];
        let tf_one_to_large = vec![0, 1, 2, 3, 1, 1];
        let tf_way_to_large = vec![10, 734, 234, 12, 93, 1523];

        assert!(Dfa::build(tf_correct, HashSet::from([0]), 2, 1).is_ok());
        assert!(Dfa::build(tf_one_to_large, HashSet::from([0]), 2, 1).is_err());
        assert!(Dfa::build(tf_way_to_large, HashSet::from([0]), 2, 1).is_err());
    }

    #[test]
    fn invalid_input() {
        let correct = vec![0, 1, 1, 0];
        let small = vec![0, 1, 2, 1, 2, 1];
        let large = vec![5325, 2325, 23564, 3252, 312];
        let dfa = Dfa::build(vec![1, 0, 1, 0], HashSet::from([0]), 1, 1).unwrap();

        assert!(dfa.accepts(&correct).is_ok());
        assert!(dfa.accepts(&small).is_err());
        assert!(dfa.accepts(&large).is_err());

        assert!(dfa.trace_states(&correct).is_ok());
        assert!(dfa.trace_states(&small).is_err());
        assert!(dfa.trace_states(&large).is_err());
    }

    #[test]
    fn invalid_input_many_chars() {
        let dfa = Dfa::build(vec![0; 500], HashSet::from([0]), 9, 49).unwrap();
        let long_valid: Vec<u16> = (0..1_000).map(|n| n % 50).collect();
        let long_invalid_barley: Vec<u16> = (0..1_000).map(|n| n % 51).collect();
        let long_invalid: Vec<u16> = (0..1_000).collect();

        assert!(dfa.accepts(&long_valid).is_ok());
        assert!(dfa.accepts(&long_invalid_barley).is_err());
        assert!(dfa.accepts(&long_invalid).is_err());

        assert!(dfa.trace_states(&long_valid).is_ok());
        assert!(dfa.trace_states(&long_invalid_barley).is_err());
        assert!(dfa.trace_states(&long_invalid).is_err());
    }

    #[test]
    fn valid_input_empty() {
        let dfa = Dfa::build(vec![1, 0, 1, 0], HashSet::from([0]), 1, 1).unwrap();
        let input = vec![];
        assert!(dfa.trace_states(&input).is_ok());
        assert!(dfa.accepts(&input).is_ok());
    }

    #[test]
    fn accepts_odd_length_dfa() {
        let odd_len_dfa = Dfa::build(vec![1, 0], HashSet::from([1]), 1, 0).unwrap();
        let expected_states = add_tape_mov_stay_fir([0, 1].repeat(100), TapeMovement::Right(None));
        for len in 3..100 {
            let input = [0].repeat(len);
            assert_eq!(odd_len_dfa.accepts(&input).unwrap(), len % 2 == 1);
            assert_eq!(
                odd_len_dfa.trace_states(&input).unwrap(),
                expected_states[..(len + 1)]
            )
        }
    }

    #[test]
    fn accepts_end_in_2() {
        let tf = vec![0, 0, 1, 0, 0, 1];
        let end_in_2_dfa = Dfa::build(tf, HashSet::from([1]), 1, 2).unwrap();
        assert!(!end_in_2_dfa.accepts(&[0, 1, 0, 1, 0, 1, 0]).unwrap());
        assert_eq!(
            end_in_2_dfa.trace_states(&[0, 1, 0, 1, 0, 1, 0]).unwrap(),
            add_tape_mov_stay_fir(vec![0, 0, 0, 0, 0, 0, 0, 0], TapeMovement::Right(None))
        );

        assert!(!end_in_2_dfa
            .accepts(&[0, 1, 0, 1, 0, 2, 2, 0, 1, 2, 1])
            .unwrap());
        assert_eq!(
            end_in_2_dfa
                .trace_states(&[0, 1, 0, 1, 0, 2, 2, 0, 1, 2, 1])
                .unwrap(),
            add_tape_mov_stay_fir(
                vec![0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0],
                TapeMovement::Right(None)
            )
        );

        assert!(!end_in_2_dfa.accepts(&[0, 1, 2, 1]).unwrap());
        assert_eq!(
            end_in_2_dfa.trace_states(&[0, 1, 2, 1]).unwrap(),
            add_tape_mov_stay_fir(vec![0, 0, 0, 1, 0], TapeMovement::Right(None))
        );

        assert!(end_in_2_dfa.accepts(&[0, 0, 0, 2]).unwrap());
        assert_eq!(
            end_in_2_dfa.trace_states(&[0, 0, 0, 2]).unwrap(),
            add_tape_mov_stay_fir(vec![0, 0, 0, 0, 1], TapeMovement::Right(None))
        );

        assert!(end_in_2_dfa.accepts(&[0, 1, 2]).unwrap());
        assert_eq!(
            end_in_2_dfa.trace_states(&[0, 1, 2]).unwrap(),
            add_tape_mov_stay_fir(vec![0, 0, 0, 1], TapeMovement::Right(None))
        );

        assert!(end_in_2_dfa.accepts(&[2, 2, 2, 2, 2]).unwrap());
        assert_eq!(
            end_in_2_dfa.trace_states(&[2, 2, 2, 2, 2]).unwrap(),
            add_tape_mov_stay_fir(vec![0, 1, 1, 1, 1, 1], TapeMovement::Right(None))
        );

        assert!(end_in_2_dfa.accepts(&[2, 2, 0, 2]).unwrap());
        assert_eq!(
            end_in_2_dfa.trace_states(&[2, 2, 0, 2]).unwrap(),
            add_tape_mov_stay_fir(vec![0, 1, 1, 0, 1], TapeMovement::Right(None))
        );
    }

    #[test]
    fn accepts_100_len() {
        let mut tf = vec![];
        for next in 1..=101 {
            tf.extend_from_slice(&[next].repeat(10));
        }
        tf.extend_from_slice(&[101].repeat(10));
        let accepts_len_100 = Dfa::build(tf, HashSet::from([100]), 101, 9).unwrap();
        assert!(accepts_len_100
            .accepts(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9].repeat(10))
            .unwrap());
        assert_eq!(
            accepts_len_100
                .trace_states(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9].repeat(10))
                .unwrap(),
            add_tape_mov_stay_fir((0..=100).collect::<Vec<u16>>(), TapeMovement::Right(None))
        );
        let mut input = vec![];
        for i in 0..100 {
            assert!(!accepts_len_100.accepts(&input).unwrap());
            input.push(i % 7);
        }
        input.push(0);

        for i in 0..100 {
            assert!(!accepts_len_100.accepts(&input).unwrap());
            input.push(i % 7);
        }
    }

    #[test]
    fn accepts_0_star_1_star() {
        let dfa = Dfa::build(vec![0, 1, 2, 1, 2, 2], HashSet::from([0, 1]), 2, 1).unwrap();
        assert!(dfa.accepts(&[]).unwrap());
        assert_eq!(
            dfa.trace_states(&[]).unwrap(),
            add_tape_mov_stay_fir(vec![0], TapeMovement::Right(None))
        );

        assert!(dfa.accepts(&[0, 1]).unwrap());
        assert_eq!(
            dfa.trace_states(&[0, 1]).unwrap(),
            add_tape_mov_stay_fir(vec![0, 0, 1], TapeMovement::Right(None))
        );

        assert!(dfa.accepts(&[0, 0, 0, 1]).unwrap());
        assert_eq!(
            dfa.trace_states(&[0, 0, 0, 1]).unwrap(),
            add_tape_mov_stay_fir(vec![0, 0, 0, 0, 1], TapeMovement::Right(None))
        );

        assert!(dfa.accepts(&[0, 1, 1, 1]).unwrap());
        assert_eq!(
            dfa.trace_states(&[0, 1, 1, 1]).unwrap(),
            add_tape_mov_stay_fir(vec![0, 0, 1, 1, 1], TapeMovement::Right(None))
        );

        assert!(dfa.accepts(&[0, 0, 1, 1]).unwrap());
        assert_eq!(
            dfa.trace_states(&[0, 0, 1, 1]).unwrap(),
            add_tape_mov_stay_fir(vec![0, 0, 0, 1, 1], TapeMovement::Right(None))
        );

        assert!(dfa.accepts(&[0, 0, 0, 0, 0, 0, 1, 1]).unwrap());
        assert_eq!(
            dfa.trace_states(&[0, 0, 0, 0, 0, 0, 1, 1]).unwrap(),
            add_tape_mov_stay_fir(vec![0, 0, 0, 0, 0, 0, 0, 1, 1], TapeMovement::Right(None))
        );

        assert!(dfa.accepts(&[0]).unwrap());
        assert_eq!(
            dfa.trace_states(&[0]).unwrap(),
            add_tape_mov_stay_fir(vec![0, 0], TapeMovement::Right(None)),
        );

        assert!(dfa.accepts(&[0, 0]).unwrap());
        assert_eq!(
            dfa.trace_states(&[0, 0]).unwrap(),
            add_tape_mov_stay_fir(vec![0, 0, 0], TapeMovement::Right(None))
        );

        assert!(dfa.accepts(&[1]).unwrap());
        assert_eq!(
            dfa.trace_states(&[1]).unwrap(),
            add_tape_mov_stay_fir(vec![0, 1], TapeMovement::Right(None))
        );

        assert!(!dfa.accepts(&[0, 1, 0]).unwrap());
        assert_eq!(
            dfa.trace_states(&[0, 1, 0]).unwrap(),
            add_tape_mov_stay_fir(vec![0, 0, 1, 2], TapeMovement::Right(None))
        );

        assert!(!dfa.accepts(&[0, 1, 0, 0, 0]).unwrap());
        assert_eq!(
            dfa.trace_states(&[0, 1, 0, 0, 0]).unwrap(),
            add_tape_mov_stay_fir(vec![0, 0, 1, 2, 2, 2], TapeMovement::Right(None))
        );

        assert!(!dfa.accepts(&[1, 0]).unwrap());
        assert_eq!(
            dfa.trace_states(&[1, 0]).unwrap(),
            add_tape_mov_stay_fir(vec![0, 1, 2], TapeMovement::Right(None))
        );

        assert!(!dfa.accepts(&[0, 0, 0, 0, 1, 0, 1, 0, 1]).unwrap());
        assert_eq!(
            dfa.trace_states(&[0, 0, 0, 0, 1, 0, 1, 0, 1]).unwrap(),
            add_tape_mov_stay_fir(
                vec![0, 0, 0, 0, 0, 1, 2, 2, 2, 2],
                TapeMovement::Right(None)
            )
        );
    }
}
