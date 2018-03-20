use super::*;

use std::ops::{Index, IndexMut};

pub const UNFILLED: (i32, i32) = (-1, -1);
pub const BLOCKED: (i32, i32) = (-2, -2);

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Answer {
    size: Coord,
    data: Vec<(i32, i32)>, // piece type, number in the piece
}

impl Answer {
    pub fn new(size: Coord) -> Answer {
        Answer {
            size,
            data: vec![UNFILLED; (size.x * size.y * size.z) as usize],
        }
    }
    pub fn from_answer<T: Bits>(dic: &Dictionary<T>, answer_raw: &Vec<(i32, i32)>) -> Answer {
        let mut n_piece_used = vec![0; dic.piece_count.len()];
        let mut ret = Answer::new(dic.target.size());

        for i in 0..dic.n_target_cells {
            let (piece, ori) = answer_raw[i as usize];
            if piece == -1 { continue; }

            let mut locs = dic.placements[i as usize][piece as usize][ori as usize].clone();
            let pval = (piece, n_piece_used[piece as usize]);
            n_piece_used[piece as usize] += 1;

            while !locs.is_empty() {
                let j = locs.lowest_set_bit();
                locs.unset(j);

                ret[dic.id_to_coord[j as usize]] = pval;
            }
        }

        ret
    }
    pub fn size(&self) -> Coord {
        self.size
    }
    fn coord(&self, c: Coord) -> usize {
        ((c.x * self.size.y + c.y) * self.size.z + c.z) as usize
    }
    pub fn trans(&self, trans: Transformation) -> Answer {
        let mut ret = Answer::new(trans.trans_rect(self.size));
        for cd in self.size {
            ret[trans.trans_point(cd, self.size)] = self[cd];
        }
        ret
    }
    pub fn mirror(&mut self, mirror_pair: &Vec<i32>) {
        for d in &mut self.data {
            d.0 = mirror_pair[d.0 as usize];
        }
    }
    pub fn reindex(&mut self, total_piece_count: &Vec<i32>, rem_piece: &Vec<i32>) {
        let mut ofs = vec![0; total_piece_count.len()];
        for i in 1..total_piece_count.len() {
            ofs[i] = ofs[i - 1] + (total_piece_count[i - 1] - rem_piece[i - 1]);
        }
        let total_pieces_used = ofs[ofs.len() - 1] + (total_piece_count[total_piece_count.len() - 1] - rem_piece[total_piece_count.len() - 1]);
        let mut piece_idx = vec![0; total_piece_count.len()];
        let mut new_idx = vec![-1; total_pieces_used as usize];

        for pn in &mut self.data {
            let p = pn.0 as usize;
            let idx_orig = (ofs[p] + pn.1) as usize;
            if new_idx[idx_orig] == -1 {
                new_idx[idx_orig] = piece_idx[p];
                piece_idx[p] += 1;
            }
            let n2 = new_idx[idx_orig];
            pn.1 = n2;
        }
    }
}

impl Index<Coord> for Answer {
    type Output = (i32, i32);

    fn index(&self, idx: Coord) -> &(i32, i32) {
        &self.data[self.coord(idx)]
    }
}

impl IndexMut<Coord> for Answer {
    fn index_mut(&mut self, idx: Coord) -> &mut (i32, i32) {
        let i = self.coord(idx);
        &mut self.data[i]
    }
}

pub struct Answers {
    pub answer: Vec<Answer>,
    pub count: u64,
    pub save_limit: Option<usize>,
    pub search_steps: u64,
}

impl Answers {
    pub fn new() -> Answers {
        Answers {
            answer: vec![],
            count: 0u64,
            save_limit: None,
            search_steps: 0u64,
        }
    }
}
