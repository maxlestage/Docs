# Routine : compléter la documentation

Ce fichier guide la tâche planifiée qui, toutes les 3 heures, complète la documentation.
Chaque passage lit ce fichier, fait **un petit lot de travail fini et vérifié**, met à
jour la liste ci-dessous, puis publie.

## Règles

1. Le format des pages est dans [`content/README.md`](content/README.md) : le plan de
   chapitre (objectifs, erreurs fréquentes avec les vrais messages, « Testez-vous »,
   résumé, « Suite »), le code exécuté et sa sortie exacte. Le lire avant d'écrire.
2. Écrire en français simple, pour des débutants, comme les chapitres existants du même
   parcours (en lire deux d'abord).
3. Un passage = **2 à 4 chapitres nouveaux ou fortement améliorés**, de préférence dans
   des langages différents, **ou 6 à 10 pages traduites** (voir « Traductions »
   ci-dessous). En priorité, les traductions : un passage sur deux au moins en fait.
   Mieux vaut peu et juste que beaucoup et faux.
3 bis. **Traduire** : une page `content/<parcours>/<page>.md` devient
   `content/en/<parcours>/<page>.md` (anglais, « you ») et
   `content/es/<parcours>/<page>.md` (espagnol, « tú »), même nom de fichier. Les mots
   des encadrés, des sections et de la ligne « Suite » sont dans la section
   « Traductions » de [`content/README.md`](content/README.md). Même plan, mêmes blocs
   de code (noms, textes et commentaires traduits) ; chaque bloc ```sortie et chaque
   message d'erreur est la vraie sortie du code traduit (Rythme : `RYTHME_LANG=en` ou
   `es`). Les liens vers le glossaire gardent l'ancre française, puis
   `ruby tools/i18n/ancres.rb` les réécrit ; une ancre vers une autre page traduite
   prend le titre traduit (`slug()` de `src/markdown.rs`), vers une page pas encore
   traduite, pas d'ancre. Une page sans traduction s'affiche en français avec un
   bandeau : rien d'autre à faire. Un nouveau chapitre en français ajoute sa ligne à
   « Traductions » ci-dessous.
4. Un nouveau chapitre s'ajoute à `src/catalog.rs` (dans la bonne partie du parcours),
   puis `ruby tools/suite.rb` relie les chapitres entre eux.
5. Avant de publier, tout doit passer :
   - `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test` ;
   - `ruby tools/verifier.rb content/<parcours>/<fichier>.md …` sur chaque page touchée ;
   - `ruby tools/suite.rb --check` et `ruby tools/i18n/ancres.rb --check` ;
   - l'export et ses liens : `BASE_PATH=/Docs cargo run --release -- export /tmp/dist`
     puis `ruby tools/liens.rb /tmp/dist /Docs`.
6. Publier par une pull request, jamais directement sur `master` :
   - le travail s'accumule sur la branche `claude/routine` et dans **une seule PR**
     ouverte vers `master` (titre « Routine : compléter la documentation ») ;
   - au début d'un passage : s'il y a déjà une PR ouverte depuis `claude/routine`,
     repartir de cette branche et y fusionner `origin/master` (`git merge`, sans
     réécrire l'historique) ; sinon, créer `claude/routine` depuis `origin/master` ;
   - à la fin : un commit clair, poussé sur `claude/routine` ; si la PR n'existe pas,
     la créer (`gh api -X POST repos/maxlestage/Docs/pulls …`) ;
   - **fusion, avec l'autorisation du propriétaire** : quand la PR contient
     **au moins 3 commits** et que toutes les vérifications de son dernier commit
     (`gh api repos/maxlestage/Docs/commits/<sha>/check-runs`) sont terminées et
     réussies, la fusionner (`gh api -X PUT repos/maxlestage/Docs/pulls/<n>/merge
     -f merge_method=merge`) puis supprimer la branche `claude/routine`. Une CI rouge
     se corrige d'abord ; une PR rouge ne se fusionne jamais ;
   - si une vérification échoue et ne peut pas être corrigée, ne rien pousser : noter le
     problème dans le journal ci-dessous.
   `gh` ne parle qu'à l'API REST (`gh api …`) : pas de `gh pr …`, qui utilise GraphQL.
7. Cocher les éléments faits dans la liste, en ajouter si une idée manque, et écrire
   une ligne dans le journal.

## Outils

- Ruby 3.3, Rust (`rustc`), Python 3.13, Go 1.24, Node 22, TypeScript
  (`npm ci --prefix tools/ts`), Rythme (`cargo install --git https://github.com/maxlestage/Rythme --locked rythme`,
  ou `RYTHME=/chemin/vers/rythme`).

