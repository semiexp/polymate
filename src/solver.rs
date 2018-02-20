use super::*;

// just counting # of answers
pub fn solve(problem: &Puzzle) -> Answers {
    let mut dic = Dictionary::new(problem);

    let mut rem_piece = dic.piece_count.clone();
    let mut answer_raw = vec![(-1, -1); dic.n_target_cells as usize];

    let mut answers = Answers::new();
    
    for i in 0..dic.initial_piece_count.len() {
        rem_piece = dic.initial_piece_count[i].clone();
        dic.target_symmetry = dic.initial_symmetry[i];

        for &(cell, piece, ori) in &dic.initial_placement_id[i] {
            answer_raw[cell as usize] = (piece, ori);
        }
        
        search(&dic, &mut rem_piece, &mut answer_raw, dic.initial_placement[i], &mut answers);

        for &(cell, piece, ori) in &dic.initial_placement_id[i] {
            answer_raw[cell as usize] = (piece, ori);
        }
    }

    answers
}

fn search(dic: &Dictionary, rem_piece: &mut Vec<i32>, answer_raw: &mut Vec<(i32, i32)>, mask: u64, answers: &mut Answers) {
    let pos = (!mask).trailing_zeros() as i32;

    if pos == dic.n_target_cells {
        // check for uniqueness
        let answer = Answer::from_answer(dic, answer_raw);

        for i in 1..24 {
            if (dic.target_symmetry & (1u64 << i)) != 0 {
                let answer_rot = answer.trans(ROTATIONS[i]);
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
        return;
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
