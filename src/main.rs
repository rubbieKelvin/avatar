use std::path::Path;

use bevy::prelude::*;

#[allow(unused)]
enum PuppetComponentKind {
    Hat,
    Head,
    Eyes,
    Mouth,
}

#[allow(unused)]
#[derive(Component)]
struct PuppetComponent {
    kind: PuppetComponentKind,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d::default());

    // create puppet (hat)
    commands.spawn((
        PuppetComponent {
            kind: PuppetComponentKind::Hat,
        },
        Sprite {
            image: asset_server.load("images/aang/hat.png"),
            ..Default::default()
        },
        Transform::from_xyz(0., 0., 0.),
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Aang".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, setup)
        .run();
}
