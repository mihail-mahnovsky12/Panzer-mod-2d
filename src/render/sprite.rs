use crate::math::vec2::*;
use macroquad::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Sprite {
    pub size: mVec2,
    pub position: mVec2,
    pub image: Texture2D,
}

impl Sprite {
    pub async fn new(path: &str, position: mVec2) -> Sprite {
        let texture = load_texture(path).await.unwrap();

        let size = mVec2::new(texture.width() as f64, texture.height() as f64);

        Sprite {
            size,
            position,
            image: texture,
        }
    }

    pub fn draw(&self) {
        draw_texture(
            &self.image,
            self.position.x as f32,
            self.position.y as f32,
            WHITE,
        );
    }

    pub fn draw_sized(&self) {
        draw_texture_ex(
            &self.image,
            self.position.x as f32,
            self.position.y as f32,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.size.x as f32, self.size.y as f32)),
                ..Default::default()
            },
        );
    }
}
