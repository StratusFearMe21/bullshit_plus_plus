use std::fmt::Display;

use rand_derive2::RandGen;

#[derive(RandGen)]
pub enum Room {
    Basement,
    Kitchen,
    Bedroom,
    Landing,
    Front,
    TV,
    Bathroom,
    Dining,
}

impl Display for Room {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Basement => write!(f, "in their basement"),
            Self::Kitchen => write!(f, "in their kitchen"),
            Self::Bedroom => write!(f, "in their bedroom"),
            Self::Landing => write!(f, "on their landing"),
            Self::Front => write!(f, "in their front room"),
            Self::TV => write!(f, "in their TV room"),
            Self::Bathroom => write!(f, "in their bathroom"),
            Self::Dining => write!(f, "in their dining room"),
        }
    }
}
