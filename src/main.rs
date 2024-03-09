//! TODO:
//! - add constants for tiles

pub mod math;

use macroquad::{miniquad::window::screen_size, prelude::*};
use math::{Vec2, Vec3, ZeroExt};

pub struct Game {
    view: View,
    elements: Vec<Element>,
}

impl Game {
    pub fn new(view: View) -> Self {
        Self {
            view,
            elements: Vec::new(),
        }
    }

    pub fn line(&mut self, from: Vec3<f64>, to: Vec3<f64>) {
        self.elements.push(Element::Line {
            from,
            to,
            color: RED,
        });
    }

    pub fn wire_cube(&mut self, pos: Vec3<f64>, size: f64) {
        let half = size / 2.0;
        // Bottom
        self.line(
            pos + Vec3::new(half, half, -half),
            pos + Vec3::new(half, -half, -half),
        );
        self.line(
            pos + Vec3::new(half, half, -half),
            pos + Vec3::new(-half, half, -half),
        );
        // Top
        self.line(
            pos + Vec3::new(-half, -half, half),
            pos + Vec3::new(half, -half, half),
        );
        self.line(
            pos + Vec3::new(-half, -half, half),
            pos + Vec3::new(-half, half, half),
        );
        self.line(
            pos + Vec3::new(half, half, half),
            pos + Vec3::new(half, -half, half),
        );
        self.line(
            pos + Vec3::new(half, half, half),
            pos + Vec3::new(-half, half, half),
        );
        // Sides
        self.line(
            pos + Vec3::new(half, half, half),
            pos + Vec3::new(half, half, -half),
        );
        self.line(
            pos + Vec3::new(half, -half, half),
            pos + Vec3::new(half, -half, -half),
        );
        self.line(
            pos + Vec3::new(-half, half, half),
            pos + Vec3::new(-half, half, -half),
        );
    }

    pub fn cube_sprite(&mut self, pos: Vec3<f64>, scale: f32, tex_pos: Vec2<f32>) {
        self.elements.push(Element::CubeSprite {
            pos,
            scale,
            tex_pos,
        });
    }
}

pub struct View {
    width: f32,
    height: f32,
    edge_height: f32,
    center: Vec2<f64>,
    size: Vec2<f32>,
    frame_start: Vec2<f64>,
    frame_end: Vec2<f64>,
    cube_tilemap: Tilemap,
}

impl View {
    pub fn new(width: f32, height: f32, edge_height: f32, cube_tilemap: Tilemap) -> Self {
        Self {
            width,
            height,
            edge_height,
            center: Vec2::ZERO,
            size: Vec2::ZERO,
            frame_start: Vec2::ZERO,
            frame_end: Vec2::ZERO,
            cube_tilemap,
        }
    }

    pub fn contains_point(&self, point: Vec2<f64>) -> bool {
        self.frame_start.x <= point.x
            && self.frame_start.y <= point.y
            && self.frame_end.x >= point.x
            && self.frame_end.y >= point.y
    }

    pub fn translate_vec(&self, source: Vec3<f64>) -> Vec2<f64> {
        Vec2::new(
            source.x * self.width as f64 - source.y * self.width as f64,
            source.x * self.edge_height as f64 + source.y * self.edge_height as f64
                - source.z * self.height as f64,
        )
    }

    pub fn draw(&self, element: &Element) {
        match element {
            Element::Line { from, to, color } => {
                let from = self.translate_vec(*from);
                let to = self.translate_vec(*to);
                if !self.contains_point(from) || !self.contains_point(to) {
                    return;
                }
                let from: Vec2<f32> = (from - self.frame_start).into();
                let to: Vec2<f32> = (to - self.frame_start).into();
                draw_line(from.x, from.y, to.x, to.y, 1.5, *color);
            }
            Element::CubeSprite {
                pos,
                scale,
                tex_pos,
            } => {
                let center_pos: Vec2<f32> = (self.translate_vec(*pos) - self.frame_start).into();
                let offset = Vec2::new(
                    *scale * -self.width,
                    *scale * (-self.height / 2.0 - self.edge_height),
                );
                let abs_pos = center_pos + offset;
                draw_texture_ex(
                    &self.cube_tilemap.texture,
                    abs_pos.x,
                    abs_pos.y,
                    WHITE,
                    DrawTextureParams {
                        source: Some(self.cube_tilemap.tile(*tex_pos)),
                        dest_size: Some(macroquad::math::vec2(
                            *scale * self.width * 2.0,
                            *scale * (self.height + self.edge_height * 2.0),
                        )),
                        ..Default::default()
                    },
                );
            }
        }
    }

    pub fn update_size(&mut self, size: Vec2<f32>, center: Vec3<f64>) {
        self.center = self.translate_vec(center);
        self.size = size;
        let size: Vec2<f64> = size.into();
        let half = size / 2.0;
        self.frame_start = self.center - half;
        self.frame_end = self.center + half;
    }
}

pub struct Tilemap {
    texture: Texture2D,
    tex_size: Vec2<f32>,
}

impl Tilemap {
    pub fn new(texture: Texture2D, tex_size: Vec2<f32>) -> Self {
        Self { texture, tex_size }
    }

    pub fn tile(&self, tex_pos: Vec2<f32>) -> macroquad::math::Rect {
        macroquad::math::Rect::new(
            self.tex_size.x * tex_pos.x,
            self.tex_size.y * tex_pos.y,
            self.tex_size.x,
            self.tex_size.y,
        )
    }
}

pub enum Element {
    Line {
        from: Vec3<f64>,
        to: Vec3<f64>,
        color: Color,
    },
    CubeSprite {
        pos: Vec3<f64>,
        scale: f32,
        tex_pos: Vec2<f32>,
    },
}

fn load_tilemap() -> Texture2D {
    let bytes = include_bytes!("../assets/test_tilemap.png");
    let image = Image::from_file_with_format(bytes, Some(ImageFormat::Png)).expect("Valid image");
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);
    texture
}

#[macroquad::main("Orthagonal projection testing")]
async fn main() {
    let cube_tilemap = Tilemap::new(load_tilemap(), Vec2::new(64.0, 72.0));
    let w = 100.0;
    let h = w * 1.125;
    let eh = h / 2.0;
    let mut game = Game::new(View::new(w, h, eh, cube_tilemap));

    // game.line(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
    // game.line(Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 0.0));
    // game.line(Vec3::new(1.0, 1.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    // game.line(Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
    //
    // game.line(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));

    game.cube_sprite(Vec3::new(0.0, 0.0, 0.0), 1.0, Vec2::new(0.0, 0.0));
    game.wire_cube(Vec3::new(0.0, 0.0, 0.0), 1.0);
    game.cube_sprite(Vec3::new(0.0, 0.0, 1.0), 1.0, Vec2::new(0.0, 0.0));

    loop {
        let (width, height) = screen_size();
        game.view
            .update_size(Vec2::new(width, height), Vec3::new(0.0, 0.0, -1.0));

        clear_background(BLUE);

        for element in &game.elements {
            game.view.draw(element);
        }

        next_frame().await
    }
}
