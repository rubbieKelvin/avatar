use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, egui};
use nanoid::nanoid;

struct PuppetLayer {
    id: String,
    name: String,
}

impl Default for PuppetLayer {
    fn default() -> Self {
        return Self {
            id: nanoid!(8),
            name: "new layer".into(),
        };
    }
}

#[derive(Default, Resource)]
struct ApplicationState {
    layers: Vec<PuppetLayer>,
}

fn setup_layer_components(mut commands: Commands, asset_server: Res<AssetServer>) {
    // create puppet (hat)
    commands.spawn((
        Sprite {
            image: asset_server.load("images/aang/hat.png"),
            ..Default::default()
        },
        Transform::from_xyz(0., 0., 0.),
    ));
}

fn setup_egui(mut contexts: EguiContexts, mut state: ResMut<ApplicationState>) -> Result {
    egui::Window::new("Layers")
        .movable(false)
        .resizable(false)
        .show(contexts.ctx_mut()?, |ui| {
            ui.vertical_centered_justified(|ui| {
                for layer in state.layers.iter() {
                    let button = egui::Button::new(layer.name.clone());
                    ui.add(button);
                }
            });

            if state.layers.len() > 0 {
                ui.separator();
            }

            if ui.button("New Layer").clicked() {
                state.layers.push(PuppetLayer::default());
            }
        });
    return Ok(());
}

fn setup_2d_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::LinearRgba(LinearRgba::rgb(0., 0., 0.))))
        .init_resource::<ApplicationState>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Aang".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(EguiPlugin::default())
        .add_systems(Startup, (setup_2d_camera, setup_layer_components))
        .add_systems(EguiPrimaryContextPass, setup_egui)
        .run();
}
