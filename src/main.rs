use std::{
    io::{BufWriter, Write},
    process::Stdio,
};

use anyhow::{bail, Context};
use bullshit_plus_plus::{BSFormatter, BSObj, Format};
use const_colors::{bold, color256, end, green, yellow};
use horrorshow::{helper::doctype, Template};
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;

use structopt::StructOpt;
use tiny_skia::Pixmap;
use unchecked_unwrap::UncheckedUnwrap;

mod generators;
pub use bullshit_plus_plus::OPTS;

macro_rules! conf_prompt {
    ($a:expr,$b:expr,$c:expr) => {
        $c.write_fmt(format_args!(
            concat!(
                green!(),
                "✔",
                end!(),
                " ",
                bold!(),
                $a,
                end!(),
                " ",
                color256!("8"),
                "·",
                end!(),
                " ",
                green!(),
                "{}",
                end!(),
                "\n"
            ),
            $b
        ))?;
    };
}

macro_rules! warning_prompt {
    ($a:expr,$b:expr,$c:expr) => {
        $c.write_fmt(format_args!(
            concat!(
                yellow!(),
                "⚠",
                end!(),
                " ",
                bold!(),
                yellow!(),
                $a,
                end!(),
                " ",
                color256!("8"),
                "·",
                end!(),
                " ",
                yellow!(),
                "{}",
                end!(),
                "\n"
            ),
            $b
        ))?;
    };
}

macro_rules! auto_rng {
    ($a:literal,$term:expr,$rng:expr) => {{
        let val = $rng.gen();
        conf_prompt!($a, val, $term);
        val
    }};
}

