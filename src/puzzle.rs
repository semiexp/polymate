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

#[derive(Clone, Copy)]
pub struct Rotation {
    origin: [i32; 3],
}

impl Rotation {
    pub fn id() -> Rotation {
        Rotation {
            origin: [0, 1, 2]
        }
    }
    pub fn flip_x(&self) -> Rotation {
        Rotation {
            origin: [
                !self.origin[0],
                self.origin[1],
                self.origin[2],
            ]
        }
    }
    pub fn flip_y(&self) -> Rotation {
        Rotation {
            origin: [
                self.origin[0],
                !self.origin[1],
                self.origin[2],
            ]
        }
    }
    pub fn flip_z(&self) -> Rotation {
        Rotation {
            origin: [
                self.origin[0],
                self.origin[1],
                !self.origin[2],
            ]
        }
    }
    pub fn rotate_x_axis(&self) -> Rotation {
        Rotation {
            origin: [
                self.origin[0],
                self.origin[2],
                !self.origin[1],
            ]
        }
    }
    pub fn rotate_y_axis(&self) -> Rotation {
        Rotation {
            origin: [
                !self.origin[2],
                self.origin[1],
                self.origin[0],
            ]
        }
    }
    pub fn rotate_z_axis(&self) -> Rotation {
        Rotation {
            origin: [
                self.origin[1],
                !self.origin[0],
                self.origin[2],
            ]
        }
    }
}

pub const ROTATIONS: [Rotation; 24] = [
    Rotation { origin: [0, 1, 2] },
    Rotation { origin: [0, !1, !2] },
    Rotation { origin: [!0, !1, 2] },
    Rotation { origin: [!0, 1, !2] },
    Rotation { origin: [1, 2, 0] },
    Rotation { origin: [1, !2, !0] },
    Rotation { origin: [!1, !2, 0] },
    Rotation { origin: [!1, 2, !0] },
    Rotation { origin: [2, 0, 1] },
    Rotation { origin: [2, !0, !1] },
    Rotation { origin: [!2, !0, 1] },
    Rotation { origin: [!2, 0, !1] },
    Rotation { origin: [0, 2, !1] },
    Rotation { origin: [0, !2, 1] },
    Rotation { origin: [!0, 2, 1] },
    Rotation { origin: [!0, !2, !1] },
    Rotation { origin: [1, 0, !2] },
    Rotation { origin: [1, !0, 2] },
    Rotation { origin: [!1, 0, 2] },
    Rotation { origin: [!1, !0, !2] },
    Rotation { origin: [2, 1, !0] },
    Rotation { origin: [2, !1, 0] },
    Rotation { origin: [!2, 1, 0] },
    Rotation { origin: [!2, !1, !0] },
];

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
        let current_dim = [size.x, size.y, size.z];

        let new_dim = [
            current_dim[if rot.origin[0] >= 0 { rot.origin[0] } else { (!rot.origin[0]) } as usize],
            current_dim[if rot.origin[1] >= 0 { rot.origin[1] } else { (!rot.origin[1]) } as usize],
            current_dim[if rot.origin[2] >= 0 { rot.origin[2] } else { (!rot.origin[2]) } as usize],
        ];

        let mut ret = Shape::new(Coord { x: new_dim[0], y: new_dim[1], z: new_dim[2] });
        for cd in size {
            let pos = [cd.x, cd.y, cd.z];
            let new_pos = Coord {
                x: if rot.origin[0] >= 0 { pos[rot.origin[0] as usize] } else { new_dim[0] - pos[!rot.origin[0] as usize] - 1 },
                y: if rot.origin[1] >= 0 { pos[rot.origin[1] as usize] } else { new_dim[1] - pos[!rot.origin[1] as usize] - 1 },
                z: if rot.origin[2] >= 0 { pos[rot.origin[2] as usize] } else { new_dim[2] - pos[!rot.origin[2] as usize] - 1 },
            };
            ret.set(new_pos, self.get(cd));
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
