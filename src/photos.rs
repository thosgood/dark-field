use crate::locations::*;
use crate::movement::*;

use std::iter;

// We think of BirdPhoto` as being indexed like a matrice, i.e. with the
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

    pub fn take_photo(
        location: &Location,
        centre: &DiscretePosition,
        half_height: usize,
        half_width: usize,
    ) -> Self {
        // We interpret the half_height and half_width as excluding the space that
        // the player occupies, i.e. every photo will be of odd height and odd width.
        let photo_height = 2 * half_height + 1;
        let photo_width = 2 * half_width + 1;

        let mut cropped_photo: Vec<Vec<char>> = Vec::new();
        let empty_space = location.empty_space;
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
                    if location.is_in_bounds(&point) {
                        cropped_line.push(location.ascii_map[point.y][point.x]);
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
}

// Unlike `BirdPhoto`, we interpret `EyePhoto` as a list of *columns* of characters.
// This means that when we want to display it (with `to_vec_string`) we will need to
// do some sort of transpose type operation.
type EyePhotoColumn = Vec<char>;
pub struct EyePhoto(pub Vec<EyePhotoColumn>);

impl EyePhoto {
    // TODO: these single characters should eventually be replaced with arrays (for distance rendering)
    const EMPTY_SPACE_CHAR: char = '.';
    const OBSTACLE_CHAR: char = '0';
    const FLOOR_CHAR: char = '-';

    pub fn max_height(&self) -> usize {
        self.0
            .iter()
            .map(|column| column.len())
            .fold(0usize, |acc, x| acc.max(x))
    }

    pub fn min_height(&self) -> usize {
        self.0
            .iter()
            .map(|column| column.len())
            .fold(self.max_height(), |acc, x| acc.min(x))
    }

    // Because EyePhoto consists of columns of data, we need to sort of "transpose" it to get the
    // desired output, namely a vector of strings to display as *rows*.
    pub fn to_vec_string(&self) -> Vec<String> {
        let height = self.min_height();
        let width = self.0.len();
        (0..height)
            .map(|row| {
                (0..width)
                    .map(|column| self.0[column][row])
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
    }

    pub fn take_photo(
        location: &Location,
        position: &DiscretePosition,
        direction: &RealDirection,
        eyesight: usize,
        half_width: usize,
        half_height: usize,
    ) -> EyePhoto {
        let epsilon = 0.5;
        // We're not gonna worry about overflow or anything safe like that.
        let fov = (half_width * 2 + 1) as f64;
        let eyesight = eyesight as f64;
        // To get (2*half_width + 1) columns, we need to cast that many rays.
        let ray_directions = (0..=(2 * half_width))
            .map(|n| direction.0.sin().asin() + (n as f64 - half_width as f64) / fov)
            .collect::<Vec<f64>>();

        // Now we do some simple raycasting.
        let what_how_far: Vec<(LocationItem, f64)> = ray_directions
            .into_iter()
            .map(|ray_direction| {
                let delta = RealDirection(ray_direction).unit_vector();
                let mut distance = epsilon;
                let mut point = RealPosition {
                    x: (position.x as f64) + epsilon * delta.0,
                    y: (position.y as f64) + epsilon * delta.1,
                };

                // TODO: another magic number (maybe use self.eyesight when calling this as a player?)
                while distance < eyesight {
                    let discrete_point = point.nearest_discrete_position();
                    let item = location.what_is_here(&discrete_point);
                    match item {
                        LocationItem::EmptySpace => break,
                        LocationItem::Obstacle => break,
                        LocationItem::Floor => {
                            point = RealPosition {
                                x: point.x + epsilon * delta.0,
                                y: point.y + epsilon * delta.1,
                            };
                            distance += epsilon;
                        }
                    }
                }
                (
                    location.what_is_here(&(point.nearest_discrete_position())),
                    distance,
                )
            })
            .collect::<Vec<_>>();

        let distance_into_length = {
            |dist: f64| {
                ((2.0 * dist / eyesight).atan()
                    * std::f64::consts::FRAC_PI_2
                    * (half_height as f64))
                    .floor() as usize
            }
        };

        // Finally, we "render" the columns.
        let columns: Vec<EyePhotoColumn> = what_how_far
            .into_iter()
            .map(|(item, distance)| {
                match item {
                    // TODO: rewrite all the following to actually change the symbol depending on distance
                    LocationItem::EmptySpace => {
                        // Upper half.
                        let mut column = iter::repeat(Self::EMPTY_SPACE_CHAR)
                            .take(half_height)
                            .collect::<EyePhotoColumn>();
                        // Lower half.
                        let breakpoint = distance_into_length(distance);
                        for i in 0..half_height {
                            if i <= breakpoint {
                                column.push(Self::EMPTY_SPACE_CHAR)
                            } else {
                                column.push(Self::FLOOR_CHAR)
                            }
                        }
                        column
                    }
                    LocationItem::Obstacle => {
                        let mut column: Vec<char> = Vec::new();
                        let breakpoint = distance_into_length(distance);
                        for i in 0..(half_height * 2) {
                            if i <= half_height.saturating_sub(breakpoint) {
                                column.push(Self::EMPTY_SPACE_CHAR);
                            } else if i <= half_height + breakpoint {
                                column.push(Self::OBSTACLE_CHAR);
                            } else {
                                column.push(Self::FLOOR_CHAR);
                            }
                        }
                        // Upper half.
                        let mut column = iter::repeat(Self::OBSTACLE_CHAR)
                            .take(half_height)
                            .collect::<EyePhotoColumn>();
                        // Lower half.
                        let mut lower_half = column.clone();
                        lower_half.reverse();
                        column.append(&mut lower_half);
                        column
                    }
                    _ => {
                        // TODO: this would be bad
                        iter::repeat('!')
                            .take(half_height * 2)
                            .collect::<EyePhotoColumn>()
                    }
                }
            })
            .collect::<Vec<_>>();
        EyePhoto(columns)
    }
}
