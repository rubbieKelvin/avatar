use sdl2::{event::Event, rect::Rect, render::Canvas, video::Window};

use crate::{
    gui::{GuiWidget, button::Button, text_input::TextInput},
    puppet::Component,
    text::{GlobalTextManager, GlobalyLoadedFonts},
};

pub struct ConfigPanel {
    pub x: i32,
    pub y: i32,
    add_state_button: Button,
    state_edit_input: TextInput,
}

impl ConfigPanel {
    pub fn new(x: i32, y: i32, w: u32) -> ConfigPanel {
        let mut add_state_button = Button::new(
            "Add State".to_string(),
            Rect::new((x + w as i32) - 100, y + 20, 90, 22),
        );
        add_state_button.font = GlobalyLoadedFonts::Tarzeau12;

        let state_edit_input =
            TextInput::new("State Name".to_string(), Rect::new(x, y + 80, w - 20, 30));
        return ConfigPanel {
            x,
            y,
            add_state_button,
            state_edit_input,
        };
    }

    pub fn draw<'a, 'b>(
        &self,
        canvas: &mut Canvas<Window>,
        component: &Component,
        tm: &GlobalTextManager<'a, 'b>,
    ) -> Result<(), String> {
        tm.write(component.kind.to_string(), GlobalyLoadedFonts::Tarzeau16)
            .position(self.x, self.y + 20)
            .render(canvas)?;

        // draw widgets
        self.add_state_button.draw(canvas, tm)?;
        self.state_edit_input.draw(canvas, tm)?;

        return Ok(());
    }

    pub fn handle_event(&mut self, event: Event) {
        self.add_state_button.handle_event(event.clone());
        self.state_edit_input.handle_event(event.clone());
    }
}
