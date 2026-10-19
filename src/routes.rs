//! Les adresses du site : `/`, `/comparer/`, `/{parcours}/`, `/{parcours}/{page}/`,
//! en français ; les mêmes derrière `/en` et `/es` (avec `/en/compare/` et
//! `/es/comparar/`) ; le tout derrière un chemin de base facultatif (`/Docs` sur GitHub
//! Pages).

use std::sync::OnceLock;

use crate::catalog::{self, TRACKS, Track};
use crate::i18n::Lang;

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

/// Un lien vers `path` dans la langue `lang` : `/en/…`.
pub fn lang_link(lang: Lang, path: &str) -> String {
    link(&format!("{}{path}", lang.prefix()))
}

/// Le nom de la page Comparer dans chaque langue.
fn compare_slug(lang: Lang) -> &'static str {
    lang.pick("comparer", "compare", "comparar")
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
        self.path(Lang::Fr) == other.path(Lang::Fr)
    }
}

impl std::fmt::Debug for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Route({})", self.path(Lang::Fr))
    }
}

impl Route {
    /// La langue et la route d'un chemin (sans le chemin de base).
    pub fn parse(path: &str) -> (Lang, Route) {
        let mut parts: Vec<&str> = path.split('/').filter(|p| !p.is_empty()).collect();
        let lang = match parts.first().and_then(|p| Lang::from_prefix(p)) {
            Some(lang) => {
                parts.remove(0);
                lang
            }
            None => Lang::Fr,
        };
        let route = match parts[..] {
            [] => Route::Home,
            [c] if c == compare_slug(lang) => Route::Compare,
            [track] => catalog::track(track).map_or(Route::NotFound, Route::Track),
            [track, page] => catalog::find(track, page).map_or(Route::NotFound, |(t, i)| Route::Doc(t, i)),
            _ => Route::NotFound,
        };
        (lang, route)
    }

    /// Le chemin, sans le chemin de base : `/rust/fonctions/`, `/en/rust/fonctions/`.
    pub fn path(&self, lang: Lang) -> String {
        let path = match self {
            Route::Home => "/".to_owned(),
            Route::Compare => format!("/{}/", compare_slug(lang)),
            Route::Track(t) => format!("/{}/", t.slug),
            Route::Doc(t, i) => format!("/{}/{}/", t.slug, t.pages[*i].slug),
            Route::NotFound => "/404/".to_owned(),
        };
        format!("{}{path}", lang.prefix())
    }

    /// Le lien, avec le chemin de base.
    pub fn href(&self, lang: Lang) -> String {
        link(&self.path(lang))
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
pub fn doc_href(lang: Lang, track: &str, page: &str, anchor: Option<&str>) -> String {
    let mut href = lang_link(lang, &format!("/{track}/{page}/"));
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
            for lang in Lang::ALL {
                assert_eq!(Route::parse(&route.path(lang)), (lang, route));
            }
        }
        let path = |p: &str| {
            let (lang, route) = Route::parse(p);
            route.path(lang)
        };
        assert_eq!(path("/rust/possession"), "/rust/possession/");
        assert_eq!(path("/en/rust/possession"), "/en/rust/possession/");
        assert_eq!(path("/es/comparar/"), "/es/comparar/");
        assert_eq!(path("/en/comparer/"), "/en/404/");
        assert_eq!(path("/ruby/possession/"), "/404/");
        assert_eq!(path("/a/b/c"), "/404/");
        assert_eq!(normalize_base("Docs/"), "/Docs");
        assert_eq!(normalize_base(" "), "");
    }
}
