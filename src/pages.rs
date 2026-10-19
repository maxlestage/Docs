//! Toutes les pages du site, écrites avec active (par le prélude de Paquets) : rendues
//! sur le serveur, avec leurs balises SEO et une Content-Security-Policy stricte.

use paquets::prelude::*;

use crate::catalog::{TRACKS, Track, languages};
use crate::compare::{self, CHOOSE, DEFAULT, LANGS, ROWS};
use crate::highlight::highlight;
use crate::i18n::{self, Lang, Meta, tr};
use crate::library::{self, Doc};
use crate::routes::{Route, doc_href, lang_link, link};

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
    pub lang: Lang,
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

    /// Un lien dans la langue de la page.
    fn link(&self, path: &str) -> String {
        lang_link(self.lang, path)
    }
}

/// La présentation d'un parcours dans la langue `l`.
fn info(l: Lang, track: &Track) -> Meta {
    i18n::meta(l, track.slug).unwrap_or(Meta {
        name: track.name,
        title: track.title,
        tagline: track.tagline,
        description: track.description,
        site: track.site,
    })
}

/// Le nom d'un parcours dans la langue `l`.
fn name(l: Lang, track: &Track) -> &'static str {
    library::track_name(l, track)
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

/// Les parties d'un parcours, avec les positions de leurs chapitres.
fn parts(track: &Track) -> Vec<(&'static str, std::ops::Range<usize>)> {
    track
        .parts
        .iter()
        .enumerate()
        .map(|(k, (start, name))| {
            let end = track.parts.get(k + 1).map_or(track.pages.len(), |p| p.0);
            (*name, *start..end)
        })
        .collect()
}

/// Les deux lettres d'un parcours : l'extension de ses fichiers (`Rb`, `Rs`, `Ry`).
fn badge(track: &Track) -> String {
    if !track.language {
        return "✦".to_owned();
    }
    let mut chars = track.ext.chars();
    chars
        .next()
        .map(|c| c.to_uppercase().chain(chars).collect())
        .unwrap_or_default()
}

fn minutes(n: usize) -> String {
    format!("{n} min")
}

/// Le nombre d'exemples de code d'un parcours (ou de tous).
fn examples(track: Option<&Track>) -> usize {
    library::in_lang(Lang::Fr)
        .filter(|d| track.is_none_or(|t| t.slug == d.track.slug))
        .map(|d| d.html.matches(r#"<div class="code" data-lang=""#).count())
        .sum()
}

// ----- Mise en page ----------------------------------------------------------------

fn site_header(ctx: &Ctx) -> Element {
    let l = ctx.lang;
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
                    .attr("aria-label", tr(l, "Chapitres"))
                    .attr("aria-controls", "sidebar")
                    .attr("aria-expanded", "false")
                    .html(MENU_ICON)
            } else {
                span().class("menu-spacer")
            })
            .child(
                a().class("brand")
                    .href(ctx.link("/"))
                    .attr("aria-label", tr(l, "Docs, accueil"))
                    .child(raw("brand__logo", LOGO))
                    .child(span().class("brand__name").text("Docs")),
            )
            .child(
                nav()
                    .class("bar__nav")
                    .attr("aria-label", tr(l, "Parcours"))
                    .children(TRACKS.iter().filter(|t| !t.language).map(|t| {
                        nav_link(
                            Route::Track(t).href(l),
                            name(l, t),
                            format!("t-{}", t.slug),
                            current == Some(t.slug),
                        )
                    }))
                    .child(
                        details()
                            .class("langs")
                            .child(
                                summary()
                                    .class("bar__link langs__toggle")
                                    .class(match current.and_then(crate::catalog::track) {
                                        Some(t) if t.language => format!("t-{} is-current", t.slug),
                                        _ => String::new(),
                                    })
                                    .text(match current.and_then(crate::catalog::track) {
                                        Some(t) if t.language => t.name,
                                        _ => tr(l, "Langages"),
                                    }),
                            )
                            .child(
                                ul().class("langs__menu")
                                    .children(languages().map(|t| {
                                        let l = a()
                                            .class("langs__item")
                                            .class(format!("t-{}", t.slug))
                                            .href(Route::Track(t).href(l))
                                            .child(
                                                span().class("track__badge").attr("aria-hidden", "true").text(badge(t)),
                                            )
                                            .child(span().class("langs__name").text(t.name))
                                            .child(span().class("langs__count").text(i18n::chapters(l, t.pages.len())));
                                        li().child(if current == Some(t.slug) {
                                            l.attr("aria-current", "page")
                                        } else {
                                            l
                                        })
                                    }))
                                    // Sur téléphone, le choix de la langue est ici (l'en-tête est étroit).
                                    .child(li().class("langs__site-lang").child(lang_switch(ctx))),
                            ),
                    )
                    .child(nav_link(
                        Route::Compare.href(l),
                        tr(l, "Comparer"),
                        "bar__link--compare".into(),
                        compare,
                    )),
            )
            .child(
                button()
                    .class("search-btn")
                    .attr("type", "button")
                    .attr("data-search", "")
                    .attr("aria-label", tr(l, "Rechercher dans la documentation"))
                    .attr("hidden", "")
                    .child(raw("search-btn__icon", SEARCH_ICON))
                    .child(span().class("search-btn__text").text(tr(l, "Rechercher")))
                    .child(kbd().text("/")),
            )
            .child(lang_switch(ctx))
            .child(
                button()
                    .class("icon-btn theme-btn")
                    .attr("type", "button")
                    .id("theme-toggle")
                    .attr("aria-label", tr(l, "Passer en thème clair ou sombre"))
                    .attr("title", tr(l, "Thème clair ou sombre"))
                    .attr("hidden", "")
                    .html(THEME_ICON),
            )
            .child(
                a().class("icon-btn gh-btn")
                    .href(REPO)
                    .attr("target", "_blank")
                    .attr("rel", "noopener")
                    .attr("aria-label", tr(l, "Le code du site sur GitHub"))
                    .html(GITHUB_ICON),
            ),
    )
}

