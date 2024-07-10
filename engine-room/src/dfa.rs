use std::collections::HashSet;

#[cfg(test)]
mod dfa_tests;
pub struct Dfa {
    transition_table: Vec<usize>,
    accept_states: HashSet<usize>,
    // max_state: usize,
    max_char: usize,
}

impl Dfa {
    pub fn build(
        transition_table: Vec<usize>,
        accept_states: HashSet<usize>,
        max_state: usize,
        max_char: usize,
    ) -> Result<Dfa, ()> {
        if transition_table.len() != (max_state + 1) * (max_char + 1) {
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
            // max_state,
            max_char,
        })
    }

    pub fn accepts(&self, input: &Vec<usize>) -> Result<bool, ()> {
        self.validate_input(input)?;
        Ok(self.accept_states.contains(
            &self
                .states(&input)
                .last()
                .expect("The first state will always be visited no matter the input"),
        ))
    }

    pub fn state_trace(&self, input: &Vec<usize>) -> Result<Vec<usize>, ()> {
        self.validate_input(input)?;

        Ok(self.states(input).collect())
    }

    fn validate_input(&self, input: &Vec<usize>) -> Result<(), ()> {
        if input.iter().any(|c| c > &self.max_char) {
            return Err(());
        }
        Ok(())
    }

    fn states<'a>(&'a self, input: &'a Vec<usize>) -> impl Iterator<Item = usize> + 'a {
        let mut state = 0;
        Some(0).into_iter().chain(input.iter().map(move |c| {
            state = self.next_state(state, *c);
            state
        }))
    }

    // TODO convert to vec<u8> and use max_state to determine # of bytes
    fn next_state(&self, cur_state: usize, char: usize) -> usize {
        self.transition_table[cur_state * (self.max_char + 1) + char]
    }
}
