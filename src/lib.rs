use std::{convert::Infallible, fmt::Display, path::PathBuf, str::FromStr};

#[cfg(debug_assertions)]
use const_colors::yellow;
use const_colors::{bold, end};
use rand::Rng;
use rand_xoshiro::Xoshiro256PlusPlus;
use structopt::StructOpt;
use tiny_skia::Pixmap;

mod generators;

pub static OPTS: once_cell::sync::OnceCell<Opts> = once_cell::sync::OnceCell::new();

pub trait LogoMaker {
    fn create_logo<R: Rng + ?Sized>(&mut self, logo_map: &mut Pixmap, rng: &mut R);
}

pub trait PhotoProvider {
    fn provide_photo<R: Rng + ?Sized>(&mut self, rng: &mut R) -> Pixmap;
}

pub trait BSFormatter {
    fn format(&mut self, bsobj: BSObj, rng: &mut Xoshiro256PlusPlus);
}

#[derive(StructOpt, Debug)]
#[cfg_attr(not(debug_assertions), structopt(
    name = concat!(bold!(), "Bullshit++", end!()),
    about = "A feature rich bullshit generator inspired by the p9 bullshit command"
))]
#[cfg_attr(debug_assertions, structopt(
    name = concat!(bold!(), "Bullshit++", end!(), " ", yellow!(), "(Debug Version)", end!()),
    about = "A feature rich bullshit generator inspired by the p9 bullshit command"
))]
pub struct Opts {
    /// The format that your bullshit should be in
    #[structopt(short, long, default_value = "html")]
    pub format: Format,
    /// If you pass a file, that will be the logo, if you pass a directory, it will generatea logo
    /// using your pictures.
    #[structopt(short, long)]
    pub logo: Option<std::path::PathBuf>,
    /// The width of the border that the frame and the divider are in the logo
    #[structopt(short, long, default_value = "6")]
    pub border_width: f32,
    /// The random seed to use for generation
    #[structopt(short, long)]
    pub seed: Option<String>,
    /// Query to search on unsplash
    #[structopt(short, long, default_value = "abstract")]
    pub query: String,
}

pub struct BSObj {
    pub creator: generators::full_names::Name,
    pub origin_beginning: generators::origin_story::beginning::Beginning,
    pub origin_middle: generators::origin_story::middle::Middle,
    pub language: generators::languages::Language,
    pub name: generators::product_names::Name,
    pub logo: (tiny_skia::Pixmap, Vec<(u8, u8, u8)>),
    pub motto: generators::catch_phrases::CatchPhrase,
    pub features: generators::features::Features,
}

impl Display for BSObj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"{} {} thought of {} after {}. Then they thought, "{}", then {} was born! {} was programmed in {}"#,
            self.creator.first,
            self.creator.last,
            self.name,
            self.origin_beginning,
            self.origin_middle,
            self.name,
            self.name,
            self.language
        )
    }
}

#[derive(Debug)]
pub enum Format {
    Groff,
    None,
    HtmlMin,
    Html,
    Custon(PathBuf),
}

impl FromStr for Format {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "groff" | "pdf" => Ok(Self::Groff),
            "none" => Ok(Self::None),
            "html_min" => Ok(Self::HtmlMin),
            "html" => Ok(Self::Html),
            s => Ok(Self::Custon(s.into())),
        }
    }
}
