# Axum + Vue Template

Template for shipping a standalone Rust binary without embedding the frontend into the executable.

The intended model is:

- the Rust app is built as a normal binary
- the frontend is built as static files in `frontend/dist`
- production binaries serve those files from disk
- if the files are not present locally, the binary can download `frontend.tar.gz` from a matching GitHub Release

This is useful for apps that should feel self-contained, but where you do not want to bloat the binary with bundled HTML, JS, and CSS.

## How It Works

### Development

- `cargo run` starts the Axum server on `127.0.0.1:3000`
- the binary spawns Vite on `127.0.0.1:5173` if it is not already running
- `/api/*` is handled by Axum
- everything else is proxied to Vite

The starter frontend currently includes:

- `/` as the landing page
- `/another-page` as a client-side routing demo page
- a Vue Router catch-all 404 page for unknown frontend routes

`build.rs` installs frontend dependencies with `pnpm install` the first time if `frontend/node_modules` is missing.

### Release

Release binaries do not embed frontend assets.

At startup, a release build resolves assets in this order:

1. `frontend/dist` next to the source tree, if it exists
2. the runtime asset directory:
   `/var/www/<app-name>` when writable, otherwise the platform local data directory under `<app-name>/assets`
3. a `frontend.tar.gz` asset downloaded from `https://github.com/<owner>/<repo>/releases/download/v<version>/frontend.tar.gz`

The release binary needs a GitHub repository name to perform step 3. It gets that from:

- `GH_REPO` baked in at compile time, or
- `GH_REPO` set at runtime

The included GitHub Actions workflow sets this automatically for published releases.

## Local Commands

### Run in development

```sh
cargo run
```

### Build frontend assets

```sh
cd frontend
pnpm build
```

### Test a release build locally

Build the frontend first, then run the release binary:

```sh
cd frontend
pnpm build

cd ..
cargo run --release
```

If `frontend/dist` is absent, supply `GH_REPO=<owner/repo>` so the binary knows where to fetch `frontend.tar.gz`.

## Release Flow

Tagging a commit with `v<version>` triggers [.github/workflows/release.yml](./.github/workflows/release.yml). The tag must match `package.version` in `Cargo.toml`.

That workflow:

- builds `frontend/dist`
- packages it as `frontend.tar.gz`
- creates a GitHub Release if needed
- builds release binaries for Linux, macOS Intel, macOS Apple Silicon, and Windows
- uploads both the frontend tarball and platform binaries to the same release
- bakes the current GitHub repository into the binary as `GH_REPO`

Binary archives are uploaded with installer-friendly names:

- `axum_template-linux-x86_64.tar.gz`
- `axum_template-darwin-x86_64.tar.gz`
- `axum_template-darwin-aarch64.tar.gz`
- `axum_template-windows-x86_64.zip`

Typical release steps:

1. Update `Cargo.toml`
2. Commit and push
3. Create tag `v<version>`
4. Push the tag

When that binary version starts and assets are missing on disk, it fetches `frontend.tar.gz` from the matching release tag.

## Install Script

The repo includes [install.sh](./install.sh) for curl-pipe installs:

```sh
curl -fsSL https://raw.githubusercontent.com/StanleyMasinde/axum-template/main/install.sh | sh
```

To install a specific version:

```sh
curl -fsSL https://raw.githubusercontent.com/StanleyMasinde/axum-template/main/install.sh | sh -s -- v0.1.0
```

The script downloads the matching platform archive from GitHub Releases, verifies the SHA256 digest exposed by the GitHub Releases API when available, and installs the binary into `/usr/local/bin` by default.

## Template Readiness Checklist

Before calling this ready for reuse, replace the remaining starter content:

- app name in `Cargo.toml`
- page title in `frontend/index.html`
- starter page copy in `frontend/src/pages/`
- API routes in `src/server/mod.rs`
- favicon and logo assets in `frontend/public/`
- `REPO`, `BIN_NAME`, and `AXUM_TEMPLATE_INSTALL` in `install.sh`
