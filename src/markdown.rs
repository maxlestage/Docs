//! Une page Markdown en HTML : titres avec ancres et sommaire, code coloré avec son
//! langage et un bouton « Copier », sorties de programmes, encadrés (« Astuce », « À
//! retenir »…), tableaux qui défilent, liens entre les pages devenus liens du site.

use pulldown_cmark::{CodeBlockKind, CowStr, Event, HeadingLevel, Options, Parser, Tag, TagEnd};

use crate::highlight::{escape, highlight, label};
use crate::routes::doc_href;

/// Un titre du sommaire.
#[derive(Clone, Debug, PartialEq)]
pub struct Heading {
    /// 2 ou 3.
    pub level: u8,
    pub id: String,
    pub text: String,
}

pub struct Rendered {
    /// Le texte du `# Titre`.
    pub title: String,
    /// Le premier paragraphe, en texte simple.
    pub description: String,
    pub html: String,
    pub toc: Vec<Heading>,
    /// Le texte de la page sans le code, pour la recherche.
    pub text: String,
    /// Le nombre de mots, code compris (pour le temps de lecture).
    pub words: usize,
    /// Les liens vers d'autres pages du site : (parcours, page, ancre).
    pub links: Vec<(String, String, String)>,
}

/// `## Les boucles « for »` → `les-boucles-for`.
pub fn slug(text: &str) -> String {
    let mut out = String::new();
    for c in text.chars().flat_map(char::to_lowercase) {
        let plain = match c {
            'à' | 'â' | 'ä' | 'á' => "a",
            'é' | 'è' | 'ê' | 'ë' => "e",
            'î' | 'ï' | 'í' => "i",
            'ô' | 'ö' | 'ó' => "o",
            'ù' | 'û' | 'ü' | 'ú' => "u",
            'ç' => "c",
            'ñ' => "n",
            'œ' => "oe",
            'æ' => "ae",
            _ => "",
        };
        if !plain.is_empty() {
            out.push_str(plain);
        } else if c.is_ascii_alphanumeric() {
            out.push(c);
        } else if !out.is_empty() && !out.ends_with('-') {
            out.push('-');
        }
    }
    out.trim_end_matches('-').to_owned()
}

/// Les encadrés : le mot qui commence la citation, et la classe de l'encadré.
const CALLOUTS: [(&str, &str); 5] = [
    ("Astuce", "tip"),
    ("À retenir", "key"),
    ("Attention", "warn"),
    ("Note", "note"),
    ("Le saviez-vous", "note"),
];

/// Où mène un lien de la page `track` : un lien du site pour un fichier `.md`
/// (`fonctions.md#x`, `../rust/possession.md`), l'adresse telle quelle sinon ; et
/// `(parcours, page, ancre)` pour un lien vers une autre page.
pub fn rewrite_link(dest: &str, track: &str) -> (String, Option<(String, String, String)>) {
    if dest.starts_with("http://")
        || dest.starts_with("https://")
        || dest.starts_with('#')
        || dest.starts_with("mailto:")
    {
        return (dest.to_owned(), None);
    }
    let (file, anchor) = dest.split_once('#').unwrap_or((dest, ""));
    let Some(file) = file.strip_suffix(".md") else {
        return (dest.to_owned(), None);
    };
    let file = file.trim_start_matches("./");
    let (track, page) = match file.strip_prefix("../").and_then(|f| f.split_once('/')) {
        Some((other, page)) => (other, page),
        None => (track, file),
    };
    (
        doc_href(track, page, Some(anchor)),
        Some((track.to_owned(), page.to_owned(), anchor.to_owned())),
    )
}

/// Un bloc de code : son en-tête (langage, bouton « Copier ») et le code coloré.
pub fn code_block(lang: &str, code: &str) -> String {
    let code = code.strip_suffix('\n').unwrap_or(code);
    if lang == "sortie" {
        return format!(
            r#"<div class="output"><div class="output__head"><span class="output__label">Sortie</span></div><pre><code>{}</code></pre></div>"#,
            escape(code)
        );
    }
    format!(
        concat!(
            r#"<div class="code" data-lang="{lang}"><div class="code__head">"#,
            r#"<span class="code__lang code__lang--{lang}">{label}</span>"#,
            r#"<button class="code__copy" type="button" aria-label="Copier le code">Copier</button>"#,
            r#"</div><pre><code class="language-{lang}">{html}</code></pre></div>"#,
        ),
        lang = escape(lang),
        label = label(lang),
        html = highlight(lang, code),
    )
}

