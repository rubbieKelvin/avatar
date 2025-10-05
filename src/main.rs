use sdl2::{
    Sdl, event::Event, keyboard::Keycode, pixels::Color, render::Canvas, ttf::Sdl2TtfContext,
    video::Window,
};
use std::{
    thread,
    time::{Duration, Instant},
};

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
            .window("Aang", 1200, 800)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        let texture_creator = canvas.texture_creator();
        let textmanager = GlobalTextManager::new(ttfctx, texture_creator)?;
        let editor = Editor::new();

        return Ok(Application {
            canvas,
            context: ctx,
            editor,
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
        // top level match
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

        // pass to editor
        self.editor.handle_events(event);
    }

    fn process(&mut self, _delta: f32) {}

    fn draw(&mut self) -> Result<(), String> {
        self.editor.draw(&mut self.canvas, &self.textmanager)?;
        return Ok(());
    }

    /// Mainloop
    fn mainloop(&mut self) -> Result<(), String> {
        let mut event_pump = self.context.event_pump()?;
        let mut last_frame_time = Instant::now();

        while self.running {
            // get timing
            let elasped_time = last_frame_time.elapsed();
            last_frame_time = Instant::now();

            // delta is the time one frame took...
            // 1.0 / dt is how many frames could fit into one second
            let dt = elasped_time.as_secs_f32();
            let fps = if dt > 0.0 { 1.0 / dt } else { 0.0 };

            // clear screen
            self.canvas.set_draw_color(Color::BLACK);
            self.canvas.clear();

            // process data (animations and stuff)
            self.process(dt);

            // process events
            for event in event_pump.poll_iter() {
                self.handle_events(event);
            }

            // draw fps??
            self.textmanager
                .write(format!("{fps:.2}"), GlobalyLoadedFonts::Tarzeau16)
                .position(120, 20)
                .color(Color::GRAY)
                .render(&mut self.canvas)?;

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
