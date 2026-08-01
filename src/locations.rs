use crate::Either;
use crate::movement::DiscretePosition;

pub const LEVEL_MAP: &str = r#"......     0     0     
......   0    0    0  0
......      0  0   0   
......  0      0      0
......      0     0    ......        
......     0     0                   
         0    0    0  0......        
 .....      0id0ido0est
 .....  0sindum0ninser0
 .....      0flaer0dost
 .....     0ipsum0dolor
         0amet0luus0to0
......quanto0id0ido0estq
......ic0sindum0ninser0
......floran0flaer0dost"#;

// We think of `Map` and `Photo` as being indexed like matrices, i.e. with the
// top-left-most character being (0,0) and the bottom-right-most being (w, h),
// where w = width and h = height. We realise this by modelling them as a list
// of rows, where each row is a list of `char`.
pub struct BirdPhoto(pub Vec<Vec<char>>);

impl BirdPhoto {
    pub fn to_vec_string(&self) -> Vec<String> {
        self.0
            .iter()
            .map(|line| line.iter().collect::<String>())
            .collect::<Vec<String>>()
    }
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

    pub fn take_photo(
        &self,
        centre: &DiscretePosition,
        half_height: usize,
        half_width: usize,
    ) -> BirdPhoto {
        // We interpret the half_height and half_width as excluding the space that
        // the player occupies, i.e. every photo will be of odd height and odd width.
        let photo_height = 2 * half_height + 1;
        let photo_width = 2 * half_width + 1;

        let mut cropped_photo: Vec<Vec<char>> = Vec::new();
        let empty_space = self.empty_space;
        // We build up the cropped photo by just checking if each coordinate is in bounds
        // or not and then pushing the character at that coordinate or the `empty_space`
        // character (respectively).
        for j in 0..photo_height {
            let mut cropped_line: Vec<char> = Vec::new();
            for i in 0..photo_width {
                let point_x = (centre.x + i)
                    .checked_signed_diff(half_width)
                    .expect("horizontal numbers should never be this big");
                let point_y = (centre.y + j)
                    .checked_signed_diff(half_height)
                    .expect("vertical numbers should never be this big");
                if let Some(point) = DiscretePosition::try_new(point_x, point_y) {
                    if self.is_in_bounds(&point) {
                        cropped_line.push(self.ascii_map[point.y][point.x]);
                    } else {
                        cropped_line.push(empty_space);
                    }
                } else {
                    cropped_line.push(empty_space);
                }
            }
            cropped_photo.push(cropped_line);
        }

        BirdPhoto(cropped_photo)
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

    pub fn can_walk_on(&self, position: &DiscretePosition) -> bool {
        if self.is_in_bounds(position) {
            let character = self
                .ascii_map
                .get(position.y)
                .expect("`is_in_bounds` made a mistake")
                .get(position.x)
                .expect("`is_in_bounds` made a mistake");
            *character != self.obstacle
        } else {
            false
        }
    }
}
