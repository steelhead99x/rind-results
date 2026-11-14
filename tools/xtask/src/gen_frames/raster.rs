// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
//! FreeType's monochrome scan converter (`src/raster/ftraster.c`, the
//! "black" rasterizer behind `FT_RENDER_MODE_MONO`), ported line by line so
//! that glyphs come out pixel for pixel as Pillow draws them with
//! `fontmode = "1"`. Only what TrueType outlines need is kept: lines and
//! conic arcs, the two sweeps, and drop-out control.

/// A glyph outline in 26.6 pixel coordinates, y up.
pub struct Outline {
    pub points: Vec<(i64, i64)>,
    /// `true` for on-curve points, `false` for conic control points.
    pub on: Vec<bool>,
    /// Index of the last point of each contour.
    pub contours: Vec<usize>,
}

/// A rendered glyph: `rows` lines of `width` pixels, top line first.
pub struct Bitmap {
    pub width: i32,
    pub rows: i32,
    pub bits: Vec<bool>,
}

impl Bitmap {
    pub fn get(&self, x: i32, y: i32) -> bool {
        self.bits[(y * self.width + x) as usize]
    }
}

const FLOW_UP: u16 = 0x08;
const OVERSHOOT_TOP: u16 = 0x10;
const OVERSHOOT_BOTTOM: u16 = 0x20;
const DROPOUT: u16 = 0x40;

#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    Unknown,
    Ascending,
    Descending,
}

#[derive(Clone, Copy, Default)]
struct Pt {
    x: i64,
    y: i64,
}

#[derive(Clone)]
struct Profile {
    next: usize,
    offset: i32,
    height: i32,
    start: i32,
    flags: u16,
    cur_x: i64,
    x: Vec<i64>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Sweep {
    Vertical,
    Horizontal,
}

struct Ras<'a> {
    precision_bits: u32,
    precision: i64,
    precision_half: i64,
    precision_scale: i64,
    precision_step: i64,
    drop_out_control: u16,
    last_x: i64,
    last_y: i64,
    min_y: i64,
    max_y: i64,
    state: State,
    /// Committed profiles, in creation order (`link` order in C).
    profiles: Vec<Profile>,
    /// The profile being filled (`cProfile`), not yet committed.
    cur: Option<Profile>,
    /// Index of the current contour's first committed profile (`gProfile`).
    g_profile: Option<usize>,
    /// Sorted profile bottoms (`y_turns`), then the highest top.
    turns: Vec<i64>,
    top_turn: Option<i64>,
    b_top: i32,
    b_right: i32,
    bitmap: &'a mut Bitmap,
    outline: &'a Outline,
}

