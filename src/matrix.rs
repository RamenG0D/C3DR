use std::f32::consts::PI;

#[warn(dead_code)]

#[derive(Debug, Copy, Clone)]
pub struct Matrix4 {
    pub m: [[f32; 4]; 4]
}

#[allow(non_snake_case)]
impl Matrix4 {

    pub fn new(m: [[f32; 4]; 4]) -> Self {
        Self { m }
    }

    pub fn identity() -> Self {
        Self { m: [
            [1.0, 0.0, 0.0, 0.0], 
            [0.0, 1.0, 0.0, 0.0], 
            [0.0, 0.0, 1.0, 0.0], 
            [0.0, 0.0, 0.0, 1.0]
        ] }
    }

    pub fn Projection(near: f32, far: f32, fov: f32, aspect_ratio: f32) -> Self {
        let fov = 1.0 / (fov * 0.5 / 180.0 * PI).tan();
        let mut o = Self::identity();
        o.m[0][0] = aspect_ratio * fov;
        o.m[1][1] = fov;
        o.m[2][2] = far / (far - near);
        o.m[3][2] = (-far * near) / (far - near);
        o.m[2][3] = 1.0;
        o.m[3][3] = 0.0;
        o
    }

    pub fn RotateX(angle: f32) -> Self {
        let angle = angle.to_radians();
        let mut o = Self::identity();
        o.m[1][1] = angle.cos();
        o.m[1][2] = -angle.sin();
        o.m[2][1] = angle.sin();
        o.m[2][2] = angle.cos();
        o
    }

    pub fn RotateY(angle: f32) -> Self {
        let angle = angle.to_radians();
        let mut o = Self::identity();
        o.m[0][0] = angle.cos();
        o.m[0][2] = angle.sin();
        o.m[2][0] = -angle.sin();
        o.m[2][2] = angle.cos();
        o
    }

    pub fn RotateZ(angle: f32) -> Self {
        let angle = angle.to_radians();
        let mut o = Self::identity();
        o.m[0][0] = angle.cos();
        o.m[0][1] = -angle.sin();
        o.m[1][0] = angle.sin();
        o.m[1][1] = angle.cos();
        o
    }

    pub fn Rotate(x: f32, y: f32, z: f32) -> Self {
        let mut o = Self::identity();
        o.multiply(Self::RotateX(x));
        o.multiply(Self::RotateY(y));
        o.multiply(Self::RotateZ(z));
        o
    }

    pub fn multiply(&mut self, other: Self) {
        let mut o = Self::identity();
        for c in 0..4 {
            for r in 0..4 {
                o.m[r][c] = self.m[r][0] * other.m[0][c] + self.m[r][1] * other.m[1][c] + self.m[r][2] * other.m[2][c] + self.m[r][3] * other.m[3][c];
            }
        }
        self.m = o.m;
    }

    /// creates a new matrix for translation
    pub fn Translate(x: f32, y: f32, z: f32) -> Self {
        Self::new([
            [1.0, 0.0, 0.0, x], 
            [0.0, 1.0, 0.0, y], 
            [0.0, 0.0, 1.0, z], 
            [0.0, 0.0, 0.0, 1.0]
        ])
    }

    /// creates a new matrix for scaling
    pub fn Scale(x: f32, y: f32, z: f32) -> Self {
        Self::new([
            [x, 0.0, 0.0, 0.0], 
            [0.0, y, 0.0, 0.0], 
            [0.0, 0.0, z, 0.0], 
            [0.0, 0.0, 0.0, 1.0]
        ])
    }

    /// creates a new copy of the matrix
    pub fn transpose(&self) -> Self {
        Self::new(self.m)
    }
}
