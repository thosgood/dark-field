#[derive(Debug)]
pub enum DiscreteDirection {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone)]
pub struct DiscretePosition {
    pub x: usize,
    pub y: usize,
}

impl DiscretePosition {
    pub fn try_new(x: isize, y: isize) -> Option<Self> {
        if x >= 0 && y >= 0 {
            Some(DiscretePosition {
                x: x as usize,
                y: y as usize,
            })
        } else {
            None
        }
    }
}

// TODO: impl `Iterator` by spiraling around or something
//       (probably use `max_width` and stuff)

#[derive(Debug)]
pub struct RealPosition {
    pub x: f32,
    pub y: f32,
}

impl RealPosition {
    pub fn from_grid_position(grid_position: &DiscretePosition) -> Self {
        // TODO: check this is ok? what is the usize is too big...
        RealPosition {
            x: grid_position.x as f32,
            y: grid_position.y as f32,
        }
    }

    pub fn nearest_grid_position(&self) -> DiscretePosition {
        DiscretePosition {
            x: self.x.round() as usize,
            y: self.y.round() as usize,
        }
    }
}
