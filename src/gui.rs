use sdl2::{
    pixels::Color,
    rect::{FRect, Point, Rect},
    render::Canvas,
    video::Window,
};

use crate::{
    text::{GlobalTextManager, GlobalyLoadedFonts},
    typedefs::Orientation,
};
const PROGRESS_THICKNESS: u32 = 14;

pub fn draw_progress(
    value: f32,
    pos: Point,
    color: Color,
    orientation: Orientation,
    lenght: u32,
    canvas: &mut Canvas<Window>,
    // Render the progress from the end of the bar to the beginning
    reversed: bool,
) -> Result<(), String> {
    if value < 0.0 || value > 1.0 {
        return Err(format!("Value should be btw 0 & 1. got {value}"));
    }

    let (x, y) = (pos.x, pos.y);
    let (w, h) = match orientation {
        Orientation::Horizontal => (lenght, PROGRESS_THICKNESS),
        Orientation::Vertical => (PROGRESS_THICKNESS, lenght),
    };

    // draw base
    canvas.set_draw_color(Color::GRAY);
    canvas.fill_rect(Rect::new(x, y, w, h))?;

    let level = value * lenght as f32;
    let (level_w, level_h) = match orientation {
        Orientation::Horizontal => (level, PROGRESS_THICKNESS as f32),
        Orientation::Vertical => (PROGRESS_THICKNESS as f32, level),
    };
    let (level_x, level_y) = if reversed {
        let ly = (y as f32 + h as f32) - level_h;
        let lx = (x as f32 + w as f32) - level_w;
        match orientation {
            Orientation::Horizontal => (lx, y as f32),
            Orientation::Vertical => (x as f32, ly),
        }
    } else {
        (x as f32, y as f32)
    };

    // level
    canvas.set_draw_color(color);
    canvas.fill_frect(FRect::new(level_x, level_y, level_w, level_h))?;

    return Ok(());
}

pub fn draw_select_area<'a, 'b, S: AsRef<str>>(
    label: S,
    area: Rect,
    canvas: &mut Canvas<Window>,
    tm: &GlobalTextManager<'a, 'b>,
    active: bool,
) -> Result<(), String> {
    let corner_rect = Rect::new(0, 0, 4, 4);
    canvas.draw_rect(area)?;

    if active {
        canvas.fill_rect(corner_rect.centered_on(area.top_left()))?;
        canvas.fill_rect(corner_rect.centered_on(area.top_right()))?;
        canvas.fill_rect(corner_rect.centered_on(area.bottom_left()))?;
        canvas.fill_rect(corner_rect.centered_on(area.bottom_right()))?;

        tm.write(label, GlobalyLoadedFonts::Tarzeau12)
            .color(Color::GRAY)
            .position(area.x, area.y - 20)
            .render(canvas)?;
    }

    return Ok(());
}
