# Penser fonctionnel

Cette page explique la façon de penser de Rythme : des valeurs qui ne changent pas, des enchaînements de fonctions avec `|>` ou des points, des blocs à la Ruby, et la récursion.

## Des valeurs qui ne changent pas

Une fonction de Rythme ne modifie jamais ce qu'elle reçoit. Elle donne une
**nouvelle** valeur :

```rythme
nombres = [3, 1, 2]
tries = sort(nombres)
print(nombres, tries)
```

```sortie
[3, 1, 2] [1, 2, 3]
```

C'est plus simple à suivre : une valeur que vous avez nommée garde son sens
jusqu'au bout du programme. Et une fonction donne toujours le même résultat
pour les mêmes arguments.

## Enchaîner avec |>

`x |> f` veut dire `f(x)`. Et `x |> f(a)` veut dire `f(x, a)` : la valeur
devient le **premier argument**. On lit alors le calcul dans l'ordre où il
se fait :

```rythme
resultat = [1, 2, 3, 4, 5, 6]
  |> filter(is_even)
  |> map(fn(x) = x * x)
  |> sum
print(resultat)
```

```sortie
56
```

Sans `|>`, il faudrait lire à l'envers : `sum(map(filter(...), ...))`.

## Enchaîner avec des points

Le même calcul, à la manière de Ruby, avec des appels en style méthode.
Chaque ligne qui commence par `.` continue la précédente :

```rythme
resultat = [1, 2, 3, 4, 5, 6]
  .select(even?)
  .map(fn(x) = x * x)
  .sum
print(resultat)
```

```sortie
56
```

`select` et `even?` sont les noms Ruby de `filter` et `is_even`.

## Les blocs à la Ruby

Au lieu d'écrire `fn(x) = ...`, vous pouvez donner un **bloc** à la fin
d'un appel. Deux formes existent :

- `do |x| ... end`, souvent sur plusieurs lignes ;
- `{ |x| ... }`, souvent sur une ligne.

```rythme
carres = [1, 2, 3].map { |x| x * x }
print(carres)

[1, 2, 3].each do |x|
  double = x * 2
  puts "#{x} fois 2 = #{double}"
end
```

```sortie
[1, 4, 9]
1 fois 2 = 2
2 fois 2 = 4
3 fois 2 = 6
```

Un bloc est exactement une fonction anonyme : `xs.map { |x| x * x }` est
`map(xs, fn(x) = x * x)`.

## map, filter et reject

- `map` transforme chaque élément ;
- `filter` (ou `select`) garde ceux qui répondent `true` ;
- `reject` enlève ceux qui répondent `true`.

```rythme
mots = ["chat", "éléphant", "rat", "girafe"]
print(mots.map { |m| m.upper })
print(mots.filter { |m| len(m) > 3 })
print(mots.reject { |m| len(m) > 3 })
```

```sortie
["CHAT", "ÉLÉPHANT", "RAT", "GIRAFE"]
["chat", "éléphant", "girafe"]
["rat"]
```

## fold et reduce

`fold(liste, depart, f)` parcourt la liste en gardant un **accumulateur**.
`f` reçoit l'accumulateur et l'élément, et donne le nouvel accumulateur.
`reduce` est son nom Ruby :

```rythme
total = fold([1, 2, 3, 4], 0, fn(acc, x) = acc + x)
plus_long = ["kiwi", "banane", "figue"].reduce("") do |meilleur, m|
  if len(m) > len(meilleur) then m else meilleur
end
print(total, plus_long)
```

```sortie
10 banane
```

## Remplacer les boucles

| En Python ou en Ruby | En Rythme |
|----------------------|-----------|
| une boucle qui affiche | `each` |
| une boucle qui remplit une liste | `map` |
| une boucle avec un `if` qui garde | `filter`, `reject` |
| une boucle qui additionne | `sum`, `fold` |
| une boucle qui cherche | `find`, `any`, `all` |
| `break` | `take_while` |
| `while` | une fonction récursive |

```rythme
nombres = [4, 8, 15, 16, 23, 42]
print(find(nombres, is_odd), any(nombres, fn(x) = x > 40), all(nombres, fn(x) = x > 0))
print(take_while(nombres, is_even))
```

```sortie
Some(15) true true
[4, 8]
```

## Répéter avec un accumulateur

Une fonction peut s'appeler elle-même. Pour qu'elle puisse se répéter
**des millions de fois**, l'appel à elle-même doit être la toute dernière
chose qu'elle fait. L'astuce : on garde le résultat partiel dans un
paramètre en plus, l'**accumulateur**.

```rythme
fn somme([], acc) = acc
fn somme([x, ..reste], acc) = somme(reste, acc + x)

print(somme(1..1000000, 0))
```

```sortie
500000500000
```

Ici, `somme` reçoit la liste et le total déjà calculé. Quand la liste est
vide, le total est la réponse.

> **Astuce :** quand une fonction de liste existe (`sum`, `fold`, `map`),
> préférez-la : elle n'a pas de limite et elle se lit mieux.

## Une boucle tant que

Voici la suite de Collatz : tant que `n` ne vaut pas 1, on le divise par 2
s'il est pair, sinon on calcule `3 * n + 1`. On compte les étapes :

```rythme
fn collatz(1, etapes) = etapes
fn collatz(n, etapes) =
  if n % 2 == 0 then collatz(n // 2, etapes + 1)
  else collatz(3 * n + 1, etapes + 1)

print(collatz(27, 0))
```

```sortie
111
```

## En résumé

- Une fonction ne modifie jamais ce qu'elle reçoit : elle donne une
  nouvelle valeur.
- `x |> f(a)` veut dire `f(x, a)` : on lit le calcul dans l'ordre.
- Un bloc `{ |x| ... }` ou `do |x| ... end` est une fonction sans nom.
- `map`, `filter`, `reject` et `fold` remplacent les boucles.
- Pour répéter « tant que », une fonction récursive, si possible avec un
  accumulateur.

Suite : [vos propres types](types.md). En Ruby, les blocs ont
[leur propre page](../ruby/blocs.md).
