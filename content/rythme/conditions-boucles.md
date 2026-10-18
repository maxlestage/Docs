# Conditions et répétition

Cette page montre comment choisir entre plusieurs cas avec `if`, `unless` et `match`, et comment répéter une action sans boucle, avec `each`, `map` et la récursion.

> **Objectifs :**
>
> - vous saurez écrire une condition avec `==`, `<`, `and`, `or` et `not` ;
> - vous saurez choisir une valeur avec `if ... then ... else`, et pourquoi le `else` est obligatoire ;
> - vous saurez comparer une valeur à plusieurs cas avec `match` ;
> - vous saurez répéter une action sans boucle, avec `each`, `each_with_index` et `map` ;
> - vous saurez écrire une répétition « tant que » avec une fonction récursive.

## Vrai ou faux : les comparaisons

Une [condition](../debuter/glossaire.md#condition) est une question dont la
réponse est `true` (vrai) ou `false` (faux), un
[booléen](../debuter/glossaire.md#booleen). On la pose avec un opérateur de
comparaison :

| Opérateur | Question | Exemple | Résultat |
|-----------|----------|---------|----------|
| `==` | égal ? | `3 == 3` | `true` |
| `!=` | différent ? | `3 != 3` | `false` |
| `<`, `>` | plus petit, plus grand ? | `2 < 5` | `true` |
| `<=`, `>=` | plus petit ou égal, plus grand ou égal ? | `5 >= 5` | `true` |

> **Attention :** `=` donne un nom, `==` compare. Ce sont deux choses
> différentes.

Pour combiner des conditions, on utilise des mots :

- `a and b` : vrai si les **deux** sont vraies ;
- `a or b` : vrai si **au moins une** est vraie ;
- `not a` : le contraire de `a`.

```rythme
x = 7
print(x > 0 and x < 10, x < 0 or x == 7, not (x == 7))
```

```sortie
true true false
```

## if donne une valeur

`if` choisit entre deux valeurs selon une condition. En Rythme, `if` n'est
pas un ordre : c'est une **expression**. Il donne une valeur, comme un
calcul. On peut donc la ranger dans un nom :

```rythme
age = 20
statut = if age >= 18 then "majeur" else "mineur"
print(statut)
```

```sortie
majeur
```

La forme est toujours `if condition then valeur else autre_valeur`. Lisez :
« si l'âge est au moins 18, alors "majeur", sinon "mineur" ».

## Le else est obligatoire

Comme `if` donne toujours une valeur, il lui faut les deux cas : que
vaudrait `statut` si la condition était fausse ? Un `if` sans `else` est donc
une erreur (voir [les erreurs fréquentes](#erreurs-frequentes)).

Quand il n'y a rien à faire dans un cas, donnez la valeur vide `()`, qui
veut dire « rien d'intéressant » :

```rythme
x = 5
if x > 0 then print("positif") else ()
```

```sortie
positif
```

Une branche contient une seule expression. Pour faire plusieurs choses,
regroupez-les dans un bloc `do ... end` : sa dernière ligne donne la valeur.

```rythme
fn classer(n) =
  if n > 100 then do
    print("un grand nombre !")
    "grand"
  end else "petit"

print(classer(500), classer(3))
```

```sortie
un grand nombre !
grand petit
```

## Enchaîner les conditions

`else if` enchaîne plusieurs conditions. `then` et `else` peuvent commencer
une nouvelle ligne :

```rythme
fn signe(n) =
  if n < 0 then "négatif"
  else if n == 0 then "zéro"
  else "positif"

print(signe(-3), signe(0), signe(8))
```

```sortie
négatif zéro positif
```

Les conditions sont essayées de haut en bas, et la première vraie gagne.

La condition doit être `true` ou `false`. `if 1 then ...` est une erreur :
rien n'est « vrai » par défaut, ni un nombre, ni un texte, ni une liste.

## unless, le contraire de if

Comme en Ruby, `unless` (« à moins que ») est le contraire de `if` : il
donne la première valeur quand la condition est **fausse**. Lui aussi a
toujours un `else` :

```rythme
stock = 0
print(unless stock > 0 then "épuisé" else "en stock")
```

```sortie
épuisé
```

`unless stock > 0 then a else b` est exactement
`if stock > 0 then b else a`. Utilisez celui qui se lit le mieux.

## Choisir avec match

Quand il y a beaucoup de cas, `match` est plus clair qu'une suite de
`else if`. Il compare une valeur à plusieurs **motifs**, dans l'ordre. Le
premier qui correspond donne le résultat. `_` correspond à tout le reste :

```rythme
fn jour(n) = match n
  6 -> "samedi"
  7 -> "dimanche"
  _ -> "un jour de semaine"
end

print(jour(6), jour(2))
```

```sortie
samedi un jour de semaine
```

Chaque ligne s'écrit `motif -> résultat`, et `end` ferme le `match`.
Pensez à un tri de courrier : chaque lettre va dans la première case qui
lui convient.

Un motif peut aussi être un intervalle de nombres (`16..20`, les deux bouts
compris), ou plusieurs possibilités séparées par `|` :

```rythme
fn mention(note) = match note
  16..20 -> "très bien"
  12..15 -> "bien"
  10 | 11 -> "passable"
  _ -> "à retravailler"
end

print(mention(17), mention(13), mention(10), mention(8))
```

```sortie
très bien bien passable à retravailler
```

Un motif peut même reconnaître le début d'un texte :

```rythme
fn saluer("Dr " + nom) = "Bonjour docteur {nom}"
fn saluer(nom) = "Bonjour {nom}"

print(saluer("Dr Grace"), "/", saluer("Ada"))
```

```sortie
Bonjour docteur Grace / Bonjour Ada
```

`match` sait faire beaucoup plus : voir [vos propres types et match](types.md).

## Pas de boucles

Rythme n'a ni `for` ni `while`. Si vous en écrivez une, Rythme vous explique
quoi faire à la place (voir [les erreurs fréquentes](#erreurs-frequentes)).

Au lieu de dire « pour chaque élément, fais ceci », on dit « fais ceci à
toute la liste ». C'est comme dire « lavez toutes les assiettes » au lieu de
« prenez l'assiette 1, lavez-la, prenez l'assiette 2... ». Voici les outils.

## Faire quelque chose pour chaque élément avec each

`each` appelle une fonction pour chaque élément. C'est le remplaçant de la
boucle qui affiche. Ici, `1..3` est l'**intervalle** des nombres de 1 à 3,
c'est-à-dire la liste `[1, 2, 3]`, et `fn(i) = ...` est une petite fonction
sans nom (voir [les fonctions](fonctions.md#les-fonctions-anonymes)) :

```rythme
1..3 |> each(fn(i) = print("Tour numéro {i}"))
```

```sortie
Tour numéro 1
Tour numéro 2
Tour numéro 3
```

Le même, à la manière de Ruby, avec un **bloc** `do |nom| ... end` :
`|nom|` donne un nom à l'élément en cours.

```rythme
["Ada", "Alan"].each do |nom|
  puts "Bonjour, #{nom} !"
end
```

```sortie
Bonjour, Ada !
Bonjour, Alan !
```

## Compter de 2 en 2, ou à l'envers

`a..b` compte de 1 en 1, en montant. `step_by(n)` garde un élément sur `n`,
et `reverse` retourne la liste :

```rythme
print(0..10 |> step_by(2))
print(reverse(1..5))
print(5..1)
```

```sortie
[0, 2, 4, 6, 8, 10]
[5, 4, 3, 2, 1]
[]
```

> **Attention :** `5..1` est vide. Pour compter à rebours, écrivez
> `reverse(1..5)`.

## Connaître la position avec each_with_index

`each_with_index` donne à la fonction l'élément **et** sa position, à
partir de 0 :

```rythme
["pain", "lait", "œufs"].each_with_index do |article, i|
  print("{i + 1}. {article}")
end
```

```sortie
1. pain
2. lait
3. œufs
```

## Transformer chaque élément avec map

`map` fabrique une **nouvelle** liste, avec le résultat de la fonction pour
chaque élément. C'est le remplaçant de la boucle qui remplit une liste :

```rythme
carres = 1..5 |> map(fn(x) = x * x)
print(carres)
```

```sortie
[1, 4, 9, 16, 25]
```

`each` sert à **faire** quelque chose (afficher) ; `map` sert à
**calculer** une nouvelle liste.

## Répéter avec la récursion

Pour une boucle « tant que », écrivez une fonction qui s'appelle
elle-même : c'est la [récursion](../debuter/glossaire.md#recursion). Il faut
toujours un cas où elle s'arrête, sinon elle ne s'arrête jamais :

```rythme
fn compte_a_rebours(0) = print("Décollage !")
fn compte_a_rebours(n)
  print(n)
  compte_a_rebours(n - 1)
end

compte_a_rebours(3)
```

```sortie
3
2
1
Décollage !
```

Ici, la fonction a deux définitions : une pour `0` (le cas d'arrêt), une
pour les autres nombres. `compte_a_rebours(3)` affiche 3 puis appelle
`compte_a_rebours(2)`, et ainsi de suite jusqu'à 0. Voir
[les fonctions](fonctions.md#definir-par-cas).

## Erreurs fréquentes

### Écrire une boucle for

```rythme norun
nombres = [1, 2, 3]
for n in nombres
  print(n)
end
```

```text
erreur[E001]: `for` n'est pas utilisé en Rythme
 --> main.ry:2:1
  |
2 | for n in nombres
  | ^^^ vient d'un autre langage
  |
  = aide: Rythme n'a pas de boucles. Travaillez sur toute la liste à la place :
        1..5 |> each(fn(i) = print(i))
        [1, 2, 3] |> map(fn(x) = x * 2)
      ou écrivez une fonction récursive. Voir `rythme help functional`.
  = en savoir plus : `rythme explain E001`
```

Écrivez `nombres.each do |n| print(n) end`, ou
`nombres |> each(fn(n) = print(n))`. `while` donne la même erreur.

### Oublier le else

```rythme norun
age = 20
print(if age >= 18 then "majeur")
```

```text
erreur[E019]: ce `if` n'a pas de `else`
 --> main.ry:2:7
  |
2 | print(if age >= 18 then "majeur")
  |       ^^ chaque `if` doit avoir un `else`
  |
2 | print(if age >= 18 then "majeur")
  |                                 - `else` attendu ici
  |
  = aide: en Rythme, `if` donne toujours une valeur, il lui faut donc les deux cas :
        if age >= 18 then "majeur" else "mineur"
  = en savoir plus : `rythme explain E019`
```

Ajoutez l'autre cas : `else "mineur"`, ou `else ()` s'il n'y a rien à faire.

### Comparer avec = au lieu de ==

```rythme norun
x = 5
if x = 5 then print("cinq") else ()
```

```text
erreur[E001]: `then` attendu, mais trouvé `=`
 --> main.ry:2:6
  |
2 | if x = 5 then print("cinq") else ()
  |      ^ `then` attendu ici
  |
  = aide: pour comparer deux valeurs, utilisez `==`
  = aide: un `if` s'écrit : `if condition then valeur else autre_valeur`
  = en savoir plus : `rythme explain E001`
```

Pour comparer, deux signes égal : `if x == 5 then ...`.

### Enchaîner deux comparaisons

```rythme norun
note = 15
print(if 10 <= note <= 20 then "ok" else "non")
```

```text
erreur[E001]: les comparaisons ne peuvent pas s'enchaîner
 --> main.ry:2:21
  |
2 | print(if 10 <= note <= 20 then "ok" else "non")
  |                     ^^
  |
  = aide: combinez-les avec `and` : `a < b and b < c`
  = en savoir plus : `rythme explain E001`
```

Écrivez les deux questions et reliez-les : `10 <= note and note <= 20`.
`&&` et `||` n'existent pas non plus : écrivez `and` et `or`.

## Testez-vous

**Question 1 :** que vaut `unless 3 > 5 then "a" else "b"` ?

> **Réponse**
>
> `"a"`. La condition `3 > 5` est fausse, et `unless` donne la première
> valeur quand la condition est fausse.

**Question 2 :** qu'affiche ce programme ?

```rythme norun
fn taille(n) = match n
  0 -> "rien"
  1..9 -> "petit"
  _ -> "grand"
end
print(taille(0), taille(7), taille(42))
```

> **Réponse**
>
> `rien petit grand`. Chaque nombre va au premier motif qui lui convient.

**Question 3 :** comment afficher les nombres pairs de 2 à 10, sans boucle ?

> **Réponse**
>
> Par exemple `2..10 |> step_by(2) |> each(fn(n) = print(n))`, ou
> `2..10 |> filter(is_even) |> each(fn(n) = print(n))`.

## En résumé

- `==`, `!=`, `<`, `>`, `<=`, `>=` comparent ; `and`, `or` et `not`
  combinent les conditions.
- `if condition then a else b` donne une valeur, et le `else` est
  obligatoire ; `unless` est son contraire.
- `match` compare une valeur à des motifs, dans l'ordre ; `_` attrape tout
  le reste.
- Il n'y a pas de boucles : `each` fait quelque chose pour chaque élément,
  `each_with_index` donne aussi sa position, `map` transforme chaque élément.
- `step_by` compte par pas, `reverse` compte à l'envers.
- Pour « tant que », on écrit une fonction qui s'appelle elle-même, avec un
  cas où elle s'arrête.

Suite : [les fonctions](fonctions.md). Comparez avec les conditions et
boucles de [Ruby](../ruby/conditions-boucles.md) et de
[Rust](../rust/conditions-boucles.md).
