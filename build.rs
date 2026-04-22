use std::{env, path::Path, process::Command};

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let frontend_dir = Path::new(&manifest_dir).join("frontend");

    println!("cargo:rerun-if-changed=frontend/src");
    println!("cargo:rerun-if-changed=frontend/package.json");
    println!("cargo:rerun-if-changed=frontend/vite.config.ts");

    let profile = env::var("PROFILE").unwrap(); // "debug" or "release"

    if profile == "debug" {
        ensure_deps_installed(&frontend_dir);
        // Dev server is spawned at runtime by the binary, not here.
        // build.rs can't hold a long-running process.
        println!("cargo:rustc-env=APP_ENV=development");
    } else {
        // Release: frontend assets are NOT embedded.
        // They are built separately and uploaded to GH Releases.
        // The binary downloads the matching versioned release asset at first run (see assets.rs).
        println!("cargo:rustc-env=APP_ENV=production");
    }
}

fn ensure_deps_installed(frontend_dir: &Path) {
    if !frontend_dir.join("node_modules").exists() {
        let status = Command::new("pnpm")
            .args(["install"])
            .current_dir(frontend_dir)
            .status()
            .expect("Failed to run pnpm install");

        if !status.success() {
            panic!("pnpm install failed");
        }
    }
}
