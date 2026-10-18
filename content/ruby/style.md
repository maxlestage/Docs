# Bien écrire du Ruby

Cette page présente les habitudes d'écriture des programmeurs Ruby : les noms, la mise en page, les tournures idiomatiques, et l'outil RuboCop qui vérifie tout cela pour vous.

> **Objectifs :**
>
> - nommer variables, méthodes et classes comme le veut l'usage ;
> - simplifier une méthode avec les clauses de garde et le retour implicite ;
> - préférer `each`, `map` et `select` aux boucles écrites à la main ;
> - écrire de petites méthodes et des commentaires utiles ;
> - vérifier son style avec RuboCop.

## Pourquoi soigner son style ?

Un programme est lu bien plus souvent qu'il n'est écrit. Un code qui suit les habitudes
de tous se lit comme un texte bien ponctué. Ces règles, tirées du *Ruby Style Guide* de
la communauté, ne changent pas ce que fait le programme : seulement la facilité à le lire.

## Les noms

| Quoi | Style | Exemple |
|------|-------|---------|
| variable, méthode | `snake_case` (minuscules et `_`) | `prix_total`, `calculer_tva` |
| classe, module | `CamelCase` (majuscule à chaque mot) | `CompteBancaire` |
| constante | `SCREAMING_SNAKE_CASE` (majuscules) | `TAUX_TVA` |
| fichier | `snake_case.rb` | `compte_bancaire.rb` |

Choisissez des noms qui disent **ce que contient** la valeur, pas son type : `prix` plutôt
que `nombre`, `eleves` plutôt que `tableau`. Les abréviations obscures (`nb_elv`) font
gagner trois lettres et perdre une minute à chaque lecture.

```ruby
TAUX_TVA = 0.2

def prix_ttc(prix_ht)
  prix_ht * (1 + TAUX_TVA)
end

puts prix_ttc(10)
```

```sortie
12.0
```

## La mise en page

- **Deux espaces** pour indenter, jamais de tabulations.
- Des espaces autour des opérateurs : `a + b`, pas `a+b`.
- Une ligne vide entre deux méthodes.
- Des lignes courtes : environ 80 à 120 caractères au plus.
- Des parenthèses autour des arguments d'une méthode, sauf pour les méthodes très
  courantes comme `puts`, `p` ou `require`.

```ruby
def perimetre(largeur, hauteur)
  2 * (largeur + hauteur)
end

puts perimetre(3, 4)
```

```sortie
14
```

## Le retour implicite

