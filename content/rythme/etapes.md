# Comprendre un calcul étape par étape

`rythme etapes` montre comment Rythme calcule une valeur, une étape à la fois : cette page s'en sert pour comprendre la récursion, trouver une erreur de raisonnement, puis explore le mode interactif, où l'on essaie du code ligne par ligne.

> **Objectifs :**
>
> - vous saurez lancer `rythme etapes fichier.ry "expression"` et lire ce qu'il affiche ;
> - vous saurez suivre une fonction récursive, comme `factorielle` ou `fib` ;
> - vous saurez vous servir des étapes pour trouver une erreur de raisonnement ;
> - vous saurez utiliser le mode interactif, avec `:etapes`, `:doc` et `help` ;
> - vous saurez documenter vos fonctions pour que `help` les explique.

## Calculer comme à l'école

À l'école, pour calculer `(2 + 3) * 4`, on écrit une ligne par étape :
d'abord `5 * 4`, puis `20`. Rythme peut montrer ses calculs de la même
façon. Pour un appel de fonction, il **remplace l'appel par le corps de la
fonction**, avec les valeurs des arguments à la place des paramètres. On
appelle cela le **modèle par substitution**.

Prenons ce fichier, `fib.ry` :

```rythme
fn fib(n) = if n < 2 then n else fib(n - 1) + fib(n - 2)

print(fib(4))
```

```sortie
3
```

Il affiche 3, mais comment ? Demandons les étapes de `fib(3)` :

```text
$ rythme etapes fib.ry "fib(3)"
fib(3)
  → fib(2) + fib(1)
  → fib(1) + fib(0) + fib(1)
  → 1 + fib(0) + fib(1)
  → 1 + 0 + fib(1)
  → 1 + fib(1)
  → 1 + 1
  → 2
```

La première ligne est l'expression demandée. Chaque ligne qui commence par
`→` est l'expression après **une étape de plus** : un appel, ou une
opération. À la fin, il ne reste que le résultat.

Regardez la première étape : `fib(3)` devient directement
`fib(2) + fib(1)`. Rythme a remplacé `n` par 3, vu que `3 < 2` est faux, et
gardé la branche `else`. Le corps d'une fonction est montré déjà simplifié
autant que possible.

## La commande

```text
rythme etapes FICHIER "EXPRESSION"
```

- `FICHIER` donne les définitions : vos fonctions, vos types, vos valeurs.
  Ses `print` et autres expressions ne sont pas lancés.
- `EXPRESSION`, entre guillemets, est le calcul à montrer.
- `rythme steps` est le nom anglais de la même commande, et `rythme étapes`
  marche aussi.

## Comprendre la récursion

Une fonction [récursive](../debuter/glossaire.md#recursion) s'appelle
elle-même. C'est souvent difficile à imaginer : les étapes la rendent
visible. Voici deux façons de calculer une factorielle
(4! = 1 × 2 × 3 × 4), dans `factorielle.ry` :

```rythme
fn factorielle(0) = 1
fn factorielle(n) = n * factorielle(n - 1)

fn factorielle_acc(0, acc) = acc
fn factorielle_acc(n, acc) = factorielle_acc(n - 1, n * acc)

print(factorielle(4), factorielle_acc(4, 1))
```

```sortie
24 24
```

Même résultat. Mais les étapes racontent deux histoires différentes :

```text
$ rythme etapes factorielle.ry "factorielle(4)"
factorielle(4)
  → 4 * factorielle(3)
  → 4 * (3 * factorielle(2))
  → 4 * (3 * (2 * factorielle(1)))
  → 4 * (3 * (2 * (1 * factorielle(0))))
  → 4 * (3 * (2 * (1 * 1)))
  → 4 * (3 * (2 * 1))
  → 4 * (3 * 2)
  → 4 * 6
  → 24
```

L'expression **grandit** vers la droite : chaque multiplication attend le
résultat de l'appel suivant. Une fois `factorielle(0)` atteint, elle
**rétrécit** : les multiplications se font enfin, de l'intérieur vers
l'extérieur. C'est la forme d'une pile d'assiettes qu'on empile, puis qu'on
dépile.

```text
$ rythme etapes factorielle.ry "factorielle_acc(4, 1)"
factorielle_acc(4, 1)
  → factorielle_acc(3, 4)
  → factorielle_acc(2, 12)
  → factorielle_acc(1, 24)
  → factorielle_acc(0, 24)
  → 24
```

Ici, rien n'attend : le résultat partiel voyage dans le paramètre `acc`
(un *accumulateur*), et l'expression garde la même taille. L'appel est la
toute dernière chose que fait la fonction : c'est un **appel terminal**,
que Rythme peut répéter des millions de fois sans s'essouffler (voir
[la récursion](recursion.md)).

