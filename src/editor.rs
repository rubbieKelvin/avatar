use sdl2::{rect::Rect, render::Canvas, surface::Surface, video::Window};

use crate::text::{GlobalTextManager, GlobalyLoadedFonts};

pub struct Editor {
    section_text_surfaces: Vec<(Surface<'static>, Rect)>,
}

impl Editor {
    pub fn new<'a, 'b>(tm: &GlobalTextManager<'a, 'b>) -> Editor {
        let section_text_surfaces = ["hat", "head", "eyes", "mouth"]
            .iter()
            .enumerate()
            .map(|(i, text)| {
                // y is initial top-padding + incremental y + spacing btw text
                let y = 20 + (20 * i as i32) + (5 * i as i32);
                return tm
                    .write(text, GlobalyLoadedFonts::Tarzeau16)
                    .position(20, y)
                    .surface()
                    .unwrap();
            })
            .collect::<Vec<(Surface<'static>, Rect)>>();

        return Editor {
            section_text_surfaces,
        };
    }

    pub fn draw<'a, 'b>(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let texture_creator = canvas.texture_creator();

        for (surf, rect) in &self.section_text_surfaces {
            let texture = surf
                .as_texture(&texture_creator)
                .map_err(|e| e.to_string())?;

            canvas.copy(&texture, None, Some(*rect))?;
        }
        return Ok(());
    }
}
