use sdl2::rect::{Point, Rect};

use crate::{editor::Editor, puppet::Component};

impl Editor {
    #[allow(unused)]
    /// in the workspace, the coordinate begins at the center.
    /// so we have to offset every rect at the workspace center
    pub(super) fn use_canvas_coord_for_rect(&self, rect: Rect) -> Rect {
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
    pub(super) fn use_canvas_coord_for_point(&self, point: Point) -> Point {
        let center = self.workspace_rect.center();
        return Point::new(point.x + center.x, point.y + center.y);
    }

    #[allow(unused)]
    pub(super) fn get_active_component(&self) -> Option<&Component> {
        return self
            .puppet
            .components
            .iter()
            .find(|c| c.kind == self.active_puppet_component);
    }

    #[allow(unused)]
    pub(super) fn get_active_component_mut(&mut self) -> Option<&mut Component> {
        return self
            .puppet
            .components
            .iter_mut()
            .find(|c| c.kind == self.active_puppet_component);
    }
}