impl Ras<'_> {
    fn floor(&self, x: i64) -> i64 {
        x & -self.precision
    }
    fn ceiling(&self, x: i64) -> i64 {
        (x + self.precision - 1) & -self.precision
    }
    fn trunc(&self, x: i64) -> i64 {
        x >> self.precision_bits
    }
    fn frac(&self, x: i64) -> i64 {
        x & (self.precision - 1)
    }
    fn scaled(&self, x: i64) -> i64 {
        x * self.precision_scale - self.precision_half
    }
    fn is_bottom_overshoot(&self, x: i64) -> bool {
        self.ceiling(x) - x >= self.precision_half
    }
    fn is_top_overshoot(&self, x: i64) -> bool {
        x - self.floor(x) >= self.precision_half
    }

    fn set_high_precision(&mut self, high: bool) {
        if high && self.b_top + self.b_right < 256 {
            self.precision_bits = 12;
            self.precision_step = 256;
        } else {
            self.precision_bits = 6;
            self.precision_step = 32;
        }
        self.precision = 1 << self.precision_bits;
        self.precision_half = self.precision >> 1;
        self.precision_scale = self.precision >> 6;
    }

    fn insert_y_turns(&mut self, y: i64, top: i64) {
        if self.top_turn.is_none_or(|t| top > t) {
            self.top_turn = Some(top);
        }
        if let Err(pos) = self.turns.binary_search(&y) {
            self.turns.insert(pos, y);
        }
    }

    fn push_x(&mut self, x: i64) {
        self.cur.as_mut().expect("profile").x.push(x);
    }

    fn new_profile(&mut self, state: State) {
        let mut p = match self.cur.take() {
            Some(p) if p.height == 0 => p,
            _ => Profile {
                next: 0,
                offset: 0,
                height: 0,
                start: 0,
                flags: 0,
                cur_x: 0,
                x: Vec::new(),
            },
        };
        p.x.clear();
        p.height = 0;
        p.flags = self.drop_out_control;
        let mut e = match state {
            State::Ascending => {
                p.flags |= FLOW_UP;
                if self.is_bottom_overshoot(self.last_y) {
                    p.flags |= OVERSHOOT_BOTTOM;
                }
                self.ceiling(self.last_y)
            }
            State::Descending => {
                if self.is_top_overshoot(self.last_y) {
                    p.flags |= OVERSHOOT_TOP;
                }
                self.floor(self.last_y)
            }
            State::Unknown => unreachable!("invalid profile direction"),
        };
        if e > self.max_y {
            e = self.max_y;
        }
        if e < self.min_y {
            e = self.min_y;
        }
        p.start = self.trunc(e) as i32;
        if self.last_y == e {
            p.x.push(self.last_x);
        }
        self.cur = Some(p);
        self.state = state;
    }

    fn end_profile(&mut self) {
        let last_y = self.last_y;
        let top_over = self.is_top_overshoot(last_y);
        let bottom_over = self.is_bottom_overshoot(last_y);
        let Some(mut p) = self.cur.take() else {
            return;
        };
        let h = p.x.len() as i32;
        if h > 0 {
            p.height = h;
            let (bottom, top);
            if p.flags & FLOW_UP != 0 {
                if top_over {
                    p.flags |= OVERSHOOT_TOP;
                }
                bottom = p.start;
                top = bottom + h;
                p.offset = 0;
                p.cur_x = p.x[0];
            } else {
                if bottom_over {
                    p.flags |= OVERSHOOT_BOTTOM;
                }
                top = p.start + 1;
                bottom = top - h;
                p.start = bottom;
                p.offset = h - 1;
                p.cur_x = p.x[(h - 1) as usize];
            }
            self.insert_y_turns(i64::from(bottom), i64::from(top));
            let g = *self.g_profile.get_or_insert(self.profiles.len());
            p.next = g;
            self.profiles.push(p);
        } else {
            // An empty profile is reused by the next `New_Profile`.
            self.cur = Some(p);
        }
    }

    fn line_up(&mut self, mut x1: i64, y1: i64, x2: i64, y2: i64, miny: i64, maxy: i64) {
        if y2 < miny || y1 > maxy {
            return;
        }
        let e2 = if y2 > maxy { maxy } else { self.floor(y2) };
        let mut e = if y1 < miny { miny } else { self.ceiling(y1) };
        if y1 == e {
            e += self.precision;
        }
        if e2 < e {
            return;
        }
        let mut size = self.trunc(e2 - e) + 1;
        let mut dx = x2 - x1;
        let dy = y2 - y1;
        if dx == 0 {
            while size > 0 {
                self.push_x(x1);
                size -= 1;
            }
            return;
        }
        let mut ix = (e - y1) * dx / dy;
        x1 += ix;
        self.push_x(x1);
        size -= 1;
        if size > 0 {
            let mut ax = dx * (e - y1) - dy * ix;
            ix = self.precision * dx / dy;
            let mut rx = dx * self.precision - dy * ix;
            dx = 1;
            if x2 < x1 {
                ax = -ax;
                rx = -rx;
                dx = -dx;
            }
            while size > 0 {
                x1 += ix;
                ax += rx;
                if ax >= dy {
                    ax -= dy;
                    x1 += dx;
                }
                self.push_x(x1);
                size -= 1;
            }
        }
    }

    fn line_down(&mut self, x1: i64, y1: i64, x2: i64, y2: i64, miny: i64, maxy: i64) {
        self.line_up(x1, -y1, x2, -y2, -maxy, -miny);
    }

    fn bezier_up(&mut self, arcs: &mut [Pt], base: usize, miny: i64, maxy: i64) {
        let mut a = base as isize;
        let y1 = arcs[base + 2].y;
        let y2 = arcs[base].y;
        if y2 < miny || y1 > maxy {
            return;
        }
        let e2 = if y2 > maxy { maxy } else { self.floor(y2) };
        let mut e = if y1 < miny { miny } else { self.ceiling(y1) };
        if y1 == e {
            e += self.precision;
        }
        if e2 < e {
            return;
        }
        loop {
            let i = a as usize;
            let y2 = arcs[i].y;
            let x2 = arcs[i].x;
            if y2 > e {
                let dy = y2 - arcs[i + 2].y;
                let dx = x2 - arcs[i + 2].x;
                if dy > self.precision_step || dx > self.precision_step || -dx > self.precision_step
                {
                    split_conic(arcs, i);
                    a += 2;
                } else {
                    self.push_x(x2 - (y2 - e) * dx / dy);
                    e += self.precision;
                    a -= 2;
                }
            } else {
                if y2 == e {
                    self.push_x(x2);
                    e += self.precision;
                }
                a -= 2;
            }
            if e > e2 || a < 0 {
                break;
            }
        }
    }

    fn bezier_down(&mut self, arcs: &mut [Pt], base: usize, miny: i64, maxy: i64) {
        for p in &mut arcs[base..base + 3] {
            p.y = -p.y;
        }
        self.bezier_up(arcs, base, -maxy, -miny);
        arcs[base].y = -arcs[base].y;
    }

    fn line_to(&mut self, x: i64, y: i64) {
        if y != self.last_y {
            let state = if self.last_y < y {
                State::Ascending
            } else {
                State::Descending
            };
            if self.state != state {
                if self.state != State::Unknown {
                    self.end_profile();
                }
                self.new_profile(state);
            }
            let (lx, ly, miny, maxy) = (self.last_x, self.last_y, self.min_y, self.max_y);
            if state == State::Ascending {
                self.line_up(lx, ly, x, y, miny, maxy);
            } else {
                self.line_down(lx, ly, x, y, miny, maxy);
            }
        }
        self.last_x = x;
        self.last_y = y;
    }

    fn conic_to(&mut self, cx: i64, cy: i64, x: i64, y: i64) {
        // The Bezier stack: 2 * MaxBezier + 1 points in C; grown on demand.
        let mut arcs = vec![Pt::default(); 2 * 32 + 3];
        arcs[2] = Pt {
            x: self.last_x,
            y: self.last_y,
        };
        arcs[1] = Pt { x: cx, y: cy };
        arcs[0] = Pt { x, y };
        let mut a = 0usize;
        loop {
            if a + 4 >= arcs.len() {
                arcs.resize(arcs.len() * 2, Pt::default());
            }
            let y1 = arcs[a + 2].y;
            let y2 = arcs[a + 1].y;
            let y3 = arcs[a].y;
            let x3 = arcs[a].x;
            let (ymin, ymax) = if y1 <= y3 { (y1, y3) } else { (y3, y1) };
            if y2 < self.floor(ymin) || y2 > self.ceiling(ymax) {
                split_conic(&mut arcs, a);
                a += 2;
                continue;
            } else if y1 != y3 {
                let state = if y1 < y3 {
                    State::Ascending
                } else {
                    State::Descending
                };
                if self.state != state {
                    if self.state != State::Unknown {
                        self.end_profile();
                    }
                    self.new_profile(state);
                }
                let (miny, maxy) = (self.min_y, self.max_y);
                // Bezier_Up may split the arc upwards; leave it room.
                let need = a + 2 * 40;
                if arcs.len() < need {
                    arcs.resize(need, Pt::default());
                }
                if state == State::Ascending {
                    self.bezier_up(&mut arcs, a, miny, maxy);
                } else {
                    self.bezier_down(&mut arcs, a, miny, maxy);
                }
            }
            self.last_x = x3;
            self.last_y = y3;
            if a == 0 {
                break;
            }
            a -= 2;
        }
    }

    fn decompose_curve(&mut self, first: usize, last: usize, flipped: bool) {
        let o = self.outline;
        let pt = |s: &Self, i: usize| {
            let (x, y) = o.points[i];
            let (x, y) = (s.scaled(x), s.scaled(y));
            if flipped { (y, x) } else { (x, y) }
        };
        let mut v_start = pt(self, first);
        let v_last = pt(self, last);
        let mut limit = last;
        let mut point = first;
        if !o.on[first] {
            if o.on[last] {
                v_start = v_last;
                limit -= 1;
            } else {
                v_start = ((v_start.0 + v_last.0) / 2, (v_start.1 + v_last.1) / 2);
            }
            // `point--`: the loop below starts at `first` itself.
            point = point.wrapping_sub(1);
        }
        self.last_x = v_start.0;
        self.last_y = v_start.1;
        while point != limit {
            point = point.wrapping_add(1);
            if o.on[point] {
                let (x, y) = pt(self, point);
                self.line_to(x, y);
                continue;
            }
            let mut v_control = pt(self, point);
            loop {
                if point < limit {
                    point += 1;
                    let (x, y) = pt(self, point);
                    if o.on[point] {
                        self.conic_to(v_control.0, v_control.1, x, y);
                        break;
                    }
                    let mid = ((v_control.0 + x) / 2, (v_control.1 + y) / 2);
                    self.conic_to(v_control.0, v_control.1, mid.0, mid.1);
                    v_control = (x, y);
                } else {
                    self.conic_to(v_control.0, v_control.1, v_start.0, v_start.1);
                    return;
                }
            }
        }
        self.line_to(v_start.0, v_start.1);
    }

    fn convert_glyph(&mut self, flipped: bool) {
        self.profiles.clear();
        self.cur = None;
        self.turns.clear();
        self.top_turn = None;
        let mut first = 0usize;
        for &last in &self.outline.contours.clone() {
            self.state = State::Unknown;
            self.g_profile = None;
            self.decompose_curve(first, last, flipped);
            first = last + 1;
            // A contour too small to cross a scanline has no profile.
            let Some(g) = self.g_profile else {
                continue;
            };
            // Do the contour's first and last arcs join on a scanline?
            let g_flow = self.profiles[g].flags & FLOW_UP;
            if self.frac(self.last_y) == 0
                && self.last_y >= self.min_y
                && self.last_y <= self.max_y
                && self
                    .cur
                    .as_ref()
                    .is_some_and(|p| p.flags & FLOW_UP == g_flow)
            {
                self.cur.as_mut().expect("profile").x.pop();
            }
            self.end_profile();
        }
        // Finalize_Profile_Table: `next` is the successor in the contour.
        for i in 0..self.profiles.len().saturating_sub(1) {
            if self.profiles[i + 1].next == self.profiles[i].next {
                self.profiles[i].next = i + 1;
            }
        }
    }

    fn sweep_span(&mut self, sweep: Sweep, y: i32, x1: i64, x2: i64) {
        match sweep {
            Sweep::Vertical => {
                let mut e1 = self.trunc(self.ceiling(x1)) as i32;
                let mut e2 = self.trunc(self.floor(x2)) as i32;
                if e2 >= 0 && e1 <= self.b_right {
                    if e1 < 0 {
                        e1 = 0;
                    }
                    if e2 > self.b_right {
                        e2 = self.b_right;
                    }
                    for x in e1..=e2 {
                        self.set(x, y);
                    }
                }
            }
            Sweep::Horizontal => {
                let e1 = self.ceiling(x1);
                let e2 = self.floor(x2);
                if x1 == e1 {
                    let e1 = self.trunc(e1) as i32;
                    if e1 >= 0 && e1 <= self.b_top {
                        self.set(y, e1);
                    }
                }
                if x2 == e2 {
                    let e2 = self.trunc(e2) as i32;
                    if e2 >= 0 && e2 <= self.b_top {
                        self.set(y, e2);
                    }
                }
            }
        }
    }

    fn sweep_drop(&mut self, sweep: Sweep, y: i32, x1: i64, x2: i64) {
        let mut e1 = self.trunc(x1) as i32;
        let e2 = self.trunc(x2) as i32;
        let max = match sweep {
            Sweep::Vertical => self.b_right,
            Sweep::Horizontal => self.b_top,
        };
        let at = |e: i32| match sweep {
            Sweep::Vertical => (e, y),
            Sweep::Horizontal => (y, e),
        };
        if e1 < 0 || e1 > max {
            e1 = e2;
        } else if e2 >= 0 && e2 <= max {
            let (px, py) = at(e2);
            if self.is_set(px, py) {
                return;
            }
        }
        if e1 >= 0 && e1 <= max {
            let (px, py) = at(e1);
            self.set(px, py);
        }
    }

    /// Pixel `(x, y)` with y counted up from the bottom line, as in C.
    fn set(&mut self, x: i32, y: i32) {
        let row = self.b_top - y;
        let w = self.bitmap.width;
        self.bitmap.bits[(row * w + x) as usize] = true;
    }

    fn is_set(&self, x: i32, y: i32) -> bool {
        let row = self.b_top - y;
        self.bitmap.bits[(row * self.bitmap.width + x) as usize]
    }

    fn draw_sweep(&mut self, sweep: Sweep) {
        let mut turns = self.turns.clone();
        turns.push(self.top_turn.expect("turns"));
        let min_y = turns[0] as i32;
        let max_y = turns[turns.len() - 1] as i32 - 1;
        let mut waiting: Vec<usize> = (0..self.profiles.len()).collect();
        let mut left: Vec<usize> = Vec::new();
        let mut right: Vec<usize> = Vec::new();
        let mut k = 0usize;
        let mut y = min_y;
        while y <= max_y {
            let mut i = 0;
            while i < waiting.len() {
                let pi = waiting[i];
                if self.profiles[pi].start == y {
                    waiting.remove(i);
                    let list = if self.profiles[pi].flags & FLOW_UP != 0 {
                        &mut left
                    } else {
                        &mut right
                    };
                    let x = self.profiles[pi].cur_x;
                    let pos = list
                        .iter()
                        .position(|&q| self.profiles[q].cur_x >= x)
                        .unwrap_or(list.len());
                    list.insert(pos, pi);
                } else {
                    i += 1;
                }
            }
            k += 1;
            let y_turn = turns[k] as i32;
            loop {
                let mut dropouts = 0;
                for n in 0..left.len().min(right.len()) {
                    let (pl, pr) = (left[n], right[n]);
                    let mut x1 = self.profiles[pl].cur_x;
                    let mut x2 = self.profiles[pr].cur_x;
                    if x1 > x2 {
                        std::mem::swap(&mut x1, &mut x2);
                    }
                    if self.ceiling(x1) <= self.floor(x2) {
                        self.sweep_span(sweep, y, x1, x2);
                        continue;
                    }
                    let doc = self.profiles[pl].flags & 7;
                    if doc & 2 != 0 {
                        continue;
                    }
                    if doc & 1 != 0 {
                        let l = &self.profiles[pl];
                        if l.height == 1
                            && l.next == pr
                            && !(l.flags & OVERSHOOT_TOP != 0 && x2 - x1 >= self.precision_half)
                        {
                            continue;
                        }
                        if l.offset == 0
                            && self.profiles[pr].next == pl
                            && !(l.flags & OVERSHOOT_BOTTOM != 0 && x2 - x1 >= self.precision_half)
                        {
                            continue;
                        }
                    }
                    if doc & 4 != 0 {
                        x2 = self.floor((x1 + x2 + self.precision * 63 / 64) >> 1);
                        x1 = if x1 > x2 {
                            x2 + self.precision
                        } else {
                            x2 - self.precision
                        };
                    } else {
                        x2 = self.floor(x2);
                        x1 = self.ceiling(x1);
                    }
                    self.profiles[pl].cur_x = x2;
                    self.profiles[pr].cur_x = x1;
                    self.profiles[pl].flags |= DROPOUT;
                    dropouts += 1;
                }
                let mut n = 0;
                while dropouts > 0 {
                    let (pl, pr) = (left[n], right[n]);
                    if self.profiles[pl].flags & DROPOUT != 0 {
                        let (a, b) = (self.profiles[pl].cur_x, self.profiles[pr].cur_x);
                        self.sweep_drop(sweep, y, a, b);
                        self.profiles[pl].flags &= !DROPOUT;
                        dropouts -= 1;
                    }
                    n += 1;
                }
                self.increment(&mut left, 1);
                self.increment(&mut right, -1);
                y += 1;
                if y >= y_turn {
                    break;
                }
            }
        }
    }

    fn increment(&mut self, list: &mut Vec<usize>, flow: i32) {
        list.retain(|&pi| {
            let p = &mut self.profiles[pi];
            p.height -= 1;
            if p.height != 0 {
                p.offset += flow;
                p.cur_x = p.x[p.offset as usize];
                true
            } else {
                false
            }
        });
        let mut i = 0;
        while i + 1 < list.len() {
            if self.profiles[list[i]].cur_x <= self.profiles[list[i + 1]].cur_x {
                i += 1;
            } else {
                list.swap(i, i + 1);
                i = 0;
            }
        }
    }

    fn render_single_pass(&mut self, flipped: bool, y_max: i32) {
        self.min_y = 0;
        self.max_y = i64::from(y_max) * self.precision;
        self.convert_glyph(flipped);
        if !self.profiles.is_empty() {
            self.draw_sweep(if flipped {
                Sweep::Horizontal
            } else {
                Sweep::Vertical
            });
        }
    }
}

