# Project Website — Design

| | |
|---|---|
| **Status** | Approved by owner (2026-11-15), with changes: address `generatedsecrets.jabbas.eu`; visual design from Stitch (§5); no separate manifest section (§4) |
| **Date** | 2026-11-15 |
| **Depends on** | Rename design (`docs/design/2026-11-15-rename-to-generated-secrets-operator-design.md`) merged; 0.2.0 final released; contributor-readiness pull requests A (demo recording `docs/assets/demo.cast`) and B (README rewrite) merged (§7) |

> **Decisions reviewed and approved by the owner on 2026-11-15** (also: §7 sequencing — the site is built after the README rewrite and is first public on its custom domain)
>
> 1. Plain hand-written HTML/CSS, no site generator (§3).
> 2. The site source lives in this repository under `site/` and deploys with GitHub Actions (§3).
> 3. Commands on the site do not pin a version (§4).
> 4. No analytics, no third-party requests (§3).
> 5. Visual direction (§5).

**Conventions.** The key words MUST, MUST NOT, SHOULD, SHOULD NOT and MAY are normative (RFC 2119 sense).
Every decision carries its origin tag:

- **[owner]** — decided by the project owner.
- **[architect]** — architect's call; the owner may override it during review.

Section 9 lists what is deliberately left to the builder.

---

## 1. Intent

