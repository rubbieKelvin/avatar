#[allow(unused)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

pub struct DragState {
    initial_x: i32,
    initial_y: i32,
}

impl DragState {
    pub fn start(x: i32, y: i32) -> Self {
        return DragState {
            initial_x: x,
            initial_y: y,
        };
    }

    pub fn reset(&mut self, x: i32, y: i32) {
        self.initial_x = x;
        self.initial_y = y;
    }

    pub fn dxdy(&self, x: i32, y: i32) -> (i32, i32) {
        return (x - self.initial_x, y - self.initial_y);
    }
}
