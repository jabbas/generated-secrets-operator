# Project Website — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Publish a one-page website for Generated Secrets Operator at `https://generatedsecrets.jabbas.eu`, built from the approved Stitch design, with the demo player and copy taken from the merged README.

**Architecture:** One branch `site/website` in the OpenCode worktree `/Users/jabbas/.local/share/opencode/worktree/26553b/website`, cut from `origin/main` after README pull request #38 is merged. Task 1 builds `site/` (HTML, one CSS file, vendored player, social image). Task 2 adds the deploy workflow. Task 3 is the whole-branch review, owner review and merge. Task 4 publishes the site on the custom domain (owner DNS steps plus agent `gh api` steps) and links it from the repository.

**Tech Stack:** Plain HTML and CSS, asciinema-player 3.18.0 (vendored, no build), GitHub Actions (`actions/upload-pages-artifact`, `actions/deploy-pages`), GitHub Pages with a custom domain, headless Chrome for screenshots and the social image, Lighthouse for accessibility.

**Spec:** `docs/design/2026-11-15-project-website-design.md` (approved by the owner, 2026-11-15, with changes: address `generatedsecrets.jabbas.eu`, Stitch design, no separate manifest section). Copy source: `README.md` on `main` after pull request #38.

## Global Constraints

- English; plain, no marketing language; abbreviations expanded on first use.
- Names: project "Generated Secrets Operator", kind `GeneratedSecret`, short name `gensecret`, API group `generatedsecrets.jabbas.eu`, chart `generated-secrets-operator`, namespace `generated-secrets-system`, repository `jabbas/generated-secrets-operator`, site `https://generatedsecrets.jabbas.eu`.
- No third-party requests at run time: no CDN, no web fonts, no analytics, no cookies, no remote images. External **links** (`<a href>`) are allowed.
- JavaScript only for the vendored asciinema-player and an optional few-line inline copy button; the page must stay readable with JavaScript disabled.
- One hand-written CSS file; system font stacks only: sans `-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif`; mono `ui-monospace, SFMono-Regular, Menlo, Consolas, "Liberation Mono", monospace`.
- Palette, radius, borders exactly as in design §5 (dark `#101413`/`#161C1A`/`#0B0F0E`/`#26302D`/`#E6EBE9`/`#9AA5A1`/accent `#4EC9A0`; light `#FBFCFB`/`#F2F5F4`/`#DDE3E1`/`#15201C`/`#56625E`/accent `#0F7A5A`; demo terminal stays dark; 6 px radius; 1 px borders; no shadows). Wordmark `$_ Generated Secrets Operator`, `$_` in the accent colour.
- Page weight under 500 KB excluding `demo.cast`.
- `site/` never contains its own copy of the recording; the workflow copies `docs/assets/demo.cast`.
- Copy is ported from the merged README, not rewritten: the one sentence ("Generate Kubernetes Secrets in-cluster: passwords, tokens, SSH keys. Declared next to your app; the values are never stored in Git."), "Why" (three pains and the promise), "Try it in 5 minutes" (same commands **without** `--version`), the comparison (short), and the release-candidate claim exactly as the README states it ("Every release is promoted from a release candidate that passed a test on a real cluster").
- Actions pinned like the existing workflows: full commit SHA plus `# vX.Y.Z` comment (see `.github/workflows/ci.yml`).
- No change to operator code, chart, `ci.yml` or `release.yml`; a site change never triggers or blocks a release.
- Never run `gh auth token`; no force pushes; no tags. Do not touch `/Users/jabbas/Projects/secretspec-operator` (main clone) except reading `.superpowers/promotion/site-design/` (Stitch exports, read-only).

## Review Focus

1. **Visitor with JavaScript disabled** — expected: all text, the manifest, the commands and links are visible; in place of the player a linked GIF (`https://github.com/jabbas/generated-secrets-operator/blob/main/docs/assets/demo.gif`) or the poster text. Test in Task 1.
2. **Phone-width screen (390 px)** — expected: no horizontal page scroll; long commands scroll inside their code block; the player scales. Test in Task 1 (mobile screenshot).
3. **Light theme** (`prefers-color-scheme: light`) — expected: readable contrast (Lighthouse accessibility ≥ 95), demo terminal stays dark. Test in Task 1.
4. **Copied "Try it" commands** — expected: they run verbatim on a fresh kind cluster and install the latest final release (unpinned `helm install`). Test in Task 3.
5. **Hidden third-party request** (a leftover Tailwind/Google Fonts/Material Symbols reference from the Stitch export) — expected: none. Test in Task 1 (grep plus a headless-browser network log).

