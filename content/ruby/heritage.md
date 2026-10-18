# Héritage et polymorphisme

Cette page montre comment une classe peut hériter d'une autre, comment des objets différents peuvent répondre au même message, et quand préférer la composition à l'héritage.

> **Objectifs :**
>
> - vous saurez créer une sous-classe avec `<` et appeler le parent avec `super` ;
> - vous saurez redéfinir une méthode et lire la chaîne `ancestors` ;
> - vous saurez traiter une liste d'objets différents de la même façon (polymorphisme) ;
> - vous saurez la différence entre `private` et `protected` ;
> - vous saurez quand utiliser la composition plutôt que l'héritage.

## Créer une sous-classe

Le chapitre [classes et objets](types.md#l-heritage) a présenté l'héritage. Rappel : une
[classe](../debuter/glossaire.md#classe) peut **hériter** d'une autre avec `<`. Elle
reçoit toutes ses méthodes, comme un enfant hérite des traits de ses parents.

```ruby
class Vehicule
  def initialize(marque)
    @marque = marque
  end

  def demarrer
    "#{@marque} démarre."
  end
end

class Voiture < Vehicule
end

puts Voiture.new("Renault").demarrer
puts Voiture.superclass
```

```sortie
Renault démarre.
Vehicule
```

`Voiture` n'a rien écrit, mais sait déjà `demarrer`. `superclass` donne le parent
direct.

## Redéfinir une méthode

La sous-classe peut écrire sa propre version d'une méthode du parent. On dit qu'elle la
**redéfinit**. C'est sa version qui est utilisée.

```ruby
class Vehicule
  def bruit
    "..."
  end
end

class Moto < Vehicule
  def bruit
    "Vroum !"
  end
end

class Velo < Vehicule
  def bruit
    "Dring !"
  end
end

puts Vehicule.new.bruit
puts Moto.new.bruit
puts Velo.new.bruit
```

```sortie
...
Vroum !
Dring !
```

## Compléter avec super

`super` appelle la méthode du même nom chez le parent. Vous pouvez ainsi **ajouter**
quelque chose au lieu de tout réécrire.

```ruby
class Employe
  def initialize(nom, salaire)
    @nom = nom
    @salaire = salaire
  end

  def fiche
    "#{@nom} : #{@salaire} €"
  end
end

class Manager < Employe
  def initialize(nom, salaire, equipe)
    super(nom, salaire)
    @equipe = equipe
  end

  def fiche
    super + " (équipe de #{@equipe.size})"
  end
end

puts Employe.new("Léa", 2500).fiche
puts Manager.new("Hugo", 4000, ["Léa", "Tom"]).fiche
```

```sortie
Léa : 2500 €
Hugo : 4000 € (équipe de 2)
```

> **Attention :** `super` tout seul, sans parenthèses, renvoie **les mêmes arguments**
> que ceux reçus. `super()` avec des parenthèses vides n'en renvoie aucun.

## La chaîne des ancêtres

Quand vous appelez une méthode, Ruby la cherche dans la classe de l'objet, puis chez son
parent, puis chez le parent du parent… `ancestors` montre cet ordre de recherche.

```ruby
class Animal
end

class Chien < Animal
end

p Chien.ancestors
puts Chien.new.is_a?(Animal)
puts Chien.new.instance_of?(Animal)
```

```sortie
[Chien, Animal, Object, Kernel, BasicObject]
true
false
```

Toute classe hérite finalement de `Object`. `Kernel` est un
[module](../debuter/glossaire.md#module) inclus dans `Object` : c'est lui qui fournit
`puts`. `is_a?` regarde toute la chaîne, `instance_of?` seulement la classe exacte.

## Le polymorphisme

« Polymorphisme » veut dire « plusieurs formes ». Des objets de classes différentes
répondent au **même message**, chacun à sa manière. Le code qui les utilise n'a pas
besoin de savoir qui est qui.

```ruby
class Forme
  def description
    "#{self.class.name} d'aire #{aire.round(2)}"
  end
end

class Rectangle < Forme
  def initialize(largeur, hauteur)
    @largeur = largeur
    @hauteur = hauteur
  end

  def aire = @largeur * @hauteur
end

class Cercle < Forme
  def initialize(rayon)
    @rayon = rayon
  end

  def aire = Math::PI * @rayon**2
end

formes = [Rectangle.new(3, 4), Cercle.new(1), Rectangle.new(2, 2)]
formes.each { |f| puts f.description }
puts "Total : #{formes.sum(&:aire).round(2)}"
```

```sortie
Rectangle d'aire 12
Cercle d'aire 3.14
Rectangle d'aire 4
Total : 19.14
```

La boucle ne contient aucun `if`. Pour ajouter un triangle, il suffit d'écrire une
nouvelle classe : la boucle ne change pas.

## Les méthodes abstraites

`Forme` ci-dessus utilise `aire`, mais ne la définit pas. Si une sous-classe oublie de
l'écrire, l'erreur est confuse. Une méthode **abstraite** rend la règle claire : elle
existe chez le parent seulement pour lever `NotImplementedError`.

```ruby
class Forme
  def aire
    raise NotImplementedError, "#{self.class} doit définir aire"
  end
end

class Triangle < Forme
end

begin
  Triangle.new.aire
rescue NotImplementedError => e
  puts "Oubli : #{e.message}"
end
```

```sortie
Oubli : Triangle doit définir aire
```

## Le duck typing

En Ruby, l'héritage n'est même pas nécessaire pour le polymorphisme. Ce qui compte, c'est
que l'objet **sache répondre** au message. « Si ça marche comme un canard et que ça
cancane comme un canard, alors c'est un canard » : on appelle cela le *duck typing*.

```ruby
class Canard
  def parler = "Coin !"
end

class Robot
  def parler = "Bip."
end

[Canard.new, Robot.new, "texte"].each do |chose|
  if chose.respond_to?(:parler)
    puts chose.parler
  else
    puts "#{chose.inspect} ne sait pas parler"
  end
end
```

```sortie
Coin !
Bip.
"texte" ne sait pas parler
```

`Canard` et `Robot` n'ont aucun parent commun (à part `Object`). `respond_to?` demande
si l'objet connaît une méthode.

## private et protected

- Une méthode `private` ne peut être appelée que **de l'intérieur** de l'objet, sur
  lui-même.
- Une méthode `protected` peut aussi être appelée sur **un autre objet de la même
  famille** de classes. C'est utile pour comparer deux objets sans tout montrer à
  l'extérieur.

```ruby
class Compte
  def initialize(solde)
    @solde = solde
  end

  def plus_riche_que?(autre)
    solde > autre.solde
  end

  protected

  def solde
    @solde
  end
end

a = Compte.new(500)
b = Compte.new(200)
puts a.plus_riche_que?(b)

begin
  a.solde
rescue NoMethodError => e
  puts "Interdit : #{e.message}"
end
```

```sortie
true
Interdit : protected method `solde' called for an instance of Compte
```

Avec `private`, `autre.solde` aurait été refusé, même à l'intérieur de la classe.

## Préférer la composition

L'héritage dit « **est un** » : un `Chien` est un `Animal`. Si la phrase sonne faux, ne
faites pas hériter. Une `Voiture` n'est pas un `Moteur` : elle **a un** moteur. On range
alors l'objet dans une variable : c'est la **composition**.

```ruby
class Moteur
  def initialize(puissance)
    @puissance = puissance
  end

  def demarrer = "Moteur de #{@puissance} ch allumé"
end

class Voiture
  def initialize(moteur)
    @moteur = moteur
  end

  def demarrer = "#{@moteur.demarrer}, la voiture roule."
end

puts Voiture.new(Moteur.new(90)).demarrer
puts Voiture.new(Moteur.new(150)).demarrer
```

```sortie
Moteur de 90 ch allumé, la voiture roule.
Moteur de 150 ch allumé, la voiture roule.
```

On peut changer de moteur sans changer de classe. Pour partager des méthodes entre des
classes sans lien de parenté, pensez aussi aux [modules](modules.md).

> **À retenir :** utilisez l'héritage pour une vraie relation « est un », sur un ou deux
> niveaux. Pour le reste, la composition et les modules sont plus souples.

## Erreurs fréquentes

### Oublier d'appeler super dans initialize

```ruby norun
class Animal
  def initialize(nom)
    @nom = nom
  end

  def nom = @nom
end

class Chat < Animal
  def initialize(nom, couleur)
    @couleur = couleur
  end
end

p Chat.new("Félix", "roux").nom
```

```text
nil
```

Pas d'erreur, mais `@nom` n'a jamais été rempli : l'`initialize` du parent n'a pas été
appelé. Ajoutez `super(nom)` au début.

### Écrire super sans parenthèses avec d'autres arguments

```ruby norun
class Animal
  def initialize(nom)
    @nom = nom
  end
end

class Chat < Animal
  def initialize(nom, couleur)
    super
    @couleur = couleur
  end
end

Chat.new("Félix", "roux")
```

```text
main.rb:2:in `initialize': wrong number of arguments (given 2, expected 1) (ArgumentError)
	from main.rb:9:in `initialize'
	from main.rb:14:in `new'
	from main.rb:14:in `<main>'
```

`super` seul transmet **tous** les arguments reçus (ici 2). Écrivez `super(nom)`.

### Appeler une méthode privée depuis l'extérieur

```ruby norun
class Coffre
  private

  def code = 1234
end

Coffre.new.code
```

```text
main.rb:7:in `<main>': private method `code' called for an instance of Coffre (NoMethodError)

Coffre.new.code
          ^^^^^
```

Une méthode `private` sert au fonctionnement interne. Si l'extérieur en a besoin,
placez-la avant `private`.

## Testez-vous

**Question 1 :** que renvoie `Integer.ancestors.include?(Comparable)` ?

> **Réponse**
>
> `true` : `Integer` inclut le module `Comparable`, qui fait partie de ses ancêtres.
>
> ```ruby
> puts Integer.ancestors.include?(Comparable)
> ```
>
> ```sortie
> true
> ```

**Question 2 :** dans une méthode redéfinie `parler`, comment réutiliser le texte du
parent et y ajouter « ! » ?

> **Réponse**
>
> Avec `super + " !"` : `super` appelle `parler` du parent et renvoie son résultat.

**Question 3 :** une classe `Bibliotheque` contient des livres. Doit-elle hériter de
`Array` ?

> **Réponse**
>
> Non : une bibliothèque **a des** livres, ce n'est pas un tableau. Rangez un tableau
> dans une variable `@livres` (composition).

## En résumé

- `class Enfant < Parent` hérite de toutes les méthodes du parent.
- Une sous-classe peut redéfinir une méthode ; `super` appelle la version du parent.
- `ancestors` montre l'ordre dans lequel Ruby cherche une méthode.
- Polymorphisme : des objets différents répondent au même message ; le duck typing ne
  regarde que `respond_to?`.
- `private` : seulement soi-même ; `protected` : aussi les objets de la même famille.
- « Est un » → héritage ; « a un » → composition.

Suite : [Modules et mixins](modules.md)
