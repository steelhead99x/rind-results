# gen-frames references

What `cargo xtask gen-frames` is checked against (`tools/xtask/src/gen_frames/tests.rs`).
All of it was made on Debian with the old `baremetal/mg24/oled-demo/tools/gen_frames.py`
(cairosvg 2.9.1, Pillow 13.0.0 with FreeType 2.14.3 and libraqm 0.11.0) and the DejaVu 2.37
fonts in `tools/xtask/fonts`.

- `python-frames.h`: the script's `frame_rind` and `frame_bench` arrays.
- `moq-python.sha256`: SHA-256 of the script's `frame_moq` array (1024 bytes, page order).
  The array itself is not here, because it is drawn from the moq.dev favicon, which has no
  license (see `baremetal/mg24/oled-demo/SOURCES.md`).
- `preview-bench-python.sha256`: SHA-256 of `preview-3-bench.png`'s RGB pixels.
- `pillow-text.tsv`: 510 strings drawn with `ImageDraw.text((x, 5), text, fill=255)` and
  `fontmode = "1"` on a 700x40 "L" image: both fonts, sizes 7 to 23, five strings, x = 0, 3
  and 2.5. Each line has `textlength * 64` and the SHA-256 of the image bytes.
- `wordmark-light.svg`: the rind wordmark, unmodified from moq-dev/rind
  `admin/wordmark-light.svg` (Apache-2.0, ISC, or MIT-0, like rind). The same
  bytes are vendored at `baremetal/mg24/oled-demo/vendor/wordmark-light.svg`.

The text matches Pillow exactly. The two logos are drawn with resvg rather than cairo, and
cairo anti-aliases edges differently. After the script's threshold, box downsample and
second threshold, one pixel of the 60x60 moq.dev mark and two pixels of the 58x48 wordmark
come out the other way. The tests list those pixels. Any other change fails.
