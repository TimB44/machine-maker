use crate::dfa::Dfa;
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
    let large = vec![5325, 124325, 23564, 3252, 31252312];
    let dfa = Dfa::build(vec![1, 0, 1, 0], HashSet::from([0]), 1, 1).unwrap();

    assert!(dfa.accepts(&correct).is_ok());
    assert!(dfa.accepts(&small).is_err());
    assert!(dfa.accepts(&large).is_err());

    assert!(dfa.state_trace(&correct).is_ok());
    assert!(dfa.state_trace(&small).is_err());
    assert!(dfa.state_trace(&large).is_err());
}

#[test]
fn invalid_input_many_chars() {
    let dfa = Dfa::build(vec![0; 500], HashSet::from([0]), 9, 49).unwrap();
    let long_valid = (0..1_000).into_iter().map(|n| n % 50).collect();
    let long_invalid_barley = (0..1_000).into_iter().map(|n| n % 51).collect();
    let long_invalid = (0..1_000).into_iter().collect();

    assert!(dfa.accepts(&long_valid).is_ok());
    assert!(dfa.accepts(&long_invalid_barley).is_err());
    assert!(dfa.accepts(&long_invalid).is_err());

    assert!(dfa.state_trace(&long_valid).is_ok());
    assert!(dfa.state_trace(&long_invalid_barley).is_err());
    assert!(dfa.state_trace(&long_invalid).is_err());
}

#[test]
fn valid_input_empty() {
    let dfa = Dfa::build(vec![1, 0, 1, 0], HashSet::from([0]), 1, 1).unwrap();
    let input = vec![];
    assert!(dfa.state_trace(&input).is_ok());
    assert!(dfa.accepts(&input).is_ok());
}

#[test]
fn accepts_odd_length_dfa() {
    let odd_len_dfa = Dfa::build(vec![1, 0], HashSet::from([1]), 1, 0).unwrap();
    let expected_states = vec![0, 1].repeat(100);
    for len in 3..100 {
        let input = vec![0].repeat(len);
        assert_eq!(odd_len_dfa.accepts(&input).unwrap(), len % 2 == 1);
        assert_eq!(
            odd_len_dfa.state_trace(&input).unwrap(),
            expected_states[..(len + 1)]
        )
    }
}

#[test]
fn accepts_end_in_2() {
    let tf = vec![0, 0, 1, 0, 0, 1];
    let end_in_2_dfa = Dfa::build(tf, HashSet::from([1]), 1, 2).unwrap();
    assert!(!end_in_2_dfa.accepts(&vec![0, 1, 0, 1, 0, 1, 0]).unwrap());
    assert_eq!(
        end_in_2_dfa
            .state_trace(&vec![0, 1, 0, 1, 0, 1, 0])
            .unwrap(),
        vec![0, 0, 0, 0, 0, 0, 0, 0]
    );

    assert!(!end_in_2_dfa
        .accepts(&vec![0, 1, 0, 1, 0, 2, 2, 0, 1, 2, 1])
        .unwrap());
    assert_eq!(
        end_in_2_dfa
            .state_trace(&vec![0, 1, 0, 1, 0, 2, 2, 0, 1, 2, 1])
            .unwrap(),
        vec![0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0]
    );

    assert!(!end_in_2_dfa.accepts(&vec![0, 1, 2, 1]).unwrap());
    assert_eq!(
        end_in_2_dfa.state_trace(&vec![0, 1, 2, 1]).unwrap(),
        vec![0, 0, 0, 1, 0]
    );

    assert!(end_in_2_dfa.accepts(&vec![0, 0, 0, 2]).unwrap());
    assert_eq!(
        end_in_2_dfa.state_trace(&vec![0, 0, 0, 2]).unwrap(),
        vec![0, 0, 0, 0, 1]
    );

    assert!(end_in_2_dfa.accepts(&vec![0, 1, 2]).unwrap());
    assert_eq!(
        end_in_2_dfa.state_trace(&vec![0, 1, 2]).unwrap(),
        vec![0, 0, 0, 1]
    );

    assert!(end_in_2_dfa.accepts(&vec![2, 2, 2, 2, 2]).unwrap());
    assert_eq!(
        end_in_2_dfa.state_trace(&vec![2, 2, 2, 2, 2]).unwrap(),
        vec![0, 1, 1, 1, 1, 1]
    );

    assert!(end_in_2_dfa.accepts(&vec![2, 2, 0, 2]).unwrap());
    assert_eq!(
        end_in_2_dfa.state_trace(&vec![2, 2, 0, 2]).unwrap(),
        vec![0, 1, 1, 0, 1]
    );
}

#[test]
fn accepts_100_len() {
    let mut tf = vec![];
    for next in 1..=101 {
        tf.extend_from_slice(&vec![next].repeat(10));
    }
    tf.extend_from_slice(&vec![101].repeat(10));
    let accepts_len_100 = Dfa::build(tf, HashSet::from([100]), 101, 9).unwrap();
    assert!(accepts_len_100
        .accepts(&vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9].repeat(10))
        .unwrap());
    assert_eq!(
        accepts_len_100
            .state_trace(&vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9].repeat(10))
            .unwrap(),
        (0..=100).collect::<Vec<usize>>()
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
