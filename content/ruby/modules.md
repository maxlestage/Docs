# Modules et mixins

Cette page montre comment ranger votre code dans des modules et partager des méthodes entre plusieurs classes avec `include`.

> **Objectifs :**
>
> - ranger des méthodes et des constantes dans un module ;
> - appeler une fonction de module avec `Module.methode` ;
> - partager des méthodes entre classes avec `include` (un mixin) ;
> - comparer et trier vos objets avec `Comparable` ;
> - donner les méthodes de `Enumerable` à votre propre classe ;
> - répartir le code dans plusieurs fichiers avec `require_relative`.

## Qu'est-ce qu'un module ?

Un **module** est une boîte qui porte un nom. Il range des méthodes et des constantes
qui vont ensemble. Le nom d'un module commence par une majuscule, comme une classe.

```ruby
module Salutations
  def self.bonjour(nom)
    "Bonjour, #{nom} !"
  end
end

puts Salutations.bonjour("Alice")
```

```sortie
Bonjour, Alice !
```

Un module ressemble à une classe, avec une grande différence : **on ne crée pas
d'objet à partir d'un module**. Pas de `Salutations.new`.

> **Note :** le mot « module » existe dans d'autres langages, avec un sens proche
> (voir [le glossaire](../debuter/glossaire.md#module)).

## Un module comme espace de noms

Deux bibliothèques peuvent avoir une méthode ou une classe du même nom. En les rangeant
dans des modules différents, elles ne se gênent pas : chaque module est un **espace de
noms**.

```ruby
module Francais
  def self.saluer
    "Bonjour"
  end
end

module Anglais
  def self.saluer
    "Hello"
  end
end

puts Francais.saluer
puts Anglais.saluer
```

```sortie
Bonjour
Hello
```

Une classe peut aussi vivre dans un module. Pour l'atteindre, écrivez
`Module::Classe` (deux fois deux-points).

```ruby
module Zoo
  class Animal
    def initialize(nom)
      @nom = nom
    end

    def to_s
      "#{@nom} (animal du zoo)"
    end
  end
end

puts Zoo::Animal.new("Rex")
```

```sortie
Rex (animal du zoo)
```

## Les fonctions de module

Une méthode définie avec `def self.nom` appartient au module lui-même. Vous l'appelez
avec `Module.nom`, sans créer d'objet. C'est pratique pour une boîte à outils.

```ruby
module Calcul
  def self.double(x)
    x * 2
  end

  def self.moyenne(nombres)
    nombres.sum.to_f / nombres.size
  end
end

puts Calcul.double(21)
puts Calcul.moyenne([12, 15, 18])
```

```sortie
42
15.0
```

Ruby lui-même range ainsi ses outils de mathématiques dans le module `Math` :

```ruby
puts Math.sqrt(16)
puts Math::PI.round(2)
```

```sortie
4.0
3.14
```

## Les constantes d'un module

Un module range aussi des **constantes** (des valeurs qui ne changent pas). Leur nom
s'écrit en majuscules. Hors du module, écrivez `Module::CONSTANTE`.

```ruby
module Config
  VERSION = "1.2.0"
  LANGUES = ["fr", "en"]
end

puts Config::VERSION
p Config::LANGUES
```

```sortie
1.2.0
["fr", "en"]
```

Dans le module, le nom seul suffit :

```ruby
module Cercle
  PI = 3.14159

  def self.aire(rayon)
    (PI * rayon * rayon).round(2)
  end
end

puts Cercle.aire(2)
```

```sortie
12.57
```

## Partager des méthodes avec include

Voici l'autre grand usage des modules. Écrivez des méthodes **sans** `self.` dans un
module, puis ajoutez `include NomDuModule` dans une classe : chaque objet de la classe
reçoit ces méthodes. On appelle ce module un **mixin** (un « mélange »).

```ruby
module Presentable
  def presenter
    "Je m'appelle #{nom}."
  end
end

class Eleve
  include Presentable
  attr_reader :nom

  def initialize(nom)
    @nom = nom
  end
end

class Chien
  include Presentable
  attr_reader :nom

  def initialize(nom)
    @nom = nom
  end
end

puts Eleve.new("Alice").presenter
puts Chien.new("Rex").presenter
```

```sortie
Je m'appelle Alice.
Je m'appelle Rex.
```

`Eleve` et `Chien` n'ont rien en commun, mais ils partagent la méthode `presenter`.
Dans le module, `nom` appelle la méthode `nom` de l'objet qui a inclus le module.

> **À retenir :** une classe n'a qu'un seul parent (`class Chat < Animal`), mais elle
> peut inclure autant de modules qu'elle veut. Voir aussi
> [l'héritage](types.md#l-heritage).

### Vérifier qu'une classe inclut un module

`include?` répond à la question « cette classe a-t-elle reçu ce module ? ».

```ruby
module Volant
  def voler
    "Je vole !"
  end
end

class Oiseau
  include Volant
end

class Poisson
end

puts Oiseau.include?(Volant)
puts Poisson.include?(Volant)
puts Oiseau.new.voler
```

```sortie
true
false
Je vole !
```

## Comparable : comparer vos objets

Ruby fournit des mixins tout prêts. Le premier est `Comparable`. Il vous donne `<`,
`>`, `<=`, `>=`, `==`, `between?` et `clamp`. En échange, vous écrivez **une seule**
méthode : `<=>`.

### L'opérateur <=>

`a <=> b` compare deux valeurs. Il renvoie `-1` si `a` est plus petit, `0` s'ils sont
égaux, `1` si `a` est plus grand.

```ruby
p 3 <=> 5
p 5 <=> 5
p 7 <=> 5
p "abricot" <=> "banane"
```

```sortie
-1
0
1
-1
```

### Inclure Comparable

Définissez `<=>` en comparant ce qui compte pour vous (ici, la valeur de la note),
puis ajoutez `include Comparable`.

```ruby
class Note
  include Comparable
  attr_reader :valeur

  def initialize(valeur)
    @valeur = valeur
  end

  def <=>(autre)
    valeur <=> autre.valeur
  end

  def to_s
    "#{valeur}/20"
  end
end

a = Note.new(12)
b = Note.new(15)

puts a < b
puts a == Note.new(12)
puts b.between?(Note.new(10), Note.new(16))
puts [b, a, Note.new(9)].sort.join(", ")
puts [b, a].max
```

```sortie
true
true
true
9/20, 12/20, 15/20
15/20
```

> **Astuce :** avec `<=>`, `sort`, `min` et `max` fonctionnent aussi sur un tableau de
> vos objets.

## Enumerable : rendre votre classe parcourable

Le second mixin célèbre est `Enumerable`, celui qui donne `map`, `select`, `sort`…
aux tableaux (voir [les blocs](blocs.md#les-methodes-de-enumerable)). Vous pouvez le
donner à votre propre classe. Il suffit d'écrire **une** méthode : `each`, qui passe
chaque élément au bloc avec `yield`.

```ruby
class Playlist
  include Enumerable

  def initialize
    @titres = []
  end

  def ajouter(titre)
    @titres << titre
    self
  end

  def each
    @titres.each { |titre| yield titre }
  end
end

liste = Playlist.new
liste.ajouter("Imagine").ajouter("Yesterday").ajouter("Hey Jude")

liste.each { |t| puts "- #{t}" }
p liste.map(&:upcase)
p liste.select { |t| t.start_with?("Y") }
p liste.sort
p liste.count
p liste.include?("Imagine")
```

```sortie
- Imagine
- Yesterday
- Hey Jude
["IMAGINE", "YESTERDAY", "HEY JUDE"]
["Yesterday"]
["Hey Jude", "Imagine", "Yesterday"]
3
true
```

Avec une seule méthode `each`, votre `Playlist` reçoit des dizaines de méthodes :
`map`, `select`, `sort`, `count`, `include?`, `first`, `min`, `sum`…

> **Note :** `ajouter` renvoie `self` (l'objet lui-même). C'est ce qui permet
> d'enchaîner les appels : `liste.ajouter(…).ajouter(…)`.

## Un module par fichier avec require_relative

Dans un vrai projet, chaque module va dans son propre fichier. `require_relative`
charge un fichier situé à côté du fichier courant (sans l'extension `.rb`).

```text
mon_projet/
├── main.rb
└── lib/
    └── calcul.rb
```

Le fichier `lib/calcul.rb` contient le module :

```ruby norun
# lib/calcul.rb
module Calcul
  def self.double(x)
    x * 2
  end
end
```

Le fichier `main.rb` le charge, puis l'utilise :

```ruby norun
# main.rb
require_relative "lib/calcul"

puts Calcul.double(21)
```

Dans le terminal, depuis le dossier `mon_projet` :

```sh
ruby main.rb
```

```text
42
```

> **À retenir :** le chemin de `require_relative` part du dossier **du fichier qui
> l'écrit**, pas du dossier où vous lancez la commande. Le chapitre
> [Organiser un projet](projet.md) va plus loin.

## Erreurs fréquentes

### Appeler une méthode sans self. sur le module

Une méthode écrite sans `self.` est faite pour `include`. Le module lui-même ne la
connaît pas.

```ruby norun
module Outils
  def double(x)
    x * 2
  end
end

Outils.double(2)
```

```text
main.rb:7:in `<main>': undefined method `double' for module Outils (NoMethodError)
```

Pour l'appeler avec `Outils.double`, écrivez `def self.double(x)`.

### Oublier le nom du module devant une constante

Hors du module, la constante n'est pas visible avec son nom seul.

```ruby norun
module Geometrie
  PI = 3.14
end

puts PI
```

```text
main.rb:5:in `<main>': uninitialized constant PI (NameError)
```

Écrivez `Geometrie::PI`.

### Inclure Comparable sans écrire <=>

`Comparable` a besoin de `<=>` pour comparer. Sans lui, Ruby ne sait pas faire.

```ruby norun
class Note
  include Comparable
  attr_reader :valeur

  def initialize(valeur)
    @valeur = valeur
  end
end

p Note.new(12) < Note.new(15)
```

```text
main.rb:10:in `<': comparison of Note with Note failed (ArgumentError)
```

Ajoutez `def <=>(autre) = valeur <=> autre.valeur`. De même, inclure `Enumerable` sans
écrire `each` donne une `NoMethodError` (« undefined method each »).

### Créer un objet à partir d'un module

Un module ne fabrique pas d'objets.

```ruby norun
module Outils
end

Outils.new
```

```text
main.rb:4:in `<main>': undefined method `new' for module Outils (NoMethodError)
```

Si vous avez besoin d'objets, écrivez une `class`.

## Testez-vous

**1.** Quelle est la différence entre `def self.aire` et `def aire` dans un module ?

> **Réponse**
>
> `def self.aire` crée une fonction du module : on l'appelle avec `Module.aire`.
> `def aire` crée une méthode à partager : une classe la reçoit avec `include`.

**2.** Vous voulez trier un tableau de vos objets `Joueur` par score. Que faut-il écrire ?

> **Réponse**
>
> Ajoutez `include Comparable` et une méthode `<=>` qui compare les scores :
> `def <=>(autre) = score <=> autre.score`. Ensuite, `joueurs.sort` fonctionne.

**3.** Quelle méthode faut-il écrire pour qu'une classe qui inclut `Enumerable` ait `map`
et `select` ?

> **Réponse**
>
> La méthode `each`, qui passe chaque élément au bloc avec `yield`.

## En résumé

- Un module range des méthodes et des constantes ; on n'en crée pas d'objet.
- `def self.nom` crée une fonction de module, appelée avec `Module.nom` ; une constante
  s'écrit `Module::CONSTANTE`.
- `include Module` donne les méthodes du module à une classe : c'est un mixin.
- `Comparable` demande `<=>` ; `Enumerable` demande `each`.
- `require_relative "lib/fichier"` charge un fichier de votre projet.

Suite : [Struct, Data et Set](structures.md)
