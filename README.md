# 📚 Docs : Ruby, Rust et Rythme

**Des documentations simples, en français, pour apprendre Ruby, Rust et Rythme.**
Trois parcours de dix chapitres, un exemple pour chaque idée, et du code qui marche
vraiment : chacun des exemples est exécuté avant d'être publié.

Le site est écrit en Rust avec [Paquets](https://github.com/maxlestage/Paquets) : un
seul programme sert les pages (rendues sur le serveur, avec leurs balises SEO et une
Content-Security-Policy stricte), la recherche, le plan du site, et l'application
installable qui marche hors ligne.

| Parcours | Chapitres |
|---|---|
| [Ruby](content/ruby) | introduction, premiers pas, conditions et boucles, méthodes, tableaux et hashs, blocs et Enumerable, classes et objets, erreurs, gems et tests, aide-mémoire |
| [Rust](content/rust) | introduction, premiers pas, conditions et boucles, fonctions, collections, possession et emprunt, structures et traits, erreurs, Cargo et tests, aide-mémoire |
| [Rythme](content/rythme) | introduction, premiers pas, conditions et répétition, fonctions, listes et enregistrements, penser fonctionnel, types et `match`, erreurs, modules et tests, aide-mémoire |

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
- **Fichiers statiques** (GitHub Pages, Netlify…) : `BASE_PATH=/Docs cargo run --release -- export dist`
  écrit tout le site dans `dist/`, rendu par le serveur lui-même. Le workflow
  [`pages.yml`](.github/workflows/pages.yml) le publie sur GitHub Pages, à la main.

Si des dépôts dont dépend Paquets sont privés, donnez un jeton GitHub qui peut les lire
dans le secret (ou la variable Heroku) `REPOS_TOKEN`.

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
