//! Les parcours (Débuter, puis un par langage) et leurs pages, dans l'ordre de
//! lecture.
//! Le texte des pages est dans `content/{parcours}/{page}.md`, compilé dans le serveur :
//! rien à copier pour le déployer.

/// Une page : son adresse dans le parcours et son texte Markdown.
pub struct Source {
    pub slug: &'static str,
    pub markdown: &'static str,
}

/// Un langage et ses pages.
pub struct Track {
    /// `debuter`, `ruby`, `rust`, `rythme` : l'adresse du parcours (`/ruby/`).
    pub slug: &'static str,
    pub name: &'static str,
    /// Le titre de la page du parcours.
    pub title: &'static str,
    /// Un langage (Ruby, Rust, Rythme), pas le parcours « Débuter ».
    pub language: bool,
    /// Une phrase sous le nom, sur l'accueil.
    pub tagline: &'static str,
    /// La description du parcours (page du parcours, moteurs de recherche).
    pub description: &'static str,
    /// L'extension des fichiers de code.
    pub ext: &'static str,
    /// Le nom du site officiel et son adresse.
    pub site: (&'static str, &'static str),
    pub pages: &'static [Source],
    /// Les parties du parcours : (position de leur premier chapitre, nom).
    pub parts: &'static [(usize, &'static str)],
}

macro_rules! pages {
    ($track:literal: $($slug:literal),* $(,)?) => {
        &[$(Source {
            slug: $slug,
            markdown: include_str!(concat!("../content/", $track, "/", $slug, ".md")),
        }),*]
    };
}

