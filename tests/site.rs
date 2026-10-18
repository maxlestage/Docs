//! Le site entier, par le client de test de Paquets : chaque page, chaque lien entre
//! les pages (et son ancre), l'index de recherche, le plan du site.

use std::sync::Arc;

use docs::catalog::languages;
use docs::library;
use docs::routes::Route;
use docs::server::{Site, app};

async fn client() -> paquets::test::TestClient {
    let mut app = app(Arc::new(Site::load(Some("https://docs.example".into()))));
    app.security(None);
    app.test().await
}

#[tokio::test]
async fn every_page_answers() {
    let client = client().await;
    for route in Route::all() {
        let res = client.get(&route.path()).await;
        assert_eq!(res.status(), 200, "{}", route.path());
        let html = res.text();
        assert!(html.contains(r#"<html lang="fr""#), "{}", route.path());
        assert!(
            html.contains(&format!(
                r#"<link rel="canonical" href="https://docs.example{}""#,
                route.path()
            )),
            "{}",
            route.path()
        );
        assert!(html.contains("<title>"), "{}", route.path());
        assert!(html.contains("/site.css?v="));
        assert!(!html.contains("<script>alert"), "{}", route.path());
    }
    let missing = client.get("/ruby/nulle-part/").await;
    assert_eq!(missing.status(), 404);
    assert!(missing.text().contains("noindex"));
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
        let at = format!("{}/{}", doc.track.slug, doc.slug);
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
            let at = format!("{}/{} → {track}/{page}#{anchor}", doc.track.slug, doc.slug);
            let target = library::get(track, page).unwrap_or_else(|| panic!("{at}: no such page"));
            if !anchor.is_empty() {
                assert!(target.toc.iter().any(|h| &h.id == anchor), "{at}: no such anchor");
            }
            count += 1;
        }
    }
    assert!(count > 10, "the pages link to each other");
}

#[test]
fn the_three_tracks_share_their_chapters() {
    for slug in [
        "introduction",
        "premiers-pas",
        "conditions-boucles",
        "fonctions",
        "texte",
        "collections",
        "types",
        "erreurs",
        "fichiers",
        "projet",
        "exercices",
        "aide-memoire",
    ] {
        for track in languages() {
            assert!(library::get(track.slug, slug).is_some(), "{}/{slug}", track.slug);
        }
    }
}

#[tokio::test]
async fn search_index_and_sitemap() {
    let client = client().await;
    let index = client.get("/search.json").await;
    assert_eq!(index.status(), 200);
    let entries: Vec<paquets::serde_json::Value> = paquets::serde_json::from_str(&index.text()).expect("JSON");
    let pages = entries.iter().filter(|e| e["k"] == "page").count();
    assert_eq!(pages, library::all().len() + 1);
    assert!(entries.iter().any(|e| e["u"] == "/rust/possession/"));
    let sitemap = client.get("/sitemap.xml").await.text();
    assert_eq!(sitemap.matches("<url>").count(), Route::all().len());
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
        if ["aide-memoire", "exercices", "glossaire"].contains(&doc.slug) {
            continue;
        }
        let last = doc.toc.iter().rfind(|h| h.level == 2).map(|h| h.text.as_str());
        assert_eq!(last, Some("En résumé"), "{}/{}", doc.track.slug, doc.slug);
    }
}

#[test]
fn exercises_have_folded_solutions() {
    for track in languages() {
        let doc = library::get(track.slug, "exercices").expect("exercices");
        let solutions = doc.html.matches(r#"<details class="solution">"#).count();
        assert!(solutions >= 10, "{}: {solutions} solutions", track.slug);
    }
}