/// Les trois langues du site : la même page dans chacune.
fn lang_switch(ctx: &Ctx) -> Element {
    nav()
        .class("lang-switch")
        .attr("aria-label", tr(ctx.lang, "Langue"))
        .children(Lang::ALL.into_iter().map(|lang| {
            let href = match ctx.route {
                Route::NotFound => Route::Home.href(lang),
                route => route.href(lang),
            };
            let item = a()
                .class("lang-switch__item")
                .href(href)
                .attr("hreflang", lang.code())
                .attr("lang", lang.code())
                .attr("title", lang.name())
                .text(lang.code().to_uppercase());
            if lang == ctx.lang {
                item.attr("aria-current", "true")
            } else {
                item
            }
        }))
}

fn site_footer(ctx: &Ctx) -> Element {
    let l = ctx.lang;
    footer().class("foot").child(
        div()
            .class("foot__inner")
            .child(
                div()
                    .class("foot__brand")
                    .child(raw("brand__logo", LOGO))
                    .child(
                        div()
                            .child(p().text(tr(l, "Des documentations simples, en français, pour Python, JavaScript, TypeScript, Go, Ruby, Rust et Rythme. Chaque exemple est exécuté avant d'être publié.")))
                            .child(lang_switch(ctx)),
                    ),
            )
            .child(
                nav().class("foot__cols").attr("aria-label", tr(l, "Plan du site")).child(
                    ul().children(TRACKS.iter().map(|t| li().child(a().href(Route::Track(t).href(l)).text(name(l, t)))))
                        .child(li().child(a().href(Route::Compare.href(l)).text(tr(l, "Comparer")))),
                ).child(
                    ul().child(li().child(external(REPO, tr(l, "Le code du site"))))
                        .child(li().child(external("https://github.com/maxlestage/Rythme", "Rythme")))
                        .child(li().child(external(PAQUETS, tr(l, "Fait avec Paquets 📦")))),
                ),
            ),
    )
}

/// La fenêtre de recherche (ouverte par `site.js` : bouton, `/` ou Ctrl+K).
fn search_dialog(l: Lang) -> Element {
    dialog()
        .class("search")
        .id("search")
        .attr("aria-label", tr(l, "Rechercher"))
        .attr("data-index", lang_link(l, "/search.json"))
        .child(
            div()
                .class("search__box")
                .child(raw("search__icon", SEARCH_ICON))
                .child(
                    input()
                        .class("search__input")
                        .attr("type", "search")
                        .attr("placeholder", tr(l, "Chercher un sujet : boucle, closure, match…"))
                        .attr("aria-label", tr(l, "Chercher"))
                        .attr("autocomplete", "off")
                        .attr("spellcheck", "false"),
                )
                .child(
                    button()
                        .class("search__close")
                        .attr("type", "button")
                        .attr("aria-label", tr(l, "Fermer"))
                        .child(kbd().text(tr(l, "Échap"))),
                ),
        )
        .child(
            div()
                .class("search__filters")
                .attr("role", "group")
                .attr("aria-label", tr(l, "Parcours"))
                .child(
                    button()
                        .class("chip is-on")
                        .attr("type", "button")
                        .attr("data-filter", "")
                        .text(tr(l, "Tout")),
                )
                .children(TRACKS.iter().map(|t| {
                    button()
                        .class("chip")
                        .class(format!("t-{}", t.slug))
                        .attr("type", "button")
                        .attr("data-filter", t.slug)
                        .text(name(l, t))
                })),
        )
        .child(
            ul().class("search__results")
                .attr("role", "listbox")
                .attr("aria-label", tr(l, "Résultats")),
        )
        .child(
            p().class("search__hint")
                .child(kbd().text("↑"))
                .child(kbd().text("↓"))
                .text(tr(l, " pour choisir, "))
                .child(kbd().text(tr(l, "Entrée")))
                .text(tr(l, " pour ouvrir")),
        )
}