pub static TRACKS: [Track; 8] = [
    Track {
        slug: "debuter",
        name: "Débuter",
        title: "Débuter en programmation",
        language: false,
        tagline: "Jamais programmé ? Tout commence ici.",
        description: "Pour celles et ceux qui n'ont jamais programmé : ce qu'est un programme, penser un algorithme, le terminal, un éditeur de code, corriger ses erreurs, Git, quel langage choisir, et un glossaire.",
        ext: "txt",
        site: ("Les langages côte à côte", "/comparer/"),
        pages: pages!("debuter":
            "programmer",
            "ordinateur",
            "algorithmes",
            "variables",
            "logique",
            "boucles",
            "fonctions",
            "donnees",
            "lire-une-erreur",
            "bloque",
            "terminal",
            "editeur",
            "deboguer",
            "git",
            "web",
            "bases-de-donnees",
            "securite",
            "apprendre",
            "choisir",
            "open-source",
            "metiers",
            "glossaire",
        ),
        parts: &[
            (0, "Comprendre"),
            (3, "Les notions de base"),
            (10, "Les outils"),
            (14, "Et après"),
        ],
    },
    Track {
        slug: "python",
        name: "Python",
        title: "Apprendre Python",
        language: true,
        tagline: "Le langage le plus utilisé pour apprendre, et bien plus.",
        description: "Apprenez Python pas à pas : variables, conditions, fonctions, texte, listes et dictionnaires, compréhensions, classes, annotations de type, générateurs, décorateurs, erreurs, modules, fichiers, JSON, regex, dates, asyncio, web, tests, projets guidés et exercices corrigés.",
        ext: "py",
        site: ("python.org", "https://www.python.org/"),
        pages: pages!("python":
            "introduction",
            "premiers-pas",
            "nombres",
            "conditions-boucles",
            "fonctions",
            "texte",
            "collections",
            "dictionnaires",
            "comprehensions",
            "pieges",
            "exercices-faciles",
            "types",
            "heritage",
            "typage",
            "iterateurs",
            "decorateurs",
            "erreurs",
            "contextes",
            "modules",
            "fichiers",
            "formats",
            "sqlite",
            "regex",
            "dates",
            "ligne-de-commande",
            "concurrence",
            "async",
            "web",
            "tests",
            "debogage",
            "style",
            "projet",
            "projets",
            "exercices",
            "aide-memoire",
        ),
        parts: &[
            (0, "Les bases"),
            (11, "Aller plus loin"),
            (19, "Le monde réel"),
            (28, "Bien programmer"),
        ],
    },
    Track {
        slug: "javascript",
        name: "JavaScript",
        title: "Apprendre JavaScript",
        language: true,
        tagline: "Le langage du web, dans le navigateur et sur le serveur.",
        description: "Apprenez JavaScript pas à pas : variables, conditions, fonctions, texte, tableaux, objets, classes, closures, itérateurs, erreurs, modules et npm, promesses et async/await, le DOM, fichiers avec Node, JSON, regex, dates, web, tests, projets guidés et exercices corrigés.",
        ext: "js",
        site: (
            "developer.mozilla.org",
            "https://developer.mozilla.org/fr/docs/Web/JavaScript",
        ),
        pages: pages!("javascript":
            "introduction",
            "premiers-pas",
            "nombres",
            "conditions-boucles",
            "fonctions",
            "texte",
            "collections",
            "objets",
            "pieges",
            "exercices-faciles",
            "types",
            "heritage",
            "closures",
            "iterateurs",
            "tableaux-avances",
            "erreurs",
            "modules",
            "asynchrone",
            "dom",
            "fichiers",
            "formats",
            "regex",
            "dates",
            "ligne-de-commande",
            "web",
            "tests",
            "debogage",
            "style",
            "projet",
            "projets",
            "exercices",
            "aide-memoire",
        ),
        parts: &[
            (0, "Les bases"),
            (10, "Aller plus loin"),
            (18, "Le monde réel"),
            (25, "Bien programmer"),
        ],
    },
    Track {
        slug: "typescript",
        name: "TypeScript",
        title: "Apprendre TypeScript",
        language: true,
        tagline: "JavaScript avec des types : les erreurs trouvées avant l'exécution.",
        description: "Apprenez TypeScript pas à pas : types de base, fonctions, tableaux et tuples, interfaces, unions et rétrécissement, classes, génériques, types littéraux, types avancés, erreurs, modules, async, tsconfig, migration depuis JavaScript, web, tests, projets guidés et exercices corrigés.",
        ext: "ts",
        site: ("typescriptlang.org", "https://www.typescriptlang.org/"),
        pages: pages!("typescript":
            "introduction",
            "premiers-pas",
            "nombres",
            "conditions-boucles",
            "fonctions",
            "texte",
            "collections",
            "objets",
            "pieges",
            "exercices-faciles",
            "unions",
            "types",
            "generiques",
            "litteraux",
            "types-avances",
            "erreurs",
            "modules",
            "asynchrone",
            "configuration",
            "migrer",
            "fichiers",
            "formats",
            "regex",
            "web",
            "tests",
            "debogage",
            "style",
            "projet",
            "projets",
            "exercices",
            "aide-memoire",
        ),
        parts: &[
            (0, "Les bases"),
            (10, "Les types en profondeur"),
            (17, "Le monde réel"),
            (24, "Bien programmer"),
        ],
    },
    Track {
        slug: "go",
        name: "Go",
        title: "Apprendre Go",
        language: true,
        tagline: "Simple, rapide, fait pour les serveurs et les outils.",
        description: "Apprenez Go pas à pas : variables, conditions, fonctions, texte, slices et maps, structures et méthodes, interfaces, pointeurs, génériques, erreurs, packages, goroutines et canaux, fichiers, JSON, regex, dates, web avec net/http, tests, projets guidés et exercices corrigés.",
        ext: "go",
        site: ("go.dev", "https://go.dev/"),
        pages: pages!("go":
            "introduction",
            "premiers-pas",
            "nombres",
            "conditions-boucles",
            "fonctions",
            "texte",
            "collections",
            "maps",
            "pieges",
            "exercices-faciles",
            "types",
            "interfaces",
            "pointeurs",
            "generiques",
            "erreurs",
            "modules",
            "concurrence",
            "canaux",
            "context",
            "fichiers",
            "embed",
            "formats",
            "regex",
            "dates",
            "ligne-de-commande",
            "web",
            "tests",
            "debogage",
            "style",
            "projet",
            "projets",
            "exercices",
            "aide-memoire",
        ),
        parts: &[
            (0, "Les bases"),
            (10, "Aller plus loin"),
            (16, "Le monde réel"),
            (26, "Bien programmer"),
        ],
    },
    Track {
        slug: "ruby",
        name: "Ruby",
        title: "Apprendre Ruby",
        language: true,
        tagline: "Le langage pensé pour le bonheur des développeurs.",
        description: "Apprenez Ruby pas à pas : variables, conditions, méthodes, texte, tableaux et hashs, blocs, classes, modules, erreurs, fichiers, tests, gems, projets guidés et exercices corrigés.",
        ext: "rb",
        site: ("ruby-lang.org", "https://www.ruby-lang.org/fr/"),
        pages: pages!("ruby":
            "introduction",
            "premiers-pas",
            "nombres",
            "conditions-boucles",
            "fonctions",
            "texte",
            "collections",
            "intervalles",
            "pieges",
            "exercices-faciles",
            "blocs",
            "procs-lambdas",
            "enumerateurs",
            "types",
            "heritage",
            "modules",
            "structures",
            "comparaison",
            "pattern-matching",
            "erreurs",
            "fichiers",
            "formats",
            "base-de-donnees",
            "regex",
            "dates",
            "ligne-de-commande",
            "concurrence",
            "web",
            "tests",
            "debogage",
            "style",
            "metaprogrammation",
            "projet",
            "projets",
            "exercices",
            "aide-memoire",
        ),
        parts: &[
            (0, "Les bases"),
            (10, "Aller plus loin"),
            (20, "Le monde réel"),
            (28, "Bien programmer"),
        ],
    },
    Track {
        slug: "rust",
        name: "Rust",
        title: "Apprendre Rust",
        language: true,
        tagline: "Rapide, fiable, et un compilateur qui vous aide.",
        description: "Apprenez Rust pas à pas : variables, fonctions, texte, collections, possession et emprunt, structures, traits, erreurs, itérateurs, modules, fichiers, Box et Rc, threads, tests, Cargo, projets guidés et exercices corrigés.",
        ext: "rs",
        site: ("rust-lang.org", "https://www.rust-lang.org/fr"),
        pages: pages!("rust":
            "introduction",
            "premiers-pas",
            "nombres",
            "conditions-boucles",
            "fonctions",
            "texte",
            "collections",
            "hashmap",
            "pieges",
            "exercices-faciles",
            "possession",
            "durees-de-vie",
            "types",
            "pattern-matching",
            "generiques",
            "traits-objets",
            "traits-standard",
            "erreurs",
            "closures",
            "iterateurs",
            "modules",
            "fichiers",
            "pointeurs",
            "concurrence",
            "async",
            "macros",
            "unsafe",
            "ligne-de-commande",
            "formats",
            "web",
            "performance",
            "tests",
            "debogage",
            "style",
            "projet",
            "projets",
            "exercices",
            "aide-memoire",
        ),
        parts: &[
            (0, "Les bases"),
            (10, "Le cœur de Rust"),
            (20, "Aller plus loin"),
            (27, "Le monde réel"),
            (31, "Bien programmer"),
        ],
    },
    Track {
        slug: "rythme",
        name: "Rythme",
        title: "Apprendre Rythme",
        language: true,
        tagline: "Plus simple que Python, aussi agréable que Ruby, sûr comme Rust.",
        description: "Apprenez Rythme pas à pas : valeurs, fonctions, texte, listes et enregistrements, style fonctionnel, types et match, erreurs, modules, fichiers, tests, projets guidés et exercices corrigés.",
        ext: "ry",
        site: ("Rythme sur GitHub", "https://github.com/maxlestage/Rythme"),
        pages: pages!("rythme":
            "introduction",
            "premiers-pas",
            "nombres",
            "conditions-boucles",
            "fonctions",
            "texte",
            "collections",
            "dictionnaires",
            "pieges",
            "exercices-faciles",
            "fonctionnel",
            "recursion",
            "types",
            "symboles",
            "pattern-matching",
            "erreurs",
            "modules",
            "fichiers",
            "formats",
            "regex",
            "dates",
            "ligne-de-commande",
            "verificateur",
            "etapes",
            "tests",
            "style",
            "venir-de-ruby",
            "projet",
            "projets",
            "exercices",
            "aide-memoire",
        ),
        parts: &[
            (0, "Les bases"),
            (10, "Aller plus loin"),
            (17, "Le monde réel"),
            (22, "Bien programmer"),
        ],
    },
];

