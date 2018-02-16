use super::*;

#[derive(Clone, Copy)]
pub struct Transformation {
    origin: [i32; 3],
}

impl Transformation {
    pub fn id() -> Transformation {
        Transformation {
            origin: [0, 1, 2]
        }
    }
    pub fn trans_rect(&self, size: Coord) -> Coord {
        let dim = [size.x, size.y, size.z];
        Coord {
            x: dim[if self.origin[0] >= 0 { self.origin[0] } else { (!self.origin[0]) } as usize],
            y: dim[if self.origin[1] >= 0 { self.origin[1] } else { (!self.origin[1]) } as usize],
            z: dim[if self.origin[2] >= 0 { self.origin[2] } else { (!self.origin[2]) } as usize],
        }
    }
    pub fn trans_point(&self, p: Coord, rect: Coord) -> Coord {
        let dim = [rect.x, rect.y, rect.z];
        let pt = [p.x, p.y, p.z];
        Coord {
            x: if self.origin[0] >= 0 { pt[self.origin[0] as usize] } else { dim[!self.origin[0] as usize] - pt[!self.origin[0] as usize] - 1 },
            y: if self.origin[1] >= 0 { pt[self.origin[1] as usize] } else { dim[!self.origin[1] as usize] - pt[!self.origin[1] as usize] - 1 },
            z: if self.origin[2] >= 0 { pt[self.origin[2] as usize] } else { dim[!self.origin[2] as usize] - pt[!self.origin[2] as usize] - 1 },
        }
    }
    pub fn flip_x(&self) -> Transformation {
        Transformation {
            origin: [
                !self.origin[0],
                self.origin[1],
                self.origin[2],
            ]
        }
    }
    pub fn flip_y(&self) -> Transformation {
        Transformation {
            origin: [
                self.origin[0],
                !self.origin[1],
                self.origin[2],
            ]
        }
    }
    pub fn flip_z(&self) -> Transformation {
        Transformation {
            origin: [
                self.origin[0],
                self.origin[1],
                !self.origin[2],
            ]
        }
    }
    pub fn rotate_x_axis(&self) -> Transformation {
        Transformation {
            origin: [
                self.origin[0],
                self.origin[2],
                !self.origin[1],
            ]
        }
    }
    pub fn rotate_y_axis(&self) -> Transformation {
        Transformation {
            origin: [
                !self.origin[2],
                self.origin[1],
                self.origin[0],
            ]
        }
    }
    pub fn rotate_z_axis(&self) -> Transformation {
        Transformation {
            origin: [
                self.origin[1],
                !self.origin[0],
                self.origin[2],
            ]
        }
    }

    /// Compose two transformations.
    /// 
    /// It is guaranteed that, for any `r`, `s`: `Rotation` and `p`: `Coord`,
    /// `r.compose(s).rotate_rect(p) == r.rotate_rect(s.rotate_rect(p))`
    /// holds (similar for `rotate_point`).
    pub fn compose(self, other: Transformation) -> Transformation {
        Transformation {
            origin: [
                if self.origin[0] >= 0 { other.origin[self.origin[0] as usize] } else { !other.origin[!self.origin[0] as usize] },
                if self.origin[1] >= 0 { other.origin[self.origin[1] as usize] } else { !other.origin[!self.origin[1] as usize] },
                if self.origin[2] >= 0 { other.origin[self.origin[2] as usize] } else { !other.origin[!self.origin[2] as usize] },
            ]
        }
    }
}

pub const ROTATIONS: [Transformation; 24] = [
    Transformation { origin: [0, 1, 2] },
    Transformation { origin: [0, !1, !2] },
    Transformation { origin: [!0, !1, 2] },
    Transformation { origin: [!0, 1, !2] },
    Transformation { origin: [1, 2, 0] },
    Transformation { origin: [1, !2, !0] },
    Transformation { origin: [!1, !2, 0] },
    Transformation { origin: [!1, 2, !0] },
    Transformation { origin: [2, 0, 1] },
    Transformation { origin: [2, !0, !1] },
    Transformation { origin: [!2, !0, 1] },
    Transformation { origin: [!2, 0, !1] },
    Transformation { origin: [0, 2, !1] },
    Transformation { origin: [0, !2, 1] },
    Transformation { origin: [!0, 2, 1] },
    Transformation { origin: [!0, !2, !1] },
    Transformation { origin: [1, 0, !2] },
    Transformation { origin: [1, !0, 2] },
    Transformation { origin: [!1, 0, 2] },
    Transformation { origin: [!1, !0, !2] },
    Transformation { origin: [2, 1, !0] },
    Transformation { origin: [2, !1, 0] },
    Transformation { origin: [!2, 1, 0] },
    Transformation { origin: [!2, !1, !0] },
];

