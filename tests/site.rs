//! Le site entier, par le client de test de Paquets : chaque page, chaque lien entre
//! les pages (et son ancre), l'index de recherche, le plan du site.

use std::sync::Arc;

use docs::catalog::languages;
use docs::i18n::Lang;
use docs::library::{self, Doc};
use docs::routes::Route;
use docs::server::{Site, app};

/// `en/rust/fonctions`, ou `rust/fonctions` en français.
fn at(doc: &Doc) -> String {
    let lang = if doc.lang == Lang::Fr {
        String::new()
    } else {
        format!("{}/", doc.lang.code())
    };
    format!("{lang}{}/{}", doc.track.slug, doc.slug)
}

/// Les trois sections obligatoires d'un chapitre, dans la langue de son texte.
fn plan(doc: &Doc) -> [&'static str; 3] {
    match (doc.translated, doc.lang) {
        (true, Lang::En) => ["Common mistakes", "Test yourself", "Summary"],
        (true, Lang::Es) => ["Errores frecuentes", "Ponte a prueba", "En resumen"],
        _ => ["Erreurs fréquentes", "Testez-vous", "En résumé"],
    }
}

async fn client() -> paquets::test::TestClient {
    let mut app = app(Arc::new(Site::load(Some("https://docs.example".into()))));
    app.security(None);
    app.test().await
}

