# Le filtrage par motif (case/in)

Cette page montre comment `case`/`in` vérifie la forme d'une donnée et en extrait les morceaux en une seule étape.

> **Objectifs :**
>
> - vous saurez filtrer des valeurs, des tableaux et des hashs avec `case`/`in` ;
> - vous saurez ajouter une condition avec `if` et fixer une valeur avec `^` ;
> - vous saurez rendre vos propres classes filtrables avec `deconstruct` et `deconstruct_keys` ;
> - vous saurez utiliser `in` comme question et `=>` pour extraire des valeurs ;
> - vous saurez éviter l'erreur `NoMatchingPatternError`.

## L'idée : un gabarit

Un **motif** est un gabarit, comme une forme découpée dans du carton. Vous posez la
donnée dessous : si elle a la bonne forme, elle passe, et Ruby range ses morceaux dans
des variables. Ce mécanisme s'appelle le **filtrage par motif** (voir
[match](../debuter/glossaire.md#match) dans le glossaire).

Vous connaissez déjà `case`/`when` ([les conditions](conditions-boucles.md)).
`case`/`in` lui ressemble, mais il sait aussi **ouvrir** la donnée.

```ruby
def decrire(valeur)
  case valeur
  in 0
    "zéro"
  in Integer
    "un entier"
  in String
    "un texte"
  in 1.0..100.0
    "un nombre à virgule entre 1 et 100"
  else
    "autre chose"
  end
end

puts decrire(0)
puts decrire(42)
puts decrire("salut")
puts decrire(3.5)
puts decrire(nil)
```

```sortie
zéro
un entier
un texte
un nombre à virgule entre 1 et 100
autre chose
```

Ruby essaie les motifs dans l'ordre et s'arrête au premier qui convient. Un motif peut
être une valeur (`0`), une classe (`Integer`) ou un
[intervalle](../debuter/glossaire.md#intervalle) (`1.0..100.0`).

### Plusieurs choix avec |

La barre `|` veut dire « ou ».

```ruby
def type_de_jour(jour)
  case jour
  in "samedi" | "dimanche" then "week-end"
  in String then "semaine"
  end
end

puts type_de_jour("dimanche")
puts type_de_jour("mardi")
```

```sortie
week-end
semaine
```

`then` permet d'écrire le motif et le résultat sur une seule ligne.

## Filtrer un tableau

Un motif entre crochets vérifie la **longueur** du tableau et le contenu de chaque case.
Un nom de variable accepte n'importe quoi et retient la valeur.

```ruby
def lire(commande)
  case commande
  in ["aller", direction]
    "Vous allez vers le #{direction}."
  in ["prendre", objet, quantite]
    "Vous prenez #{quantite} × #{objet}."
  in ["quitter"]
    "Au revoir !"
  in []
    "Commande vide."
  end
end

puts lire(["aller", "nord"])
puts lire(["prendre", "pomme", 3])
puts lire(["quitter"])
puts lire([])
```

```sortie
Vous allez vers le nord.
Vous prenez 3 × pomme.
Au revoir !
Commande vide.
```

### Le reste avec *

`*reste` attrape toutes les cases restantes, et `_` ignore une case.

```ruby
case [1, 2, 3, 4, 5]
in [premier, _, *reste]
  puts "premier : #{premier}"
  puts "reste : #{reste}"
end
```

```sortie
premier : 1
reste : [3, 4, 5]
```

### Chercher dans un tableau

Le motif `[*, x, *]` cherche **un endroit** du tableau qui correspond, n'importe où.
C'est le motif de recherche (*find pattern*).

```ruby
mesures = [12, 15, 31, 18, 40]

case mesures
in [*, 31.. => trop, *]
  puts "Premier dépassement : #{trop}"
end

case ["pomme", "kiwi", "poire"]
in [*avant, "kiwi", *apres]
  puts "Avant le kiwi : #{avant}"
  puts "Après le kiwi : #{apres}"
end
```

```sortie
Premier dépassement : 31
Avant le kiwi : ["pomme"]
Après le kiwi : ["poire"]
```

`31.. => trop` veut dire : « une valeur à partir de 31, que je nomme `trop` ».

## Filtrer un hash

Un motif entre accolades vérifie que certaines **clés** existent. Les autres clés sont
permises. `nom:` tout seul crée la variable `nom`.

```ruby
def accueillir(personne)
  case personne
  in { role: "admin", nom: }
    "Bienvenue, chef #{nom} !"
  in { nom:, age: Integer => age } if age < 18
    "Salut #{nom} !"
  in { nom: }
    "Bonjour #{nom}."
  end
end

puts accueillir({ nom: "Alice", role: "admin" })
puts accueillir({ nom: "Tom", age: 12 })
puts accueillir({ nom: "Marc", age: 40 })
```

```sortie
Bienvenue, chef Alice !
Salut Tom !
Bonjour Marc.
```

Les motifs s'emboîtent : un hash qui contient un tableau qui contient un hash, tout se
filtre d'un coup.

```ruby
reponse = { statut: "ok", donnees: { villes: ["Paris", "Lyon"] } }

case reponse
in { statut: "ok", donnees: { villes: [premiere, *] } }
  puts "Première ville : #{premiere}"
in { statut: "erreur" }
  puts "Échec"
end
```

```sortie
Première ville : Paris
```

> **Attention :** les clés d'un motif de hash sont des
> [symboles](../debuter/glossaire.md#symbole). Un hash à clés texte (`"nom" => …`) ne
> correspond pas.

## Les gardes avec if

Après un motif, `if` ajoute une condition : c'est une **garde**. Le motif doit
correspondre **et** la condition être vraie. `unless` marche aussi.

```ruby
def classer(point)
  case point
  in [0, 0] then "origine"
  in [x, y] if x == y then "sur la diagonale"
  in [x, _] if x.negative? then "à gauche"
  in [_, _] then "ailleurs"
  end
end

puts classer([0, 0])
puts classer([3, 3])
puts classer([-2, 5])
puts classer([4, 1])
```

```sortie
origine
sur la diagonale
à gauche
ailleurs
```

## Fixer une valeur avec ^

Dans un motif, un nom de variable **reçoit** toujours une nouvelle valeur. Pour
**comparer** avec une variable existante, mettez `^` devant : c'est l'épingle (*pin*).
Elle épingle la valeur actuelle.

```ruby
attendu = 5

case 5
in ^attendu then puts "C'est la bonne valeur."
else puts "Raté."
end

case [3, 3]
in [a, ^a] then puts "Deux fois #{a}."
end
```

```sortie
C'est la bonne valeur.
Deux fois 3.
```

Dans `[a, ^a]`, la première case remplit `a`, la deuxième doit lui être égale.

## Vos classes dans un motif

Un motif `Classe(...)` vérifie la classe, puis demande ses morceaux à l'objet :

- `deconstruct` renvoie un tableau, pour les motifs `Point(x, y)` ;
- `deconstruct_keys(cles)` renvoie un hash, pour les motifs `Point(x:, y:)`.

```ruby
class Point
  attr_reader :x, :y

  def initialize(x, y)
    @x = x
    @y = y
  end

  def deconstruct
    [x, y]
  end

  def deconstruct_keys(cles)
    { x: x, y: y }
  end
end

def ou(point)
  case point
  in Point(0, 0) then "à l'origine"
  in Point(x:, y: 0) then "sur l'axe horizontal, x = #{x}"
  in Point(x, y) then "en (#{x}, #{y})"
  end
end

puts ou(Point.new(0, 0))
puts ou(Point.new(7, 0))
puts ou(Point.new(2, 9))
```

```sortie
à l'origine
sur l'axe horizontal, x = 7
en (2, 9)
```

> **Astuce :** `Struct` et `Data` définissent déjà ces deux méthodes. Avec
> `Point = Data.define(:x, :y)`, les motifs fonctionnent tout de suite.

## in comme question, => pour extraire

Sans `case`, `valeur in motif` répond `true` ou `false`. Pratique dans un `if`.

```ruby
config = { port: 8080, hote: "localhost" }

ok = (config in { port: Integer })
puts ok
puts "Port valide" if config in { port: 1..65535 }
deux_entiers = ([1, "a"] in [Integer, Integer])
puts deux_entiers
```

```sortie
true
Port valide
false
```

> **Attention :** entourez `valeur in motif` de parenthèses quand vous le rangez
> dans une variable ou le passez à une méthode, comme ci-dessus.

`valeur => motif` extrait les morceaux, sans condition : la donnée **doit** avoir cette
forme. C'est une affectation vers la droite.

```ruby
utilisateur = { nom: "Alice", adresse: { ville: "Nantes", cp: "44000" } }

utilisateur => { nom:, adresse: { ville: } }
puts "#{nom} habite à #{ville}."
```

```sortie
Alice habite à Nantes.
```

Si la forme ne correspond pas, `=>` lève une erreur. `in` ne lève jamais d'erreur.

## Erreurs fréquentes

### Aucun motif ne correspond

```ruby norun
case 42
in String then puts "texte"
end
```

```text
main.rb:1:in `<main>': 42: String === 42 does not return true (NoMatchingPatternError)
```

Contrairement à `case`/`when`, `case`/`in` lève une erreur si aucun motif ne
convient. Ajoutez une branche `else` pour les cas restants.

### Un hash à clés texte

```ruby norun
personne = { "nom" => "Alice" }
case personne
in { nom: String => nom }
  puts nom
end
```

```text
main.rb:2:in `<main>': {"nom"=>"Alice"}: key not found: :nom (NoMatchingPatternKeyError)
```

Le motif cherche le symbole `:nom`, pas le texte `"nom"`. Convertissez d'abord le hash :
`personne.transform_keys(&:to_sym)`.

### Oublier l'épingle ^

```ruby norun
attendu = 5
case 8
in attendu then puts "trouvé : #{attendu}"
end
```

```text
trouvé : 8
```

Pas d'erreur, mais un résultat faux : sans `^`, `attendu` est une nouvelle variable
qui accepte tout, et elle vaut maintenant `8`. Écrivez `in ^attendu`.

## Testez-vous

**Question 1 :** que vaut `([1, 2, 3] in [_, _])` ?

> **Réponse**
>
> `false` : le motif demande exactement deux cases, le tableau en a trois. Il faudrait
> `[_, _, *]`.

**Question 2 :** quelle est la différence entre `donnee in motif` et `donnee => motif` ?

> **Réponse**
>
> `in` répond `true` ou `false`. `=>` extrait les variables et lève
> `NoMatchingPatternError` si la forme ne correspond pas.

**Question 3 :** écrivez une méthode `aire(forme)` qui accepte `{ type: :carre, cote: c }`
et `{ type: :rect, l: l, h: h }`, et renvoie `0` sinon.

> **Réponse**
>
> ```ruby
> def aire(forme)
>   case forme
>   in { type: :carre, cote: } then cote * cote
>   in { type: :rect, l:, h: } then l * h
>   else 0
>   end
> end
>
> puts aire({ type: :carre, cote: 3 })
> puts aire({ type: :rect, l: 2, h: 5 })
> puts aire({ type: :rond })
> ```
>
> ```sortie
> 9
> 10
> 0
> ```

## En résumé

- `case`/`in` vérifie la forme d'une donnée et range ses morceaux dans des variables.
- `[a, *reste]` filtre un tableau, `{ cle: }` un hash, `[*, x, *]` cherche dans un tableau.
- `if` ajoute une garde ; `^` compare avec une variable existante ; `|` veut dire « ou ».
- `deconstruct` et `deconstruct_keys` rendent vos classes filtrables.
- `in` répond `true`/`false`, `=>` extrait ou lève une erreur ; pensez au `else`.

Suite : [Gérer les erreurs](erreurs.md)