/// Rend `source`, une page du parcours `track`.
pub fn render(source: &str, track: &str) -> Rendered {
    let mut title = String::new();
    let mut description = String::new();
    let mut toc = Vec::new();
    let mut text = String::new();
    let mut words = 0;
    let mut links = Vec::new();
    let mut events: Vec<Event<'_>> = Vec::new();

    // Un titre en cours : (niveau, texte, position de sa balise ouvrante).
    let mut heading: Option<(HeadingLevel, String, usize)> = None;
    // Le premier paragraphe après le titre : la description.
    let mut paragraphs = 0;
    let mut in_first_paragraph = false;
    // Un bloc de code en cours : (langage, texte).
    let mut code: Option<(String, String)> = None;
    // Les citations ouvertes : (position de la balise ouvrante, encadré, décidé ?).
    let mut quotes: Vec<(usize, Option<&str>, bool)> = Vec::new();
    let mut strong = false;
    // Les liens ouverts : externe ou non.
    let mut open_links: Vec<bool> = Vec::new();

    let options = Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH;
    for event in Parser::new_ext(source, options) {
        if let Some((_, body)) = code.as_mut() {
            match event {
                Event::Text(t) => {
                    body.push_str(&t);
                    continue;
                }
                Event::End(TagEnd::CodeBlock) => {}
                _ => continue,
            }
        }
        match &event {
            Event::Text(t) | Event::Code(t) => {
                words += t.split_whitespace().count();
                text.push_str(t);
                text.push(' ');
                if let Some((_, h, _)) = heading.as_mut() {
                    h.push_str(t);
                }
                if in_first_paragraph {
                    description.push_str(t);
                }
                if let Some(quote) = quotes.last_mut().filter(|q| !q.2) {
                    quote.2 = true;
                    if strong {
                        let word = t.trim_end_matches([' ', ':', '\u{a0}']);
                        quote.1 = CALLOUTS.iter().find(|(w, _)| *w == word).map(|(_, class)| *class);
                    }
                }
            }
            Event::SoftBreak | Event::HardBreak => {
                text.push(' ');
                if in_first_paragraph {
                    description.push(' ');
                }
            }
            _ => {}
        }
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                heading = Some((level, String::new(), events.len()));
                events.push(Event::Html(CowStr::Borrowed("")));
            }
            Event::End(TagEnd::Heading(_)) => {
                let Some((level, h, at)) = heading.take() else {
                    continue;
                };
                if level == HeadingLevel::H1 {
                    title = h.trim().to_owned();
                    events.truncate(at);
                    continue;
                }
                let mut id = slug(&h);
                let mut n = 1;
                while toc.iter().any(|t: &Heading| t.id == id) {
                    n += 1;
                    id = format!("{}-{n}", slug(&h));
                }
                let level = level as u8;
                events[at] = Event::Html(format!(r#"<h{level} id="{id}">"#).into());
                events.push(Event::Html(
                    format!(
                        r##"<a class="anchor" href="#{id}" aria-label="Lien vers cette section">#</a></h{level}>"##
                    )
                    .into(),
                ));
                if level <= 3 {
                    toc.push(Heading {
                        level,
                        id,
                        text: h.trim().to_owned(),
                    });
                }
            }
            Event::Start(Tag::Paragraph) if quotes.is_empty() => {
                paragraphs += 1;
                if paragraphs == 1 && toc.is_empty() {
                    in_first_paragraph = true;
                    events.push(Event::Html(r#"<p class="lead">"#.into()));
                } else {
                    events.push(event);
                }
            }
            Event::End(TagEnd::Paragraph) if in_first_paragraph => {
                in_first_paragraph = false;
                events.push(Event::Html("</p>".into()));
            }
            Event::Start(Tag::Strong) => {
                strong = true;
                events.push(event);
            }
            Event::End(TagEnd::Strong) => {
                strong = false;
                events.push(event);
            }
            Event::Start(Tag::BlockQuote(_)) => {
                quotes.push((events.len(), None, false));
                events.push(Event::Html("".into()));
            }
            Event::End(TagEnd::BlockQuote(_)) => {
                let Some((at, callout, _)) = quotes.pop() else {
                    continue;
                };
                let (open, close) = match callout {
                    Some(class) => (format!(r#"<aside class="callout callout--{class}">"#), "</aside>"),
                    None => ("<blockquote>".to_owned(), "</blockquote>"),
                };
                events[at] = Event::Html(open.into());
                events.push(Event::Html(close.into()));
            }
            Event::Start(Tag::Link {
                link_type,
                dest_url,
                title,
                id,
            }) => {
                let (dest, internal) = rewrite_link(&dest_url, track);
                let external = dest.starts_with("http");
                open_links.push(external);
                if let Some(target) = internal {
                    links.push(target);
                }
                if external {
                    events.push(Event::Html(
                        format!(r#"<a href="{}" target="_blank" rel="noopener">"#, escape(&dest)).into(),
                    ));
                } else {
                    events.push(Event::Start(Tag::Link {
                        link_type,
                        dest_url: dest.into(),
                        title,
                        id,
                    }));
                }
            }
            Event::End(TagEnd::Link) => {
                if open_links.pop() == Some(true) {
                    events.push(Event::Html("</a>".into()));
                } else {
                    events.push(event);
                }
            }
            Event::Start(Tag::CodeBlock(kind)) => {
                let lang = match kind {
                    CodeBlockKind::Fenced(info) => info.split_whitespace().next().unwrap_or("").to_owned(),
                    CodeBlockKind::Indented => String::new(),
                };
                code = Some((lang, String::new()));
            }
            Event::End(TagEnd::CodeBlock) => {
                if let Some((lang, body)) = code.take() {
                    words += body.split_whitespace().count();
                    events.push(Event::Html(code_block(&lang, &body).into()));
                }
            }
            Event::Start(Tag::Table(_)) => {
                events.push(Event::Html(r#"<div class="table-wrap">"#.into()));
                events.push(event);
            }
            Event::End(TagEnd::Table) => {
                events.push(event);
                events.push(Event::Html("</div>".into()));
            }
            // Pas de HTML écrit dans les pages : il est montré comme du texte.
            Event::Html(raw) | Event::InlineHtml(raw) => events.push(Event::Text(raw)),
            _ => events.push(event),
        }
    }
    let mut html = String::with_capacity(source.len() * 2);
    pulldown_cmark::html::push_html(&mut html, events.into_iter());
    Rendered {
        title,
        description: description.split_whitespace().collect::<Vec<_>>().join(" "),
        html,
        toc,
        text: text.split_whitespace().collect::<Vec<_>>().join(" "),
        words,
        links,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PAGE: &str = "# Les fonctions\n\nUne fonction **range** du code.\n\n## Définir `fn`\n\nTexte avec [un lien](collections.md#les-listes) et [Rust](../rust/fonctions.md) et [web](https://example.com).\n\n```ruby\nputs 1\n```\n\n```sortie\n1\n```\n\n> **Astuce :** une astuce.\n\n> Une citation.\n\n### Définir `fn`\n\n| a | b |\n|---|---|\n| 1 | 2 |\n\n<script>alert(1)</script>\n";

    #[test]
    fn renders_a_page() {
        let r = render(PAGE, "ruby");
        assert_eq!(r.title, "Les fonctions");
        assert_eq!(r.description, "Une fonction range du code.");
        assert!(
            r.html
                .starts_with(r#"<p class="lead">Une fonction <strong>range</strong>"#),
            "{}",
            r.html
        );
        assert!(!r.html.contains("<h1"));
        assert_eq!(
            r.toc,
            vec![
                Heading {
                    level: 2,
                    id: "definir-fn".into(),
                    text: "Définir fn".into()
                },
                Heading {
                    level: 3,
                    id: "definir-fn-2".into(),
                    text: "Définir fn".into()
                },
            ]
        );
        assert!(
            r.html
                .contains(r##"<h2 id="definir-fn">Définir <code>fn</code><a class="anchor" href="#definir-fn""##)
        );
        assert!(r.html.contains(r##"href="/ruby/collections/#les-listes""##));
        assert!(r.html.contains(r#"href="/rust/fonctions/""#));
        assert!(
            r.html
                .contains(r#"<a href="https://example.com" target="_blank" rel="noopener">web</a>"#)
        );
        assert!(r.html.contains(r#"<div class="code" data-lang="ruby">"#));
        assert!(r.html.contains(r#"<span class="output__label">Sortie</span>"#));
        assert!(
            r.html.contains(
                r#"<aside class="callout callout--tip">
<p><strong>Astuce :</strong> une astuce.</p>"#
            ),
            "{}",
            r.html
        );
        assert!(r.html.contains("<blockquote>\n<p>Une citation.</p>\n</blockquote>"));
        assert!(r.html.contains(r#"<div class="table-wrap"><table>"#));
        assert!(!r.html.contains("<script>"));
        assert_eq!(
            r.links,
            vec![
                ("ruby".into(), "collections".into(), "les-listes".into()),
                ("rust".into(), "fonctions".into(), String::new()),
            ]
        );
        assert!(r.text.contains("une astuce"));
    }

    #[test]
    fn slugs() {
        assert_eq!(slug("Les boucles « for » et `while`"), "les-boucles-for-et-while");
        assert_eq!(slug("Où ? Ça ! Œuvre"), "ou-ca-oeuvre");
        assert_eq!(slug("Option<T> et Result<T, E>"), "option-t-et-result-t-e");
    }
}
