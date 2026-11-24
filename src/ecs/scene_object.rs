use crate::ecs::component::*;
use crate::ecs::entity::*;
use crate::render::drawble::*;

#[derive(Debug, Clone, PartialEq)]
pub struct SceneObject {
    name: String,
    entity: Entity,
    components: Vec<Component>,
}

impl SceneObject {
    pub fn new(name: String) -> SceneObject {
        SceneObject {
            name: name,
            entity: Entity::new(),
            components: Vec::new(),
        }
    }

    pub fn get_component_mut(&mut self) -> &mut Vec<Component> {
        &mut self.components
    }

    pub fn get_component(&self) -> Vec<Component> {
        self.components.clone()
    }

    pub fn add_component(&mut self, comp: Component) {
        self.components.push(comp);
    }

    pub fn get_drawable(&self) -> Option<&Drawble> {
        self.components.iter().find_map(|c| {
            if let Component::Drawble(drawable) = c {
                Some(drawable)
            } else {
                None
            }
        })
    }

    pub fn has_drawable(&self) -> bool {
        self.components
            .iter()
            .any(|c| matches!(c, Component::Drawble(_)))
    }
}
