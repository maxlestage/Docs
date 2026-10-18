# Métaprogrammation en douceur

Cette page montre comment un programme Ruby peut s'examiner et se modifier lui-même : interroger un objet, appeler une méthode par son nom, et créer des méthodes pendant l'exécution.

> **Objectifs :**
>
> - interroger un objet avec `class`, `methods` et `respond_to?` ;
> - appeler une méthode dont le nom est dans une variable avec `public_send` ;
> - créer des méthodes avec `define_method` ;
> - réagir aux méthodes inconnues avec `method_missing` ;
> - savoir quand ne **pas** utiliser ces outils.

## Qu'est-ce que la métaprogrammation ?

Programmer, c'est écrire du code qui manipule des données. **Métaprogrammer**, c'est
écrire du code qui manipule… du code : des classes, des méthodes, des noms de méthodes.

Imaginez un robot cuisinier. D'habitude, vous lui donnez une recette. Avec la
métaprogrammation, le robot peut aussi lire son propre manuel, et y ajouter de nouvelles
recettes tout seul.

Ruby est très doué pour cela. Vous en avez déjà utilisé sans le savoir : `attr_accessor`
**écrit** des méthodes pour vous.

## Tout est un objet

En Ruby, tout est un [objet](../debuter/glossaire.md#objet) : les nombres, les chaînes,
`nil`, et même les classes. Chaque objet sait dire de quelle
[classe](../debuter/glossaire.md#classe) il est.

```ruby
p 42.class
p "bonjour".class
p nil.class
p [1, 2].class
p String.class
```

```sortie
Integer
String
NilClass
Array
Class
```

La dernière ligne surprend : `String` est lui-même un objet, de la classe `Class`. Une
classe est donc une valeur comme une autre, que l'on peut ranger dans une variable.

```ruby
type = Array
liste = type.new(3, 0)
p liste
```

```sortie
[0, 0, 0]
```

## Interroger un objet

Un objet peut répondre à des questions sur lui-même. On parle d'**introspection**.

- `methods` liste ses méthodes (il y en a des centaines) ;
- `respond_to?(:nom)` dit s'il connaît une méthode ;
- `is_a?(Classe)` dit s'il appartient à une classe.

```ruby
mot = "chat"
p mot.respond_to?(:upcase)
p mot.respond_to?(:voler)
p mot.is_a?(String)
p mot.methods.size > 100
p mot.methods.grep(/case/).sort.first(3)
```

```sortie
true
false
true
true
[:casecmp, :casecmp?, :downcase]
```

`respond_to?` est utile pour accepter des objets variés : on vérifie qu'un objet **sait
faire** une chose, plutôt que de vérifier sa classe.

## Appeler une méthode par son nom

`public_send` appelle une méthode dont le nom est une valeur : un
[symbole](../debuter/glossaire.md#symbole) ou une chaîne. Les arguments suivent.

```ruby
p 10.public_send(:+, 5)
p "bonjour".public_send(:upcase)

operation = :max
p [3, 9, 4].public_send(operation)
```

```sortie
15
"BONJOUR"
9
```

C'est pratique quand le nom vient d'une donnée, par exemple d'un menu :

```ruby
transformations = %w[upcase reverse capitalize]

transformations.each do |nom|
  puts "#{nom} : #{"ruby".public_send(nom)}"
end
```

```sortie
upcase : RUBY
reverse : ybur
capitalize : Ruby
```

> **Attention :** `send` fait la même chose mais appelle **aussi** les méthodes privées.
> Il contourne les protections de la classe. Préférez `public_send`.

## Lire et écrire les variables d'instance

`instance_variable_get` et `instance_variable_set` accèdent aux variables d'instance
(celles qui commencent par `@`) depuis l'extérieur. Le nom s'écrit avec le `@`.

```ruby
class Chien
  def initialize(nom)
    @nom = nom
  end
end

rex = Chien.new("Rex")
p rex.instance_variables
p rex.instance_variable_get(:@nom)

rex.instance_variable_set(:@nom, "Médor")
p rex.instance_variable_get(:@nom)
```

```sortie
[:@nom]
"Rex"
"Médor"
```

C'est un passe-partout : utile pour comprendre ou déboguer, mais il ouvre des portes que
la classe voulait fermées.

## Créer des méthodes avec define_method

`define_method` crée une méthode pendant l'exécution. On lui donne un nom et un
[bloc](../debuter/glossaire.md#bloc) : le bloc devient le corps de la méthode, ses
paramètres deviennent ceux de la méthode.

Sans métaprogrammation, trois méthodes presque identiques :

```ruby norun
class Feu
  def rouge? = @couleur == :rouge
  def orange? = @couleur == :orange
  def vert? = @couleur == :vert
end
```

Avec `define_method`, une boucle les écrit pour vous :

```ruby
class Feu
  COULEURS = %i[rouge orange vert]

  COULEURS.each do |couleur|
    define_method("#{couleur}?") do
      @couleur == couleur
    end
  end

  def initialize(couleur)
    @couleur = couleur
  end
end

feu = Feu.new(:orange)
p feu.rouge?
p feu.orange?
p feu.vert?
```

```sortie
false
true
false
```

Ajouter une couleur à `COULEURS` suffit pour créer une méthode de plus.

## attr_accessor refait à la main

`attr_accessor` n'a rien de magique : c'est une méthode de classe qui appelle
`define_method`. Refaisons-la, sous le nom `mon_accesseur`.

```ruby
class Class
  def mon_accesseur(*noms)
    noms.each do |nom|
      define_method(nom) do
        instance_variable_get("@#{nom}")
      end

      define_method("#{nom}=") do |valeur|
        instance_variable_set("@#{nom}", valeur)
      end
    end
  end
end

class Livre
  mon_accesseur :titre, :pages
end

livre = Livre.new
livre.titre = "Dune"
livre.pages = 600
puts "#{livre.titre} (#{livre.pages} pages)"
```

```sortie
Dune (600 pages)
```

Pour chaque nom, deux méthodes sont créées : la lecture (`titre`) et l'écriture
(`titre=`). Les deux outils de la section précédente font le travail.

> **Note :** ici, on ajoute une méthode à **toutes** les classes en rouvrant `Class`.
> C'est parfait pour comprendre, mais à éviter dans un vrai projet.

## method_missing : rattraper les méthodes inconnues

Quand vous appelez une méthode qui n'existe pas, Ruby appelle d'abord `method_missing`.
Par défaut, elle lève une `NoMethodError`. Vous pouvez la redéfinir pour répondre
quand même.

Ici, un objet de réglages répond à n'importe quel nom présent dans son hash :

```ruby
class Reglages
  def initialize(valeurs)
    @valeurs = valeurs
  end

  def method_missing(nom, *args)
    if @valeurs.key?(nom)
      @valeurs[nom]
    else
      super
    end
  end

  def respond_to_missing?(nom, inclure_prive = false)
    @valeurs.key?(nom) || super
  end
end

r = Reglages.new(langue: "fr", volume: 7)
p r.langue
p r.volume
p r.respond_to?(:volume)
p r.respond_to?(:couleur)

begin
  r.couleur
rescue NoMethodError => e
  puts "Erreur : méthode #{e.name} inconnue"
end
```

```sortie
"fr"
7
true
false
Erreur : méthode couleur inconnue
```

Deux règles à toujours suivre :

1. Appelez `super` pour les noms que vous ne gérez pas : l'erreur normale est levée.
2. Définissez `respond_to_missing?` pour que `respond_to?` dise la vérité.

## Les modules comme points d'accroche

[Comparable et Enumerable](modules.md#comparable-comparer-vos-objets) reposent sur la
même idée : le module appelle **une** méthode que vous écrivez, et vous offre toutes les
autres. Votre méthode est un point d'accroche (*hook*).

| Module | Vous écrivez | Vous recevez |
|--------|--------------|--------------|
| `Comparable` | `<=>` | `<`, `>`, `==`, `between?`, `clamp`… |
| `Enumerable` | `each` | `map`, `select`, `sort`, `sum`, `include?`… |

Les modules eux-mêmes ont des accroches. `included` est appelée quand un module est
inclus dans une classe :

```ruby
module Bavard
  def self.included(classe)
    puts "Bavard est inclus dans #{classe}"
  end

  def parler = "Je suis #{self.class}"
end

class Perroquet
  include Bavard
end

puts Perroquet.new.parler
```

```sortie
Bavard est inclus dans Perroquet
Je suis Perroquet
```

Les grandes bibliothèques, comme Rails, utilisent beaucoup ces accroches pour ajouter
des méthodes de classe au moment du `include`.

## Avec modération

La métaprogrammation est un outil puissant, et donc dangereux :

- le code devient **difficile à lire** : on cherche une méthode, mais elle n'est écrite
  nulle part ;
- les **erreurs sont déroutantes** : une faute de frappe peut être avalée par
  `method_missing` ;
- les éditeurs et les outils ne savent plus trouver les méthodes ;
- c'est souvent **plus lent** qu'une méthode normale.

> **À retenir :** écrivez d'abord du code simple et direct. Utilisez la métaprogrammation
> seulement quand elle supprime une vraie répétition, et gardez-la petite et cachée
> dans un seul endroit.

## Erreurs fréquentes

### Un method_missing qui avale tout

```ruby norun
class Robot
  def method_missing(nom, *args)
    "je ne sais pas faire #{nom}"
  end
end

puts Robot.new.dansr
```

```text
je ne sais pas faire dansr
```

Aucune erreur pour la faute de frappe `dansr` : elle passe inaperçue. Ne traitez que les
noms prévus, et appelez `super` pour les autres afin de garder la `NoMethodError`.

### Oublier le @ du nom de variable

```ruby norun
class Point
  def initialize
    @x = 1
  end
end

p Point.new.instance_variable_get(:x)
```

```text
main.rb:7:in `instance_variable_get': `x' is not allowed as an instance variable name (NameError)

p Point.new.instance_variable_get(:x)
           ^^^^^^^^^^^^^^^^^^^^^^
	from main.rb:7:in `<main>'
```

Le nom d'une variable d'instance commence toujours par `@` : écrivez `:@x`.

### Appeler une méthode privée avec public_send

```ruby norun
class Compte
  private

  def secret = 42
end

p Compte.new.public_send(:secret)
```

```text
main.rb:7:in `public_send': private method `secret' called for an instance of Compte (NoMethodError)

p Compte.new.public_send(:secret)
            ^^^^^^^^^^^^
	from main.rb:7:in `<main>'
```

C'est voulu : `public_send` respecte `private`. Si la méthode doit être appelée de
l'extérieur, rendez-la publique ; ne contournez pas la protection avec `send`.

## Testez-vous

**Question 1 :** que renvoie `5.respond_to?(:upcase)` ?

> **Réponse**
>
> `false` : un entier n'a pas de méthode `upcase`, réservée aux chaînes.

**Question 2 :** pourquoi définir `respond_to_missing?` avec `method_missing` ?

> **Réponse**
>
> Sans elle, `respond_to?` répond `false` pour des méthodes qui marchent pourtant grâce à
> `method_missing`. L'objet mentirait sur ce qu'il sait faire.

**Question 3 :** avec `define_method`, créez dans une classe `Calcul` les méthodes
`double`, `triple` et `quadruple`, qui multiplient un nombre par 2, 3 et 4.

> **Réponse**
>
> ```ruby
> class Calcul
>   { double: 2, triple: 3, quadruple: 4 }.each do |nom, facteur|
>     define_method(nom) { |x| x * facteur }
>   end
> end
>
> c = Calcul.new
> p [c.double(5), c.triple(5), c.quadruple(5)]
> ```
>
> ```sortie
> [10, 15, 20]
> ```

## En résumé

- Tout est un objet, même les classes ; `class`, `methods` et `respond_to?` les
  interrogent.
- `public_send` appelle une méthode dont le nom est une valeur ; évitez `send`.
- `define_method` crée des méthodes ; `instance_variable_get` et `_set` accèdent aux `@`.
- `method_missing` rattrape les méthodes inconnues : appelez `super` et définissez
  `respond_to_missing?`.
- Utilisez la métaprogrammation avec modération : le code simple reste le meilleur.

Suite : [organiser un projet](projet.md).
