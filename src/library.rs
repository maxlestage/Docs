//! Toutes les pages, dans les trois langues, rendues une fois au démarrage, et l'index
//! de recherche de chaque langue. Une page pas encore traduite est rendue en français
//! (avec les liens et l'interface de sa langue).

use std::sync::LazyLock;

use crate::catalog::{TRACKS, Track, translation};
use crate::i18n::{Lang, tr};
use crate::markdown::{Heading, render};
use crate::routes::{Route, doc_href};

/// Une page rendue.
pub struct Doc {
    pub lang: Lang,
    /// Écrite dans sa langue (toujours vrai en français).
    pub translated: bool,
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
    for lang in Lang::ALL {
        for track in &TRACKS {
            for (index, page) in track.pages.iter().enumerate() {
                let translated = translation(lang, track.slug, page.slug);
                let r = render(
                    translated.unwrap_or(page.markdown),
                    track.slug,
                    lang,
                    lang == Lang::Fr || translated.is_some(),
                );
                docs.push(Doc {
                    lang,
                    translated: lang == Lang::Fr || translated.is_some(),
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
    }
    docs
});

/// Toutes les pages, langue après langue, parcours après parcours.
pub fn all() -> &'static [Doc] {
    &DOCS
}

/// Les pages d'une langue.
pub fn in_lang(lang: Lang) -> impl Iterator<Item = &'static Doc> {
    DOCS.iter().filter(move |d| d.lang == lang)
}

/// La page `slug` du parcours `track`, en `lang`.
pub fn get(lang: Lang, track: &str, slug: &str) -> Option<&'static Doc> {
    in_lang(lang).find(|d| d.track.slug == track && d.slug == slug)
}

/// Les pages d'un parcours, en `lang`.
pub fn of(lang: Lang, track: &Track) -> impl Iterator<Item = &'static Doc> {
    let slug = track.slug;
    in_lang(lang).filter(move |d| d.track.slug == slug)
}

/// Le temps de lecture d'un parcours entier, en minutes.
pub fn minutes(lang: Lang, track: &Track) -> usize {
    of(lang, track).map(|d| d.minutes).sum()
}

/// Le nom d'un parcours dans une langue (« Débuter », « Start here »…).
pub fn track_name(lang: Lang, track: &Track) -> &'static str {
    crate::i18n::meta(lang, track.slug).map_or(track.name, |m| m.name)
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

/// L'index de recherche d'une langue, en JSON : une entrée par page (avec son texte) et
/// par section ; `p` est le nom du parcours, `s` son adresse. Le navigateur le
/// télécharge une fois et cherche lui-même.
pub fn search_index(lang: Lang) -> String {
    let mut entries = Vec::new();
    for doc in in_lang(lang) {
        let href = doc_href(lang, doc.track.slug, doc.slug, None);
        let name = json_string(track_name(lang, doc.track));
        let track = json_string(doc.track.slug);
        entries.push(format!(
            r#"{{"t":{},"d":{},"p":{name},"s":{track},"u":{},"k":"page","x":{}}}"#,
            json_string(&doc.title),
            json_string(&doc.description),
            json_string(&href),
            json_string(&doc.text),
        ));
        for h in &doc.toc {
            entries.push(format!(
                r#"{{"t":{},"d":{},"p":{name},"s":{track},"u":{},"k":"section"}}"#,
                json_string(&h.text),
                json_string(&doc.title),
                json_string(&format!("{href}#{}", h.id)),
            ));
        }
    }
    entries.push(format!(
        r#"{{"t":{},"d":{},"p":"Docs","s":"","u":{},"k":"page"}}"#,
        json_string(tr(lang, "Comparer les langages")),
        json_string(tr(lang, "Les mêmes idées dans les sept langages, côte à côte.")),
        json_string(&Route::Compare.href(lang))
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
