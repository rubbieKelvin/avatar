use sdl2::rect::{Point, Rect};
use std::path::PathBuf;
use strum::IntoEnumIterator;

#[derive(strum::Display, strum::EnumIter, Default, PartialEq, Clone, Copy)]
pub enum ComponentKind {
    Head,
    #[default]
    Hat,
    Eyes,
    Mouth,
}

// a component can have a state, ie: mouth_open, mouth_close and sprites to account for each
#[derive(Default)]
pub struct State {
    id: String,
    filepath: PathBuf,
    pub position: (i32, i32),
}

// A component is a part of the avatar's body
#[derive(Default)]
pub struct Component {
    pub kind: ComponentKind,
    pub zindex: u8,
    pub default_state: State,
    pub active_state: Option<String>, // if none: default state, else the state id
    pub states: Vec<State>,
}

impl Component {
    pub fn rect(&self) -> Rect {
        // default value
        // TODO: if there's an image surface, override
        let r = Rect::new(0, 0, 10, 10);
        let position = self.get_state().position;
        return r.centered_on(Point::new(position.0, position.1));
    }

    pub fn get_state(&self) -> &State {
        if let Some(state_id) = &self.active_state {
            let state = self.states.iter().find(|s| &s.id == state_id);

            if let Some(state) = state {
                return state;
            }
        }
        return &self.default_state;
    }

    pub fn get_state_mut(&mut self) -> &mut State {
        if let Some(state_id) = &mut self.active_state {
            let state = self.states.iter_mut().find(|s| &s.id == state_id);

            if let Some(state) = state {
                return state;
            }
        }
        return &mut self.default_state;
    }
}

pub struct Puppet {
    pub components: Vec<Component>,
}

impl Default for Puppet {
    fn default() -> Self {
        return Puppet {
            components: ComponentKind::iter()
                .enumerate()
                .map(|(i, kind)| Component {
                    kind,
                    zindex: i as u8,
                    ..Default::default()
                })
                .collect(),
        };
    }
}
