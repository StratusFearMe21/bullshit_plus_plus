mod ing;
use std::fmt::Display;

use ing::*;
mod er;
use er::*;

use rand_derive2::RandGen;

#[derive(RandGen)]
pub enum CatchPhrase {
    //Double(ActionFirst<'a>, ActionFirst<'a>),
    Ing(Ing),
    Er(Er),
}

impl Display for CatchPhrase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ing(ing) => write!(f, "{}", ing),
            Self::Er(er) => write!(f, "{}", er),
        }
    }
}