#[tokio::test]
async fn every_page_answers() {
    let client = client().await;
    for lang in Lang::ALL {
        for route in Route::all() {
            let path = route.path(lang);
            let res = client.get(&path).await;
            assert_eq!(res.status(), 200, "{path}");
            let html = res.text();
            assert!(html.contains(&format!(r#"<html lang="{}""#, lang.code())), "{path}");
            assert!(
                html.contains(&format!(r#"<link rel="canonical" href="https://docs.example{path}""#)),
                "{path}"
            );
            for other in Lang::ALL {
                assert!(
                    html.contains(&format!(
                        r#"<link rel="alternate" hreflang="{}" href="https://docs.example{}"/>"#,
                        other.code(),
                        route.path(other)
                    )),
                    "{path}"
                );
            }
            assert!(html.contains("<title>"), "{path}");
            assert!(html.contains("/site.css?v="));
            assert!(!html.contains("<script>alert"), "{path}");
        }
    }
    let home = client.get("/en/").await.text();
    assert!(home.contains("Learn to program, "), "{home}");
    assert!(home.contains(r#"href="/en/compare/""#));
    assert!(home.contains(r#"hreflang="es" lang="es""#), "{home}");
    let untranslated = client.get("/en/rust/unsafe/").await.text();
    assert!(untranslated.contains("not translated into English yet"));
    let missing = client.get("/ruby/nulle-part/").await;
    assert_eq!(missing.status(), 404);
    assert!(missing.text().contains("noindex"));
    let missing = client.get("/es/ruby/nulle-part/").await;
    assert_eq!(missing.status(), 404);
    assert!(missing.text().contains("Esta página no existe"));
    let redirect = client.get("/rust").await;
    assert_eq!(redirect.status(), 301);
}

#[tokio::test]
async fn pages_have_a_strict_policy() {
    let client = client().await;
    let res = client.get("/rust/possession/").await;
    let csp = res.header("content-security-policy").expect("CSP");
    assert!(csp.contains("default-src 'self'"));
    assert!(csp.contains("script-src 'self'"));
}

#[test]
fn every_page_has_a_title_a_summary_and_sections() {
    for doc in library::all() {
        let at = at(doc);
        assert!(!doc.title.is_empty() && doc.title != doc.slug, "{at}: no # title");
        assert!(doc.description.len() > 30, "{at}: no summary paragraph");
        assert!(doc.toc.len() >= 3, "{at}: fewer than 3 sections");
        if doc.track.language {
            assert!(doc.html.contains(r#"<div class="code""#), "{at}: no code");
        }
    }
}

#[test]
fn every_link_leads_to_a_page_and_an_anchor() {
    let mut count = 0;
    for doc in library::all() {
        for (track, page, anchor) in &doc.links {
            let at = format!("{} → {track}/{page}#{anchor}", at(doc));
            let target = library::get(doc.lang, track, page).unwrap_or_else(|| panic!("{at}: no such page"));
            if !anchor.is_empty() {
                assert!(target.toc.iter().any(|h| &h.id == anchor), "{at}: no such anchor");
            }
            count += 1;
        }
    }
    assert!(count > 10, "the pages link to each other");
}

#[test]
fn the_languages_share_their_core_chapters() {
    for slug in [
        "introduction",
        "premiers-pas",
        "conditions-boucles",
        "fonctions",
        "texte",
        "collections",
        "types",
        "erreurs",
        "modules",
        "fichiers",
        "formats",
        "tests",
        "projet",
        "projets",
        "exercices",
        "aide-memoire",
    ] {
        for track in languages() {
            assert!(
                library::get(Lang::Fr, track.slug, slug).is_some(),
                "{}/{slug}",
                track.slug
            );
        }
    }
}

#[tokio::test]
async fn search_index_and_sitemap() {
    let client = client().await;
    for lang in Lang::ALL {
        let index = client.get(&format!("{}/search.json", lang.prefix())).await;
        assert_eq!(index.status(), 200);
        let entries: Vec<paquets::serde_json::Value> = paquets::serde_json::from_str(&index.text()).expect("JSON");
        let pages = entries.iter().filter(|e| e["k"] == "page").count();
        assert_eq!(pages, library::in_lang(lang).count() + 1);
        let possession = format!("{}/rust/possession/", lang.prefix());
        assert!(
            entries
                .iter()
                .any(|e| e["u"] == possession.as_str() && e["s"] == "rust")
        );
    }
    let sitemap = client.get("/sitemap.xml").await.text();
    assert_eq!(sitemap.matches("<url>").count(), Route::all().len() * 3);
    assert!(sitemap.contains("<loc>https://docs.example/en/rythme/types/</loc>"));
    assert!(sitemap.contains("<loc>https://docs.example/rythme/types/</loc>"));
    assert!(
        client
            .get("/robots.txt")
            .await
            .text()
            .contains("Sitemap: https://docs.example/sitemap.xml")
    );
}

#[test]
fn every_chapter_ends_with_a_summary() {
    for doc in library::all() {
        if [
            "aide-memoire",
            "exercices",
            "exercices-faciles",
            "exercices-2",
            "glossaire",
        ]
        .contains(&doc.slug)
        {
            continue;
        }
        let last = doc.toc.iter().rfind(|h| h.level == 2).map(|h| h.text.as_str());
        assert_eq!(last, Some(plan(doc)[2]), "{}", at(doc));
    }
}

#[test]
fn exercises_have_folded_solutions() {
    for track in languages() {
        let doc = library::get(Lang::Fr, track.slug, "exercices").expect("exercices");
        let solutions = doc.html.matches(r#"<details class="solution">"#).count();
        assert!(solutions >= 10, "{}: {solutions} solutions", track.slug);
    }
}

/// Le plan commun des chapitres (voir `content/README.md`).
#[test]
fn every_chapter_follows_the_plan() {
    let mut problems = Vec::new();
    for doc in library::all() {
        if [
            "aide-memoire",
            "exercices",
            "exercices-faciles",
            "exercices-2",
            "glossaire",
        ]
        .contains(&doc.slug)
        {
            continue;
        }
        let at = at(doc);
        if !doc.html.contains(r#"<aside class="callout callout--goals">"#) {
            problems.push(format!("{at}: no « Objectifs » callout"));
        }
        let h2: Vec<&str> = doc
            .toc
            .iter()
            .filter(|h| h.level == 2)
            .map(|h| h.text.as_str())
            .collect();
        for section in plan(doc) {
            if !h2.contains(&section) {
                problems.push(format!("{at}: no « {section} » section"));
            }
        }
        let answers = doc.html.matches("solution--answer").count();
        if answers < 3 {
            problems.push(format!("{at}: {answers} answers in « Testez-vous »"));
        }
    }
    assert!(problems.is_empty(), "\n{}", problems.join("\n"));
}
