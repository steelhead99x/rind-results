// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
//! Parity with the old `tools/gen_frames.py`. The references in
//! `tests/fixtures/gen-frames` come from that script and from Pillow 13.0
//! (FreeType 2.14.3, libraqm 0.11) with cairosvg 2.9.1; see its README.

use super::*;
use sha2::{Digest, Sha256};

fn fixtures() -> PathBuf {
    crate::root().join("tools/xtask/tests/fixtures/gen-frames")
}

fn fonts() -> Fonts {
    Fonts::load(&crate::root().join("tools/xtask/fonts")).expect("fonts")
}

fn sha256_hex(data: &[u8]) -> String {
    Sha256::digest(data)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

fn fixture_text(name: &str) -> String {
    std::fs::read_to_string(fixtures().join(name)).expect(name)
}

/// One `static const uint8_t frame_<name>[1024] = {...};` block.
fn python_block(name: &str) -> String {
    let text = fixture_text("python-frames.h");
    let start = text
        .find(&format!("static const uint8_t frame_{name}[1024]"))
        .expect("frame");
    let end = start + text[start..].find("};").expect("end") + 2;
    text[start..end].to_string()
}

fn ours_block(name: &str, f: &Gray) -> String {
    let h = frames_h(&[(name, f.clone())]);
    let start = h.find("static const").expect("frame");
    let end = start + h[start..].find("};").expect("end") + 2;
    h[start..end].to_string()
}

fn parse_pages(block: &str) -> Vec<u8> {
    let body = &block[block.find('{').expect("{") + 1..];
    body.split(|c: char| c == ',' || c.is_whitespace() || c == '}' || c == ';')
        .filter_map(|t| t.strip_prefix("0x"))
        .map(|t| u8::from_str_radix(t, 16).expect("hex"))
        .collect()
}

/// Pixels (x, y) that differ between two page tables.
fn pixel_diff(a: &[u8], b: &[u8]) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    for y in 0..H {
        for x in 0..W {
            let i = (y / 8) * W + x;
            if (a[i] >> (y % 8)) & 1 != (b[i] >> (y % 8)) & 1 {
                out.push((x, y));
            }
        }
    }
    out
}

/// Logo pixels where resvg's anti-aliasing lands on the other side of the
/// script's thresholds than cairo's (all inside the 60x60 mark and the
/// 58x48 wordmark; the text matches).
const MOQ_DIFFS: &[(usize, usize)] = &[(13, 10)];
const RIND_DIFFS: &[(usize, usize)] = &[(46, 31), (37, 41)];

#[test]
fn bench_frame_matches_python() {
    let f = frame_bench(&fonts()).expect("frame");
    assert_eq!(ours_block("bench", &f), python_block("bench"));
}

#[test]
fn rind_frame_matches_python_but_two_wordmark_pixels() {
    let wordmark = read(&fixtures().join("wordmark-light.svg")).expect("svg");
    let f = frame_rind(&wordmark, &fonts()).expect("frame");
    let diff = pixel_diff(&pages(&f), &parse_pages(&python_block("rind")));
    let mut want = RIND_DIFFS.to_vec();
    want.sort_by_key(|&(x, y)| (y, x));
    assert_eq!(diff, want);
    // Both are inside the wordmark, pasted at (35, 1) and 58x48.
    assert!(
        diff.iter()
            .all(|&(x, y)| (35..93).contains(&x) && (1..49).contains(&y))
    );
}

/// The favicon is not redistributable, so it is not a fixture: run
/// `cargo xtask gen-frames` once (it fetches the SVG into the demo's
/// gitignored assets/), then `cargo test -p xtask -- --ignored`.
#[test]
#[ignore = "needs baremetal/mg24/oled-demo/assets/moqdev-layout_favicon.svg"]
fn moq_frame_matches_python_but_one_mark_pixel() {
    let path = crate::root().join("baremetal/mg24/oled-demo/assets/moqdev-layout_favicon.svg");
    let svg = read(&path).expect("favicon");
    assert_eq!(
        sha256_hex(&svg),
        "241bd5a8c3291315d7a617a4ac0c869c9cc5e4a34f6342ff91ae999c3d3dc8f1",
        "the reference was drawn from this favicon"
    );
    let mut f = frame_moq(&svg, &fonts()).expect("frame");
    for &(x, y) in MOQ_DIFFS {
        assert!(x < 62 && (2..62).contains(&y), "inside the mark");
        f.set(x, y, if f.get(x, y) == 0 { 255 } else { 0 });
    }
    assert_eq!(
        sha256_hex(&pages(&f)),
        fixture_text("moq-python.sha256").trim()
    );
}

#[test]
fn bench_preview_matches_python() {
    let f = frame_bench(&fonts()).expect("frame");
    let p = preview(&f);
    assert_eq!((p.w, p.h), (512, 256));
    let rgb: Vec<u8> = p.px.iter().flatten().copied().collect();
    assert_eq!(
        sha256_hex(&rgb),
        fixture_text("preview-bench-python.sha256").trim()
    );
    // And it survives the PNG round trip.
    let png = encode_png(&p).expect("png");
    let dec = png::Decoder::new(std::io::Cursor::new(png));
    let mut reader = dec.read_info().expect("png");
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).expect("png");
    assert_eq!(&buf[..info.buffer_size()], &rgb[..]);
}

/// Every line of the Pillow sweep: 2 fonts x sizes 7-23 x 5 strings x 3
/// start positions (one fractional), drawn and measured as Pillow does.
#[test]
fn text_matches_pillow() {
    let fonts = fonts();
    let mut n = 0;
    for line in fixture_text("pillow-text.tsv").lines() {
        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        let cols: Vec<&str> = line.splitn(6, '\t').collect();
        let data = match cols[0] {
            "DejaVuSans.ttf" => &fonts.sans,
            "DejaVuSans-Bold.ttf" => &fonts.bold,
            other => panic!("font {other}"),
        };
        let size: u32 = cols[1].parse().expect("size");
        let x: f64 = cols[2].parse().expect("x");
        let length: i64 = cols[3].parse().expect("length");
        let text = cols[5];
        let font = text::Font::new(data, size).expect("font");
        assert_eq!(
            (font.text_length(text).expect("layout") * 64.0) as i64,
            length,
            "{line}"
        );
        let mut img = Gray::new(700, 40);
        font.draw(&mut img, x, 5.0, text).expect("draw");
        assert_eq!(sha256_hex(&img.px), cols[4], "{line}");
        n += 1;
    }
    assert_eq!(n, 510);
}

#[test]
fn vendored_wordmark_matches_pin() {
    assert_eq!(sha256_hex(WORDMARK), WORDMARK_SHA256);
    let fixture = read(&fixtures().join("wordmark-light.svg")).expect("svg");
    assert_eq!(sha256_hex(&fixture), WORDMARK_SHA256);
}

#[test]
fn favicon_digest_check_fails_closed() {
    assert!(check_digest(FAVICON_NAME, b"not-the-svg", FAVICON_SHA256).is_err());
    assert!(check_digest(FAVICON_NAME, WORDMARK, FAVICON_SHA256).is_err());
}

#[test]
fn frames_h_names_the_command() {
    let f = Gray::new(W, H);
    let h = frames_h(&[("a", f.clone()), ("b", f)]);
    assert!(h.starts_with("/* Generated by `cargo xtask gen-frames`."));
    assert!(h.contains("#define DEMO_FRAME_COUNT 2\n"));
    assert!(h.ends_with(
        "static const uint8_t *const demo_frames[DEMO_FRAME_COUNT] = {frame_a, frame_b};\n"
    ));
}
