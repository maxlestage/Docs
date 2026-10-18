# Écrire une page

Les pages sont des fichiers Markdown : `content/{ruby,rust,rythme}/{page}.md`. La liste
des pages et leur ordre sont dans `src/catalog.rs`.

## Forme d'une page

- La première ligne est le titre : `# Les fonctions`.
- Le premier paragraphe, une phrase, résume la page : il sert de description (moteurs de
  recherche, cartes de l'accueil, recherche du site).
- Des sections `##` et des sous-sections `###` : elles forment le sommaire de droite.
- Des phrases courtes, en français simple, en vouvoyant le lecteur. Une idée par section,
  toujours avec un exemple.

## Code

- ` ```ruby `, ` ```rust ` et ` ```rythme ` sont **exécutés** par `tools/verifier.rb` :
  un bloc Rust sans `fn main` est placé dans un `fn main() { … }`.
- ` ```rust norun ` (ou `ruby norun`, `rythme norun`) : un bloc montré mais pas exécuté
  (une erreur volontaire, une bibliothèque externe, une lecture au clavier…).
- Un bloc ` ```sortie ` juste après un bloc exécuté donne sa sortie exacte : elle est
  vérifiée.
- ` ```sh ` pour les commandes, ` ```toml `, ` ```text ` pour le reste.

## Encadrés

Une citation qui commence par un de ces mots devient un encadré coloré :

```text
> **Astuce :** …
> **À retenir :** …
> **Attention :** …
> **Note :** …
```

## Liens

Entre les pages, des liens relatifs vers les fichiers : `[les fonctions](fonctions.md)`,
`[le même en Rust](../rust/fonctions.md#closures)`. Ils deviennent des liens du site, et
les tests vérifient que la page et l'ancre existent. Une ancre est le titre en minuscules,
sans accents, les espaces remplacés par des `-` : `## Les boucles for` → `#les-boucles-for`.

## Vérifier

```sh
ruby tools/verifier.rb                      # tout
ruby tools/verifier.rb content/rust/*.md    # une partie
```
