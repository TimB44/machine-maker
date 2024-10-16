use std::{
    collections::{HashMap, HashSet, VecDeque},
    iter::repeat,
};

use crate::{machine_utils::table_lookup, nfa::Nfa, StateMachine, TapeMovement};

#[derive(Debug, Clone)]
pub struct EpsilonNfa {
    nfa: Nfa,
    transition_table: Vec<HashSet<u16>>,
    accept_states: HashSet<u16>,
    epslion_closure_paths: Vec<HashMap<u16, Vec<u16>>>,
}

#[derive(Debug, Clone)]
pub struct EpsilonNfaBuilder {}

//TODO:
impl From<EpsilonNfa> for EpsilonNfaBuilder {
    fn from(value: EpsilonNfa) -> Self {
        todo!()
    }
}

//TODO:
impl TryInto<EpsilonNfa> for EpsilonNfaBuilder {
    type Error = ();

    fn try_into(self) -> Result<EpsilonNfa, Self::Error> {
        todo!()
    }
}

impl EpsilonNfa {
    /// Builds a NFA that contains epsilon transitions. The transition_table is laid out in the
    /// following form: The first states tranition for the input 0, are at index 0, 1 at 1, 2 at
    /// 2, ... max_char at max_char. At index max_char + 1 are the epsilon transitions for state 0.
    /// The same strucutre is used to store the rest of the tranitions. This means that the
    /// input transition table should has a length of `((max_state + 1) * (max_char + 2))`
    pub fn build(
        transition_table: Vec<HashSet<u16>>,
        accept_states: HashSet<u16>,
        max_state: u16,
        max_char: u16,
    ) -> Result<EpsilonNfa, ()> {
        if transition_table.len() != ((max_state + 1) * (max_char + 2)) as usize {
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

        let epslion_closure_paths =
            Self::epsilon_closure_paths(&transition_table, max_state, max_char);
        let nfa = Self::convert_to_nfa(
            &transition_table,
            &accept_states,
            &epslion_closure_paths,
            max_state,
            max_char,
        );
        Ok(EpsilonNfa {
            nfa,
            transition_table,
            accept_states,
            epslion_closure_paths,
        })
    }

    /// Takes a transition_table for a EpsilonNfa and returns epslion closure paths for each state.
    /// This is a vector of HashMaps. Vec[i] contains a HashMap h where the keys of h is the set of
    /// states that can be reached from the ith state using only epslion transitions. For each key
    /// in this map their associated value is the path from state i to the key state taken though
    /// the epslion transitions
    fn epsilon_closure_paths(
        transition_table: &Vec<HashSet<u16>>,
        max_state: u16,
        max_char: u16,
    ) -> Vec<HashMap<u16, Vec<u16>>> {
        debug_assert_eq!(
            transition_table.len(),
            ((max_state + 1) * (max_char + 2)) as usize
        );
        let mut epsilon_paths = vec![HashMap::new(); max_state as usize + 1];
        let mut seen = vec![false; max_state as usize + 1];
        for search_src_state in 0..=max_state {
            seen.fill(false);
            seen[search_src_state as usize] = true;
            let mut q: VecDeque<Vec<u16>> = VecDeque::from([vec![search_src_state]]);
            while !q.is_empty() {
                // Get the next item in the queue and get the last item in the path, which will be
                // the cur_state being explored
                let cur_state_path = q
                    .pop_front()
                    .expect("Queue should not be empty due to loop condition");
                let &cur_state = cur_state_path.last().expect("Path should never be empty");

                // Mark cur_state as seen
                seen[cur_state as usize] = true;

                // Iterate over all of the states that can be reached though a epslion tranition
                // and visit add their path to the queue if they have not been seen yet.
                for &dest_state in &transition_table[table_lookup(
                    cur_state as usize,
                    max_char as usize + 1,
                    max_char as usize + 1,
                )] {
                    if seen[dest_state as usize] {
                        continue;
                    }
                    let mut dest_path = cur_state_path.clone();
                    dest_path.push(dest_state);
                    q.push_back(dest_path);
                }
                // Add the path to the map
                epsilon_paths[search_src_state as usize].insert(cur_state, cur_state_path);
            }
        }
        debug_assert!(epsilon_paths.iter().enumerate().all(|(i, map)| map
            .iter()
            .all(|(state, path)| path[0] as usize == i && path.last().unwrap() == state)));

        debug_assert!(epsilon_paths
            .iter()
            .enumerate()
            .all(|(state, map)| map.get(&(state as u16)) == Some(&vec![state as u16])));

        epsilon_paths
    }

    fn convert_to_nfa(
        transition_table: &Vec<HashSet<u16>>,
        accept_states: &HashSet<u16>,
        epsilon_closure_paths: &Vec<HashMap<u16, Vec<u16>>>,
        max_state: u16,
        max_char: u16,
    ) -> Nfa {
        debug_assert_eq!(
            transition_table.len(),
            ((max_state + 1) * (max_char + 2)) as usize
        );
        let mut nfa_transition_table =
            Vec::with_capacity(((max_state + 1) * (max_char + 1)) as usize);
        nfa_transition_table.extend(
            transition_table
                .iter()
                .enumerate()
                // Filter out all of the epsilon transitions
                .filter(|(index, _)| !(index % (max_char + 2) as usize == (max_char + 1) as usize))
                .map(|(_, set)| set.clone()),
        );
        debug_assert_eq!(
            ((max_state + 1) * (max_char + 1)) as usize,
            nfa_transition_table.len()
        );
        for (index, set) in nfa_transition_table.iter_mut().enumerate() {
            let cur_state = index / (max_char as usize + 1);
            let cur_char = index % (max_char as usize + 1);

            let can_reach: HashSet<u16> = epsilon_closure_paths[cur_state]
                .keys()
                .copied()
                .flat_map(|state| {
                    &transition_table[table_lookup(state as usize, cur_char, max_char as usize + 1)]
                })
                .copied()
                .collect();

            set.extend(
                can_reach
                    .iter()
                    .copied()
                    .flat_map(|s| epsilon_closure_paths[s as usize].keys()),
            )
        }

        let nfa_accept_states = epsilon_closure_paths
            .iter()
            .enumerate()
            .filter(|(_, e_closure)| accept_states.iter().any(|s| e_closure.contains_key(s)))
            .map(|(state, _)| state as u16)
            .collect();

        Nfa::build(nfa_transition_table, nfa_accept_states, max_state, max_char)
            .expect("Nfa Could not be built")
    }
}

impl StateMachine for EpsilonNfa {
    fn accepts_validated(&self, input: &[u16]) -> bool {
        self.nfa.accepts_validated(input)
    }

