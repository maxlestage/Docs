# Égalité, comparaison et tri

Cette page explique comment Ruby décide que deux objets sont égaux ou que l'un est plus petit que l'autre, et comment trier une collection selon vos propres règles.

> **Objectifs :**
>
> - vous saurez distinguer `==`, `equal?` et `eql?` ;
> - vous saurez rendre vos objets égaux entre eux, y compris dans un hash ou avec `uniq` ;
> - vous saurez comparer vos objets avec `<=>` et le module `Comparable` ;
> - vous saurez trier avec `sort`, `sort_by`, `min_by` et `max_by`, sur une ou plusieurs clés ;
> - vous saurez ce qu'est un tri stable et comment l'obtenir.

## Trois façons de demander « égal ? »

Ruby a trois questions d'égalité. Elles ne posent pas la même question.

| Méthode | Question posée | Exemple |
|---------|----------------|---------|
| `a == b` | ont-ils la même **valeur** ? | `1 == 1.0` vaut `true` |
| `a.eql?(b)` | même valeur **et** même type ? | `1.eql?(1.0)` vaut `false` |
| `a.equal?(b)` | est-ce **le même objet** en mémoire ? | deux textes `"a"` différents : `false` |

Pensez à deux exemplaires du même livre. Ils ont le même contenu (`==`), la même
édition (`eql?`), mais ce sont deux objets différents sur l'étagère (`equal?` dit non).

```ruby
a = "chat"
b = "chat"
c = a

puts a == b
puts a.equal?(b)
puts a.equal?(c)
puts 1 == 1.0
puts 1.eql?(1.0)
```

```sortie
true
false
true
true
false
```

