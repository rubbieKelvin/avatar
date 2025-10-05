use sdl2::{
    event::Event,
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
};
use strum::IntoEnumIterator;

use crate::{
    gui::draw_progress,
    text::{GlobalTextManager, GlobalyLoadedFonts},
    timer::Timer,
    typedefs::Orientation,
};

#[derive(strum::Display, strum::EnumIter, Default, PartialEq, Clone, Copy)]
enum Layers {
    #[default]
    Hat,
    Head,
    Eyes,
    Mouth,
}

struct LayerButton {
    kind: Layers,
    rect: Rect,
    hovered: bool,
}

// NOTE: I dont know this yet, so i'll leave it at 100
const MAX_AUDIO_LEVEL: f32 = 100.0;

#[derive(Default)]
pub struct Editor {
    active_layer: Layers,
    audio_level: f32,             // audio level in db
    audio_level_set_timer: Timer, // TODO: i dont have mic input, so i'll use this to simulate values for audio for now
    layer_button_area: Vec<LayerButton>,
}

impl Editor {
    pub fn new() -> Editor {
        let layer_button_area = Layers::iter()
            .enumerate()
            .map(|(i, layer)| {
                // y is initial top-padding + incremental y + spacing btw text
                let y = 20 + (40 * i as i32) + (10 * i as i32);

                return LayerButton {
                    kind: layer,
                    rect: Rect::new(20, y, 80, 40),
                    hovered: false,
                };
            })
            .collect::<Vec<LayerButton>>();

        // TODO: remove this when we have actual audio levels
        let mut audio_level_set_timer = Timer::new(1000.);
        audio_level_set_timer.is_loop = true;
        audio_level_set_timer.play();

        return Editor {
            layer_button_area,
            audio_level_set_timer,
            ..Default::default()
        };
    }

    pub fn handle_events(&mut self, event: Event) {
        match event {
            Event::MouseMotion { x, y, .. } => {
                self.check_layer_text_surface_hover(x, y);
            }
            Event::MouseButtonDown { x, y, .. } => {
                self.check_layer_text_surface_click(x, y);
            }
            _ => {}
        }
    }

    pub fn draw<'a, 'b>(
        &self,
        canvas: &mut Canvas<Window>,
        tm: &GlobalTextManager<'a, 'b>,
    ) -> Result<(), String> {
        let canvas_viewport = canvas.viewport();
        let texture_creator = canvas.texture_creator();

        // draw button
        for button in &self.layer_button_area {
            let is_active = button.kind == self.active_layer;
            canvas.set_draw_color(Color::GREEN);

            if button.hovered {
                canvas.draw_rect(button.rect)?;
            }

            if is_active {
                canvas.fill_rect(button.rect)?;
            }

            let center = button.rect.center();

            let (surf, rect) = tm
                .write(&button.kind.to_string(), GlobalyLoadedFonts::Tarzeau16)
                .position(center.x, center.y)
                .color(if is_active {
                    Color::BLACK
                } else {
                    Color::WHITE
                })
                .centered()
                .surface()?;

            let texture = surf
                .as_texture(&texture_creator)
                .map_err(|e| e.to_string())?;

            canvas.copy(&texture, None, Some(rect))?;
        }

        // draw audio level
        let audio_level_percent = self.audio_level / MAX_AUDIO_LEVEL;
        draw_progress(
            audio_level_percent,
            Point::new(20, 300),
            match audio_level_percent {
                n if n < 0.25 => Color::BLUE,
                n if n < 0.70 => Color::GREEN,
                _ => Color::RED,
            },
            Orientation::Vertical,
            (canvas_viewport.h - 340) as u32,
            canvas,
            true,
        )?;

        return Ok(());
    }

    pub fn process(&mut self, delta_ms: f32) {
        // TODO: remove
        // increase or decrease the audio level by a random value
        if self.audio_level_set_timer.is_triggered() {
            let mut new_audio_level = self.audio_level + rand::random_range(-30..30) as f32;

            // check just so we're within range
            if new_audio_level <= 0.0 {
                new_audio_level = rand::random_range(2..10) as f32;
            }
            if new_audio_level >= MAX_AUDIO_LEVEL {
                new_audio_level = rand::random_range(80..100) as f32;
            }

            self.audio_level = new_audio_level;
        }

        self.audio_level_set_timer.tick(delta_ms);
    }

    fn check_layer_text_surface_hover(&mut self, x: i32, y: i32) {
        let mut picked = false;
        for button in self.layer_button_area.iter_mut() {
            if picked {
                // if we've checked hover and we picked a button at some point
                // just set the rest to false instead of recalculating
                button.hovered = false;
            } else {
                let hovered = button.rect.contains_point(Point::new(x, y));
                button.hovered = hovered;
                if hovered {
                    picked = hovered
                }
            }
        }
    }

    fn check_layer_text_surface_click(&mut self, x: i32, y: i32) {
        for button in self.layer_button_area.iter_mut() {
            if button.rect.contains_point(Point::new(x, y)) {
                self.active_layer = button.kind;
                // once we find the active layer just quit checking
                break;
            }
        }
    }
}