include!(concat!(env!("OUT_DIR"), "/translations.rs"));

/// Le texte de la page `page` du parcours `track` traduit en `lang`, s'il l'est.
pub fn translation(lang: crate::i18n::Lang, track: &str, page: &str) -> Option<&'static str> {
    TRANSLATIONS
        .iter()
        .find(|(l, t, p, _)| *l == lang.code() && *t == track && *p == page)
        .map(|(_, _, _, markdown)| *markdown)
}

/// Les langages, sans le parcours « Débuter ».
pub fn languages() -> impl Iterator<Item = &'static Track> {
    TRACKS.iter().filter(|t| t.language)
}

impl Track {
    /// Le nom de la partie qui commence au chapitre `index`, s'il en commence une.
    pub fn part_at(&self, index: usize) -> Option<&'static str> {
        self.parts.iter().find(|(at, _)| *at == index).map(|(_, name)| *name)
    }
}

/// Le parcours `slug`.
pub fn track(slug: &str) -> Option<&'static Track> {
    TRACKS.iter().find(|t| t.slug == slug)
}

/// La page `page` du parcours `track`, avec sa position.
pub fn find(track: &str, page: &str) -> Option<(&'static Track, usize)> {
    let track = self::track(track)?;
    let at = track.pages.iter().position(|p| p.slug == page)?;
    Some((track, at))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_track_has_its_pages() {
        assert_eq!(languages().count(), 7);
        for track in languages() {
            assert!(track.pages.len() >= 5, "{}", track.slug);
            assert_eq!(track.pages[0].slug, "introduction");
            assert!(track.pages.iter().all(|p| !p.markdown.trim().is_empty()));
        }
        assert_eq!(find("rust", "possession").map(|(t, i)| (t.name, i)), Some(("Rust", 10)));
        assert!(find("rust", "blocs").is_none());
        for track in &TRACKS {
            assert_eq!(track.parts.first().map(|p| p.0), Some(0), "{}", track.slug);
            assert!(track.parts.iter().all(|(at, _)| *at < track.pages.len()));
        }
        assert!(find("python", "introduction").is_some());
        assert!(find("cobol", "introduction").is_none());
        // Chaque traduction est celle d'une page du catalogue.
        for (lang, track, page, _) in TRANSLATIONS {
            assert!(find(track, page).is_some(), "content/{lang}/{track}/{page}.md");
        }
    }
}
