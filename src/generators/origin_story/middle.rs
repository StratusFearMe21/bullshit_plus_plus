use std::fmt::Display;

use rand_derive2::RandGen;

use crate::generators::actions::{ActionFirst, ActionSecond};

#[derive(RandGen)]
pub struct Middle {
    beginning: Beginning,
    first: ActionFirst,
    second: &'static ActionSecond,
    third: ActionFirst,
}

#[derive(RandGen)]
pub enum Beginning {
    WhatIf(&'static ActionSecond),
    HowAbout(&'static ActionSecond),
}

impl Display for Beginning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::WhatIf(action) => write!(f, "what if I {}", action.ed),
            Self::HowAbout(action) => write!(f, "how about I {}", action.no_suffix),
        }
    }
}

impl Display for Middle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} the ", self.beginning)?;
        self.first.er(f)?;
        write!(f, " to {} the ", self.second.no_suffix)?;
        self.third.er(f)
    }
}
