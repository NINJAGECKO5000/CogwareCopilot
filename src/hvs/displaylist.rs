use super::Plane;

#[derive(Clone, Copy)]
enum Control {
    Valid = 1 << 30,
    Unity = 1 << 4,
    End = 1 << 31,
}

/// DisplayList is 16KiB in size.
type DisplayListMem = [u32; 4096];

pub struct DisplayList {
    mem: *mut DisplayListMem,
    offset: usize,
}

impl Default for DisplayList {
    fn default() -> Self {
        DisplayList::new()
    }
}

impl DisplayList {
    pub const fn new() -> DisplayList {
        DisplayList {
            mem: 0x3F402000 as *mut DisplayListMem,
            offset: 0,
        }
    }

    pub fn write_planes(&mut self, planes: &[Plane]) {
        for p in planes {
            self.write_plane(p);
        }

        self.write_word(Control::End as u32);
    }

    pub fn write_plane(&mut self, plane: &Plane) {
        let num_words: u32 = 7; // TODO: do we need more words than this sometimes?
        let ctl: u32 = Control::Valid as u32
            | Control::Unity as u32
            | (plane.order as u32) << 13
            | num_words << 24
            | (plane.format as u32);
        self.write_word(ctl);

        let pos0 = plane.start_x as u32 | (plane.start_y as u32) << 16;
        self.write_word(pos0);

        let pos2 = plane.width as u32 | (plane.height as u32) << 16;
        self.write_word(pos2);

        self.write_word(0xDEADBEEF);
        self.write_word((plane as *const Plane) as u32);
        self.write_word(0xDEADBEEF);
        self.write_word(plane.pitch as u32);
    }

    fn write_word(&mut self, word: u32) {
        unsafe {
            (*self.mem)[self.offset] = word;
        }
        self.offset += 1;
    }
}
