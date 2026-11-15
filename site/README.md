# Project website

Source of https://generatedsecrets.jabbas.eu — one static page, no build step.
Design: [docs/design/2026-11-15-project-website-design.md](../docs/design/2026-11-15-project-website-design.md).

| File | Purpose |
|---|---|
| `index.html` | The page. Plain HTML; the player and the copy buttons are the only JavaScript. |
| `style.css` | The only hand-written stylesheet. System fonts, dark and light through `prefers-color-scheme`. |
| `og.png` | Social preview image, 1200×630 (`og:image`; also uploaded as the repository social preview). Rendered from [`docs/assets/og-source.html`](../docs/assets/og-source.html), which is not published. |
| `vendor/asciinema-player/` | asciinema-player 3.18.0 (`asciinema-player.min.js`, `asciinema-player.css`, `LICENSE`). |

The page loads the recording by the relative URL `demo.cast`. `site/` does not
contain it: the deploy workflow copies `docs/assets/demo.cast` next to
`index.html`, so the repository keeps one copy.

## Preview locally

Copy the site and the recording into a scratch directory and serve it:

```sh
mkdir -p "$TMPDIR/site-preview"
cp -R site/. "$TMPDIR/site-preview/"
cp docs/assets/demo.cast "$TMPDIR/site-preview/demo.cast"
python3 -m http.server --directory "$TMPDIR/site-preview" 8000
```

Open http://127.0.0.1:8000/. Opening `index.html` through `file://` does not
work: the player fetches `demo.cast`.

## Where the copy comes from

The page ports text from the repository [README](../README.md): the one
sentence, "Why", "Try it in 5 minutes" (the same commands without the
`--version` pin, so the page does not go stale between releases), the
comparison (shortened; the page links to the full one) and the
release-candidate sentence.

**Sync rule:** when the README wording of these changes, update
`site/index.html` in the same pull request. There is no automated check.

The tagline also appears in the social preview image. **When the tagline
changes, update `site/index.html` AND `docs/assets/og-source.html`, then
re-render `site/og.png`** (see "Regenerating `og.png`").

## No third-party requests

No CDN, web font, analytics, cookie or remote image. External links are plain
`<a href>`. Check after every change:

```sh
grep -rnE 'https?://' site/ --include='*.html' --include='*.css'
```

Every hit must be a link, a `<meta>` URL or a comment — never a script,
stylesheet, font or image to load.

## Vendored files

asciinema-player 3.18.0, from
https://github.com/asciinema/asciinema-player/releases/tag/v3.18.0 (the licence
file is from the `v3.18.0` source tag). SHA-256:

```text
752c9b528b664f5b16f27c68ba22fc7973d1d6063a1c3e4795a4c8015943acf6  vendor/asciinema-player/asciinema-player.min.js
981fe7bc6042048b2a373b6da297f747e324e90a8b6264e833e3a92cec22dfb9  vendor/asciinema-player/asciinema-player.css
63422284ed65be54ab38d8a94e7518d27e222fbb26c9ae4cd984c63926e91c12  vendor/asciinema-player/LICENSE
```

To upgrade, download the new release assets, replace the files, update the
version and checksums here, and check that the poster frame and playback still
work.

## Regenerating `og.png`

`og.png` is a screenshot of `docs/assets/og-source.html` (wordmark, the README
sentence, a cropped `app-admin.yaml` terminal; system fonts). The source lives
under `docs/assets/` so it is not published with the site. Edit it, then run
from the repository root:

```sh
"/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" --headless=new --hide-scrollbars --force-device-scale-factor=1 --window-size=1200,630 --screenshot=site/og.png "file://$PWD/docs/assets/og-source.html"
```

Look at the result: the `apiVersion` line must end before the right edge while
the terminal frame still reads as cropped.
