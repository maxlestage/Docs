# Travailler avec du texte

Cette page montre comment créer, assembler, découper et transformer du texte en Ruby.

## Créer un texte

Un texte s'appelle une **chaîne** (`String`). Vous l'écrivez entre guillemets doubles
`"..."` ou simples `'...'`.

```ruby
salut = "Bonjour"
nom = 'Alice'
puts salut
puts nom
```

```sortie
Bonjour
Alice
```

> **Astuce :** prenez l'habitude des guillemets doubles. Ils permettent l'interpolation,
> expliquée juste après.

## Assembler des textes

### Avec +

Le signe `+` colle deux textes bout à bout.

```ruby
prenom = "Alice"
puts "Bonjour " + prenom + " !"
```

```sortie
Bonjour Alice !
```

`+` ne marche qu'entre deux textes. Pour un nombre, transformez-le d'abord avec `to_s` :

```ruby
age = 30
puts "J'ai " + age.to_s + " ans."
```

```sortie
J'ai 30 ans.
```

### Avec l'interpolation

Entre guillemets doubles, `#{...}` place une valeur dans le texte. C'est plus simple
que `+`, et cela marche avec les nombres.

```ruby
prenom = "Alice"
age = 30
puts "#{prenom} a #{age} ans."
puts "L'an prochain : #{age + 1} ans."
```

```sortie
Alice a 30 ans.
L'an prochain : 31 ans.
```

> **Attention :** avec des guillemets simples, `'#{age}'` reste tel quel. Ruby ne
> remplace rien.

## La longueur d'un texte

`length` donne le nombre de caractères. Les espaces comptent aussi.

```ruby
puts "chat".length
puts "un chat".length
puts "".empty?
```

```sortie
4
7
true
```

`empty?` répond `true` quand le texte est vide.

## Majuscules et minuscules

- `upcase` met tout en majuscules.
- `downcase` met tout en minuscules.
- `capitalize` met une majuscule au début, et le reste en minuscules.

```ruby
mot = "bonJOUR"
puts mot.upcase
puts mot.downcase
puts mot.capitalize
```

```sortie
BONJOUR
bonjour
Bonjour
```

> **Note :** ces méthodes renvoient un **nouveau** texte. La variable `mot` ne change
> pas.

## Enlever les espaces autour

`strip` retire les espaces (et les retours à la ligne) au début et à la fin.

```ruby
saisie = "   Alice   "
puts "[" + saisie + "]"
puts "[" + saisie.strip + "]"
```

```sortie
[   Alice   ]
[Alice]
```

> **Astuce :** utilisez `strip` sur tout texte tapé par quelqu'un : on ajoute souvent
> des espaces sans le vouloir.

## Chercher dans un texte

- `include?` : le texte contient-il ce morceau ?
- `start_with?` : commence-t-il par ce morceau ?
- `end_with?` : finit-il par ce morceau ?

```ruby
phrase = "J'aime le chocolat"
puts phrase.include?("choco")
puts phrase.start_with?("J'aime")
puts phrase.end_with?(".")
```

```sortie
true
true
false
```

> **Attention :** la recherche tient compte des majuscules. `"Chat".include?("chat")`
> vaut `false`. Passez par `downcase` pour comparer sans les majuscules.

## Découper et recoller

`split` coupe un texte en morceaux et renvoie un tableau. `join` fait l'inverse : il
recolle un tableau en un seul texte.

```ruby
phrase = "pomme banane cerise"
fruits = phrase.split(" ")
p fruits

liste = "rouge,vert,bleu".split(",")
p liste
puts liste.join(" / ")
```

```sortie
["pomme", "banane", "cerise"]
["rouge", "vert", "bleu"]
rouge / vert / bleu
```

Les tableaux sont expliqués dans [les tableaux et hashs](collections.md).

## Remplacer un morceau

`sub` remplace la **première** fois qu'un morceau apparaît. `gsub` les remplace
**toutes**.

