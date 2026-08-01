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
}
