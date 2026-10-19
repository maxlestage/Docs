# Suivre un programme pas à pas

Cette page vous apprend à faire tourner un programme Rythme « dans votre tête », avec un tableau qui suit les valeurs à chaque étape, sur six petits programmes de plus en plus riches.

> **Objectifs :**
>
> - vous saurez remplir un tableau de suivi, une ligne par étape ;
> - vous saurez suivre une condition, un `fold`, un appel de fonction et une fonction récursive ;
> - vous saurez deviner ce qu'un programme affiche **avant** de le lancer ;
> - vous saurez faire afficher ses étapes à un programme, avec `print`, `debug`, `scan` et `rythme etapes`.

## Pourquoi suivre un programme à la main

Un ordinateur suit les instructions une par une, sans jamais sauter une
ligne. Pour comprendre un programme, faites pareil : jouez l'ordinateur.

C'est comme suivre une recette de cuisine avec le doigt : on lit une ligne,
on fait ce qu'elle dit, on note où on en est, puis on passe à la suivante.

Votre outil est un **tableau de suivi** : une colonne par nom, une ligne par
étape. Il vous faut une feuille et un crayon.

```text
 ┌───────┬──────────────────────┬────────┬────────┐
 │ Ligne │ Ce qui se passe      │ nom 1  │ nom 2  │
 ├───────┼──────────────────────┼────────┼────────┤
 │   1   │ ...                  │  ...   │        │
 │   2   │ ...                  │  ...   │  ...   │
 └───────┴──────────────────────┴────────┴────────┘
```

Chaque exemple de cette page suit la même méthode :

1. lisez le code ;
2. **avant de lancer, devinez ce qui s'affiche**, et notez-le ;
3. remplissez le tableau de suivi ;
4. lancez le programme et comparez.

> **À retenir :** se tromper dans sa devinette n'est pas grave. C'est
> justement là qu'on apprend : le tableau montre où votre idée et
> l'ordinateur se séparent.

## Une particularité de Rythme : les valeurs ne changent jamais

