
// disable the console window when building in release mode
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use raylib::prelude::*;

mod matrix;
mod shapes;

use matrix::Matrix4;
use shapes::Cube;

/*
struct Player {
    x: f32,
    y: f32,
    z: f32,
    angle: f32
}

impl Player {

    fn new(x: f32, y: f32, z: f32, angle: f32) -> Player {
        Self { 
            x, 
            y,
            z, 
            // vx: 0.0, 
            // vy: 0.0,
            // vz: 0.0, 
            angle,
            veiw_matrix: Matrix3::identity()
        }
    }

    fn clamp(&mut self) {
        if self.angle > 359.0 {
            self.angle -= 360.0;
        } else if self.angle < 0.0 {
            self.angle += 360.0;
        }
    }

    fn r#move(&mut self, draw: &RaylibDrawHandle) {
        const MOVEMENT: f32 = 1.0;
        if draw.is_key_down(KeyboardKey::KEY_W) || draw.is_key_down(KeyboardKey::KEY_UP) {
            self.z -= MOVEMENT;
        }
        if draw.is_key_down(KeyboardKey::KEY_S) || draw.is_key_down(KeyboardKey::KEY_DOWN) {
            self.z += MOVEMENT;
        }
        if draw.is_key_down(KeyboardKey::KEY_A) || draw.is_key_down(KeyboardKey::KEY_LEFT) {
            self.x -= MOVEMENT;
        }
        if draw.is_key_down(KeyboardKey::KEY_D) || draw.is_key_down(KeyboardKey::KEY_RIGHT) {
            self.x += MOVEMENT;
        }
        if draw.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) {
            self.y += MOVEMENT;
        }
        if draw.is_key_down(KeyboardKey::KEY_SPACE) {
            self.y -= MOVEMENT;
        }

        if draw.is_key_down(KeyboardKey::KEY_Q) {
            self.angle -= 0.1;
            self.clamp();
        } else if draw.is_key_down(KeyboardKey::KEY_E) {
            self.angle += 0.1;
            self.clamp();
        }

        // self.x += ;
        // self.y += ;
        // self.z += ;

        // if self.vx > 0.0 {
        //     self.vx -= 0.0001;
        // } else if self.vx < 0.0 {
        //     self.vx += 0.0001;
        // }
        // if self.vy > 0.0 {
        //     self.vy -= 0.0001;
        // } else if self.vy < 0.0 {
        //     self.vy += 0.0001;
        // }
        // if self.vz > 0.0 {
        //     self.vz -= 0.0001;
        // } else if self.vz < 0.0 {
        //     self.vz += 0.0001;
        // }
    }

    fn draw(&mut self, draw: &mut RaylibDrawHandle) {
        draw.draw_rectangle(self.x as i32, self.y as i32, 12, 12, Color::RED);

        let (dx, dy) = (self.angle.to_radians().cos(), self.angle.to_radians().sin());

        // draw a line that represents the direction the player is facing / the player angle
        draw.draw_line(
            self.x as i32 + 6, 
            self.y as i32 + 6, 
            (self.x + dx * 20.0) as i32 + 6, 
            (self.y + dy * 20.0) as i32 + 6, 
            Color::RED
        );
    }

    fn update(&mut self, draw: &mut RaylibDrawHandle) {
        self.r#move(draw);
        self.draw(draw);
    }
}
*/

fn main() {
    let (mut rl, t) = 
        raylib::init()
            .title("Test")
            .size(600, 600)
            .resizable()
            .build();

    rl.set_target_fps(60);

    let cube = Cube::new();

    let world = Matrix4::Projection(0.1, 1000.0, 90.0, 600.0 / 600.0);

    while !rl.window_should_close() {
        let theta: f32 = 10.0 * rl.get_time() as f32;
        let mut drawv = rl.begin_drawing(&t);

        drawv.clear_background(Color::BLACK);
        
        // drawv.draw_text(&format!("Position: [ {}, {}, {} ]", player.x, player.y, player.z), 10, 10, 20, Color::WHITE);
        // drawv.draw_text(&format!("Angle: {}", player.angle), 10, 30, 20, Color::WHITE);
        drawv.draw_text(&format!("FPS: {}", drawv.get_fps()), 10, 50, 20, Color::WHITE);

        // player.update(&mut drawv);

        for triangle in cube.triangles {
            let mut tri = triangle.clone();
            // rotate the triangle z
            tri.multiply(Matrix4::RotateZ(theta));
            // rotate the triangle x
            tri.multiply(Matrix4::RotateX(theta * 0.5));
            // translate the triangles z
            tri.add((0, 0, 3));
            // move coords to world space
            tri.multiply(world);
            // convert to 3d space
            tri.add((1, 1, 0));
            // move the triangle to the center of the screen
            tri.multiply_x(0.5 * drawv.get_screen_width() as f32);
            tri.multiply_y(0.5 * drawv.get_screen_height() as f32);

            // draw the triangle
            tri.draw(&mut drawv);
        }
    }
}
