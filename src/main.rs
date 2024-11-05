use bevy::prelude::*;

mod application;
mod game;
mod loading;
mod world;

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::linear_rgb(0.83, 0.96, 0.96)))
        .add_plugins((DefaultPlugins, world::WorldPlugin))
        .run();
}
