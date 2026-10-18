# Classes et objets

Cette page montre comment créer vos propres objets avec les classes, l'héritage, les modules et `Struct`.

> **Objectifs :**
>
> - vous saurez écrire une classe et créer des objets avec `new` ;
> - vous saurez donner des données à un objet avec `initialize` et les variables `@` ;
> - vous saurez rendre ces données lisibles avec `attr_reader` et `attr_accessor` ;
> - vous saurez cacher une méthode avec `private` ;
> - vous saurez réutiliser une classe avec l'héritage et `super`.

## Définir une classe

Une [classe](../debuter/glossaire.md#classe) est un modèle pour fabriquer des objets.
Pensez à un moule à gâteaux : le moule est la classe, chaque gâteau est un
[objet](../debuter/glossaire.md#objet). Tous les gâteaux ont la même forme, mais chacun
peut avoir son parfum. Le nom d'une classe commence par une majuscule.

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

`Chien.new` crée un nouvel objet : une **instance** de la classe. `rex.aboyer` appelle
la méthode `aboyer` sur cet objet.

Vous utilisez des classes depuis le début : `"bonjour"` est une instance de `String`,
`42` une instance de `Integer`. `class` et `is_a?` le montrent.

```ruby
class Chien
end

rex = Chien.new
puts rex.class
puts rex.is_a?(Chien)
puts 42.is_a?(Integer)
puts "bonjour".class
```

```sortie
Chien
true
true
String
```

## initialize et les variables d'instance

`initialize` est appelée par `new`. Elle prépare l'objet, comme on remplit une fiche
à l'inscription. Les arguments de `new` sont passés à `initialize`.

Les variables qui commencent par `@` sont des **variables d'instance** : elles
appartiennent à l'objet. Chaque objet a les siennes, et elles vivent aussi longtemps
que lui. Toutes les méthodes de l'objet peuvent les lire.

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

Les variables `@` sont cachées : on ne peut pas les lire depuis l'extérieur de l'objet.
Pour lire ou modifier `@nom` depuis l'extérieur, utilisez `attr_reader`, `attr_writer`
ou `attr_accessor`. Ils écrivent pour vous les petites méthodes `nom` et `nom=`.

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

## self et les méthodes de classe

Dans une méthode, `self` désigne l'objet lui-même. Devant le nom d'une méthode
(`def self.nom`), il crée une **méthode de classe** : on l'appelle sur la classe, pas
sur un objet.

```ruby
class Temperature
  attr_reader :degres

  def initialize(degres)
    @degres = degres
  end

  def self.depuis_fahrenheit(f)
    new(((f - 32) * 5 / 9.0).round(1))
  end

  def chaude?
    self.degres > 25
  end
end

t = Temperature.depuis_fahrenheit(86)
puts t.degres
puts t.chaude?
```

```sortie
30.0
true
```

## Les méthodes privées

Après le mot `private`, les méthodes ne sont utilisables que **dans** la classe. Elles
servent d'outils internes, comme la cuisine d'un restaurant : le client ne la voit
pas.

```ruby
class Compte
  def initialize(solde)
    @solde = solde
  end

  def afficher
    "Solde : #{format_euros(@solde)}"
  end

  private

  def format_euros(montant)
    format("%.2f €", montant)
  end
end

compte = Compte.new(42)
puts compte.afficher
puts compte.respond_to?(:format_euros)
```

```sortie
Solde : 42.00 €
false
```

`respond_to?` demande si l'objet accepte une méthode depuis l'extérieur.

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

`Chat` est la classe **enfant**, `Animal` la classe **parente**. `Chat` redéfinit
`parler` : sa version remplace celle du parent.

### Appeler le parent avec super

Dans une méthode redéfinie, `super` appelle la version de la classe parente. C'est
pratique pour compléter au lieu de tout réécrire.

```ruby
class Animal
  def initialize(nom)
    @nom = nom
  end
end

class Chien < Animal
  def initialize(nom, race)
    super(nom)
    @race = race
  end

  def to_s
    "#{@nom} (#{@race})"
  end
end

puts Chien.new("Rex", "berger")
puts Chien.new("Rex", "berger").is_a?(Animal)
```

```sortie
Rex (berger)
true
```

Un `Chien` **est un** `Animal` : `is_a?(Animal)` répond `true`.

## Les modules et les mixins

Un [module](../debuter/glossaire.md#module) regroupe des méthodes. Avec `include`, une
classe les reçoit : c'est un **mixin**. Le chapitre [les modules](modules.md) les
présente en détail.

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

`Struct` crée vite une petite classe avec des attributs : `new`, les accesseurs et un
affichage lisible sont écrits pour vous.

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

## Erreurs fréquentes

### Oublier un argument de new

```ruby norun
class Chien
  def initialize(nom)
    @nom = nom
  end
end
rex = Chien.new
```

```text
main.rb:2:in `initialize': wrong number of arguments (given 0, expected 1) (ArgumentError)
	from main.rb:6:in `new'
	from main.rb:6:in `<main>'
```

`new` passe ses arguments à `initialize`, qui attend un nom. Écrivez `Chien.new("Rex")`.

### Lire une variable @ sans accesseur

```ruby norun
class Personne
  def initialize(nom)
    @nom = nom
  end
end
alice = Personne.new("Alice")
puts alice.nom
```

```text
main.rb:7:in `<main>': undefined method `nom' for an instance of Personne (NoMethodError)

puts alice.nom
          ^^^^
```

`@nom` est caché dans l'objet. Ajoutez `attr_reader :nom` dans la classe.

### Un nom de classe en minuscules

```ruby norun
class chien
end
```

```text
main.rb: --> main.rb
expected a constant name after `class`
> 1  class chien
> 2  end
main.rb:1: class/module name must be CONSTANT (SyntaxError)
class chien
      ^~~~~
```

Un nom de classe est une constante : il commence par une majuscule. Écrivez
`class Chien`.

## Testez-vous

**Question 1 :** quelle méthode est appelée par `Livre.new("Dune")` ?

> **Réponse**
>
> `initialize`, avec l'argument `"Dune"`.

**Question 2 :** que faut-il ajouter pour que ce code affiche `Rex` ?

```ruby norun
class Chien
  def initialize(nom)
    @nom = nom
  end
end
puts Chien.new("Rex").nom
```

> **Réponse**
>
> Un accesseur de lecture dans la classe :
>
> ```ruby
> class Chien
>   attr_reader :nom
>
>   def initialize(nom)
>     @nom = nom
>   end
> end
> puts Chien.new("Rex").nom
> ```
>
> ```sortie
> Rex
> ```

**Question 3 :** dans `class Chat < Animal`, qui est le parent ?

> **Réponse**
>
> `Animal`. `Chat` reçoit toutes ses méthodes et peut les redéfinir.

## En résumé

- Une classe est un modèle ; `new` crée un objet et appelle `initialize`.
- Les variables en `@` appartiennent à l'objet.
- `attr_reader`, `attr_writer` et `attr_accessor` les rendent lisibles ou modifiables.
- `private` cache des méthodes ; `self.` crée une méthode de classe.
- `class Chat < Animal` hérite des méthodes de `Animal` ; `super` appelle le parent.
- Un module partage des méthodes avec `include` ; `Struct` crée vite une petite classe.

Suite : [Héritage et polymorphisme](heritage.md)