- This is step 4 of the promotion campaign **[owner]**: rename → repository ready for strangers → Artifact Hub listing → website → Show HN.
- Address: `generatedsecrets.jabbas.eu` **[owner]** — the owner owns `jabbas.eu`. The address is identical to the API group `generatedsecrets.jabbas.eu` that appears in every manifest, so `apiVersion: generatedsecrets.jabbas.eu/v1alpha1` is also the project's address.
- Role **[architect]**: the Hacker News post links the GitHub repository (research shows repository links do better), so the website is not the main landing page. It is the shareable, visual front door for everything else (Reddit, Slack, social posts, the repository's homepage field, Artifact Hub links), the only place with the interactive demo player, and the source of the social preview image. It MUST convince in one screen and send people to try the operator and to GitHub.
- Audience: as in the contributor-readiness design — Kubernetes users who need credentials nobody has to know in advance.
- Success **[owner]**: stars, issues and pull requests from strangers. The site's job: the visitor understands in 10 seconds, believes in 1 minute (demo), tries it in 5 minutes (commands), or stars the repository.

## 2. Goals, non-goals, anti-goals

**Goals:**

- One page.
- Demo player.
- Copy-paste "try it" commands.
- Honest "when to use it / when not".
- Links to documentation, GitHub and Artifact Hub.
- Social preview image.

**Non-goals [architect]:**

- A documentation site. Documentation stays in the repository (one source of truth); the site links to it.
- A blog.
- Multiple languages.
- Search.
- A logo design project — a simple wordmark is enough.
- Analytics.

**Anti-goals** (the site MUST NOT have):

- Marketing superlatives or stock imagery.
- Cookie banners or trackers.
- Third-party fonts or CDNs (content delivery networks).
- Anything that breaks with JavaScript disabled, except the player itself.
- A page heavier than needed. Target: under 500 KB, excluding the recording.

## 3. Technology and hosting **[architect]**

- **Plain hand-written HTML and one CSS file.** No static site generator, no Node.js build, no framework. Requirement check: a single page has no problem a generator would solve, and every dependency is a moving part that can break.
- **The only JavaScript is asciinema-player 3.18.0** (`asciinema-player.min.js` and `asciinema-player.css`), vendored into `site/vendor/asciinema-player/` together with its licence file. It supports asciicast v3. Renovate SHOULD track it if a custom manager is cheap; otherwise the version is bumped by hand (there is no requirement to stay current).
- **Source in this repository under `site/`.** One source of truth for the recording: the deploy workflow copies `docs/assets/demo.cast` into the published artifact; `site/` MUST NOT keep its own copy.
- **Deployment:** a new workflow `.github/workflows/pages.yml`.
  - Triggers: push to `main` touching `site/**` or `docs/assets/demo.cast`, plus manual dispatch.
  - Steps: checkout; assemble `site/` plus `demo.cast`; `actions/upload-pages-artifact@v5`; `actions/deploy-pages@v5`.
  - Permissions: `contents: read`, `pages: write`, `id-token: write`. Environment: `github-pages`.
  - Actions MUST be pinned the same way the existing workflows pin them (check the convention in `ci.yml` / `release.yml`).
  - Pages source in repository settings: "GitHub Actions".
  - No `CNAME` file (it is ignored for Actions deployments). The custom domain is set in repository settings or through the API (`PUT /repos/jabbas/generated-secrets-operator/pages` with `cname`).
- **Separate from the release pipeline:** a site change never triggers or blocks a release.
- **No analytics, no cookies, no external requests.** GitHub's repository traffic insights (referrers) are enough to see what the site brings.
- The existing project page `jabbas.github.io/helm-charts` is not affected: a project repository with its own custom domain does not change other project pages.

## 4. Page content, top to bottom **[architect]**

1. **Header.** Wordmark "Generated Secrets Operator" and the one sentence used in the README: "Generate Kubernetes Secrets in-cluster: passwords, tokens, SSH keys. Declared next to your app, never stored in Git." Two buttons: "Try it in 5 minutes" (anchor to item 5) and "GitHub" (a plain link; the star count is NOT fetched — no third-party or API calls).
2. **Demo player.** `AsciinemaPlayer.create` with:
   - `poster` set to a frame that shows the title and the manifest (the recording's first frame already does — contributor-readiness design §7);
   - `fit: "width"`;
   - `idleTimeLimit: 2`;
   - no autoplay.

   Fallback without JavaScript: a link to the GIF in the repository.

   Directly under the player, one sentence: "The operator creates the Secret `app-admin` with a generated 32-character password — once.", and a muted line "asciinema recording". **[owner]** There is no separate manifest section: the player's poster frame already shows the manifest, and the "Try it" block (item 5) carries it as text.
3. *(removed by the owner: a separate "Declare it next to your app" manifest section duplicated the player's poster frame)*
4. **Why.** The same three pains and the one promise as the README.
5. **Try it in 5 minutes.** The same commands as the README, except without the `--version` pin, so the page never goes stale between releases (`helm install` picks the latest final release). Copy buttons are optional and are the only other allowed JavaScript (a few lines, inline).
6. **When to use it, and when not.** A short version of the README comparison, with a link to the full one.
7. **Trust signals**, one line each:
   - Apache-2.0.
   - cosign-signed multi-architecture images with an SBOM (software bill of materials).
   - `FROM scratch` image.
   - Least-privilege RBAC (role-based access control); values never appear in logs, Events, status or metrics.
   - Every release candidate is tested on a real cluster.
   - Written in Rust.
8. **Footer.** Links to the documentation (GitHub `docs/`), the Artifact Hub package, the changelog, the security policy, and the author (Grzegorz Dzięgielewski, linking to the GitHub profile `jabbas`).

Code blocks are syntax-highlighted with CSS only (pre-rendered spans, no highlighter library).

Content duplication with the README is accepted for items 1, 4 and 5 and kept small. Rule: when the README's wording of these items changes, the same pull request MUST update `site/index.html`. There is no automated check — the requirement does not justify a lasting component.

## 5. Visual direction **[architect; built by the frontend agent]**

**[owner]** The visual design was made in Google Stitch and is the reference for the build: project "Generated Secrets Operator — website", https://stitch.withgoogle.com/projects/16971326538841034893 — screens desktop dark `1e321b30646c4a35aeb7c59ba7fb3049`, desktop light `038259448ce941e1a8957394a5b6ba31`, mobile dark `e3db2225619249f7ac0094be670401f2`, social preview `f65bebf2b0db42adb0b11279f122a27a` (ignore the discarded mobile screen `236dd5c060954ba4aa41dce988380eb2`). Palette: dark background `#101413`, surface `#161C1A`, code `#0B0F0E`, border `#26302D`, text `#E6EBE9`, muted `#9AA5A1`, accent `#4EC9A0`; light background `#FBFCFB`, surface `#F2F5F4`, border `#DDE3E1`, text `#15201C`, muted `#56625E`, accent `#0F7A5A` (the demo terminal stays dark in the light theme). 6 px radius, 1 px borders, no shadows; wordmark `$_ Generated Secrets Operator` with `$_` in the accent colour. Stitch's HTML uses the Tailwind CDN, Google Fonts and Material Symbols: the build MUST replace them with one hand-written CSS file, the system font stacks below and inline SVG or text characters (▶ ⧉ →). Stitch's placeholder links and the "1:20" duration caption are not part of the design.

- **Terminal-native and quiet:** dark and light themes via `prefers-color-scheme`, a monospace accent for code and the wordmark, one accent colour, generous whitespace. It should look like a well-made tool page (for example the tone of kind.sigs.k8s.io or ripgrep's README), not a startup landing page.
- **Responsive** down to phone width; the player scales with `fit: "width"`.
- **System font stack** — no web fonts, no third-party requests.
- **Social preview image:** `og:image`, 1200×630 PNG (`site/og.png`) showing the wordmark, the one sentence and a cropped terminal frame. It is also used as the GitHub repository social preview; the owner uploads it in repository settings (the GitHub API cannot set it).
- **Meta tags:** `title`, `description`, `og:*`, `twitter:card` = `summary_large_image`, canonical URL.
- **Accessibility:** semantic HTML; sufficient contrast in both themes; the demo is never the only carrier of information (the manifest and commands are text).

## 6. Domain and DNS — actions, in this order

Source: GitHub Docs, "Verifying your custom domain" and "Managing a custom domain".

1. **Owner:** verify `jabbas.eu` at account level — GitHub profile Settings → Pages → Add a domain → create the TXT record `_github-pages-challenge-jabbas.jabbas.eu` at the DNS provider and keep it permanently. Reason: after verification only the owner's repositories can publish on `jabbas.eu` and its immediate subdomains (protection against domain takeover).
2. **Agent:** enable Pages with source "GitHub Actions" and set the custom domain `generatedsecrets.jabbas.eu` (`gh api`).
3. **Owner:** create the DNS record `generatedsecrets.jabbas.eu CNAME jabbas.github.io` (no wildcard records). If the zone has CAA (Certification Authority Authorization) records, they MUST allow `letsencrypt.org`.
4. **Agent:** once the certificate is issued (minutes, up to 24 hours), enforce HTTPS and set the repository homepage URL to `https://generatedsecrets.jabbas.eu`.

## 7. Sequencing

**[architect]** The README is the source of the shared text (§4 items 1, 3, 4, 5 and the full comparison the site links to), so the site is a port of the finished README, not a second author of the copy.

1. Build after contributor-readiness pull request B (the README rewrite, which itself comes after 0.2.0 final) is merged. Pull request A's recording exists by then.
2. Review on the pull request branch, locally only (open `site/index.html` through a local static file server with `docs/assets/demo.cast` copied next to it). Pages is not enabled yet, so nothing is public.
3. Before merging, run the "try it" commands verbatim on a fresh kind cluster; 0.2.0 final exists by then, so the unpinned `helm install` resolves.
4. Merge, then do §6 in order: domain verification, enable Pages with the custom domain, DNS record, HTTPS. The site is first public at `https://generatedsecrets.jabbas.eu`; there is no review step on the default `github.io` URL (once the custom domain is set, that URL redirects to it).
5. Footer link to the Artifact Hub package: included only if the listing (campaign step 3) already exists; otherwise omitted and added in a later small pull request.
6. Then set the repository homepage URL and link the site from the README.

## 8. Done criteria

- `https://generatedsecrets.jabbas.eu` serves the page over HTTPS.
- The player plays `docs/assets/demo.cast`.
- The "try it" commands work verbatim on a fresh kind cluster.
- No third-party requests (checked in the browser developer tools).
- Lighthouse accessibility score ≥ 95 in both themes.
- `og:image` renders in a link preview.
- The repository homepage URL and social preview are set.

## 9. Left to the builder (frontend agent)

- Exact layout, colours and typography within §5.
- Copy-button implementation.
- Wordmark styling.
- `og.png` composition.
