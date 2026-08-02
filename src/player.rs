use crate::locations::*;
use crate::movement::*;
use crate::photos::*;

use std::fmt;

const WALKING_SPEED: f64 = 1.0;
const TURNING_SPEED: f64 = std::f64::consts::FRAC_PI_4;

#[derive(Debug)]
pub struct Player {
    pub discrete_position: DiscretePosition,
    pub real_position: RealPosition,
    pub discrete_direction: DiscreteDirection,
    pub real_direction: RealDirection,
    pub eyesight: usize,
    pub debug: String,
    // TODO: pointer to map: &Location?
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let step = self.real_direction.unit_vector();
        let tentative_real_position = RealPosition {
            x: self.real_position.x + step.0,
            y: self.real_position.y + step.1,
        };
        let tentative_discrete_position = tentative_real_position.nearest_discrete_position();
        write!(
            f,
            "Real position: {:?}
Real direction: {:?}
Real direction vector: {:?}
Next real step: {:?}\n
Grid position: {:?}
Grid direction: {:?}
Next grid step: {:?}\n
Debug: {:?}",
            (self.real_position.x, self.real_position.y),
            (self.real_direction.0 * 180.0 / std::f64::consts::PI),
            step,
            (tentative_real_position.x, tentative_real_position.y),
            (self.discrete_position.x, self.discrete_position.y),
            self.discrete_direction,
            (tentative_discrete_position.x, tentative_real_position.y),
            self.debug,
        )
    }
}

impl Player {
    pub fn new(
        location: &Location,
        starting_x: usize,
        starting_y: usize,
        discrete_direction: DiscreteDirection,
        eyesight: usize,
    ) -> Self {
        let discrete_position = location
            .force_can_walk_on(&DiscretePosition {
                x: starting_x,
                y: starting_y,
            })
            .unwrap()
            .clone();
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

    pub fn plan_step(&mut self) -> (RealPosition, DiscretePosition) {
        let step = self.real_direction.unit_vector();
        let tentative_real_position = RealPosition {
            x: self.real_position.x + step.0 * WALKING_SPEED,
            y: self.real_position.y + step.1 * WALKING_SPEED,
        };
        let tentative_discrete_position = tentative_real_position.nearest_discrete_position();
        (tentative_real_position, tentative_discrete_position)
    }

    pub fn take_step(&mut self, map: &Location) {
        let (tentative_real_position, tentative_discrete_position) = self.plan_step();

        if map.is_obstacle(&tentative_discrete_position) {
            self.debug = format!("obstacle: {:?}", &tentative_discrete_position);
        }
        if !map.is_in_bounds(&tentative_discrete_position) {
            self.debug = "out of bounds".to_string();
        }

        if map.is_in_bounds(&tentative_discrete_position)
            && !map.is_obstacle(&tentative_discrete_position)
        {
            self.discrete_position = tentative_discrete_position;
            self.real_position = tentative_real_position;
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
    pub fn _turn_discrete_left(&mut self) {
        self.discrete_direction = match self.discrete_direction {
            DiscreteDirection::North => DiscreteDirection::West,
            DiscreteDirection::East => DiscreteDirection::North,
            DiscreteDirection::South => DiscreteDirection::East,
            DiscreteDirection::West => DiscreteDirection::South,
        };
        self.real_direction = RealDirection::from_discrete_direction(&self.discrete_direction);
    }

    pub fn _turn_discrete_right(&mut self) {
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

    pub fn take_bird_photo(&self, map: &Location) -> BirdPhoto {
        let BirdPhoto(mut photo) =
            BirdPhoto::take_photo(map, &(self.discrete_position), self.eyesight, self.eyesight);
        // Because we specify photos as having centre points, but we access points
        // in a `Vec<Vec<_>>` by working from the top left, we always need to shift
        // by the (half) size of the photo (which, here, is given by `eyesight`).
        photo[self.eyesight][self.eyesight] = self.selfie();
        BirdPhoto(photo)
    }

    pub fn take_eye_photo(&self, map: &Location) -> EyePhoto {
        EyePhoto::take_photo(
            map,
            &(self.discrete_position),
            &(self.real_direction),
            self.eyesight,
            40,
            15,
        )
    }
}
