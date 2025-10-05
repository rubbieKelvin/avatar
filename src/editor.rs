use sdl2::{
    event::Event,
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
};
use strum::IntoEnumIterator;

use crate::text::{GlobalTextManager, GlobalyLoadedFonts};

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

#[derive(Default)]
pub struct Editor {
    active_layer: Layers,
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
                    rect: Rect::new(40, y, 80, 40),
                    hovered: false,
                };
            })
            .collect::<Vec<LayerButton>>();

        return Editor {
            layer_button_area,
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
        return Ok(());
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
