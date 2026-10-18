# Penser fonctionnel

Cette page explique la façon de penser de Rythme : des valeurs qui ne changent pas, des enchaînements de fonctions avec `|>` ou des points, des blocs à la Ruby, et la récursion.

> **Objectifs :**
>
> - vous saurez pourquoi une fonction de Rythme ne modifie jamais ce qu'elle reçoit ;
> - vous saurez enchaîner des étapes avec `|>` ou avec des points ;
> - vous saurez donner un bloc `{ |x| ... }` ou `do |x| ... end` à une fonction ;
> - vous saurez remplacer chaque sorte de boucle par `map`, `filter`, `fold`, `find`... ;
> - vous saurez écrire une fonction récursive avec un accumulateur.

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
pour les mêmes arguments, comme une touche de calculatrice : `√9` vaut 3
aujourd'hui, demain, et dans n'importe quel ordre.

C'est ce qu'on appelle la **programmation fonctionnelle** : on construit un
programme en assemblant des fonctions, au lieu de modifier des valeurs pas à
pas.

## Enchaîner avec |>

`|>` se lit « puis ». `x |> f` veut dire `f(x)`. Et `x |> f(a)` veut dire
`f(x, a)` : la valeur devient le **premier argument**. On lit alors le
calcul dans l'ordre où il se fait :

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

Étape par étape : `[1, 2, 3, 4, 5, 6]`, puis on garde les pairs
`[2, 4, 6]`, puis on les met au carré `[4, 16, 36]`, puis on additionne :
56. Sans `|>`, il faudrait lire à l'envers : `sum(map(filter(...), ...))`.

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

`select` et `even?` sont les noms Ruby de `filter` et `is_even`. Choisissez
le style que vous préférez, et gardez-le dans tout votre programme.

## Les blocs à la Ruby

Au lieu d'écrire `fn(x) = ...`, vous pouvez donner un **bloc** à la fin
d'un appel. Deux formes existent :

- `do |x| ... end`, souvent sur plusieurs lignes ;
- `{ |x| ... }`, souvent sur une ligne.

Entre les barres `|x|`, vous nommez l'élément reçu.

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
- `filter` (ou `select`) garde ceux pour lesquels la fonction répond `true` ;
- `reject` enlève ceux pour lesquels la fonction répond `true`.

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

Pensez à une passoire : `filter` garde ce qui reste dans la passoire,
`reject` garde ce qui passe à travers.

## fold et reduce

`fold(liste, depart, f)` parcourt la liste en gardant un **accumulateur**,
une valeur qui grossit à chaque élément. `f` reçoit l'accumulateur et
l'élément, et donne le nouvel accumulateur. `reduce` est son nom Ruby :

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

Pour `total`, l'accumulateur vaut 0 au départ, puis 0 + 1 = 1, puis
1 + 2 = 3, puis 3 + 3 = 6, puis 6 + 4 = 10. `scan` montre ces étapes :

```rythme
print(scan([1, 2, 3, 4], 0, fn(acc, x) = acc + x))
```

```sortie
[1, 3, 6, 10]
```

## Remplacer les boucles

| En Python ou en Ruby | En Rythme |
|----------------------|-----------|
| une boucle qui affiche | `each` |
| une boucle qui remplit une liste | `map` |
| une boucle avec un `if` qui garde | `filter`, `reject` |
| une boucle qui additionne | `sum`, `fold` |
| une boucle qui compte | `count` |
| une boucle qui cherche | `find`, `any`, `all` |
| `break` | `take_while` |
| `while` | une fonction récursive |

```rythme
nombres = [4, 8, 15, 16, 23, 42]
print(find(nombres, is_odd), any(nombres, fn(x) = x > 40), all(nombres, fn(x) = x > 0))
print(take_while(nombres, is_even), count(nombres, is_even))
```

```sortie
Some(15) true true
[4, 8] 4
```

- `find` donne le premier élément qui convient, dans une `Option` ;
- `any` répond `true` si **au moins un** élément convient, `all` si **tous**
  conviennent ;
- `take_while` prend les éléments tant qu'ils conviennent, puis s'arrête ;
- `count` compte ceux qui conviennent.

## Répéter avec un accumulateur

