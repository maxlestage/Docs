# Blocs, itérateurs et Enumerable

Cette page explique les blocs, les itérateurs comme `map` et `select`, et présente rapidement les procs et les lambdas.

> **Objectifs :**
>
> - vous saurez écrire un bloc avec `{ }` ou `do ... end` ;
> - vous saurez transformer, filtrer et combiner une collection avec `map`, `select` et `reduce` ;
> - vous saurez poser des questions à une collection (`any?`, `count`, `max_by`…) ;
> - vous saurez qu'un bloc voit les variables qui l'entourent ;
> - vous saurez ranger du code dans une variable avec une lambda.

## Les blocs

Un [bloc](../debuter/glossaire.md#bloc) est un morceau de code que vous donnez à une
méthode. La méthode décide quand l'exécuter, et combien de fois. C'est comme donner
une consigne à un serveur : « pour chaque assiette, ajoute du sel ». Le serveur
(la méthode) passe d'assiette en assiette ; vous avez juste écrit la consigne
(le bloc).

```ruby
[1, 2, 3].each { |n| puts n * 2 }
```

```sortie
2
4
6
```

Entre les barres `|n|`, vous nommez la valeur que le bloc reçoit. Ce nom est un
**paramètre du bloc** : il change à chaque passage (chaque
[itération](../debuter/glossaire.md#iteration)). Choisissez un nom parlant : `|fruit|`,
`|note|`…

Une méthode qui parcourt une collection en appelant un bloc s'appelle un
**itérateur**. `each`, `times` et `map` sont des itérateurs.

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
`Enumerable` (« énumérable » : qu'on peut parcourir un par un). Un module est une boîte
de méthodes partagées : voir [les modules](modules.md).

> **À retenir :** ces méthodes **renvoient** un nouveau résultat. Elles ne modifient pas
> la collection de départ.

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

> **Astuce :** `map(&:upcase)` est un raccourci pour `map { |s| s.upcase }`. Il marche
> quand le bloc appelle juste une méthode sans argument sur chaque élément.

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

`find` renvoie le **premier** élément pour lequel le bloc est vrai (ou `nil`).

### reduce : combiner

`reduce` combine tous les éléments en une seule valeur.

```ruby
nombres = [1, 2, 3, 4]
total = nombres.reduce(0) { |somme, n| somme + n }
puts total
```

```sortie
10
```

À chaque tour, `somme` contient le total déjà calculé, et `n` l'élément suivant. Le
`0` est la valeur de départ.

### Poser des questions à une collection

- `any?` : au moins un élément vérifie-t-il le bloc ?
- `all?` : tous les éléments le vérifient-ils ?
- `count` : combien d'éléments le vérifient ?
- `min_by` et `max_by` : quel élément donne la plus petite ou la plus grande valeur ?
- `sum` : la somme, éventuellement de ce que renvoie le bloc.
- `tally` : combien de fois chaque valeur apparaît.

```ruby
notes = [12, 7, 15, 9]
puts notes.any? { |n| n < 8 }
puts notes.all? { |n| n >= 10 }
puts notes.count { |n| n >= 10 }

mots = ["kiwi", "ananas", "pomme"]
puts mots.max_by { |m| m.length }
puts mots.sum { |m| m.length }
p ["a", "b", "a"].tally
```

```sortie
true
false
2
ananas
15
{"a"=>2, "b"=>1}
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
p mots.sort_by { |m| m.length }
p mots.sort_by { |m| m.length }.last
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

## Un bloc voit les variables autour de lui

Une méthode ne voit pas les variables du dehors (voir
[la portée](fonctions.md#la-portee-des-variables)). Un bloc, lui, les voit et peut les
modifier. On dit que le bloc est une [closure](../debuter/glossaire.md#closure)
(« fermeture ») : il emporte avec lui son entourage.

```ruby
total = 0
[5, 10, 20].each { |prix| total += prix }
puts total
```

```sortie
35
```

En revanche, une variable créée **dans** le bloc disparaît à la fin du bloc.

### Écrire une méthode qui prend un bloc

`yield` exécute le bloc donné (voir [les méthodes](fonctions.md#passer-un-bloc-avec-yield)).
`block_given?` répond `true` si un bloc a été donné.

```ruby
def annoncer(texte)
  if block_given?
    puts yield(texte)
  else
    puts texte
  end
end

annoncer("bonjour")
annoncer("bonjour") { |t| t.upcase + " !" }
```

```sortie
bonjour
BONJOUR !
```

## Les procs et les lambdas

Un bloc ne peut pas être rangé dans une variable. Pour cela, Ruby propose les **procs**
et les **lambdas**. Pour débuter, il suffit de savoir les reconnaître.

```ruby
doubler = ->(x) { x * 2 }
puts doubler.call(5)
puts doubler.call(7)
```

```sortie
10
14
```

`call` exécute la lambda avec la valeur donnée.

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

## Erreurs fréquentes

### Oublier de nommer le paramètre du bloc

```ruby norun
[1, 2, 3].each { puts n * 2 }
```

```text
main.rb:1:in `block in <main>': undefined local variable or method `n' for main (NameError)

[1, 2, 3].each { puts n * 2 }
                      ^
	from main.rb:1:in `each'
	from main.rb:1:in `<main>'
```

Le bloc ne sait pas ce qu'est `n`. Déclarez-le entre barres : `{ |n| puts n * 2 }`.

### do ... end après p ou puts

```ruby norun
p [1, 2].map do |x|
  x * 2
end
```

```text
#<Enumerator: [1, 2]:map>
```

Pas d'erreur, mais pas le bon résultat ! Avec `do ... end`, le bloc est donné à `p`, pas
à `map`. `map` sans bloc renvoie un objet « en attente » (un `Enumerator`). Rangez
d'abord le résultat dans une variable, ou utilisez des accolades :

```ruby
p [1, 2].map { |x| x * 2 }
```

```sortie
[2, 4]
```

### Appeler une lambda avec le mauvais nombre d'arguments

```ruby norun
additionner = ->(a, b) { a + b }
puts additionner.call(1)
```

```text
main.rb:1:in `block in <main>': wrong number of arguments (given 1, expected 2) (ArgumentError)
	from main.rb:2:in `<main>'
```

Une lambda vérifie ses arguments, comme une méthode. Donnez-les tous :
`additionner.call(1, 2)`.

## Testez-vous

**Question 1 :** quelle est la différence entre `each` et `map` ?

> **Réponse**
>
> `each` exécute le bloc pour chaque élément et renvoie la collection de départ.
> `map` renvoie un **nouveau** tableau rempli des résultats du bloc.
>
> ```ruby
> p [1, 2].each { |x| x * 10 }
> p [1, 2].map { |x| x * 10 }
> ```
>
> ```sortie
> [1, 2]
> [10, 20]
> ```

**Question 2 :** comment garder seulement les mots de plus de 4 lettres de
`["chat", "cheval", "pie", "girafe"]` ?

> **Réponse**
>
> ```ruby
> p ["chat", "cheval", "pie", "girafe"].select { |m| m.length > 4 }
> ```
>
> ```sortie
> ["cheval", "girafe"]
> ```

**Question 3 :** que vaut `[1, 2, 3].reduce(10) { |acc, n| acc + n }` ?

> **Réponse**
>
> `16` : on part de 10, puis on ajoute 1, 2 et 3.

## En résumé

- Un bloc est du code donné à une méthode, entre `{ }` ou entre `do` et `end`.
- `map` transforme, `select` et `reject` filtrent, `find` cherche le premier.
- `reduce` combine tous les éléments en une seule valeur.
- `sort_by` trie et `group_by` regroupe selon le résultat du bloc.
- Un bloc voit les variables qui l'entourent ; une lambda (`->(x) { ... }`) est un bloc
  rangé dans une variable.

Suite : [Procs, lambdas et méthodes](procs-lambdas.md)
