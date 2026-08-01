use crate::locations::*;
use crate::movement::*;
use crate::player::*;

pub struct EyePhoto(pub Vec<Vec<char>>);

pub struct FPV {
    player: &Player,
    location: &Location,
    view: EyePhoto,
}
