# Logo sources

Files are fetched by `cargo xtask gen-frames` into `assets/` (gitignored). They are not committed, except that the wordmark is also a test fixture in `tools/xtask/tests/fixtures/gen-frames/`.

| File | URL | License |
|---|---|---|
| `moqdev-layout_favicon.svg` | https://moq.dev/layout/favicon.svg | No license found on moq.dev. Do not redistribute the file. Fetch only to generate frames locally. |
| `wordmark-light.svg` | https://raw.githubusercontent.com/moq-dev/rind/main/admin/wordmark-light.svg | Same as rind: Apache-2.0, ISC, or MIT-0. |

`oled_demo/frames.h` and `preview*.png` are derived from both SVGs, so they are also generated at build time and gitignored.