/// La page autour de `content`.
fn layout(ctx: &Ctx, title: &str, description: &str, class: &str, content: Element) -> Page {
    let l = ctx.lang;
    let mut seo = Seo::new()
        .site_name(tr(l, "Docs : apprendre à programmer"))
        .base_url(ctx.origin.clone())
        .og_type(if matches!(ctx.route, Route::Doc(..)) {
            "article"
        } else {
            "website"
        })
        .locale(l.locale());
    seo = if matches!(ctx.route, Route::NotFound) {
        seo.noindex(true)
    } else {
        seo.canonical(ctx.route.path(l))
    };
    // La même page dans les autres langues, pour les moteurs de recherche.
    let alternates: String = if matches!(ctx.route, Route::NotFound) {
        String::new()
    } else {
        Lang::ALL
            .iter()
            .map(|lang| (lang.code(), *lang))
            .chain(std::iter::once(("x-default", Lang::Fr)))
            .map(|(code, lang)| {
                format!(
                    r#"<link rel="alternate" hreflang="{code}" href="{}{}"/>"#,
                    ctx.origin,
                    ctx.route.path(lang)
                )
            })
            .collect()
    };
    let head = format!(
        concat!(
            r#"<meta name="color-scheme" content="light dark"/>"#,
            r##"<meta name="theme-color" content="#fbfaf7" media="(prefers-color-scheme: light)"/>"##,
            r##"<meta name="theme-color" content="#0e0d12" media="(prefers-color-scheme: dark)"/>"##,
            r#"<link rel="icon" href="{icon}" type="image/svg+xml"/>"#,
            r#"<link rel="apple-touch-icon" href="{apple}"/>"#,
            r#"<link rel="preconnect" href="https://fonts.googleapis.com"/>"#,
            r#"<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin=""/>"#,
            r#"<script>{script}</script>"#,
            r#"<script src="{js}" defer=""></script>"#,
            "{alternates}",
        ),
        icon = link("/assets/logo.svg"),
        apple = link("/icons/apple-touch-icon.png"),
        script = HEAD_SCRIPT,
        js = ctx.asset("/site.js"),
        alternates = alternates,
    );
    let body = div()
        .class("site")
        .class(class.to_owned())
        .child(div().class("progress").attr("aria-hidden", "true"))
        .child(a().class("skip").href("#main").text(tr(l, "Aller au contenu")))
        .child(site_header(ctx))
        .child(content.id("main"))
        .child(site_footer(ctx))
        .child(search_dialog(l));
    let full_title = match ctx.route {
        Route::Home => title.to_owned(),
        _ => format!("{title} · Docs"),
    };
    Page::new(full_title)
        .lang(l.code())
        .description(description)
        .seo(seo)
        .plain()
        .stylesheet(FONTS)
        .stylesheet(ctx.asset("/site.css"))
        .head(head)
        .body(body)
}

// ----- Accueil ---------------------------------------------------------------------

/// Les langages du premier programme, sur l'accueil.
const HELLO: [&str; 3] = ["python", "javascript", "rust"];

