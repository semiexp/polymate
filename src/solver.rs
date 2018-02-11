use super::*;

// just counting # of answers
pub fn solve(problem: &Puzzle) -> u64 {
    let dic = Dictionary::new(problem);

    let mut rem_piece = dic.piece_count.clone();
    let mut answer_raw = vec![(-1, -1); dic.n_target_cells as usize];

    search(&dic, &mut rem_piece, &mut answer_raw, 0u64)
}

fn search(dic: &Dictionary, rem_piece: &mut Vec<i32>, answer_raw: &mut Vec<(i32, i32)>, mask: u64) -> u64 {
    let pos = (!mask).trailing_zeros() as i32;

    if pos == dic.n_target_cells {
        return 1u64;
    }

    let mut ret = 0u64;

    for i in 0..rem_piece.len() {
        if unsafe { *rem_piece.get_unchecked(i) } > 0 {
            unsafe { *rem_piece.get_unchecked_mut(i) -= 1 };
            let pl = unsafe { dic.placements.get_unchecked(pos as usize).get_unchecked(i) };
            for j in 0..pl.len() {
                let m = unsafe { *pl.get_unchecked(j) };
                if (mask & m) == 0 {
                    unsafe { *answer_raw.get_unchecked_mut(pos as usize) = (i as i32, j as i32); }
                    ret += search(dic, rem_piece, answer_raw, mask | m);
                }
            }
            unsafe { *rem_piece.get_unchecked_mut(i) += 1 };
        }
    }

    unsafe { *answer_raw.get_unchecked_mut(pos as usize) = (-1, -1); }
    ret
}
