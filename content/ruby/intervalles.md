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

Ligne par ligne :

- `(1..5).to_a` donne toutes les valeurs de 1 à 5, et 5 en fait partie ;
- `(1...5).to_a` s'arrête juste avant 5 : le troisième point exclut la fin ;
- `class` affiche le type de la valeur : ici, `Range` ;
- `size` compte les valeurs : 5 pour `1..5`, mais 4 pour `1...5`.

`to_a` transforme l'intervalle en tableau. L'intervalle lui-même ne range pas ses
valeurs : il ne connaît que son début et sa fin. `1..1_000_000` ne prend donc presque
pas de place en mémoire.

> **Astuce :** pour retenir la différence, pensez que le troisième point « pousse » la
> fin dehors.

## Parcourir un intervalle

Un intervalle sert souvent à répéter une action un nombre précis de fois : faire trois
tours de piste, écrire une table de multiplication.

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

Ligne par ligne :

- `(1..3).each` donne à `n` la valeur 1, puis 2, puis 3 ;
- à chaque tour, le bloc `{ … }` affiche « Tour » suivi du nombre ;
- `(1...4)` s'arrête avant 4 : `i` vaut 1, puis 2, puis 3 ;
- entre `do` et `end`, le bloc calcule `i * 5` et affiche une ligne de la table.

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

**À vous :** écrivez un programme qui affiche la table de 2, de « 2 × 1 = 2 » à
« 2 × 5 = 10 », avec un intervalle et `each`.

> **Solution**
>
> ```ruby
> (1..5).each do |i|
>   puts "2 × #{i} = #{i * 2}"
> end
> ```
>
> ```sortie
> 2 × 1 = 2
> 2 × 2 = 4
> 2 × 3 = 6
> 2 × 4 = 8
> 2 × 5 = 10
> ```

## Avancer par pas avec step

Parfois, vous ne voulez pas toutes les valeurs, mais une sur deux ou une sur cinq, comme
quand on compte de 5 en 5 jusqu'à 100.

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

Ligne par ligne :

- `(0..20).step(5)` part de 0 et ajoute 5 à chaque fois : 0, 5, 10, 15, 20 ;
- `((0..20) % 5)` donne exactement le même résultat ;
- avec des nombres à virgule, le pas peut être `0.25` ;
- `10.step(1, -3)` part de 10 et retire 3 à chaque tour, sans descendre sous 1.

Pour compter à rebours, un intervalle comme `10..1` est **vide**. Utilisez `step` avec
un pas négatif, `downto`, ou `(1..10).reverse_each`.

**À vous :** écrivez un programme qui affiche, dans un tableau, les multiples de 3 de 0
à 30.

> **Solution**
>
> ```ruby
> p (0..30).step(3).to_a
> ```
>
> ```sortie
> [0, 3, 6, 9, 12, 15, 18, 21, 24, 27, 30]
> ```

## Tester l'appartenance

C'est une question que les programmes se posent sans arrêt : « cet âge donne-t-il droit
au tarif enfant ? », « cette note est-elle entre 10 et 20 ? ». Un intervalle y répond
en une ligne.

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

Ligne par ligne :

- `ages_ado = 13..19` range l'intervalle dans une variable, comme n'importe quelle valeur ;
- 15 est entre 13 et 19 : `true`. 20 est au-delà : `false` ;
- un intervalle peut aussi aller d'un nombre à virgule à un autre : 2.1 est bien dedans ;
- `1...10` exclut 10 : la réponse est donc `false`.

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

**À vous :** écrivez un programme qui range `temperature = 22` dans une variable, puis
affiche « Agréable » si elle est entre 18 et 25 compris, et « Pas agréable » sinon.

> **Solution**
>
> ```ruby
> temperature = 22
> if (18..25).include?(temperature)
>   puts "Agréable"
> else
>   puts "Pas agréable"
> end
> ```
>
> ```sortie
> Agréable
> ```

## Des intervalles de lettres

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

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

Un intervalle de nombres se résume souvent en un seul nombre : le total d'une série, le
plus petit, le plus grand. Ruby les calcule pour vous, sans écrire de boucle.

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

Souvent, vous ne voulez qu'une partie d'une liste : les trois premiers d'un classement,
les derniers messages reçus, le début d'un mot.

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

Ligne par ligne :

- `lettres[1..3]` prend les positions 1, 2 et 3 : `"b"`, `"c"` et `"d"` ;
- `lettres[2..]` n'a pas de fin : il prend tout à partir de la position 2 ;
- `lettres[..1]` n'a pas de début : il prend tout jusqu'à la position 1 comprise ;
- `-2..` part de l'avant-dernier élément et va jusqu'au bout ;
- sur un texte, c'est pareil, lettre par lettre : `mot[0..2]` donne `"bon"`.

**À vous :** écrivez un programme qui range les sept jours de la semaine dans un tableau
`jours`, puis affiche les cinq premiers, puis les deux derniers.

> **Solution**
>
> ```ruby
> jours = ["lun", "mar", "mer", "jeu", "ven", "sam", "dim"]
> p jours[0..4]
> p jours[-2..]
> ```
>
> ```sortie
> ["lun", "mar", "mer", "jeu", "ven"]
> ["sam", "dim"]
> ```

## Sans fin ou sans début

Certaines catégories n'ont pas de limite d'un côté : « 65 ans et plus », « moins de 18
ans ». Un intervalle sans fin ou sans début les décrit simplement.

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

Ligne par ligne :

- `(5..).first(3)` prend les trois premières valeurs à partir de 5, sans chercher la fin ;
- `(..5)` contient tout ce qui est inférieur ou égal à 5, même -100 ;
- dans `categorie`, `...18` veut dire « moins de 18 » et `65..` « 65 ou plus » ;
- 12 tombe dans le premier `when`, 40 dans le deuxième, 80 dans le troisième.

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

Suite : [Suivre un programme pas à pas en Ruby](pas-a-pas.md)