fn home(ctx: &Ctx) -> Page {
    let l = ctx.lang;
    let total: usize = TRACKS.iter().map(|t| t.pages.len()).sum();
    let hero = section()
        .class("hero")
        .attr("aria-labelledby", "hero-title")
        .child(
            div()
                .class("hero__text")
                .child(p().class("eyebrow langs-line").children(languages().map(|t| span().class(format!("w-{}", t.slug)).text(t.name))))
                .child(
                    h1().class("hero__title")
                        .id("hero-title")
                        .text(tr(l, "Apprendre à programmer, "))
                        .child(span().class("w-grad").text(tr(l, "simplement.")))
                        .child(br()),
                )
                .child(p().class("hero__lead").text(tr(
                    l,
                    "Des chapitres courts, un exemple pour chaque idée, et du code qui marche vraiment : chaque exemple est exécuté avant d'être publié.",
                )))
                .child(
                    div()
                        .class("hero__actions")
                        .child(a().class("btn btn--primary").href("#parcours").text(tr(l, "Choisir un parcours")))
                        .child(a().class("btn btn--ghost").href(Route::Compare.href(l)).text(tr(l, "Comparer les langages"))),
                )
                .child(
                    button()
                        .class("hero__search")
                        .attr("type", "button")
                        .attr("data-search", "")
                        .attr("hidden", "")
                        .child(raw("search-btn__icon", SEARCH_ICON))
                        .child(span().text(tr(l, "Chercher : « closure », « match », « hash »…")))
                        .child(kbd().text("/")),
                )
                .child(
                    ul().class("stats")
                        .child(li().child(strong().text(total.to_string())).text(tr(l, " chapitres")))
                        .child(li().child(strong().text(examples(None).to_string())).text(tr(l, " exemples vérifiés")))
                        .child(li().child(strong().text(languages().count().to_string())).text(tr(l, " langages"))),
                ),
        )
        .child(
            div().class("stack").attr("aria-label", tr(l, "Le même programme dans plusieurs langages")).children(HELLO.iter().map(|lang| {
                let track = crate::catalog::track(lang).expect("parcours");
                let (file, code) = i18n::hello(l, lang);
                figure()
                    .class("window")
                    .class(format!("t-{lang}"))
                    .child(
                        div()
                            .class("window__bar")
                            .child(span().class("dots").attr("aria-hidden", "true").child(i()).child(i()).child(i()))
                            .child(span().class("window__file").text(format!("{file}.{}", track.ext))),
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
                .child(
                    h2().id("parcours-title")
                        .text(tr(l, "Sept langages, un parcours chacun")),
                )
                .child(p().text(tr(
                    l,
                    "Commencez par l'introduction, ou allez directement au chapitre qui vous intéresse.",
                ))),
        )
        .child(ul().class("tracks").children(languages().map(|t| track_card(l, t))));

    let start = section()
        .class("section")
        .attr("aria-labelledby", "start-title")
        .child(
            header()
                .class("section__head")
                .child(h2().id("start-title").text(tr(l, "Comment apprendre ici")))
                .child(p().text(tr(
                    l,
                    "Trois étapes, à votre rythme. Le site retient les chapitres que vous avez lus.",
                ))),
        )
        .child(
            ol().class("steps")
                .children(STEPS.iter().map(|(title, text, href, cta)| {
                    li().class("step")
                        .child(h3().text(tr(l, title)))
                        .child(p().text(tr(l, text)))
                        .child(
                            a().class("more")
                                .href(if *href == "/comparer/" {
                                    Route::Compare.href(l)
                                } else {
                                    ctx.link(href)
                                })
                                .text(tr(l, cta)),
                        )
                })),
        )
        .child(debuter_banner(l));

    let choose = section()
        .class("section")
        .attr("aria-labelledby", "choisir-title")
        .child(
            header()
                .class("section__head")
                .child(h2().id("choisir-title").text(tr(l, "Lequel choisir ?")))
                .child(p().text(tr(
                    l,
                    "Tous sont de bons premiers langages. Ils ne visent pas la même chose.",
                ))),
        )
        .child(choose_cards(l));

    let features = section()
        .class("section")
        .attr("aria-labelledby", "features-title")
        .child(header().class("section__head").child(h2().id("features-title").text(tr(l, "Pensé pour apprendre"))))
        .child(ul().class("features").children(
            [
                ("✓", "Des exemples vérifiés", "Chaque morceau de code est lancé avec le vrai Python, Node, TypeScript, Go, Ruby, Rust ou Rythme, et sa sortie comparée à celle de la page."),
                ("/", "Une recherche instantanée", "Touche / ou Ctrl+K, n'importe où : les chapitres et leurs sections, dans tous les langages."),
                ("⇄", "Les langages côte à côte", "Chaque chapitre renvoie au même chapitre dans les autres langages, et une page les compare ligne à ligne."),
                ("☾", "Partout, même hors ligne", "Sur téléphone comme sur ordinateur, en clair ou en sombre ; installable, et lisible sans réseau."),
            ]
            .into_iter()
            .map(|(icon, title, text)| {
                li().class("feature")
                    .child(span().class("feature__icon").attr("aria-hidden", "true").text(icon))
                    .child(h3().text(tr(l, title)))
                    .child(p().text(tr(l, text)))
            }),
        ));

    let teaser = section()
        .class("section")
        .attr("aria-labelledby", "teaser-title")
        .child(
            header()
                .class("section__head")
                .child(
                    h2().id("teaser-title")
                        .text(tr(l, "La même idée, plusieurs façons de l'écrire")),
                )
                .child(p().text(tr(l, "Un aperçu de la page Comparer."))),
        )
        .child(compare_table(l, &ROWS[8..13], &DEFAULT))
        .child(
            p().class("center").child(
                a().class("btn btn--ghost")
                    .href(Route::Compare.href(l))
                    .text(tr(l, "Voir toute la comparaison →")),
            ),
        );

    let content = main()
        .class("home")
        .child(hero)
        .child(start)
        .child(tracks)
        .child(choose)
        .child(teaser)
        .child(features);
    layout(
        ctx,
        tr(l, "Docs : apprendre à programmer, simplement"),
        tr(
            l,
            "Des documentations simples et en français pour apprendre Python, JavaScript, TypeScript, Go, Ruby, Rust et Rythme : des chapitres courts, des exemples vérifiés, une recherche instantanée.",
        ),
        "page-home",
        content,
    )
}

/// Les trois étapes de l'accueil : (titre, texte, lien, bouton), traduits par `tr`.
const STEPS: [(&str, &str, &str, &str); 3] = [
    (
        "1. Les bases",
        "Jamais programmé ? Le parcours Débuter explique ce qu'est un programme, comment penser un algorithme, le terminal, l'éditeur, le débogage et Git.",
        "/debuter/",
        "Débuter →",
    ),
    (
        "2. Un langage",
        "Choisissez Python, JavaScript, TypeScript, Go, Ruby, Rust ou Rythme, et lisez les chapitres dans l'ordre : objectifs, exemples à essayer, erreurs fréquentes, questions pour vous tester.",
        "/#parcours",
        "Choisir un langage →",
    ),
    (
        "3. La pratique",
        "Chaque parcours finit par des tests, des projets guidés pas à pas, des exercices corrigés et un aide-mémoire.",
        "/comparer/",
        "Comparer les langages →",
    ),
];

/// Le bandeau du parcours « Débuter ».
fn debuter_banner(l: Lang) -> Element {
    let Some(track) = crate::catalog::track("debuter") else {
        return div();
    };
    a().class("banner t-debuter")
        .href(Route::Track(track).href(l))
        .child(span().class("track__badge").attr("aria-hidden", "true").text("✦"))
        .child(
            span()
                .class("banner__text")
                .child(strong().text(tr(l, "Jamais programmé ? Commencez ici.")))
                .child(span().text(i18n::debuter_pages(l, track.pages.len()))),
        )
        .child(span().class("banner__go").attr("aria-hidden", "true").text("→"))
}

fn track_card(l: Lang, track: &'static Track) -> Element {
    let docs: Vec<&Doc> = library::of(l, track).collect();
    let meta = info(l, track);
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
                                .href(Route::Track(track).href(l))
                                .text(meta.name),
                        ),
                    )
                    .child(p().class("track__tagline").text(meta.tagline)),
            )
            .child(
                ol().class("track__toc").children(
                    docs.iter()
                        .take(6)
                        .map(|d| li().child(a().href(doc_href(l, track.slug, d.slug, None)).text(d.title.clone()))),
                ),
            )
            .child(
                footer()
                    .class("track__foot")
                    .child(
                        span()
                            .class("muted")
                            .text(format!(
                                "{} · {}",
                                i18n::chapters(l, docs.len()),
                                minutes(library::minutes(l, track))
                            ))
                            .child(
                                span()
                                    .class("done-count")
                                    .attr("data-done-track", track.slug)
                                    .attr("data-done-total", docs.len().to_string()),
                            ),
                    )
                    .child(
                        a().class("btn btn--track")
                            .href(doc_href(l, track.slug, track.pages[0].slug, None))
                            .text(tr(l, "Commencer →")),
                    ),
            ),
    )
}