Dans beaucoup de langages, une [variable](../debuter/glossaire.md#variable)
est une boîte dont on change le contenu. En Rythme, une
[valeur](../debuter/glossaire.md#valeur) est
[immuable](../debuter/glossaire.md#immuable) : un nom, une fois donné, garde
sa valeur jusqu'au bout (voir [les pièges](pieges.md)).

Bonne nouvelle pour le suivi : dans votre tableau, **chaque case se remplit
une seule fois**. On n'efface jamais rien. Le tableau descend en escalier.

Et comme il n'y a pas de boucle, les répétitions se font avec `map`,
`fold` ou une fonction récursive. On les suit avec une ligne par **tour**.

## Programme 1 : des noms les uns après les autres

```rythme
prix = 8
quantite = 3
total = prix * quantite
reduction = total // 4
a_payer = total - reduction
print("À payer : {a_payer} €")
```

**Avant de lancer, devinez ce qui s'affiche.** Rappel : `//` est la
division entière, elle garde seulement la partie entière.

Le tableau de suivi, ligne par ligne :

| Ligne | Ce qui se passe | prix | quantite | total | reduction | a_payer |
|-------|-----------------|------|----------|-------|-----------|---------|
| 1 | `prix` reçoit 8 | 8 | | | | |
| 2 | `quantite` reçoit 3 | 8 | 3 | | | |
| 3 | `8 * 3` | 8 | 3 | 24 | | |
| 4 | `24 // 4` | 8 | 3 | 24 | 6 | |
| 5 | `24 - 6` | 8 | 3 | 24 | 6 | 18 |
| 6 | affiche le texte | 8 | 3 | 24 | 6 | 18 |

```sortie
À payer : 18 €
```

À la ligne 6, Rythme remplace `{a_payer}` par la valeur de la colonne
`a_payer` : 18. Regardez l'escalier : chaque ligne ajoute **une** case, et
les cases déjà remplies ne bougent plus.

Essayez : mettez `quantite = 5`. Remplissez d'abord le tableau : `total`
vaut 40, `reduction` 10, et le programme affiche `À payer : 30 €`.

> **À retenir :** une ligne du programme, une ligne du tableau. Pour
> calculer, cherchez la valeur d'un nom dans **sa colonne**, plus haut.

## Programme 2 : une condition

```rythme
temperature = 18
pluie = true
tenue = if temperature >= 25 then "un short"
  else if temperature >= 15 then "un pull"
  else "un manteau"
accessoire = if pluie then " et un parapluie" else ""
print("Aujourd'hui : {tenue}{accessoire}")
```

**Avant de lancer, devinez ce qui s'affiche.**

Pour une [condition](../debuter/glossaire.md#condition), ajoutez deux
colonnes : la **question** posée, et la **réponse**, `true` ou `false`.

| Ligne | Question | Réponse | Ce qui se passe | tenue | accessoire |
|-------|----------|---------|-----------------|-------|------------|
| 1-2 | | | `temperature` = 18, `pluie` = true | | |
| 3 | `18 >= 25` ? | `false` | on passe au `else` | | |
| 4 | `18 >= 15` ? | `true` | on prend `"un pull"` | `"un pull"` | |
| 6 | `pluie` ? | `true` | on prend le texte du `then` | `"un pull"` | `" et un parapluie"` |
| 7 | | | on colle les deux textes | `"un pull"` | `" et un parapluie"` |

```sortie
Aujourd'hui : un pull et un parapluie
```

Les questions sont posées **de haut en bas**, et la première réponse
`true` gagne : la ligne 5 (`"un manteau"`) n'est jamais lue.

```text
 18 >= 25 ? ── non ──► 18 >= 15 ? ── oui ──► "un pull"   (on s'arrête là)
     │
    oui
     ▼
 "un short"
```

Essayez : mettez `temperature = 30` et `pluie = false`. Le programme
affiche `Aujourd'hui : un short` : la première question répond `true`, et
`accessoire` vaut le texte vide `""`.

> **À retenir :** pour une condition, écrivez la question et sa réponse
> dans le tableau. Seule la branche choisie est calculée.

## Programme 3 : additionner avec fold

En Rythme, une « boucle qui additionne » s'écrit avec
[fold](fonctionnel.md#fold-et-reduce). `fold` porte une valeur d'un tour à
l'autre, l'**accumulateur** (`acc`), comme un panier qu'on remplit en
avançant dans un magasin.

```rythme
achats = [4, 10, 3, 7]
total = fold(achats, 0, fn(acc, x) = acc + x)
print("Total : {total}")
```

**Avant de lancer, devinez ce qui s'affiche.**

`fold` part de `0`, puis appelle la petite fonction une fois **par
élément**. Chaque tour reçoit l'accumulateur du tour précédent :

| Tour | acc (reçu) | x | acc + x (donné au tour suivant) |
|------|------------|---|---------------------------------|
| départ | | | 0 |
| 1 | 0 | 4 | 4 |
| 2 | 4 | 10 | 14 |
| 3 | 14 | 3 | 17 |
| 4 | 17 | 7 | 24 |
| fin | | | `total` = 24 |

```sortie
Total : 24
```

Le secret d'un bon tableau pour `fold` : la colonne **acc (reçu)** d'un
tour est la dernière colonne du tour **d'avant**. Suivez la flèche :

```text
 0 ──(+4)──► 4 ──(+10)──► 14 ──(+3)──► 17 ──(+7)──► 24
```

Rythme peut vérifier votre tableau. `scan` fait le même calcul que `fold`,
mais donne **tous** les résultats intermédiaires, c'est-à-dire votre
dernière colonne :

```rythme
achats = [4, 10, 3, 7]
print(scan(achats, 0, fn(acc, x) = acc + x))
```

```sortie
[4, 14, 17, 24]
```

Essayez : remplacez `0` par `100` dans le `fold`, comme un porte-monnaie
qui contient déjà 100 €. Le tableau commence à 100, et le programme
affiche `Total : 124`.

> **À retenir :** pour `fold`, une ligne par élément ; l'accumulateur reçu
> est toujours le résultat de la ligne d'au-dessus.

## Programme 4 : une fonction appelée deux fois

```rythme
fn ttc(ht) = ht + ht * 20 // 100

livre = ttc(15)
stylo = ttc(5)
print(livre, stylo, livre + stylo)
```

**Avant de lancer, devinez ce qui s'affiche.** `ttc` ajoute 20 % à un prix.

Une [fonction](../debuter/glossaire.md#fonction) est une petite machine :
on y entre une valeur, elle en ressort une autre. À chaque appel, son
[paramètre](../debuter/glossaire.md#parametre) `ht` prend la valeur donnée,
le temps de l'appel seulement.

```text
 programme                         fonction ttc
 ─────────                         ────────────
 livre = ttc(15)  ── ht = 15 ──►   15 + 15 * 20 // 100
                                   = 15 + 300 // 100
                                   = 15 + 3
                  ◄── 18 ───────   = 18
 stylo = ttc(5)   ── ht = 5 ───►   5 + 5 * 20 // 100
                                   = 5 + 100 // 100
                                   = 5 + 1
                  ◄── 6 ────────   = 6
```

Le tableau de suivi a une colonne pour `ht`. Elle n'a de valeur
**que pendant** un appel :

| Étape | Qui travaille | ht | Calcul | livre | stylo |
|-------|---------------|----|--------|-------|-------|
| 1 | le programme appelle `ttc(15)` | 15 | | | |
| 2 | `ttc` | 15 | `15 + 300 // 100` = 18 | | |
| 3 | retour : `livre` reçoit 18 | | | 18 | |
| 4 | le programme appelle `ttc(5)` | 5 | | 18 | |
| 5 | `ttc` | 5 | `5 + 100 // 100` = 6 | 18 | |
| 6 | retour : `stylo` reçoit 6 | | | 18 | 6 |
| 7 | `print`, avec `18 + 6` = 24 | | | 18 | 6 |

```sortie
18 6 24
```

Les deux appels sont indépendants : le `ht` du premier appel est oublié
avant le second. C'est comme une machine à café : chaque tasse est
préparée à partir de zéro.

Essayez : ajoutez `cahier = ttc(ttc(10))`. Suivez d'abord l'appel du
**dedans** : `ttc(10)` donne 12, puis `ttc(12)` donne `12 + 240 // 100`,
soit 14.

> **À retenir :** à chaque appel, notez la valeur du paramètre, calculez,
> puis rapportez le résultat là où l'appel était écrit.

## Programme 5 : une liste, avec une condition dedans

On compte les élèves reçus (note de 10 ou plus) :

```rythme
notes = [12, 7, 15, 9, 10]

fn compter_si_recu(compte, note) =
  if note >= 10 then compte + 1 else compte

recus = fold(notes, 0, compter_si_recu)
print("{recus} reçus sur {len(notes)}")
```

**Avant de lancer, devinez ce qui s'affiche.** Attention à la note 10.

C'est un `fold` (programme 3) avec une condition (programme 2) à chaque
tour. Le tableau réunit les deux :

| Tour | compte (reçu) | note | `note >= 10` ? | compte (donné) |
|------|---------------|------|----------------|----------------|
| départ | | | | 0 |
| 1 | 0 | 12 | `true` | 1 |
| 2 | 1 | 7 | `false` | 1 |
| 3 | 1 | 15 | `true` | 2 |
| 4 | 2 | 9 | `false` | 2 |
| 5 | 2 | 10 | `true` | 3 |

```sortie
3 reçus sur 5
```

Au tour 5, `10 >= 10` est vrai : `>=` veut dire « plus grand **ou
égal** ». Avec `>`, l'élève qui a 10 ne serait pas compté.

Quand vous avez compris le tableau, voici la version courte. `count` fait
le même travail en une ligne :

```rythme
notes = [12, 7, 15, 9, 10]
print(count(notes, fn(n) = n >= 10))
```

```sortie
3
```

Essayez : remplacez `>=` par `>` dans `compter_si_recu`. Seule la ligne du
tour 5 change dans le tableau (`false`, compte reste 2), et le programme
affiche `2 reçus sur 5`.

> **À retenir :** une condition dans un `fold` ajoute une colonne
> « question ? » à chaque tour. Quand la réponse est `false`, l'accumulateur
> passe tel quel.

## Programme 6 : une fonction récursive

Une fonction [récursive](../debuter/glossaire.md#recursion) s'appelle
elle-même, sur une liste plus courte, jusqu'à la liste vide (voir
[la récursion](recursion.md)). Voici une addition faite à la main :

```rythme
fn additionner([], total) = total
fn additionner([x, ..reste], total) = additionner(reste, total + x)

print(additionner([4, 10, 3, 7], 0))
```

**Avant de lancer, devinez ce qui s'affiche.**

Pour chaque appel, notez **quelle définition** est choisie, et les valeurs
des paramètres :

| Appel | Liste reçue | total reçu | Définition choisie | x | reste | Appel suivant |
|-------|-------------|------------|--------------------|---|-------|---------------|
| 1 | `[4, 10, 3, 7]` | 0 | `[x, ..reste]` | 4 | `[10, 3, 7]` | `additionner([10, 3, 7], 4)` |
| 2 | `[10, 3, 7]` | 4 | `[x, ..reste]` | 10 | `[3, 7]` | `additionner([3, 7], 14)` |
| 3 | `[3, 7]` | 14 | `[x, ..reste]` | 3 | `[7]` | `additionner([7], 17)` |
| 4 | `[7]` | 17 | `[x, ..reste]` | 7 | `[]` | `additionner([], 24)` |
| 5 | `[]` | 24 | `[]` : on s'arrête | | | résultat : 24 |

```sortie
24
```

Comparez avec le programme 3 : la colonne `total` est exactement la colonne
`acc` du `fold`. Un `fold`, c'est cette récursion déjà écrite pour vous.

```text
 additionner([4, 10, 3, 7], 0)
   └─► additionner([10, 3, 7], 4)
         └─► additionner([3, 7], 14)
               └─► additionner([7], 17)
                     └─► additionner([], 24)  ──►  24
```

Essayez : appelez `additionner([], 0)`. Le tableau n'a qu'une ligne : la
première définition est choisie tout de suite, et le programme affiche `0`.

> **À retenir :** pour une récursion, une ligne par appel. Notez quelle
> définition est choisie : c'est elle qui décide si l'on continue.

## Faire afficher ses étapes au programme

Votre tableau et le programme ne sont pas d'accord ? Demandez au programme
de montrer ses étapes.

### print, avec des étiquettes

Ajoutez un `print` **avec une étiquette** : le nom de ce que vous
affichez. Un nombre seul, au milieu d'autres, ne dit rien. Dans un `fold`,
un bloc `do ... end` permet d'afficher, puis de donner le nouvel
accumulateur (la dernière ligne du bloc) :

```rythme
notes = [12, 7, 15]
recus = fold(notes, 0, fn(compte, note) = do
  nouveau = if note >= 10 then compte + 1 else compte
  print("compte = {compte}, note = {note} -> {nouveau}")
  nouveau
end)
print("recus = {recus}")
```

```sortie
compte = 0, note = 12 -> 1
compte = 1, note = 7 -> 1
compte = 2, note = 15 -> 2
```

Hmm
