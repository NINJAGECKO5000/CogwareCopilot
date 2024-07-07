mod displaylist;
mod plane;

use alloc::vec::Vec;
use displaylist::DisplayList;
pub use plane::*;

#[derive(Default)]
pub struct Hvs {
    planes: Vec<Plane>,
    display_list: DisplayList,
}

impl Hvs {
    pub fn new() -> Hvs {
        Hvs::default()
    }

    /// Add a new plane to the display list.
    ///
    /// NOTE: The order in which planes are added will determine the order they are drawn to the
    /// display. Planes added later will be drawn on top of planes added before.
    pub fn add_plane(&mut self, plane: Plane) {
        self.planes.push(plane);
    }

    pub fn draw(&mut self) {
        self.planes
            .iter()
            .for_each(|p| self.display_list.write_plane(p));
    }
}
