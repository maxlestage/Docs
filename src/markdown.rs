//! Une page Markdown en HTML : titres avec ancres et sommaire, code coloré avec son
//! langage et un bouton « Copier », sorties de programmes, encadrés (« Astuce », « À
//! retenir »…), tableaux qui défilent, liens entre les pages devenus liens du site.

use pulldown_cmark::{Alignment, CodeBlockKind, CowStr, Event, HeadingLevel, Options, Parser, Tag, TagEnd};

use crate::highlight::{escape, highlight, label};
use crate::i18n::{Lang, tr};
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

/// Les encadrés : le mot qui commence la citation (en français, en anglais ou en
/// espagnol), et la classe de l'encadré.
const CALLOUTS: [(&str, &str); 23] = [
    ("Astuce", "tip"),
    ("Tip", "tip"),
    ("Consejo", "tip"),
    ("Indice", "hint"),
    ("Hint", "hint"),
    ("Pista", "hint"),
    ("Objectifs", "goals"),
    ("Goals", "goals"),
    ("Objetivos", "goals"),
    // La solution d'un exercice, la réponse d'une question : repliées.
    ("Solution", "solution"),
    ("Solución", "solution"),
    ("Réponse", "answer"),
    ("Answer", "answer"),
    ("Respuesta", "answer"),
    ("À retenir", "key"),
    ("Remember", "key"),
    ("Recuerda", "key"),
    ("Attention", "warn"),
    ("Warning", "warn"),
    ("Atención", "warn"),
    ("Note", "note"),
    ("Nota", "note"),
    ("Le saviez-vous", "note"),
];

/// Où mène un lien de la page `track` : un lien du site pour un fichier `.md`
/// (`fonctions.md#x`, `../rust/possession.md`), l'adresse telle quelle sinon ; et
/// `(parcours, page, ancre)` pour un lien vers une autre page.
///
/// Sur le site en `l`, une page `translated` (ou pas encore traduite) qui mène à une
/// page qui ne l'est pas (ou l'est déjà) perd son ancre : les titres, donc les ancres,
/// ne sont pas dans la même langue.
pub fn rewrite_link(dest: &str, track: &str, l: Lang, translated: bool) -> (String, Option<(String, String, String)>) {
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
    let target = l == Lang::Fr || crate::catalog::translation(l, track, page).is_some();
    let anchor = if target == translated { anchor } else { "" };
    (
        doc_href(l, track, page, Some(anchor)),
        Some((track.to_owned(), page.to_owned(), anchor.to_owned())),
    )
}

/// Un bloc de code : son en-tête (langage, bouton « Copier ») et le code coloré.
pub fn code_block(lang: &str, code: &str, l: Lang) -> String {
    let code = code.strip_suffix('\n').unwrap_or(code);
    if lang == "sortie" {
        return format!(
            r#"<div class="output"><div class="output__head"><span class="output__label">{}</span></div><pre><code>{}</code></pre></div>"#,
            tr(l, "Sortie"),
            lines(&escape(code))
        );
    }
    format!(
        concat!(
            r#"<div class="code" data-lang="{lang}"><div class="code__head">"#,
            r#"<span class="code__lang code__lang--{lang}">{label}</span>"#,
            r#"<button class="code__copy" type="button" aria-label="{copy_label}">{copy}</button>"#,
            r#"</div><pre><code class="language-{lang}">{html}</code></pre></div>"#,
        ),
        lang = escape(lang),
        label = label(lang),
        html = lines(&highlight(lang, code)),
        copy_label = tr(l, "Copier le code"),
        copy = tr(l, "Copier"),
    )
}

