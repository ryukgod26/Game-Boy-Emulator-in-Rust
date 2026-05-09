use crate::utils::bit;

#[cfg_attr(feature="serialize",derive(Serialize))]
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub enum Column{
    Zero,
    One,
}

#[cfg_attr(feature="serialize",derive(Serialize))]
#[derive(Copy,Clone,Debug)]
pub struct Joypad{
    pub column: Column,
    pub start: bool,
    pub select:bool,
    pub a: bool,
    pub b: bool,
    pub up: bool,
    pub down: bool,
    pub right: bool,
    pub left: bool,
}