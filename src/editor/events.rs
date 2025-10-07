use sdl2::{event::Event, rect::Point};

use crate::{editor::Editor, typedefs::DragState};

impl Editor {
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

        // let component = self.get_active_component_mut().unwrap();
        // let state = component.get_state_mut();
        // self.config_panel
        //     .handle_event(event.clone(), &mut state.name);
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
