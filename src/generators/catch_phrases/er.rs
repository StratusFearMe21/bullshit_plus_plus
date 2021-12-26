use std::fmt::Display;

use crate::generators::actions::ActionFirst;

use super::ing::WordInMiddleEnd;
use rand_derive2::RandGen;

#[derive(RandGen)]
pub enum Er {
    WordInMiddle(WordInMiddleBegin, ActionFirst, WordInMiddleEnd),
}

impl Display for Er {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WordInMiddle(begin, middle, end) => {
                write!(f, "{} ", begin)?;
                middle.er(f)?;
                write!(f, " {}", end)
            }
        }
    }
}

#[derive(RandGen)]
pub enum WordInMiddleBegin {
    FinallyA,
    TheWorldsFirst,
    WowA,
    PresentingThe,
    TheDefinitive,
    TheOneAndOnly,
    AReliable,
    ALoved,
    AFast,
    ABattleTested,
    TheMostReliable,
    TheVeryBest,
    A,
}

impl Display for WordInMiddleBegin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::FinallyA => write!(f, "Finally, a"),
            Self::TheWorldsFirst => write!(f, "The world's first"),
            Self::WowA => write!(f, "Wow! A"),
            Self::PresentingThe => write!(f, "Presenting, a"),
            Self::TheDefinitive => write!(f, "The definitive"),
            Self::TheOneAndOnly => write!(f, "The one and only"),
            Self::AReliable => write!(f, "A reliable"),
            Self::ALoved => write!(f, "A loved"),
            Self::AFast => write!(f, "A fast"),
            Self::ABattleTested => write!(f, "A battle-tested"),
            Self::TheMostReliable => write!(f, "The most reliable"),
            Self::TheVeryBest => write!(f, "The very best"),
            Self::A => write!(f, "A"),
        }
    }
}
