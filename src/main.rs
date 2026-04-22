pub mod assets;
pub mod server;

#[cfg(not(debug_assertions))]
const APP_NAME: &str = env!("CARGO_PKG_NAME");
#[cfg(not(debug_assertions))]
const VERSION: &str = env!("CARGO_PKG_VERSION");
#[cfg(not(debug_assertions))]
const COMPILED_GH_REPO: &str = match option_env!("GH_REPO") {
    Some(repo) => repo,
    None => "",
};

#[cfg(not(debug_assertions))]
fn release_repo() -> Option<String> {
    std::env::var("GH_REPO")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .or_else(|| (!COMPILED_GH_REPO.trim().is_empty()).then(|| COMPILED_GH_REPO.to_string()))
}

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    let mut vite = {
        let frontend_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("frontend");
        assets::spawn_vite(&frontend_dir)
    };

    let asset_dir = {
        #[cfg(debug_assertions)]
        {
            std::path::PathBuf::new()
        }

        #[cfg(not(debug_assertions))]
        {
            if let Some(dist_dir) = assets::local_dist_dir() {
                dist_dir
            } else {
                let dir = assets::asset_dir(APP_NAME);
                if !assets::assets_present(&dir) {
                    let Some(repo) = release_repo() else {
                        panic!(
                            "Release assets are unavailable locally. Build with GH_REPO=<owner/repo> or set it at runtime so the binary can download frontend.tar.gz."
                        );
                    };
                    let tag = format!("v{VERSION}");
                    assets::bootstrap_assets(&repo, &tag, &dir).await;
                }
                dir
            }
        }
    };

    let app = server::app(asset_dir);
    server::start(app).await;

    #[cfg(debug_assertions)]
    if let Some(child) = vite.as_mut() {
        assets::stop_vite(child);
    }
}