Une fonction peut s'appeler elle-même (la
[récursion](../debuter/glossaire.md#recursion)). Pour qu'elle puisse se
répéter **des millions de fois**, l'appel à elle-même doit être la toute
dernière chose qu'elle fait. L'astuce : on garde le résultat partiel dans un
paramètre en plus, l'**accumulateur**.

```rythme
fn somme([], acc) = acc
fn somme([x, ..reste], acc) = somme(reste, acc + x)

print(somme(1..1000000, 0))
```

```sortie
500000500000
```

Ici, `somme` reçoit la liste et le total déjà calculé. `[x, ..reste]`
sépare le premier élément du reste. Quand la liste est vide, le total est
la réponse.

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

Le « tant que » devient : « si on n'est pas arrivé, recommence avec la
valeur suivante ». Le cas `collatz(1, etapes)` est la condition d'arrêt.

## Erreurs fréquentes

### Oublier fn dans map

```rythme norun
print([1, 2, 3] |> map(x * 2))
```

```text
erreur[E003]: nom inconnu `x`
 --> main.ry:1:24
  |
1 | print([1, 2, 3] |> map(x * 2))
  |                        ^ non défini
  |
  = aide: définissez-le avant de l'utiliser : `x = ...` ou `fn x(...) = ...`
  = en savoir plus : `rythme explain E003`
```

`map` attend une **fonction**, pas un calcul. Écrivez
`map(fn(x) = x * 2)`, ou `[1, 2, 3].map { |x| x * 2 }`.

### Mettre les arguments de fold dans le désordre

```rythme norun
total = fold([1, 2, 3], fn(acc, x) = acc + x, 0)
print(total)
```

```text
erreur[E005]: `fold` attend une fonction comme argument 3, mais a reçu Int 0
 --> main.ry:1:47
  |
1 | total = fold([1, 2, 3], fn(acc, x) = acc + x, 0)
  |                                               ^ mauvaise sorte de valeur
  |
  = aide: utilisation : `fold(list, start, f)`
  = en savoir plus : `rythme explain E005`
```

L'ordre est : la liste, la valeur de départ, puis la fonction.
`fold([1, 2, 3], 0, fn(acc, x) = acc + x)`.

### Un filtre qui ne répond pas true ou false

```rythme norun
print([1, 2, 3].filter { |x| x % 2 })
```

```text
erreur[E005]: la fonction donnée à `filter` doit renvoyer true ou false, mais elle a renvoyé une valeur de type Int
 --> main.ry:1:24
  |
1 | print([1, 2, 3].filter { |x| x % 2 })
  |                        ^^^^^^^^^^^^^ renvoie une valeur de type Int
  |
  = aide: utilisation : `filter(list, f)`
  = en savoir plus : `rythme explain E005`
```

En Python, `1` compte comme vrai. Pas en Rythme : posez une vraie question,
`x % 2 == 1`.

### Calculer juste après un enchaînement

```rythme norun
print([1, 2, 3] |> sum + 1)
```

```text
erreur[E001]: `+` ne peut pas suivre directement une étape `|>`
 --> main.ry:1:24
  |
1 | print([1, 2, 3] |> sum + 1)
  |                        ^
  |
  = aide: `|>` passe après les calculs : mettez l'enchaînement entre parenthèses,
      comme `(x |> f) + 1`, ou continuez-le avec une fonction : `x |> f |> fn(v) = v + 1`
  = en savoir plus : `rythme explain E001`
```

Mettez l'enchaînement entre parenthèses : `([1, 2, 3] |> sum) + 1`.

## Testez-vous

**Question 1 :** réécrivez `upper(trim("  salut "))` avec `|>`.

> **Réponse**
>
> `"  salut " |> trim |> upper`. On lit dans l'ordre : d'abord `trim`,
> puis `upper`.

**Question 2 :** que vaut `fold([2, 3, 4], 1, fn(acc, x) = acc * x)` ?

> **Réponse**
>
> `24` : 1 × 2 = 2, puis 2 × 3 = 6, puis 6 × 4 = 24.

**Question 3 :** quelle fonction remplace une boucle qui cherche le premier
nombre négatif d'une liste ?

> **Réponse**
>
> `find(liste, fn(x) = x < 0)`. Elle donne `Some(nombre)`, ou `None` s'il
> n'y en a pas.

## En résumé

- Une fonction ne modifie jamais ce qu'elle reçoit : elle donne une
  nouvelle valeur.
- `x |> f(a)` veut dire `f(x, a)` : on lit le calcul dans l'ordre.
- Un bloc `{ |x| ... }` ou `do |x| ... end` est une fonction sans nom.
- `map`, `filter`, `reject`, `fold`, `find` et `count` remplacent les
  boucles.
- Pour répéter « tant que », une fonction récursive, si possible avec un
  accumulateur.

Suite : [La récursion](recursion.md)