fn main() -> anyhow::Result<()> {
    let opts = unsafe {
        bullshit_plus_plus::OPTS
            .try_insert(bullshit_plus_plus::Opts::from_args())
            .unchecked_unwrap()
    };
    let termunlock = std::io::stdout();
    let mut term = BufWriter::new(termunlock.lock());
    let mut rng;
    if let Some(ref seed) = opts.seed {
        let rand: [u8; 32] = base91::slice_decode(seed.as_bytes())
            .try_into()
            .map_err(|_| anyhow::anyhow!("The seed is not 32 bytes long (invalid seed)"))?;
        rng = Xoshiro256PlusPlus::from_seed(rand);
        conf_prompt!("Seed", seed, term);
    } else {
        let mut rand: [u8; 32] = [0; 32];
        getrandom::getrandom(&mut rand)?;
        rng = Xoshiro256PlusPlus::from_seed(rand);
        conf_prompt!(
            "Seed",
            &unsafe { String::from_utf8_unchecked(base91::slice_encode(&rand)) },
            term
        );
    }
    let bsobj = BSObj {
        creator: auto_rng!("Creator", term, rng),
        origin_beginning: auto_rng!("Origin Story (beginning)", term, rng),
        origin_middle: auto_rng!("Origin Story (middle)", term, rng),
        language: auto_rng!("Programming language(s)", term, rng),
        name: auto_rng!("Product Name", term, rng),
        motto: auto_rng!("Catch Phrase", term, rng),
        features: auto_rng!("Features", term, rng),
        logo: {
            if let Some(ref logopts) = opts.logo {
                if logopts.is_file() {
                    conf_prompt!("Logo from file", logopts.display(), term);
                    (
                        Pixmap::decode_png(unsafe {
                            &memmap::Mmap::map(
                                &std::fs::File::open(logopts)
                                    .context("Error opening logo from file")?,
                            )
                            .context("Error mapping logo from file")?
                        })
                        .context("Error decoding logo from file")?,
                        Vec::new(),
                    )
                } else if logopts.is_dir() {
                    let logo: generators::logo::Logo = rng.gen();
                    conf_prompt!("Logo from directory", logo, term);
                    logo.draw()?
                } else {
                    bail!("Couldn't open logo, no such file or directory");
                }
            } else {
                if opts.seed.is_some() {
                    warning_prompt!(
                        "Seed will not be used in logo",
                        "The randomness of Unsplash cannot be controlled",
                        term
                    );
                }
                warning_prompt!("Loading Photos", "Loading photos from Unsplash API", term);
                term.flush()?;
                let logo: generators::logo::Logo = rng.gen();
                conf_prompt!("Logo from Unsplash", logo, term);
                logo.draw()?
            }
        },
    };

    match opts.format {
        Format::None => {}
        Format::Groff => {
            let cmd = std::process::Command::new("groff")
                .arg("-ms")
                .arg("-Tpdf")
                .stdout(
                    std::fs::File::create("bullshit.pdf")
                        .context("Could not create bullshit.pdf")?,
                )
                .stdin(Stdio::piped())
                .spawn()
                .context("Could not execute groff -ms -Tpdf")?;
            let mut stdin = cmd.stdin.context("Could not get groff stdin")?;
            stdin.write_all(b".TL\n")?;
            stdin.write_fmt(format_args!("{}", bsobj.name))?;
            stdin.write_all(b"\n.AU\n")?;
            stdin.write_fmt(format_args!("{}", bsobj.creator))?;
            stdin.write_all(b"\n.AI\n")?;
            stdin.write_all(bsobj.motto.to_string().as_bytes())?;
            stdin.write_all(b"\n.PP\n")?;
            stdin.write_fmt(format_args!("{}", bsobj))?;
        }
        Format::Html => {
            if !std::path::Path::new("dist").exists() {
                std::fs::create_dir("dist").context("Could not create dist directory")?;
            }
            let colormin = unsafe {
                bsobj
                    .logo
                    .1
                    .iter()
                    .min_by(|f, b| {
                        (f.0 as u32 + f.1 as u32 + f.2 as u32)
                            .cmp(&(b.0 as u32 + b.1 as u32 + b.2 as u32))
                    })
                    .unchecked_unwrap()
            };

            let colormax = unsafe {
                bsobj
                    .logo
                    .1
                    .iter()
                    .max_by(|f, b| {
                        (f.0 as u32 + f.1 as u32 + f.2 as u32)
                            .cmp(&(b.0 as u32 + b.1 as u32 + b.2 as u32))
                    })
                    .unchecked_unwrap()
            };
            let darktheme = colormax.0 as u32 + colormax.1 as u32 + colormax.2 as u32 > 383;
            (horrorshow::html! {
                : doctype::HTML;
                html {
                    head {
                        meta(charset="utf-8");
                        meta(name="viewport", content="width=device-width, initial-scale=1");
                        style: format_args!(r#".center{{margin-left:auto;margin-right:auto;padding-top:10px;display:block}}.rect{{height:50%;width:100%;background:linear-gradient(145deg,rgb({},{},{}) 0%,rgb({},{},{}) 100%);position:relative}}.recomplete{{background-color:rgb({},{},{});position:relative}}html,body{{height:100%;margin:0;background-color:rgb({},{},{})}}h1{{color:{};font-family:'Helvetica Neue',sans-serif;font-size:5em;font-weight:700;letter-spacing:-1px;line-height:1;text-align:center}}h2{{color:{};font-family:'Open Sans',sans-serif;font-size:30px;font-weight:300;line-height:32px;margin:0 0 72px;text-align:center}}p, li, a{{color:{};font-family:'Helvetica Neue',sans-serif;font-size:14px;line-height:24px;margin:0 0 24px;text-align:justify;text-justify:inter-word}}"#,
                            bsobj.logo.1[0].0,
                            bsobj.logo.1[0].1,
                            bsobj.logo.1[0].2,
                            bsobj.logo.1[1].0,
                            bsobj.logo.1[1].1,
                            bsobj.logo.1[1].2,
                            colormin.0,
                            colormin.1,
                            colormin.2,
                            colormax.0,
                            colormax.1,
                            colormax.2,
                            if darktheme { "#111" } else { "#FFF" },
                            if darktheme { "#111" } else { "#FFF" },
                            if darktheme { "#111" } else { "#FFF" }
                        );
                    }
                    body {
                        div(style="height:420px") {
                            div(class="rect") {
                                img(src="logo.png", class="center");
                            }
                        }
                        h1: format_args!("{}", bsobj.name);
                        h2: format_args!("{}", bsobj.motto);
                        div(class="recomplete") {
                            div(style="padding:10px") {
                                h2(style="color:white"): "Origin";
                                p(style="color:white"): format_args!("{}", bsobj);
                            }
                        }
                        div {
                            div(style="padding:10px") {
                                h2: "Features";
                                ul {
                                    li: format_args!("{}", bsobj.features.feature_1);
                                    li: format_args!("{}", bsobj.features.feature_2);
                                    li: format_args!("{}", bsobj.features.feature_3);
                                    li: format_args!("{}", bsobj.features.feature_4);
                                }
                            }
                        }
                        div(class="recomplete") {
                            div(style="padding:10px") {
                                a(style="color:white", href="https://github.com/StratusFearMe21/bullshit_plus_plus"): "Generated by Bullshit++";
                            }
                        }
                    }
                }
            })
            .write_to_io(&mut BufWriter::new(
                    std::fs::File::create("dist/index.html").context("Could not create index.html")?,
                    ))
                .context("Could not write to index.html")?;
            std::fs::write("dist/logo.png", unsafe {
                bsobj.logo.0.encode_png().unchecked_unwrap()
            })
            .context("Could not write to logo.png")?;
        }
        Format::Custon(ref path) => unsafe {
            let lib = libloading::Library::new(path)?;
            let mut plug = Box::from_raw((lib.get::<fn() -> *mut dyn BSFormatter>(b"init\0")?)());
            plug.format(bsobj, &mut rng);
        },
        _ => unsafe { core::hint::unreachable_unchecked() },
    }
    term.flush()?;

    Ok(())
}
