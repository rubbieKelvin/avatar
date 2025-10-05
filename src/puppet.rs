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
pub struct State {
    id: String,
    filepath: PathBuf,
}

// TODO: componenets should have default states
// A component is a part of the avatar's body
#[derive(Default)]
pub struct Component {
    pub kind: ComponentKind,
    pub zindex: u8,
    pub position: (i32, i32),
    pub states: Vec<State>,
}

impl Component {
    pub fn rect(&self) -> Rect {
        // default value
        // TODO: if there's an image surface, override
        let r = Rect::new(0, 0, 10, 10);
        return r.centered_on(Point::new(self.position.0, self.position.1));
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
