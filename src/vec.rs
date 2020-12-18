use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec2 {
    pub x: isize,
    pub y: isize,
}

impl Vec2 {
    pub fn to_vec3(&self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: 0,
        }
    }

    pub fn to_vec4(&self) -> Vec4 {
        Vec4 {
            x: self.x,
            y: self.y,
            z: 0,
            w: 0,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Add<isize> for Vec3 {
    type Output = Self;

    fn add(self, rhs: isize) -> Self::Output {
        Vec3 {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Vec4 {
    pub x: isize,
    pub y: isize,
    pub z: isize,
    pub w: isize,
}

impl Add<isize> for Vec4 {
    type Output = Self;

    fn add(self, rhs: isize) -> Self::Output {
        Vec4 {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
            w: self.w + rhs,
        }
    }
}

impl Add for Vec4 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

pub const OFFSETS_3: [Vec3; 26] = [
    Vec3 {
        x: -1,
        y: -1,
        z: -1,
    },
    Vec3 { x: -1, y: -1, z: 0 },
    Vec3 { x: -1, y: -1, z: 1 },
    Vec3 { x: -1, y: 0, z: -1 },
    Vec3 { x: -1, y: 0, z: 0 },
    Vec3 { x: -1, y: 0, z: 1 },
    Vec3 { x: -1, y: 1, z: -1 },
    Vec3 { x: -1, y: 1, z: 0 },
    Vec3 { x: -1, y: 1, z: 1 },
    Vec3 { x: 0, y: -1, z: -1 },
    Vec3 { x: 0, y: -1, z: 0 },
    Vec3 { x: 0, y: -1, z: 1 },
    Vec3 { x: 0, y: 0, z: -1 },
    Vec3 { x: 0, y: 0, z: 1 },
    Vec3 { x: 0, y: 1, z: -1 },
    Vec3 { x: 0, y: 1, z: 0 },
    Vec3 { x: 0, y: 1, z: 1 },
    Vec3 { x: 1, y: -1, z: -1 },
    Vec3 { x: 1, y: -1, z: 0 },
    Vec3 { x: 1, y: -1, z: 1 },
    Vec3 { x: 1, y: 0, z: -1 },
    Vec3 { x: 1, y: 0, z: 0 },
    Vec3 { x: 1, y: 0, z: 1 },
    Vec3 { x: 1, y: 1, z: -1 },
    Vec3 { x: 1, y: 1, z: 0 },
    Vec3 { x: 1, y: 1, z: 1 },
];

pub const OFFSETS_4: [Vec4; 80] = [
    Vec4 {
        x: -1,
        y: -1,
        z: -1,
        w: -1,
    },
    Vec4 {
        x: -1,
        y: -1,
        z: 0,
        w: -1,
    },
    Vec4 {
        x: -1,
        y: -1,
        z: 1,
        w: -1,
    },
    Vec4 {
        x: -1,
        y: 0,
        z: -1,
        w: -1,
    },
    Vec4 {
        x: -1,
        y: 0,
        z: 0,
        w: -1,
    },
    Vec4 {
        x: -1,
        y: 0,
        z: 1,
        w: -1,
    },
    Vec4 {
        x: -1,
        y: 1,
        z: -1,
        w: -1,
    },
    Vec4 {
        x: -1,
        y: 1,
        z: 0,
        w: -1,
    },
    Vec4 {
        x: -1,
        y: 1,
        z: 1,
        w: -1,
    },
    Vec4 {
        x: 0,
        y: -1,
        z: -1,
        w: -1,
    },
    Vec4 {
        x: 0,
        y: -1,
        z: 0,
        w: -1,
    },
    Vec4 {
        x: 0,
        y: -1,
        z: 1,
        w: -1,
    },
    Vec4 {
        x: 0,
        y: 0,
        z: -1,
        w: -1,
    },
    Vec4 {
        x: 0,
        y: 0,
        z: 0,
        w: -1,
    },
    Vec4 {
        x: 0,
        y: 0,
        z: 1,
        w: -1,
    },
    Vec4 {
        x: 0,
        y: 1,
        z: -1,
        w: -1,
    },
    Vec4 {
        x: 0,
        y: 1,
        z: 0,
        w: -1,
    },
    Vec4 {
        x: 0,
        y: 1,
        z: 1,
        w: -1,
    },
    Vec4 {
        x: 1,
        y: -1,
        z: -1,
        w: -1,
    },
    Vec4 {
        x: 1,
        y: -1,
        z: 0,
        w: -1,
    },
    Vec4 {
        x: 1,
        y: -1,
        z: 1,
        w: -1,
    },
    Vec4 {
        x: 1,
        y: 0,
        z: -1,
        w: -1,
    },
    Vec4 {
        x: 1,
        y: 0,
        z: 0,
        w: -1,
    },
    Vec4 {
        x: 1,
        y: 0,
        z: 1,
        w: -1,
    },
    Vec4 {
        x: 1,
        y: 1,
        z: -1,
        w: -1,
    },
    Vec4 {
        x: 1,
        y: 1,
        z: 0,
        w: -1,
    },
    Vec4 {
        x: 1,
        y: 1,
        z: 1,
        w: -1,
    },
    Vec4 {
        x: -1,
        y: -1,
        z: -1,
        w: 0,
    },
    Vec4 {
        x: -1,
        y: -1,
        z: 0,
        w: 0,
    },
    Vec4 {
        x: -1,
        y: -1,
        z: 1,
        w: 0,
    },
    Vec4 {
        x: -1,
        y: 0,
        z: -1,
        w: 0,
    },
    Vec4 {
        x: -1,
        y: 0,
        z: 0,
        w: 0,
    },
    Vec4 {
        x: -1,
        y: 0,
        z: 1,
        w: 0,
    },
    Vec4 {
        x: -1,
        y: 1,
        z: -1,
        w: 0,
    },
    Vec4 {
        x: -1,
        y: 1,
        z: 0,
        w: 0,
    },
    Vec4 {
        x: -1,
        y: 1,
        z: 1,
        w: 0,
    },
    Vec4 {
        x: 0,
        y: -1,
        z: -1,
        w: 0,
    },
    Vec4 {
        x: 0,
        y: -1,
        z: 0,
        w: 0,
    },
    Vec4 {
        x: 0,
        y: -1,
        z: 1,
        w: 0,
    },
    Vec4 {
        x: 0,
        y: 0,
        z: -1,
        w: 0,
    },
    Vec4 {
        x: 0,
        y: 0,
        z: 1,
        w: 0,
    },
    Vec4 {
        x: 0,
        y: 1,
        z: -1,
        w: 0,
    },
    Vec4 {
        x: 0,
        y: 1,
        z: 0,
        w: 0,
    },
    Vec4 {
        x: 0,
        y: 1,
        z: 1,
        w: 0,
    },
    Vec4 {
        x: 1,
        y: -1,
        z: -1,
        w: 0,
    },
    Vec4 {
        x: 1,
        y: -1,
        z: 0,
        w: 0,
    },
    Vec4 {
        x: 1,
        y: -1,
        z: 1,
        w: 0,
    },
    Vec4 {
        x: 1,
        y: 0,
        z: -1,
        w: 0,
    },
    Vec4 {
        x: 1,
        y: 0,
        z: 0,
        w: 0,
    },
    Vec4 {
        x: 1,
        y: 0,
        z: 1,
        w: 0,
    },
    Vec4 {
        x: 1,
        y: 1,
        z: -1,
        w: 0,
    },
    Vec4 {
        x: 1,
        y: 1,
        z: 0,
        w: 0,
    },
    Vec4 {
        x: 1,
        y: 1,
        z: 1,
        w: 0,
    },
    Vec4 {
        x: -1,
        y: -1,
        z: -1,
        w: 1,
    },
    Vec4 {
        x: -1,
        y: -1,
        z: 0,
        w: 1,
    },
    Vec4 {
        x: -1,
        y: -1,
        z: 1,
        w: 1,
    },
    Vec4 {
        x: -1,
        y: 0,
        z: -1,
        w: 1,
    },
    Vec4 {
        x: -1,
        y: 0,
        z: 0,
        w: 1,
    },
    Vec4 {
        x: -1,
        y: 0,
        z: 1,
        w: 1,
    },
    Vec4 {
        x: -1,
        y: 1,
        z: -1,
        w: 1,
    },
    Vec4 {
        x: -1,
        y: 1,
        z: 0,
        w: 1,
    },
    Vec4 {
        x: -1,
        y: 1,
        z: 1,
        w: 1,
    },
    Vec4 {
        x: 0,
        y: -1,
        z: -1,
        w: 1,
    },
    Vec4 {
        x: 0,
        y: -1,
        z: 0,
        w: 1,
    },
    Vec4 {
        x: 0,
        y: -1,
        z: 1,
        w: 1,
    },
    Vec4 {
        x: 0,
        y: 0,
        z: -1,
        w: 1,
    },
    Vec4 {
        x: 0,
        y: 0,
        z: 0,
        w: 1,
    },
    Vec4 {
        x: 0,
        y: 0,
        z: 1,
        w: 1,
    },
    Vec4 {
        x: 0,
        y: 1,
        z: -1,
        w: 1,
    },
    Vec4 {
        x: 0,
        y: 1,
        z: 0,
        w: 1,
    },
    Vec4 {
        x: 0,
        y: 1,
        z: 1,
        w: 1,
    },
    Vec4 {
        x: 1,
        y: -1,
        z: -1,
        w: 1,
    },
    Vec4 {
        x: 1,
        y: -1,
        z: 0,
        w: 1,
    },
    Vec4 {
        x: 1,
        y: -1,
        z: 1,
        w: 1,
    },
    Vec4 {
        x: 1,
        y: 0,
        z: -1,
        w: 1,
    },
    Vec4 {
        x: 1,
        y: 0,
        z: 0,
        w: 1,
    },
    Vec4 {
        x: 1,
        y: 0,
        z: 1,
        w: 1,
    },
    Vec4 {
        x: 1,
        y: 1,
        z: -1,
        w: 1,
    },
    Vec4 {
        x: 1,
        y: 1,
        z: 0,
        w: 1,
    },
    Vec4 {
        x: 1,
        y: 1,
        z: 1,
        w: 1,
    },
];
