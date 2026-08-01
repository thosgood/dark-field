use crate::locations::*;
use crate::movement::*;

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

// TODO: EyePhoto isn't really indexed: it's columns of data.
type EyePhotoColumn = Vec<char>;
pub struct EyePhoto(pub Vec<EyePhotoColumn>);

impl EyePhoto {
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
    ) -> EyePhoto {
        // TODO: raycast, i guess?
        EyePhoto(vec![vec!['a', 'b', 'c']])
    }
}