fn choose_cards(l: Lang) -> Element {
    ul().class("choose").children(CHOOSE.iter().map(|(slug, title, text)| {
        let track = crate::catalog::track(slug).expect("parcours");
        let (title, text) = i18n::choose(l, slug).unwrap_or((title, text));
        li().class("choose__card")
            .class(format!("t-{slug}"))
            .child(p().class("eyebrow").text(track.name))
            .child(h3().text(title))
            .child(p().text(text))
            .child(
                a().class("more")
                    .href(Route::Track(track).href(l))
                    .text(i18n::track_link(l, track.name)),
            )
    }))
}

// ----- Comparer --------------------------------------------------------------------

/// Le tableau de comparaison, avec les colonnes des langages `langs` (dans l'ordre de
/// `LANGS`). Chaque cellule porte `data-col` : la page Comparer cache ou montre les
/// colonnes que le lecteur choisit.
fn compare_table(l: Lang, rows: &[compare::Row], langs: &[&str]) -> Element {
    let cols: Vec<usize> = (0..LANGS.len()).filter(|&k| langs.contains(&LANGS[k])).collect();
    div().class("table-wrap compare-wrap").child(
        table()
            .class("compare")
            .attr("data-cols", cols.len().to_string())
            .child(
                thead().child(
                    paquets::prelude::tr()
                        .child(th().attr("scope", "col").text(i18n::tr(l, "Idée")))
                        .children(cols.iter().map(|&k| {
                            let slug = LANGS[k];
                            let track = crate::catalog::track(slug).expect("parcours");
                            th().attr("scope", "col")
                                .attr("data-col", slug)
                                .class(format!("t-{slug}"))
                                .child(a().href(Route::Track(track).href(l)).text(track.name))
                        })),
                ),
            )
            .child(tbody().children(rows.iter().map(|row| {
                let idea = i18n::tr(l, row.idea);
                paquets::prelude::tr()
                    .child(th().attr("scope", "row").text(idea))
                    .children(cols.iter().map(|&k| {
                        let (lang, code, chapter) = (LANGS[k], row.code[k], row.chapter[k]);
                        let name = crate::catalog::track(lang).map_or("", |t| t.name);
                        td().attr("data-label", name)
                            .attr("data-col", lang)
                            .class(format!("t-{lang}"))
                            .child(code_pre(lang, code))
                            .child(
                                a().class("compare__more")
                                    .href(doc_href(l, lang, chapter, None))
                                    .attr("aria-label", i18n::idea_in(l, idea, name))
                                    .text(i18n::tr(l, "Le chapitre →")),
                            )
                    }))
            }))),
    )
}

