# Premiers pas

Cette page présente les bases de Ruby : variables, nombres, textes, symboles, booléens, `nil` et affichage.

## Les variables

Une variable est un nom qui désigne une valeur. Vous la créez avec `=`.

```ruby
age = 30
prenom = "Alice"
puts prenom
puts age
```

```sortie
Alice
30
```

Vous n'avez pas besoin d'indiquer le type : Ruby le devine.

## Les nombres

Ruby distingue les entiers (`Integer`) et les nombres à virgule (`Float`).

```ruby
puts 7 + 3
puts 7 / 2
puts 7.0 / 2
puts 7 % 2
puts 2 ** 10
```

```sortie
10
3
3.5
1
1024
```

> **Attention :** `7 / 2` donne `3`. Entre deux entiers, la division garde seulement la
> partie entière. Écrivez `7.0 / 2` pour obtenir `3.5`.

## Les chaînes de caractères

Un texte s'écrit entre guillemets. On parle de **chaîne** (`String`).

```ruby
nom = "Ruby"
puts nom.length
puts nom.upcase
puts nom + " est sympa"
```

```sortie
4
RUBY
Ruby est sympa
```

### L'interpolation

Entre guillemets doubles, `#{...}` insère une valeur dans le texte.

```ruby
prenom = "Alice"
age = 30
puts "#{prenom} a #{age} ans."
puts "Dans 5 ans : #{age + 5}"
```

```sortie
Alice a 30 ans.
Dans 5 ans : 35
```

> **Attention :** l'interpolation ne marche pas avec des guillemets simples :
> `'#{age}'` affiche le texte tel quel.

## Les symboles

Un symbole commence par `:`. C'est un nom fixe, souvent utilisé comme étiquette.

```ruby
couleur = :rouge
puts couleur
p couleur
```

```sortie
rouge
:rouge
```

Vous les retrouverez comme clés de hash dans [les collections](collections.md).

## Les booléens et nil

`true` et `false` sont les booléens. `nil` veut dire « rien, aucune valeur ».

```ruby
puts 3 > 2
puts 3 == 4
p nil
puts nil.nil?
```

```sortie
true
false
nil
true
```

> **À retenir :** en Ruby, seuls `false` et `nil` sont « faux ». Tout le reste est vrai,
> même `0` et `""`.

## Afficher avec puts, print et p

- `puts` affiche puis passe à la ligne.
- `print` affiche sans passer à la ligne.
- `p` affiche la valeur telle que Ruby la voit (pratique pour déboguer).

```ruby
print "a"
print "b"
puts
puts "texte"
p "texte"
p 42
```

```sortie
ab
texte
"texte"
42
```

## Les commentaires

Un commentaire commence par `#`. Ruby l'ignore.

```ruby
# Ceci est un commentaire
puts "visible" # un commentaire en fin de ligne
```

```sortie
visible
```

## Les conventions de nommage

- Variables et méthodes : `snake_case` (`prix_total`).
- Classes et modules : `CamelCase` (`CompteBancaire`).
- Constantes : en majuscules (`TAUX_TVA`).

```ruby
TAUX_TVA = 0.2
prix_ht = 100
prix_ttc = prix_ht * (1 + TAUX_TVA)
puts prix_ttc
```

```sortie
120.0
```

> **Astuce :** suivez ces conventions : tout le monde les utilise, et votre code sera
> plus facile à lire.

## Et ensuite ?

Découvrez [les conditions et les boucles](conditions-boucles.md). Comparez aussi avec
[les premiers pas en Rust](../rust/premiers-pas.md).