---

### Task 1: The page (`site/`)

**Files:**
- Create: `site/index.html`, `site/style.css`, `site/og.png`, `site/vendor/asciinema-player/asciinema-player.min.js`, `site/vendor/asciinema-player/asciinema-player.css`, `site/vendor/asciinema-player/LICENSE`, `site/README.md` (how to preview locally; where the copy comes from; the README-sync rule)
- Reference (read-only): `/Users/jabbas/Projects/secretspec-operator/.superpowers/promotion/site-design/{desktop-dark,desktop-light,mobile-dark,og-image}.{html,png}`, `README.md`

**Interfaces:**
- Produces: `site/` as a self-contained static directory; at deploy time `demo.cast` is placed next to `index.html` (`site/demo.cast` in the artifact only), and `index.html` loads it by the relative URL `demo.cast`.

- [ ] **Step 1:** Branch: `git fetch origin`; confirm `origin/main` contains the merged README (`git log origin/main --oneline -5` shows the merge of pull request #38); `git switch -c site/website origin/main`.
- [ ] **Step 2:** Vendor asciinema-player 3.18.0: download `asciinema-player.min.js` and `asciinema-player.css` from `https://github.com/asciinema/asciinema-player/releases/download/v3.18.0/` and its licence from the tagged source; record the SHA-256 of each file in `site/README.md`.
- [ ] **Step 3:** Write `site/index.html` and `site/style.css`, porting the Stitch screens (layout, spacing, components) to semantic HTML and one CSS file; sections in design §4 order (header, player with the sentence "The operator creates the Secret `app-admin` with a generated 32-character password — once." and a muted "asciinema recording" line, Why, Try it, When to use it / when not, Built to be trusted, footer). Footer links: Documentation (`…/tree/main/docs`), Changelog, Security policy, GitHub, author "Grzegorz Dzięgielewski" → `https://github.com/jabbas`; **no Artifact Hub link yet** (design §7 step 5). Player: `AsciinemaPlayer.create('demo.cast', element, { poster: 'npt:0:01', fit: 'width', idleTimeLimit: 2, autoplay: false })` — choose the poster time so it shows the title and manifest. `<noscript>` fallback: the GIF link. Meta: `title`, `description`, canonical `https://generatedsecrets.jabbas.eu/`, `og:title`, `og:description`, `og:url`, `og:image` = `https://generatedsecrets.jabbas.eu/og.png`, `twitter:card` = `summary_large_image`. Code blocks: pre-rendered `<span>` classes for YAML/shell highlighting (no library). Copy button: optional, inline, a few lines, progressive enhancement.
- [ ] **Step 4:** `site/og.png` (1200×630): render the Stitch `og-image.html` with headless Chrome after replacing its CDN/fonts with system fonts, or recreate it in plain HTML/CSS and screenshot; check the text is current (one sentence as in the README).
- [ ] **Step 5 (Review Focus 5):** `grep -rnE 'https?://' site/ --include='*.html' --include='*.css'` — every hit is an `<a href>`, a `<meta>` URL or a comment, never `src=`/`href` of a stylesheet/script/font/image on another host. Then serve locally (`python3 -m http.server --directory <tmp copy of site/ + demo.cast>`), load with headless Chrome logging network requests: only same-origin requests. Expected: zero third-party requests.
- [ ] **Step 6 (Review Focus 1–3):** Screenshots to `$TMPDIR/opencode/site-*.png`: desktop dark, desktop light (emulated `prefers-color-scheme`), mobile 390 px dark, and desktop with JavaScript disabled. Check: no horizontal page scroll at 390 px; noscript fallback visible. Lighthouse (`npx lighthouse http://localhost:<port>/ --only-categories=accessibility --output=json`, once per theme via emulation or a forced class) ≥ 95. Page weight: `du -ch site/` excluding nothing in `site/` (the cast is not in `site/`) < 500 KB.
- [ ] **Step 7:** Commit `feat(site): add the project website`.

### Task 2: Deploy workflow

**Files:**
- Create: `.github/workflows/pages.yml`

**Interfaces:**
- Consumes: `site/` and `docs/assets/demo.cast`.
- Produces: a Pages deployment of `site/` + `demo.cast`; job name `deploy`, environment `github-pages`.

- [ ] **Step 1:** `pages.yml`: `on: push` to `main` with `paths: [site/**, docs/assets/demo.cast, .github/workflows/pages.yml]`, plus `workflow_dispatch`; `permissions: contents: read, pages: write, id-token: write`; `concurrency: group: pages, cancel-in-progress: false`; one job `deploy` on `ubuntu-latest`, `environment: { name: github-pages, url: ${{ steps.deployment.outputs.page_url }} }`; steps: checkout; assemble (`mkdir -p _site && cp -R site/. _site/ && cp docs/assets/demo.cast _site/`); `actions/upload-pages-artifact` (v5) with `path: _site`; `actions/deploy-pages` (v5) with `id: deployment`. Pin `actions/checkout` to the same SHA as `ci.yml`; resolve SHAs for `upload-pages-artifact@v5` and `deploy-pages@v5` with `gh api repos/actions/<name>/git/ref/tags/<tag>` (dereference annotated tags) and pin them with `# vX.Y.Z`.
- [ ] **Step 2:** If Renovate's config pins actions (check `renovate.json`), confirm the new workflow is covered by the existing rules; change nothing else.
- [ ] **Step 3:** Validate: `actionlint .github/workflows/pages.yml` if installed (else `yq` parse); confirm `release.yml` and `.github/scripts/release-graph-check.sh` are unaffected (`bash .github/scripts/release-graph-check.sh` still passes).
- [ ] **Step 4:** Commit `ci(site): deploy the website to GitHub Pages`.

### Task 3: Review, try-it check, owner review, merge

- [ ] **Step 1 (Review Focus 4):** On a fresh kind cluster (default name must be free; otherwise report and stop), run the site's "Try it" commands verbatim — extract them from `site/index.html` (strip markup) into `$TMPDIR/opencode/site-tryit.sh` and run with `bash -e`; delete only that cluster. Expected: every command succeeds; the unpinned install resolves to `0.2.0`.
- [ ] **Step 2:** Push the branch, open a pull request (`feat(site): add the project website`), wait for CI green. Pages is not enabled yet, so merging does not publish anything; the deploy job will fail or be skipped until Task 4 enables Pages — note this in the pull request body.
- [ ] **Step 3:** Whole-branch review by `@reviewer` (spec §2–§5, §8; Review Focus 1–5).
- [ ] **Step 4 — STOP (owner):** show the owner the four screenshots and the pull request; wait for approval.
- [ ] **Step 5:** Merge (merge commit, `--match-head-commit`, from a neutral directory).

### Task 4: Publish on the custom domain

- [ ] **Step 1 — STOP (owner):** verify `jabbas.eu` for GitHub Pages at account level (GitHub profile Settings → Pages → Add a domain → TXT record `_github-pages-challenge-jabbas.jabbas.eu` at the DNS provider; keep it permanently). Wait for the owner's confirmation that GitHub shows the domain as verified.
- [ ] **Step 2:** Enable Pages with build type "workflow" and set the custom domain: `gh api -X POST repos/jabbas/generated-secrets-operator/pages -f build_type=workflow` (or `PUT` if it exists), then `gh api -X PUT repos/jabbas/generated-secrets-operator/pages -f cname=generatedsecrets.jabbas.eu`. Run the `pages.yml` workflow (`gh workflow run pages.yml`) and confirm the deployment succeeded.
- [ ] **Step 3 — STOP (owner):** DNS record `generatedsecrets.jabbas.eu CNAME jabbas.github.io` (no wildcard; CAA, if any, must allow `letsencrypt.org`).
- [ ] **Step 4:** Poll `gh api repos/jabbas/generated-secrets-operator/pages` until the certificate is issued (`https_certificate.state` = `approved`), then `gh api -X PUT repos/jabbas/generated-secrets-operator/pages -F https_enforced=true`. Check `curl -sI https://generatedsecrets.jabbas.eu/` → 200 and `https://generatedsecrets.jabbas.eu/demo.cast` → 200; `curl -sI http://generatedsecrets.jabbas.eu/` redirects to HTTPS.
- [ ] **Step 5:** `gh repo edit jabbas/generated-secrets-operator --homepage https://generatedsecrets.jabbas.eu`. Small pull request adding a "Website" link to the README (one line near the top); review and merge as for other doc changes.
- [ ] **Step 6 — owner:** upload `site/og.png` as the repository social preview (Settings → General → Social preview; the API cannot set it).
- [ ] **Step 7:** Done criteria (design §8): HTTPS 200; player plays `demo.cast`; no third-party requests on the live page (headless network log); Lighthouse accessibility ≥ 95 on the live page in both themes; `og:image` resolves (`curl -sI https://generatedsecrets.jabbas.eu/og.png` → 200, `image/png`); homepage URL set.
