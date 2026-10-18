# Les méthodes

Cette page explique comment écrire des méthodes en Ruby avec `def`, leurs paramètres et leur valeur de retour.

> **Objectifs :**
>
> - vous saurez écrire une méthode avec `def` et l'appeler ;
> - vous saurez donner des paramètres à une méthode, avec ou sans valeur par défaut ;
> - vous saurez renvoyer une valeur, avec ou sans `return` ;
> - vous saurez quelles variables une méthode peut voir ;
> - vous saurez passer un bloc à une méthode avec `yield`.

## Définir une méthode

Une [méthode](../debuter/glossaire.md#methode) est un morceau de code nommé, que vous
pouvez réutiliser. C'est comme une recette : vous l'écrivez une fois, puis vous la
suivez autant de fois que vous voulez. Elle commence par `def` (*define*, « définir »)
et finit par `end`.

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

Écrire `def` ne lance rien : cela enregistre la recette. Écrire `saluer` **appelle** la
méthode, c'est-à-dire l'exécute.

> **Note :** dans d'autres langages, on dit [fonction](../debuter/glossaire.md#fonction).
> En Ruby, tout est un objet, et chaque fonction appartient à un objet : on parle donc
> de méthode.

## Les paramètres

Les [paramètres](../debuter/glossaire.md#parametre) sont les valeurs que la méthode
reçoit. Ce sont des cases vides, remplies au moment de l'appel. Les valeurs données à
l'appel s'appellent les [arguments](../debuter/glossaire.md#argument).

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

Ici, `prenom` est le paramètre ; `"Alice"` et `"Bob"` sont les arguments.

> **Note :** les parenthèses sont facultatives à l'appel. Gardez-les quand il y a
> plusieurs arguments : c'est plus lisible.

Plusieurs paramètres se séparent par des virgules. Les arguments sont donnés dans le
même ordre.

```ruby
def aire(largeur, hauteur)
  largeur * hauteur
end

puts aire(3, 4)
```

```sortie
12
```

## Le retour implicite

Une méthode renvoie automatiquement la valeur de sa **dernière expression**. Le mot
`return` est inutile dans ce cas.

```ruby
def carre(x)
  x * x
end

puts carre(4)
resultat = carre(3) + 1
puts resultat
```

```sortie
16
10
```

La valeur renvoyée remplace l'appel : `carre(3) + 1` devient `9 + 1`.

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

### Afficher ou renvoyer ?

`puts` **affiche** une valeur à l'écran. Renvoyer une valeur la **donne** au code qui a
appelé la méthode, qui peut s'en servir. Une méthode qui finit par `puts` renvoie `nil`.

```ruby
def double_affiche(x)
  puts x * 2
end

def double(x)
  x * 2
end

p double_affiche(5)
p double(5)
```

```sortie
10
nil
10
```

> **À retenir :** préférez les méthodes qui renvoient une valeur. Vous choisirez
> ensuite de l'afficher, de la stocker ou de la réutiliser.

### La méthode sur une ligne

Pour une méthode très courte, écrivez `def nom(paramètres) = expression`.

```ruby
def triple(x) = x * 3

puts triple(7)
```

```sortie
21
```

### Renvoyer plusieurs valeurs

Séparez les valeurs par des virgules. Ruby les renvoie dans un tableau, que vous pouvez
découper en plusieurs variables.

```ruby
def min_max(nombres)
  return nombres.min, nombres.max
end

petit, grand = min_max([4, 9, 1])
puts petit
puts grand
```

```sortie
1
9
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

- `produit:` est **obligatoire** : il n'a pas de valeur par défaut.
- `quantite: 1` est facultatif : il vaut `1` si on ne le donne pas.

> **Astuce :** les arguments nommés rendent les appels très clairs, surtout avec des
> booléens : `envoyer(urgent: true)` se comprend tout de suite.

## Un nombre variable d'arguments

Avec `*`, un paramètre reçoit tous les arguments restants, rangés dans un tableau.

```ruby
def somme(*nombres)
  nombres.sum
end

puts somme(1, 2)
puts somme(1, 2, 3, 4)
puts somme
```

```sortie
3
10
0
```

## La portée des variables

La [portée](../debuter/glossaire.md#portee) d'une variable est la zone du code où elle
existe. Une méthode est une pièce fermée : elle ne voit pas les variables créées
dehors. Elle ne voit que ses paramètres et ses propres variables.

```ruby
def prix_ttc(prix, taux)
  total = prix * (1 + taux)
  total.round(2)
end

taux_tva = 0.2
puts prix_ttc(100, taux_tva)
p defined?(total)
```

```sortie
120.0
nil
```

- Pour donner une valeur à une méthode, passez-la en argument.
- `total` n'existe que dans la méthode : dehors, `defined?(total)` répond `nil`.

> **Astuce :** une **constante** (`TAUX_TVA = 0.2`), elle, est visible partout, même
> dans une méthode.

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

## Une méthode qui s'appelle elle-même

Une méthode peut s'appeler elle-même : c'est la
[récursion](../debuter/glossaire.md#recursion). Il faut un **cas d'arrêt**, sinon elle
ne s'arrête jamais.

```ruby
def compte_a_rebours(n)
  return puts("Partez !") if n.zero?
  puts n
  compte_a_rebours(n - 1)
end

compte_a_rebours(3)
```

```sortie
3
2
1
Partez !
```

Ici, le cas d'arrêt est `n.zero?`. Chaque appel rapproche `n` de zéro.

## Passer un bloc avec yield

Une méthode peut recevoir un **bloc** de code, écrit entre `{ }` après l'appel.
`yield` (« céder la place ») exécute ce bloc.

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

## Erreurs fréquentes

### Oublier un argument

```ruby norun
def aire(l, h)
  l * h
end
puts aire(3)
```

```text
main.rb:1:in `aire': wrong number of arguments (given 1, expected 2) (ArgumentError)
	from main.rb:4:in `<main>'
```

La méthode attend 2 arguments (*expected 2*), elle en reçoit 1 (*given 1*). Donnez-les
tous : `aire(3, 4)`. Ou donnez une valeur par défaut au paramètre.

### Appeler la méthode avant de la définir

```ruby norun
saluer("Alice")
def saluer(nom)
  puts "Bonjour, #{nom} !"
end
```

```text
main.rb:1:in `<main>': undefined method `saluer' for main (NoMethodError)

saluer("Alice")
^^^^^^
Did you mean?  super
```

Ruby lit le fichier de haut en bas. À la ligne 1, `saluer` n'existe pas encore.
Placez le `def` avant l'appel.

### Utiliser une variable du dehors

```ruby norun
taux = 0.2
def prix_ttc(prix)
  prix * (1 + taux)
end
puts prix_ttc(100)
```

```text
main.rb:3:in `prix_ttc': undefined local variable or method `taux' for main (NameError)

  prix * (1 + taux)
              ^^^^
	from main.rb:5:in `<main>'
```

La méthode ne voit pas `taux`. Passez-le en paramètre (`def prix_ttc(prix, taux)`),
ou faites-en une constante (`TAUX = 0.2`).

### Oublier un argument nommé obligatoire

```ruby norun
def commande(produit:, quantite: 1)
  "#{quantite} x #{produit}"
end
puts commande(quantite: 2)
```

```text
main.rb:1:in `commande': missing keyword: :produit (ArgumentError)
	from main.rb:4:in `<main>'
```

Il manque `produit:`. Écrivez `commande(produit: "thé", quantite: 2)`.

## Testez-vous

**Question 1 :** qu'affiche ce code ?

```ruby norun
def f(x)
  x + 1
  x * 2
end
puts f(5)
```

> **Réponse**
>
> `10`. La méthode renvoie sa **dernière** expression, `x * 2`. La ligne `x + 1` est
> calculée, puis oubliée.

**Question 2 :** écrivez une méthode `moyenne(a, b)` qui renvoie la moyenne de deux
entiers, avec une virgule si besoin.

> **Réponse**
>
> ```ruby
> def moyenne(a, b)
>   (a + b) / 2.0
> end
>
> puts moyenne(3, 4)
> ```
>
> ```sortie
> 3.5
> ```
>
> Le `2.0` évite la division entière.

**Question 3 :** quelle est la différence entre un paramètre et un argument ?

> **Réponse**
>
> Le paramètre est le nom écrit dans `def` (`prenom`). L'argument est la valeur donnée
> à l'appel (`"Alice"`).

## En résumé

- Une méthode commence par `def` et finit par `end` ; on l'appelle par son nom.
- Elle renvoie la valeur de sa dernière ligne ; `return` sert à sortir plus tôt.
- Un paramètre peut avoir une valeur par défaut, ou être nommé (`produit:`).
- Une méthode ne voit pas les variables du dehors : passez-les en arguments.
- Un nom en `?` répond vrai ou faux ; un nom en `!` modifie l'objet.
- `yield` exécute le bloc donné à la méthode.

Suite : [travailler avec du texte](texte.md). Comparez avec
[les fonctions en Rust](../rust/fonctions.md).
