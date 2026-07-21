use crate::movement::*;
use crate::Either;

#[derive(Debug)]
pub struct Player {
    pub grid_position: GridPosition,
    pub real_position: RealPosition,
    pub direction: Direction,
    pub eyesight: usize,
    pub debug: String,
}

impl Player {
    pub fn new(
        location: &Location,
        starting_x: usize,
        starting_y: usize,
        direction: Direction,
        eyesight: usize,
    ) -> Self {
        // TODO: isn't there a slicker way to do this? surely
        let grid_position = match location.force_can_walk_on(&GridPosition {
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
        let tentative_position = match self.direction {
            Direction::North => GridPosition {
                x: self.grid_position.x,
                y: (self.grid_position.y).saturating_sub(1),
            },
            Direction::East => GridPosition {
                x: self.grid_position.x + 1,
                y: self.grid_position.y,
            },
            Direction::South => GridPosition {
                x: self.grid_position.x,
                y: self.grid_position.y + 1,
            },
            Direction::West => GridPosition {
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
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };
    }

    pub fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
    }

    pub fn selfie(&self) -> char {
        match self.direction {
            Direction::North => '⮝',
            Direction::East => '⮞',
            Direction::South => '⮟',
            Direction::West => '⮜',
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
