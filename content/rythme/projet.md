# Organiser un projet : rythme new, fmt, doc et éditeurs

Cette page montre comment créer un vrai projet Rythme avec `rythme new`, puis profiter des outils du langage : mise en forme, documentation, éditeur et exercices.

> **Objectifs :**
>
> - créer un projet avec `rythme new`, et savoir ce que contient chaque fichier ;
> - lancer et tester un projet sans donner de nom de fichier ;
> - mettre le code en forme avec `rythme fmt` et le documenter avec `rythme doc` ;
> - configurer son éditeur, et s'entraîner avec `rythme learn` et `rythme exercise`.

## Avant de commencer : modules et tests

Un projet rassemble deux choses que vous connaissez déjà :

- **plusieurs fichiers**, reliés par `use` : chaque fichier est un module.
  Tout est expliqué dans [les modules](modules.md).
- **des tests**, écrits avec `test "nom" = ...` et lancés par
  `rythme test`. Tout est expliqué dans [tester son code](tests.md).

Cette page montre comment les ranger ensemble dans un dossier bien organisé.

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

Le nom d'un projet s'écrit en minuscules, avec des lettres, des chiffres et
`_` : `mon_jeu`, pas `mon-jeu`.

## Ce que contient le projet

```text
bonjour/
├── rythme.toml                 la carte d'identité du projet
├── README.md                   une présentation, pour les humains
├── .gitignore                  les fichiers que git ne garde pas
├── src/
│   ├── main.ry                 le programme
│   └── salutation.ry           un module
└── tests/
    └── salutation_test.ry      les tests du module
```

Le code va dans `src/`, les tests dans `tests/`. Voici `src/salutation.ry`,
un module avec une seule fonction :

```rythme norun
## Une salutation pour `nom`.
fn bonjour(nom) = "Bonjour, {nom} !"
```

Le programme, `src/main.ry`, l'utilise avec `use` :

```rythme norun
# Le programme : `rythme run` dans le dossier du projet.
use salutation

print(salutation.bonjour("le monde"))
```

Et le test, `tests/salutation_test.ry`, l'utilise aussi. Depuis `tests/`, on
écrit le nom du projet devant le module : `use bonjour/salutation` désigne
le fichier `src/salutation.ry` du projet `bonjour`.

```rythme norun
# Les tests : `rythme test` dans le dossier du projet.
use bonjour/salutation

test "bonjour" = assert_eq(salutation.bonjour("Ada"), "Bonjour, Ada !")
```

## Lancer et tester le projet

Dans le dossier du projet, les commandes n'ont plus besoin de nom de
fichier :

```sh
cd bonjour
rythme run
```

```text
Bonjour, le monde !
```

```sh
rythme test
```

```text
Exécution de 1 test(s)
  ✓ bonjour
1 test(s) : 1 réussi(s), 0 échoué(s) (1 ms)
```

| Commande | Ce qu'elle fait dans un projet |
|----------|--------------------------------|
| `rythme run` | lance le programme, `src/main.ry` |
| `rythme test` | lance les tests de `src/` et de `tests/` |
| `rythme check` | vérifie tous les fichiers, sans rien lancer |
| `rythme fmt` | met en forme `src/` et `tests/` |
| `rythme doc` | écrit la documentation de `src/` |

## Le fichier rythme.toml

`rythme.toml` décrit le projet. Le nom et la version sont obligatoires :

```toml
name = "bonjour"
version = "0.1.0"
entry = "src/main.ry"
```

- `name` : le nom du projet, celui qu'on écrit dans `use bonjour/...` ;
- `version` : trois nombres, que vous augmentez à chaque nouvelle version ;
- `entry` : le fichier lancé par `rythme run`.

Le fichier créé par `rythme new` contient aussi une section
`[dependencies]`, en commentaire : elle sert à utiliser le code d'autres
projets. `rythme help projects` explique comment faire.

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
ses commentaires `##`. Avec le fichier `points.ry` de
[la page sur les modules](modules.md) :

```sh
rythme doc points.ry            # du Markdown
rythme doc --html points.ry     # une page web
```

```text
# points.ry

## Fonctions

### distance

~~~rythme
fn distance(a, b)
~~~

La distance entre deux points {x, y}.

## Valeurs

### origine

~~~rythme
origine = {x: 0, y: 0}
~~~

Le point de départ, au centre.
```

Les noms privés (comme `_carre`) sont laissés de côté.

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

## Erreurs fréquentes

### Un tiret dans le nom du projet

```sh
rythme new mon-app
```

```text
erreur[E025]: `mon-app` ne peut pas être le nom d'un paquet : il sert dans `use mon-app/fichier`, donc ce doit être un nom Rythme en minuscules (lettres, chiffres, `_`)
  = aide: essayez `rythme new mon_app`
  = en savoir plus : `rythme explain E025`
```

Correction : `rythme new mon_app`, avec `_` à la place du tiret.

### Un dossier qui existe déjà

```sh
rythme new bonjour
```

```text
erreur[E025]: `bonjour` existe déjà
  = aide: choisissez un autre nom, ou un autre dossier
  = en savoir plus : `rythme explain E025`
```

Rythme ne touche jamais un dossier qui existe déjà : vos fichiers sont en
sécurité. Choisissez un autre nom.

### rythme run en dehors d'un projet

Dans un dossier qui n'a pas de `rythme.toml`, `rythme run` ne sait pas quoi
lancer :

```text
quel fichier ? Exemple : rythme run bonjour.ry
```

Correction : donnez le nom du fichier (`rythme run jeu.ry`), ou placez-vous
d'abord dans le dossier du projet (`cd bonjour`).

## Testez-vous

1. Dans un projet, où va le code, et où vont les tests ?

> **Réponse**
>
> Le code va dans `src/` (le programme est `src/main.ry`), les tests dans
> `tests/`.

2. Que fait `rythme fmt` à ce que fait le programme ?

> **Réponse**
>
> Rien : il change seulement la présentation (espaces, retraits, lignes
> vides). Le programme fait exactement la même chose après.

3. Quels commentaires `rythme doc` utilise-t-il ?

> **Réponse**
>
> Les commentaires de documentation, qui commencent par `##`, juste au-dessus
> d'une fonction, d'une valeur ou d'un type. Les noms privés (`_nom`) sont
> laissés de côté.

## En résumé

- `rythme new nom` crée un projet : `rythme.toml`, le code dans `src/`, les
  tests dans `tests/`.
- Dans le dossier du projet, `rythme run`, `rythme test` et `rythme check`
  marchent sans nom de fichier.
- `rythme fmt` met le code en forme, `rythme doc` écrit sa documentation à
  partir des commentaires `##`.
- Votre éditeur vous aide grâce à `rythme lsp`.
- `rythme learn` et `rythme exercise` permettent de s'entraîner.

Suite : [projets pas à pas](projets.md). Comparez avec les projets en
[Ruby](../ruby/projet.md) et en [Rust](../rust/projet.md).
