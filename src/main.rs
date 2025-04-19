use bevy::input::{ButtonState, keyboard::KeyboardInput};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, close_on_escape)
        .add_systems(Startup, startup)
        .run();
}

fn startup(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn(Camera2d::default());
    commands.spawn(Sprite::from_image(server.load("clouds.png")));
}

fn close_on_escape(mut input: EventReader<KeyboardInput>, mut writer: EventWriter<AppExit>) {
    for e in input.read() {
        if e.key_code == KeyCode::Escape && e.state == ButtonState::Pressed {
            writer.send(AppExit::Success);
        }
    }
}