    fn trace_states_validated(&self, input: &[u16]) -> Vec<(u16, Vec<TapeMovement>)> {
        // Use the Nfa to compute the path that skips though epsilon trasitions
        let nfa_state_trace = dbg!(self.nfa.trace_states_validated(input));

        assert!(nfa_state_trace.len() <= input.len() + 1);

        // This state trace will tranition between states that require an epsilon trasition to
        // happen in the EpsilonNfa.
        let mut e_nfa_state_trace = Vec::with_capacity(nfa_state_trace.len());
        let mut nfa_iter = nfa_state_trace.into_iter();
        let mut prev_trace = nfa_iter
            .next()
            .expect("NFA State Trace should always have at least the start state in it");

        e_nfa_state_trace.push((prev_trace.0, vec![TapeMovement::Stay(None)]));
        for (cur_trace, cur_char) in nfa_iter.zip(input.iter().copied()) {
            let src_state = prev_trace.0;
            let dest_state = cur_trace.0;
            let cur_neighbors = &self.transition_table[table_lookup(
                src_state as usize,
                cur_char as usize,
                self.nfa.chars() as usize + 1,
            )];

            // If this transition existis in the EpsilonNfa then we can move on
            if cur_neighbors.contains(&dest_state) {
                eprintln!("Skiped, {:?}", dest_state);
                e_nfa_state_trace.push((dest_state, vec![TapeMovement::Right(None)]));
                prev_trace = cur_trace;
                continue;
            }

            // TODO: Possible Optimization by adding a start state to the nfa and prepending a zero
            // to every input so that the state state has no epslion transition meaning we can always
            // assume that the path the nfa takes is:
            // Src State -> Transition that consumes input character -> Mid Point -> Epsilon Transition -> Dest State
            // Instead of:
            // Src State -> Epsilon Transition -> Mid State 1 -> Transition that consumes Character -> Mid Point 2 -> Epsilon Transition -> Dest State

            // If not then we traveled though epslion transitions in order to move src_state to
            // dest_state
            let (mid_point_1, mid_point_2) = self.epslion_closure_paths[src_state as usize]
            .keys()
            .copied()
            .flat_map(|s| {
                repeat(s).zip(
                    self.transition_table[table_lookup(
                        s as usize,
                        cur_char as usize,
                        self.nfa.chars() as usize + 1,
                    )]
                    .iter()
                    .copied(),
                )
            })
            .filter(|&(_, s)| self.epslion_closure_paths[s as usize].contains_key(&dest_state))
            .min_by_key(|&(m1, m2)| {
                self.epslion_closure_paths[src_state as usize]
                    .get(&m1)
                    .unwrap()
                    .len()
                    + self.epslion_closure_paths[m2 as usize]
                        .get(&dest_state)
                        .unwrap()
                        .len()
            })
            .expect("Path between src_state and dest_state must exist as the non-epsilon Nfa moved between them");

            e_nfa_state_trace.extend(
                self.epslion_closure_paths[src_state as usize][&mid_point_1]
                    .iter()
                    .skip(1)
                    .map(|&s| (s, vec![TapeMovement::Stay(None)]))
                    .chain([(mid_point_2, vec![TapeMovement::Right(None)])].into_iter())
                    .chain(
                        self.epslion_closure_paths[mid_point_2 as usize][&dest_state]
                            .iter()
                            .map(|&s| (s, vec![TapeMovement::Stay(None)]))
                            .skip(1),
                    ),
            );
            prev_trace = cur_trace;
        }

        let last_state = e_nfa_state_trace.last().unwrap().0;
        let nfa_accept_state = self.nfa.accept_states();

        if nfa_accept_state.contains(&last_state) {
            e_nfa_state_trace.extend(
                self.epslion_closure_paths[last_state as usize]
                    .iter()
                    .filter(|(k, _)| self.accept_states.contains(k))
                    .map(|(_, path)| path)
                    .min_by_key(|path| path.len())
                    .unwrap()
                    .iter()
                    .skip(1)
                    .map(|&s| (s, vec![TapeMovement::Stay(None)])),
            )
        }

        e_nfa_state_trace
    }

