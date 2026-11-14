# Logo sources

`vendor/wordmark-light.svg` is committed (the same bytes are the gen-frames test fixture). `cargo xtask gen-frames` checks its SHA-256 (`8a16e64f1e13df5e03b2312315cfa6270443dc130e9ebcb973cb72f8afc96111`) and exits before drawing if the bytes differ.

The moq.dev favicon is not redistributed. The command downloads it into gitignored `assets/` and exits before use unless the bytes match SHA-256 `241bd5a8c3291315d7a617a4ac0c869c9cc5e4a34f6342ff91ae999c3d3dc8f1`.

| File | URL | License |
|---|---|---|
| `moqdev-layout_favicon.svg` | https://moq.dev/layout/favicon.svg | No license found on moq.dev. Do not redistribute the file. Fetch only to generate frames locally, and only when the pinned SHA-256 matches. |
| `wordmark-light.svg` | https://raw.githubusercontent.com/moq-dev/rind/main/admin/wordmark-light.svg | Same as rind: Apache-2.0, ISC, or MIT-0. Vendored in this tree. |

`oled_demo/frames.h` and `preview*.png` are derived from both SVGs, so they are also generated at build time and gitignored.
