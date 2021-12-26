use crate::generators::actions::ActionFirst;
use rand_derive2::RandGen;
use std::fmt::Display;

#[derive(RandGen)]
pub enum Ing {
    WordInMiddle(WordInMiddleBegin, ActionFirst, WordInMiddleEnd),
}

impl Display for Ing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WordInMiddle(begin, action, end) => {
                write!(f, "{} ", begin)?;
                action.ing(f)?;
                write!(f, " {}", end)
            }
        }
    }
}

#[derive(RandGen)]
pub enum WordInMiddleBegin {
    Finally,
    Yes,
    Presenting,
    Wow,
    Idiomatic,
    Reliable,
    Better,
    Fast,
    BattleTested,
    None,
}

impl Display for WordInMiddleBegin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Finally => write!(f, "Finally,"),
            Self::Yes => write!(f, "Yes,"),
            Self::Presenting => write!(f, "Presenting,"),
            Self::Wow => write!(f, "Wow,"),
            Self::Idiomatic => write!(f, "Idiomatic"),
            Self::Reliable => write!(f, "Reliable"),
            Self::Better => write!(f, "Better"),
            Self::Fast => write!(f, "Fast"),
            Self::BattleTested => write!(f, "Battle-tested"),
            Self::None => Ok(()),
        }
    }
}

#[derive(RandGen)]
pub enum WordInMiddleEnd {
    ForTheModernWeb,
    MadeEasy,
    ForYourAverageJoe,
    YouCantDeny,
    ThatWontScrewYouOver,
    ThatDoesntHateYou,
    ThatActuallyWorks,
    ThatJustWorks,
    ForEveryone,
}

impl Display for WordInMiddleEnd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::ForTheModernWeb => write!(f, "for the modern web!"),
            Self::MadeEasy => write!(f, "made easy!"),
            Self::ForYourAverageJoe => write!(f, "for your average Joe!"),
            Self::YouCantDeny => write!(f, "you can't deny"),
            Self::ThatWontScrewYouOver => write!(f, "that won't screw you over"),
            Self::ThatDoesntHateYou => write!(f, "that doesn't hate you"),
            Self::ThatActuallyWorks => write!(f, "that actually works"),
            Self::ThatJustWorks => write!(f, "that just works"),
            Self::ForEveryone => write!(f, "for everyone!"),
        }
    }
}
