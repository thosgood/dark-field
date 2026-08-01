use crate::locations::*;
use crate::movement::*;
use crate::player::*;

pub struct EyePhoto(pub Vec<Vec<char>>);

pub struct FPV<'a> {
    player: &'a Player,
    location: &'a Location,
    view: EyePhoto,
}
