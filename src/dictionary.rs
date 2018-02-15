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
    pub special_piece_id: usize,
    pub special_piece_placements_id: Vec<(i32, i32)>, // specifies the entry index in `placements`
    pub special_piece_symmetry: Vec<Symmetry>, // symmetry after putting the special piece
}

impl Dictionary {
    pub fn new(problem: &Puzzle) -> Dictionary {
        let n_pieces = problem.pieces.len();

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
        let special_piece_id = 0;
        let mut special_piece_placements_id = vec![];
        let mut special_piece_symmetry = vec![];

        for i in 0..(n_target_cells as usize) {
            for j in 0..placements[i][special_piece_id].len() {
                let mut target_with_special = target.clone();
                let mut pl = placements[i][special_piece_id][j];
                while pl != 0 {
                    let id = pl.trailing_zeros();
                    pl ^= 1u64 << id;
                    target_with_special.set(id_to_coord[id as usize], false);
                }

                let mut sym = 1u64;
                let mut isok = true;
                for s in 1..24 {
                    if (target_symmetry & (1u64 << s)) != 0 {
                        let rot_field = target_with_special.trans(ROTATIONS[s]);
                        match target_with_special.cmp(&rot_field) {
                            Ordering::Less => (),
                            Ordering::Equal => sym |= 1u64 << s,
                            Ordering::Greater => {
                                isok = false;
                                break;
                            },
                        }
                    }
                }

                if isok {
                    special_piece_placements_id.push((i as i32, j as i32));
                    special_piece_symmetry.push(sym);
                }
            }
        }

        let piece_count = problem.pieces.iter().map(|&(_, c)| c).collect::<Vec<i32>>();
        Dictionary {
            n_target_cells,
            piece_count,
            placements,
            target: target.clone(),
            target_symmetry,
            id_to_coord,
            special_piece_id,
            special_piece_placements_id,
            special_piece_symmetry,
        }
    }
}
