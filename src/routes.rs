//! Les adresses du site : `/`, `/comparer/`, `/{parcours}/`, `/{parcours}/{page}/`,
//! derrière un chemin de base facultatif (`/Docs` sur GitHub Pages).

use std::sync::OnceLock;

use crate::catalog::{self, TRACKS, Track};

static BASE: OnceLock<String> = OnceLock::new();

/// Fixe le chemin de base, une fois, avant de rendre quoi que ce soit : `""` (par
/// défaut) ou `/Docs`.
pub fn set_base(base: &str) {
    let _ = BASE.set(normalize_base(base));
}

/// `""`, ou `/quelque-chose` sans barre finale.
pub fn normalize_base(base: &str) -> String {
    match base.trim().trim_end_matches('/') {
        "" => String::new(),
        b if b.starts_with('/') => b.to_owned(),
        b => format!("/{b}"),
    }
}

/// Le chemin de base, sans barre finale (vide par défaut).
pub fn base() -> &'static str {
    BASE.get().map_or("", String::as_str)
}

/// Un lien vers `path` (qui commence par `/`), derrière le chemin de base.
pub fn link(path: &str) -> String {
    format!("{}{path}", base())
}

#[derive(Clone, Copy)]
pub enum Route {
    Home,
    Compare,
    Track(&'static Track),
    /// Une page d'un parcours, par sa position.
    Doc(&'static Track, usize),
    NotFound,
}

impl PartialEq for Route {
    fn eq(&self, other: &Route) -> bool {
        self.path() == other.path()
    }
}

impl std::fmt::Debug for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Route({})", self.path())
    }
}

impl Route {
    /// La route d'un chemin (sans le chemin de base).
    pub fn parse(path: &str) -> Route {
        let parts: Vec<&str> = path.split('/').filter(|p| !p.is_empty()).collect();
        match parts[..] {
            [] => Route::Home,
            ["comparer"] => Route::Compare,
            [track] => catalog::track(track).map_or(Route::NotFound, Route::Track),
            [track, page] => catalog::find(track, page).map_or(Route::NotFound, |(t, i)| Route::Doc(t, i)),
            _ => Route::NotFound,
        }
    }

    /// Le chemin, sans le chemin de base : `/rust/fonctions/`.
    pub fn path(&self) -> String {
        match self {
            Route::Home => "/".to_owned(),
            Route::Compare => "/comparer/".to_owned(),
            Route::Track(t) => format!("/{}/", t.slug),
            Route::Doc(t, i) => format!("/{}/{}/", t.slug, t.pages[*i].slug),
            Route::NotFound => "/404/".to_owned(),
        }
    }

    /// Le lien, avec le chemin de base.
    pub fn href(&self) -> String {
        link(&self.path())
    }

    /// Le parcours de la page, s'il y en a un.
    pub fn track(&self) -> Option<&'static Track> {
        match self {
            Route::Track(t) | Route::Doc(t, _) => Some(t),
            _ => None,
        }
    }

    /// Toutes les pages du site, dans l'ordre du plan.
    pub fn all() -> Vec<Route> {
        let mut routes = vec![Route::Home, Route::Compare];
        for track in &TRACKS {
            routes.push(Route::Track(track));
            routes.extend((0..track.pages.len()).map(|i| Route::Doc(track, i)));
        }
        routes
    }
}

/// Le lien vers une page d'un parcours, avec une ancre facultative.
pub fn doc_href(track: &str, page: &str, anchor: Option<&str>) -> String {
    let mut href = link(&format!("/{track}/{page}/"));
    if let Some(anchor) = anchor.filter(|a| !a.is_empty()) {
        href.push('#');
        href.push_str(anchor);
    }
    href
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_prints_every_route() {
        for route in Route::all() {
            assert_eq!(Route::parse(&route.path()), route);
        }
        assert_eq!(Route::parse("/rust/possession").path(), "/rust/possession/");
        assert_eq!(Route::parse("/ruby/possession/").path(), "/404/");
        assert_eq!(Route::parse("/a/b/c").path(), "/404/");
        assert_eq!(normalize_base("Docs/"), "/Docs");
        assert_eq!(normalize_base(" "), "");
    }
}