    fn states(&self) -> u16 {
        self.nfa.states()
    }

    fn chars(&self) -> u16 {
        self.nfa.chars()
    }
}
#[cfg(test)]
mod epslion_nfa_tests {
    use std::collections::HashSet;

    use crate::{StateMachine, TapeMovement};

    use super::EpsilonNfa;

    #[test]
    fn build_most_basic() {
        let enfa = EpsilonNfa::build(
            vec![HashSet::new(), HashSet::new()],
            HashSet::from([0]),
            0,
            0,
        )
        .unwrap();
        assert!(enfa.accepts(&[]).unwrap());
        assert!(!enfa.accepts(&[0, 0, 0, 0]).unwrap())
    }

    #[test]
    fn doulbe_epsilon_trasition() {}

    #[test]
    // Thie State Machine accepts 0∑*1*
    fn small_nfa() {
        let transition_table = vec![
            HashSet::from([1]),
            HashSet::new(),
            HashSet::new(),
            // State 1
            HashSet::from([1]),
            HashSet::from([1]),
            HashSet::from([2]),
            // State 2 (accept state)
            HashSet::new(),
            HashSet::from([2]),
            HashSet::new(),
        ];

        let e_nfa = EpsilonNfa::build(transition_table, HashSet::from([2]), 2, 1).unwrap();
        assert!(e_nfa.accepts_validated(&[0, 1, 1, 1]));
        assert!(e_nfa.accepts_validated(&[0, 0, 1, 1]));
        assert!(e_nfa.accepts_validated(&[0, 1, 0, 1, 0]));
        assert!(e_nfa.accepts_validated(&[0, 0, 0]));
        assert!(e_nfa.accepts_validated(&[0, 1, 1, 1]));

        assert!(!e_nfa.accepts_validated(&[1, 1, 1, 1]));
        assert!(!e_nfa.accepts_validated(&[]));
        assert!(!e_nfa.accepts_validated(&[1, 0, 0, 1]));

        let possibilities = HashSet::from([
            vec![
                (0, vec![TapeMovement::Stay(None)]),
                (1, vec![TapeMovement::Right(None)]),
                (1, vec![TapeMovement::Right(None)]),
                (2, vec![TapeMovement::Stay(None)]),
            ],
            vec![
                (0, vec![TapeMovement::Stay(None)]),
                (1, vec![TapeMovement::Right(None)]),
                (2, vec![TapeMovement::Stay(None)]),
                (2, vec![TapeMovement::Right(None)]),
            ],
        ]);
        //TODO fix bug in this test
        let path = e_nfa.trace_states_validated(&[0, 1]);
        assert!(
            possibilities.contains(&path),
            "Path = {:?}\n not found in the possibilities set = {:?}",
            path,
            possibilities
        );
    }

