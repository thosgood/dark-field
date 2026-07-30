use crate::Either;
use crate::locations::*;
use crate::movement::*;

#[derive(Debug)]
pub struct Player {
    pub grid_position: DiscretePosition,
    pub real_position: RealPosition,
    pub direction: DiscreteDirection,
    pub eyesight: usize,
    pub debug: String,
}

impl Player {
    pub fn new(
        location: &Location,
        starting_x: usize,
        starting_y: usize,
        direction: DiscreteDirection,
        eyesight: usize,
    ) -> Self {
        // TODO: isn't there a slicker way to do this? surely
        let grid_position = match location.force_can_walk_on(&DiscretePosition {
            x: starting_x,
            y: starting_y,
        }) {
            Either::Left(desired_point) => desired_point,
            Either::Right(forced_point) => forced_point,
        };
        let real_position = RealPosition::from_grid_position(&grid_position);

        Player {
            grid_position,
            real_position,
            direction,
            eyesight,
            debug: String::new(),
        }
    }

    pub fn take_step(&mut self, map: &Location) {
        // TODO: update this to also update real_position
        let tentative_position = match self.direction {
            DiscreteDirection::North => DiscretePosition {
                x: self.grid_position.x,
                y: (self.grid_position.y).saturating_sub(1),
            },
            DiscreteDirection::East => DiscretePosition {
                x: self.grid_position.x + 1,
                y: self.grid_position.y,
            },
            DiscreteDirection::South => DiscretePosition {
                x: self.grid_position.x,
                y: self.grid_position.y + 1,
            },
            DiscreteDirection::West => DiscretePosition {
                x: (self.grid_position.x).saturating_sub(1),
                y: self.grid_position.y,
            },
        };

        if !map.can_walk_on(&tentative_position) {
            self.debug = format!("obstacle: {:?}", &tentative_position);
        }
        if !map.is_in_bounds(&tentative_position) {
            self.debug = "out of bounds".to_string();
        }

        if map.is_in_bounds(&tentative_position) && map.can_walk_on(&tentative_position) {
            self.grid_position = tentative_position;
        }
    }

    // TODO: can't you do something like Direction.x => Direction.(x-1) ?
    //       (maybe even by just implementing Iterate? but surely simpler...)
    pub fn turn_left(&mut self) {
        self.direction = match self.direction {
            DiscreteDirection::North => DiscreteDirection::West,
            DiscreteDirection::East => DiscreteDirection::North,
            DiscreteDirection::South => DiscreteDirection::East,
            DiscreteDirection::West => DiscreteDirection::South,
        };
    }

    pub fn turn_right(&mut self) {
        self.direction = match self.direction {
            DiscreteDirection::North => DiscreteDirection::East,
            DiscreteDirection::East => DiscreteDirection::South,
            DiscreteDirection::South => DiscreteDirection::West,
            DiscreteDirection::West => DiscreteDirection::North,
        };
    }

    pub fn selfie(&self) -> char {
        match self.direction {
            DiscreteDirection::North => '⮝',
            DiscreteDirection::East => '⮞',
            DiscreteDirection::South => '⮟',
            DiscreteDirection::West => '⮜',
        }
    }

    pub fn take_photo(&self, map: &Location) -> Photo {
        let Photo(mut photo) = map.take_photo(&(self.grid_position), self.eyesight, self.eyesight);
        // Because we specify photos as having centre points, but we access points
        // in a `Vec<Vec<_>>` by working from the top left, we always need to shift
        // by the (half) size of the photo (which, here, is given by `eyesight`).
        photo[self.eyesight][self.eyesight] = self.selfie();
        Photo(photo)
    }
}
