use crate::{
    dfa::{Dfa, DfaBuilder},
    machine_utils::add_tape_mov_stay_fir,
    transitions::{self, SingleChar},
    StateMachine, StateMachineBuilder, TapeMovement,
};
use std::{cmp::min, collections::HashSet, iter::RepeatN};

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
fn remove_state_large() {
    let table = (0..8).into_iter().rev().collect::<Vec<u16>>().repeat(8);
    let dfa = Dfa::build(table, HashSet::from([0, 2, 4, 6]), 8, 8).unwrap();
    let mut builder: DfaBuilder = dfa.into();

    assert_eq!(builder.remove_state(1).unwrap(), Some(7));
    assert_eq!(builder.states, 7);
    assert_eq!(builder.chars, 8);
    assert_eq!(builder.accept_states, HashSet::from([0, 2, 4, 6]));
    assert_eq!(
        builder.building_layers,
        [
            Some(1),
            Some(6),
            Some(5),
            Some(4),
            Some(3),
            Some(2),
            None,
            Some(0)
        ]
        .into_iter()
        .collect::<Vec<Option<u16>>>()
        .repeat(7)
    );

    assert_eq!(builder.remove_state(1).unwrap(), Some(6));
    assert_eq!(builder.states, 6);
    assert_eq!(builder.chars, 8);
    assert_eq!(builder.accept_states, HashSet::from([0, 2, 4, 1]));
    assert_eq!(
        builder.building_layers,
        [
            None,
            Some(1),
            Some(5),
            Some(4),
            Some(3),
            Some(2),
            None,
            Some(0)
        ]
        .into_iter()
        .collect::<Vec<Option<u16>>>()
        .repeat(6)
    );
}

#[test]
fn remove_state_invalid_no_effect() {
    let dfa = Dfa {
        transition_table: vec![0, 1, 0, 0, 0, 0],
        accept_states: HashSet::from([1]),
        states: 2,
        chars: 3,
    };
    let mut builder: DfaBuilder = dfa.try_into().unwrap();
    let copy = builder.clone();

    for i in 2..100 {
        assert!(builder.remove_state(i).is_err());
        assert_eq!(builder, copy);
    }

    assert_eq!(builder.remove_state(0).unwrap(), Some(1));
    assert_eq!(builder.states, 1);
    assert_eq!(builder.accept_states, HashSet::from([0]));
    assert_eq!(builder.building_layers, [None, None, None,]);

    let copy = builder.clone();

    assert!(builder.remove_state(0).is_err());
    assert_eq!(builder, copy);
}

#[test]
fn add_then_remove() {
    let dfa = Dfa {
        transition_table: vec![0, 1, 0, 0, 0, 0],
        accept_states: HashSet::from([1]),
        states: 2,
        chars: 3,
    };
    let mut builder: DfaBuilder = dfa.try_into().unwrap();
    let copy = builder.clone();

    for i in 1..100 {
        for _ in 0..i {
            builder.add_state();
        }

        for _ in 0..i {
            builder
                .remove_state(min(builder.states - 1, 2 + i % 7))
                .unwrap();
        }
        assert_eq!(builder, copy);
    }
}

#[test]
fn set_transition() {
    let dfa = Dfa {
        transition_table: vec![0, 1, 0, 0, 0, 0],
        accept_states: HashSet::from([1]),
        states: 2,
        chars: 3,
    };

    let mut builder: DfaBuilder = dfa.try_into().unwrap();
    assert!(builder
        .set_transition(SingleChar {
            start: 0,
            end: 0,
            char: 0
        })
        .is_ok());
    assert!(builder
        .set_transition(SingleChar {
            start: 0,
            end: 0,
            char: 1
        })
        .is_ok());
    assert!(builder
        .set_transition(SingleChar {
            start: 0,
            end: 0,
            char: 2
        })
        .is_ok());
    assert!(builder
        .set_transition(SingleChar {
            start: 1,
            end: 0,
            char: 0
        })
        .is_ok());
    assert!(builder
        .set_transition(SingleChar {
            start: 0,
            end: 1,
            char: 1
        })
        .is_ok());
    assert!(builder
        .set_transition(SingleChar {
            start: 1,
            end: 1,
            char: 2
        })
        .is_ok());

    assert!(builder
        .set_transition(SingleChar {
            start: 0,
            end: 0,
            char: 3
        })
        .is_err());
    assert!(builder
        .set_transition(SingleChar {
            start: 0,
            end: 0,
            char: 4
        })
        .is_err());
    assert!(builder
        .set_transition(SingleChar {
            start: 0,
            end: 2,
            char: 0
        })
        .is_err());
    assert!(builder
        .set_transition(SingleChar {
            start: 2,
            end: 0,
            char: 1
        })
        .is_err());
    assert!(builder
        .set_transition(SingleChar {
            start: 5,
            end: 4,
            char: 2
        })
        .is_err());

    assert_eq!(builder.add_state(), 2);

    assert!(builder
        .set_transition(SingleChar {
            start: 2,
            end: 1,
            char: 0
        })
        .is_ok());
    assert!(builder
        .set_transition(SingleChar {
            start: 1,
            end: 2,
            char: 2
        })
        .is_ok());
    assert!(builder
        .set_transition(SingleChar {
            start: 2,
            end: 2,
            char: 0
        })
        .is_ok());

    assert!(builder
        .set_transition(SingleChar {
            start: 3,
            end: 2,
            char: 0
        })
        .is_err());
    assert!(builder
        .set_transition(SingleChar {
            start: 2,
            end: 3,
            char: 1
        })
        .is_err());
    assert!(builder
        .set_transition(SingleChar {
            start: 5,
            end: 4,
            char: 2
        })
        .is_err());
}

