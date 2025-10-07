use sdl2::{
    event::Event,
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
};

use crate::{
    gui::GuiWidget,
    text::{GlobalTextManager, GlobalyLoadedFonts},
};

pub struct Button {
    pub text: String,
    pub rect: Rect,
    pub is_hovered: bool,
    pub font: GlobalyLoadedFonts,
}

impl Button {
    pub fn new(text: String, rect: Rect) -> Self {
        return Button {
            text,
            rect,
            is_hovered: false,
            font: GlobalyLoadedFonts::Tarzeau16,
        };
    }

    pub fn is_clicked(&self, event: Event) -> bool {
        return match event {
            Event::MouseButtonDown { x, y, .. } => self.rect.contains_point(Point::new(x, y)),
            _ => false,
        };
    }
}

impl GuiWidget for Button {
    fn draw<'a, 'b>(
        &self,
        canvas: &mut Canvas<Window>,
        tm: &GlobalTextManager<'a, 'b>,
    ) -> Result<(), String> {
        let texture_creator = canvas.texture_creator();
        canvas.set_draw_color(if self.is_hovered {
            Color::GREEN
        } else {
            Color::RGB(50, 50, 50)
        });

        canvas.fill_rect(self.rect)?;
        let (surf, rect) = tm
            .write(&self.text, self.font.clone())
            .position(self.rect.x + 5, self.rect.y - 2)
            .color(if self.is_hovered {
                Color::BLACK
            } else {
                Color::WHITE
            })
            .surface()?;

        let texture = surf
            .as_texture(&texture_creator)
            .map_err(|e| e.to_string())?;
        canvas.copy(&texture, None, Some(rect.centered_on(self.rect.center())))?;
        return Ok(());
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::MouseMotion { x, y, .. } => {
                self.is_hovered = self.rect.contains_point(Point::new(x, y));
            }
            _ => {}
        }
    }
}
