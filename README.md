# 📚 Docs : apprendre à programmer

**Des documentations simples, en français, en anglais et en espagnol, pour apprendre Python, JavaScript, TypeScript, Go, Ruby, Rust et Rythme.**
Un parcours pour débuter, puis un parcours par langage, un exemple pour chaque idée, et du code qui marche
vraiment : chacun des exemples est exécuté avant d'être publié.

Le site est écrit en Rust avec [Paquets](https://github.com/maxlestage/Paquets) : un
seul programme sert les pages (rendues sur le serveur, avec leurs balises SEO et une
Content-Security-Policy stricte), la recherche, le plan du site, et l'application
installable qui marche hors ligne.

| Parcours | Chapitres |
|---|---|
| [Débuter](content/debuter) (22) | **Comprendre** : Qu'est-ce que programmer ?, Comment marche un ordinateur, Penser comme un programmeur · **Les notions de base** : Les variables, expliquées simplement, Vrai, faux et les conditions, Les boucles, pas à pas, Les fonctions, sans jargon, Les types de données : nombres, texte, listes, Lire un message d'erreur sans paniquer, Quand on bloque : une méthode pas à pas · **Les outils** : Le terminal en 10 minutes, Installer un éditeur de code, Trouver et corriger ses erreurs, Sauvegarder son travail avec Git · **Et après** : Comment marche le web, Qu'est-ce qu'une base de données ?, Les bases de la sécurité, Bien apprendre la programmation, Quel langage choisir ?, Contribuer à un projet open source, Les métiers du développement, Glossaire |
| [Python](content/python) (40) | **Les bases** : Python : introduction et installation, Premiers pas, Nombres et calculs, Conditions et boucles, Les fonctions, Travailler avec du texte, Listes et tuples, Dictionnaires et ensembles, Les compréhensions, Suivre un programme pas à pas, Les pièges du débutant en Python, Exercices pour bien démarrer · **Aller plus loin** : Classes et objets, Héritage et polymorphisme, Les annotations de type, Itérateurs et générateurs, Les collections avancées, Fonctions avancées et décorateurs, Gérer les erreurs, Les gestionnaires de contexte, Modules, paquets et pip · **Le monde réel** : Lire et écrire : clavier et fichiers, JSON et CSV, Une base de données avec sqlite3, Analyser des données, Les expressions régulières, Dates et heures, Écrire un outil en ligne de commande, Faire plusieurs choses à la fois, La programmation asynchrone, Python pour le web · **Bien programmer** : Tester son code, Déboguer en Python, Journaliser avec logging, Bien écrire du Python, Organiser un projet, Projets pas à pas, Exercices corrigés, Exercices corrigés : deuxième série, Aide-mémoire |
| [JavaScript](content/javascript) (37) | **Les bases** : JavaScript : introduction et installation, Premiers pas, Nombres et calculs, Conditions et boucles, Les fonctions, Travailler avec du texte, Les tableaux, Les objets, Suivre un programme pas à pas en JavaScript, Les pièges du débutant en JavaScript, Exercices pour bien démarrer · **Aller plus loin** : Les classes, Héritage et prototypes, Portée et closures, Itérateurs et générateurs, Les tableaux en profondeur, Gérer les erreurs, Modules et npm, La programmation asynchrone · **Le monde réel** : Le navigateur et le DOM, Dessiner dans le navigateur avec canvas, Garder des données dans le navigateur, Lire et écrire des fichiers avec Node, JSON et CSV, Les expressions régulières, Dates et heures, Écrire un outil en ligne de commande, JavaScript pour le web, Un serveur web avec node:http · **Bien programmer** : Tester son code, Déboguer en JavaScript, Bien écrire du JavaScript, Organiser un projet, Projets pas à pas, Exercices corrigés, Exercices corrigés : deuxième série, Aide-mémoire |
| [TypeScript](content/typescript) (36) | **Les bases** : TypeScript : introduction et installation, Premiers pas, Nombres et calculs, Conditions et boucles, Les fonctions, Travailler avec du texte, Tableaux et tuples, Objets, interfaces et alias de type, Suivre un programme pas à pas, Les pièges du débutant en TypeScript, Exercices pour bien démarrer · **Les types en profondeur** : Unions et rétrécissement, Les classes, Les génériques, Types littéraux, enums et as const, Types avancés, Les décorateurs, Gérer les erreurs, Modules et npm · **Le monde réel** : La programmation asynchrone, Configurer TypeScript (tsconfig), Passer de JavaScript à TypeScript, Lire et écrire des fichiers avec Node, JSON et validation des données, Les expressions régulières en TypeScript, Dates et heures, Un outil en ligne de commande typé, TypeScript pour le web · **Bien programmer** : Tester son code, Déboguer en TypeScript, Bien écrire du TypeScript, Organiser un projet, Projets pas à pas, Exercices corrigés, Exercices corrigés : deuxième série, Aide-mémoire |
| [Go](content/go) (38) | **Les bases** : Go : introduction et installation, Premiers pas, Nombres et calculs, Conditions et boucles, Les fonctions, Travailler avec du texte, Tableaux et slices, Les maps, Suivre un programme pas à pas en Go, Les pièges du débutant en Go, Exercices pour bien démarrer · **Aller plus loin** : Structures et méthodes, Les interfaces, Les pointeurs, Les génériques, Gérer les erreurs, Packages et modules · **Le monde réel** : Les goroutines, Les canaux, Annuler et limiter dans le temps avec context, sync : Once, RWMutex, atomic et errgroup, Lire et écrire : clavier et fichiers, Embarquer des fichiers avec embed, JSON et CSV, Générer du texte et du HTML avec les templates, Une base de données en Go, Les expressions régulières, Dates et heures, Écrire un outil en ligne de commande, Go pour le web · **Bien programmer** : Tester son code, Déboguer en Go, Bien écrire du Go, Organiser un projet, Projets pas à pas, Exercices corrigés, Exercices corrigés : deuxième série, Aide-mémoire |
| [Ruby](content/ruby) (40) | **Les bases** : Ruby : introduction et installation, Premiers pas, Nombres et calculs, Conditions et boucles, Les méthodes, Travailler avec du texte, Tableaux et hashs, Les intervalles, Suivre un programme pas à pas en Ruby, Les pièges du débutant en Ruby, Exercices pour bien démarrer · **Aller plus loin** : Blocs, itérateurs et Enumerable, Procs, lambdas et méthodes, Énumérateurs et paresse, Classes et objets, Héritage et polymorphisme, Modules et mixins, Struct, Data et Set, Égalité, comparaison et tri, Le filtrage par motif (case/in), Gérer les erreurs · **Le monde réel** : Lire et écrire : clavier et fichiers, JSON, CSV et YAML, Une base de données en Ruby, Les expressions régulières, Dates et heures, Écrire un outil en ligne de commande, Faire plusieurs choses à la fois, Ruby pour le web, Un premier site avec Rails, pas à pas · **Bien programmer** : Tester son code, Déboguer en Ruby, Bien écrire du Ruby, Métaprogrammation en douceur, Organiser un projet : fichiers, gems et Bundler, Créer et publier sa propre gem, Projets pas à pas, Exercices corrigés, Exercices corrigés : deuxième série, Aide-mémoire |
| [Rust](content/rust) (42) | **Les bases** : Rust : introduction et installation, Premiers pas, Nombres et calculs, Conditions et boucles, Les fonctions, Travailler avec du texte, Collections, HashMap, HashSet et BTreeMap, Suivre un programme pas à pas, Les pièges du débutant en Rust, Exercices pour bien démarrer · **Le cœur de Rust** : Possession et emprunt, Les durées de vie, Structures, énumérations et traits, Le filtrage par motif, Génériques et traits, Les objets traits : dyn Trait, Les traits de la bibliothèque standard, Gérer les erreurs, Les closures en détail, Les itérateurs · **Aller plus loin** : Modules et crates, Lire et écrire : clavier et fichiers, Box, Rc et RefCell, Faire plusieurs choses à la fois, La programmation asynchrone, Les macros, unsafe et les règles qu'on peut enfreindre · **Le monde réel** : Écrire un outil en ligne de commande, JSON, CSV et TOML avec serde, Une base de données en Rust, Rust pour le web, Rust dans le navigateur avec WebAssembly, Écrire du code rapide · **Bien programmer** : Tester son code, Déboguer en Rust, Bien écrire du Rust, Organiser un projet avec Cargo, Projets pas à pas, Exercices corrigés, Exercices corrigés : deuxième série, Aide-mémoire |
| [Rythme](content/rythme) (34) | **Les bases** : Rythme : introduction et installation, Premiers pas, Nombres et calculs, Conditions et répétition, Les fonctions, Travailler avec du texte, Listes, enregistrements et dictionnaires, Dictionnaires et ensembles, Suivre un programme pas à pas, Les pièges du débutant en Rythme, Exercices pour bien démarrer · **Aller plus loin** : Penser fonctionnel, La récursion, Vos propres types et match, Les symboles et quand les utiliser, Le filtrage par motif, Gérer les erreurs, Modules : un programme en plusieurs fichiers · **Le monde réel** : Lire et écrire : clavier et fichiers, Données JSON, Textes avancés : nombres, colonnes et rapports, Les expressions régulières, Les dates, Écrire un outil en ligne de commande · **Bien programmer** : Le vérificateur et les messages d'erreur, Comprendre un calcul étape par étape, Tester son code, Bien écrire du Rythme, Venir de Ruby (ou de Python), Organiser un projet : rythme new, fmt, doc et éditeurs, Projets pas à pas, Exercices corrigés, Exercices corrigés : deuxième série, Aide-mémoire |

Chaque chapitre suit le même plan : objectifs, sections avec un exemple chacune, erreurs
fréquentes (avec les vrais messages d'erreur), « Testez-vous » (réponses repliées) et
résumé. Chaque langage a une page « Suivre un programme pas à pas », ses pièges du débutant, 15 exercices pour bien démarrer, 3 projets guidés, deux séries de 12 exercices corrigés et un aide-mémoire.

## Ce que fait le site

- **Trois langues** : le français aux adresses d'origine (`/rust/fonctions/`),
  l'anglais derrière `/en/` et l'espagnol derrière `/es/` (`/en/compare/`,
  `/es/comparar/`). Le sélecteur FR · EN · ES de l'en-tête mène à la même page dans
  l'autre langue. Toute l'interface est traduite ; un chapitre pas encore traduit
  s'affiche en français, avec un bandeau et la mention « FR » dans les listes.

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

Les pages sont dans `content/<parcours>/`, en Markdown ; leur ordre est dans
[`src/catalog.rs`](src/catalog.rs). Les traductions sont dans `content/en/<parcours>/`
et `content/es/<parcours>/`, sous le même nom ; elles sont prises en compte toutes
seules à la compilation (`build.rs`). Les règles d'écriture (titre, résumé, code exécuté,
sorties vérifiées, encadrés, liens) sont dans [`content/README.md`](content/README.md).

```sh
ruby tools/verifier.rb       # exécute chaque exemple et compare sa sortie
ruby tools/suite.rb          # relie chaque chapitre au suivant (« Suite : », « Next: », « Siguiente: »)
ruby tools/i18n/ancres.rb    # les liens des traductions vers le glossaire traduit
cargo test                   # chaque page, chaque lien et chaque ancre
```

Le vérificateur a besoin de `ruby` (3.3), `rustc`, `python3` (3.13), `go` (1.24), `node` (22),
de TypeScript (`npm ci --prefix tools/ts`) et de `rythme`
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
| `content/` | les pages, en Markdown (`content/en/`, `content/es/` : les traductions) |
| `src/i18n.rs` | les trois langues et les textes de l'interface |
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
| `tools/i18n/` | les ancres du glossaire traduit, et le script qui les réécrit |
