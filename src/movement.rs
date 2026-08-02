use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI};

#[derive(Debug)]
pub enum DiscreteDirection {
    North,
    East,
    South,
    West,
}

/// The real direction is the angle in radians, measured anti-clockwise from due east.
/// Note that that the real direction pi/2, i.e. due north, should cause us to travel in
/// the direction of *decreasing* y, due to the flipped y-coordinates in our map.
#[derive(Debug)]
pub struct RealDirection(pub f64);

impl RealDirection {
    pub fn nearest_discrete_direction(&self) -> DiscreteDirection {
        // We rotate the angle by pi/4 so that we can simply check which quadrant it
        // lies in, rather than which axis it's closest to.
        let rotated = RealDirection(self.0 + FRAC_PI_4);
        let unit_vector = rotated.unit_vector();
        let s_cos = unit_vector.0.is_sign_positive();
        // Recall that `RealDirection::unit_vector` flips the y-coordinate,
        // so we need to compensate for that by flipping again here.
        let s_sin = unit_vector.1.is_sign_negative();
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
            DiscreteDirection::East => RealDirection(0f64),
            DiscreteDirection::South => RealDirection(-FRAC_PI_2),
            DiscreteDirection::West => RealDirection(PI),
        }
    }

    pub fn unit_vector(&self) -> (f64, f64) {
        // Recall that the y-axis is flipped in our map coordinates: increasing y
        // means moving south!
        (self.0.cos(), -self.0.sin())
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
    pub x: f64,
    pub y: f64,
}

impl RealPosition {
    pub fn from_discrete_position(discrete_position: &DiscretePosition) -> Self {
        // TODO: check this is ok? what is the usize is too big...
        RealPosition {
            x: discrete_position.x as f64,
            y: discrete_position.y as f64,
        }
    }

    pub fn nearest_discrete_position(&self) -> DiscretePosition {
        DiscretePosition {
            x: self.x.round() as usize,
            y: self.y.round() as usize,
        }
    }
}
