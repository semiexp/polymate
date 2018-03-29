use super::*;

// just counting # of answers
pub fn solve(problem: &Puzzle) -> Answers {
    let use_bitset = (problem.target.volume() > 64);

    if use_bitset {
        solve_typed::<Bitset>(problem)
    } else {
        solve_typed::<u64>(problem)
    }
}

fn solve_typed<T: Bits + SearchHandler>(problem: &Puzzle) -> Answers {
    let mut dic = Dictionary::<T>::new(problem);

    let mut answer_raw = vec![(-1, -1); dic.n_target_cells as usize];
    let mut answers = Answers::new();
    
    for i in 0..dic.initial_piece_count.len() {
        let mut rem_piece = dic.initial_piece_count[i].clone();
        dic.target_symmetry = dic.initial_symmetry[i];

        for &(cell, piece, ori) in &dic.initial_placement_id[i] {
            answer_raw[cell as usize] = (piece, ori);
        }
        
        T::search(&dic, &mut rem_piece, &mut answer_raw, dic.initial_placement[i].clone(), &mut answers);

        for &(cell, piece, ori) in &dic.initial_placement_id[i] {
            answer_raw[cell as usize] = (-1, -1);
        }
    }

    answers
}

trait SearchHandler : Bits {
    fn search(dic: &Dictionary<Self>, rem_piece: &mut Vec<i32>, answer_raw: &mut Vec<(i32, i32)>, mask_default: Self, answers: &mut Answers);
}

impl SearchHandler for u64 {
    fn search(dic: &Dictionary<Self>, rem_piece: &mut Vec<i32>, answer_raw: &mut Vec<(i32, i32)>, mask_default: Self, answers: &mut Answers) {
        let mut is_zero_one = true;
        for i in 0..rem_piece.len() {
            if rem_piece[i] > 1 {
                is_zero_one = false;
                break;
            }
        }

        if is_zero_one && rem_piece.len() <= 64 {
            let mut rem_piece_bits = 0u64;
            for i in 0..rem_piece.len() {
                rem_piece_bits |= (rem_piece[i] as u64) << (i as u64);
            }
            search_with_u64_rem_piece(dic, rem_piece_bits, answer_raw, mask_default, answers);
        } else {
            search(dic, rem_piece, answer_raw, mask_default, answers);
        }
    }
}

impl SearchHandler for Bitset {
    fn search(dic: &Dictionary<Self>, rem_piece: &mut Vec<i32>, answer_raw: &mut Vec<(i32, i32)>, mask_default: Self, answers: &mut Answers) {
        let mut mask_default = mask_default;
        search_generic(dic, rem_piece, answer_raw, &mut mask_default, answers);
    }
}

#[derive(Clone, Copy)]
struct PieceCountBitSeq {
    val: u64
}

struct PieceCountBitSeqIter {
    val: u64
}

impl IntoIterator for PieceCountBitSeq {
    type Item = usize;
    type IntoIter = PieceCountBitSeqIter;

    fn into_iter(self) -> PieceCountBitSeqIter {
        PieceCountBitSeqIter { val: self.val }
    }
}

impl Iterator for PieceCountBitSeqIter {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        if self.val != 0 {
            let ret = self.val.trailing_zeros();
            self.val ^= 1u64 << (ret as u64);
            Some(ret as usize)
        } else {
            None
        }
    }
}

fn search_with_u64_rem_piece(dic: &Dictionary<u64>, rem_piece: u64, answer_raw: &mut Vec<(i32, i32)>, mask: u64, answers: &mut Answers) {
    let pos = (!mask).trailing_zeros() as i32;

    if pos == dic.n_target_cells {
        let mut rem_piece_as_vec = vec![0i32; dic.piece_count.len()];
        for i in 0..rem_piece_as_vec.len() {
            rem_piece_as_vec[i] = ((rem_piece >> i) & 1) as i32;
        }
        save_answer(dic, &mut rem_piece_as_vec, answer_raw, answers);
        return;
    }

    if isolated_cell_pruning(dic, mask) { return; }

    let rem_piece_orig = rem_piece;
    let mut rem_piece = rem_piece;
    while rem_piece != 0 {
        let i = rem_piece.trailing_zeros() as usize;
        rem_piece ^= 1u64 << (i as u64);
        let pl = unsafe { dic.placements.get_unchecked(pos as usize).get_unchecked(i) };
        for j in 0..pl.len() {
            let m = unsafe { *pl.get_unchecked(j) };
            answers.search_steps += 1;
            if (mask & m) == 0 {
                unsafe { *answer_raw.get_unchecked_mut(pos as usize) = (i as i32, j as i32); }
                search_with_u64_rem_piece(dic, rem_piece_orig ^ (1u64 << (i as u64)), answer_raw, mask | m, answers);
            }
        }
    }

    unsafe { *answer_raw.get_unchecked_mut(pos as usize) = (-1, -1); }
}

