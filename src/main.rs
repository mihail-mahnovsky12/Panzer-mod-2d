mod app;

mod ecs;
mod math;
mod render;
mod transform;

use macroquad::prelude::*;

use crate::app::App;

#[macroquad::main("PM2D")]
async fn main() {
    let mut app: App = App::new().await;
    app.run().await;
}
