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
| [Débuter](content/debuter) (11) | **Comprendre** : Qu'est-ce que programmer ?, Comment marche un ordinateur, Penser comme un programmeur · **Les outils** : Le terminal en 10 minutes, Installer un éditeur de code, Trouver et corriger ses erreurs, Sauvegarder son travail avec Git · **Et après** : Comment marche le web, Bien apprendre la programmation, Quel langage choisir ?, Glossaire |
| [Ruby](content/ruby) (33) | **Les bases** : Ruby : introduction et installation, Premiers pas, Nombres et calculs, Conditions et boucles, Les méthodes, Travailler avec du texte, Tableaux et hashs, Les intervalles · **Aller plus loin** : Blocs, itérateurs et Enumerable, Procs, lambdas et méthodes, Énumérateurs et paresse, Classes et objets, Héritage et polymorphisme, Modules et mixins, Struct, Data et Set, Égalité, comparaison et tri, Le filtrage par motif (case/in), Gérer les erreurs · **Le monde réel** : Lire et écrire : clavier et fichiers, JSON, CSV et YAML, Les expressions régulières, Dates et heures, Écrire un outil en ligne de commande, Faire plusieurs choses à la fois, Ruby pour le web · **Bien programmer** : Tester son code, Déboguer en Ruby, Bien écrire du Ruby, Métaprogrammation en douceur, Organiser un projet : fichiers, gems et Bundler, Projets pas à pas, Exercices corrigés, Aide-mémoire |
| [Rust](content/rust) (35) | **Les bases** : Rust : introduction et installation, Premiers pas, Nombres et calculs, Conditions et boucles, Les fonctions, Travailler avec du texte, Collections, HashMap, HashSet et BTreeMap · **Le cœur de Rust** : Possession et emprunt, Les durées de vie, Structures, énumérations et traits, Le filtrage par motif, Génériques et traits, Les traits de la bibliothèque standard, Gérer les erreurs, Les closures en détail, Les itérateurs · **Aller plus loin** : Modules et crates, Lire et écrire : clavier et fichiers, Box, Rc et RefCell, Faire plusieurs choses à la fois, La programmation asynchrone, Les macros, unsafe et les règles qu'on peut enfreindre · **Le monde réel** : Écrire un outil en ligne de commande, JSON, CSV et TOML avec serde, Rust pour le web, Écrire du code rapide · **Bien programmer** : Tester son code, Déboguer en Rust, Bien écrire du Rust, Organiser un projet avec Cargo, Projets pas à pas, Exercices corrigés, Aide-mémoire |
| [Rythme](content/rythme) (28) | **Les bases** : Rythme : introduction et installation, Premiers pas, Nombres et calculs, Conditions et répétition, Les fonctions, Travailler avec du texte, Listes, enregistrements et dictionnaires, Dictionnaires et ensembles · **Aller plus loin** : Penser fonctionnel, La récursion, Vos propres types et match, Le filtrage par motif, Gérer les erreurs, Modules : un programme en plusieurs fichiers · **Le monde réel** : Lire et écrire : clavier et fichiers, Données JSON, Les expressions régulières, Les dates, Écrire un outil en ligne de commande · **Bien programmer** : Le vérificateur et les messages d'erreur, Comprendre un calcul étape par étape, Tester son code, Bien écrire du Rythme, Venir de Ruby (ou de Python), Organiser un projet : rythme new, fmt, doc et éditeurs, Projets pas à pas, Exercices corrigés, Aide-mémoire |

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
ruby tools/suite.rb          # relie chaque chapitre au suivant (« Suite : »)
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
