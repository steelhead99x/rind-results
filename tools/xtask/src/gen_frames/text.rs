// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
//! Text as Pillow draws it with `ImageDraw.fontmode = "1"` and its RAQM
//! layout (`_imagingft.c`): HarfBuzz shaping for glyphs and positions,
//! FreeType's TrueType bytecode hinting for `FT_LOAD_TARGET_MONO` and its
//! monochrome rasterizer. Shaping is HarfRust, hinting is skrifa and the
//! rasterizer is [`super::raster`], so no FreeType is involved.

use skrifa::instance::{LocationRef, Size};
use skrifa::outline::{
    DrawSettings, Engine, HintingInstance, HintingOptions, OutlineGlyphCollection, OutlinePen,
    Target, pen::PathStyle,
};
use skrifa::raw::TableProvider;
use skrifa::{FontRef, GlyphId, MetadataProvider};

use super::Gray;
use super::raster::{self, Outline};

/// Round a 26.6 value to whole pixels (`PIXEL` in Pillow).
fn pixel(x: i64) -> i64 {
    ((x + 32) & -64) >> 6
}

fn mul_fix(a: i64, b: i64) -> i64 {
    let s = (a < 0) != (b < 0);
    let v = (a.abs() * b.abs() + 0x8000) >> 16;
    if s { -v } else { v }
}

fn div_fix(a: i64, b: i64) -> i64 {
    let s = (a < 0) != (b < 0);
    let v = ((a.abs() << 16) + b.abs() / 2) / b.abs();
    if s { -v } else { v }
}

struct Glyph {
    id: u32,
    x_offset: i64,
    y_offset: i64,
    x_advance: i64,
    y_advance: i64,
}

/// One TrueType font at one pixel size, like `ImageFont.truetype(path, size)`.
pub struct Font<'a> {
    shaper: harfrust::Font,
    outlines: OutlineGlyphCollection<'a>,
    hinting: HintingInstance,
    /// The HarfBuzz font scale hb-ft sets: one em in 26.6.
    hb_scale: i32,
    /// `face->size->metrics.ascender`, 26.6.
    ascender: i64,
    ppem: i64,
}

impl<'a> Font<'a> {
    pub fn new(data: &'a [u8], size: u32) -> Result<Self, String> {
        let font = FontRef::new(data).map_err(|e| format!("font: {e}"))?;
        let shaper = harfrust::Font::new(data.to_vec(), 0).ok_or("font: unreadable for shaping")?;
        let outlines = font.outline_glyphs();
        let hinting = HintingInstance::new(
            &outlines,
            Size::new(size as f32),
            LocationRef::default(),
            HintingOptions {
                engine: Engine::Interpreter,
                target: Target::Mono,
            },
        )
        .map_err(|e| format!("font hinting: {e}"))?;
        let upem = i64::from(
            font.head()
                .map_err(|e| format!("font: {e}"))?
                .units_per_em(),
        );
        let ppem = i64::from(size);
        // FT_Set_Char_Size(size * 64) at 72 dpi; HarfBuzz's hb-ft takes the
        // same x_scale (16.16 per font unit) and multiplies by upem.
        let y_scale = div_fix(ppem * 64, upem);
        let hb_scale = ((y_scale * upem + (1 << 15)) >> 16) as i32;
        let hhea = font.hhea().map_err(|e| format!("font: {e}"))?;
        let asc = mul_fix(i64::from(hhea.ascender().to_i16()), y_scale);
        let ascender = (asc + 63) & -64;
        Ok(Self {
            shaper,
            outlines,
            hinting,
            hb_scale,
            ascender,
            ppem,
        })
    }

    /// raqm's layout: HarfBuzz shaping at hb-ft's scale, so advances
    /// and GPOS kerning come out in 26.6 with HarfBuzz's rounding.
    fn layout(&self, text: &str) -> Result<Vec<Glyph>, String> {
        let font = harfrust::ShaperFont::new(&self.shaper).with_scale(self.hb_scale);
        let mut buffer = harfrust::Buffer::new();
        buffer.push_str(text);
        buffer.guess_segment_properties();
        harfrust::shape(&font, &mut buffer, harfrust::ShapeOptions::new())
            .map_err(|e| format!("shaping {text:?}: {e}"))?;
        Ok(buffer
            .glyph_infos()
            .iter()
            .zip(buffer.glyph_positions())
            .map(|(info, pos)| Glyph {
                id: info.glyph_id,
                x_offset: i64::from(pos.x_offset),
                y_offset: i64::from(pos.y_offset),
                x_advance: i64::from(pos.x_advance),
                y_advance: i64::from(pos.y_advance),
            })
            .collect())
    }

