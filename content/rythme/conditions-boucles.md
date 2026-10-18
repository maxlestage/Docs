# Conditions et répétition

Cette page montre comment choisir entre plusieurs cas avec `if`, `unless` et `match`, et comment répéter une action sans boucle.

## if donne une valeur

En Rythme, `if` n'est pas une instruction : c'est une **expression**. Il
donne une valeur, comme un calcul. On peut donc la ranger dans un nom :

```rythme
age = 20
statut = if age >= 18 then "majeur" else "mineur"
print(statut)
```

```sortie
majeur
```

La forme est toujours `if condition then valeur else autre_valeur`.

## Le else est obligatoire

Comme `if` donne toujours une valeur, il lui faut les deux cas. Un `if` sans
`else` est une erreur :

```rythme norun
age = 20
print(if age >= 18 then "majeur")
```

```text
erreur[E019]: ce `if` n'a pas de `else`
 --> majeur.ry:2:7
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

Quand il n'y a rien à faire dans un cas, donnez la valeur vide `()` :

```rythme
x = 5
if x > 0 then print("positif") else ()
```

```sortie
positif
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

La condition doit être `true` ou `false`. `if 1 then ...` est une erreur :
rien n'est « vrai » par défaut. Pour combiner des conditions, utilisez
`and`, `or` et `not` :

```rythme
x = 7
print(x > 0 and x < 10, x < 0 or x == 7, not (x == 7))
```

```sortie
true true false
```

## unless, le contraire de if

Comme en Ruby, `unless` est le contraire de `if`. Lui aussi a toujours un
`else` :

```rythme
stock = 0
print(unless stock > 0 then "épuisé" else "en stock")
```

```sortie
épuisé
```

## Choisir avec match

`match` compare une valeur à plusieurs **motifs**, dans l'ordre. Le premier
qui correspond donne le résultat. `_` correspond à tout le reste :

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

Un motif peut aussi être un intervalle de nombres :

```rythme
fn mention(note) = match note
  16..20 -> "très bien"
  12..15 -> "bien"
  _ -> "à retravailler"
end

print(mention(17), mention(13), mention(8))
```

```sortie
très bien bien à retravailler
```

`match` sait faire beaucoup plus : voir [vos propres types et match](types.md).

## Pas de boucles

Rythme n'a ni `for` ni `while`. Si vous en écrivez une, Rythme vous explique
quoi faire à la place :

```rythme norun
nombres = [1, 2, 3]
for n in nombres
  print(n)
end
```

```text
erreur[E001]: `for` n'est pas utilisé en Rythme
 --> boucle.ry:2:1
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

Au lieu de dire « pour chaque élément, fais ceci », on dit « fais ceci à
toute la liste ». Voici les outils.

## Faire quelque chose pour chaque élément avec each

`each` appelle une fonction pour chaque élément. C'est le remplaçant de la
boucle qui affiche. Ici, `1..3` est l'**intervalle** des nombres de 1 à 3,
c'est-à-dire la liste `[1, 2, 3]` :

```rythme
1..3 |> each(fn(i) = print("Tour numéro {i}"))
```

```sortie
Tour numéro 1
Tour numéro 2
Tour numéro 3
```

Le même, à la manière de Ruby, avec un bloc :

```rythme
["Ada", "Alan"].each do |nom|
  puts "Bonjour, #{nom} !"
end
```

```sortie
Bonjour, Ada !
Bonjour, Alan !
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

## Répéter avec la récursion

Pour une boucle « tant que », écrivez une fonction qui s'appelle elle-même.
Il faut toujours un cas où elle s'arrête :

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

Ici, la fonction a deux définitions : une pour `0`, une pour les autres
nombres. Voir [les fonctions](fonctions.md#definir-par-cas).

## En résumé

- `if condition then a else b` donne une valeur, et le `else` est
  obligatoire.
- `match` compare une valeur à des motifs, dans l'ordre ; `_` attrape tout
  le reste.
- Il n'y a pas de boucles : `each` fait quelque chose pour chaque élément,
  `map` transforme chaque élément.
- Pour « tant que », on écrit une fonction qui s'appelle elle-même, avec un
  cas où elle s'arrête.

Suite : [les fonctions](fonctions.md). Comparez avec les conditions et
boucles de [Ruby](../ruby/conditions-boucles.md) et de
[Rust](../rust/conditions-boucles.md).