## Fib : les calculs répétés

Revenez aux étapes de `fib(3)` plus haut : `fib(1)` y est calculé **deux
fois**. Pour un nombre plus grand, cela devient énorme. `--max N` limite le
nombre d'étapes montrées, et `--width N` coupe les lignes trop longues :

```text
$ rythme etapes fib.ry "fib(10)" --max 4 --width 40
fib(10)
  → fib(9) + fib(8)
  → fib(8) + fib(7) + fib(8)
  → fib(7) + fib(6) + fib(7) + fib(8)
  → fib(6) + fib(5) + fib(6) + fib(7) +…
  ... (d'autres étapes, non montrées : 4 au plus sont montrées, à changer avec --max)
  → 55
```

Après les étapes montrées, `...` dit que la suite est calculée sans être
affichée, puis vient le résultat. Les appels s'additionnent sans fin dès la
quatrième ligne : voilà pourquoi cette version de `fib` est lente pour de
grands nombres.

## Les listes et map

Les fonctions intégrées, comme `map`, `sum` ou `len`, donnent leur
résultat **en une seule étape** : on ne voit pas leur intérieur. Vos
fonctions, elles, sont détaillées. Avec ce fichier, `listes.ry` :

```rythme
fn carre(x) = x * x
fn somme([]) = 0
fn somme([x, ..reste]) = x + somme(reste)

print(map([1, 2, 3], carre), somme([1, 4, 9]))
```

```sortie
[1, 4, 9] 14
```

```text
$ rythme etapes listes.ry "[1, 2, 3] |> map(carre) |> somme"
somme(map([1, 2, 3], carre))
  → somme([1, 4, 9])
  → 1 + somme([4, 9])
  → 1 + (4 + somme([9]))
  → 1 + (4 + (9 + somme([])))
  → 1 + (4 + (9 + 0))
  → 1 + (4 + 9)
  → 1 + 13
  → 14
```

Deux choses à remarquer : le `|>` est montré comme les appels qu'il fait
(`somme(map(...))`), et `map` se fait en une étape. `somme`, elle, défait
la liste élément par élément : `[x, ..reste]` prend le premier et laisse
le reste, jusqu'à la liste vide.

## Tout un fichier

Sans expression, `rythme etapes` montre chaque définition et chaque
expression du fichier, dans l'ordre. Ce que le programme affiche apparaît
au moment où il est affiché. Avec `notes.ry` :

```rythme
notes = [12, 15, 9]
moyenne = sum(notes) / len(notes)
print("moyenne : {moyenne}")
```

```sortie
moyenne : 12.0
```

```text
$ rythme etapes notes.ry
notes = [12, 15, 9]

moyenne = sum(notes) / len(notes)
  → sum([12, 15, 9]) / len(notes)
  → 36 / len(notes)
  → 36 / len([12, 15, 9])
  → 36 / 3
  → 12.0

print("moyenne : {moyenne}")
  → print("moyenne : 12.0")
moyenne : 12.0
  → ()
```

La ligne `moyenne : 12.0` sans flèche est ce que `print` affiche ; puis
`print` donne `()`, la valeur « rien d'intéressant ».

## Déboguer avec les étapes