pub const TRANSFORMATIONS: [Transformation; 48] = [
    Transformation { origin: [0, 1, 2] },
    Transformation { origin: [0, !1, !2] },
    Transformation { origin: [!0, !1, 2] },
    Transformation { origin: [!0, 1, !2] },
    Transformation { origin: [1, 2, 0] },
    Transformation { origin: [1, !2, !0] },
    Transformation { origin: [!1, !2, 0] },
    Transformation { origin: [!1, 2, !0] },
    Transformation { origin: [2, 0, 1] },
    Transformation { origin: [2, !0, !1] },
    Transformation { origin: [!2, !0, 1] },
    Transformation { origin: [!2, 0, !1] },
    Transformation { origin: [0, 2, !1] },
    Transformation { origin: [0, !2, 1] },
    Transformation { origin: [!0, 2, 1] },
    Transformation { origin: [!0, !2, !1] },
    Transformation { origin: [1, 0, !2] },
    Transformation { origin: [1, !0, 2] },
    Transformation { origin: [!1, 0, 2] },
    Transformation { origin: [!1, !0, !2] },
    Transformation { origin: [2, 1, !0] },
    Transformation { origin: [2, !1, 0] },
    Transformation { origin: [!2, 1, 0] },
    Transformation { origin: [!2, !1, !0] },

    Transformation { origin: [!0, 1, 2] },
    Transformation { origin: [!0, !1, !2] },
    Transformation { origin: [0, !1, 2] },
    Transformation { origin: [0, 1, !2] },
    Transformation { origin: [!1, 2, 0] },
    Transformation { origin: [!1, !2, !0] },
    Transformation { origin: [1, !2, 0] },
    Transformation { origin: [1, 2, !0] },
    Transformation { origin: [!2, 0, 1] },
    Transformation { origin: [!2, !0, !1] },
    Transformation { origin: [2, !0, 1] },
    Transformation { origin: [2, 0, !1] },
    Transformation { origin: [!0, 2, !1] },
    Transformation { origin: [!0, !2, 1] },
    Transformation { origin: [0, 2, 1] },
    Transformation { origin: [0, !2, !1] },
    Transformation { origin: [!1, 0, !2] },
    Transformation { origin: [!1, !0, 2] },
    Transformation { origin: [1, 0, 2] },
    Transformation { origin: [1, !0, !2] },
    Transformation { origin: [!2, 1, !0] },
    Transformation { origin: [!2, !1, 0] },
    Transformation { origin: [2, 1, 0] },
    Transformation { origin: [2, !1, !0] },
];

pub type Symmetry = u64;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trans() {
        assert_eq!(Transformation::id().trans_rect(Coord { x: 1, y: 2, z: 3 }), Coord { x: 1, y: 2, z: 3 });
        assert_eq!(
            Transformation { origin: [1, !2, 0] }.trans_rect(Coord { x: 2, y: 3, z: 4 }),
            Coord { x: 3, y: 4, z: 2 }
        );
        assert_eq!(
            Transformation { origin: [1, !2, 0] }.trans_point(Coord { x: 0, y: 1, z: 3 }, Coord { x: 2, y: 3, z: 4 }),
            Coord { x: 1, y: 0, z: 0 }
        );
    }

    #[test]
    fn test_trans_composition() {
        for &r in &ROTATIONS {
            for &s in &ROTATIONS {
                let rs = r.compose(s);
                let pt = Coord { x: 12, y: 7, z: 31 };
                let rect = Coord { x: 98, y: 54, z: 36 };

                let s_pt = s.trans_point(pt, rect);
                let s_rect = s.trans_rect(rect);

                assert_eq!(
                    r.trans_point(s_pt, s_rect),
                    rs.trans_point(pt, rect)
                );
            }
        }
    }
}