## À faire

### Python
- [x] `sqlite` — Une base de données avec sqlite3 (exécutable : base en mémoire)
- [x] `collections-avancees` — namedtuple, deque, heapq, bisect, Counter en détail
- [x] `contextes` — Les gestionnaires de contexte (with, contextlib)
- [x] `logging` — Journaliser avec logging
- [x] `donnees` — Analyser des données (csv + statistics ; pandas en `norun`)

### JavaScript
- [x] `tableaux-avances` — (flatMap, groupBy, toSorted, structuredClone)
- [x] `node-http` — Un serveur avec node:http et ses routes
- [x] `canvas` — Dessiner dans le navigateur (norun) et la logique à côté
- [x] `stockage` — localStorage, IndexedDB (norun) et JSON

### TypeScript
- [x] `nombres` — Nombres et calculs en TypeScript
- [x] `regex` — Les expressions régulières typées
- [x] `dates` — Dates et heures
- [x] `ligne-de-commande` — Un outil en ligne de commande typé
- [x] `decorateurs` — Les décorateurs (TypeScript 5)

### Go
- [x] `templates` — Générer du texte et du HTML avec text/template et html/template
- [x] `context` — Annuler et limiter dans le temps avec context
- [x] `sync` — sync.Once, RWMutex, atomic, errgroup (norun)
- [x] `base-de-donnees` — database/sql (norun) et un stockage en mémoire testable
- [x] `embed` — Embarquer des fichiers avec embed

### Ruby
- [x] `base-de-donnees` — SQLite et ActiveRecord (norun) ; un stockage en mémoire testable
- [x] `rails` — Un premier site avec Rails, pas à pas (norun)
- [x] `gems` — Créer et publier sa propre gem

### Rust
- [x] `traits-objets` — dyn Trait, Box<dyn Trait> et la répartition dynamique
- [x] `base-de-donnees` — rusqlite (norun) et un stockage en mémoire testable
- [x] `wasm` — Rust dans le navigateur avec WebAssembly (norun)

### Rythme
- [x] `symboles` — Les symboles et quand les utiliser
- [x] `textes-avances` — format_number, colonnes, rapports

### Débuter
- [x] `bases-de-donnees` — Qu'est-ce qu'une base de données
- [x] `securite` — Les bases de la sécurité (mots de passe, injections)
- [x] `open-source` — Contribuer à un projet open source
- [x] `metiers` — Les métiers du développement

### Traductions (anglais et espagnol)

Traduit dans les deux langues : tout « Débuter » ; `introduction`, `premiers-pas`,
`conditions-boucles` et `fonctions` de chaque langage. À traduire ensuite, dans
l'ordre des parcours (une ligne par lot de 3 à 5 pages, dans les deux langues) :

- [x] Python : `nombres`, `texte`, `collections`, `dictionnaires`, `comprehensions`
- [x] JavaScript : `nombres`, `texte`, `collections`, `objets`
- [x] TypeScript : `texte`, `collections`, `objets`, `unions`
- [x] Go : `nombres`, `texte`, `collections`, `maps`
- [x] Ruby : `nombres`, `texte`, `collections`, `intervalles`
- [x] Rust : `nombres`, `texte`, `collections`, `hashmap`
- [x] Rythme : `nombres`, `texte`, `collections`, `dictionnaires`
- [x] Chaque parcours : `erreurs`, `tests`, `exercices`, `aide-memoire`
- [ ] Débuter : `variables`, `logique`, `boucles`, `fonctions`, `donnees`, `lire-une-erreur`, `bloque`
- [ ] Chaque langage : `pieges`, `exercices-faciles`
- [ ] Débuter : `bases-de-donnees`, `securite`, `open-source`, `metiers`
- [x] Nouveaux chapitres : Python `contextes`, TypeScript `regex`, Go `embed`
- [ ] Chaque langage : `pas-a-pas`, `exercices-2` ; et les chapitres cochés le 2026-10-19 (session principale)
- [ ] Puis le reste de chaque parcours, dans l'ordre de `src/catalog.rs`

