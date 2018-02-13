use super::*;

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
    pub fn rotate_rect(&self, size: Coord) -> Coord {
        let dim = [size.x, size.y, size.z];
        Coord {
            x: dim[if self.origin[0] >= 0 { self.origin[0] } else { (!self.origin[0]) } as usize],
            y: dim[if self.origin[1] >= 0 { self.origin[1] } else { (!self.origin[1]) } as usize],
            z: dim[if self.origin[2] >= 0 { self.origin[2] } else { (!self.origin[2]) } as usize],
        }
    }
    pub fn rotate_point(&self, p: Coord, rect: Coord) -> Coord {
        let dim = [rect.x, rect.y, rect.z];
        let pt = [p.x, p.y, p.z];
        Coord {
            x: if self.origin[0] >= 0 { pt[self.origin[0] as usize] } else { dim[!self.origin[0] as usize] - pt[!self.origin[0] as usize] - 1 },
            y: if self.origin[1] >= 0 { pt[self.origin[1] as usize] } else { dim[!self.origin[1] as usize] - pt[!self.origin[1] as usize] - 1 },
            z: if self.origin[2] >= 0 { pt[self.origin[2] as usize] } else { dim[!self.origin[2] as usize] - pt[!self.origin[2] as usize] - 1 },
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

    /// Compose two rotations.
    /// 
    /// It is guaranteed that, for any `r`, `s`: `Rotation` and `p`: `Coord`,
    /// `r.compose(s).rotate_rect(p) == r.rotate_rect(s.rotate_rect(p))`
    /// holds (similar for `rotate_point`).
    pub fn compose(self, other: Rotation) -> Rotation {
        Rotation {
            origin: [
                if self.origin[0] >= 0 { other.origin[self.origin[0] as usize] } else { !other.origin[!self.origin[0] as usize] },
                if self.origin[1] >= 0 { other.origin[self.origin[1] as usize] } else { !other.origin[!self.origin[1] as usize] },
                if self.origin[2] >= 0 { other.origin[self.origin[2] as usize] } else { !other.origin[!self.origin[2] as usize] },
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotation() {
        assert_eq!(Rotation::id().rotate_rect(Coord { x: 1, y: 2, z: 3 }), Coord { x: 1, y: 2, z: 3 });
        assert_eq!(
            Rotation { origin: [1, !2, 0] }.rotate_rect(Coord { x: 2, y: 3, z: 4 }),
            Coord { x: 3, y: 4, z: 2 }
        );
        assert_eq!(
            Rotation { origin: [1, !2, 0] }.rotate_point(Coord { x: 0, y: 1, z: 3 }, Coord { x: 2, y: 3, z: 4 }),
            Coord { x: 1, y: 0, z: 0 }
        );
    }

    #[test]
    fn test_rotation_composition() {
        for &r in &ROTATIONS {
            for &s in &ROTATIONS {
                let rs = r.compose(s);
                let pt = Coord { x: 12, y: 7, z: 31 };
                let rect = Coord { x: 98, y: 54, z: 36 };

                let s_pt = s.rotate_point(pt, rect);
                let s_rect = s.rotate_rect(rect);

                assert_eq!(
                    r.rotate_point(s_pt, s_rect),
                    rs.rotate_point(pt, rect)
                );
            }
        }
    }
}
