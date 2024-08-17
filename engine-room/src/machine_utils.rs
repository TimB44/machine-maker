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
    debug_assert!(cur_char <= max_char);
    cur_state * (max_char + 1) + cur_char
}

pub(crate) fn add_tape_mov(
    state_trace: Vec<u16>,
    tape_mov: TapeMovement,
) -> Vec<(u16, Vec<TapeMovement>)> {
    state_trace
        .into_iter()
        .zip(iter::repeat(vec![tape_mov]))
        .collect()
}

pub(crate) fn add_tape_mov_stay_fir(
    state_trace: Vec<u16>,
    tape_mov: TapeMovement,
) -> Vec<(u16, Vec<TapeMovement>)> {
    let mut seen_first = false;
    state_trace
        .into_iter()
        .map(|s| {
            if seen_first {
                (s, vec![tape_mov])
            } else {
                seen_first = true;
                (s, vec![TapeMovement::Stay(None)])
            }
        })
        .collect()
}
