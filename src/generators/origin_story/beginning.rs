use std::fmt::Display;

use rand_derive2::RandGen;

#[derive(RandGen)]
pub enum Beginning {
    ManicEpisode(super::rooms::Room),
    CarCrash(super::locations::Location),
    Divorce,
    StubbingToe(super::rooms::Room),
    HittingHead(super::rooms::Room),
}

impl Display for Beginning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ManicEpisode(room) => write!(f, "experiencing a manic episode {}", room),
            Self::CarCrash(location) => write!(f, "crashing their car into {}", location),
            Self::Divorce => write!(f, "experiencing a hard divorce"),
            Self::StubbingToe(room) => write!(f, "stubbing their toe {}", room),
            Self::HittingHead(room) => write!(f, "hitting their head {}", room),
        }
    }
}