Un programme qui donne un mauvais résultat sans message d'erreur a souvent
une erreur de **raisonnement**. Les étapes la montrent. Ce programme devait
afficher 24 :

```rythme
fn factorielle(n) = if n == 0 then 0 else n * factorielle(n - 1)

print(factorielle(4))
```

```sortie
0
```

```text
$ rythme etapes bug.ry "factorielle(3)"
factorielle(3)
  → 3 * factorielle(2)
  → 3 * (2 * factorielle(1))
  → 3 * (2 * (1 * factorielle(0)))
  → 3 * (2 * (1 * 0))
  → 3 * (2 * 0)
  → 3 * 0
  → 0
```

Le coupable saute aux yeux : `factorielle(0)` vaut `0`, et tout est
multiplié par zéro. Il fallait `then 1`. Choisissez un **petit** exemple
(`factorielle(3)` plutôt que `factorielle(20)`) : moins d'étapes, plus
facile à lire. Pour d'autres façons de chercher un bug, voir le
[débogage](../debuter/deboguer.md).

## Le mode interactif

`rythme`, sans rien d'autre, ouvre le **mode interactif** (le
[REPL](../debuter/glossaire.md#repl)) : vous tapez une ligne après l'invite
`ry> `, Rythme la vérifie, l'exécute et affiche sa valeur. C'est un
brouillon idéal pour essayer une idée. Dans le mode interactif, on peut
même redéfinir un nom ; `:etapes` y montre les étapes d'une expression :

```text
$ rythme
Rythme 0.1.0 — :help pour l'aide, :quit pour quitter
ry> 1 + 2
3
ry> fn fact(0) = 1
ry> fn fact(n) = n * fact(n - 1)
(`fact` a maintenant 2 définitions, essayées dans l'ordre ; :reset pour recommencer)
ry> :etapes fact(3)
fact(3)
  → 3 * fact(2)
  → 3 * (2 * fact(1))
  → 3 * (2 * (1 * fact(0)))
  → 3 * (2 * (1 * 1))
  → 3 * (2 * 1)
  → 3 * 2
  → 6
ry> fact(20)
2432902008176640000
ry> :quit
```

`:doc NOM` affiche la documentation d'une fonction intégrée, avec un
exemple :

```text
ry> :doc map
map(list, f)
  Applique la fonction f à chaque élément et renvoie la liste des résultats. Fonctionne aussi sur Option et Result, et sur un ensemble (en donnant l'ensemble des résultats).
  Exemple :
    [1, 2, 3] |> map(fn(x) = x * 2)   # [2, 4, 6]
```

Les autres commandes utiles : `:load fichier` charge un fichier,
`:names` liste les noms définis, `:explain E019` explique un code d'erreur,
`:reset` oublie tout, et Tab complète les noms.

> **Attention :** une fonction chargée avec `:load` vient d'un autre
> fichier : `:etapes` donne son résultat en une étape. Pour voir son
> détail, utilisez `rythme etapes fichier.ry "expression"`.

## Documenter ses fonctions pour help

`help(x)` fait la même chose que `:doc`, mais dans un programme. Pour vos
propres fonctions, il affiche les lignes `##` écrites juste au-dessus de
`fn` : c'est un **commentaire de documentation**.

```rythme norun
## L'aire d'un rectangle.
## Les deux côtés sont des nombres, dans la même unité.
fn aire(largeur, hauteur) = largeur * hauteur

help(aire)
```

```text
$ rythme aire.ry
aire(largeur, hauteur)
  L'aire d'un rectangle.
  Les deux côtés sont des nombres, dans la même unité.
  définie en aire.ry:3:4
```

La dernière ligne dit où la fonction est définie : fichier `aire.ry`,
ligne 3, colonne 4. Pour une fonction intégrée, `help` montre un exemple :

```rythme
help(len)
```

```sortie
len(x)
  Longueur d'un texte (en caractères), d'une liste, d'un enregistrement, d'un dictionnaire ou d'un ensemble.
  Pareil que : `length`, `size` (ses noms Ruby)
  Exemple :
    len("été")                 # 3
    len([1, 2])                # 2
```

Dans le mode interactif, après `:load` de ce fichier, `:doc aire` montre la
même chose. Sans commentaire `##`, `help` le dit, et rappelle comment en
écrire un.

## Erreurs fréquentes

### Oublier le cas de base

```rythme norun
fn compte_a_rebours(n) = if n == 0 then "partez !" else compte_a_rebours(n - 2)
```

```text
$ rythme etapes sans_fin.ry "compte_a_rebours(5)" --max 6
compte_a_rebours(5)
  → compte_a_rebours(3)
  → compte_a_rebours(1)
  → compte_a_rebours(-1)
  → compte_a_rebours(-3)
  → compte_a_rebours(-5)
  → compte_a_rebours(-7)
  ... (d'autres étapes, non montrées : 6 au plus sont montrées, à changer avec --max)
erreur: l'évaluation a été arrêtée après 10000000 étapes : se termine-t-elle un jour ?
 --> sans_fin.ry:1:57
  |
1 | fn compte_a_rebours(n) = if n == 0 then "partez !" else compte_a_rebours(n - 2)
  |                                                         ^^^^^^^^^^^^^^^^^^^^^^^
  |
  = aide: une fonction qui s'appelle elle-même doit toujours atteindre un cas où elle s'arrête
```

En partant de 5 et en enlevant 2, on saute par-dessus 0 : la fonction ne
s'arrête jamais. Les étapes le montrent tout de suite. Correction : tester
`n <= 0` au lieu de `n == 0`.

### Se tromper de nom dans l'expression

```text
$ rythme etapes fib.ry "fibo(3)"
erreur[E003]: nom inconnu `fibo`
 --> fib.ry:4:1
  |
4 | fibo(3)
  | ^^^^ non défini
  |
  = aide: vouliez-vous dire `fib` ?
  = en savoir plus : `rythme explain E003`
```

L'expression est vérifiée comme si elle était écrite à la fin du fichier :
d'où la « ligne 4 » d'un fichier qui n'en a que 3. Elle ne peut utiliser
que les noms que le fichier définit. Ici, l'aide donne le bon nom :
`rythme etapes fib.ry "fib(3)"`. Gardez aussi les guillemets autour de
l'expression : sans eux, le terminal donne lui-même un sens aux
parenthèses.

## Testez-vous

**Question 1 :** dans les étapes de `factorielle(4)`, pourquoi l'expression
grandit-elle avant de rétrécir ?

> **Réponse**
>
> Chaque `n * factorielle(n - 1)` doit attendre le résultat de l'appel
> suivant pour multiplier. Les multiplications ne se font qu'une fois
> `factorielle(0)` atteint.

**Question 2 :** pourquoi `map([1, 2, 3], carre)` se fait-il en une seule
étape ?

> **Réponse**
>
> `map` est une fonction intégrée : les fonctions intégrées donnent leur
> résultat en une étape. Seules vos fonctions sont détaillées.

**Question 3 :** comment faire afficher par `help(moyenne)` une phrase qui
explique votre fonction `moyenne` ?

> **Réponse**
>
> Écrivez cette phrase dans un commentaire `## ...` sur la ligne juste
> au-dessus de `fn moyenne(...)`.

## En résumé

- `rythme etapes fichier.ry "expression"` montre un calcul une étape à la
  fois : chaque appel est remplacé par le corps de la fonction.
- Les fonctions intégrées donnent leur résultat en une étape ; les vôtres
  sont détaillées.
- Les étapes rendent la récursion visible, et révèlent les erreurs de
  raisonnement : testez sur un petit exemple.
- `--max N` limite les étapes montrées, `--width N` coupe les lignes.
- Le mode interactif (`rythme`) essaie du code ligne par ligne ; `:etapes`,
  `:doc` et `help(x)` y sont disponibles, et `##` documente vos fonctions.

Suite : [Tester son code](tests.md)
