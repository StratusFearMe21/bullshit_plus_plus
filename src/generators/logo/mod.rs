use std::fmt::Display;

use anyhow::Context;
use rand_derive2::RandGen;
use tiny_skia::{
    ClipMask, FillRule, Paint, PathBuilder, Pixmap, PixmapPaint, Rect, Stroke, Transform,
};
use unchecked_unwrap::UncheckedUnwrap;

use self::{divide::Divide, frame::Frame};

mod divide;
mod frame;
mod unsplash;

macro_rules! draw_2_photos {
    ($photos:expr,$mask:expr,$pixmap:expr,$path1:expr,$path2:expr) => {
        let mut firstmask = $mask.clone();
        let mut path = PathBuilder::default();
        $path1(&mut path);
        firstmask
            .intersect_path(
                &path.finish().context("First ClipMask path was invalid")?,
                FillRule::EvenOdd,
                true,
                )
            .context("Could not intersect first clip path")?;
        let mut secondmask = $mask.clone();
        let mut path = PathBuilder::default();
        $path2(&mut path);
        secondmask
            .intersect_path(
                &path.finish().context("Second ClipMask path was invalid")?,
                FillRule::EvenOdd,
                true,
                )
            .context("Could not intersect second clip path")?;
        $pixmap
            .draw_pixmap(
                $photos[0].1,
                $photos[0].2,
                $photos[0].0.as_ref(),
                &PixmapPaint::default(),
                Transform::default(),
                Some(&firstmask),
                )
            .with_context(|| {
                $photos[0].0.save_png("error.png").unwrap();
                format!(
                    "Pixmap with X {} and Y {} is invalid for {}x{} pixmap. Offender output to error.png",
                    $photos[0].1,
                    $photos[0].2,
                    $photos[0].0.width(),
                    $photos[0].0.height()
                    )
            })?;
        $pixmap
            .draw_pixmap(
                $photos[1].1,
                $photos[1].2,
                $photos[1].0.as_ref(),
                &PixmapPaint::default(),
                Transform::default(),
                Some(&secondmask),
                )
            .with_context(|| {
                $photos[1].0.save_png("error.png").unwrap();
                format!(
                    "Pixmap with X {} and Y {} is invalid for {}x{} pixmap. Offender output to error.png",
                    $photos[1].1,
                    $photos[1].2,
                    $photos[1].0.width(),
                    $photos[1].0.height()
                    )
            })?;
    };
}

#[derive(RandGen)]
pub struct Logo {
    frame: Frame,
    divide: Divide,
}

impl Display for Logo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.frame, self.divide)
    }
}

type ColorVec = Vec<(u8, u8, u8)>;

