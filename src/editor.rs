use sdl2::{
    event::Event,
    pixels::Color,
    rect::{Point, Rect},
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext},
};

use crate::{
    gui::{draw_progress, draw_select_area},
    puppet::{Component, ComponentKind, Puppet},
    text::{GlobalTextManager, GlobalyLoadedFonts},
    timer::Timer,
    typedefs::{DragState, Orientation},
};

struct LayerButton {
    kind: ComponentKind,
    rect: Rect,
    hovered: bool,
}

// NOTE: I dont know this yet, so i'll leave it at 100
const MAX_AUDIO_LEVEL: f32 = 100.0;

pub struct Editor {
    puppet: Puppet,
    active_puppet_component: ComponentKind,
    active_puppet_dragstate: Option<DragState>,
    audio_level: f32,             // audio level in db
    audio_level_set_timer: Timer, // TODO: i dont have mic input, so i'll use this to simulate values for audio for now
    workspace_rect: Rect,         // the area the puppet is displayed
    layer_button_area: Vec<LayerButton>,
    texture_creator: TextureCreator<WindowContext>,
}

impl Editor {
    pub fn new(canvas: &mut Canvas<Window>) -> Editor {
        let canvas_viewport = canvas.viewport();
        let texture_creator = canvas.texture_creator();
        let puppet = Puppet::default(); // idealy we can load from file
        let layer_button_area = puppet
            .components
            .iter()
            .enumerate()
            .map(|(i, component)| {
                // y is initial top-padding + incremental y + spacing btw text
                let y = 20 + (40 * i as i32) + (10 * i as i32);

                return LayerButton {
                    kind: component.kind,
                    rect: Rect::new(20, y, 80, 40),
                    hovered: false,
                };
            })
            .collect::<Vec<LayerButton>>();

        // TODO: remove this when we have actual audio levels
        let mut audio_level_set_timer = Timer::new(200.);
        audio_level_set_timer.is_loop = true;
        audio_level_set_timer.play();

        return Editor {
            layer_button_area,
            audio_level_set_timer,
            active_puppet_component: ComponentKind::default(),
            active_puppet_dragstate: None,
            audio_level: 0.,
            puppet,
            workspace_rect: Rect::new(0, 0, 800, 800).centered_on(canvas_viewport.center()), // we'll set this after having access to the canvas
            texture_creator,
        };
    }

    pub fn handle_events(&mut self, event: Event) {
        match event {
            Event::MouseMotion { x, y, .. } => {
                self.check_layer_text_surface_hover(x, y);
                self.update_active_component_drag_state(x, y);
            }
            Event::MouseButtonDown { x, y, .. } => {
                self.check_layer_text_surface_click(x, y);
                self.check_puppet_component_mouse_down(x, y);
            }
            Event::MouseButtonUp { .. } => {
                // if there's a drag state, then we should release it
                if self.active_puppet_dragstate.is_some() {
                    self.active_puppet_dragstate = None;
                }
            }
            _ => {}
        }
    }

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

    #[allow(unused)]
    /// in the workspace, the coordinate begins at the center.
    /// so we have to offset every rect at the workspace center
    fn use_canvas_coord_for_rect(&self, rect: Rect) -> Rect {
        let center = self.workspace_rect.center();
        return Rect::new(
            rect.x + center.x,
            rect.y + center.y,
            rect.w as u32,
            rect.h as u32,
        );
    }

    #[allow(unused)]
    /// same as the above but for points
    fn use_canvas_coord_for_point(&self, point: Point) -> Point {
        let center = self.workspace_rect.center();
        return Point::new(point.x + center.x, point.y + center.y);
    }

    #[allow(unused)]
    fn get_active_component(&self) -> Option<&Component> {
        return self
            .puppet
            .components
            .iter()
            .find(|c| c.kind == self.active_puppet_component);
    }

    #[allow(unused)]
    fn get_active_component_mut(&mut self) -> Option<&mut Component> {
        return self
            .puppet
            .components
            .iter_mut()
            .find(|c| c.kind == self.active_puppet_component);
    }

    pub fn process(&mut self, delta_ms: f32) {
        // TODO: remove
        // increase or decrease the audio level by a random value
        if self.audio_level_set_timer.is_triggered() {
            let mut new_audio_level = self.audio_level + rand::random_range(-5..5) as f32;

            // check just so we're within range
            if new_audio_level <= 0.0 {
                new_audio_level = rand::random_range(1..5) as f32;
            }
            if new_audio_level >= MAX_AUDIO_LEVEL {
                new_audio_level = rand::random_range(95..99) as f32;
            }

            self.audio_level = new_audio_level;
        }

        self.audio_level_set_timer.tick(delta_ms);
    }

    fn check_layer_text_surface_hover(&mut self, x: i32, y: i32) {
        let mut picked = false;
        for button in self.layer_button_area.iter_mut() {
            if picked {
                // if we've checked hover and we picked a button at some point
                // just set the rest to false instead of recalculating
                button.hovered = false;
            } else {
                let hovered = button.rect.contains_point(Point::new(x, y));
                button.hovered = hovered;
                if hovered {
                    picked = hovered
                }
            }
        }
    }

    fn check_layer_text_surface_click(&mut self, x: i32, y: i32) {
        for button in self.layer_button_area.iter_mut() {
            if button.rect.contains_point(Point::new(x, y)) {
                self.active_puppet_component = button.kind;
                // once we find the active layer just quit checking
                break;
            }
        }
    }

    fn check_puppet_component_mouse_down(&mut self, x: i32, y: i32) {
        for component in self.puppet.components.iter() {
            // add canvas offset
            let rect = self.use_canvas_coord_for_rect(component.rect());
            if rect.contains_point(Point::new(x, y)) {
                // select this component and start dragstate
                self.active_puppet_component = component.kind;
                self.active_puppet_dragstate = Some(DragState::start(x, y));
                break;
            }
        }
    }

    fn update_active_component_drag_state(&mut self, x: i32, y: i32) {
        // if there's a drag state for active puppet, update
        if self.active_puppet_dragstate.is_none() {
            return;
        }

        // take ownershipt from self.active_puppet_dragstate
        let drag_state = self.active_puppet_dragstate.take();
        let mut drag_state = drag_state.unwrap();

        let (dx, dy) = drag_state.dxdy(x, y);
        let active_component = self.get_active_component_mut().unwrap();
        let active_component_state = active_component.get_state_mut();

        active_component_state.position.0 += dx;
        active_component_state.position.1 += dy;
        drag_state.reset(x, y);

        // pass ownership back to self
        self.active_puppet_dragstate = Some(drag_state);
    }
}
