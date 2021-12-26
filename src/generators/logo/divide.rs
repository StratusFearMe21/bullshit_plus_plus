use rand::prelude::IteratorRandom;
use rand::{distributions::Standard, prelude::Distribution};
use std::fmt::Display;
use tiny_skia::Pixmap;
use unchecked_unwrap::UncheckedUnwrap;

macro_rules! grab_photos {
    ($a:expr,$b:expr,$rng:expr) => {
        if let Some(ref library) = unsafe { crate::OPTS.get_unchecked() }.logo {
            $a(std::fs::read_dir(library)
                .unwrap()
                .filter_map(|f| {
                    let fr = unsafe { f.unchecked_unwrap() };
                    if fr.file_name().to_str().unwrap().ends_with(".png") {
                        Some(fr)
                    } else {
                        None
                    }
                })
                .choose_multiple($rng, $b)
                .into_iter()
                .map(|i| {
                    let pixmap = Pixmap::decode_png(unsafe {
                        &memmap::Mmap::map(&std::fs::File::open(i.path()).unwrap()).unwrap()
                    })
                    .unwrap();
                    let width = $rng.gen_range(-(pixmap.width() as i32) + 400..0);
                    let height = $rng.gen_range(-(pixmap.height() as i32) + 400..0);
                    (pixmap, width, height)
                })
                .collect())
        } else {
            $a(super::unsplash::Unsplash::photo($b)
                .unwrap()
                .into_iter()
                .map(|i| {
                    (
                        i.0,
                        $rng.gen_range(-i.1 + 400..0),
                        $rng.gen_range(-i.2 + 400..0),
                    )
                })
                .collect())
        }
    };
}

pub enum Divide {
    VertHalf(Vec<(Pixmap, i32, i32)>),
    HorHalf(Vec<(Pixmap, i32, i32)>),
    DiagRToL(Vec<(Pixmap, i32, i32)>),
    DiagLToR(Vec<(Pixmap, i32, i32)>),
}

impl Divide {
    pub fn photos(&self) -> &Vec<(Pixmap, i32, i32)> {
        match self {
            Self::VertHalf(photos) => photos,
            Self::HorHalf(photos) => photos,
            Self::DiagLToR(photos) => photos,
            Self::DiagRToL(photos) => photos,
        }
    }
}

impl Distribution<Divide> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Divide {
        match rng.gen_range(0..4) {
            0 => grab_photos!(Divide::VertHalf, 2, rng),
            1 => grab_photos!(Divide::HorHalf, 2, rng),
            2 => grab_photos!(Divide::DiagRToL, 2, rng),
            3 => grab_photos!(Divide::DiagLToR, 2, rng),
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}

impl Display for Divide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            //Self::None => write!(f, "None"),
            Self::HorHalf(..) => write!(f, "Horizontal Halves"),
            Self::VertHalf(..) => write!(f, "Vertical Halves"),
            Self::DiagRToL(..) => write!(f, "Diagonal Right to Left"),
            Self::DiagLToR(..) => write!(f, "Diagonal Left to Right"),
            //Self::HorThird => write!(f, "Horizontal Thirds"),
        }
    }
}