/// Met chaque ligne de `html` (du code déjà coloré) dans un `<span class="ln">`, pour
/// qu'une ligne trop longue passe à la ligne en retrait au lieu de faire défiler le bloc.
/// Les balises ouvertes en fin de ligne sont refermées puis rouvertes à la suivante ;
/// `--i` donne le retrait de la ligne. Le texte (et donc la copie) ne change pas.
fn lines(html: &str) -> String {
    let mut out = String::with_capacity(html.len() + html.len() / 4);
    let mut open: Vec<&str> = Vec::new();
    for (n, line) in html.split('\n').enumerate() {
        if n > 0 {
            out.push_str(&"</span>".repeat(open.len()));
            out.push_str("\n</span>");
        }
        let indent = line.chars().take_while(|&c| c == ' ').count();
        out.push_str(&format!(r#"<span class="ln" style="--i:{indent}">"#));
        for tag in &open {
            out.push_str(tag);
        }
        let mut rest = line;
        while let Some(i) = rest.find('<') {
            let j = rest[i..].find('>').map_or(rest.len(), |j| i + j + 1);
            let tag = &rest[i..j];
            if tag.starts_with("</") {
                open.pop();
            } else {
                open.push(tag);
            }
            rest = &rest[j..];
        }
        out.push_str(line);
    }
    out.push_str(&"</span>".repeat(open.len()));
    out.push_str("</span>");
    out
}

/// Rend `source`, une page du parcours `track`, pour le site en langue `l` : traduite,
/// ou en français si elle ne l'est pas encore (`translated`).
pub fn render(source: &str, track: &str, l: Lang, translated: bool) -> Rendered {
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
    // Le paragraphe « **Solution** » est remplacé par le bouton qui la déplie.
    let mut skipping = false;
    // Les liens ouverts : externe ou non.
    let mut open_links: Vec<bool> = Vec::new();
    // Un tableau large en cours, empilé en fiches sur téléphone : alignements, intitulés
    // des colonnes, dans l'en-tête ?, colonne de la cellule suivante.
    let mut wide: Option<(Vec<Alignment>, Vec<String>, bool, usize)> = None;

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
            Event::Text(t) | Event::Code(t) | Event::InlineHtml(t) => {
                words += t.split_whitespace().count();
                text.push_str(t);
                text.push(' ');
                if let Some((_, h, _)) = heading.as_mut() {
                    h.push_str(t);
                }
                if in_first_paragraph {
                    description.push_str(t);
                }
                if let Some((_, labels, true, _)) = wide.as_mut()
                    && let Some(label) = labels.last_mut()
                {
                    label.push_str(t);
                }
                if let Some(quote) = quotes.last_mut().filter(|q| !q.2) {
                    quote.2 = true;
                    if strong {
                        let word = t.trim_end_matches([' ', ':', '\u{a0}']);
                        quote.1 = CALLOUTS.iter().find(|(w, _)| *w == word).map(|(_, class)| *class);
                        if matches!(quote.1, Some("solution" | "answer")) {
                            events.truncate(quote.0 + 1);
                            skipping = true;
                        }
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
        if skipping {
            if matches!(event, Event::End(TagEnd::Paragraph)) {
                skipping = false;
                strong = false;
            }
            continue;
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
                        r##"<a class="anchor" href="#{id}" aria-label="{}">#</a></h{level}>"##,
                        tr(l, "Lien vers cette section")
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
                    Some("solution") => (
                        format!(
                            r#"<details class="solution"><summary>{}</summary>"#,
                            tr(l, "Voir la solution")
                        ),
                        "</details>",
                    ),
                    Some("answer") => (
                        format!(
                            r#"<details class="solution solution--answer"><summary>{}</summary>"#,
                            tr(l, "Voir la réponse")
                        ),
                        "</details>",
                    ),
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
                let (dest, internal) = rewrite_link(&dest_url, track, l, translated);
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
                    events.push(Event::Html(code_block(&lang, &body, l).into()));
                }
            }
            Event::Start(Tag::Table(ref alignments)) => {
                // Quatre colonnes ou plus ne tiennent pas sur un téléphone : chaque ligne y
                // devient une fiche, chaque cellule précédée de l'intitulé de sa colonne.
                if alignments.len() >= 4 {
                    wide = Some((alignments.clone(), Vec::new(), false, 0));
                    events.push(Event::Html(r#"<div class="table-wrap table-wrap--stack">"#.into()));
                } else {
                    events.push(Event::Html(r#"<div class="table-wrap">"#.into()));
                }
                events.push(event);
            }
            Event::End(TagEnd::Table) => {
                wide = None;
                events.push(event);
                events.push(Event::Html("</div>".into()));
            }
            Event::Start(Tag::TableHead) | Event::End(TagEnd::TableHead) => {
                if let Some(w) = wide.as_mut() {
                    w.2 = matches!(event, Event::Start(_));
                }
                events.push(event);
            }
            Event::Start(Tag::TableRow) => {
                if let Some(w) = wide.as_mut() {
                    w.3 = 0;
                }
                events.push(event);
            }
            Event::Start(Tag::TableCell) if wide.is_some() => {
                let (alignments, labels, head, col) = wide.as_mut().unwrap();
                if *head {
                    labels.push(String::new());
                    events.push(event);
                } else {
                    let align = match alignments.get(*col) {
                        Some(Alignment::Left) => r#" style="text-align: left""#,
                        Some(Alignment::Center) => r#" style="text-align: center""#,
                        Some(Alignment::Right) => r#" style="text-align: right""#,
                        _ => "",
                    };
                    let label = labels.get(*col).map_or("", |l| l.trim());
                    events.push(Event::Html(
                        format!(r#"<td data-label="{}"{align}><div>"#, escape(label)).into(),
                    ));
                    *col += 1;
                }
            }
            Event::End(TagEnd::TableCell) if wide.as_ref().is_some_and(|w| !w.2) => {
                events.push(Event::Html("</div></td>".into()));
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
        let r = render(PAGE, "ruby", Lang::Fr, true);
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
    fn solutions_fold_and_hints_are_callouts() {
        let r = render(
            "# E\n\nUn exercice.\n\n> **Indice :** pensez-y.\n\n> **Solution**\n>\n> ```ruby\n> puts 1\n> ```\n",
            "ruby",
            Lang::Fr,
            true,
        );
        assert!(
            r.html.contains(r#"<aside class="callout callout--hint">"#),
            "{}",
            r.html
        );
        assert!(
            r.html
                .contains(r#"<details class="solution"><summary>Voir la solution</summary>"#),
            "{}",
            r.html
        );
        assert!(!r.html.contains("<strong>Solution</strong>"), "{}", r.html);
        assert!(r.html.contains(r#"<div class="code" data-lang="ruby">"#));
        assert!(r.html.trim_end().ends_with("</details>"), "{}", r.html);
    }

    #[test]
    fn english_pages() {
        let r = render(
            "# E\n\nAn exercise, [see](fonctions.md).\n\n> **Tip:** try it.\n\n> **Answer**\n>\n> Yes.\n\n```sortie\n1\n```\n",
            "rust",
            Lang::En,
            true,
        );
        assert!(r.html.contains(r#"<aside class="callout callout--tip">"#), "{}", r.html);
        assert!(r.html.contains("<summary>Show the answer</summary>"), "{}", r.html);
        assert!(r.html.contains(r#"href="/en/rust/fonctions/""#), "{}", r.html);
        assert!(r.html.contains(">Output<"), "{}", r.html);
    }

    #[test]
    fn slugs() {
        assert_eq!(slug("Les boucles « for » et `while`"), "les-boucles-for-et-while");
        assert_eq!(slug("Où ? Ça ! Œuvre"), "ou-ca-oeuvre");
        assert_eq!(slug("Option<T> et Result<T, E>"), "option-t-et-result-t-e");
    }

    #[test]
    fn tableaux_larges_et_lignes_de_code() {
        let r = render(
            "# T\n\n| A | B |\n|---|---|\n| 1 | 2 |\n\n| Nom | `x` | C | D |\n|---|---|--:|---|\n| a | b | 3 | d |\n",
            "ruby",
            Lang::Fr,
            true,
        );
        assert!(r.html.contains(r#"<div class="table-wrap"><table>"#), "{}", r.html);
        assert!(
            r.html.contains(r#"<div class="table-wrap table-wrap--stack"><table>"#),
            "{}",
            r.html
        );
        assert!(r.html.contains("<td>1</td>"), "{}", r.html);
        assert!(
            r.html
                .contains(r#"<td data-label="Nom"><div>a</div></td><td data-label="x"><div>b</div></td>"#),
            "{}",
            r.html
        );
        assert!(
            r.html
                .contains(r#"<td data-label="C" style="text-align: right"><div>3</div></td>"#),
            "{}",
            r.html
        );

        let html = lines("<span class=\"c\">/* a\n  b */</span>\nx");
        assert_eq!(
            html,
            concat!(
                r#"<span class="ln" style="--i:0"><span class="c">/* a</span>"#,
                "\n</span>",
                r#"<span class="ln" style="--i:2"><span class="c">  b */</span>"#,
                "\n</span>",
                r#"<span class="ln" style="--i:0">x</span>"#,
            )
        );
    }
}