    /// `ImageDraw.textlength`: the summed advances, in pixels.
    pub fn text_length(&self, text: &str) -> Result<f64, String> {
        let sum: i64 = self.layout(text)?.iter().map(|g| g.x_advance).sum();
        Ok(sum as f64 / 64.0)
    }

    /// The hinted outline of glyph `id`, in 26.6 with the origin at (0, 0).
    pub fn outline(&self, id: u32) -> Result<Outline, String> {
        let glyph = self
            .outlines
            .get(GlyphId::new(id))
            .ok_or_else(|| format!("font: no glyph {id}"))?;
        let mut pen = Points::default();
        glyph
            .draw(
                DrawSettings::hinted(&self.hinting, false).with_path_style(PathStyle::FreeType),
                &mut pen,
            )
            .map_err(|e| format!("font: glyph {id}: {e}"))?;
        pen.end_contour();
        Ok(pen.finish())
    }

    fn mode(&self) -> raster::Mode {
        raster::Mode {
            high_precision: self.ppem < 24,
            // DejaVu's `prep` turns on SCANCTRL with SCANTYPE 1 (simple
            // drop-out control, stubs excluded) at these sizes, which the
            // loader maps to no extra outline flags: Render_Glyph's 1.
            drop_out_control: 1,
        }
    }

    /// `ImageDraw.text((x, y), text, fill=255, font=self)` in mode "1"
    /// with the default "la" anchor.
    pub fn draw(&self, img: &mut Gray, x: f64, y: f64, text: &str) -> Result<(), String> {
        let glyphs = self.layout(text)?;
        if glyphs.is_empty() {
            return Ok(());
        }
        let x_start = x.fract();
        let y_start = y.fract();
        let mode = self.mode();
        let mut rendered = Vec::with_capacity(glyphs.len());
        for g in &glyphs {
            let outline = self.outline(g.id)?;
            let cbox = raster::cbox(&outline);
            let bitmap = raster::render_mono(&outline, mode);
            rendered.push((cbox, bitmap));
        }

        // bounding_box_and_anchors, from the grid-fitted control boxes.
        let (mut position, mut x_min, mut x_max, mut y_min, mut y_max) =
            (0i64, 0i64, 0i64, 0i64, 0i64);
        for (g, ((cx0, cy0, cx1, cy1), _)) in glyphs.iter().zip(&rendered) {
            let px = pixel(position + g.x_offset);
            let py = pixel(g.y_offset);
            position += g.x_advance;
            x_max = x_max.max(pixel(position));
            let (bx0, by0) = ((cx0 & -64) >> 6, (cy0 & -64) >> 6);
            let (bx1, by1) = (((cx1 + 63) & -64) >> 6, ((cy1 + 63) & -64) >> 6);
            x_max = x_max.max(bx1 + px);
            x_min = x_min.min(bx0 + px);
            y_max = y_max.max(by1 + py);
            y_min = y_min.min(by0 + py);
        }
        let y_anchor = pixel(self.ascender);
        let width = (x_max - x_min) as f64 + x_start.ceil();
        let height = (y_max - y_min) as f64 + y_start.ceil();
        let x_offset = x_min;
        let y_offset = y_anchor - y_max;
        let mut mask = Gray::new(width as usize, height as usize);

        // font_render: the pen starts at the rendered bitmaps' extent.
        let (mut pen_x, mut pen_y, mut bx_min, mut by_max) = (0i64, 0i64, 0i64, 0i64);
        for (g, (_, (_, left, top))) in glyphs.iter().zip(&rendered) {
            let px = pixel(pen_x + g.x_offset);
            let py = pixel(pen_y + g.y_offset);
            by_max = by_max.max(i64::from(*top) + py);
            bx_min = bx_min.min(i64::from(*left) + px);
            pen_x += g.x_advance;
            pen_y += g.y_advance;
        }
        let mut pen_x = ((-bx_min as f64 + x_start) * 64.0).round() as i64;
        let mut pen_y = ((-by_max as f64 - y_start) * 64.0).round() as i64;
        for (g, (_, (bitmap, left, top))) in glyphs.iter().zip(&rendered) {
            let px = pixel(pen_x + g.x_offset);
            let py = pixel(pen_y + g.y_offset);
            let xx = px + i64::from(*left);
            let yy = -(py + i64::from(*top));
            for row in 0..bitmap.rows {
                let ty = yy + i64::from(row);
                if ty < 0 || ty >= mask.h as i64 {
                    continue;
                }
                for col in 0..bitmap.width {
                    let tx = xx + i64::from(col);
                    if tx < 0 || tx >= mask.w as i64 {
                        continue;
                    }
                    if bitmap.get(col, row) {
                        mask.set(tx as usize, ty as usize, 255);
                    }
                }
            }
            pen_x += g.x_advance;
            pen_y += g.y_advance;
        }

        // ImageDraw.draw_bitmap at (int(x), int(y)) + offset.
        let ox = x.trunc() as i64 + x_offset;
        let oy = y.trunc() as i64 + y_offset;
        for my in 0..mask.h {
            for mx in 0..mask.w {
                if mask.get(mx, my) == 0 {
                    continue;
                }
                let (tx, ty) = (ox + mx as i64, oy + my as i64);
                if tx >= 0 && ty >= 0 && (tx as usize) < img.w && (ty as usize) < img.h {
                    img.set(tx as usize, ty as usize, 255);
                }
            }
        }
        Ok(())
    }
}

