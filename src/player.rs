use crate::Either;
use crate::locations::*;
use crate::movement::*;

const WALKING_SPEED: f32 = 0.8;
const TURNING_SPEED: f32 = 0.3;

#[derive(Debug)]
pub struct Player {
    pub discrete_position: DiscretePosition,
    pub real_position: RealPosition,
    pub discrete_direction: DiscreteDirection,
    pub real_direction: RealDirection,
    pub eyesight: usize,
    pub debug: String,
}

impl Player {
    pub fn new(
        location: &Location,
        starting_x: usize,
        starting_y: usize,
        discrete_direction: DiscreteDirection,
        eyesight: usize,
    ) -> Self {
        // TODO: isn't there a slicker way to do this? surely
        let discrete_position = match location.force_can_walk_on(&DiscretePosition {
            x: starting_x,
            y: starting_y,
        }) {
            Either::Left(desired_point) => desired_point,
            Either::Right(forced_point) => forced_point,
        };
        let real_position = RealPosition::from_discrete_position(&discrete_position);
        let real_direction = RealDirection::from_discrete_direction(&discrete_direction);

        Player {
            discrete_position,
            real_position,
            discrete_direction,
            real_direction,
            eyesight,
            debug: String::new(),
        }
    }

    pub fn take_step(&mut self, map: &Location) {
        // TODO: update this to also update real_position
        let tentative_position = match self.discrete_direction {
            DiscreteDirection::North => DiscretePosition {
                x: self.discrete_position.x,
                y: (self.discrete_position.y).saturating_sub(1),
            },
            DiscreteDirection::East => DiscretePosition {
                x: self.discrete_position.x + 1,
                y: self.discrete_position.y,
            },
            DiscreteDirection::South => DiscretePosition {
                x: self.discrete_position.x,
                y: self.discrete_position.y + 1,
            },
            DiscreteDirection::West => DiscretePosition {
                x: (self.discrete_position.x).saturating_sub(1),
                y: self.discrete_position.y,
            },
        };

        if !map.can_walk_on(&tentative_position) {
            self.debug = format!("obstacle: {:?}", &tentative_position);
        }
        if !map.is_in_bounds(&tentative_position) {
            self.debug = "out of bounds".to_string();
        }

        if map.is_in_bounds(&tentative_position) && map.can_walk_on(&tentative_position) {
            self.discrete_position = tentative_position;
        }
    }

    pub fn turn_left(&mut self) {
        self.real_direction.0 += TURNING_SPEED;
        self.discrete_direction = self.real_direction.nearest_discrete_direction();
    }

    pub fn turn_right(&mut self) {
        self.real_direction.0 -= TURNING_SPEED;
        self.discrete_direction = self.real_direction.nearest_discrete_direction();
    }

    // TODO: can't you do something like Direction.x => Direction.(x-1) ?
    //       (maybe even by just implementing Iterate? but surely simpler...)
    pub fn turn_discrete_left(&mut self) {
        self.discrete_direction = match self.discrete_direction {
            DiscreteDirection::North => DiscreteDirection::West,
            DiscreteDirection::East => DiscreteDirection::North,
            DiscreteDirection::South => DiscreteDirection::East,
            DiscreteDirection::West => DiscreteDirection::South,
        };
        self.real_direction = RealDirection::from_discrete_direction(&self.discrete_direction);
    }

    pub fn turn_discrete_right(&mut self) {
        self.discrete_direction = match self.discrete_direction {
            DiscreteDirection::North => DiscreteDirection::East,
            DiscreteDirection::East => DiscreteDirection::South,
            DiscreteDirection::South => DiscreteDirection::West,
            DiscreteDirection::West => DiscreteDirection::North,
        };
        self.real_direction = RealDirection::from_discrete_direction(&self.discrete_direction);
    }

    pub fn selfie(&self) -> char {
        match self.discrete_direction {
            DiscreteDirection::North => '↑',
            DiscreteDirection::East => '→',
            DiscreteDirection::South => '↓',
            DiscreteDirection::West => '←',
        }
    }

    pub fn take_photo(&self, map: &Location) -> Photo {
        let Photo(mut photo) =
            map.take_photo(&(self.discrete_position), self.eyesight, self.eyesight);
        // Because we specify photos as having centre points, but we access points
        // in a `Vec<Vec<_>>` by working from the top left, we always need to shift
        // by the (half) size of the photo (which, here, is given by `eyesight`).
        photo[self.eyesight][self.eyesight] = self.selfie();
        Photo(photo)
    }
}
