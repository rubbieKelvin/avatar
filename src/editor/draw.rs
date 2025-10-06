use sdl2::{pixels::Color, rect::Point, render::Canvas, video::Window};

use crate::{
    editor::{Editor, constants::MAX_AUDIO_LEVEL},
    gui::{draw_progress, draw_select_area},
    text::{GlobalTextManager, GlobalyLoadedFonts},
    typedefs::Orientation,
};

impl Editor {
    pub fn draw<'a, 'b>(
        &self,
        canvas: &mut Canvas<Window>,
        tm: &GlobalTextManager<'a, 'b>,
    ) -> Result<(), String> {
        self._draw_button(canvas, tm)?;
        self._draw_audio_level(canvas)?;
        self._draw_puppet_canvas(canvas)?;
        self._draw_puppet(canvas, tm)?;

        return Ok(());
    }

    fn _draw_button<'a, 'b>(
        &self,
        canvas: &mut Canvas<Window>,
        tm: &GlobalTextManager<'a, 'b>,
    ) -> Result<(), String> {
        for button in &self.layer_button_area {
            let is_active = button.kind == self.active_puppet_component;
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
                .as_texture(&self.texture_creator)
                .map_err(|e| e.to_string())?;

            canvas.copy(&texture, None, Some(rect))?;
        }

        return Ok(());
    }

    fn _draw_audio_level(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let canvas_viewport = canvas.viewport();

        let audio_level_percent = self.audio_level / MAX_AUDIO_LEVEL;
        draw_progress(
            audio_level_percent,
            Point::new(20, 300),
            match audio_level_percent {
                n if n < 0.25 => Color::BLUE,
                n if n < 0.80 => Color::GREEN,
                _ => Color::RED,
            },
            Orientation::Vertical,
            (canvas_viewport.h - 340) as u32,
            canvas,
            true,
        )?;

        return Ok(());
    }

    fn _draw_puppet_canvas(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let workspace_rect = self.workspace_rect;
        let center = workspace_rect.center();

        canvas.set_draw_color(Color::RGB(50, 50, 50));
        canvas.draw_rect(workspace_rect)?;
        canvas.draw_line(
            Point::new(center.x, workspace_rect.top()),
            Point::new(center.x, workspace_rect.bottom()),
        )?;
        canvas.draw_line(
            Point::new(workspace_rect.left(), center.y),
            Point::new(workspace_rect.right(), center.y),
        )?;

        return Ok(());
    }

    fn _draw_puppet<'a, 'b>(
        &self,
        canvas: &mut Canvas<Window>,
        tm: &GlobalTextManager<'a, 'b>,
    ) -> Result<(), String> {
        for component in self.puppet.components.iter() {
            canvas.set_draw_color(Color::RGB(100, 100, 100));
            draw_select_area(
                component.kind.to_string(),
                self.use_canvas_coord_for_rect(component.rect()),
                canvas,
                tm,
                self.active_puppet_component == component.kind,
            )?;
        }
        return Ok(());
    }
}
