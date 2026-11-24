use crate::ecs::component::*;
use crate::ecs::scene::*;
use crate::math::vec2::*;
use crate::render::drawble::Drawble;

use std::rc::Rc;

use macroquad::prelude::*;

pub struct Renderer {}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {}
    }

    pub async fn draw(&self, current_scene: &Scene) {
        clear_background(WHITE);

        for i in 0..current_scene.get_let() {
            if current_scene.get_object(i).has_drawable() {
                if let Some(drawable) = current_scene.get_object(i).get_drawable() {
                    match drawable {
                        Drawble::Sprite(sprite) => {
                            sprite.draw_sized();
                        }
                    }
                }
            }
        }

        next_frame().await
    }
}
