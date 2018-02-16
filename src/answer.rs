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
    pub fn from_answer(dic: &Dictionary, answer_raw: &Vec<(i32, i32)>) -> Answer {
        let mut n_piece_used = vec![0; dic.piece_count.len()];
        let mut ret = Answer::new(dic.target.size());

        for i in 0..dic.n_target_cells {
            let (piece, ori) = answer_raw[i as usize];
            if piece == -1 { continue; }

            let mut locs = dic.placements[i as usize][piece as usize][ori as usize];
            let pval = (piece, n_piece_used[piece as usize]);
            n_piece_used[piece as usize] += 1;

            while locs != 0 {
                let j = locs.trailing_zeros();
                locs ^= 1u64 << (j as u64);

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
}

impl Answers {
    pub fn new() -> Answers {
        Answers {
            answer: vec![],
            count: 0u64,
            save_limit: None,
        }
    }
}
