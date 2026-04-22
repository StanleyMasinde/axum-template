use std::{
    fs, io,
    path::{Path, PathBuf},
};

#[cfg(debug_assertions)]
use std::{
    net::{SocketAddr, TcpStream},
    process::Child,
    thread,
    time::{Duration, Instant},
};

pub fn asset_dir(app_name: &str) -> PathBuf {
    let system = Path::new("/var/www").join(app_name);
    if is_writable_or_creatable(&system) {
        return system;
    }

    let base = dirs::data_local_dir().expect("Cannot determine local data dir");

    base.join(app_name).join("assets")
}

/// Returns true if assets already exist and look valid.
pub fn assets_present(dir: &Path) -> bool {
    dir.join("index.html").exists()
}

#[cfg(not(debug_assertions))]
pub fn local_dist_dir() -> Option<PathBuf> {
    let dist_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("frontend/dist");
    assets_present(&dist_dir).then_some(dist_dir)
}

/// Download and extract the frontend assets from a GH Release tarball.
/// `tag`  — e.g. "v1.2.0"
/// `repo` — e.g. "you/yourrepo"
pub async fn bootstrap_assets(repo: &str, tag: &str, dest: &Path) {
    let url = format!("https://github.com/{repo}/releases/download/{tag}/frontend.tar.gz");

    eprintln!("Downloading frontend assets from {url} ...");

    let bytes = reqwest::get(&url)
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .bytes()
        .await
        .unwrap();

    eprintln!("Extracting to {} ...", dest.display());
    fs::create_dir_all(dest).unwrap();
    extract_tar_gz(&bytes, dest).unwrap();

    eprintln!("Assets ready.");
}

fn extract_tar_gz(data: &[u8], dest: &Path) -> io::Result<()> {
    use flate2::read::GzDecoder;
    use tar::Archive;

    let gz = GzDecoder::new(data);
    let mut archive = Archive::new(gz);
    archive.unpack(dest)?;
    Ok(())
}

fn is_writable_or_creatable(path: &Path) -> bool {
    if path.exists() {
        // Try creating a temp file to confirm write access
        let probe = path.join(".write_probe");
        if fs::File::create(&probe).is_ok() {
            let _ = fs::remove_file(probe);
            return true;
        }
        return false;
    }
    // Try creating the dir
    fs::create_dir_all(path).is_ok()
}

// ── Dev mode ────────────────────────────────────────────────────────────────

/// Spawns the Vite dev server as a child process when one isn't already running.
#[cfg(debug_assertions)]
pub fn spawn_vite(frontend_dir: &Path) -> Option<Child> {
    use std::process::{Command, Stdio};

    let addr = SocketAddr::from(([127, 0, 0, 1], 5173));
    if vite_is_ready(addr) {
        return None;
    }

    let mut command = Command::new(vite_bin(frontend_dir));
    let child = command
        .current_dir(frontend_dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to spawn Vite dev server");

    wait_for_vite(addr);
    Some(child)
}

#[cfg(debug_assertions)]
pub fn stop_vite(child: &mut Child) {
    if child.try_wait().ok().flatten().is_some() {
        return;
    }

    let deadline = Instant::now() + Duration::from_secs(2);
    while Instant::now() < deadline {
        if child.try_wait().ok().flatten().is_some() {
            return;
        }
        thread::sleep(Duration::from_millis(100));
    }

    let _ = child.kill();
    let _ = child.wait();
}

#[cfg(debug_assertions)]
fn vite_bin(frontend_dir: &Path) -> PathBuf {
    let vite = frontend_dir
        .join("node_modules")
        .join(".bin")
        .join(if cfg!(windows) { "vite.cmd" } else { "vite" });

    if vite.exists() {
        vite
    } else {
        panic!(
            "Vite binary not found at {}. Run `pnpm install` in frontend/ first.",
            vite.display()
        );
    }
}

#[cfg(debug_assertions)]
fn vite_is_ready(addr: SocketAddr) -> bool {
    TcpStream::connect_timeout(&addr, Duration::from_millis(250)).is_ok()
}

#[cfg(debug_assertions)]
fn wait_for_vite(addr: SocketAddr) {
    let deadline = Instant::now() + Duration::from_secs(10);
    while Instant::now() < deadline {
        if vite_is_ready(addr) {
            return;
        }
        thread::sleep(Duration::from_millis(100));
    }

    panic!("Timed out waiting for Vite dev server on http://{addr}");
}
