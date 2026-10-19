# Les boucles

Cette page montre comment répéter une action sans boucle, avec `each`, `each_with_index`, `map` et la récursion.

> **Objectifs :**
>
> - vous saurez pourquoi Rythme n'a ni `for` ni `while`, et quoi écrire à la place ;
> - vous saurez répéter une action sans boucle, avec `each`, `each_with_index` et `map` ;
> - vous saurez écrire une répétition « tant que » avec une fonction récursive.

Ce chapitre suit [les conditions](conditions-boucles.md) : vous savez choisir une valeur avec `if` et `match`, vous allez maintenant apprendre à répéter une action.

## Pas de boucles

Rythme n'a ni `for` ni `while`. Si vous en écrivez une, Rythme vous explique
quoi faire à la place (voir [les erreurs fréquentes](#erreurs-frequentes)).

Au lieu de dire « pour chaque élément, fais ceci », on dit « fais ceci à
toute la liste ». C'est comme dire « lavez toutes les assiettes » au lieu de
« prenez l'assiette 1, lavez-la, prenez l'assiette 2... ». Voici les outils.

## Faire quelque chose pour chaque élément avec each

Imaginez qu'on vous demande d'afficher chaque article d'une liste de courses. Écrire
un `print` par article serait long et pénible. `each` fait ce travail à votre place.

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

**À vous :** écrivez un programme qui affiche « À acheter : … » pour chaque article de
la liste `["pain", "lait", "riz"]`, avec `each`.

> **Solution**
>
> ```rythme
> ["pain", "lait", "riz"] |> each(fn(article) = print("À acheter : {article}"))
> ```
>
> ```sortie
> À acheter : pain
> À acheter : lait
> À acheter : riz
> ```

## Compter de 2 en 2, ou à l'envers

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

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

Vous voulez parfois numéroter ce que vous parcourez, comme les étapes d'une recette
ou une liste de courses numérotée.

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

Ligne par ligne :

- la liste contient trois articles ; `each_with_index` les prend un par un ;
- au premier tour, `article` vaut `"pain"` et `i` vaut 0 ; au deuxième, `"lait"` et
  1 ; au troisième, `"œufs"` et 2 ;
- `print` affiche `i + 1` pour que la liste commence à 1, comme on compte d'habitude.

## Transformer chaque élément avec map

Parfois, vous voulez changer toute une liste d'un coup : doubler chaque prix, mettre
chaque nombre au carré. C'est comme une machine où chaque objet entre d'un côté et
ressort transformé de l'autre.

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

Certaines répétitions n'ont pas de liste toute prête : on répète « tant que » ce n'est
pas fini, comme un compte à rebours qui descend jusqu'à zéro.

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

Ligne par ligne :

- `fn compte_a_rebours(0) = ...` : si le nombre reçu est 0, la fonction affiche
  « Décollage ! » et s'arrête là ;
- `fn compte_a_rebours(n)` : pour tout autre nombre, elle affiche `n`, puis s'appelle
  elle-même avec `n - 1` ;
- `compte_a_rebours(3)` lance tout : 3 est affiché, puis l'appel continue avec 2,
  puis 1, puis 0, qui affiche « Décollage ! ».

Ici, la fonction a deux définitions : une pour `0` (le cas d'arrêt), une
pour les autres nombres. `compte_a_rebours(3)` affiche 3 puis appelle
`compte_a_rebours(2)`, et ainsi de suite jusqu'à 0. Voir
[les fonctions](fonctions.md#definir-par-cas).

**À vous :** écrivez une fonction récursive `applaudir(n)` qui affiche « clap » `n`
fois, puis « Bravo ! ». Appelez `applaudir(3)`.

> **Solution**
>
> ```rythme
> fn applaudir(0) = print("Bravo !")
> fn applaudir(n)
>   print("clap")
>   applaudir(n - 1)
> end
>
> applaudir(3)
> ```
>
> ```sortie
> clap
> clap
> clap
> Bravo !
> ```

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

### Utiliser each au lieu de map

```rythme norun
carres = 1..3 |> each(fn(x) = x * x)
print(carres |> map(fn(x) = x + 1))
```

```text
erreur[E005]: `map` attend une liste, un ensemble, une Option ou un Result comme argument 1, mais a reçu une valeur de type Unit
 --> main.ry:2:7
  |
2 | print(carres |> map(fn(x) = x + 1))
  |       ^^^^^^ mauvaise sorte de valeur
  |
  = aide: utilisation : `map(list, f)`
  = en savoir plus : `rythme explain E005`
```

`each` fait quelque chose pour chaque élément, mais ne fabrique pas de liste : il
donne la valeur vide `()`, de type `Unit`. Pour calculer une nouvelle liste, utilisez
`map` : `carres = 1..3 |> map(fn(x) = x * x)`.

## Testez-vous

**Question 1 :** comment afficher les nombres pairs de 2 à 10, sans boucle ?

> **Réponse**
>
> Par exemple `2..10 |> step_by(2) |> each(fn(n) = print(n))`, ou
> `2..10 |> filter(is_even) |> each(fn(n) = print(n))`.

**Question 2 :** que fait `["a", "b"] |> map(fn(x) = x + "!")`, et en quoi est-ce différent de `each` ?

> **Réponse**
>
> Il fabrique une nouvelle liste, `["a!", "b!"]`. `each` appelle la fonction pour
> chaque élément mais ne donne pas de liste : il sert à **faire** quelque chose
> (afficher), `map` sert à **calculer**.

**Question 3 :** dans le compte à rebours, que se passe-t-il si vous enlevez la ligne
`fn compte_a_rebours(0) = print("Décollage !")` ?

> **Réponse**
>
> La fonction ne s'arrête plus : après 1, elle affiche 0, puis -1, -2… sans fin. Une
> fonction récursive a toujours besoin d'un cas où elle s'arrête.

## En résumé

- Il n'y a ni `for` ni `while` : on travaille sur toute la liste.
- `each` fait quelque chose pour chaque élément, `each_with_index` donne
  aussi sa position, `map` transforme chaque élément et donne une nouvelle liste.
- `step_by` compte par pas, `reverse` compte à l'envers.
- Pour « tant que », on écrit une fonction qui s'appelle elle-même, avec un
  cas où elle s'arrête.

Suite : [les fonctions](fonctions.md).
