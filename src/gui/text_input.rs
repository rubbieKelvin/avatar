use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    sys::KeyCode,
    video::Window,
};

use crate::{
    gui::GuiWidget,
    text::{GlobalTextManager, GlobalyLoadedFonts},
};

pub struct TextInput {
    pub text: String,
    pub rect: Rect,
    pub focused: bool,
    pub hovered: bool,
    pub font: GlobalyLoadedFonts,
}

impl TextInput {
    pub fn new(text: String, rect: Rect) -> Self {
        return Self {
            text,
            rect,
            focused: false,
            hovered: false,
            font: GlobalyLoadedFonts::Tarzeau16,
        };
    }
}

impl GuiWidget for TextInput {
    fn draw<'a, 'b>(
        &self,
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
        tm.write(&self.text, self.font.clone())
            .position(self.rect.x + 4, self.rect.y + 2)
            .color(if self.focused {
                Color::WHITE
            } else {
                Color::GRAY
            })
            .render(canvas)?;
        return Ok(());
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::MouseButtonDown { x, y, .. } => {
                self.focused = self.rect.contains_point(Point::new(x, y));
            }
            Event::MouseMotion { x, y, .. } => {
                self.hovered = self.rect.contains_point(Point::new(x, y));
            }
            Event::KeyDown {
                keycode: Some(Keycode::BACKSPACE),
                ..
            }
            | Event::KeyDown {
                keycode: Some(Keycode::DELETE),
                ..
            } => {
                if !self.focused || self.text.is_empty() {
                    return;
                }
                self.text = self.text.chars().take(self.text.len() - 1).collect();
            }
            Event::TextInput { text, .. } => {
                if !self.focused {
                    return;
                }
                self.text.push_str(&text);
            }
            _ => {}
        }
    }
}
