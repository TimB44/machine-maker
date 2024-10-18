use crate::{dfa::Dfa, machine_utils::add_tape_mov_stay_fir, StateMachine, TapeMovement};
use std::collections::HashSet;

#[test]
fn wrong_length_errors() {
    let tf_long = vec![0, 1, 1, 0, 1];
    let tf_correct = vec![0, 1, 1, 0];
    let tf_short = vec![0, 1, 1];
    assert!(Dfa::build(tf_long, HashSet::from([1]), 2, 2).is_err());
    assert!(Dfa::build(tf_short, HashSet::from([1]), 2, 2).is_err());
    assert!(Dfa::build(tf_correct, HashSet::from([1]), 2, 2).is_ok());
}

#[test]
fn wrong_length_uneven_errors() {
    let tf_long = vec![0; 11];
    let tf_short = vec![0; 8];
    let tf_correct = vec![0; 10];
    assert!(Dfa::build(tf_long, HashSet::from([1]), 2, 5).is_err());
    assert!(Dfa::build(tf_short, HashSet::from([1]), 2, 5).is_err());
    assert!(Dfa::build(tf_correct, HashSet::from([1]), 2, 5).is_ok());
}

#[test]
fn invalid_accept_states() {
    let tf = vec![0; 20];
    assert!(Dfa::build(tf.clone(), HashSet::from([5]), 5, 4).is_err());
    assert!(Dfa::build(tf.clone(), HashSet::from([4, 5]), 5, 4).is_err());
    assert!(Dfa::build(tf.clone(), HashSet::from([500]), 5, 4).is_err());
    assert!(Dfa::build(tf.clone(), HashSet::from([0, 1, 2, 3, 4]), 5, 4).is_ok());
}

#[test]
fn to_large_state_errors() {
    let tf_correct = vec![0, 1, 2, 2, 1, 1];
    let tf_one_to_large = vec![0, 1, 2, 3, 1, 1];
    let tf_way_to_large = vec![10, 734, 234, 12, 93, 1523];

    assert!(Dfa::build(tf_correct, HashSet::from([0]), 3, 2).is_ok());
    assert!(Dfa::build(tf_one_to_large, HashSet::from([0]), 3, 2).is_err());
    assert!(Dfa::build(tf_way_to_large, HashSet::from([0]), 3, 2).is_err());
}

#[test]
fn invalid_input() {
    let correct = vec![0, 1, 1, 0];
    let small = vec![0, 1, 2, 1, 2, 1];
    let large = vec![5325, 2325, 23564, 3252, 312];
    let dfa = Dfa::build(vec![1, 0, 1, 0], HashSet::from([0]), 2, 2).unwrap();

    assert!(dfa.accepts(&correct).is_ok());
    assert!(dfa.accepts(&small).is_err());
    assert!(dfa.accepts(&large).is_err());

    assert!(dfa.trace_states(&correct).is_ok());
    assert!(dfa.trace_states(&small).is_err());
    assert!(dfa.trace_states(&large).is_err());
}

#[test]
fn invalid_input_many_chars() {
    let dfa = Dfa::build(vec![0; 500], HashSet::from([0]), 10, 50).unwrap();
    let long_valid: Vec<u16> = (0..1_000).map(|n| n % 50).collect();
    let long_invalid_barely: Vec<u16> = (0..1_000).map(|n| n % 51).collect();
    let long_invalid: Vec<u16> = (0..1_000).collect();

    assert!(dfa.accepts(&long_valid).is_ok());
    assert!(dfa.accepts(&long_invalid_barely).is_err());
    assert!(dfa.accepts(&long_invalid).is_err());

    assert!(dfa.trace_states(&long_valid).is_ok());
    assert!(dfa.trace_states(&long_invalid_barely).is_err());
    assert!(dfa.trace_states(&long_invalid).is_err());
}

#[test]
fn valid_input_empty() {
    let dfa = Dfa::build(vec![1, 0, 1, 0], HashSet::from([0]), 2, 2).unwrap();
    let input = vec![];
    assert!(dfa.trace_states(&input).is_ok());
    assert!(dfa.accepts(&input).is_ok());
}

#[test]
fn accepts_odd_length_dfa() {
    let odd_len_dfa = Dfa::build(vec![1, 0], HashSet::from([1]), 2, 1).unwrap();
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
    let end_in_2_dfa = Dfa::build(tf, HashSet::from([1]), 2, 3).unwrap();
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
    let accepts_len_100 = Dfa::build(tf, HashSet::from([100]), 102, 10).unwrap();
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
    let dfa = Dfa::build(vec![0, 1, 2, 1, 2, 2], HashSet::from([0, 1]), 3, 2).unwrap();
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