/// Les boutons qui choisissent les colonnes de la page Comparer (montrés par `site.js`).
fn compare_picker(l: Lang) -> Element {
    div()
        .class("picker")
        .attr("hidden", "")
        .attr("data-default", DEFAULT.join(" "))
        .child(p().class("picker__label").text(tr(l, "Langages affichés :")))
        .child(
            div()
                .class("picker__chips")
                .attr("role", "group")
                .attr("aria-label", tr(l, "Langages affichés"))
                .children(LANGS.iter().map(|l| {
                    let name = crate::catalog::track(l).map_or("", |t| t.name);
                    button()
                        .class("chip")
                        .class(format!("t-{l}"))
                        .attr("type", "button")
                        .attr("data-pick", *l)
                        .attr("aria-pressed", "true")
                        .text(name)
                })),
        )
}

fn compare_page(ctx: &Ctx) -> Page {
    let l = ctx.lang;
    let content = main()
        .class("compare-page")
        .child(
            header()
                .class("page-head")
                .child(p().class("eyebrow").text("Python · JavaScript · TypeScript · Go · Ruby · Rust · Rythme"))
                .child(h1().text(tr(l, "Comparer les langages")))
                .child(p().class("lead").text(tr(
                    l,
                    "Les mêmes idées, écrites dans les sept langages du site. Choisissez ceux que vous voulez voir côte à côte ; chaque case renvoie au chapitre qui l'explique.",
                )))
                .child(compare_picker(l)),
        )
        .child(compare_table(l, ROWS, &LANGS))
        .child(
            section()
                .class("section")
                .attr("aria-labelledby", "choisir-title")
                .child(header().class("section__head").child(h2().id("choisir-title").text(tr(l, "Lequel choisir ?"))))
                .child(choose_cards(l)),
        );
    layout(
        ctx,
        tr(l, "Comparer les langages"),
        tr(
            l,
            "Afficher, nommer, choisir, répéter, fonctions, listes, types, erreurs, tests : les mêmes idées en Python, JavaScript, TypeScript, Go, Ruby, Rust et Rythme, côte à côte.",
        ),
        "page-compare",
        content,
    )
}

// ----- Un parcours -----------------------------------------------------------------

fn track_page(ctx: &Ctx, track: &'static Track) -> Page {
    let l = ctx.lang;
    let meta = info(l, track);
    let docs: Vec<&Doc> = library::of(l, track).collect();
    let content = main()
        .class("track-page")
        .class(format!("t-{}", track.slug))
        .child(
            header()
                .class("page-head page-head--track")
                .child(
                    nav()
                        .class("crumbs")
                        .attr("aria-label", tr(l, "Fil d'Ariane"))
                        .child(a().href(ctx.link("/")).text(tr(l, "Accueil")))
                        .child(span().text(meta.name)),
                )
                .child(
                    span()
                        .class("track__badge track__badge--big")
                        .attr("aria-hidden", "true")
                        .text(badge(track)),
                )
                .child(h1().text(meta.title))
                .child(p().class("lead").text(meta.description))
                .child(
                    ul().class("stats")
                        .child(
                            li().child(strong().text(docs.len().to_string()))
                                .text(tr(l, " chapitres")),
                        )
                        .child(
                            li().child(strong().text(minutes(library::minutes(l, track))))
                                .text(tr(l, " de lecture")),
                        )
                        .child(
                            li().child(strong().text(examples(Some(track)).to_string()))
                                .text(tr(l, " exemples")),
                        ),
                )
                .child(
                    div()
                        .class("hero__actions")
                        .child(
                            a().class("btn btn--track")
                                .href(doc_href(l, track.slug, track.pages[0].slug, None))
                                .text(tr(l, "Commencer le parcours →")),
                        )
                        .child(if meta.site.1.starts_with('/') {
                            a().class("btn btn--ghost")
                                .href(Route::Compare.href(l))
                                .text(format!("{} →", meta.site.0))
                        } else {
                            a().class("btn btn--ghost")
                                .href(meta.site.1)
                                .attr("target", "_blank")
                                .attr("rel", "noopener")
                                .text(format!("{} ↗", meta.site.0))
                        }),
                ),
        )
        .children(parts(track).into_iter().map(|(name, range)| {
            section()
                .class("part")
                .child(h2().class("part__title").text(tr(l, name)))
                .child(
                    ol().class("chapters")
                        .attr("start", (range.start + 1).to_string())
                        .children(docs[range].iter().map(|d| {
                            li().class("chapter").child(
                                a().class("chapter__link")
                                    .href(doc_href(l, track.slug, d.slug, None))
                                    .attr("data-key", format!("{}/{}", track.slug, d.slug))
                                    .child(span().class("chapter__num").text(format!("{:02}", d.index + 1)))
                                    .child(
                                        span()
                                            .class("chapter__body")
                                            .child(span().class("chapter__title").text(d.title.clone()))
                                            .child(span().class("chapter__desc").text(d.description.clone())),
                                    )
                                    .child(span().class("chapter__time").text(minutes(d.minutes)))
                                    .child(fr_badge(l, d)),
                            )
                        })),
                )
        }))
        .child(
            section()
                .class("section others")
                .attr("aria-label", tr(l, "Les autres parcours"))
                .child(h2().text(tr(l, "Les autres parcours")))
                .child(
                    ul().class("tracks tracks--small")
                        .children(TRACKS.iter().filter(|t| t.slug != track.slug).map(|t| track_card(l, t))),
                ),
        );
    layout(ctx, meta.title, meta.description, "page-track", content)
}

