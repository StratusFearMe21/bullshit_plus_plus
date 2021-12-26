use std::fmt::Display;

use rand::{
    distributions::Standard,
    prelude::{Distribution, SliceRandom},
};
use unchecked_unwrap::UncheckedUnwrap;

pub struct Name {
    pub prefix: &'static str,
    pub suffix: &'static str,
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            some_kind_of_uppercase_first_letter(self.prefix),
            self.suffix
        )
    }
}

impl Distribution<Name> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Name {
        Name {
            suffix: unsafe { super::prefixes::NAMEPARTS.choose(rng).unchecked_unwrap() },
            prefix: unsafe { super::prefixes::NAMEPARTS.choose(rng).unchecked_unwrap() },
        }
    }
}

fn some_kind_of_uppercase_first_letter(s: &'static str) -> String {
    let mut c = s.chars();
    unsafe {
        c.next()
            .unchecked_unwrap()
            .to_uppercase()
            .collect::<String>()
            + c.as_str()
    }
}
