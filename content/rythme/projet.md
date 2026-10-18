# Projets, modules et tests

Cette page montre comment tester votre code, le répartir en plusieurs fichiers, créer un projet, et profiter des outils de Rythme : mise en forme, documentation, éditeurs et exercices.

## Écrire des tests

En Rythme, les tests vivent **à côté du code**, dans le même fichier. Un test
a un nom et une vérification :

```rythme
fn double(x) = x * 2

test "le double de 21 est 42" = assert_eq(double(21), 42)

test "le double garde le signe"
  assert(double(-3) < 0, "doit rester négatif")
  assert_eq(double(0), 0)
end

print(double(5))
```

```sortie
10
```

Quand on lance le programme avec `rythme run`, les tests sont ignorés : seul
le `print` s'exécute.

- `assert_eq(obtenu, attendu)` vérifie que deux valeurs sont égales ;
- `assert(condition, message)` vérifie qu'une condition est vraie.

## Lancer les tests

`rythme test` lance les tests, et saute les autres lignes du premier
niveau. Ajoutons un test faux, pour voir :

```rythme norun
test "une erreur volontaire" = assert_eq(double(3), 7)
```

```sh
rythme test calculs.ry
```

```text
Exécution de 3 test(s)
  ✓ le double de 21 est 42
  ✓ le double garde le signe
  ✗ une erreur volontaire
erreur[E015]: assertion échouée : les valeurs sont différentes
  --> calculs.ry:10:32
   |
10 | test "une erreur volontaire" = assert_eq(double(3), 7)
   |                                ^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: obtenu :  6
       attendu : 7
   = en savoir plus : `rythme explain E015`

3 test(s) : 2 réussi(s), 1 échoué(s) (0 ms)
```

Quelques options utiles :

```sh
rythme test .                      # tous les fichiers .ry du dossier
rythme test calculs.ry --filter signe   # seulement les tests dont le nom contient « signe »
rythme test calculs.ry --watch     # relancer à chaque modification
```

## Utiliser plusieurs fichiers avec use

Un programme peut utiliser un autre fichier. Voici `geometrie.ry` :

```rythme norun
## L'aire d'un rectangle.
fn aire_rectangle(l, h) = l * h

fn aire_carre(cote) = aire_rectangle(cote, cote)

fn _secret() = 42
```

Et `principal.ry`, dans le même dossier :

```rythme norun
use geometrie

print(geometrie.aire_rectangle(2, 3))
print(geometrie.aire_carre(4))
```

```text
6
16
```

- `use geometrie` charge `geometrie.ry` (sans guillemets, sans `.ry`).
- On écrit ensuite `geometrie.nom` : chaque nom dit d'où il vient.
- Un nom qui commence par `_`, comme `_secret`, reste **privé** à son
  fichier.
- `use formes/cercle` charge `formes/cercle.ry`, utilisé comme `cercle`.
- `use geometrie as geo` donne un nom plus court.

## Créer un projet

`rythme new` crée un dossier prêt à l'emploi :

```sh
rythme new bonjour
```

```text
Projet `bonjour` créé :
  bonjour/rythme.toml
  bonjour/src/main.ry
  bonjour/src/salutation.ry
  bonjour/tests/salutation_test.ry
  bonjour/README.md
  bonjour/.gitignore
Ensuite : `cd bonjour`, puis `rythme run` pour le lancer et `rythme test` pour le tester (voir `rythme help projects`)
```

Dans le dossier du projet, les commandes n'ont plus besoin de nom de
fichier : `rythme run` lance `src/main.ry`, et `rythme test` lance les
tests de `src/` et de `tests/`.

```sh
cd bonjour
rythme run
```

```text
Bonjour, le monde !
```

## Le fichier rythme.toml

`rythme.toml` décrit le projet. Le nom et la version sont obligatoires :

```toml
name = "bonjour"
version = "0.1.0"
entry = "src/main.ry"
```

`entry` est le fichier lancé par `rythme run`.

## Mettre en forme avec rythme fmt

`rythme fmt` réécrit le code dans le style officiel : espaces, retraits,
lignes vides. Il ne change jamais ce que fait le programme.

```rythme norun
fn  aire(l,h)=l*h
print( aire(2,3) )
```

Après `rythme fmt brouillon.ry` :

```rythme
fn aire(l, h) = l * h
print(aire(2, 3))
```

```sortie
6
```

`rythme fmt --check .` liste les fichiers à mettre en forme sans les
changer.

## Documenter avec rythme doc

`rythme doc` écrit la documentation d'un fichier, en Markdown, à partir de
ses commentaires `##` :

```sh
rythme doc geometrie.ry            # du Markdown
rythme doc --html geometrie.ry     # une page web
```

Les noms privés (`_secret`) sont laissés de côté.

## Votre éditeur

Un bon éditeur souligne les erreurs pendant que vous tapez, et affiche
l'aide d'une fonction quand vous passez la souris dessus. Pour cela, il
utilise `rythme lsp` : vous ne le lancez pas vous-même, l'éditeur le fait.

- **VS Code** : une extension existe, dans le dossier `editors/vscode` du
  dépôt de Rythme.
- **Les autres éditeurs** (Neovim, Helix, Zed...) : `rythme help editors`
  explique comment faire.

Pour choisir un éditeur, voir [l'éditeur de code](../debuter/editeur.md).

## Apprendre avec learn et exercise

`rythme learn` est une visite guidée du langage, en 13 sujets.

`rythme exercise` propose 30 exercices gradués, corrigés par des tests :

```sh
rythme exercise list                       # la liste
rythme exercise start 1                    # écrit 01-valeurs.ry ici
rythme exercise check 01-valeurs.ry        # vérifie votre solution
```

Chaque exercice contient un énoncé, des indices et des tests. Vous
remplacez les `0` ou les `fail("à faire")` par votre code, puis vous
vérifiez. Pour commencer en douceur, essayez d'abord les
[exercices corrigés](exercices.md) de ce site.

## En résumé

- `test "..." = ...` écrit un test à côté du code, `rythme test` les lance.
- `use geometrie` utilise le fichier `geometrie.ry` ; ses noms s'écrivent
  `geometrie.nom`.
- `rythme new nom` crée un projet prêt à l'emploi.
- `rythme fmt` met le code en forme, et votre éditeur vous aide grâce à
  `rythme lsp`.
- `rythme learn` et `rythme exercise` permettent de s'entraîner.

Suite : [les exercices corrigés](exercices.md), puis
[l'aide-mémoire](aide-memoire.md). Comparez avec les projets en
[Ruby](../ruby/projet.md) et en [Rust](../rust/projet.md).