// ----- Un chapitre -----------------------------------------------------------------

/// « FR » après le titre d'un chapitre pas encore traduit.
fn fr_badge(l: Lang, doc: &Doc) -> Element {
    if doc.translated {
        return span();
    }
    span()
        .class("fr-badge")
        .attr("title", tr(l, "Pas encore traduit"))
        .attr("lang", "fr")
        .text("FR")
}

fn sidebar(doc: &Doc) -> Element {
    let track = doc.track;
    let l = doc.lang;
    aside()
        .class("sidebar")
        .id("sidebar")
        .attr("aria-label", i18n::chapters_of(l, name(l, track)))
        .child(
            div()
                .class("sidebar__inner")
                .child(
                    a().class("sidebar__track")
                        .href(Route::Track(track).href(l))
                        .child(
                            span()
                                .class("track__badge")
                                .attr("aria-hidden", "true")
                                .text(badge(track)),
                        )
                        .child(span().text(name(l, track))),
                )
                .child(
                    div()
                        .class("meter")
                        .attr("role", "img")
                        .attr("aria-label", i18n::chapter_of(l, doc.index + 1, track.pages.len()))
                        .child(
                            div()
                                .class("meter__fill")
                                .style(format!("width:{}%", (doc.index + 1) * 100 / track.pages.len())),
                        ),
                )
                .child(
                    ol().class("sidebar__list")
                        .children(library::of(l, track).flat_map(|d| {
                            let part = track.part_at(d.index).map(|name| {
                                li().class("sidebar__part")
                                    .attr("role", "presentation")
                                    .text(tr(l, name))
                            });
                            let item = a()
                                .href(doc_href(l, track.slug, d.slug, None))
                                .attr("data-key", format!("{}/{}", track.slug, d.slug))
                                .child(span().class("sidebar__num").text(format!("{}", d.index + 1)))
                                .child(span().text(d.title.clone()))
                                .child(fr_badge(l, d));
                            let item = li().child(if d.slug == doc.slug {
                                item.class("is-current").attr("aria-current", "page")
                            } else {
                                item
                            });
                            part.into_iter().chain(std::iter::once(item))
                        })),
                )
                .child(
                    div()
                        .class("sidebar__switch")
                        .child(p().class("sidebar__label").text(if track.language {
                            tr(l, "Autres langages")
                        } else {
                            tr(l, "Les langages")
                        }))
                        .children(languages().filter(|t| t.slug != track.slug).map(|t| {
                            let href = match library::get(l, t.slug, doc.slug) {
                                Some(d) => doc_href(l, t.slug, d.slug, None),
                                None => Route::Track(t).href(l),
                            };
                            a().class("chip").class(format!("t-{}", t.slug)).href(href).text(t.name)
                        })),
                ),
        )
}

fn toc(doc: &Doc) -> Element {
    let l = doc.lang;
    aside().class("toc").attr("aria-label", tr(l, "Sur cette page")).child(
        div()
            .class("toc__inner")
            .child(p().class("toc__title").text(tr(l, "Sur cette page")))
            .child(ul().children(doc.toc.iter().map(|h| {
                li().class(format!("toc__l{}", h.level)).child(
                    a().href(format!("#{}", h.id))
                        .attr("data-toc", h.id.clone())
                        .text(h.text.clone()),
                )
            })))
            .child(a().class("toc__top").href("#main").text(tr(l, "↑ Haut de page"))),
    )
}

