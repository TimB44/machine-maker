use std::collections::HashSet;

use crate::{machine_utils::table_lookup, nfa::Nfa, StateMachine, TapeMovement};

pub struct EpsilonNfa {
    nfa: Nfa,
    transition_table: Vec<HashSet<u16>>,
    accept_states: HashSet<u16>,
}

impl EpsilonNfa {
    /// Builds a NFA that contains epsilon transitions
    /// The character 0 is used for the epsilon character
    pub fn build(
        transition_table: Vec<HashSet<u16>>,
        accept_states: HashSet<u16>,
        max_state: u16,
        max_char: u16,
    ) -> Result<EpsilonNfa, ()> {
        if transition_table.len() != ((max_state + 1) * (max_char + 1)) as usize {
            return Err(());
        }
        if transition_table
            .iter()
            .flatten()
            .chain(accept_states.iter())
            .any(|item| item > &max_state)
        {
            return Err(());
        }

        let nfa = Self::convert_to_nfa(&transition_table, &accept_states, max_state, max_char);
        Ok(EpsilonNfa {
            nfa,
            transition_table,
            accept_states,
        })
    }

    fn convert_to_nfa(
        transition_table: &Vec<HashSet<u16>>,
        accept_states: &HashSet<u16>,
        max_state: u16,
        max_char: u16,
    ) -> Nfa {
        let mut epsilon_closure = vec![HashSet::new(); max_state as usize + 1];
        let mut seen = vec![false; max_state as usize + 1];
        for cur_state in 0..=max_state {
            seen.fill(false);
            dfs(
                transition_table,
                &mut seen,
                max_state,
                max_char,
                cur_state,
                cur_state,
                &mut epsilon_closure,
            );
        }
        debug_assert!(epsilon_closure
            .iter()
            .enumerate()
            .all(|(i, s)| s.contains(&i.try_into().unwrap())));
        let mut nfa_transition_table = Vec::with_capacity(((max_state + 1) * max_char) as usize);
        nfa_transition_table.extend(
            transition_table
                .iter()
                .enumerate()
                // Filter out all of the epsilon transitions
                .filter(|(index, _)| index % max_state as usize == 0)
                .map(|(_, set)| set.clone()),
        );
        debug_assert_eq!(
            ((max_state + 1) * max_char) as usize,
            nfa_transition_table.len()
        );
        for (index, set) in nfa_transition_table.iter_mut().enumerate() {
            let cur_state = index / (max_state as usize + 1);
            let cur_char = index % (max_state as usize + 1);

            let can_reach: HashSet<u16> = epsilon_closure[cur_state]
                .iter()
                .copied()
                .flat_map(|state| {
                    &transition_table
                        [table_lookup(state as usize, cur_char, max_char as usize)]
                })
                .copied()
                .collect();

            set.extend(
                can_reach
                    .iter()
                    .copied()
                    .flat_map(|s| &epsilon_closure[s as usize]),
            )
        }

        let nfa_accept_states = epsilon_closure
            .iter()
            .enumerate()
            .filter(|(_, e_closure)| !e_closure.is_disjoint(accept_states))
            .map(|(state, _)| state as u16)
            .collect();

        fn dfs(
            transition_table: &Vec<HashSet<u16>>,
            seen: &mut [bool],
            max_state: u16,
            max_char: u16,
            cur_state: u16,
            search_src: u16,
            can_reach_from: &mut Vec<HashSet<u16>>,
        ) {
            debug_assert_eq!(max_state as usize + 1, can_reach_from.len());
            debug_assert_eq!(max_state as usize + 1, seen.len());
            debug_assert_eq!(
                ((max_state + 1) * (max_char + 1)) as usize,
                transition_table.len()
            );
            if seen[cur_state as usize] {
                return;
            }
            seen[cur_state as usize] = true;
            can_reach_from[search_src as usize].insert(cur_state);
            for next_state_though_epsilon in transition_table
                [table_lookup(cur_state as usize, 0, max_char as usize)]
            .iter()
            .copied()
            {
                dfs(
                    transition_table,
                    seen,
                    max_state,
                    max_char,
                    next_state_though_epsilon,
                    search_src,
                    can_reach_from,
                );
            }
        }
        Nfa::build(
            nfa_transition_table,
            nfa_accept_states,
            max_state,
            max_char - 1,
        )
        .expect("Nfa Could not be built")
    }
}

impl StateMachine for EpsilonNfa {
    fn accepts_validated(&self, input: &[u16]) -> bool {
        self.nfa.accepts_validated(input)
    }

    fn trace_states_validated(&self, input: &[u16]) -> Vec<(u16, Vec<TapeMovement>)> {
        let nfa_state_trace = self.nfa.trace_states_validated(input);
        dbg!(nfa_state_trace);

        todo!()
    }

    fn max_state(&self) -> u16 {
        self.nfa.max_state()
    }

    fn max_input(&self) -> u16 {
        self.nfa.max_input()
    }
}
