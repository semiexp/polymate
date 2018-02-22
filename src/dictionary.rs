use super::*;

use std::cmp::Ordering;

pub struct Dictionary {
    pub n_target_cells: i32,
    pub piece_count: Vec<i32>,
    pub placements: Vec<Vec<Vec<u64>>>, // cell, piece, orientation
    pub target: Shape,
    pub target_symmetry: Symmetry,
    pub id_to_coord: Vec<Coord>,

    // about the special piece for uniqueneess
    pub initial_piece_count: Vec<Vec<i32>>,
    pub initial_placement: Vec<u64>,
    pub initial_placement_id: Vec<Vec<(i32, i32, i32)>>, // cell, piece, orientation
    pub initial_symmetry: Vec<Symmetry>,

    pub mirror_pair: Vec<i32>,
}

impl Dictionary {
    pub fn new(problem: &Puzzle) -> Dictionary {
        let n_pieces = problem.pieces.len();

        let piece_count = problem.pieces.iter().map(|&(_, c)| c).collect::<Vec<i32>>();

        let target = &problem.target;
        let target_size = target.size();
        let target_symmetry = target.symmetry();

        let mut n_target_cells = 0;
        for cd in target_size {
            if target.get(cd) {
                n_target_cells += 1;
            }
        }

        if n_target_cells > 64 { unimplemented!(); }

        let mut piece_canonical = vec![];
        for i in 0..n_pieces {
            let p = problem.pieces[i].0.canonize();
            let mp = problem.pieces[i].0.trans(Transformation::id().flip_x()).canonize();

            piece_canonical.push((p, mp));
        }
        let mut mirror_pair = vec![];
        for i in 0..n_pieces {
            let mut pair = -1;
            for j in 0..n_pieces {
                if piece_canonical[i].1 == piece_canonical[j].0 {
                    pair = j as i32;
                    break;
                }
            }
            mirror_pair.push(pair);
        }

        let mut id_to_coord = vec![];
        for cd in target_size {
            if target.get(cd) {
                id_to_coord.push(cd);
            }
        }
        
        let mut placements = vec![vec![vec![]; n_pieces as usize]; n_target_cells as usize];
        
        for i in 0..n_pieces {
            let piece = &problem.pieces[i].0;

            // compute unique rotation patterns
            let mut rots = vec![];
            for &rot in &ROTATIONS {
                let piece_rot = piece.trans(rot);

                // is unique?
                let mut is_unique = true;

                for p in &rots {
                    if *p == piece_rot {
                        is_unique = false;
                        break;
                    }
                }

                if is_unique {
                    rots.push(piece_rot);
                }
            }

            // compute all possible placements
            for p in &rots {
                let p_size = p.size();

                if p_size.x > target_size.x || p_size.y > target_size.y || p_size.z > target_size.z { continue; }

                for offset in (target_size - p_size + Coord { x: 1, y: 1, z: 1 }) {
                    if target.is_fit(p, offset) {
                        let mask = target.get_piece_mask(p, offset);
                        let handle = mask.trailing_zeros();

                        placements[handle as usize][i].push(mask);
                    }
                }
            }
        }

        // handle the special piece
        let mut total_piece_volume = 0;
        for i in 0..n_pieces {
            total_piece_volume += problem.pieces[i].0.volume() * problem.pieces[i].1;
        }
        let use_all_pieces = total_piece_volume == target.volume();

        let mut special_piece_cand = vec![];
        
        if use_all_pieces {
            for i in 0..n_pieces {
                if mirror_pair[i] != i as i32 { continue; }
                if problem.pieces[i].1 > 1 { continue; }

                special_piece_cand.push(i);
                continue;
            }
        }
        let mut special_piece_id = None;

        if use_all_pieces {
            for i in 0..n_pieces {
                // special piece must be achiral
                if mirror_pair[i] != i as i32 {
                    continue;
                }

                // special piece must be used only once
                if problem.pieces[i].1 > 1 {
                    continue;
                }

                special_piece_id = Some(i);
                break;
            }
        }

        while special_piece_cand.len() > 2 {
            special_piece_cand.pop();
        }

        let mut initial_piece_count = vec![];
        let mut initial_placement = vec![];
        let mut initial_placement_id = vec![];
        let mut initial_symmetry = vec![];

        Dictionary::compute_initial_placement(
            0,
            &special_piece_cand,
            &placements,
            &id_to_coord,
            &target,
            &mut piece_count.clone(),
            0u64,
            &mut vec![],
            target_symmetry,
            &mut initial_piece_count,
            &mut initial_placement,
            &mut initial_placement_id,
            &mut initial_symmetry
        );

        Dictionary {
            n_target_cells,
            piece_count,
            placements,
            target: target.clone(),
            target_symmetry,
            id_to_coord,

            initial_piece_count,
            initial_placement,
            initial_placement_id,
            initial_symmetry,

            mirror_pair,
        }
    }

    fn compute_initial_placement(
        idx: usize,
        special_piece_cand: &Vec<usize>,
        placements: &Vec<Vec<Vec<u64>>>,
        id_to_coord: &Vec<Coord>,
        current_target: &Shape,
        current_piece_count: &mut Vec<i32>,
        current_placement: u64,
        current_placement_id: &mut Vec<(i32, i32, i32)>,
        current_symmetry: Symmetry,
        initial_piece_count: &mut Vec<Vec<i32>>,
        initial_placement: &mut Vec<u64>,
        initial_placement_id: &mut Vec<Vec<(i32, i32, i32)>>,
        initial_symmetry: &mut Vec<Symmetry>,
    ) {
        if idx == special_piece_cand.len() || current_symmetry.count_ones() == 1 {
            initial_piece_count.push(current_piece_count.clone());
            initial_placement.push(current_placement);
            initial_placement_id.push(current_placement_id.clone());
            initial_symmetry.push(current_symmetry);
            return;
        }

        let p = special_piece_cand[idx];
        current_piece_count[p] -= 1;

        for i in 0..placements.len() {
            for j in 0..placements[i][p].len() {
                let mut new_target = current_target.clone();
                let mut pl = placements[i][p][j];

                if (current_placement & pl) != 0 { continue; }

                let new_placement = current_placement | pl;

                while pl != 0 {
                    let id = pl.trailing_zeros();
                    pl ^= 1u64 << id;
                    new_target.set(id_to_coord[id as usize], false);
                }

                let mut new_symmetry = 1u64;
                let mut isok = true;
                for s in 1..48 {
                    if (current_symmetry & (1u64 << s)) != 0 {
                        let rot_field = new_target.trans(TRANSFORMATIONS[s]);
                        match new_target.cmp(&rot_field) {
                            Ordering::Less => (),
                            Ordering::Equal => new_symmetry |= 1u64 << s,
                            Ordering::Greater => {
                                isok = false;
                                break;
                            },
                        }
                    }
                }

                if isok {
                    current_placement_id.push((i as i32, p as i32, j as i32));
                    Dictionary::compute_initial_placement(
                        idx + 1,
                        special_piece_cand,
                        placements,
                        id_to_coord,
                        &new_target,
                        current_piece_count,
                        new_placement,
                        current_placement_id,
                        new_symmetry,
                        initial_piece_count,
                        initial_placement,
                        initial_placement_id,
                        initial_symmetry
                    );
                    current_placement_id.pop();
                }
            }
        }

        current_piece_count[p] += 1;
    }
}