#[test]
fn set_transition_after_remove() {
    let dfa = Dfa::build(vec![1, 2, 0], HashSet::from([2]), 3, 1).unwrap();
    let mut builder: DfaBuilder = dfa.into();
    assert_eq!(builder.remove_state(2).unwrap(), None);
    assert_eq!(builder.accept_states, HashSet::new());
    assert_eq!(builder.states, 2);
    assert_eq!(builder.chars, 1);

    assert!(builder
        .set_transition(SingleChar {
            start: 0,
            end: 1,
            char: 0,
        })
        .is_ok());
    assert!(builder
        .set_transition(SingleChar {
            start: 1,
            end: 0,
            char: 0,
        })
        .is_ok());
    assert!(builder
        .set_transition(SingleChar {
            start: 1,
            end: 1,
            char: 0,
        })
        .is_ok());

    assert!(builder
        .set_transition(SingleChar {
            start: 0,
            end: 2,
            char: 0,
        })
        .is_err());
    assert!(builder
        .set_transition(SingleChar {
            start: 2,
            end: 1,
            char: 0,
        })
        .is_err());

    assert!(builder
        .set_transition(SingleChar {
            start: 2,
            end: 2,
            char: 0,
        })
        .is_err());
    assert!(builder
        .set_transition(SingleChar {
            start: 10,
            end: 0,
            char: 0,
        })
        .is_err());
    assert!(builder
        .set_transition(SingleChar {
            start: 4,
            end: 10,
            char: 0,
        })
        .is_err());
}

#[test]
fn set_start_state() {
    let dfa = Dfa::build(vec![1, 0], HashSet::from([1]), 2, 1).unwrap();
    let mut builder: DfaBuilder = dfa.into();
    let copy = builder.clone();
    builder.set_start_state(0).unwrap();
    assert_eq!(builder, copy);

    builder.set_start_state(1).unwrap();
    assert_eq!(builder.building_layers, vec![Some(1), Some(0)]);
    assert_eq!(builder.accept_states, HashSet::from([0]));
    assert_eq!(builder.states, 2);
    assert_eq!(builder.chars, 1);
}

#[test]
fn set_start_state_invalid() {
    let dfa = Dfa::build(vec![1, 0], HashSet::from([1]), 2, 1).unwrap();
    let mut builder: DfaBuilder = dfa.into();
    let copy = builder.clone();
    for i in 2..100 {
        assert!(builder.set_start_state(i).is_err());
    }

    assert_eq!(builder, copy);
}

#[test]
fn add_char() {
    let dfa = Dfa::build(vec![1, 0], HashSet::from([1]), 2, 1).unwrap();
    let mut builder: DfaBuilder = dfa.into();
    let copy = builder.clone();
    builder.add_char();
    builder.remove_char(1).unwrap();
    assert_eq!(builder, copy);

    builder.add_char();
    assert_eq!(builder.building_layers, vec![Some(1), None, Some(0), None]);
    assert_eq!(builder.accept_states, copy.accept_states);
    assert_eq!(builder.states, copy.states);
    assert_eq!(builder.chars, 2);
}

#[test]
fn add_accept_state() {
    let dfa = Dfa::build(vec![1, 0], HashSet::from([1]), 2, 1).unwrap();
    let mut builder: DfaBuilder = dfa.into();
    let copy = builder.clone();

    for i in 2..100 {
        assert!(builder.add_accept_state(i).is_err());
        assert_eq!(builder, copy);
    }
    builder.add_accept_state(1).unwrap();
    assert_eq!(builder, copy);

    builder.add_accept_state(0).unwrap();
    assert_eq!(builder.building_layers, copy.building_layers);
    assert_eq!(builder.accept_states, HashSet::from([0, 1]));
    assert_eq!(builder.states, copy.states);
    assert_eq!(builder.chars, copy.chars);
}
