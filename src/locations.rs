use crate::Either;
use crate::movement::*;

// We index ASCII maps in the same way we index `Photos::BirdPhoto`: like matrices.
pub const LEVEL_MAP: &str = r#"......                   
......   0000000   0000  
......      0  0   0     
......  00     0   0  0  
......     00     0      ......        
......     0     00                  
         000000    0  0......        
 .....         00000
 .....  0sindum0ninser0
 .....      0flaer0dost
 .....     0ipsum0dolor
         0amet0luus0to0
......quanto0id0ido0estq
......ic0sindum0ninser0
......floran0flaer0dost"#;

#[derive(PartialEq, Eq)]
pub enum LocationItem {
    Obstacle,
    EmptySpace,
    Floor,
}

#[derive(Clone)]
pub struct Location {
    pub ascii_map: Vec<Vec<char>>,
    pub empty_space: char,
    pub obstacle: char,
}

impl Location {
    pub fn from_string(ascii: &str, empty_space: char, obstacle: char) -> Self {
        let lines: Vec<Vec<char>> = ascii
            .split("\n")
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect();
        Location {
            ascii_map: lines,
            empty_space,
            obstacle,
        }
    }

    pub fn _max_width(&self) -> usize {
        self.ascii_map
            .iter()
            .map(|line| line.len())
            .fold(0usize, |acc, x| acc.max(x))
    }

    pub fn _min_width(&self) -> usize {
        self.ascii_map
            .iter()
            .map(|line| line.len())
            .fold(self._max_width(), |acc, x| acc.min(x))
    }

    pub fn _naive_height(&self) -> usize {
        self.ascii_map.len()
    }

    pub fn force_in_bounds(
        &self,
        position: &DiscretePosition,
    ) -> Either<DiscretePosition, DiscretePosition> {
        if let Some(line) = self.ascii_map.get(position.y)
            && let Some(character) = line.get(position.x)
            && *character != self.empty_space
        {
            return Either::Left(position.clone());
        }
        // TODO: make this actually compute the nearest point... maybe just by
        //       spiralling outwards?
        Either::Right(DiscretePosition { x: 0, y: 0 })
    }

    pub fn is_in_bounds(&self, position: &DiscretePosition) -> bool {
        match self.force_in_bounds(position) {
            Either::Left(_) => true,
            Either::Right(_) => false,
        }
    }

    pub fn force_can_walk_on(
        &self,
        position: &DiscretePosition,
    ) -> Either<DiscretePosition, DiscretePosition> {
        // TODO: do you actually need this much information?  i.e. "remembering" the result of
        //       `force_in_bounds` in the `Left` clause?
        match self.force_in_bounds(position) {
            Either::Left(original) => {
                let character = self
                    .ascii_map
                    .get(original.y)
                    .expect("`is_in_bounds` made a vertical mistake")
                    .get(original.x)
                    .expect("`is_in_bounds` made a horizontal mistake");
                if *character == self.obstacle {
                    // TODO: make this also actually compute the nearest point
                    Either::Right(DiscretePosition { x: 1, y: 1 })
                } else {
                    Either::Left(original)
                }
            }
            Either::Right(forced_in_bounds) => {
                let character = self
                    .ascii_map
                    .get(forced_in_bounds.y)
                    .expect("`is_in_bounds` made a vertical mistake")
                    .get(forced_in_bounds.x)
                    .expect("`is_in_bounds` made a horizontal mistake");
                if *character == self.obstacle {
                    // TODO: make this also also actually compute the nearest point
                    Either::Right(DiscretePosition { x: 2, y: 2 })
                } else {
                    Either::Right(forced_in_bounds)
                }
            }
        }
    }

    // Note that being an obstacle requires being in bounds.
    pub fn is_obstacle(&self, position: &DiscretePosition) -> bool {
        if self.is_in_bounds(position) {
            let character = self
                .ascii_map
                .get(position.y)
                .expect("`is_in_bounds` made a mistake")
                .get(position.x)
                .expect("`is_in_bounds` made a mistake");
            *character == self.obstacle
        } else {
            false
        }
    }

    pub fn what_is_here(&self, position: &DiscretePosition) -> LocationItem {
        if !self.is_in_bounds(position) {
            return LocationItem::EmptySpace;
        };
        if self.is_obstacle(position) {
            return LocationItem::Obstacle;
        };
        LocationItem::Floor
    }
}
