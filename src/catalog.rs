//! Les trois parcours (Ruby, Rust, Rythme) et leurs pages, dans l'ordre de lecture.
//! Le texte des pages est dans `content/{parcours}/{page}.md`, compilé dans le serveur :
//! rien à copier pour le déployer.

/// Une page : son adresse dans le parcours et son texte Markdown.
pub struct Source {
    pub slug: &'static str,
    pub markdown: &'static str,
}

/// Un langage et ses pages.
pub struct Track {
    /// `ruby`, `rust`, `rythme` : l'adresse du parcours (`/ruby/`).
    pub slug: &'static str,
    pub name: &'static str,
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

pub static TRACKS: [Track; 3] = [
    Track {
        slug: "ruby",
        name: "Ruby",
        tagline: "Le langage pensé pour le bonheur des développeurs.",
        description: "Apprenez Ruby pas à pas : variables, conditions, méthodes, tableaux, blocs, classes, erreurs et gems, avec des exemples vérifiés.",
        ext: "rb",
        site: ("ruby-lang.org", "https://www.ruby-lang.org/fr/"),
        pages: pages!("ruby":
            "introduction",
            "premiers-pas",
            "conditions-boucles",
            "fonctions",
            "collections",
            "blocs",
            "types",
            "erreurs",
            "projet",
            "aide-memoire",
        ),
    },
    Track {
        slug: "rust",
        name: "Rust",
        tagline: "Rapide, fiable, et un compilateur qui vous aide.",
        description: "Apprenez Rust pas à pas : variables, fonctions, collections, possession et emprunt, structures, traits, erreurs et Cargo, avec des exemples vérifiés.",
        ext: "rs",
        site: ("rust-lang.org", "https://www.rust-lang.org/fr"),
        pages: pages!("rust":
            "introduction",
            "premiers-pas",
            "conditions-boucles",
            "fonctions",
            "collections",
            "possession",
            "types",
            "erreurs",
            "projet",
            "aide-memoire",
        ),
    },
    Track {
        slug: "rythme",
        name: "Rythme",
        tagline: "Plus simple que Python, aussi agréable que Ruby, sûr comme Rust.",
        description: "Apprenez Rythme pas à pas : valeurs, fonctions, listes et enregistrements, style fonctionnel, types, match, erreurs et tests, avec des exemples vérifiés.",
        ext: "ry",
        site: ("Rythme sur GitHub", "https://github.com/maxlestage/Rythme"),
        pages: pages!("rythme":
            "introduction",
            "premiers-pas",
            "conditions-boucles",
            "fonctions",
            "collections",
            "fonctionnel",
            "types",
            "erreurs",
            "projet",
            "aide-memoire",
        ),
    },
];

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
        for track in &TRACKS {
            assert!(track.pages.len() >= 5, "{}", track.slug);
            assert_eq!(track.pages[0].slug, "introduction");
            assert!(track.pages.iter().all(|p| !p.markdown.trim().is_empty()));
        }
        assert_eq!(find("rust", "possession").map(|(t, i)| (t.name, i)), Some(("Rust", 5)));
        assert!(find("rust", "blocs").is_none());
        assert!(find("python", "introduction").is_none());
    }
}
