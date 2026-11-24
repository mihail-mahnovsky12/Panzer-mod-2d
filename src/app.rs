use crate::{
    ecs::{component::Component, scene::Scene, scene_object::SceneObject},
    math::vec2::mVec2,
    render::{drawble::Drawble, renderer::*, sprite::Sprite},
};

const FIXED_DT: f64 = 1.0 / 120.0;

pub struct App {
    is_end: bool,
    accumulator: f64,
    renderer: Renderer,
    scenes: Vec<Scene>,
}

impl App {
    pub async fn new() -> App {
        let mut scenes: Vec<Scene> = Vec::new();
        scenes.push(Scene::new());
        println!("scene vec has created");

        let mut test: SceneObject = SceneObject::new("test".to_string());

        println!("sceneObject vec has created");

        test.add_component(Component::Drawble(Drawble::Sprite(
            Sprite::new(
                "C:/Users/mahom/OneDrive/Desktop/rust projects/panzer_mod_2d/res/Makoofficial.png",
                mVec2::new(200.0, 200.0),
            )
            .await,
        )));

        for component in test.get_component_mut() {
            match component {
                Component::Drawble(Drawble::Sprite(sprite)) => {
                    sprite.size = mVec2::new(250.0, 350.0);
                }

                _ => {}
            }
        }

        println!("commponent added");

        scenes[0].add(test);

        let renderer = Renderer::new();

        println!("renderer created");

        App {
            is_end: false,
            accumulator: 0.0,
            renderer: renderer,
            scenes: scenes,
        }
    }

    pub fn update(&mut self) {}

    pub async fn draw(&self) {
        self.renderer.draw(&self.scenes[0]).await;
    }

    pub async fn run(&mut self) {
        println!("Starting game loop");

        while !self.is_end {
            self.update();
            self.draw().await;
        }
    }
}