Une méthode Ruby renvoie sa dernière expression (voir
[le retour implicite](fonctions.md#le-retour-implicite)). `return` en fin de méthode est
donc inutile : le style Ruby l'omet.

```ruby
# Avant
def carre(x)
  return x * x
end

# Après
def carre_court(x)
  x * x
end

puts carre(5), carre_court(5)
```

```sortie
25
25
```

## Les clauses de garde

Des `if` imbriqués poussent le code de plus en plus à droite. Une **clause de garde**
traite d'abord les cas particuliers et quitte tout de suite la méthode. C'est le videur à
l'entrée d'une salle : il refuse les cas à problème, et la suite n'a plus à s'en soucier.

```ruby
# Avant
def prix_entree(age)
  if age
    if age < 12
      5
    else
      10
    end
  else
    "âge inconnu"
  end
end

# Après
def prix_entree_garde(age)
  return "âge inconnu" if age.nil?
  return 5 if age < 12

  10
end

p [prix_entree(8), prix_entree(nil), prix_entree(30)]
p [prix_entree_garde(8), prix_entree_garde(nil), prix_entree_garde(30)]
```

```sortie
[5, "âge inconnu", 10]
[5, "âge inconnu", 10]
```

Ici `return` est utile : il sort **avant** la fin. On laisse une ligne vide après les
gardes.

## Les noms en ? et en !

- Une méthode qui répond oui ou non finit par `?` : `vide?`, `majeur?`.
- Une méthode « dangereuse », qui modifie l'objet lui-même, finit par `!` quand une
  version sans `!` existe aussi.

```ruby
def majeur?(age)
  age >= 18
end

p majeur?(20)

nom = "alice"
nom.capitalize!
p nom
```

```sortie
true
"Alice"
```

Ne commencez pas ces méthodes par `est_` : `majeur?` se lit mieux que `est_majeur?`.

## Les tournures idiomatiques

### each plutôt que for

`for` existe, mais les rubyistes utilisent `each`. Avec `for`, la variable de boucle
« fuit » après la boucle ; avec `each`, elle reste dans le bloc.

```ruby
for i in 1..3
end
p i

[1, 2, 3].each { |j| }
p defined?(j)
```

```sortie
3
nil
```

### map et select plutôt qu'une boucle

Remplir un tableau vide dans une boucle est un signe : il existe sûrement une méthode
de [Enumerable](blocs.md#les-methodes-de-enumerable) qui le fait.

```ruby
nombres = [1, 2, 3, 4, 5, 6]

# Avant
pairs_doubles = []
nombres.each do |n|
  pairs_doubles << n * 2 if n.even?
end
p pairs_doubles

# Après
p nombres.select(&:even?).map { |n| n * 2 }
```

```sortie
[4, 8, 12]
[4, 8, 12]
```

La version courte se lit comme une phrase : « garde les pairs, puis double-les ».

### La navigation sûre

`&.` remplace un test de `nil` (voir [se protéger de nil](erreurs.md#se-proteger-de-nil)).

```ruby
ville = nil

# Avant
p(ville.nil? ? nil : ville.upcase)

# Après
p ville&.upcase
```

```sortie
nil
nil
```

### frozen_string_literal

Ce commentaire magique, **en première ligne** du fichier, rend toutes les chaînes écrites
entre guillemets [immuables](../debuter/glossaire.md#immuable) : on ne peut plus les
modifier sur place. Le programme est un peu plus rapide, et les modifications
accidentelles deviennent des erreurs visibles.

```ruby
# frozen_string_literal: true

salut = "bonjour"
p salut.frozen?
```

```sortie
true
```

Pour obtenir une chaîne modifiable, utilisez `dup`, `+"texte"` ou `String.new`.

## De petites méthodes

Une méthode devrait faire **une seule chose**, et tenir en quelques lignes. Si vous avez
besoin d'un commentaire pour séparer ses parties, chaque partie mérite sa propre
méthode.

```ruby
# Avant : une méthode qui fait tout
def bulletin(notes)
  total = 0
  notes.each { |n| total += n }
  moy = total.to_f / notes.size
  mention = if moy >= 16 then "très bien" elsif moy >= 12 then "bien" else "passable" end
  "Moyenne #{moy.round(1)} : #{mention}"
end

# Après : trois petites méthodes au nom clair
def moyenne(notes)
  notes.sum.fdiv(notes.size)
end

def mention(moyenne)
  return "très bien" if moyenne >= 16
  return "bien" if moyenne >= 12

  "passable"
end

def bulletin_clair(notes)
  moy = moyenne(notes)
  "Moyenne #{moy.round(1)} : #{mention(moy)}"
end

puts bulletin([12, 15, 17])
puts bulletin_clair([12, 15, 17])
```

```sortie
Moyenne 14.7 : bien
Moyenne 14.7 : bien
```

## Des commentaires qui expliquent pourquoi

Le code dit déjà **ce qu'il fait**. Un bon commentaire dit **pourquoi** : une raison
qu'on ne peut pas deviner en lisant.

```ruby norun
# Inutile : le code dit déjà la même chose.
# Ajoute 1 au compteur.
compteur += 1

# Utile : explique un choix.
# La banque arrondit au centime inférieur, pas au plus proche.
interets = (montant * 100).floor / 100.0
```

## Vérifier avec RuboCop

**RuboCop** est un *linter* : un correcteur de style, comme un correcteur d'orthographe
pour le code. C'est une [gem](../debuter/glossaire.md#gem) à installer :

```sh
gem install rubocop
```

Voici un fichier `panier.rb` écrit sans soin :

```ruby norun
def calculTotal(prix)
    total = 0
    for p in prix
      total = total + p
    end
    return total
end

puts calculTotal([3, 4])
```

```sh
rubocop panier.rb
```

```text
Inspecting 1 file
C

Offenses:

panier.rb:1:1: C: [Correctable] Style/FrozenStringLiteralComment: Missing frozen string literal comment.
def calculTotal(prix)
^
panier.rb:1:5: C: Naming/MethodName: Use snake_case for method names.
def calculTotal(prix)
    ^^^^^^^^^^^
panier.rb:2:1: C: [Correctable] Layout/IndentationWidth: Use 2 (not 4) spaces for indentation.
    total = 0
^^^^
panier.rb:3:5: C: [Correctable] Style/For: Prefer each over for.
    for p in prix ...
    ^^^^^^^^^^^^^
panier.rb:4:7: C: [Correctable] Style/SelfAssignment: Use self-assignment shorthand +=.
      total = total + p
      ^^^^^^^^^^^^^^^^^
panier.rb:6:5: C: [Correctable] Style/RedundantReturn: Redundant return detected.
    return total
    ^^^^^^

1 file inspected, 6 offenses detected, 5 offenses autocorrectable
```

Chaque remarque donne la ligne, la colonne, le nom de la règle et une explication.
`rubocop -a` corrige automatiquement les remarques marquées `[Correctable]` ; relisez
toujours le résultat. Après corrections, le fichier devient :

```ruby
# frozen_string_literal: true

def calcul_total(prix)
  prix.sum
end

puts calcul_total([3, 4])
```

```sortie
7
```

> **Astuce :** les règles se règlent dans un fichier `.rubocop.yml` à la racine du projet.
> Une équipe y note ses choix une fois pour toutes.

## Erreurs fréquentes

### Modifier une chaîne gelée

```ruby norun
# frozen_string_literal: true
s = "abc"
s << "d"
```

```text
main.rb:3:in `<main>': can't modify frozen String: "abc" (FrozenError)
```

Avec `frozen_string_literal`, les chaînes littérales sont gelées. Créez une nouvelle
chaîne (`s += "d"`) ou partez d'une copie modifiable (`s = +"abc"`).

### Enchaîner après une méthode en !

```ruby norun
nom = "ABC"
puts nom.upcase!.downcase
```

```text
main.rb:2:in `<main>': undefined method `downcase' for nil (NoMethodError)

puts nom.upcase!.downcase
                ^^^^^^^^^
```

`upcase!` renvoie `nil` quand il n'a rien changé (le texte était déjà en majuscules).
N'enchaînez pas après une méthode en `!` : utilisez `nom.upcase.downcase`.

### Mettre une majuscule à une méthode

```ruby norun
def Bonjour
  "Bonjour !"
end

puts Bonjour
```

```text
main.rb:5:in `<main>': uninitialized constant Bonjour (NameError)

puts Bonjour
     ^^^^^^^
```

Un nom qui commence par une majuscule est lu comme une **constante**, pas comme un
appel de méthode. Écrivez les méthodes en `snake_case` : `def bonjour`.

## Testez-vous

**Question 1 :** comment nommer une constante contenant le nombre maximal de joueurs ?

> **Réponse**
>
> En majuscules avec des `_` : `JOUEURS_MAX` (ou `NOMBRE_MAX_JOUEURS`).

**Question 2 :** réécrivez cette méthode avec une clause de garde.

```ruby norun
def saluer(nom)
  if nom.empty?
    "Bonjour, inconnu"
  else
    "Bonjour, #{nom}"
  end
end
```

> **Réponse**
>
> ```ruby
> def saluer(nom)
>   return "Bonjour, inconnu" if nom.empty?
>
>   "Bonjour, #{nom}"
> end
>
> puts saluer("")
> puts saluer("Alice")
> ```
>
> ```sortie
> Bonjour, inconnu
> Bonjour, Alice
> ```

**Question 3 :** remplacez cette boucle par une seule ligne idiomatique.

```ruby norun
longueurs = []
%w[chat chien oiseau].each { |mot| longueurs << mot.length }
```

> **Réponse**
>
> ```ruby
> longueurs = %w[chat chien oiseau].map(&:length)
> p longueurs
> ```
>
> ```sortie
> [4, 5, 6]
> ```

## En résumé

- `snake_case` pour les variables et méthodes, `CamelCase` pour les classes,
  `MAJUSCULES` pour les constantes ; deux espaces d'indentation.
- Pas de `return` final ; des clauses de garde plutôt que des `if` imbriqués.
- `?` pour les questions, `!` pour les versions qui modifient l'objet.
- `each`, `map`, `select` et `&.` plutôt que des boucles et des tests de `nil`.
- De petites méthodes, des commentaires qui disent pourquoi, et RuboCop pour vérifier.

Suite : [métaprogrammation en douceur](metaprogrammation.md).
