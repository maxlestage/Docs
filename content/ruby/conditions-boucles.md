# Conditions et boucles

Cette page montre comment prendre des décisions avec `if`, `unless` et `case`, et comment répéter des actions avec les boucles.

## if, elsif et else

`if` exécute du code seulement si une condition est vraie. Le bloc se termine par `end`.

```ruby
note = 14
if note >= 16
  puts "Très bien"
elsif note >= 10
  puts "Admis"
else
  puts "Refusé"
end
```

```sortie
Admis
```

### La forme courte

Pour une seule instruction, vous pouvez mettre `if` à la fin de la ligne.

```ruby
temperature = 35
puts "Il fait chaud !" if temperature > 30
```

```sortie
Il fait chaud !
```

## unless

`unless` est le contraire de `if` : le code s'exécute si la condition est **fausse**.

```ruby
connecte = false
puts "Veuillez vous connecter." unless connecte
```

```sortie
Veuillez vous connecter.
```

> **Astuce :** évitez `unless` avec `else` : c'est difficile à lire. Utilisez plutôt `if`.

## Combiner des conditions

`&&` veut dire « et », `||` veut dire « ou », `!` veut dire « non ».

```ruby
age = 20
membre = true
puts "Entrée gratuite" if age < 26 && membre
puts "Pas membre" if !membre
```

```sortie
Entrée gratuite
```

## Le ternaire

`condition ? valeur_si_vrai : valeur_si_faux` choisit une valeur en une ligne.

```ruby
age = 15
statut = age >= 18 ? "majeur" : "mineur"
puts statut
```

```sortie
mineur
```

## case et when

`case` compare une valeur à plusieurs possibilités.

```ruby
jour = "samedi"
case jour
when "samedi", "dimanche"
  puts "Week-end !"
when "lundi"
  puts "Courage..."
else
  puts "Jour de semaine"
end
```

```sortie
Week-end !
```

`when` accepte aussi des intervalles :

```ruby
note = 12
mention = case note
          when 0...10 then "insuffisant"
          when 10...14 then "passable"
          else "bien"
          end
puts mention
```

```sortie
passable
```

## Les intervalles

Un intervalle (`Range`) va d'une valeur à une autre. `1..5` inclut 5, `1...5` l'exclut.

```ruby
p (1..5).to_a
p (1...5).to_a
puts (1..10).include?(7)
```

```sortie
[1, 2, 3, 4, 5]
[1, 2, 3, 4]
true
```

## La boucle while

`while` répète tant que la condition est vraie.

```ruby
compteur = 1
while compteur <= 3
  puts "Tour #{compteur}"
  compteur += 1
end
```

```sortie
Tour 1
Tour 2
Tour 3
```

## La boucle until

`until` répète **jusqu'à** ce que la condition devienne vraie.

```ruby
n = 3
until n.zero?
  puts n
  n -= 1
end
puts "Partez !"
```

```sortie
3
2
1
Partez !
```

## times et each

En Ruby, on préfère souvent `times` et `each` aux boucles classiques.

```ruby
3.times { |i| puts "i vaut #{i}" }
(1..3).each { |n| puts n * 10 }
```

```sortie
i vaut 0
i vaut 1
i vaut 2
10
20
30
```

> **À retenir :** `times` compte à partir de `0`. `each` parcourt chaque élément d'une
> collection. Pour en savoir plus, lisez [les blocs](blocs.md).

## break et next

`break` arrête la boucle. `next` passe directement au tour suivant.

```ruby
(1..10).each do |n|
  next if n.even?
  break if n > 7
  puts n
end
```

```sortie
1
3
5
7
```

## En résumé

- `if`, `elsif` et `else` choisissent quoi faire ; `unless` est le contraire de `if`.
- `case` et `when` comparent une valeur à plusieurs possibilités.
- `1..5` inclut 5, `1...5` l'exclut.
- `while` et `until` répètent selon une condition ; `times` et `each` sont plus courants.
- `break` arrête la boucle, `next` passe au tour suivant.

Suite : apprenez à regrouper du code dans [les méthodes](fonctions.md). Comparez avec
[les conditions et boucles en Rythme](../rythme/conditions-boucles.md).
