use raylib::{drawing::{RaylibDrawHandle, RaylibDraw}, color::Color};

use crate::matrix::Matrix4;

#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl std::ops::Add<(f32, f32, f32)> for Position {
    type Output = (f32, f32, f32);
    fn add(self, other: (f32, f32, f32)) -> (f32, f32, f32) {
        (self.x + other.0, self.y + other.1, self.z + other.2)
    }
}

impl Into<Position> for (i32, i32, i32) {
    fn into(self) -> Position {
        Position::new(self.0 as f32, self.1 as f32, self.2 as f32)
    }
}

impl Into<Position> for (f32, f32, f32) {
    fn into(self) -> Position {
        Position::new(self.0, self.1, self.2)
    }
}

impl Position {

    #[allow(dead_code)]
    pub fn new(x: f32, y: f32, z: f32) -> Position {
        Self { x, y, z }
    }

    #[allow(dead_code)]
    pub fn get_point(&self) -> (f32, f32, f32) {
        (self.x, self.y, self.z)
    }

    #[allow(dead_code)]
    pub fn normalize(&self) -> Self {
        let (x, y, z) = self.get_point();
        let d = (x * x + y * y + z * z) as f32;
        let d = d.sqrt();
        let (x, y, z) = (x / d, y / d, z / d);
        Self::new(x, y, z)
    }

    #[allow(dead_code)]
    pub fn cross(&self, other: impl Into<Self>) -> Self {
        let other: Self = other.into();
        let (x1, y1, z1) = self.get_point();
        let (x2, y2, z2) = other.get_point();
        let x = y1 * z2 - z1 * y2;
        let y = z1 * x2 - x1 * z2;
        let z = x1 * y2 - y1 * x2;
        Self::new(x, y, z)
    }

    #[allow(dead_code)]
    pub fn multiply(&mut self, other: Matrix4) {
        let (x, y, z) = self.get_point();
        let (x, y, z) = (
            x * other.m[0][0] + y * other.m[1][0] + z * other.m[2][0] + other.m[3][0],
            x * other.m[0][1] + y * other.m[1][1] + z * other.m[2][1] + other.m[3][1],
            x * other.m[0][2] + y * other.m[1][2] + z * other.m[2][2] + other.m[3][2]
        );
        let w = x * other.m[0][3] + y * other.m[1][3] + z * other.m[2][3] + other.m[3][3];
        if w != 0.0 {
            self.x = x / w;
            self.y = y / w;
            self.z = z / w;
        } else {
            self.x = x;
            self.y = y;
            self.z = z;
        }
    }

    #[allow(dead_code)]
    pub fn add(&mut self, other: impl Into<Self>) {
        let other: Self = other.into();
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }

    #[allow(dead_code)]
    pub fn multiply_x(&mut self, scale: f32) {
        self.x *= scale;
    }

    #[allow(dead_code)]
    pub fn multiply_y(&mut self, scale: f32) {
        self.y *= scale;
    }

    #[allow(dead_code)]
    pub fn multiply_z(&mut self, scale: f32) {
        self.z *= scale;
    }

    #[allow(dead_code)]
    pub fn multiply_scalar(&mut self, sx: f32, sy: f32, sz: f32) {
        self.x *= sx;
        self.y *= sy;
        self.z *= sz;
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Triangle {
    pub p1: Position, 
    pub p2: Position, 
    pub p3: Position
}

impl Default for Triangle {
    fn default() -> Self {
        Self { p1: (0, 0, 0).into(), p2: (0, 0, 0).into(), p3: (0, 0, 0).into() }
    }
}

impl Triangle {

    #[allow(dead_code)]
    /// creates a new triangle from 3 points
    pub fn new(
        p1: impl Into<Position>, 
        p2: impl Into<Position>, 
        p3: impl Into<Position>
    ) -> Self {
        Self { p1: p1.into(), p2: p2.into(), p3: p3.into() }
    }

    #[allow(dead_code)]
    pub fn get_points(&self) -> (Position, Position, Position) {
        (self.p1, self.p2, self.p3)
    }

    #[allow(dead_code)]
    pub fn multiply(&mut self, other: Matrix4) {
        self.p1.multiply(other);
        self.p2.multiply(other);
        self.p3.multiply(other);
    }

    #[allow(dead_code)]
    pub fn multiply_x(&mut self, scale: f32) {
        self.p1.multiply_x(scale);
        self.p2.multiply_x(scale);
        self.p3.multiply_x(scale);
    }

    #[allow(dead_code)]
    pub fn multiply_y(&mut self, scale: f32) {
        self.p1.multiply_y(scale);
        self.p2.multiply_y(scale);
        self.p3.multiply_y(scale);
    }

    #[allow(dead_code)]
    pub fn multiply_z(&mut self, scale: f32) {
        self.p1.multiply_z(scale);
        self.p2.multiply_z(scale);
        self.p3.multiply_z(scale);
    }

    #[allow(dead_code)]
    pub fn multiply_scalar(&mut self, sx: f32, sy: f32, sz: f32) {
        self.p1.multiply_scalar(sx, sy, sz);
        self.p2.multiply_scalar(sx, sy, sz);
        self.p3.multiply_scalar(sx, sy, sz);
    }

    #[allow(dead_code)]
    pub fn add(&mut self, other: impl Into<Position>) {
        let other: Position = other.into();
        self.p1.add(other);
        self.p2.add(other);
        self.p3.add(other);
    }

    #[allow(dead_code)]
    pub fn draw(&self, draw: &mut RaylibDrawHandle) {
        let (p1, p2, p3) = self.get_points();
        draw.draw_line(p1.x as i32, p1.y as i32, p2.x as i32, p2.y as i32, Color::RED);
        draw.draw_line(p2.x as i32, p2.y as i32, p3.x as i32, p3.y as i32, Color::RED);
        draw.draw_line(p3.x as i32, p3.y as i32, p1.x as i32, p1.y as i32, Color::RED);
    }
}

pub struct Cube {
    pub triangles: [Triangle; 12]
}

impl Cube {

    pub fn new() -> Self {
        let triangles: [Triangle; 12] = [
            // South
            Triangle::new((0, 0, 0), (0, 1, 0), (1, 1, 0)),
            Triangle::new((0, 0, 0), (1, 1, 0), (1, 0, 0)),
            // East
            Triangle::new((1, 0, 0), (1, 1, 0), (1, 1, 1)),
            Triangle::new((1, 0, 0), (1, 1, 1), (1, 0, 1)),
            // North
            Triangle::new((1, 0, 1), (1, 1, 1), (0, 1, 1)),
            Triangle::new((1, 0, 1), (0, 1, 1), (0, 0, 1)),
            // West
            Triangle::new((0, 0, 1), (0, 1, 1), (0, 1, 0)),
            Triangle::new((0, 0, 1), (0, 1, 0), (0, 0, 0)),
            // Top
            Triangle::new((0, 1, 0), (0, 1, 1), (1, 1, 1)),
            Triangle::new((0, 1, 0), (1, 1, 1), (1, 1, 0)),
            // Bottom
            Triangle::new((1, 0, 1), (0, 0, 1), (0, 0, 0)),
            Triangle::new((1, 0, 1), (0, 0, 0), (1, 0, 0))
        ];

        Self { triangles }
    }

}
