# Classes et objets

Cette page montre comment créer vos propres objets avec les classes, l'héritage, les modules et `Struct`.

## Définir une classe

Une classe est un modèle pour fabriquer des objets. Son nom commence par une majuscule.

```ruby
class Chien
  def aboyer
    puts "Wouf !"
  end
end

rex = Chien.new
rex.aboyer
```

```sortie
Wouf !
```

`Chien.new` crée un nouvel objet : une **instance** de la classe.

## initialize et les variables d'instance

`initialize` est appelée par `new`. Les variables qui commencent par `@` appartiennent à
l'objet.

```ruby
class Chien
  def initialize(nom)
    @nom = nom
  end

  def presenter
    puts "Je m'appelle #{@nom}."
  end
end

Chien.new("Rex").presenter
Chien.new("Pluto").presenter
```

```sortie
Je m'appelle Rex.
Je m'appelle Pluto.
```

## attr_accessor

Pour lire ou modifier `@nom` depuis l'extérieur, utilisez `attr_reader`, `attr_writer`
ou `attr_accessor`.

```ruby
class Personne
  attr_accessor :nom
  attr_reader :age

  def initialize(nom, age)
    @nom = nom
    @age = age
  end
end

alice = Personne.new("Alice", 30)
alice.nom = "Alice Martin"
puts alice.nom
puts alice.age
```

```sortie
Alice Martin
30
```

> **À retenir :** `attr_reader` permet de lire, `attr_writer` d'écrire, `attr_accessor`
> les deux.

## Les méthodes et to_s

Une classe peut avoir autant de méthodes que vous voulez. `to_s` décide comment l'objet
s'affiche avec `puts`.

```ruby
class Point
  def initialize(x, y)
    @x = x
    @y = y
  end

  def distance_origine
    Math.sqrt(@x**2 + @y**2)
  end

  def to_s
    "(#{@x}, #{@y})"
  end
end

pt = Point.new(3, 4)
puts pt
puts pt.distance_origine
```

```sortie
(3, 4)
5.0
```

## L'héritage

Une classe peut hériter d'une autre avec `<`. Elle reçoit toutes ses méthodes.

```ruby
class Animal
  def initialize(nom)
    @nom = nom
  end

  def parler
    "#{@nom} fait un bruit."
  end
end

class Chat < Animal
  def parler
    "#{@nom} miaule."
  end
end

puts Animal.new("Truc").parler
puts Chat.new("Félix").parler
```

```sortie
Truc fait un bruit.
Félix miaule.
```

> **Astuce :** dans une méthode redéfinie, `super` appelle la version de la classe
> parente.

## Les modules et les mixins

Un module regroupe des méthodes. Avec `include`, une classe les reçoit : c'est un
**mixin**.

```ruby
module Volant
  def voler
    "#{nom} s'envole !"
  end
end

class Oiseau
  include Volant
  attr_reader :nom

  def initialize(nom)
    @nom = nom
  end
end

puts Oiseau.new("Titi").voler
```

```sortie
Titi s'envole !
```

> **Note :** une classe n'a qu'un seul parent, mais elle peut inclure plusieurs modules.

## Struct

`Struct` crée vite une petite classe avec des attributs.

```ruby
Livre = Struct.new(:titre, :auteur)

livre = Livre.new("Les Misérables", "Victor Hugo")
puts livre.titre
livre.auteur = "V. Hugo"
p livre
```

```sortie
Les Misérables
#<struct Livre titre="Les Misérables", auteur="V. Hugo">
```

## Et ensuite ?

Apprenez à [gérer les erreurs](erreurs.md). Comparez avec
[les types en Rust](../rust/types.md).
