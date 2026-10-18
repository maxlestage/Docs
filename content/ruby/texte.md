# Travailler avec du texte

Cette page montre comment créer, assembler, découper et transformer du texte en Ruby.

> **Objectifs :**
>
> - vous saurez écrire un texte et y insérer des valeurs ;
> - vous saurez transformer un texte (majuscules, espaces, remplacements) ;
> - vous saurez chercher, découper et extraire une partie d'un texte ;
> - vous saurez passer d'un texte à un nombre, et l'inverse ;
> - vous saurez afficher un nombre proprement avec `format`.

## Créer un texte

Un texte s'appelle une **chaîne** (`String`), ou
[chaîne de caractères](../debuter/glossaire.md#chaine-de-caracteres) : une suite de
caractères (lettres, chiffres, espaces, signes) enfilés comme des perles. Vous l'écrivez
entre guillemets doubles `"..."` ou simples `'...'`.

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

### Les caractères spéciaux

Entre guillemets doubles, la barre `\` annonce un caractère spécial. On parle de
**séquence d'échappement** :

- `\n` passe à la ligne ;
- `\t` insère une tabulation (un grand espace) ;
- `\"` écrit un guillemet sans fermer le texte ;
- `\\` écrit une barre `\`.

```ruby
puts "Ligne 1\nLigne 2"
puts "Nom\tÂge"
puts "Il a dit : \"Bonjour\""
```

```sortie
Ligne 1
Ligne 2
Nom	Âge
Il a dit : "Bonjour"
```

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

### Répéter un texte

`*` répète un texte plusieurs fois.

```ruby
puts "ha" * 3
puts "-" * 10
```

```sortie
hahaha
----------
```

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

Pour modifier le texte lui-même, ajoutez un `!` : `mot.upcase!`. C'est la convention
vue dans [les méthodes](fonctions.md#les-noms-en-et-en).

```ruby
mot = "bonjour"
mot.upcase
puts mot
mot.upcase!
puts mot
```

```sortie
bonjour
BONJOUR
```

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

### Comparer, compter et trouver une position

- `==` compare deux textes, lettre par lettre ;
- `casecmp?` compare sans tenir compte des majuscules ;
- `count` compte des caractères ;
- `index` donne la position d'un morceau (ou `nil` s'il est absent).

```ruby
puts "Chat" == "chat"
puts "Chat".casecmp?("chat")
puts "ananas".count("a")
p "ananas".index("n")
p "ananas".index("z")
```

```sortie
false
true
3
1
nil
```

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

Comme dans un tableau, chaque caractère a une position, appelée
[indice](../debuter/glossaire.md#indice). La première est `0`, pas `1` : l'indice
compte combien de pas il faut faire depuis le début.

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

Une position qui n'existe pas donne `nil` (aucune valeur) :

```ruby
p "chat"[10]
```

```sortie
nil
```

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

### Aligner du texte

`ljust` complète à droite avec des espaces, `rjust` à gauche, `center` des deux côtés.
Pratique pour faire des colonnes.

```ruby
puts "pain".ljust(8, ".") + "1,20 €"
puts "confiture".ljust(8, ".") + "3,50 €"
puts "42".rjust(5, "0")
puts "titre".center(11, "*")
```

```sortie
pain....1,20 €
confiture3,50 €
00042
***titre***
```

Le texte n'est jamais coupé : `"confiture"` fait déjà plus de 8 caractères.

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

Un texte **gelé** (`freeze`) ne peut plus être modifié : il devient
[immuable](../debuter/glossaire.md#immuable). Pour le changer, faites-en une
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

## Erreurs fréquentes

### Oublier de convertir un texte en nombre

```ruby norun
age = "30"
puts age + 1
```

```text
main.rb:2:in `+': no implicit conversion of Integer into String (TypeError)

puts age + 1
           ^
	from main.rb:2:in `<main>'
```

`"30"` est un texte, pas un nombre. Convertissez-le d'abord : `age.to_i + 1`.

### Utiliser une position qui n'existe pas

```ruby norun
mot = "chat"
puts mot[10].upcase
```

```text
main.rb:2:in `<main>': undefined method `upcase' for nil (NoMethodError)

puts mot[10].upcase
            ^^^^^^^
```

`mot[10]` vaut `nil`, et `nil` ne sait pas faire `upcase`. Vérifiez la longueur
(`mot.length`), ou utilisez `&.` : `mot[10]&.upcase` donne `nil` sans planter.

### Modifier un texte gelé

```ruby norun
mot = "salut".freeze
mot << " !"
```

```text
main.rb:2:in `<main>': can't modify frozen String: "salut" (FrozenError)
```

Un texte gelé ne change plus. Créez un nouveau texte (`mot + " !"`), ou modifiez une
copie (`mot.dup << " !"`).

## Testez-vous

**Question 1 :** qu'affiche `puts "Ruby".downcase.reverse` ?

> **Réponse**
>
> `ybur`. `downcase` donne `"ruby"`, puis `reverse` le retourne. Les méthodes
> s'enchaînent de gauche à droite.

**Question 2 :** comment obtenir `"Bonjour Alice"` à partir de `"  bonjour alice "` ?

> **Réponse**
>
> ```ruby
> texte = "  bonjour alice "
> puts texte.strip.split(" ").map(&:capitalize).join(" ")
> ```
>
> ```sortie
> Bonjour Alice
> ```
>
> `strip` enlève les espaces, `split` découpe en mots, `map(&:capitalize)` met une
> majuscule à chaque mot (voir [les blocs](blocs.md)), `join` recolle.

**Question 3 :** que vaut `"12" + "3"` ? Et `"12".to_i + "3".to_i` ?

> **Réponse**
>
> `"123"` (deux textes collés), puis `15` (deux nombres additionnés).

## En résumé

- Un texte s'écrit entre guillemets doubles ; `#{...}` y insère une valeur.
- `length`, `upcase`, `downcase`, `capitalize` et `strip` renvoient un nouveau texte.
- `include?`, `start_with?` et `end_with?` cherchent un morceau.
- `split` découpe en tableau, `join` recolle ; `sub` et `gsub` remplacent.
- `mot[0..2]` prend une partie du texte, `chars` donne les caractères un par un.
- `round` et `format` affichent proprement les nombres ; `to_i` et `to_s` convertissent.

Suite : [les tableaux et les hashs](collections.md). Comparez avec
[le texte en Rust](../rust/texte.md).
