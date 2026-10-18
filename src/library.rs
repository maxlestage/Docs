//! Toutes les pages, rendues une fois au démarrage, et l'index de recherche.

use std::sync::LazyLock;

use crate::catalog::{TRACKS, Track};
use crate::markdown::{Heading, render};
use crate::routes::{doc_href, link};

/// Une page rendue.
pub struct Doc {
    pub track: &'static Track,
    pub index: usize,
    pub slug: &'static str,
    pub title: String,
    pub description: String,
    pub html: String,
    pub toc: Vec<Heading>,
    pub text: String,
    /// Le temps de lecture, en minutes.
    pub minutes: usize,
    pub links: Vec<(String, String, String)>,
}

static DOCS: LazyLock<Vec<Doc>> = LazyLock::new(|| {
    let mut docs = Vec::new();
    for track in &TRACKS {
        for (index, page) in track.pages.iter().enumerate() {
            let r = render(page.markdown, track.slug);
            docs.push(Doc {
                track,
                index,
                slug: page.slug,
                title: if r.title.is_empty() {
                    page.slug.to_owned()
                } else {
                    r.title
                },
                description: r.description,
                html: r.html,
                toc: r.toc,
                text: r.text,
                // 200 mots par minute : on lit le code plus lentement que le texte.
                minutes: r.words.div_ceil(200).max(1),
                links: r.links,
            });
        }
    }
    docs
});

/// Toutes les pages, parcours après parcours.
pub fn all() -> &'static [Doc] {
    &DOCS
}

/// La page `slug` du parcours `track`.
pub fn get(track: &str, slug: &str) -> Option<&'static Doc> {
    DOCS.iter().find(|d| d.track.slug == track && d.slug == slug)
}

/// Les pages d'un parcours.
pub fn of(track: &Track) -> impl Iterator<Item = &'static Doc> {
    let slug = track.slug;
    DOCS.iter().filter(move |d| d.track.slug == slug)
}

/// Le temps de lecture d'un parcours entier, en minutes.
pub fn minutes(track: &Track) -> usize {
    of(track).map(|d| d.minutes).sum()
}

fn json_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '<' => out.push_str("\\u003c"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// L'index de recherche, en JSON : une entrée par page (avec son texte) et par
/// section. Le navigateur le télécharge une fois et cherche lui-même.
pub fn search_index() -> String {
    let mut entries = Vec::new();
    for doc in all() {
        let href = doc_href(doc.track.slug, doc.slug, None);
        entries.push(format!(
            r#"{{"t":{},"d":{},"p":{},"u":{},"k":"page","x":{}}}"#,
            json_string(&doc.title),
            json_string(&doc.description),
            json_string(doc.track.name),
            json_string(&href),
            json_string(&doc.text),
        ));
        for h in &doc.toc {
            entries.push(format!(
                r#"{{"t":{},"d":{},"p":{},"u":{},"k":"section"}}"#,
                json_string(&h.text),
                json_string(&doc.title),
                json_string(doc.track.name),
                json_string(&format!("{href}#{}", h.id)),
            ));
        }
    }
    entries.push(format!(
        r#"{{"t":"Comparer les langages","d":"Les mêmes idées dans les sept langages, côte à côte.","p":"Docs","u":{},"k":"page"}}"#,
        json_string(&link("/comparer/"))
    ));
    format!("[{}]", entries.join(",\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_is_escaped() {
        let out = json_string("a\"b\\c\n<");
        assert_eq!(out, "\"a\\\"b\\\\c\\n\\u003c\"");
    }
}
