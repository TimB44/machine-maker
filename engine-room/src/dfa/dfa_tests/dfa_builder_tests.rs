use crate::{
    dfa::{Dfa, DfaBuilder},
    machine_utils::add_tape_mov_stay_fir,
    StateMachine, StateMachineBuilder, TapeMovement,
};
use std::collections::HashSet;

#[test]
fn create_builder_from_scratch() {
    let dfa = Dfa::build(vec![1, 0], HashSet::from([0]), 2, 1).unwrap();
    let _builder = DfaBuilder::new(dfa);
}

#[test]
fn create_builder_from_into() {
    let dfa = Dfa::build(vec![1, 0], HashSet::from([0]), 2, 1).unwrap();
    let _builder: DfaBuilder = dfa.into();
}

#[test]
fn add_state() {
    let dfa = Dfa::build(vec![1, 0], HashSet::from([0]), 2, 1).unwrap();
    let mut builder: DfaBuilder = dfa.into();

    assert_eq!(builder.add_state(), 2);
    let new_dfa: Result<Dfa, _> = builder.try_into();

    assert!(new_dfa.is_err());
}

#[test]
fn remove_state_last() {
    let dfa = Dfa::build(vec![0, 1], HashSet::from([0, 1]), 2, 1).unwrap();
    let mut builder: DfaBuilder = dfa.into();

    assert_eq!(builder.remove_state(1).unwrap(), None);
    let new_dfa: Dfa = builder.try_into().unwrap();
    assert_eq!(new_dfa.states, 1);
    assert_eq!(new_dfa.chars, 1);
    assert_eq!(new_dfa.transition_table, vec![0]);
    assert_eq!(new_dfa.accept_states, HashSet::from([0]));
}

#[test]
fn remove_state_first() {
    let dfa = Dfa::build(vec![0, 1], HashSet::from([0, 1]), 2, 1).unwrap();
    let mut builder: DfaBuilder = dfa.into();

    assert_eq!(builder.remove_state(0).unwrap(), Some(1));
    let new_dfa: Dfa = builder.try_into().unwrap();
    assert_eq!(new_dfa.states, 1);
    assert_eq!(new_dfa.chars, 1);
    assert_eq!(new_dfa.transition_table, vec![0]);
    assert_eq!(new_dfa.accept_states, HashSet::from([0]));
}

#[test]
fn remove_state_middle_large() {
    let dfa = Dfa::build(vec![0, 1], HashSet::from([0, 1]), 2, 1).unwrap();
    let mut builder: DfaBuilder = dfa.into();

    assert_eq!(builder.remove_state(0).unwrap(), Some(1));
    let new_dfa: Dfa = builder.try_into().unwrap();
    assert_eq!(new_dfa.states, 1);
    assert_eq!(new_dfa.chars, 1);
    assert_eq!(new_dfa.transition_table, vec![0]);
    assert_eq!(new_dfa.accept_states, HashSet::from([0]));
}