```ruby
texte = "un chat, un chien"
puts texte.sub("un", "le")
puts texte.gsub("un", "le")
```

```sortie
le chat, un chien
le chat, le chien
```

> **À retenir :** pensez « g » comme « global » : `gsub` remplace partout.

## Prendre une partie du texte

Comme dans un tableau, chaque caractère a une position. La première est `0`.

```ruby
mot = "bonjour"
puts mot[0]
puts mot[-1]
puts mot[0..2]
puts mot[3..]
```

```sortie
b
r
bon
jour
```

- `mot[0]` : le premier caractère.
- `mot[-1]` : le dernier.
- `mot[0..2]` : de la position 0 à la position 2, incluse.
- `mot[3..]` : de la position 3 jusqu'à la fin.

## Les caractères un par un

`chars` donne le tableau de tous les caractères. Pratique pour les parcourir.

```ruby
p "abc".chars
"abc".chars.each { |lettre| puts lettre.upcase }
puts "abc".reverse
```

```sortie
["a", "b", "c"]
A
B
C
cba
```

`reverse` renvoie le texte à l'envers.

## Un texte sur plusieurs lignes

Pour un long texte, utilisez un **heredoc** : `<<~FIN`, puis vos lignes, puis `FIN`
seul sur sa ligne.

```ruby
nom = "Alice"
lettre = <<~FIN
  Bonjour #{nom},
  Merci pour votre visite.
  À bientôt !
FIN
puts lettre
```

```sortie
Bonjour Alice,
Merci pour votre visite.
À bientôt !
```

> **Note :** le `~` retire les espaces du début des lignes. Vous pouvez donc indenter
> votre texte proprement. Le mot `FIN` est libre : `TEXTE` ou `MSG` marchent aussi.

## Texte et nombres

### Passer de l'un à l'autre

- `to_i` transforme un texte en entier, `to_f` en nombre à virgule.
- `to_s` transforme un nombre en texte.

```ruby
puts "42".to_i + 1
puts "2.5".to_f * 2
puts 10.to_s + "0"
```

```sortie
43
5.0
100
```

### Arrondir et formater

`round(2)` arrondit à 2 chiffres après la virgule. `format` construit un texte avec un
modèle : `%.2f` veut dire « un nombre avec 2 chiffres après la virgule ».

```ruby
prix = 12.34567
puts prix.round(2)
puts format("%.2f €", prix)
puts format("%.2f €", 5)
puts format("Ticket n°%03d", 7)
```

```sortie
12.35
12.35 €
5.00 €
Ticket n°007
```

> **Astuce :** `%03d` écrit un entier sur 3 chiffres, avec des zéros devant.

## Les textes gelés

Un texte **gelé** (`freeze`) ne peut plus être modifié. Pour le changer, faites-en une
copie avec `dup`.

```ruby
mot = "salut".freeze
puts mot.frozen?
copie = mot.dup
copie << " !"
puts copie
```

```sortie
true
salut !
```

`<<` ajoute du texte à la fin d'une chaîne. Sur un texte gelé, il provoque une erreur
`FrozenError`.

> **Note :** vous verrez parfois la ligne `# frozen_string_literal: true` en haut d'un
> fichier. Elle gèle tous les textes écrits dans ce fichier. Les méthodes comme `upcase`
> marchent toujours : elles créent un nouveau texte.

## En résumé

- Un texte s'écrit entre guillemets doubles ; `#{...}` y insère une valeur.
- `length`, `upcase`, `downcase`, `capitalize` et `strip` renvoient un nouveau texte.
- `include?`, `start_with?` et `end_with?` cherchent un morceau.
- `split` découpe en tableau, `join` recolle ; `sub` et `gsub` remplacent.
- `mot[0..2]` prend une partie du texte, `chars` donne les caractères un par un.
- `round` et `format` affichent proprement les nombres.

Suite : [les tableaux et les hashs](collections.md). Comparez avec
[le texte en Rust](../rust/texte.md).
