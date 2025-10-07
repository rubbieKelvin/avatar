use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
};

use crate::{
    gui::GuiWidget,
    text::{GlobalTextManager, GlobalyLoadedFonts},
};

pub struct TextInput {
    pub rect: Rect,
    pub focused: bool,
    pub hovered: bool,
    pub cursor: u16,
    pub font: GlobalyLoadedFonts,
}

impl TextInput {
    pub fn new(rect: Rect) -> Self {
        return Self {
            rect,
            cursor: 0,
            focused: false,
            hovered: false,
            font: GlobalyLoadedFonts::Tarzeau16,
        };
    }

    pub fn draw_text<'a, 'b>(
        &self,
        text: String,
        canvas: &mut Canvas<Window>,
        tm: &GlobalTextManager<'a, 'b>,
    ) -> Result<(), String> {
        canvas.set_draw_color(if self.focused {
            Color::GREEN
        } else {
            if self.hovered {
                Color::RGB(100, 100, 100)
            } else {
                Color::RGB(50, 50, 50)
            }
        });
        canvas.draw_rect(self.rect)?;
        tm.write(&text, self.font.clone())
            .position(self.rect.x + 4, self.rect.y + 2)
            .color(if self.focused {
                Color::WHITE
            } else {
                Color::GRAY
            })
            .render(canvas)?;
        return Ok(());
    }

    pub fn on_text_change(&mut self, text: &mut String, event: Event) {
        match event {
            Event::KeyDown {
                keycode: Some(Keycode::BACKSPACE),
                ..
            }
            | Event::KeyDown {
                keycode: Some(Keycode::DELETE),
                ..
            } => {
                if !self.focused || text.is_empty() {
                    return;
                }
                *text = text.chars().take(text.len() - 1).collect();
            }
            Event::TextInput {
                text: text_char, ..
            } => {
                if !self.focused {
                    return;
                }
                text.push_str(&text_char);
            }
            _ => {}
        }
    }
}

impl GuiWidget for TextInput {
    fn draw<'a, 'b>(
        &self,
        _canvas: &mut Canvas<Window>,
        _tm: &GlobalTextManager<'a, 'b>,
    ) -> Result<(), String> {
        // canvas.set_draw_color(if self.focused {
        //     Color::GREEN
        // } else {
        //     if self.hovered {
        //         Color::RGB(100, 100, 100)
        //     } else {
        //         Color::RGB(50, 50, 50)
        //     }
        // });
        // canvas.draw_rect(self.rect)?;
        // tm.write(&self.text, self.font.clone())
        //     .position(self.rect.x + 4, self.rect.y + 2)
        //     .color(if self.focused {
        //         Color::WHITE
        //     } else {
        //         Color::GRAY
        //     })
        //     .render(canvas)?;
        // return Ok(());
        return Err("Call draw_text instead".to_string());
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::MouseButtonDown { x, y, .. } => {
                self.focused = self.rect.contains_point(Point::new(x, y));
            }
            Event::MouseMotion { x, y, .. } => {
                self.hovered = self.rect.contains_point(Point::new(x, y));
            }
            _ => {}
        }
    }
}
