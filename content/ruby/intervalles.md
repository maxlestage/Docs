# Les intervalles

Cette page montre comment décrire une suite de valeurs, de 1 à 10 ou de « a » à « z », avec un intervalle, et tout ce qu'on peut en faire.

> **Objectifs :**
>
> - vous saurez écrire un intervalle avec `..` et avec `...` ;
> - vous saurez le parcourir, le transformer en tableau et avancer par pas ;
> - vous saurez tester si une valeur est dans un intervalle ;
> - vous saurez découper un tableau ou un texte avec un intervalle ;
> - vous saurez utiliser un intervalle sans début ou sans fin.

## Écrire un intervalle

Un [intervalle](../debuter/glossaire.md#intervalle) (`Range` en Ruby) décrit toutes les
valeurs entre un début et une fin. C'est comme dire « les pages 1 à 10 » au lieu de
les citer une par une.

- `1..10` : de 1 à 10, **10 compris** ;
- `1...10` : de 1 à 10, **10 exclu** (donc de 1 à 9).

```ruby
p (1..5).to_a
p (1...5).to_a
puts (1..5).class
puts (1..5).size
puts (1...5).size
```

```sortie
[1, 2, 3, 4, 5]
[1, 2, 3, 4]
Range
5
4
```

`to_a` transforme l'intervalle en tableau. L'intervalle lui-même ne range pas ses
valeurs : il ne connaît que son début et sa fin. `1..1_000_000` ne prend donc presque
pas de place en mémoire.

> **Astuce :** pour retenir la différence, pensez que le troisième point « pousse » la
> fin dehors.

## Parcourir un intervalle

`each` passe sur chaque valeur, comme pour un tableau.

```ruby
(1..3).each { |n| puts "Tour #{n}" }

(1...4).each do |i|
  puts "#{i} × 5 = #{i * 5}"
end
```

```sortie
Tour 1
Tour 2
Tour 3
1 × 5 = 5
2 × 5 = 10
3 × 5 = 15
```

Un intervalle connaît aussi les méthodes de
[Enumerable](blocs.md#les-methodes-de-enumerable) : `map`, `select`, `sum`…

```ruby
p (1..5).map { |n| n * n }
p (1..10).select(&:even?)
```

```sortie
[1, 4, 9, 16, 25]
[2, 4, 6, 8, 10]
```

## Avancer par pas avec step

`step` saute de plusieurs valeurs à la fois. `%` fait la même chose, en plus court.

```ruby
p (0..20).step(5).to_a
p ((0..20) % 5).to_a
p (0.0..1.0).step(0.25).to_a
10.step(1, -3) { |n| print n, " " }
puts
```

```sortie
[0, 5, 10, 15, 20]
[0, 5, 10, 15, 20]
[0.0, 0.25, 0.5, 0.75, 1.0]
10 7 4 1 
```

Pour compter à rebours, un intervalle comme `10..1` est **vide**. Utilisez `step` avec
un pas négatif, `downto`, ou `(1..10).reverse_each`.

## Tester l'appartenance

`include?` répond à la question : « cette valeur est-elle dans l'intervalle ? »

```ruby
ages_ado = 13..19
puts ages_ado.include?(15)
puts ages_ado.include?(20)
puts (1.5..2.5).include?(2.1)
puts (1...10).include?(10)
```

```sortie
true
false
true
false
```

`cover?` fait presque la même chose : il compare seulement avec le début et la fin. La
différence se voit avec des textes.

```ruby
puts ("a".."z").include?("bb")
puts ("a".."z").cover?("bb")
puts (1..10).cover?(3..5)
```

```sortie
false
true
true
```

`"bb"` n'est pas une des 26 lettres, mais il se range bien entre `"a"` et `"z"` dans
l'ordre alphabétique. `cover?` accepte aussi un intervalle entier.

## Des intervalles de lettres

Un intervalle marche avec tout ce qui a un « suivant » : les lettres aussi.

```ruby
p ("a".."e").to_a
puts ("a".."z").count
p ("A".."C").map { |l| l * 2 }
p ("aa".."ad").to_a
```

```sortie
["a", "b", "c", "d", "e"]
26
["AA", "BB", "CC"]
["aa", "ab", "ac", "ad"]
```

## Somme, minimum et maximum

```ruby
puts (1..100).sum
puts (1..10).min
puts (1..10).max
p (1..10).minmax
puts (1..5).reduce(:*)
```

```sortie
5050
1
10
[1, 10]
120
```

`sum` sur un intervalle d'entiers est instantané : Ruby utilise la formule de Gauss au
lieu d'additionner un par un. `reduce(:*)` multiplie tout : 1 × 2 × 3 × 4 × 5.

## Les intervalles dans case et when

Avec [case et when](conditions-boucles.md#case-et-when), un intervalle range une valeur
dans une catégorie. `when` utilise `cover?` en coulisses.

```ruby
def mention(note)
  case note
  when 16..20 then "Très bien"
  when 14...16 then "Bien"
  when 10...14 then "Passable"
  else "Insuffisant"
  end
end

puts mention(17)
puts mention(14)
puts mention(13.5)
puts mention(4)
```

```sortie
Très bien
Bien
Passable
Insuffisant
```

> **Attention :** avec des notes à virgule, `10..13` laisserait passer `13.5` entre deux
> cases. `10...14` couvre tout jusqu'à 14 exclu.

## Découper avec un intervalle

Entre crochets, un intervalle prend un morceau d'un tableau ou d'un texte. Les positions
commencent à 0, et `-1` est la dernière.

```ruby
lettres = ["a", "b", "c", "d", "e"]
p lettres[1..3]
p lettres[1...3]
p lettres[2..]
p lettres[..1]
p lettres[-2..]

mot = "bonjour"
puts mot[0..2]
puts mot[3..]
puts mot[..-2]
```

```sortie
["b", "c", "d"]
["b", "c"]
["c", "d", "e"]
["a", "b"]
["d", "e"]
bon
jour
bonjou
```

## Sans fin ou sans début

On peut laisser un bout vide :

- `5..` : de 5 jusqu'à l'infini (intervalle **sans fin**) ;
- `..5` : tout ce qui est inférieur ou égal à 5 (intervalle **sans début**).

```ruby
p (5..).first(3)
puts (..5).include?(-100)

def categorie(age)
  case age
  when ...18 then "mineur"
  when 18..64 then "adulte"
  when 65.. then "senior"
  end
end

puts categorie(12)
puts categorie(40)
puts categorie(80)
```

```sortie
[5, 6, 7]
true
mineur
adulte
senior
```

`first(3)` prend les 3 premières valeurs : n'appelez pas `to_a` sur `5..`, il ne
finirait jamais.

## Erreurs fréquentes

### Oublier les parenthèses autour de l'intervalle

```ruby norun
p 1..5.to_a
```

```text
main.rb:1:in `<main>': undefined method `to_a' for an instance of Integer (NoMethodError)

p 1..5.to_a
      ^^^^^
Did you mean?  to_c
               to_r
               to_f
               to_i
               to_s
```

`.to_a` s'applique à `5`, pas à l'intervalle. Entourez-le : `p (1..5).to_a`.

### Un intervalle à l'envers

```ruby norun
p (10..1).to_a
```

```text
[]
```

Pas d'erreur, mais un tableau vide : un intervalle va toujours du plus petit au plus
grand. Pour compter à rebours : `10.downto(1).to_a` ou `(1..10).to_a.reverse`.

### Transformer un intervalle sans fin en tableau

```ruby norun
p (1..).to_a
```

```text
main.rb:1:in `to_a': cannot convert endless range to an array (RangeError)
	from main.rb:1:in `<main>'
```

Un intervalle sans fin a une infinité de valeurs. Prenez-en seulement quelques-unes
avec `first(n)` ou `take(n)`.

## Testez-vous

**Question 1 :** combien de valeurs contient `0...10` ?

> **Réponse**
>
> 10 valeurs : de 0 à 9.
>
> ```ruby
> puts (0...10).size
> ```
>
> ```sortie
> 10
> ```

**Question 2 :** comment obtenir les 3 derniers éléments de `[1, 2, 3, 4, 5, 6]` ?

> **Réponse**
>
> ```ruby
> p [1, 2, 3, 4, 5, 6][-3..]
> ```
>
> ```sortie
> [4, 5, 6]
> ```

**Question 3 :** comment calculer la somme des nombres pairs de 1 à 100 ?

> **Réponse**
>
> ```ruby
> puts (2..100).step(2).sum
> puts (1..100).select(&:even?).sum
> ```
>
> ```sortie
> 2550
> 2550
> ```

## En résumé

- `a..b` inclut la fin, `a...b` l'exclut.
- `each`, `map`, `select`, `sum`, `min`, `max` marchent sur un intervalle.
- `step` avance par pas ; un intervalle à l'envers est vide.
- `include?` et `cover?` testent l'appartenance ; `case`/`when` range une valeur.
- `tableau[1..3]` ou `texte[2..]` découpent un morceau.
- `5..` n'a pas de fin, `..5` pas de début : utilisez `first(n)` pour les lire.

Suite : [Blocs, itérateurs et Enumerable](blocs.md)
