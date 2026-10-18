//! Le serveur : une `paquets::App` avec les pages, l'index de recherche, le plan du site
//! pour les moteurs de recherche et l'application installable. Les fichiers statiques
//! (styles, script, logo) viennent de `public/`, que Paquets sert depuis le dossier
//! courant.

use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use paquets::prelude::*;

use crate::pages::{self, Ctx};
use crate::routes::{Route, base, link};

/// Les fichiers de `public/` dont la version est ajoutée à leurs liens (`?v=`).
pub const ASSETS: [&str; 2] = ["site.css", "site.js"];

pub struct Site {
    /// L'adresse publique, avec le chemin de base (`SITE_URL`) ; `None` : celle de
    /// chaque requête.
    pub site_url: Option<String>,
    pub version: String,
}

/// Le dossier du site : `SITE_DIR`, le dossier courant s'il a `public/`, sinon celui du
/// paquet.
pub fn site_dir() -> PathBuf {
    if let Some(dir) = std::env::var_os("SITE_DIR") {
        return std::path::absolute(&dir).unwrap_or_else(|_| PathBuf::from(dir));
    }
    if Path::new("public").is_dir() {
        return PathBuf::from(".");
    }
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// FNV-1a : assez pour distinguer deux versions d'un fichier.
pub fn version(bytes: &[u8]) -> String {
    let h = bytes.iter().fold(0xcbf2_9ce4_8422_2325_u64, |h, b| {
        (h ^ u64::from(*b)).wrapping_mul(0x0100_0000_01b3)
    });
    format!("{h:016x}")[..10].to_owned()
}

impl Site {
    pub fn load(site_url: Option<String>) -> Site {
        let public = site_dir().join("public");
        let mut bytes = Vec::new();
        for file in ASSETS {
            bytes.extend(std::fs::read(public.join(file)).unwrap_or_default());
        }
        let site_url = site_url
            .map(|u| u.trim().trim_end_matches('/').to_owned())
            .filter(|u| !u.is_empty());
        Site {
            site_url,
            version: version(&bytes),
        }
    }

    fn origin(&self, req: &Request) -> String {
        if let Some(url) = &self.site_url {
            return url.clone();
        }
        let header = |name: &str| {
            req.header(name)
                .map(|v| v.split(',').next().unwrap_or("").trim().to_owned())
        };
        let host = header("x-forwarded-host")
            .or_else(|| header("host"))
            .unwrap_or_else(|| "localhost".into());
        if host.is_empty()
            || !host
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | ':'))
        {
            return format!("http://localhost{}", base());
        }
        let proto = if header("x-forwarded-proto").as_deref() == Some("https") {
            "https"
        } else {
            "http"
        };
        format!("{proto}://{host}{}", base())
    }

    fn ctx(&self, req: &Request, route: Route) -> Ctx {
        Ctx {
            route,
            origin: self.origin(req),
            version: self.version.clone(),
        }
    }
}

/// Une page : `/`, `/comparer/`, `/{parcours}/`, `/{parcours}/{page}/`.
fn page(site: &Site, req: &Request) -> Response {
    let path = req.path();
    let route = Route::parse(path);
    if route == Route::NotFound {
        return not_found(site, req);
    }
    if !path.ends_with('/') {
        let query = req.uri().query().map(|q| format!("?{q}")).unwrap_or_default();
        return Redirect::permanent(format!("{}/{query}", link(path))).into_response();
    }
    pages::render(&site.ctx(req, route))
        .into_response()
        .header("cache-control", "no-cache")
}

fn not_found(site: &Site, req: &Request) -> Response {
    pages::not_found(&site.ctx(req, Route::NotFound)).into_response()
}

/// `/search.json` : l'index de la recherche, téléchargé une fois par le navigateur.
fn search(_site: &Site, _req: &Request) -> Response {
    Response::new()
        .header("content-type", "application/json; charset=utf-8")
        .header("cache-control", "public, max-age=300")
        .send(crate::library::search_index())
}

fn sitemap(site: &Site, req: &Request) -> Response {
    let origin = site.origin(req);
    let mut xml = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n",
    );
    for route in Route::all() {
        xml.push_str(&format!("<url><loc>{origin}{}</loc></url>\n", route.path()));
    }
    xml.push_str("</urlset>\n");
    Response::new()
        .header("content-type", "application/xml; charset=utf-8")
        .send(xml)
}

fn robots(site: &Site, req: &Request) -> Response {
    Response::new()
        .header("content-type", "text/plain; charset=utf-8")
        .send(format!(
            "User-agent: *\nAllow: /\n\nSitemap: {}/sitemap.xml\n",
            site.origin(req)
        ))
}

fn route(app: &mut App, path: &str, site: &Arc<Site>, handler: fn(&Site, &Request) -> Response) {
    let site = site.clone();
    app.get(path, move |req: Request| {
        let site = site.clone();
        async move { handler(&site, &req) }
    });
}

/// L'application, prête à lancer ou à tester.
pub fn app(site: Arc<Site>) -> App {
    let mut app = App::new();
    route(&mut app, "/search.json", &site, search);
    route(&mut app, "/sitemap.xml", &site, sitemap);
    route(&mut app, "/robots.txt", &site, robots);
    for path in ["/", "/:a", "/:a/:b"] {
        route(&mut app, path, &site, page);
    }
    // Installable sur téléphone et ordinateur, lisible hors ligne.
    app.pwa(
        Pwa::new("Docs : Ruby, Rust et Rythme")
            .short_name("Docs")
            .description("Des documentations simples pour Ruby, Rust et Rythme.")
            .lang("fr")
            .start_url("/")
            .theme_color("#0e0d12")
            .background_color("#fbfaf7")
            .categories(["education", "developer"]),
    );
    let fallback = site.clone();
    app.fallback(move |req: Request| {
        let site = fallback.clone();
        async move { not_found(&site, &req) }
    });
    app
}

/// `cargo run` : le serveur, sur `PORT` (Heroku) ou 3000.
pub fn run() -> io::Result<()> {
    crate::routes::set_base(&std::env::var("BASE_PATH").unwrap_or_default());
    let dir = site_dir();
    if dir != Path::new(".") {
        std::env::set_current_dir(&dir)?;
    }
    let pages = crate::library::all().len();
    println!("   {pages} pages, prêtes");
    app(Arc::new(Site::load(std::env::var("SITE_URL").ok()))).run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn versions_change_with_the_content() {
        assert_eq!(version(b"a").len(), 10);
        assert_ne!(version(b"a"), version(b"b"));
    }
}
