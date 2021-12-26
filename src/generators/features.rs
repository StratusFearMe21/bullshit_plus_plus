use std::fmt::Display;

use rand_derive2::RandGen;

use super::{actions::ActionFirst, full_names::Name};

#[derive(RandGen)]
pub enum Feature {
    FirstLastFeature(Name, ActionFirst),
    FastFeature(ActionFirst),
}

#[derive(RandGen)]
pub struct Features {
    pub feature_1: Feature,
    pub feature_2: Feature,
    pub feature_3: Feature,
    pub feature_4: Feature,
}

impl Display for Features {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}, {}, {}, {}",
            self.feature_1, self.feature_2, self.feature_3, self.feature_4
        )
    }
}

impl Display for Feature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::FirstLastFeature(ref name, ref action) => {
                write!(f, "{} ", name)?;
                action.ing(f)
            }
            Self::FastFeature(ref action) => {
                write!(f, "Fast ")?;
                action.ing(f)
            }
        }
    }
}
