use crate::math::vec2::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    position: mVec2,
    size: mVec2,
    rotation: f64,
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            position: mVec2::new(0.0, 0.0),
            size: mVec2::new(0.0, 0.0),
            rotation: 0.0,
        }
    }

    pub fn get_position(&self) -> mVec2 {
        self.position
    }

    pub fn get_size(&self) -> mVec2 {
        self.size
    }

    pub fn get_rotation(&self) -> f64 {
        self.rotation
    }

    pub fn set_position(&mut self, new_pos: mVec2) {
        self.position = new_pos
    }

    pub fn set_size(&mut self, new_size: mVec2) {
        self.size = new_size;
    }

    pub fn set_rotation(&mut self, new_rot: f64) {
        self.rotation = new_rot;
    }
}
