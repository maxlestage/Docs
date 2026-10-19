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
   des langages différents. Mieux vaut peu et juste que beaucoup et faux.
4. Un nouveau chapitre s'ajoute à `src/catalog.rs` (dans la bonne partie du parcours),
   puis `ruby tools/suite.rb` relie les chapitres entre eux.
5. Avant de publier, tout doit passer :
   - `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test` ;
   - `ruby tools/verifier.rb content/<parcours>/<fichier>.md …` sur chaque page touchée ;
   - `ruby tools/suite.rb --check` ;
   - l'export et ses liens : `BASE_PATH=/Docs cargo run --release -- export /tmp/dist`
     puis `ruby tools/liens.rb /tmp/dist /Docs`.
6. Publier : un commit clair sur la branche `claude/routine`, poussé, puis `master`
   avancé sur ce commit (`git push origin HEAD:master`, en avance rapide seulement ;
   si master a bougé, d'abord `git pull --rebase origin master` et tout revérifier).
   Si une vérification échoue et ne peut pas être corrigée, ne rien publier : noter le
   problème dans le journal ci-dessous.
7. Cocher les éléments faits dans la liste, en ajouter si une idée manque, et écrire
   une ligne dans le journal.

## Outils

- Ruby 3.3, Rust (`rustc`), Python 3.13, Go 1.24, Node 22, TypeScript
  (`npm ci --prefix tools/ts`), Rythme (`cargo install --git https://github.com/maxlestage/Rythme --locked rythme`,
  ou `RYTHME=/chemin/vers/rythme`).

## À faire

### Python
- [ ] `sqlite` — Une base de données avec sqlite3 (exécutable : base en mémoire)
- [ ] `collections-avancees` — namedtuple, deque, heapq, bisect, Counter en détail
- [ ] `contextes` — Les gestionnaires de contexte (with, contextlib)
- [ ] `logging` — Journaliser avec logging
- [ ] `donnees` — Analyser des données (csv + statistics ; pandas en `norun`)

### JavaScript
- [ ] `nombres-et-maths` → déjà couvert ; à la place : `tableaux-avances` (flatMap, groupBy, toSorted, structuredClone)
- [ ] `node-http` — Un serveur avec node:http et ses routes
- [ ] `canvas` — Dessiner dans le navigateur (norun) et la logique à côté
- [ ] `stockage` — localStorage, IndexedDB (norun) et JSON

### TypeScript
- [ ] `nombres` — Nombres et calculs en TypeScript
- [ ] `regex` — Les expressions régulières typées
- [ ] `dates` — Dates et heures
- [ ] `ligne-de-commande` — Un outil en ligne de commande typé
- [ ] `decorateurs` — Les décorateurs (TypeScript 5)

### Go
- [ ] `context` — Annuler et limiter dans le temps avec context
- [ ] `sync` — sync.Once, RWMutex, atomic, errgroup (norun)
- [ ] `base-de-donnees` — database/sql (norun) et un stockage en mémoire testable
- [ ] `embed` — Embarquer des fichiers avec embed

### Ruby
- [ ] `base-de-donnees` — SQLite et ActiveRecord (norun) ; un stockage en mémoire testable
- [ ] `rails` — Un premier site avec Rails, pas à pas (norun)
- [ ] `gems` — Créer et publier sa propre gem

### Rust
- [ ] `traits-objets` — dyn Trait, Box<dyn Trait> et la répartition dynamique
- [ ] `base-de-donnees` — rusqlite (norun) et un stockage en mémoire testable
- [ ] `wasm` — Rust dans le navigateur avec WebAssembly (norun)

### Rythme
- [ ] `symboles` — Les symboles et quand les utiliser
- [ ] `textes-avances` — format_number, colonnes, rapports

### Débuter
- [ ] `bases-de-donnees` — Qu'est-ce qu'une base de données
- [ ] `securite` — Les bases de la sécurité (mots de passe, injections)
- [ ] `open-source` — Contribuer à un projet open source
- [ ] `metiers` — Les métiers du développement

### Pour tous
- [ ] `exercices-2` — Une deuxième série de 12 exercices corrigés, par langage
- [ ] La page Comparer : 5 idées de plus (bases de données, classes, itération, …)

## Journal

- 2026-10-19 — Mise en place de la routine (7 langages, 223 chapitres).
