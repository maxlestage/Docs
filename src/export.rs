//! `docs export <dossier>` : tout le site en fichiers statiques. Chaque page vient du
//! serveur lui-même, par le client de test de Paquets (`app.test()`) : les deux sont
//! identiques.

use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::routes::{Route, base, set_base};
use crate::server::{Site, app, site_dir};

/// Écrit le site dans `out`, les liens derrière `base_path` (`""` ou `/Docs`), les
/// adresses absolues sur `site_url`. Renvoie le nombre de fichiers écrits.
pub async fn export(out: &Path, base_path: &str, site_url: Option<String>) -> io::Result<usize> {
    set_base(base_path);
    let base = base();
    let out = std::path::absolute(out)?;
    let dir = std::path::absolute(site_dir())?;
    std::env::set_current_dir(&dir)?;
    let site_url = site_url.filter(|u| !u.trim().is_empty()).unwrap_or_else(|| {
        if base.is_empty() {
            "http://localhost:3000".to_owned()
        } else {
            format!("https://maxlestage.github.io{base}")
        }
    });
    let mut app = app(Arc::new(Site::load(Some(site_url))));
    // Des fichiers statiques n'ont pas de serveur à protéger, et l'export demande toutes
    // les pages d'un coup (la limite de débit l'arrêterait).
    app.security(None);
    let client = app.test().await;

    let mut written = 0;
    let mut write = |path: &str, bytes: &[u8]| -> io::Result<()> {
        let file = out.join(path.trim_start_matches('/'));
        if let Some(parent) = file.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(file, bytes)?;
        written += 1;
        Ok(())
    };
    let get = |path: String, expected: u16| {
        let client = &client;
        async move {
            let res = client.get(&path).await;
            if res.status().as_u16() != expected {
                return Err(io::Error::other(format!(
                    "GET {path} : {} au lieu de {expected}",
                    res.status()
                )));
            }
            Ok(res.bytes().to_vec())
        }
    };

    for route in Route::all() {
        let html = String::from_utf8_lossy(&get(route.path(), 200).await?).into_owned();
        write(
            &format!("{}index.html", route.path()),
            with_base(&html, base).as_bytes(),
        )?;
    }
    let missing = String::from_utf8_lossy(&get("/404/".into(), 404).await?).into_owned();
    write("404.html", with_base(&missing, base).as_bytes())?;
    for file in ["/search.json", "/sitemap.xml", "/robots.txt"] {
        write(file, &get(file.to_owned(), 200).await?)?;
    }
    for file in ["/manifest.webmanifest", "/sw.js"] {
        let body = String::from_utf8_lossy(&get(file.to_owned(), 200).await?).into_owned();
        write(file, with_base_everywhere(&body, base).as_bytes())?;
    }
    for icon in [
        "/paquets/icon-192.png",
        "/paquets/icon-512.png",
        "/paquets/icon-maskable-512.png",
        "/paquets/apple-touch-icon.png",
    ] {
        write(icon, &get(icon.to_owned(), 200).await?)?;
    }
    let public = dir.join("public");
    for file in files(&public) {
        let relative = file.strip_prefix(&public).map_err(io::Error::other)?;
        write(&relative.to_string_lossy().replace('\\', "/"), &std::fs::read(&file)?)?;
    }
    // Les fichiers sont servis tels quels : pas de Jekyll sur GitHub Pages.
    write(".nojekyll", b"")?;
    Ok(written)
}

/// Les adresses que Paquets écrit lui-même (manifeste, service worker, icônes) ne
/// connaissent pas le chemin de base : elles le reçoivent ici.
pub fn with_base(html: &str, base: &str) -> String {
    if base.is_empty() {
        return html.to_owned();
    }
    html.replace("\"/paquets/", &format!("\"{base}/paquets/"))
        .replace("\"/manifest.webmanifest\"", &format!("\"{base}/manifest.webmanifest\""))
        .replace("\"/sw.js\"", &format!("\"{base}/sw.js\""))
}

/// Chaque adresse absolue (`"/…"`) d'un fichier JSON ou JavaScript, derrière `base`.
pub fn with_base_everywhere(text: &str, base: &str) -> String {
    if base.is_empty() {
        return text.to_owned();
    }
    text.replace("\"/", &format!("\"{base}/"))
}

fn files(dir: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else {
        return found;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            found.extend(files(&path));
        } else {
            found.push(path);
        }
    }
    found.sort();
    found
}