fn split_conic(base: &mut [Pt], i: usize) {
    let b = &mut base[i..];
    b[4].x = b[2].x;
    let a = b[0].x + b[1].x;
    let c = b[1].x + b[2].x;
    b[3].x = c >> 1;
    b[2].x = (a + c) >> 2;
    b[1].x = a >> 1;
    b[4].y = b[2].y;
    let a = b[0].y + b[1].y;
    let c = b[1].y + b[2].y;
    b[3].y = c >> 1;
    b[2].y = (a + c) >> 2;
    b[1].y = a >> 1;
}

/// Scan-conversion settings, from the TrueType loader's outline flags.
#[derive(Clone, Copy)]
pub struct Mode {
    /// `FT_OUTLINE_HIGH_PRECISION` (set below 24 ppem).
    pub high_precision: bool,
    /// The drop-out bits of `Render_Glyph`: 1 excludes stubs, 2 ignores
    /// drop-outs, 4 uses smart rounding.
    pub drop_out_control: u16,
}

/// The control box of `outline`, in 26.6: (x_min, y_min, x_max, y_max).
pub fn cbox(outline: &Outline) -> (i64, i64, i64, i64) {
    let mut b = (i64::MAX, i64::MAX, i64::MIN, i64::MIN);
    for &(x, y) in &outline.points {
        b.0 = b.0.min(x);
        b.1 = b.1.min(y);
        b.2 = b.2.max(x);
        b.3 = b.3.max(y);
    }
    if outline.points.is_empty() {
        (0, 0, 0, 0)
    } else {
        b
    }
}

