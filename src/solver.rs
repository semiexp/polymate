use super::*;

// just counting # of answers
pub fn solve(problem: &Puzzle) -> Answers {
    let use_bitset = (problem.target.volume() > 64);
    let mut answers;

    if use_bitset {
        let mut dic = Dictionary::<Bitset>::new(problem);

        let mut answer_raw = vec![(-1, -1); dic.n_target_cells as usize];
        answers = Answers::new();
        
        for i in 0..dic.initial_piece_count.len() {
            let mut rem_piece = dic.initial_piece_count[i].clone();
            dic.target_symmetry = dic.initial_symmetry[i];

            for &(cell, piece, ori) in &dic.initial_placement_id[i] {
                answer_raw[cell as usize] = (piece, ori);
            }
            
            let mut pl = dic.initial_placement[i].clone();
            search_generic(&dic, &mut rem_piece, &mut answer_raw, &mut pl, &mut answers);

            for &(cell, piece, ori) in &dic.initial_placement_id[i] {
                answer_raw[cell as usize] = (-1, -1);
            }
        }
    } else {
        let mut dic = Dictionary::<u64>::new(problem);

        let mut answer_raw = vec![(-1, -1); dic.n_target_cells as usize];
        answers = Answers::new();
        
        for i in 0..dic.initial_piece_count.len() {
            let mut rem_piece = dic.initial_piece_count[i].clone();
            dic.target_symmetry = dic.initial_symmetry[i];

            for &(cell, piece, ori) in &dic.initial_placement_id[i] {
                answer_raw[cell as usize] = (piece, ori);
            }
            
            search(&dic, &mut rem_piece, &mut answer_raw, dic.initial_placement[i], &mut answers);

            for &(cell, piece, ori) in &dic.initial_placement_id[i] {
                answer_raw[cell as usize] = (-1, -1);
            }
        }
    }

    answers
}

fn search(dic: &Dictionary<u64>, rem_piece: &mut Vec<i32>, answer_raw: &mut Vec<(i32, i32)>, mask: u64, answers: &mut Answers) {
    let pos = (!mask).trailing_zeros() as i32;

    if pos == dic.n_target_cells {
        save_answer(dic, rem_piece, answer_raw, answers);
        return;
    }

    if dic.isolated_cell_pruning {
        if (!mask
         & ((mask << dic.isolated_cell_pruning_x_ofs) | dic.isolated_cell_pruning_x_mask_lo)
         & ((mask >> dic.isolated_cell_pruning_x_ofs) | dic.isolated_cell_pruning_x_mask_hi)
         & ((mask << dic.isolated_cell_pruning_y_ofs) | dic.isolated_cell_pruning_y_mask_lo)
         & ((mask >> dic.isolated_cell_pruning_y_ofs) | dic.isolated_cell_pruning_y_mask_hi)
         & ((mask << dic.isolated_cell_pruning_z_ofs) | dic.isolated_cell_pruning_z_mask_lo)
         & ((mask >> dic.isolated_cell_pruning_z_ofs) | dic.isolated_cell_pruning_z_mask_hi)
        ) != 0 {
            return;
        }
    }
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
