# 📚 Docs : Ruby, Rust et Rythme

**Des documentations simples, en français, pour apprendre Ruby, Rust et Rythme.**
Un parcours pour débuter, puis un parcours par langage, un exemple pour chaque idée, et du code qui marche
vraiment : chacun des exemples est exécuté avant d'être publié.

Le site est écrit en Rust avec [Paquets](https://github.com/maxlestage/Paquets) : un
seul programme sert les pages (rendues sur le serveur, avec leurs balises SEO et une
Content-Security-Policy stricte), la recherche, le plan du site, et l'application
installable qui marche hors ligne.

| Parcours | Chapitres |
|---|---|
| [Débuter](content/debuter) (8) | qu'est-ce que programmer, penser un algorithme, le terminal, un éditeur, trouver ses erreurs, Git, quel langage choisir, glossaire |
| [Ruby](content/ruby) (16) | **Les bases** : introduction, premiers pas, conditions et boucles, méthodes, texte, tableaux et hashs · **Aller plus loin** : blocs, classes, modules, erreurs, fichiers · **La pratique** : tests, organiser un projet, projets pas à pas, exercices, aide-mémoire |
| [Rust](content/rust) (20) | **Les bases** : introduction, premiers pas, conditions et boucles, fonctions, texte, collections · **Le cœur de Rust** : possession, structures et énumérations, génériques et traits, erreurs, itérateurs · **Aller plus loin** : modules, fichiers, Box/Rc/RefCell, threads · **La pratique** : tests, Cargo, projets pas à pas, exercices, aide-mémoire |
| [Rythme](content/rythme) (16) | **Les bases** : introduction, premiers pas, conditions et répétition, fonctions, texte, listes et enregistrements · **Aller plus loin** : penser fonctionnel, types et `match`, erreurs, modules, fichiers · **La pratique** : tests, organiser un projet, projets pas à pas, exercices, aide-mémoire |

Chaque chapitre suit le même plan : objectifs, sections avec un exemple chacune, erreurs
fréquentes (avec les vrais messages d'erreur), « Testez-vous » (réponses repliées) et
résumé. Chaque langage a 3 projets guidés, 12 exercices corrigés et un aide-mémoire.

## Ce que fait le site

- **Une page par chapitre**, avec le sommaire des chapitres à gauche, celui de la page
  à droite (qui suit la lecture), une barre de progression, le chapitre précédent et
  suivant, et le même chapitre dans les deux autres langages.
- **Comparer** (`/comparer/`) : vingt idées (afficher, nommer, choisir, répéter,
  fonctions, listes, types, erreurs, tests…) écrites dans les trois langages, côte à
  côte, chacune liée à son chapitre.
- **Recherche instantanée** : touche `/` ou `Ctrl+K`, dans les titres, les sections et le
  texte de toutes les pages, filtrable par langage, au clavier.
- **Code coloré sur le serveur** (aucune bibliothèque dans le navigateur), avec un
  bouton « Copier » et la sortie du programme juste en dessous.
- **Encadrés** (Astuce, À retenir, Attention, Note), tableaux qui défilent sur
  téléphone.
- **Chapitres lus** : un bouton « Marquer ce chapitre comme lu », des coches dans les
  sommaires et la progression de chaque parcours, retenus dans le navigateur.
- **Exercices corrigés** : indices, solutions repliées sous « Voir la solution ».
- **Clair ou sombre**, mobile d'abord, installable (PWA) et lisible hors ligne,
  imprimable.

## Lancer le site

```sh
cargo run --release          # http://localhost:3000 (PORT sur Heroku)
```

Les pages sont compilées dans le programme (`include_str!`) : seul `public/` (styles,
script, logo) doit l'accompagner. `SITE_URL` donne l'adresse publique (pour les liens
canoniques), prise de chaque requête sinon.

## Écrire ou corriger une page

Les pages sont dans `content/{ruby,rust,rythme}/`, en Markdown ; leur ordre est dans
[`src/catalog.rs`](src/catalog.rs). Les règles d'écriture (titre, résumé, code exécuté,
sorties vérifiées, encadrés, liens) sont dans [`content/README.md`](content/README.md).

```sh
ruby tools/verifier.rb       # exécute chaque exemple et compare sa sortie
cargo test                   # chaque page, chaque lien et chaque ancre
```

Le vérificateur a besoin de `ruby` (3.3), `rustc` et `rythme`
([installer Rythme](https://github.com/maxlestage/Rythme#installer)).

## Déployer

- **Heroku** : `heroku create --stack heroku-26 --buildpack emk/rust` puis
  `git push heroku master` ([`RustConfig`](RustConfig), [`heroku-build.sh`](heroku-build.sh),
  [`Procfile`](Procfile), [`app.json`](app.json)).
- **Docker** : `docker build -t docs . && docker run --rm -p 8080:8080 docs`.
- **GitHub Pages** : chaque fusion dans `master` est publiée par
  [`pages.yml`](.github/workflows/pages.yml) (une seule fois : *Settings → Pages →
  Source : GitHub Actions*). Le site est exporté par le serveur lui-même
  (`BASE_PATH=/Docs cargo run --release -- export dist`), puis ses liens sont vérifiés.

Si des dépôts dont dépend Paquets sont privés, donnez un jeton GitHub qui peut les lire
dans le secret (ou la variable Heroku) `REPOS_TOKEN`.

## Intégration continue

[`ci.yml`](.github/workflows/ci.yml), à chaque envoi et chaque pull request :

| Tâche | Ce qu'elle vérifie |
|---|---|
| Rust | `cargo fmt`, `clippy`, `cargo test` : chaque page, chaque lien, chaque ancre |
| Exemples | chaque morceau de code, exécuté avec Ruby 3.3, Rust et Rythme, et sa sortie |
| Export statique et navigateur | l'export comme sur GitHub Pages, chacun de ses liens (`tools/liens.rb`), puis chaque page dans Chromium, sur téléphone et sur ordinateur, en clair et en sombre, avec la recherche, le thème, le menu et « Copier » (`tests/navigateur.js`) |
| Serveur | construit comme sur Heroku, démarré, interrogé |
| Docker | l'image, construite, démarrée, interrogée |

## Organisation

| Dossier | Contenu |
|---|---|
| `content/` | les pages, en Markdown |
| `src/catalog.rs` | les parcours et l'ordre de leurs pages |
| `src/markdown.rs` | Markdown → HTML : ancres, sommaire, code, sorties, encadrés, liens |
| `src/highlight.rs` | les couleurs du code (Ruby, Rust, Rythme, shell, TOML) |
| `src/pages.rs` | les pages : accueil, parcours, chapitre, comparer, 404 |
| `src/compare.rs` | les lignes de la page Comparer |
| `src/server.rs`, `src/routes.rs` | le serveur Paquets et les adresses |
| `src/export.rs` | l'export en fichiers statiques |
| `public/` | `site.css`, `site.js`, le logo |
| `tools/verifier.rb` | exécute chaque exemple de la documentation |
| `tests/site.rs` | le site entier, par le client de test de Paquets |
| `tests/navigateur.js` | le site dans Chromium |
| `tools/liens.rb` | chaque lien de l'export statique |
