use sdl2::{Sdl, event::Event, keyboard::Keycode, pixels::Color, render::Canvas, video::Window};
use std::{thread, time::Duration};

enum PersonState {
    Blinking,
    Talking,
}

struct Application {
    context: Sdl,
    running: bool,
    canvas: Canvas<Window>,
}

impl Application {
    /// Initialize application
    fn init() -> Result<Self, String> {
        let context = sdl2::init()?;
        let videosub = context.video()?;
        let window = videosub
            .window("Avatar", 1200, 800)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        return Ok(Application {
            canvas,
            context,
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

    /// Mainloop
    fn mainloop(&mut self) -> Result<(), String> {
        let mut event_pump = self.context.event_pump()?;

        while self.running {
            self.canvas.clear();
            self.canvas.set_draw_color(Color::BLACK);

            for event in event_pump.poll_iter() {
                self.handle_events(event);
            }

            self.canvas.present();
            thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }

        return Ok(());
    }
}

fn main() -> Result<(), String> {
    let mut state = Application::init()?;
    state.mainloop()?;

    return Ok(());
}