fn search(dic: &Dictionary<u64>, rem_piece: &mut Vec<i32>, answer_raw: &mut Vec<(i32, i32)>, mask: u64, answers: &mut Answers) {
    let pos = (!mask).trailing_zeros() as i32;

    if pos == dic.n_target_cells {
        save_answer(dic, rem_piece, answer_raw, answers);
        return;
    }

    if isolated_cell_pruning(dic, mask) { return; }

    for i in 0..rem_piece.len() {
        if unsafe { *rem_piece.get_unchecked(i) } > 0 {
            unsafe { *rem_piece.get_unchecked_mut(i) -= 1 };
            let pl = unsafe { dic.placements.get_unchecked(pos as usize).get_unchecked(i) };
            for j in 0..pl.len() {
                let m = unsafe { *pl.get_unchecked(j) };
                answers.search_steps += 1;
                if (mask & m) == 0 {
                    unsafe { *answer_raw.get_unchecked_mut(pos as usize) = (i as i32, j as i32); }
                    search(dic, rem_piece, answer_raw, mask | m, answers);
                }
            }
            unsafe { *rem_piece.get_unchecked_mut(i) += 1 };
        }
    }

    unsafe { *answer_raw.get_unchecked_mut(pos as usize) = (-1, -1); }
}

fn isolated_cell_pruning(dic: &Dictionary<u64>, mask: u64) -> bool {
    dic.isolated_cell_pruning &&
           (!mask
         & ((mask << dic.isolated_cell_pruning_x_ofs) | dic.isolated_cell_pruning_x_mask_lo)
         & ((mask >> dic.isolated_cell_pruning_x_ofs) | dic.isolated_cell_pruning_x_mask_hi)
         & ((mask << dic.isolated_cell_pruning_y_ofs) | dic.isolated_cell_pruning_y_mask_lo)
         & ((mask >> dic.isolated_cell_pruning_y_ofs) | dic.isolated_cell_pruning_y_mask_hi)
         & ((mask << dic.isolated_cell_pruning_z_ofs) | dic.isolated_cell_pruning_z_mask_lo)
         & ((mask >> dic.isolated_cell_pruning_z_ofs) | dic.isolated_cell_pruning_z_mask_hi)
        ) != 0
}

fn search_generic<T: Bits>(dic: &Dictionary<T>, rem_piece: &mut Vec<i32>, answer_raw: &mut Vec<(i32, i32)>, mask: &mut T, answers: &mut Answers) {
    let pos = mask.lowest_unset_bit();

    if pos == dic.n_target_cells {
        save_answer(dic, rem_piece, answer_raw, answers);
        return;
    }

    for i in 0..rem_piece.len() {
        if unsafe { *rem_piece.get_unchecked(i) } > 0 {
            unsafe { *rem_piece.get_unchecked_mut(i) -= 1 };
            let pl = unsafe { dic.placements.get_unchecked(pos as usize).get_unchecked(i) };
            for j in 0..pl.len() {
                let m = unsafe { pl.get_unchecked(j) };
                answers.search_steps += 1;
                if mask.disjoint(m) {
                    unsafe { *answer_raw.get_unchecked_mut(pos as usize) = (i as i32, j as i32); }
                    mask.update(m);
                    search_generic(dic, rem_piece, answer_raw, mask, answers);
                    mask.update(m);
                }
            }
            unsafe { *rem_piece.get_unchecked_mut(i) += 1 };
        }
    }

    unsafe { *answer_raw.get_unchecked_mut(pos as usize) = (-1, -1); }
}

fn save_answer<T: Bits>(dic: &Dictionary<T>, rem_piece: &mut Vec<i32>, answer_raw: &mut Vec<(i32, i32)>, answers: &mut Answers) {
    // check for uniqueness
    let answer = Answer::from_answer(dic, answer_raw);

    for i in 1..24 {
        if (dic.target_symmetry & (1u64 << i)) != 0 {
            let mut answer_rot = answer.trans(ROTATIONS[i]);
            answer_rot.reindex(&dic.piece_count, rem_piece);
            if answer > answer_rot {
                return;
            }
        }
    }

    // check for mirror flips?
    let mut is_mirror_ok = true;
    for i in 0..rem_piece.len() {
        let n_used = dic.piece_count[i] - rem_piece[i];
        if dic.mirror_pair[i] == -1 || dic.piece_count[dic.mirror_pair[i] as usize] < n_used {
            is_mirror_ok = false;
            break;
        }
    }

    if is_mirror_ok {
        for i in 24..48 {
            if (dic.target_symmetry & (1u64 << i)) != 0 {
                let mut answer_rot = answer.trans(TRANSFORMATIONS[i]);
                answer_rot.mirror(&dic.mirror_pair);
                answer_rot.reindex(&dic.piece_count, rem_piece);
                if answer > answer_rot {
                    return;
                }
            }
        }
    }

    // save answer
    answers.count += 1;

    let save = match answers.save_limit {
        Some(lim) => answers.count <= lim as u64,
        None => true,
    };
    if save {
        answers.answer.push(answer);
    }
}
