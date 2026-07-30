use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI};

#[derive(Debug)]
pub enum DiscreteDirection {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
pub struct RealDirection(pub f32);

impl RealDirection {
    pub fn nearest_discrete_direction(&self) -> DiscreteDirection {
        // We rotate the angle by pi/4 so that we can simply check which quadrant it
        // lies in, rather than which axis it's closest to.
        let (sin, cos) = (self.0 + FRAC_PI_4).sin_cos();
        let s_cos = cos.is_sign_positive();
        let s_sin = sin.is_sign_positive();
        match (s_cos, s_sin) {
            // Bottom-left quadrant => West.
            (false, false) => DiscreteDirection::West,
            // Top-left quadrant => North.
            (false, true) => DiscreteDirection::North,
            // Bottom-right quadrant => South.
            (true, false) => DiscreteDirection::South,
            // Top-right quadrant => East.
            (true, true) => DiscreteDirection::East,
        }
    }

    pub fn from_discrete_direction(direction: &DiscreteDirection) -> Self {
        match direction {
            DiscreteDirection::North => RealDirection(FRAC_PI_2),
            DiscreteDirection::East => RealDirection(0f32),
            DiscreteDirection::South => RealDirection(-FRAC_PI_2),
            DiscreteDirection::West => RealDirection(PI),
        }
    }
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
    pub fn from_discrete_position(discrete_position: &DiscretePosition) -> Self {
        // TODO: check this is ok? what is the usize is too big...
        RealPosition {
            x: discrete_position.x as f32,
            y: discrete_position.y as f32,
        }
    }

    pub fn nearest_discrete_position(&self) -> DiscretePosition {
        DiscretePosition {
            x: self.x.round() as usize,
            y: self.y.round() as usize,
        }
    }
}