`c = a` ne copie pas le texte : `c` et `a` désignent le même
[objet](../debuter/glossaire.md#objet). C'est pourquoi `a.equal?(c)` répond `true`.

> **À retenir :** dans la vie de tous les jours, utilisez `==`. Les deux autres servent
> surtout à Ruby lui-même.

## Rendre vos objets égaux

Par défaut, `==` sur vos propres objets se comporte comme `equal?` : deux objets
distincts ne sont jamais égaux, même avec les mêmes données.

```ruby
class Point
  attr_reader :x, :y

  def initialize(x, y)
    @x = x
    @y = y
  end
end

puts Point.new(1, 2) == Point.new(1, 2)
```

```sortie
false
```

Pour changer cela, définissez `==` : deux points sont égaux si leurs coordonnées le
sont.

```ruby
class Point
  attr_reader :x, :y

  def initialize(x, y)
    @x = x
    @y = y
  end

  def ==(autre)
    autre.is_a?(Point) && x == autre.x && y == autre.y
  end
end

puts Point.new(1, 2) == Point.new(1, 2)
puts Point.new(1, 2) == Point.new(3, 4)
puts Point.new(1, 2) == "texte"
```

```sortie
true
false
false
```

Le test `is_a?(Point)` évite une erreur si on compare un point avec autre chose.

### Égaux aussi dans un hash et avec uniq

Un hash et `uniq` n'utilisent pas `==`. Ils rangent les objets dans des cases
numérotées, grâce à la méthode `hash`, puis comparent avec `eql?`. C'est comme une
bibliothèque : on va d'abord au bon rayon (`hash`), puis on compare les livres du
rayon (`eql?`).

Il faut donc définir les deux : `eql?` (le plus simple : le même que `==`) et `hash`
(le numéro de rayon, calculé à partir des mêmes données).

```ruby
class Point
  attr_reader :x, :y

  def initialize(x, y)
    @x = x
    @y = y
  end

  def ==(autre)
    autre.is_a?(Point) && x == autre.x && y == autre.y
  end
  alias eql? ==

  def hash
    [x, y].hash
  end
end

points = [Point.new(1, 2), Point.new(1, 2), Point.new(5, 5)]
puts points.uniq.size

noms = { Point.new(0, 0) => "origine" }
puts noms[Point.new(0, 0)]
```

```sortie
2
origine
```

`[x, y].hash` laisse Ruby calculer un numéro à partir des coordonnées : deux points
égaux reçoivent toujours le même numéro.

> **Astuce :** un [`Struct`](types.md#struct) ou un `Data` écrit déjà `==`, `eql?` et
> `hash` pour vous. `Point = Struct.new(:x, :y)` suffit souvent.

## L'opérateur <=>

`<=>` est surnommé « l'opérateur vaisseau spatial » à cause de sa forme. Il compare
deux valeurs et répond par un nombre :

- `-1` si la gauche est plus petite ;
- `0` si elles sont égales ;
- `1` si la gauche est plus grande ;
- `nil` si on ne peut pas les comparer.

```ruby
p 3 <=> 5
p 5 <=> 5
p "b" <=> "a"
p 3 <=> "trois"
```

```sortie
-1
0
1
nil
```

Les textes se comparent lettre par lettre, comme dans un dictionnaire. Attention :
les majuscules passent avant les minuscules (`"Z" <=> "a"` vaut `-1`).

## Le module Comparable

Si vous écrivez `<=>` dans votre classe et ajoutez `include Comparable`, Ruby vous offre
gratuitement `<`, `<=`, `>`, `>=`, `==`, `between?` et `clamp`. Un seul
[opérateur](../debuter/glossaire.md#operateur) à écrire, sept méthodes reçues : c'est un
[mixin](../debuter/glossaire.md#mixin).

```ruby
class Duree
  include Comparable
  attr_reader :minutes

  def initialize(minutes)
    @minutes = minutes
  end

  def <=>(autre)
    minutes <=> autre.minutes
  end

  def to_s
    "#{minutes} min"
  end
end

court = Duree.new(5)
long = Duree.new(90)

puts court < long
puts long >= court
puts Duree.new(30).between?(court, long)
puts [long, court, Duree.new(30)].sort.join(", ")
puts [long, court].max
```

```sortie
true
true
true
5 min, 30 min, 90 min
90 min
```

`sort`, `min` et `max` utilisent aussi `<=>` : vos durées se trient toutes seules.

## Trier avec sort et sort_by

`sort` trie dans l'ordre naturel (`<=>`). Avec un bloc, vous donnez votre propre règle :
le bloc reçoit deux éléments `a` et `b` et renvoie le résultat d'un `<=>`.

```ruby
mots = ["poire", "kiwi", "banane", "figue"]

p mots.sort
p mots.sort { |a, b| a.length <=> b.length }
```

```sortie
["banane", "figue", "kiwi", "poire"]
["kiwi", "poire", "figue", "banane"]
```

Plus simple : `sort_by` reçoit **un** élément et renvoie la **clé** de tri. Ruby
compare les clés à votre place.

```ruby
mots = ["poire", "kiwi", "banane", "figue"]

p mots.sort_by { |mot| mot.length }
p mots.sort_by(&:length)
```

```sortie
["kiwi", "poire", "figue", "banane"]
["kiwi", "poire", "figue", "banane"]
```

`&:length` est un raccourci pour `{ |mot| mot.length }`.

### Le plus petit et le plus grand

`min_by` et `max_by` donnent l'élément dont la clé est la plus petite ou la plus
grande, sans trier toute la liste.

```ruby
eleves = [
  { nom: "Léa", note: 14 },
  { nom: "Tom", note: 9 },
  { nom: "Zoé", note: 17 }
]

puts eleves.max_by { |e| e[:note] }[:nom]
puts eleves.min_by { |e| e[:note] }[:nom]
p eleves.max_by(2) { |e| e[:note] }.map { |e| e[:nom] }
```

```sortie
Zoé
Tom
["Zoé", "Léa"]
```

## L'ordre décroissant

Trois façons d'obtenir le plus grand en premier :

- `sort` puis `reverse` ;
- un bloc qui inverse `a` et `b` : `{ |a, b| b <=> a }` ;
- avec `sort_by`, une clé négative quand c'est un nombre : `-note`.

```ruby
notes = [12, 18, 7, 15]

p notes.sort.reverse
p notes.sort { |a, b| b <=> a }
p notes.sort_by { |n| -n }
```

```sortie
[18, 15, 12, 7]
[18, 15, 12, 7]
[18, 15, 12, 7]
```

## Trier sur plusieurs clés

Les tableaux se comparent case par case : la première case d'abord, puis la deuxième
si la première est égale. C'est exactement l'ordre d'un annuaire : par nom, puis par
prénom.

Il suffit donc que `sort_by` renvoie un tableau de clés.

```ruby
personnes = [
  { nom: "Martin", prenom: "Zoé", age: 30 },
  { nom: "Durand", prenom: "Paul", age: 25 },
  { nom: "Martin", prenom: "Anne", age: 41 }
]

personnes.sort_by { |p| [p[:nom], p[:prenom]] }.each do |p|
  puts "#{p[:nom]} #{p[:prenom]}"
end
```

```sortie
Durand Paul
Martin Anne
Martin Zoé
```

Pour mélanger croissant et décroissant, rendez négative la clé numérique : ici, par
nom croissant puis par âge décroissant.

```ruby
personnes = [
  { nom: "Martin", age: 30 },
  { nom: "Durand", age: 25 },
  { nom: "Martin", age: 41 }
]

p personnes.sort_by { |p| [p[:nom], -p[:age]] }.map { |p| "#{p[:nom]} #{p[:age]}" }
```

```sortie
["Durand 25", "Martin 41", "Martin 30"]
```

## Le tri stable

Un tri est **stable** quand deux éléments égaux gardent leur ordre de départ. Imaginez
une file d'attente rangée par taille : deux personnes de même taille restent dans
l'ordre où elles sont arrivées.

Ruby **ne promet pas** que `sort` et `sort_by` soient stables. Le plus souvent cela ne
se voit pas, mais vous ne pouvez pas compter dessus. Pour garantir l'ordre de départ,
ajoutez la position de l'élément comme dernière clé, avec `with_index`.

```ruby
commandes = [
  ["Café", 2], ["Thé", 1], ["Jus", 2], ["Eau", 1]
]

triees = commandes.sort_by.with_index { |(nom, priorite), i| [priorite, i] }
triees.each { |nom, priorite| puts "#{priorite} #{nom}" }
```

```sortie
1 Thé
1 Eau
2 Café
2 Jus
```

À priorité égale, `Thé` reste avant `Eau` et `Café` avant `Jus`, comme au départ.

## Erreurs fréquentes

### Trier des valeurs de types différents

```ruby norun
p [3, "a", 1].sort
```

```text
main.rb:1:in `sort': comparison of String with 1 failed (ArgumentError)
	from main.rb:1:in `<main>'
```

Ruby ne sait pas si `"a"` est plus petit que `1`. Triez des valeurs du même type, ou
donnez une clé commune : `[3, "a", 1].sort_by(&:to_s)`.

### Une clé nil dans sort_by

```ruby norun
eleves = [{ nom: "Léa", note: 12 }, { nom: "Tom" }]
p eleves.sort_by { |e| e[:note] }
```

```text
main.rb:2:in `sort_by': comparison of Integer with nil failed (ArgumentError)
	from main.rb:2:in `<main>'
```

Tom n'a pas de note : sa clé vaut `nil`. Donnez une valeur par défaut :
`e.fetch(:note, 0)`.

### Inclure Comparable sans écrire <=>

```ruby norun
class Boite
  include Comparable
  attr_reader :taille
  def initialize(t) = @taille = t
end
puts Boite.new(1) < Boite.new(2)
```

```text
main.rb:6:in `<': comparison of Boite with Boite failed (ArgumentError)

puts Boite.new(1) < Boite.new(2)
                    ^^^^^^^^^^^^
	from main.rb:6:in `<main>'
```

`Comparable` a besoin de votre `<=>` pour fonctionner. Ajoutez
`def <=>(autre) = taille <=> autre.taille`.

### Définir == mais oublier hash

```ruby norun
class Point
  attr_reader :x
  def initialize(x) = @x = x
  def ==(autre) = x == autre.x
end
p [Point.new(1), Point.new(1)].uniq.size
```

```text
2
```

Pas de message d'erreur, mais un résultat faux : `uniq` utilise `hash` et `eql?`, pas
`==`. Ajoutez `alias eql? ==` et `def hash = x.hash`.

## Testez-vous

**Question 1 :** que valent `"a" == "a"` et `"a".equal?("a")` ?

> **Réponse**
>
> `true` puis `false` : les deux textes ont la même valeur, mais ce sont deux objets
> différents.

**Question 2 :** que renvoie `7 <=> 3` ?

> **Réponse**
>
> `1`, car la valeur de gauche est plus grande.

**Question 3 :** triez ces mots par longueur décroissante, puis par ordre alphabétique
à longueur égale : `["chat", "ours", "lion", "zèbre", "âne"]`.

> **Réponse**
>
> ```ruby
> mots = ["chat", "ours", "lion", "zèbre", "âne"]
> p mots.sort_by { |m| [-m.length, m] }
> ```
>
> ```sortie
> ["zèbre", "chat", "lion", "ours", "âne"]
> ```

## En résumé

- `==` compare les valeurs, `eql?` ajoute le type, `equal?` vérifie que c'est le même objet.
- Pour vos classes, définissez `==` ; pour un hash ou `uniq`, ajoutez `eql?` et `hash`.
- `<=>` renvoie `-1`, `0`, `1` ou `nil` ; avec `include Comparable`, il donne `<`, `>`, `between?`…
- `sort_by` trie selon une clé ; un tableau de clés trie sur plusieurs critères.
- Une clé négative ou `reverse` donne l'ordre décroissant.
- Ruby ne garantit pas un tri stable : ajoutez l'index avec `sort_by.with_index`.

Suite : [le filtrage par motif](pattern-matching.md)