/// `FT_Render_Glyph(FT_RENDER_MODE_MONO)`: the bitmap and its
/// `(bitmap_left, bitmap_top)`.
pub fn render_mono(outline: &Outline, mode: Mode) -> (Bitmap, i32, i32) {
    let (x0, y0, x1, y1) = cbox(outline);
    // ft_glyphslot_preset_bitmap, FT_RENDER_MODE_MONO.
    let (mut px0, mut px1) = (x0 >> 6, x1 >> 6);
    let (mut py0, mut py1) = (y0 >> 6, y1 >> 6);
    let (rx0, rx1, ry0, ry1) = (x0 & 63, x1 & 63, y0 & 63, y1 & 63);
    px0 += (rx0 + 31) >> 6;
    px1 += (rx1 + 32) >> 6;
    if px0 == px1 {
        if ((rx0 + 31) & 63) - 31 + ((rx1 + 32) & 63) - 32 < 0 {
            px0 -= 1;
        } else {
            px1 += 1;
        }
    }
    py0 += (ry0 + 31) >> 6;
    py1 += (ry1 + 32) >> 6;
    if py0 == py1 {
        if ((ry0 + 31) & 63) - 31 + ((ry1 + 32) & 63) - 32 < 0 {
            py0 -= 1;
        } else {
            py1 += 1;
        }
    }
    let width = (px1 - px0) as i32;
    let rows = (py1 - py0) as i32;
    let mut bitmap = Bitmap {
        width,
        rows,
        bits: vec![false; (width.max(0) * rows.max(0)) as usize],
    };
    let left = px0 as i32;
    let top = py1 as i32;
    if width <= 0 || rows <= 0 || outline.points.is_empty() {
        return (bitmap, left, top);
    }
    // Translate into the bitmap: x_shift = -left * 64, y_shift = (rows - top) * 64.
    let shifted = Outline {
        points: outline
            .points
            .iter()
            .map(|&(x, y)| (x - px0 * 64, y + (i64::from(rows) - py1) * 64))
            .collect(),
        on: outline.on.clone(),
        contours: outline.contours.clone(),
    };
    let mut ras = Ras {
        precision_bits: 0,
        precision: 0,
        precision_half: 0,
        precision_scale: 0,
        precision_step: 0,
        drop_out_control: mode.drop_out_control,
        last_x: 0,
        last_y: 0,
        min_y: 0,
        max_y: 0,
        state: State::Unknown,
        profiles: Vec::new(),
        cur: None,
        g_profile: None,
        turns: Vec::new(),
        top_turn: None,
        b_top: rows - 1,
        b_right: width - 1,
        bitmap: &mut bitmap,
        outline: &shifted,
    };
    ras.set_high_precision(mode.high_precision);
    let (b_top, b_right) = (ras.b_top, ras.b_right);
    ras.render_single_pass(false, b_top);
    ras.render_single_pass(true, b_right);
    (bitmap, left, top)
}
