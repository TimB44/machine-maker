use crate::TapeMovement;
use std::iter;

// TODO: Add better error type
pub(crate) fn validate_input(input: &[u16], max_char: u16) -> Result<(), ()> {
    if input.iter().any(|c| c > &max_char) {
        return Err(());
    }
    Ok(())
}

pub(crate) fn table_lookup(cur_state: usize, cur_char: usize, max_char: usize) -> usize {
    cur_state * (max_char + 1) + cur_char
}

pub(crate) fn add_tape_mov(state_trace: Vec<u16>) -> Vec<(u16, Vec<TapeMovement>)> {
    state_trace
        .into_iter()
        .zip(iter::repeat(vec![TapeMovement::Right(None)]))
        .collect()
}
