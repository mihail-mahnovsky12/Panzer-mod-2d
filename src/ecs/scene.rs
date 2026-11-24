use crate::ecs::scene_object::*;

pub struct Scene {
    objects: Vec<SceneObject>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            objects: Vec::new(),
        }
    }

    pub fn get_object(&self, i: usize) -> &SceneObject {
        &self.objects[i]
    }

    pub fn get_object_mut(&mut self, i: usize) -> &mut SceneObject {
        &mut self.objects[i]
    }

    pub fn add(&mut self, new_obj: SceneObject) {
        self.objects.push(new_obj);
    }

    pub fn get_let(&self) -> usize {
        self.objects.len()
    }
}
