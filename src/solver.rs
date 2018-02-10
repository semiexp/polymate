use super::*;

// just counting # of answers
pub fn solve(problem: &Puzzle) -> u64 {
    let dic = Dictionary::new(problem);

    let mut rem_piece = dic.piece_count.clone();

    search(&dic, &mut rem_piece, 0, 0u64)
}

fn search(dic: &Dictionary, rem_piece: &mut Vec<i32>, pos: i32, mask: u64) -> u64 {
    let mut pos = pos;
    while pos < dic.n_target_cells && ((mask >> (pos as u64)) & 1) == 1 {
        pos += 1;
    }

    if pos == dic.n_target_cells {
        return 1u64;
    }

    let mut ret = 0u64;

    for i in 0..rem_piece.len() {
        if rem_piece[i] > 0 {
            rem_piece[i] -= 1;
            for m in &dic.placements[pos as usize][i] {
                if (mask & m) == 0 {
                    ret += search(dic, rem_piece, pos + 1, mask | m);
                }
            }
            rem_piece[i] += 1;
        }
    }

    ret
}