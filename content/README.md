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
> **Indice :** …
> **Objectifs :** …
```

## Exercices

Un exercice est une section `### Exercice 1 : Le titre`, avec son énoncé, un indice
facultatif et sa solution. La solution est une citation qui commence par
`**Solution**` : sur le site, elle est repliée sous un bouton « Voir la solution ».
Son code est exécuté comme les autres.

```text
### Exercice 1 : Doubler

Écrivez une fonction `double` qui renvoie le double d'un nombre.

> **Indice :** multipliez par 2.

> **Solution**
>
> ```ruby
> def double(x) = x * 2
> puts double(21)
> ```
>
> ```sortie
> 42
> ```
```

## Plan d'un chapitre

Tous les chapitres suivent le même plan, pour que le lecteur s'y retrouve :

1. `# Titre`, puis **une phrase** qui résume le chapitre.
2. Les objectifs, juste après, dans un encadré :

   ```text
   > **Objectifs :**
   >
   > - écrire une fonction et l'appeler ;
   > - donner des paramètres à une fonction.
   ```

3. Les sections `##`, du plus simple au plus difficile, chacune avec un exemple.
4. `## Erreurs fréquentes` : les 2 à 4 erreurs que font les débutants, chacune avec
   le code fautif (`norun`), le message d'erreur réel (bloc `text`) et la correction.
5. `## Testez-vous` : 3 questions courtes ; chaque réponse est une citation qui
   commence par `**Réponse**`, repliée sous « Voir la réponse ».
6. `## En résumé` : 3 à 6 points, ce qu'il faut retenir, puis une ligne
   « Suite : [le chapitre suivant](….md) ».

Les pages « Exercices corrigés », « Aide-mémoire » et « Glossaire » n'ont ni objectifs,
ni erreurs fréquentes, ni « Testez-vous ».

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

## Traductions (anglais, espagnol)

Le français est la langue de référence : `content/<parcours>/<page>.md`. Les traductions
sont dans `content/en/<parcours>/<page>.md` et `content/es/<parcours>/<page>.md`, avec le
même nom de fichier. Une page pas encore traduite s'affiche en français, avec un bandeau.

- Tout se traduit : le texte, les titres, les commentaires du code, les textes affichés
  par les programmes et les noms de variables quand c'est naturel. Le code doit toujours
  marcher, et chaque bloc ` ```sortie ` doit être la sortie exacte de la version traduite
  (`ruby tools/verifier.rb content/en/…`). Les messages d'erreur réels restent ceux que
  l'outil affiche (Rythme : `RYTHME_LANG=en` ou `es`, le vérificateur le fait).
- Les mots des encadrés et des sections changent avec la langue :

  | Français | English | Español |
  |---|---|---|
  | `**Astuce :**` | `**Tip:**` | `**Consejo:**` |
  | `**À retenir :**` | `**Remember:**` | `**Recuerda:**` |
  | `**Attention :**` | `**Warning:**` | `**Atención:**` |
  | `**Note :**` | `**Note:**` | `**Nota:**` |
  | `**Indice :**` | `**Hint:**` | `**Pista:**` |
  | `**Objectifs :**` | `**Goals:**` | `**Objetivos:**` |
  | `**Solution**` | `**Solution**` | `**Solución**` |
  | `**Réponse**` | `**Answer**` | `**Respuesta**` |
  | `## Erreurs fréquentes` | `## Common mistakes` | `## Errores frecuentes` |
  | `## Testez-vous` | `## Test yourself` | `## Ponte a prueba` |
  | `## En résumé` | `## Summary` | `## En resumen` |
  | `Suite : [Titre](page.md)` | `Next: [Title](page.md)` | `Siguiente: [Título](page.md)` |
  | `Niveau : ★☆☆` | `Level: ★☆☆` | `Nivel: ★☆☆` |

- Les liens relatifs (`fonctions.md`, `../rust/types.md`) restent les mêmes : ils mènent
  à la page dans la même langue. Une ancre (`page.md#ancre`) doit exister dans la page
  telle qu'elle est servie dans cette langue : la traduction si elle existe, sinon la
  page française. Dans le doute, un lien vers la page sans ancre.
