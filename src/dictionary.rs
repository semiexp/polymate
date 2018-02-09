use super::*;

pub struct Dictionary {
    pub n_target_cells: i32,
    pub piece_count: Vec<i32>,
    pub placements: Vec<Vec<Vec<u64>>>, // cell, piece, orientation
}

impl Dictionary {
    pub fn new(problem: &Puzzle) -> Dictionary {
        let n_pieces = problem.pieces.len();

        let target = &problem.target;
        let target_size = target.size();

        let mut n_target_cells = 0;
        for x in 0..target_size.x {
            for y in 0..target_size.y {
                for z in 0..target_size.z {
                    if target.get(Coord { x, y, z }) {
                        n_target_cells += 1;
                    }
                }
            }
        }

        if n_target_cells > 64 { unimplemented!(); }

        let mut placements = vec![vec![vec![]; n_pieces as usize]; n_target_cells as usize];

        for i in 0..n_pieces {
            let piece = &problem.pieces[i].0;

            // compute unique rotation patterns
            let mut rots = vec![];
            for rot in &ROTATIONS {
                let piece_rot = piece.rotate(rot);

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

                for x in 0..(target_size.x - p_size.x + 1) {
                    for y in 0..(target_size.y - p_size.y + 1) {
                        for z in 0..(target_size.z - p_size.z + 1) {
                            let offset = Coord { x, y, z };
                            if target.is_fit(p, offset) {
                                let mask = target.get_piece_mask(p, offset);
                                let handle = mask.trailing_zeros();

                                placements[handle as usize][i].push(mask);
                            }
                        }
                    }
                }
            }
        }

        Dictionary {
            n_target_cells,
            piece_count: vec![],
            placements,
        }
    }
}
