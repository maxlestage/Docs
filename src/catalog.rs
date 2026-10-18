//! Les parcours (Débuter, puis Ruby, Rust et Rythme) et leurs pages, dans l'ordre de
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
}

macro_rules! pages {
    ($track:literal: $($slug:literal),* $(,)?) => {
        &[$(Source {
            slug: $slug,
            markdown: include_str!(concat!("../content/", $track, "/", $slug, ".md")),
        }),*]
    };
}

pub static TRACKS: [Track; 4] = [
    Track {
        slug: "debuter",
        name: "Débuter",
        title: "Débuter en programmation",
        language: false,
        tagline: "Jamais programmé ? Tout commence ici.",
        description: "Pour celles et ceux qui n'ont jamais programmé : ce qu'est un programme, le terminal, un éditeur de code, quel langage choisir, et un glossaire des mots du métier.",
        ext: "txt",
        site: ("Les trois langages côte à côte", "/comparer/"),
        pages: pages!("debuter": "programmer", "terminal", "editeur", "choisir", "glossaire"),
    },
    Track {
        slug: "ruby",
        name: "Ruby",
        title: "Apprendre Ruby",
        language: true,
        tagline: "Le langage pensé pour le bonheur des développeurs.",
        description: "Apprenez Ruby pas à pas : variables, conditions, méthodes, tableaux, blocs, classes, erreurs et gems, avec des exemples vérifiés.",
        ext: "rb",
        site: ("ruby-lang.org", "https://www.ruby-lang.org/fr/"),
        pages: pages!("ruby":
            "introduction",
            "premiers-pas",
            "conditions-boucles",
            "fonctions",
            "texte",
            "collections",
            "blocs",
            "types",
            "erreurs",
            "fichiers",
            "projet",
            "exercices",
            "aide-memoire",
        ),
    },
    Track {
        slug: "rust",
        name: "Rust",
        title: "Apprendre Rust",
        language: true,
        tagline: "Rapide, fiable, et un compilateur qui vous aide.",
        description: "Apprenez Rust pas à pas : variables, fonctions, collections, possession et emprunt, structures, traits, erreurs et Cargo, avec des exemples vérifiés.",
        ext: "rs",
        site: ("rust-lang.org", "https://www.rust-lang.org/fr"),
        pages: pages!("rust":
            "introduction",
            "premiers-pas",
            "conditions-boucles",
            "fonctions",
            "texte",
            "collections",
            "possession",
            "types",
            "generiques",
            "erreurs",
            "fichiers",
            "projet",
            "exercices",
            "aide-memoire",
        ),
    },
    Track {
        slug: "rythme",
        name: "Rythme",
        title: "Apprendre Rythme",
        language: true,
        tagline: "Plus simple que Python, aussi agréable que Ruby, sûr comme Rust.",
        description: "Apprenez Rythme pas à pas : valeurs, fonctions, listes et enregistrements, style fonctionnel, types, match, erreurs et tests, avec des exemples vérifiés.",
        ext: "ry",
        site: ("Rythme sur GitHub", "https://github.com/maxlestage/Rythme"),
        pages: pages!("rythme":
            "introduction",
            "premiers-pas",
            "conditions-boucles",
            "fonctions",
            "texte",
            "collections",
            "fonctionnel",
            "types",
            "erreurs",
            "fichiers",
            "projet",
            "exercices",
            "aide-memoire",
        ),
    },
];

/// Les trois langages, sans le parcours « Débuter ».
pub fn languages() -> impl Iterator<Item = &'static Track> {
    TRACKS.iter().filter(|t| t.language)
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
        assert_eq!(languages().count(), 3);
        for track in languages() {
            assert!(track.pages.len() >= 5, "{}", track.slug);
            assert_eq!(track.pages[0].slug, "introduction");
            assert!(track.pages.iter().all(|p| !p.markdown.trim().is_empty()));
        }
        assert_eq!(find("rust", "possession").map(|(t, i)| (t.name, i)), Some(("Rust", 6)));
        assert!(find("rust", "blocs").is_none());
        assert!(find("python", "introduction").is_none());
    }
}