impl Logo {
    pub fn draw(&self) -> anyhow::Result<(Pixmap, ColorVec)> {
        let mut maskpath = PathBuilder::new();
        let mut pixmap = unsafe { Pixmap::new(400, 400).unchecked_unwrap() };
        let brush = Paint {
            anti_alias: true,
            shader: tiny_skia::Shader::SolidColor(tiny_skia::Color::BLACK),
            ..Default::default()
        };
        match self.frame {
            Frame::Circle => {
                maskpath.push_circle(
                    200.0,
                    200.0,
                    200.0 - unsafe { crate::OPTS.get_unchecked() }.border_width,
                );
            }
            Frame::Square => maskpath.push_rect(
                0.0 + unsafe { crate::OPTS.get_unchecked() }.border_width,
                0.0 + unsafe { crate::OPTS.get_unchecked() }.border_width,
                400.0 - unsafe { crate::OPTS.get_unchecked() }.border_width * 2.0,
                400.0 - unsafe { crate::OPTS.get_unchecked() }.border_width * 2.0,
            ),
            Frame::Diamond => {
                maskpath.move_to(
                    0.0 + unsafe { crate::OPTS.get_unchecked() }.border_width,
                    200.0,
                );
                maskpath.line_to(
                    200.0,
                    0.0 + unsafe { crate::OPTS.get_unchecked() }.border_width,
                );
                maskpath.line_to(
                    400.0 - unsafe { crate::OPTS.get_unchecked() }.border_width,
                    200.0,
                );
                maskpath.line_to(
                    200.0,
                    400.0 - unsafe { crate::OPTS.get_unchecked() }.border_width,
                );
                maskpath.close();
            }
            Frame::Triangle => {
                maskpath.move_to(
                    0.0 + unsafe { crate::OPTS.get_unchecked() }.border_width,
                    400.0 - unsafe { crate::OPTS.get_unchecked() }.border_width,
                );
                maskpath.line_to(
                    200.0,
                    0.0 + unsafe { crate::OPTS.get_unchecked() }.border_width,
                );
                maskpath.line_to(
                    400.0 - unsafe { crate::OPTS.get_unchecked() }.border_width,
                    400.0 - unsafe { crate::OPTS.get_unchecked() }.border_width,
                );
                maskpath.close();
            }
            Frame::RoundedTriangle => {
                maskpath.move_to(
                    30.0 + unsafe { crate::OPTS.get_unchecked() }.border_width,
                    400.0 - unsafe { crate::OPTS.get_unchecked() }.border_width,
                );
                maskpath.quad_to(
                    0.0 + unsafe { crate::OPTS.get_unchecked() }.border_width,
                    400.0 - unsafe { crate::OPTS.get_unchecked() }.border_width,
                    0.0 + unsafe { crate::OPTS.get_unchecked() }.border_width,
                    370.0 - unsafe { crate::OPTS.get_unchecked() }.border_width,
                );
                maskpath.line_to(
                    170.0 + unsafe { crate::OPTS.get_unchecked() }.border_width,
                    30.0 + unsafe { crate::OPTS.get_unchecked() }.border_width,
                );
                maskpath.quad_to(
                    200.0,
                    0.0 - unsafe { crate::OPTS.get_unchecked() }.border_width,
                    230.0 - unsafe { crate::OPTS.get_unchecked() }.border_width,
                    30.0 + unsafe { crate::OPTS.get_unchecked() }.border_width,
                );
                maskpath.line_to(
                    400.0 - unsafe { crate::OPTS.get_unchecked() }.border_width,
                    370.0 - unsafe { crate::OPTS.get_unchecked() }.border_width,
                );
                maskpath.quad_to(
                    400.0 - unsafe { crate::OPTS.get_unchecked() }.border_width,
                    400.0 - unsafe { crate::OPTS.get_unchecked() }.border_width,
                    370.0 - unsafe { crate::OPTS.get_unchecked() }.border_width,
                    400.0 - unsafe { crate::OPTS.get_unchecked() }.border_width,
                );
                maskpath.close();
            }
            Frame::Squircle => {
                maskpath.move_to(
                    30.0 + unsafe { crate::OPTS.get_unchecked() }.border_width,
                    30.0 + unsafe { crate::OPTS.get_unchecked() }.border_width,
                );
                maskpath.quad_to(
                    200.0,
                    0.0,
                    370.0,
                    30.0 + unsafe { crate::OPTS.get_unchecked() }.border_width,
                );
                maskpath.quad_to(400.0, 200.0, 370.0, 370.0);
                maskpath.quad_to(
                    200.0,
                    400.0,
                    30.0 + unsafe { crate::OPTS.get_unchecked() }.border_width,
                    370.0,
                );
                maskpath.quad_to(
                    0.0,
                    200.0,
                    30.0 + unsafe { crate::OPTS.get_unchecked() }.border_width,
                    30.0 + unsafe { crate::OPTS.get_unchecked() }.border_width,
                );
                maskpath.close();
            }
            Frame::DiamondSquircle => {
                maskpath.move_to(
                    0.0 + unsafe { crate::OPTS.get_unchecked() }.border_width,
                    200.0,
                );
                maskpath.quad_to(
                    70.0,
                    70.0,
                    200.0,
                    0.0 + unsafe { crate::OPTS.get_unchecked() }.border_width,
                );
                maskpath.quad_to(
                    330.0,
                    70.0,
                    400.0 - unsafe { crate::OPTS.get_unchecked() }.border_width,
                    200.0,
                );
                maskpath.quad_to(
                    330.0,
                    330.0,
                    200.0,
                    400.0 - unsafe { crate::OPTS.get_unchecked() }.border_width,
                );
                maskpath.quad_to(
                    70.0,
                    330.0,
                    0.0 + unsafe { crate::OPTS.get_unchecked() }.border_width,
                    200.0,
                );
                maskpath.close();
            }
            Frame::RoundedRectangle => {
                maskpath.move_to(
                    0.0 + unsafe { crate::OPTS.get_unchecked() }.border_width,
                    30.0 + unsafe { crate::OPTS.get_unchecked() }.border_width,
                );
                maskpath.quad_to(
                    0.0 + unsafe { crate::OPTS.get_unchecked() }.border_width,
                    0.0 + unsafe { crate::OPTS.get_unchecked() }.border_width,
                    30.0 + unsafe { crate::OPTS.get_unchecked() }.border_width,
                    0.0 + unsafe { crate::OPTS.get_unchecked() }.border_width,
                );
                maskpath.line_to(
                    370.0 - unsafe { crate::OPTS.get_unchecked() }.border_width,
                    0.0 + unsafe { crate::OPTS.get_unchecked() }.border_width,
                );
                maskpath.quad_to(
                    400.0 - unsafe { crate::OPTS.get_unchecked() }.border_width,
                    0.0 + unsafe { crate::OPTS.get_unchecked() }.border_width,
                    400.0 - unsafe { crate::OPTS.get_unchecked() }.border_width,
                    30.0 + unsafe { crate::OPTS.get_unchecked() }.border_width,
                );
                maskpath.line_to(
                    400.0 - unsafe { crate::OPTS.get_unchecked() }.border_width,
                    370.0 - unsafe { crate::OPTS.get_unchecked() }.border_width,
                );
                maskpath.quad_to(
                    400.0 - unsafe { crate::OPTS.get_unchecked() }.border_width,
                    400.0 - unsafe { crate::OPTS.get_unchecked() }.border_width,
                    370.0 - unsafe { crate::OPTS.get_unchecked() }.border_width,
                    400.0 - unsafe { crate::OPTS.get_unchecked() }.border_width,
                );
                maskpath.line_to(
                    30.0 + unsafe { crate::OPTS.get_unchecked() }.border_width,
                    400.0 - unsafe { crate::OPTS.get_unchecked() }.border_width,
                );
                maskpath.quad_to(
                    0.0 + unsafe { crate::OPTS.get_unchecked() }.border_width,
                    400.0 - unsafe { crate::OPTS.get_unchecked() }.border_width,
                    0.0 + unsafe { crate::OPTS.get_unchecked() }.border_width,
                    370.0 - unsafe { crate::OPTS.get_unchecked() }.border_width,
                );
                maskpath.close();
            }
        }
        let mut mask = ClipMask::new();
        let maskpath = unsafe {
            maskpath
                .finish()
                .unchecked_expect("Clip mask path is invalid")
        };
        mask.set_path(400, 400, &maskpath, FillRule::EvenOdd, true);
        unsafe {
            pixmap.fill_rect(
                Rect::from_ltrb(0.0, 0.0, 400.0, 400.0).unchecked_unwrap(),
                &brush,
                Transform::default(),
                Some(&mask),
            )
        };
        match &self.divide {
            Divide::VertHalf(photos) => {
                draw_2_photos!(
                    photos,
                    mask,
                    pixmap,
                    |path: &mut tiny_skia::PathBuilder| path.push_rect(
                        0.0,
                        0.0,
                        200.0 - unsafe { crate::OPTS.get_unchecked() }.border_width,
                        400.0
                    ),
                    |path: &mut tiny_skia::PathBuilder| path.push_rect(
                        200.0 + unsafe { crate::OPTS.get_unchecked() }.border_width,
                        0.0,
                        400.0,
                        400.0
                    )
                );
            }
            Divide::HorHalf(photos) => {
                draw_2_photos!(
                    photos,
                    mask,
                    pixmap,
                    |path: &mut tiny_skia::PathBuilder| path.push_rect(
                        0.0,
                        0.0,
                        400.0,
                        200.0 - unsafe { crate::OPTS.get_unchecked() }.border_width
                    ),
                    |path: &mut tiny_skia::PathBuilder| path.push_rect(
                        0.0,
                        200.0 + unsafe { crate::OPTS.get_unchecked() }.border_width,
                        400.0,
                        400.0
                    )
                );
            }
            Divide::DiagRToL(photos) => {
                draw_2_photos!(
                    photos,
                    mask,
                    pixmap,
                    |path: &mut tiny_skia::PathBuilder| {
                        path.move_to(0.0, 0.0);
                        path.line_to(
                            400.0 - (unsafe { crate::OPTS.get_unchecked() }.border_width * 1.5),
                            0.0,
                        );
                        path.line_to(
                            0.0,
                            400.0 - (unsafe { crate::OPTS.get_unchecked() }.border_width * 1.5),
                        );
                        path.close();
                    },
                    |path: &mut tiny_skia::PathBuilder| {
                        path.move_to(400.0, 400.0);
                        path.line_to(
                            0.0 + (unsafe { crate::OPTS.get_unchecked() }.border_width * 1.5),
                            400.0,
                        );
                        path.line_to(
                            400.0,
                            0.0 + (unsafe { crate::OPTS.get_unchecked() }.border_width * 1.5),
                        );
                        path.close();
                    }
                );
            }
            Divide::DiagLToR(photos) => {
                draw_2_photos!(
                    photos,
                    mask,
                    pixmap,
                    |path: &mut tiny_skia::PathBuilder| {
                        path.move_to(0.0, 400.0);
                        path.line_to(
                            400.0 - (unsafe { crate::OPTS.get_unchecked() }.border_width * 1.5),
                            400.0,
                        );
                        path.line_to(
                            0.0,
                            0.0 + (unsafe { crate::OPTS.get_unchecked() }.border_width * 1.5),
                        );
                        path.close();
                    },
                    |path: &mut tiny_skia::PathBuilder| {
                        path.move_to(400.0, 0.0);
                        path.line_to(
                            400.0,
                            400.0 - (unsafe { crate::OPTS.get_unchecked() }.border_width * 1.5),
                        );
                        path.line_to(
                            0.0 + (unsafe { crate::OPTS.get_unchecked() }.border_width * 1.5),
                            0.0,
                        );
                        path.close();
                    }
                );
            }
        }
        pixmap.stroke_path(
            &maskpath,
            &brush,
            &Stroke {
                width: unsafe { crate::OPTS.get_unchecked() }.border_width * 2.0,
                ..Default::default()
            },
            Transform::default(),
            None,
        );

        Ok((
            pixmap,
            self.divide
                .photos()
                .iter()
                .map(|f| (f.0.data(), (f.0.data().len() as f32 / 4.0)))
                .map(|f| {
                    (
                        f32::floor(f.0.iter().map(|f| *f as f32).step_by(4).sum::<f32>() / f.1)
                            as u8,
                        f32::floor(
                            f.0.iter()
                                .map(|f| *f as f32)
                                .skip(1)
                                .step_by(4)
                                .sum::<f32>()
                                / f.1,
                        ) as u8,
                        f32::floor(
                            f.0.iter()
                                .map(|f| *f as f32)
                                .skip(2)
                                .step_by(4)
                                .sum::<f32>()
                                / f.1,
                        ) as u8,
                    )
                })
                .collect(),
        ))
    }
}
