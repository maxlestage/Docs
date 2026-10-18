//! Toutes les pages du site, écrites avec active (par le prélude de Paquets) : rendues
//! sur le serveur, avec leurs balises SEO et une Content-Security-Policy stricte.

use paquets::prelude::*;

use crate::catalog::{TRACKS, Track};
use crate::compare::{self, CHOOSE, LANGS, ROWS};
use crate::highlight::highlight;
use crate::library::{self, Doc};
use crate::routes::{Route, doc_href, link};

pub const REPO: &str = "https://github.com/maxlestage/Docs";
const PAQUETS: &str = "https://paquets-90dfc06b6fea.herokuapp.com/";
const FONTS: &str = "https://fonts.googleapis.com/css2?family=Bricolage+Grotesque:opsz,wght@12..96,500;12..96,700;12..96,800&family=Inter:wght@400;500;600&family=JetBrains+Mono:wght@400;500&display=swap";

/// Avant le premier affichage : le thème choisi, et `js` sur `<html>` (les boutons qui
/// ont besoin de JavaScript n'apparaissent qu'avec lui).
const HEAD_SCRIPT: &str = r#"(function(){var d=document.documentElement;d.classList.add("js");try{var t=localStorage.getItem("theme");if(t==="light"||t==="dark")d.setAttribute("data-theme",t)}catch(e){}})();"#;

/// Le logo : trois touches, une par langage.
const LOGO: &str = r#"<svg class="logo" viewBox="0 0 32 32" width="28" height="28" aria-hidden="true"><rect x="3" y="4" width="26" height="7" rx="3.5" fill="var(--ruby)"/><rect x="3" y="12.5" width="19" height="7" rx="3.5" fill="var(--rust)"/><rect x="3" y="21" width="12" height="7" rx="3.5" fill="var(--rythme)"/></svg>"#;

const SEARCH_ICON: &str = r#"<svg viewBox="0 0 20 20" width="18" height="18" aria-hidden="true"><circle cx="8.5" cy="8.5" r="5.5" fill="none" stroke="currentColor" stroke-width="2"/><path d="m13 13 4.5 4.5" stroke="currentColor" stroke-width="2" stroke-linecap="round"/></svg>"#;

const THEME_ICON: &str = r#"<svg viewBox="0 0 20 20" width="18" height="18" aria-hidden="true"><path class="sun" fill="currentColor" d="M10 5.5a4.5 4.5 0 1 0 0 9 4.5 4.5 0 0 0 0-9ZM10 1v2.5M10 16.5V19M1 10h2.5M16.5 10H19M3.6 3.6l1.8 1.8M14.6 14.6l1.8 1.8M3.6 16.4l1.8-1.8M14.6 5.4l1.8-1.8" stroke="currentColor" stroke-width="1.6" stroke-linecap="round"/><path class="moon" fill="currentColor" d="M16.5 12.6A7 7 0 0 1 7.4 3.5a7 7 0 1 0 9.1 9.1Z"/></svg>"#;

const MENU_ICON: &str = r#"<svg viewBox="0 0 20 20" width="18" height="18" aria-hidden="true"><path d="M3 5.5h14M3 10h14M3 14.5h9" stroke="currentColor" stroke-width="2" stroke-linecap="round"/></svg>"#;

const GITHUB_ICON: &str = r#"<svg viewBox="0 0 16 16" width="18" height="18" aria-hidden="true"><path fill="currentColor" d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z"/></svg>"#;

/// Ce qu'il faut pour rendre une page.
pub struct Ctx {
    pub route: Route,
    /// L'adresse publique du site, avec le chemin de base, pour les URL absolues.
    pub origin: String,
    /// Version des styles et du script, pour `?v=` dans leurs liens.
    pub version: String,
}

impl Ctx {
    fn asset(&self, path: &str) -> String {
        format!("{}?v={}", link(path), self.version)
    }
}

// ----- Petits morceaux -------------------------------------------------------------

fn raw(class: &str, html: &str) -> Element {
    span()
        .class(class.to_owned())
        .attr("aria-hidden", "true")
        .html(html.to_owned())
}