fn pager(doc: &Doc) -> Element {
    let track = doc.track;
    let l = doc.lang;
    let card = |d: &Doc, rel: &str, label: &str| {
        a().class(format!("pager__card pager__card--{rel}"))
            .href(doc_href(l, track.slug, d.slug, None))
            .attr("rel", rel.to_owned())
            .child(span().class("pager__label").text(label.to_owned()))
            .child(span().class("pager__title").text(d.title.clone()))
    };
    let docs: Vec<&Doc> = library::of(l, track).collect();
    let mut nav = nav()
        .class("pager")
        .attr("aria-label", tr(l, "Chapitre précédent et suivant"));
    if doc.index > 0 {
        nav = nav.child(card(docs[doc.index - 1], "prev", tr(l, "← Précédent")));
    } else {
        nav = nav.child(span());
    }
    if let Some(next) = docs.get(doc.index + 1) {
        nav = nav.child(card(next, "next", tr(l, "Suivant →")));
    } else {
        let (href, title) = if track.language {
            (Route::Compare.href(l), tr(l, "Comparer les langages"))
        } else {
            (lang_link(l, "/#parcours"), tr(l, "Choisir un langage"))
        };
        nav = nav.child(
            a().class("pager__card pager__card--next")
                .href(href)
                .child(span().class("pager__label").text(tr(l, "Et ensuite ? →")))
                .child(span().class("pager__title").text(title)),
        );
    }
    nav
}

fn doc_page(ctx: &Ctx, track: &'static Track, index: usize) -> Page {
    let l = ctx.lang;
    let slug = track.pages[index].slug;
    let doc = library::get(l, track.slug, slug).expect("chaque page du catalogue est rendue");
    let others: Vec<Element> = languages()
        .filter(|t| track.language && t.slug != track.slug)
        .filter_map(|t| library::get(l, t.slug, slug).map(|d| (t, d)))
        .map(|(t, d)| {
            a().class("chip")
                .class(format!("t-{}", t.slug))
                .href(doc_href(l, t.slug, d.slug, None))
                .text(format!("{} : {}", t.name, d.title))
        })
        .collect();
    let mut article = article()
        .class("doc")
        .child(
            nav()
                .class("crumbs")
                .attr("aria-label", tr(l, "Fil d'Ariane"))
                .child(a().href(ctx.link("/")).text(tr(l, "Accueil")))
                .child(a().href(Route::Track(track).href(l)).text(name(l, track)))
                .child(span().attr("aria-current", "page").text(doc.title.clone())),
        )
        .child(
            header()
                .class("doc__head")
                .child(p().class("eyebrow").text(format!(
                    "{} · {} · {}",
                    name(l, track),
                    i18n::chapter_of(l, index + 1, track.pages.len()),
                    i18n::reading(l, doc.minutes)
                )))
                .child(h1().text(doc.title.clone())),
        );
    if !doc.translated {
        article = article.child(
            aside()
                .class("callout callout--note untranslated")
                .child(p().text(i18n::untranslated(l))),
        );
    }
    article = article.child(
        div()
            .class("prose")
            .html(doc.html.clone())
            .attr("lang", if doc.translated { l.code() } else { "fr" }),
    );
    if !others.is_empty() {
        article = article.child(
            aside()
                .class("also")
                .attr("aria-label", tr(l, "Le même chapitre dans les autres langages"))
                .child(
                    p().class("also__title")
                        .text(tr(l, "Le même chapitre dans les autres langages")),
                )
                .child(div().class("also__links").children(others)),
        );
    }
    article = article
        .child(
            div().class("done").child(
                button()
                    .class("done__btn")
                    .attr("type", "button")
                    .attr("data-done", format!("{}/{}", track.slug, slug))
                    .attr("aria-pressed", "false")
                    .attr("hidden", "")
                    .child(span().class("done__icon").attr("aria-hidden", "true").text("✓"))
                    .child(span().class("done__text").text(tr(l, "Marquer ce chapitre comme lu"))),
            ),
        )
        .child(pager(doc))
        .child(p().class("edit").child(external(
            &format!(
                "{REPO}/edit/master/content/{}{}/{}.md",
                if doc.translated && l != Lang::Fr {
                    format!("{}/", l.code())
                } else {
                    String::new()
                },
                track.slug,
                slug
            ),
            tr(l, "Une erreur, une idée ? Modifier cette page sur GitHub"),
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
        info(l, track).description.to_owned()
    } else {
        doc.description.clone()
    };
    layout(
        ctx,
        &format!("{} – {}", doc.title, name(l, track)),
        &description,
        "page-doc",
        content,
    )
}

// ----- Introuvable -----------------------------------------------------------------

pub fn not_found(ctx: &Ctx) -> Page {
    let l = ctx.lang;
    let content = main()
        .class("not-found")
        .child(p().class("big-404").attr("aria-hidden", "true").text("404"))
        .child(h1().text(tr(l, "Cette page n'existe pas")))
        .child(p().class("lead").text(tr(
            l,
            "Elle a peut-être changé d'adresse. Choisissez un parcours, ou cherchez ce qu'il vous faut.",
        )))
        .child(
            ul().class("tracks tracks--small")
                .children(TRACKS.iter().map(|t| track_card(l, t))),
        );
    layout(
        ctx,
        tr(l, "Page introuvable"),
        tr(l, "Cette page n'existe pas."),
        "page-404",
        content,
    )
    .status(404)
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
