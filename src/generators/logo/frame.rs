use std::fmt::Display;

use rand_derive2::RandGen;

#[derive(RandGen)]
pub enum Frame {
    Circle,
    Square,
    Diamond,
    Triangle,
    RoundedTriangle,
    Squircle,
    DiamondSquircle,
    RoundedRectangle,
}

impl Display for Frame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Circle => write!(f, "Circle"),
            Self::Square => write!(f, "Square"),
            Self::Diamond => write!(f, "Diamond"),
            Self::Triangle => write!(f, "Triangle"),
            Self::RoundedTriangle => write!(f, "Rounded triangle"),
            Self::Squircle => write!(f, "Squircle"),
            Self::DiamondSquircle => write!(f, "Diamond squircle"),
            Self::RoundedRectangle => write!(f, "Rounded rectangle"),
        }
    }
}