    #[test]
    // Nfa accpets (012)* (021)*
    fn med_nfa() {
        let transition_table = vec![
            // State 0
            HashSet::from([1]),
            HashSet::new(),
            HashSet::new(),
            HashSet::from([4]),
            // State 1
            HashSet::new(),
            HashSet::from([2]),
            HashSet::new(),
            HashSet::new(),
            // State 2
            HashSet::new(),
            HashSet::new(),
            HashSet::from([3]),
            HashSet::new(),
            // State 3
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            HashSet::from([0, 4]),
            // State 4
            HashSet::from([5]),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            // State 5
            HashSet::new(),
            HashSet::new(),
            HashSet::from([6]),
            HashSet::new(),
            // State 6
            HashSet::new(),
            HashSet::from([7]),
            HashSet::new(),
            HashSet::new(),
            // State 7
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            HashSet::from([4]),
        ];

        let e_nfa = EpsilonNfa::build(transition_table, HashSet::from([4, 7]), 7, 2).unwrap();

        assert!(e_nfa.accepts_validated(&[]));
        assert!(e_nfa.accepts_validated(&[0, 1, 2]));
        assert!(e_nfa.accepts_validated(&[0, 1, 2, 0, 1, 2]));
        assert!(e_nfa.accepts_validated(&[0, 2, 1]));
        assert!(e_nfa.accepts_validated(&[0, 1, 2, 0, 2, 1]));
        assert!(e_nfa.accepts_validated(&[0, 1, 2, 0, 1, 2, 0, 2, 1]));
        assert!(e_nfa.accepts_validated(&[0, 1, 2, 0, 1, 2, 0, 2, 1, 0, 2, 1]));
        assert!(e_nfa.accepts_validated(&[0, 1, 2, 0, 2, 1, 0, 2, 1, 0, 2, 1]));

        assert!(!e_nfa.accepts_validated(&[0]));
        assert!(!e_nfa.accepts_validated(&[0, 1]));
        assert!(!e_nfa.accepts_validated(&[0, 1, 2, 0]));
        assert!(!e_nfa.accepts_validated(&[0, 2, 1, 0, 1, 2]));
        assert!(!e_nfa.accepts_validated(&[0, 2]));
        assert!(!e_nfa.accepts_validated(&[0, 2, 1, 0, 1]));

        let options = HashSet::from([
            vec![
                (0, vec![TapeMovement::Stay(None)]),
                (1, vec![TapeMovement::Right(None)]),
                (2, vec![TapeMovement::Right(None)]),
                (3, vec![TapeMovement::Right(None)]),
            ],
            vec![
                (0, vec![TapeMovement::Stay(None)]),
                (1, vec![TapeMovement::Right(None)]),
                (2, vec![TapeMovement::Right(None)]),
                (3, vec![TapeMovement::Right(None)]),
                (4, vec![TapeMovement::Stay(None)]),
            ],
            vec![
                (0, vec![TapeMovement::Stay(None)]),
                (1, vec![TapeMovement::Right(None)]),
                (2, vec![TapeMovement::Right(None)]),
                (3, vec![TapeMovement::Right(None)]),
                (0, vec![TapeMovement::Stay(None)]),
                (4, vec![TapeMovement::Stay(None)]),
            ],
        ]);
        assert!(options.contains(&e_nfa.trace_states_validated(&[0, 1, 2])));
    }
}