### Pour tous
- [x] Pour chaque langage, une page « Pas à pas » de plus dans « Les bases » si un
  chapitre est trop dense (le découper, avec des tableaux qui suivent les variables)
- [x] `exercices-2` — Une deuxième série de 12 exercices corrigés, par langage
- [x] Les chapitres de base de chaque langage enrichis pour les grands débutants (FR, EN,
  ES) : « ligne par ligne », « À vous », « peut attendre » (voir `content/README.md`)
- [x] Étape 2, premier lot : `conditions-boucles` coupé en « Les conditions »
  (`conditions-boucles`, même adresse) et « Les boucles » (`boucles`), 7 langages, FR/EN/ES
- [ ] Étape 2, suite : découper les chapitres de base encore trop longs (plus de 650 lignes,
  ou plus de 12 notions) en deux pages, et mettre à jour leurs traductions
- [ ] Un outil qui signale les blocs `norun` d'erreur (tracebacks) dont le code ne
  correspond plus à la page traduite (ils ne sont pas vérifiés aujourd'hui)
- [x] La page Comparer : 5 idées de plus (bases de données, classes, itération, …)

## Journal

- 2026-10-19 — Mise en place de la routine (7 langages, 223 chapitres).
- 2026-10-19 — La routine publie par une PR, fusionnée à partir de 3 commits quand la CI est verte.
- 2026-10-19 — Le site en trois langues (FR, EN, ES) : interface traduite, 39 pages traduites dans chaque langue.
- 2026-10-19 — Traductions EN et ES de Python : `nombres`, `texte`, `collections`, `dictionnaires`, `comprehensions` (10 pages, 182 exemples vérifiés).
- 2026-10-19 — Session principale : 32 pages de plus pour les débutants qui peinent (Débuter : 7 pages « Les notions de base », bases de données, sécurité, open source, métiers ; dans chaque langage les pièges du débutant et 15 exercices faciles ; 7 chapitres de la liste).
- 2026-10-19 — Trois chapitres : Python `contextes`, TypeScript `regex`, Go `embed` (37 exemples exécutés, et les exemples `norun` vérifiés à la main). `go/context` et `rust/traits-objets`, pris par la session principale, ont été laissés.
- 2026-10-19 — Traductions EN et ES de JavaScript : `nombres`, `texte`, `collections`, `objets` (8 pages, 140 exemples vérifiés).
- 2026-10-19 — Traductions EN et ES : TypeScript (`texte`, `collections`, `objets`, `unions`, `regex`), Go (`nombres`, `texte`, `collections`, `maps`, `embed`), Ruby (`nombres`, `texte`, `collections`, `intervalles`), Rust (`nombres`, `texte`, `collections`, `hashmap`), Rythme (`nombres`, `texte`, `collections`, `dictionnaires`) et Python `contextes` (46 pages, 704 exemples vérifiés).
- 2026-10-19 — Session principale : toute la liste « À faire » (16 chapitres), une page « pas à pas » et une deuxième série d'exercices dans chaque langage, 5 idées de plus sur Comparer (31 pages, 674 exemples exécutés).
- 2026-10-19 — Traductions EN et ES : `erreurs`, `tests`, `exercices`, `aide-memoire` des sept langages (56 pages, 596 exemples vérifiés). Trois sorties de tests corrigées dans les pages françaises (python, ruby et go `tests`).
- 2026-10-19 — Les 50 chapitres de « Les bases » des sept langages enrichis pour les grands débutants, en FR, EN et ES (148 pages, 2942 exemples vérifiés).
- 2026-10-19 — `conditions-boucles` découpé en deux chapitres (conditions, puis `boucles`) dans les sept langages, en FR, EN et ES (42 pages, 460 exemples vérifiés).
