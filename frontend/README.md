# Frontend

This frontend is a Vite + Vue app that is served in two different ways depending on build mode:

- in development, the Rust binary proxies non-API requests to the Vite dev server on `127.0.0.1:5173`
- in release builds, Axum serves the compiled files from `frontend/dist` or from an extracted `frontend.tar.gz`

## Commands

Install dependencies:

```sh
pnpm install
```

Start the Vite dev server manually:

```sh
pnpm dev
```

Build production assets:

```sh
pnpm build
```

Lint and format:

```sh
pnpm lint
pnpm format
```

## Notes

- You usually do not need to run `pnpm dev` yourself when using `cargo run`; the Rust binary will spawn Vite if it is not already running.
- `pnpm build` produces the files that get packaged into `frontend.tar.gz` during releases.
- Vue Router uses history mode, so production serving must fall back to `index.html`. The Axum server is already configured for that.
- The starter app includes `/`, `/another-page`, and a client-side not-found page to demonstrate SPA navigation and refresh behavior.
- `frontend/public/favicon.ico`, `frontend/public/favicon-32x32.png`, and `frontend/public/logo.png` are still placeholder branding and should be replaced in real projects.
