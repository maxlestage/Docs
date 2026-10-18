# Les méthodes

Cette page explique comment écrire des méthodes en Ruby avec `def`, leurs paramètres et leur valeur de retour.

## Définir une méthode

Une méthode est un morceau de code nommé, que vous pouvez réutiliser. Elle commence par
`def` et finit par `end`.

```ruby
def saluer
  puts "Bonjour !"
end

saluer
saluer
```

```sortie
Bonjour !
Bonjour !
```

## Les paramètres

Les paramètres sont les valeurs que la méthode reçoit.

```ruby
def saluer(prenom)
  puts "Bonjour, #{prenom} !"
end

saluer("Alice")
saluer "Bob"
```

```sortie
Bonjour, Alice !
Bonjour, Bob !
```

> **Note :** les parenthèses sont facultatives à l'appel. Gardez-les quand il y a
> plusieurs arguments : c'est plus lisible.

## Le retour implicite

Une méthode renvoie automatiquement la valeur de sa **dernière expression**. Le mot
`return` est inutile dans ce cas.

```ruby
def carre(x)
  x * x
end

puts carre(4)
```

```sortie
16
```

`return` sert surtout à sortir plus tôt de la méthode :

```ruby
def diviser(a, b)
  return "division impossible" if b.zero?
  a / b
end

puts diviser(10, 2)
puts diviser(10, 0)
```

```sortie
5
division impossible
```

## Les valeurs par défaut

Un paramètre peut avoir une valeur par défaut. Elle sert si l'argument est absent.

```ruby
def saluer(prenom, salutation = "Bonjour")
  "#{salutation}, #{prenom} !"
end

puts saluer("Alice")
puts saluer("Bob", "Salut")
```

```sortie
Bonjour, Alice !
Salut, Bob !
```

## Les arguments nommés

Avec `nom:`, un argument se donne par son nom. L'ordre n'a plus d'importance.

```ruby
def commande(produit:, quantite: 1)
  "#{quantite} x #{produit}"
end

puts commande(produit: "café")
puts commande(quantite: 3, produit: "thé")
```

```sortie
1 x café
3 x thé
```

> **Astuce :** les arguments nommés rendent les appels très clairs, surtout avec des
> booléens : `envoyer(urgent: true)` se comprend tout de suite.

## Les noms en ? et en !

Par convention :

- une méthode qui finit par `?` répond par vrai ou faux ;
- une méthode qui finit par `!` modifie l'objet ou est « dangereuse ».

```ruby
def majeur?(age)
  age >= 18
end

puts majeur?(20)

nom = "alice"
nom.capitalize!
puts nom
```

```sortie
true
Alice
```

> **Attention :** `capitalize` renvoie une nouvelle chaîne. `capitalize!` modifie la
> chaîne elle-même.

## Passer un bloc avec yield

Une méthode peut recevoir un **bloc** de code. `yield` exécute ce bloc.

```ruby
def deux_fois
  yield
  yield
end

deux_fois { puts "Hip hip hourra !" }
```

```sortie
Hip hip hourra !
Hip hip hourra !
```

`yield` peut aussi passer une valeur au bloc :

```ruby
def avec_bonus(score)
  yield(score + 10)
end

avec_bonus(5) { |total| puts "Total : #{total}" }
```

```sortie
Total : 15
```

Les blocs sont expliqués en détail dans [la page sur les blocs](blocs.md#les-blocs).

## En résumé

- Une méthode commence par `def` et finit par `end`.
- Elle renvoie la valeur de sa dernière ligne ; `return` sert à sortir plus tôt.
- Un paramètre peut avoir une valeur par défaut, ou être nommé (`produit:`).
- Un nom en `?` répond vrai ou faux ; un nom en `!` modifie l'objet.
- `yield` exécute le bloc donné à la méthode.

Suite : apprenez à [travailler avec du texte](texte.md). Comparez avec
[les fonctions en Rust](../rust/fonctions.md).