fn external(href: &str, text: &str) -> Element {
    a().href(href.to_owned())
        .attr("target", "_blank")
        .attr("rel", "noopener")
        .text(text.to_owned())
}

/// Un morceau de code coloré, dans un `<pre>`.
fn code_pre(lang: &str, code: &str) -> Element {
    pre().class("snippet").child(code_el(lang, code))
}

fn code_el(lang: &str, code: &str) -> Element {
    paquets::prelude::code()
        .class(format!("language-{lang}"))
        .html(highlight(lang, code))
}

/// Les deux lettres d'un parcours : l'extension de ses fichiers (`Rb`, `Rs`, `Ry`).
fn badge(track: &Track) -> String {
    let mut chars = track.ext.chars();
    chars
        .next()
        .map(|c| c.to_uppercase().chain(chars).collect())
        .unwrap_or_default()
}

fn minutes(n: usize) -> String {
    format!("{n} min")
}

fn chapters(n: usize) -> String {
    if n > 1 {
        format!("{n} chapitres")
    } else {
        format!("{n} chapitre")
    }
}

/// Le nombre d'exemples de code d'un parcours (ou de tous).
fn examples(track: Option<&Track>) -> usize {
    library::all()
        .iter()
        .filter(|d| track.is_none_or(|t| t.slug == d.track.slug))
        .map(|d| d.html.matches(r#"<div class="code" data-lang=""#).count())
        .sum()
}

// ----- Mise en page ----------------------------------------------------------------

fn site_header(ctx: &Ctx) -> Element {
    let current = ctx.route.track().map(|t| t.slug);
    let compare = matches!(ctx.route, Route::Compare);
    let nav_link = |href: String, text: &str, class: String, active: bool| {
        let l = a().class("bar__link").class(class).href(href).text(text.to_owned());
        if active { l.attr("aria-current", "page") } else { l }
    };
    let menu = matches!(ctx.route, Route::Doc(..));
    header().class("bar").child(
        div()
            .class("bar__inner")
            .child(if menu {
                button()
                    .class("icon-btn menu-btn")
                    .attr("type", "button")
                    .attr("aria-label", "Chapitres")
                    .attr("aria-controls", "sidebar")
                    .attr("aria-expanded", "false")
                    .html(MENU_ICON)
            } else {
                span().class("menu-spacer")
            })
            .child(
                a().class("brand")
                    .href(link("/"))
                    .attr("aria-label", "Docs, accueil")
                    .child(raw("brand__logo", LOGO))
                    .child(span().class("brand__name").text("Docs")),
            )
            .child(
                nav()
                    .class("bar__nav")
                    .attr("aria-label", "Parcours")
                    .children(TRACKS.iter().map(|t| {
                        nav_link(
                            Route::Track(t).href(),
                            t.name,
                            format!("t-{}", t.slug),
                            current == Some(t.slug),
                        )
                    }))
                    .child(nav_link(
                        link("/comparer/"),
                        "Comparer",
                        "bar__link--compare".into(),
                        compare,
                    )),
            )
            .child(
                button()
                    .class("search-btn")
                    .attr("type", "button")
                    .attr("data-search", "")
                    .attr("aria-label", "Rechercher dans la documentation")
                    .attr("hidden", "")
                    .child(raw("search-btn__icon", SEARCH_ICON))
                    .child(span().class("search-btn__text").text("Rechercher"))
                    .child(kbd().text("/")),
            )
            .child(
                button()
                    .class("icon-btn theme-btn")
                    .attr("type", "button")
                    .id("theme-toggle")
                    .attr("aria-label", "Passer en thème clair ou sombre")
                    .attr("title", "Thème clair ou sombre")
                    .attr("hidden", "")
                    .html(THEME_ICON),
            )
            .child(
                a().class("icon-btn gh-btn")
                    .href(REPO)
                    .attr("target", "_blank")
                    .attr("rel", "noopener")
                    .attr("aria-label", "Le code du site sur GitHub")
                    .html(GITHUB_ICON),
            ),
    )
}

fn site_footer() -> Element {
    footer().class("foot").child(
        div()
            .class("foot__inner")
            .child(
                div()
                    .class("foot__brand")
                    .child(raw("brand__logo", LOGO))
                    .child(p().text("Des documentations simples, en français, pour Ruby, Rust et Rythme. Chaque exemple est exécuté avant d'être publié.")),
            )
            .child(
                nav().class("foot__cols").attr("aria-label", "Plan du site").child(
                    ul().children(TRACKS.iter().map(|t| li().child(a().href(Route::Track(t).href()).text(t.name))))
                        .child(li().child(a().href(link("/comparer/")).text("Comparer"))),
                ).child(
                    ul().child(li().child(external(REPO, "Le code du site")))
                        .child(li().child(external("https://github.com/maxlestage/Rythme", "Rythme")))
                        .child(li().child(external(PAQUETS, "Fait avec Paquets 📦"))),
                ),
            ),
    )
}

/// La fenêtre de recherche (ouverte par `site.js` : bouton, `/` ou Ctrl+K).
fn search_dialog() -> Element {
    dialog()
        .class("search")
        .id("search")
        .attr("aria-label", "Rechercher")
        .attr("data-index", link("/search.json"))
        .child(
            div()
                .class("search__box")
                .child(raw("search__icon", SEARCH_ICON))
                .child(
                    input()
                        .class("search__input")
                        .attr("type", "search")
                        .attr("placeholder", "Chercher un sujet : boucle, closure, match…")
                        .attr("aria-label", "Chercher")
                        .attr("autocomplete", "off")
                        .attr("spellcheck", "false"),
                )
                .child(
                    button()
                        .class("search__close")
                        .attr("type", "button")
                        .attr("aria-label", "Fermer")
                        .child(kbd().text("Échap")),
                ),
        )
        .child(
            div()
                .class("search__filters")
                .attr("role", "group")
                .attr("aria-label", "Parcours")
                .child(
                    button()
                        .class("chip is-on")
                        .attr("type", "button")
                        .attr("data-filter", "")
                        .text("Tout"),
                )
                .children(TRACKS.iter().map(|t| {
                    button()
                        .class("chip")
                        .class(format!("t-{}", t.slug))
                        .attr("type", "button")
                        .attr("data-filter", t.name)
                        .text(t.name)
                })),
        )
        .child(
            ul().class("search__results")
                .attr("role", "listbox")
                .attr("aria-label", "Résultats"),
        )
        .child(
            p().class("search__hint")
                .child(kbd().text("↑"))
                .child(kbd().text("↓"))
                .text(" pour choisir, ")
                .child(kbd().text("Entrée"))
                .text(" pour ouvrir"),
        )
}

/// La page autour de `content`.
fn layout(ctx: &Ctx, title: &str, description: &str, class: &str, content: Element) -> Page {
    let mut seo = Seo::new()
        .site_name("Docs : Ruby, Rust et Rythme")
        .base_url(ctx.origin.clone())
        .og_type(if matches!(ctx.route, Route::Doc(..)) {
            "article"
        } else {
            "website"
        })
        .locale("fr_FR");
    seo = if matches!(ctx.route, Route::NotFound) {
        seo.noindex(true)
    } else {
        seo.canonical(ctx.route.path())
    };
    let head = format!(
        concat!(
            r#"<meta name="color-scheme" content="light dark"/>"#,
            r##"<meta name="theme-color" content="#fbfaf7" media="(prefers-color-scheme: light)"/>"##,
            r##"<meta name="theme-color" content="#0e0d12" media="(prefers-color-scheme: dark)"/>"##,
            r#"<link rel="icon" href="{icon}" type="image/svg+xml"/>"#,
            r#"<link rel="preconnect" href="https://fonts.googleapis.com"/>"#,
            r#"<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin=""/>"#,
            r#"<script>{script}</script>"#,
            r#"<script src="{js}" defer=""></script>"#,
        ),
        icon = link("/assets/logo.svg"),
        script = HEAD_SCRIPT,
        js = ctx.asset("/site.js"),
    );
    let body = div()
        .class("site")
        .class(class.to_owned())
        .child(div().class("progress").attr("aria-hidden", "true"))
        .child(a().class("skip").href("#main").text("Aller au contenu"))
        .child(site_header(ctx))
        .child(content.id("main"))
        .child(site_footer())
        .child(search_dialog());
    let full_title = match ctx.route {
        Route::Home => title.to_owned(),
        _ => format!("{title} · Docs"),
    };
    Page::new(full_title)
        .lang("fr")
        .description(description)
        .seo(seo)
        .plain()
        .stylesheet(FONTS)
        .stylesheet(ctx.asset("/site.css"))
        .head(head)
        .body(body)
}

// ----- Accueil ---------------------------------------------------------------------

/// Le premier programme de chaque langage, sur l'accueil.
const HELLO: [(&str, &str); 3] = [
    (
        "ruby",
        "noms = [\"Ada\", \"Grace\"]\nnoms.each do |nom|\n  puts \"Bonjour, #{nom} !\"\nend",
    ),
    (
        "rust",
        "fn main() {\n    for nom in [\"Ada\", \"Grace\"] {\n        println!(\"Bonjour, {nom} !\");\n    }\n}",
    ),
    (
        "rythme",
        "[\"Ada\", \"Grace\"].each do |nom|\n  print(\"Bonjour, {nom} !\")\nend",
    ),
];

fn home(ctx: &Ctx) -> Page {
    let total: usize = TRACKS.iter().map(|t| t.pages.len()).sum();
    let hero = section()
        .class("hero")
        .attr("aria-labelledby", "hero-title")
        .child(
            div()
                .class("hero__text")
                .child(p().class("eyebrow").text("Documentation en français"))
                .child(
                    h1().class("hero__title")
                        .id("hero-title")
                        .child(span().class("w-ruby").text("Ruby"))
                        .text(", ")
                        .child(span().class("w-rust").text("Rust"))
                        .text(" et ")
                        .child(span().class("w-rythme").text("Rythme"))
                        .child(br())
                        .text("expliqués simplement."),
                )
                .child(p().class("hero__lead").text(
                    "Des chapitres courts, un exemple pour chaque idée, et du code qui marche vraiment : chaque exemple est exécuté avant d'être publié.",
                ))
                .child(
                    div()
                        .class("hero__actions")
                        .child(a().class("btn btn--primary").href("#parcours").text("Choisir un parcours"))
                        .child(a().class("btn btn--ghost").href(link("/comparer/")).text("Comparer les trois")),
                )
                .child(
                    button()
                        .class("hero__search")
                        .attr("type", "button")
                        .attr("data-search", "")
                        .attr("hidden", "")
                        .child(raw("search-btn__icon", SEARCH_ICON))
                        .child(span().text("Chercher : « closure », « match », « hash »…"))
                        .child(kbd().text("/")),
                )
                .child(
                    ul().class("stats")
                        .child(li().child(strong().text(total.to_string())).text(" chapitres"))
                        .child(li().child(strong().text(examples(None).to_string())).text(" exemples vérifiés"))
                        .child(li().child(strong().text("3")).text(" langages")),
                ),
        )
        .child(
            div().class("stack").attr("aria-label", "Le même programme dans les trois langages").children(HELLO.iter().map(|(lang, code)| {
                let track = crate::catalog::track(lang).expect("parcours");
                figure()
                    .class("window")
                    .class(format!("t-{lang}"))
                    .child(
                        div()
                            .class("window__bar")
                            .child(span().class("dots").attr("aria-hidden", "true").child(i()).child(i()).child(i()))
                            .child(span().class("window__file").text(format!("bonjour.{}", track.ext))),
                    )
                    .child(code_pre(lang, code))
            })),
        );

    let tracks = section()
        .class("section")
        .id("parcours")
        .attr("aria-labelledby", "parcours-title")
        .child(
            header()
                .class("section__head")
                .child(h2().id("parcours-title").text("Trois parcours, dix chapitres chacun"))
                .child(p().text("Commencez par l'introduction, ou allez directement au chapitre qui vous intéresse.")),
        )
        .child(ul().class("tracks").children(TRACKS.iter().map(track_card)));

    let choose = section()
        .class("section")
        .attr("aria-labelledby", "choisir-title")
        .child(
            header()
                .class("section__head")
                .child(h2().id("choisir-title").text("Lequel choisir ?"))
                .child(p().text("Les trois sont de bons premiers langages. Ils ne visent pas la même chose.")),
        )
        .child(choose_cards());

    let features = section()
        .class("section")
        .attr("aria-labelledby", "features-title")
        .child(header().class("section__head").child(h2().id("features-title").text("Pensé pour apprendre")))
        .child(ul().class("features").children(
            [
                ("✓", "Des exemples vérifiés", "Chaque morceau de code est lancé avec Ruby, Rust ou Rythme, et sa sortie comparée à celle de la page."),
                ("/", "Une recherche instantanée", "Touche / ou Ctrl+K, n'importe où : les chapitres et leurs sections, dans les trois langages."),
                ("⇄", "Les trois côte à côte", "Chaque chapitre renvoie au même chapitre dans les autres langages, et une page les compare ligne à ligne."),
                ("☾", "Partout, même hors ligne", "Sur téléphone comme sur ordinateur, en clair ou en sombre ; installable, et lisible sans réseau."),
            ]
            .into_iter()
            .map(|(icon, title, text)| {
                li().class("feature")
                    .child(span().class("feature__icon").attr("aria-hidden", "true").text(icon))
                    .child(h3().text(title))
                    .child(p().text(text))
            }),
        ));

    let teaser = section()
        .class("section")
        .attr("aria-labelledby", "teaser-title")
        .child(
            header()
                .class("section__head")
                .child(h2().id("teaser-title").text("La même idée, trois façons de l'écrire"))
                .child(p().text("Un aperçu de la page Comparer.")),
        )
        .child(compare_table(&ROWS[8..13]))
        .child(
            p().class("center").child(
                a().class("btn btn--ghost")
                    .href(link("/comparer/"))
                    .text("Voir toute la comparaison →"),
            ),
        );

    let content = main()
        .class("home")
        .child(hero)
        .child(tracks)
        .child(choose)
        .child(teaser)
        .child(features);
    layout(
        ctx,
        "Docs : Ruby, Rust et Rythme expliqués simplement",
        "Des documentations simples et en français pour apprendre Ruby, Rust et Rythme : des chapitres courts, des exemples vérifiés, une recherche instantanée.",
        "page-home",
        content,
    )
}

fn track_card(track: &'static Track) -> Element {
    let docs: Vec<&Doc> = library::of(track).collect();
    li().class("track").class(format!("t-{}", track.slug)).child(
        article()
            .class("track__card")
            .child(
                header()
                    .class("track__head")
                    .child(
                        span()
                            .class("track__badge")
                            .attr("aria-hidden", "true")
                            .text(badge(track)),
                    )
                    .child(
                        h3().child(
                            a().class("track__link")
                                .href(Route::Track(track).href())
                                .text(track.name),
                        ),
                    )
                    .child(p().class("track__tagline").text(track.tagline)),
            )
            .child(
                ol().class("track__toc").children(
                    docs.iter()
                        .take(6)
                        .map(|d| li().child(a().href(doc_href(track.slug, d.slug, None)).text(d.title.clone()))),
                ),
            )
            .child(
                footer()
                    .class("track__foot")
                    .child(span().class("muted").text(format!(
                        "{} · {}",
                        chapters(docs.len()),
                        minutes(library::minutes(track))
                    )))
                    .child(
                        a().class("btn btn--track")
                            .href(doc_href(track.slug, track.pages[0].slug, None))
                            .text("Commencer →"),
                    ),
            ),
    )
}

fn choose_cards() -> Element {
    ul().class("choose").children(CHOOSE.iter().map(|(slug, title, text)| {
        let track = crate::catalog::track(slug).expect("parcours");
        li().class("choose__card")
            .class(format!("t-{slug}"))
            .child(p().class("eyebrow").text(track.name))
            .child(h3().text(*title))
            .child(p().text(*text))
            .child(
                a().class("more")
                    .href(Route::Track(track).href())
                    .text(format!("Le parcours {} →", track.name)),
            )
    }))
}

// ----- Comparer --------------------------------------------------------------------

fn compare_table(rows: &[compare::Row]) -> Element {
    div().class("table-wrap compare-wrap").child(
        table()
            .class("compare")
            .child(
                thead().child(
                    tr().child(th().attr("scope", "col").text("Idée"))
                        .children(LANGS.iter().map(|l| {
                            let track = crate::catalog::track(l).expect("parcours");
                            th().attr("scope", "col")
                                .class(format!("t-{l}"))
                                .child(a().href(Route::Track(track).href()).text(track.name))
                        })),
                ),
            )
            .child(tbody().children(rows.iter().map(|row| {
                tr().child(th().attr("scope", "row").text(row.idea)).children(
                    LANGS
                        .iter()
                        .zip(row.code)
                        .zip(row.chapter)
                        .map(|((lang, code), chapter)| {
                            let name = crate::catalog::track(lang).map_or("", |t| t.name);
                            td().attr("data-label", name)
                                .class(format!("t-{lang}"))
                                .child(code_pre(lang, code))
                                .child(
                                    a().class("compare__more")
                                        .href(doc_href(lang, chapter, None))
                                        .attr("aria-label", format!("{} en {name}", row.idea))
                                        .text("Le chapitre →"),
                                )
                        }),
                )
            }))),
    )
}

fn compare_page(ctx: &Ctx) -> Page {
    let content = main()
        .class("compare-page")
        .child(
            header()
                .class("page-head")
                .child(p().class("eyebrow").text("Ruby · Rust · Rythme"))
                .child(h1().text("Comparer les trois langages"))
                .child(p().class("lead").text(
                    "Les mêmes idées, écrites en Ruby, en Rust et en Rythme. Chaque case renvoie au chapitre qui l'explique.",
                )),
        )
        .child(compare_table(ROWS))
        .child(
            section()
                .class("section")
                .attr("aria-labelledby", "choisir-title")
                .child(header().class("section__head").child(h2().id("choisir-title").text("Lequel choisir ?")))
                .child(choose_cards()),
        );
    layout(
        ctx,
        "Comparer Ruby, Rust et Rythme",
        "Afficher, nommer, choisir, répéter, fonctions, listes, types, erreurs, tests : les mêmes idées en Ruby, en Rust et en Rythme, côte à côte.",
        "page-compare",
        content,
    )
}

// ----- Un parcours -----------------------------------------------------------------

fn track_page(ctx: &Ctx, track: &'static Track) -> Page {
    let docs: Vec<&Doc> = library::of(track).collect();
    let content = main()
        .class("track-page")
        .class(format!("t-{}", track.slug))
        .child(
            header()
                .class("page-head page-head--track")
                .child(
                    nav()
                        .class("crumbs")
                        .attr("aria-label", "Fil d'Ariane")
                        .child(a().href(link("/")).text("Accueil"))
                        .child(span().text(track.name)),
                )
                .child(
                    span()
                        .class("track__badge track__badge--big")
                        .attr("aria-hidden", "true")
                        .text(badge(track)),
                )
                .child(h1().text(format!("Apprendre {}", track.name)))
                .child(p().class("lead").text(track.description))
                .child(
                    ul().class("stats")
                        .child(li().child(strong().text(docs.len().to_string())).text(" chapitres"))
                        .child(
                            li().child(strong().text(minutes(library::minutes(track))))
                                .text(" de lecture"),
                        )
                        .child(
                            li().child(strong().text(examples(Some(track)).to_string()))
                                .text(" exemples"),
                        ),
                )
                .child(
                    div()
                        .class("hero__actions")
                        .child(
                            a().class("btn btn--track")
                                .href(doc_href(track.slug, track.pages[0].slug, None))
                                .text("Commencer le parcours →"),
                        )
                        .child(
                            a().class("btn btn--ghost")
                                .href(track.site.1)
                                .attr("target", "_blank")
                                .attr("rel", "noopener")
                                .text(format!("{} ↗", track.site.0)),
                        ),
                ),
        )
        .child(ol().class("chapters").children(docs.iter().map(|d| {
            li().class("chapter").child(
                a().class("chapter__link")
                    .href(doc_href(track.slug, d.slug, None))
                    .child(span().class("chapter__num").text(format!("{:02}", d.index + 1)))
                    .child(
                        span()
                            .class("chapter__body")
                            .child(span().class("chapter__title").text(d.title.clone()))
                            .child(span().class("chapter__desc").text(d.description.clone())),
                    )
                    .child(span().class("chapter__time").text(minutes(d.minutes))),
            )
        })))
        .child(
            section()
                .class("section others")
                .attr("aria-label", "Les autres parcours")
                .child(h2().text("Les autres parcours"))
                .child(
                    ul().class("tracks tracks--small")
                        .children(TRACKS.iter().filter(|t| t.slug != track.slug).map(track_card)),
                ),
        );
    layout(
        ctx,
        &format!("Apprendre {}", track.name),
        track.description,
        "page-track",
        content,
    )
}

// ----- Un chapitre -----------------------------------------------------------------

fn sidebar(doc: &Doc) -> Element {
    let track = doc.track;
    aside()
        .class("sidebar")
        .id("sidebar")
        .attr("aria-label", format!("Chapitres de {}", track.name))
        .child(
            div()
                .class("sidebar__inner")
                .child(
                    a().class("sidebar__track")
                        .href(Route::Track(track).href())
                        .child(
                            span()
                                .class("track__badge")
                                .attr("aria-hidden", "true")
                                .text(badge(track)),
                        )
                        .child(span().text(track.name)),
                )
                .child(
                    div()
                        .class("meter")
                        .attr("role", "img")
                        .attr(
                            "aria-label",
                            format!("Chapitre {} sur {}", doc.index + 1, track.pages.len()),
                        )
                        .child(
                            div()
                                .class("meter__fill")
                                .style(format!("width:{}%", (doc.index + 1) * 100 / track.pages.len())),
                        ),
                )
                .child(ol().class("sidebar__list").children(library::of(track).map(|d| {
                    let l = a()
                        .href(doc_href(track.slug, d.slug, None))
                        .child(span().class("sidebar__num").text(format!("{}", d.index + 1)))
                        .child(span().text(d.title.clone()));
                    li().child(if d.slug == doc.slug {
                        l.class("is-current").attr("aria-current", "page")
                    } else {
                        l
                    })
                })))
                .child(
                    div()
                        .class("sidebar__switch")
                        .child(p().class("sidebar__label").text("Autres langages"))
                        .children(TRACKS.iter().filter(|t| t.slug != track.slug).map(|t| {
                            let href = match library::get(t.slug, doc.slug) {
                                Some(d) => doc_href(t.slug, d.slug, None),
                                None => Route::Track(t).href(),
                            };
                            a().class("chip").class(format!("t-{}", t.slug)).href(href).text(t.name)
                        })),
                ),
        )
}

fn toc(doc: &Doc) -> Element {
    aside().class("toc").attr("aria-label", "Sur cette page").child(
        div()
            .class("toc__inner")
            .child(p().class("toc__title").text("Sur cette page"))
            .child(ul().children(doc.toc.iter().map(|h| {
                li().class(format!("toc__l{}", h.level)).child(
                    a().href(format!("#{}", h.id))
                        .attr("data-toc", h.id.clone())
                        .text(h.text.clone()),
                )
            })))
            .child(a().class("toc__top").href("#main").text("↑ Haut de page")),
    )
}

fn pager(doc: &Doc) -> Element {
    let track = doc.track;
    let card = |d: &Doc, rel: &str, label: &str| {
        a().class(format!("pager__card pager__card--{rel}"))
            .href(doc_href(track.slug, d.slug, None))
            .attr("rel", rel.to_owned())
            .child(span().class("pager__label").text(label.to_owned()))
            .child(span().class("pager__title").text(d.title.clone()))
    };
    let docs: Vec<&Doc> = library::of(track).collect();
    let mut nav = nav().class("pager").attr("aria-label", "Chapitre précédent et suivant");
    if doc.index > 0 {
        nav = nav.child(card(docs[doc.index - 1], "prev", "← Précédent"));
    } else {
        nav = nav.child(span());
    }
    if let Some(next) = docs.get(doc.index + 1) {
        nav = nav.child(card(next, "next", "Suivant →"));
    } else {
        nav = nav.child(
            a().class("pager__card pager__card--next")
                .href(link("/comparer/"))
                .child(span().class("pager__label").text("Et ensuite ? →"))
                .child(span().class("pager__title").text("Comparer les trois langages")),
        );
    }
    nav
}

fn doc_page(ctx: &Ctx, track: &'static Track, index: usize) -> Page {
    let slug = track.pages[index].slug;
    let doc = library::get(track.slug, slug).expect("chaque page du catalogue est rendue");
    let others: Vec<Element> = TRACKS
        .iter()
        .filter(|t| t.slug != track.slug)
        .filter_map(|t| library::get(t.slug, slug).map(|d| (t, d)))
        .map(|(t, d)| {
            a().class("chip")
                .class(format!("t-{}", t.slug))
                .href(doc_href(t.slug, d.slug, None))
                .text(format!("{} : {}", t.name, d.title))
        })
        .collect();
    let mut article = article()
        .class("doc")
        .child(
            nav()
                .class("crumbs")
                .attr("aria-label", "Fil d'Ariane")
                .child(a().href(link("/")).text("Accueil"))
                .child(a().href(Route::Track(track).href()).text(track.name))
                .child(span().attr("aria-current", "page").text(doc.title.clone())),
        )
        .child(
            header()
                .class("doc__head")
                .child(p().class("eyebrow").text(format!(
                    "{} · Chapitre {} sur {} · {} de lecture",
                    track.name,
                    index + 1,
                    track.pages.len(),
                    minutes(doc.minutes)
                )))
                .child(h1().text(doc.title.clone())),
        )
        .child(div().class("prose").html(doc.html.clone()));
    if !others.is_empty() {
        article = article.child(
            aside()
                .class("also")
                .attr("aria-label", "Le même chapitre dans les autres langages")
                .child(
                    p().class("also__title")
                        .text("Le même chapitre dans les autres langages"),
                )
                .child(div().class("also__links").children(others)),
        );
    }
    article = article.child(pager(doc)).child(p().class("edit").child(external(
        &format!("{REPO}/edit/master/content/{}/{}.md", track.slug, slug),
        "Une erreur, une idée ? Modifier cette page sur GitHub",
    )));
    let content = main()
        .class("doc-layout")
        .class(format!("t-{}", track.slug))
        .child(sidebar(doc))
        .child(
            div()
                .class("scrim")
                .attr("data-close-menu", "")
                .attr("aria-hidden", "true"),
        )
        .child(article)
        .child(toc(doc));
    let description = if doc.description.is_empty() {
        track.description.to_owned()
    } else {
        doc.description.clone()
    };
    layout(
        ctx,
        &format!("{} – {}", doc.title, track.name),
        &description,
        "page-doc",
        content,
    )
}

// ----- Introuvable -----------------------------------------------------------------

pub fn not_found(ctx: &Ctx) -> Page {
    let content = main()
        .class("not-found")
        .child(p().class("big-404").attr("aria-hidden", "true").text("404"))
        .child(h1().text("Cette page n'existe pas"))
        .child(
            p().class("lead")
                .text("Elle a peut-être changé d'adresse. Choisissez un parcours, ou cherchez ce qu'il vous faut."),
        )
        .child(
            ul().class("tracks tracks--small")
                .children(TRACKS.iter().map(track_card)),
        );
    layout(ctx, "Page introuvable", "Cette page n'existe pas.", "page-404", content).status(404)
}

/// La page d'une route.
pub fn render(ctx: &Ctx) -> Page {
    match ctx.route {
        Route::Home => home(ctx),
        Route::Compare => compare_page(ctx),
        Route::Track(track) => track_page(ctx, track),
        Route::Doc(track, index) => doc_page(ctx, track, index),
        Route::NotFound => not_found(ctx),
    }
}
