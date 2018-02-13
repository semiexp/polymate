use super::*;

use std::ops::{Add, Sub};
use std::iter::IntoIterator;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Add<Coord> for Coord {
    type Output = Coord;
    fn add(self, other: Coord) -> Coord {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub<Coord> for Coord {
    type Output = Coord;
    fn sub(self, other: Coord) -> Coord {
        Coord {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl IntoIterator for Coord {
    type Item = Coord;
    type IntoIter = CoordIterator;

    fn into_iter(self) -> CoordIterator {
        CoordIterator {
            current: Coord { x: 0, y: 0, z: 0 },
            limit: self,
        }
    }
}

pub struct CoordIterator {
    current: Coord,
    limit: Coord,
}

impl Iterator for CoordIterator {
    type Item = Coord;

    fn next(&mut self) -> Option<Coord> {
        if self.current.x >= self.limit.x {
            None
        } else {
            let ret = self.current;
            self.current.z += 1;
            if self.current.z >= self.limit.z {
                self.current.z -= self.limit.z;
                self.current.y += 1;
                if self.current.y >= self.limit.y {
                    self.current.y -= self.limit.y;
                    self.current.x += 1;
                }
            }
            Some(ret)
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct Shape {
    size: Coord,
    data: Vec<bool>,
}

impl Shape {
    pub fn new(size: Coord) -> Shape {
        Shape {
            size,
            data: vec![false; (size.x * size.y * size.z) as usize],
        }
    }
    pub fn filled(size: Coord) -> Shape {
        Shape {
            size,
            data: vec![true; (size.x * size.y * size.z) as usize],
        }
    }
    pub fn from_grid(grid: &[&'static str]) -> Shape {
        let width = grid[0].len() as i32;
        let height = grid.len() as i32;
        let mut ret = Shape::new(Coord { x: width, y: height, z: 1 });

        for y in 0..height {
            let mut it = grid[y as usize].chars();
            for x in 0..width {
                ret.set(Coord { x, y, z: 0 }, it.next() == Some('#'));
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
    pub fn get(&self, c: Coord) -> bool {
        self.data[self.coord(c)]
    }
    pub fn set(&mut self, c: Coord, v: bool) {
        let cd = self.coord(c);
        self.data[cd] = v;
    }
    pub fn rotate(&self, rot: &Rotation) -> Shape {
        let size = self.size;
        let mut ret = Shape::new(rot.rotate_rect(size));

        for cd in size {
            ret.set(rot.rotate_point(cd, size), self.get(cd));
        }

        ret
    }
    pub fn is_fit(&self, piece: &Shape, offset: Coord) -> bool {
        let piece_size = piece.size();
        for cd in piece_size {
            if piece.get(cd) && !self.get(cd + offset) {
                return false;
            }
        }
        true
    }
    pub fn get_piece_mask(&self, piece: &Shape, offset: Coord) -> u64 {
        let piece_size = piece.size();
        let mut counter = 0u64;
        let mut ret = 0u64;
        for cd in self.size {
            if self.get(cd) {
                let piece_cd = cd - offset;
                if 0 <= piece_cd.x && piece_cd.x < piece_size.x && 0 <= piece_cd.y && piece_cd.y < piece_size.y && 0 <= piece_cd.z && piece_cd.z < piece_size.z {
                    if piece.get(piece_cd) {
                        ret |= 1u64 << counter;
                    }
                }
                counter += 1;
            }
        }
        ret
    }
}

pub struct Puzzle {
    pub pieces: Vec<(Shape, i32)>,
    pub target: Shape,
}
