use sdl2::{
    Sdl, event::Event, keyboard::Keycode, pixels::Color, render::Canvas, ttf::Sdl2TtfContext,
    video::Window,
};
use std::{thread, time::Duration};

use crate::{
    editor::Editor,
    text::{GlobalTextManager, GlobalyLoadedFonts},
};

mod editor;
mod text;

struct Application<'a, 'b> {
    context: &'a Sdl,
    running: bool,
    editor: Editor,
    textmanager: GlobalTextManager<'a, 'b>,
    canvas: Canvas<Window>,
}

impl<'a, 'b> Application<'a, 'b> {
    /// Initialize application
    fn init(ctx: &'a Sdl, ttfctx: &'a Sdl2TtfContext) -> Result<Self, String> {
        let videosub = ctx.video()?;
        let window = videosub
            .window("Avatar", 1200, 800)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        let texture_creator = canvas.texture_creator();
        let textmanager = GlobalTextManager::new(ttfctx, texture_creator)?;

        return Ok(Application {
            canvas,
            context: ctx,
            editor: Editor::new(),
            textmanager,
            running: true,
        });
    }

    /// Quit application by turning running state
    fn quit(&mut self) {
        println!("Quitting application");
        self.running = false
    }

    /// Handle Ui events
    fn handle_events(&mut self, event: Event) {
        match event {
            // Quit application on ui quit or Esc click
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::ESCAPE),
                ..
            } => {
                self.quit();
            }
            // ignore otther shi
            _ => {}
        }
    }

    fn draw(&mut self) -> Result<(), String> {
        self.editor.draw(&mut self.canvas)?;
        self.textmanager
            .write("Rubbie", GlobalyLoadedFonts::Tarzeau20)
            .render(&mut self.canvas)?;
        return Ok(());
    }

    /// Mainloop
    fn mainloop(&mut self) -> Result<(), String> {
        let mut event_pump = self.context.event_pump()?;

        while self.running {
            // clear screen
            self.canvas.clear();
            self.canvas.set_draw_color(Color::BLACK);

            // process events
            for event in event_pump.poll_iter() {
                self.handle_events(event);
            }

            // draw
            self.draw()?;

            // present from membuf
            self.canvas.present();
            thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }

        return Ok(());
    }
}

fn main() -> Result<(), String> {
    let context = sdl2::init()?;
    let ttfctx = sdl2::ttf::init()?;
    let mut state = Application::init(&context, &ttfctx)?;
    state.mainloop()?;

    return Ok(());
}