/// Collects a FreeType-style path back into TrueType points: implied
/// on-curve midpoints between two conic controls are dropped again, so the
/// rasterizer sees the contour FreeType's does.
#[derive(Default)]
struct Points {
    contours: Vec<Vec<((i64, i64), bool)>>,
    cur: Vec<((i64, i64), bool)>,
}

fn fx(v: f32) -> i64 {
    (f64::from(v) * 64.0).round() as i64
}

impl Points {
    fn end_contour(&mut self) {
        if self.cur.is_empty() {
            return;
        }
        let mut c = std::mem::take(&mut self.cur);
        // The closing segment ends on the start point.
        if c.len() > 1 && c.last().map(|p| p.0) == Some(c[0].0) && c.last().is_some_and(|p| p.1) {
            c.pop();
        }
        // Drop on-curve points that are the (truncated, 26.6) midpoint of
        // the conic controls on either side.
        let n = c.len();
        let mut keep = vec![true; n];
        if n >= 3 {
            for i in 0..n {
                let (p, on) = c[i];
                let (a, a_on) = c[(i + n - 1) % n];
                let (b, b_on) = c[(i + 1) % n];
                if on && !a_on && !b_on && p == ((a.0 + b.0) / 2, (a.1 + b.1) / 2) {
                    keep[i] = false;
                }
            }
        }
        let c: Vec<_> = c
            .into_iter()
            .zip(keep)
            .filter(|(_, k)| *k)
            .map(|(p, _)| p)
            .collect();
        self.contours.push(c);
    }

    fn finish(self) -> Outline {
        let mut o = Outline {
            points: Vec::new(),
            on: Vec::new(),
            contours: Vec::new(),
        };
        for c in self.contours {
            if c.is_empty() {
                continue;
            }
            for (p, on) in c {
                o.points.push(p);
                o.on.push(on);
            }
            o.contours.push(o.points.len() - 1);
        }
        o
    }
}

impl OutlinePen for Points {
    fn move_to(&mut self, x: f32, y: f32) {
        self.end_contour();
        self.cur.push(((fx(x), fx(y)), true));
    }
    fn line_to(&mut self, x: f32, y: f32) {
        self.cur.push(((fx(x), fx(y)), true));
    }
    fn quad_to(&mut self, cx0: f32, cy0: f32, x: f32, y: f32) {
        self.cur.push(((fx(cx0), fx(cy0)), false));
        self.cur.push(((fx(x), fx(y)), true));
    }
    fn curve_to(&mut self, _: f32, _: f32, _: f32, _: f32, x: f32, y: f32) {
        // TrueType outlines have no cubics.
        self.cur.push(((fx(x), fx(y)), true));
    }
    fn close(&mut self) {
        self.end_contour();
    }
}
