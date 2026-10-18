# Blocs, itérateurs et Enumerable

Cette page explique les blocs, les itérateurs comme `map` et `select`, et présente rapidement les procs et les lambdas.

## Les blocs

Un bloc est un morceau de code que vous donnez à une méthode. La méthode décide quand
l'exécuter.

```ruby
[1, 2, 3].each { |n| puts n * 2 }
```

```sortie
2
4
6
```

Entre les barres `|n|`, vous nommez la valeur que le bloc reçoit.

## do...end ou accolades

Il existe deux façons d'écrire un bloc :

- `{ ... }` pour un bloc court, sur une ligne ;
- `do ... end` pour un bloc de plusieurs lignes.

```ruby
[1, 2].each { |n| puts n }

[1, 2].each do |n|
  carre = n * n
  puts "#{n} au carré : #{carre}"
end
```

```sortie
1
2
1 au carré : 1
2 au carré : 4
```

> **À retenir :** les deux écritures font la même chose. Le choix est une question de
> lisibilité.

## Les méthodes de Enumerable

Les tableaux, les hashs et les intervalles partagent les mêmes méthodes, venues du module
`Enumerable`.

### map : transformer

`map` crée un nouveau tableau avec le résultat du bloc pour chaque élément.

```ruby
prix = [10, 20, 30]
p prix.map { |x| x * 2 }
p ["a", "b"].map(&:upcase)
```

```sortie
[20, 40, 60]
["A", "B"]
```

> **Astuce :** `map(&:upcase)` est un raccourci pour `map { |s| s.upcase }`.

### select et reject : filtrer

`select` garde les éléments pour lesquels le bloc est vrai. `reject` les enlève.

```ruby
nombres = (1..10).to_a
p nombres.select(&:even?)
p nombres.reject { |n| n > 3 }
p nombres.find { |n| n > 4 }
```

```sortie
[2, 4, 6, 8, 10]
[1, 2, 3]
5
```

### reduce : combiner

`reduce` combine tous les éléments en une seule valeur.

```ruby
nombres = [1, 2, 3, 4]
total = nombres.reduce(0) { |somme, n| somme + n }
puts total
puts nombres.reduce(:*)
```

```sortie
10
24
```

### each_with_index

```ruby
%w[lundi mardi].each_with_index do |jour, i|
  puts "#{i} : #{jour}"
end
```

```sortie
0 : lundi
1 : mardi
```

> **Note :** `%w[lundi mardi]` est un raccourci pour `["lundi", "mardi"]`.

### sort_by : trier

`sort_by` trie selon la valeur donnée par le bloc.

```ruby
mots = ["kiwi", "ananas", "framboise", "pomme"]
p mots.sort_by(&:length)
p mots.sort_by { |m| -m.length }.first
```

```sortie
["kiwi", "pomme", "ananas", "framboise"]
"framboise"
```

### group_by : regrouper

`group_by` range les éléments dans un hash, selon la valeur du bloc.

```ruby
mots = ["ane", "chat", "cheval", "abeille"]
p mots.group_by { |m| m[0] }
```

```sortie
{"a"=>["ane", "abeille"], "c"=>["chat", "cheval"]}
```

### Enchaîner les méthodes

Chaque méthode renvoie une valeur : vous pouvez les enchaîner.

```ruby
resultat = (1..10)
  .select(&:odd?)
  .map { |n| n * n }
  .sum
puts resultat
```

```sortie
165
```

## Les procs et les lambdas

Un bloc ne peut pas être rangé dans une variable. Pour cela, Ruby propose les **procs**
et les **lambdas**.

```ruby
doubler = ->(x) { x * 2 }
puts doubler.call(5)
puts doubler.(7)
p [1, 2, 3].map(&doubler)
```

```sortie
10
14
[2, 4, 6]
```

Une proc s'écrit avec `proc { ... }` :

```ruby
dire = proc { |nom| puts "Salut #{nom}" }
dire.call("Alice")
```

```sortie
Salut Alice
```

> **Attention :** une lambda vérifie le nombre d'arguments, une proc non. En cas de doute,
> préférez la lambda.

## Et ensuite ?

Créez vos propres objets avec [les classes](types.md). Pour le style fonctionnel, voyez
aussi [la page fonctionnelle de Rythme](../rythme/fonctionnel.md).
