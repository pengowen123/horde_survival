use gui::UIShape;

pub struct Align {
    offset: [f32; 2],
    align: AlignType,
}

pub enum AlignType {
    Center,
    BottomRight,
}

impl Align {
    pub fn new(align: AlignType) -> Self {
        Align {
            offset: [0.0; 2],
            align: align,
        }
    }

    pub fn center() -> Self {
        Align::new(AlignType::Center)
    }

    pub fn bottom_right() -> Self {
        Align::new(AlignType::BottomRight)
    }

    // NOTE: Offset is measured in units of object widths/heights
    pub fn with_offset(mut self, x: f32, y: f32) -> Self {
        self.offset = [x, y];
        self
    }
}

impl Align {
    pub fn apply<T: UIShape>(self, object: &mut T) {
        let dim = object.dimensions();
        let cx = dim[0] / 2.0;
        let cy = dim[1] / 2.0;
        let offset_x = self.offset[0] * dim[0];
        let offset_y = self.offset[1] * dim[1];

        let pos = match self.align {
            AlignType::Center => [-cx + offset_x, -cy + offset_y],
            AlignType::BottomRight => [1.0 + -dim[0] - offset_x, -1.0 + offset_y],
        };

        object.set_position(pos);
    }
}