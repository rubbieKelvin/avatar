use sdl2::{pixels::Color, render::Canvas, video::Window};

pub struct Layer {
    name: String,
}

pub struct Editor {
    layers: Vec<Layer>,
}

impl Editor {
    pub fn new() -> Self {
        return Editor {
            layers: ["Head", "Eyes", "Mouth", "Body", "Hat"]
                .iter()
                .map(|name| Layer {
                    name: name.to_string(),
                })
                .collect(),
        };
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let vp = canvas.viewport();
        canvas.set_draw_color(Color::RED);
        canvas.draw_rect(vp)?;
        return Ok(());
    }
}
