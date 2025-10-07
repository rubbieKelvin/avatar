use sdl2::{
    pixels::{Color, PixelFormatEnum},
    rect::{Point, Rect},
    render::{Canvas, TextureCreator},
    surface::Surface,
    video::{Window, WindowContext},
};

use crate::{
    editor::{configpanels::ConfigPanel, constants::MAX_AUDIO_LEVEL},
    gui::button::Button,
    puppet::{Component, ComponentKind, Puppet},
    timer::Timer,
    typedefs::DragState,
};

struct LayerButton {
    kind: ComponentKind,
    rect: Rect,
    hovered: bool,
}

mod configpanels;
mod constants;
mod draw;
mod events;
mod utils;

// pub struct PuppetComponentPanel {
//     position: Point,
//     component_buttons: Vec<Button>,
// }

// impl PuppetComponentPanel {
//     fn new(x: i32, y: i32, w: u32, h: u32) -> Result<Self, String> {
//         return Ok(Self {
//             position: Point::new(x, y),
//             component_buttons: vec![],
//         });
//     }

//     fn set_component_buttons(&mut self, components: &Vec<Component>) {
//         self.component_buttons = components
//             .iter()
//             .enumerate()
//             .map(|(index, component)| {
//                 let y = 20 + (40 * index as i32) + (10 * index as i32);
//                 Button::new(component.kind.to_string(), Rect::new(20, y, 80, 40))
//             })
//             .collect();
//     }

//     fn draw(
//         &self,
//         canvas: &mut Canvas<Window>,
//         components: &Vec<Component>,
//         active_component: ComponentKind,
//     ) -> Result<(), String> {
//         for button in self.component_buttons {
//             let is_active = component.kind == active_component;
//             canvas.set_draw_color(Color::GREEN);

//             if button.is_hovered {
//                 let texture = self.surface.as_texture(canvas.texture_creator())?;
//             }
//         }
//         return Ok(());
//     }
// }

pub struct Editor {
    puppet: Puppet,
    active_puppet_component: ComponentKind,
    active_puppet_dragstate: Option<DragState>,
    audio_level: f32,             // audio level in db
    audio_level_set_timer: Timer, // TODO: i dont have mic input, so i'll use this to simulate values for audio for now
    workspace_rect: Rect,         // the area the puppet is displayed
    layer_button_area: Vec<LayerButton>,
    texture_creator: TextureCreator<WindowContext>,
    config_panel: ConfigPanel,
}

impl Editor {
    pub fn new(canvas: &mut Canvas<Window>) -> Editor {
        let canvas_viewport = canvas.viewport();
        let texture_creator = canvas.texture_creator();
        let puppet = Puppet::default(); // idealy we can load from file
        let layer_button_area = puppet
            .components
            .iter()
            .enumerate()
            .map(|(i, component)| {
                // y is initial top-padding + incremental y + spacing btw text
                let y = 20 + (40 * i as i32) + (10 * i as i32);

                return LayerButton {
                    kind: component.kind,
                    rect: Rect::new(20, y, 80, 40),
                    hovered: false,
                };
            })
            .collect::<Vec<LayerButton>>();

        // TODO: remove this when we have actual audio levels
        let mut audio_level_set_timer = Timer::new(200.);
        audio_level_set_timer.is_loop = true;
        audio_level_set_timer.play();

        return Editor {
            layer_button_area,
            audio_level_set_timer,
            active_puppet_component: ComponentKind::default(),
            active_puppet_dragstate: None,
            audio_level: 0.,
            puppet,
            config_panel: ConfigPanel::new(canvas_viewport.w - 340, 0, 340),
            workspace_rect: Rect::new(0, 0, 800, 800).centered_on(canvas_viewport.center()), // we'll set this after having access to the canvas
            texture_creator,
        };
    }

    pub fn process(&mut self, delta_ms: f32) {
        // TODO: remove
        // increase or decrease the audio level by a random value
        if self.audio_level_set_timer.is_triggered() {
            let mut new_audio_level = self.audio_level + rand::random_range(-5..5) as f32;

            // check just so we're within range
            if new_audio_level <= 0.0 {
                new_audio_level = rand::random_range(1..5) as f32;
            }
            if new_audio_level >= MAX_AUDIO_LEVEL {
                new_audio_level = rand::random_range(95..99) as f32;
            }

            self.audio_level = new_audio_level;
        }

        self.audio_level_set_timer.tick(delta_ms);
    }
}
