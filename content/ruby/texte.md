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

Presque tous les programmes manipulent du texte : un prénom à saluer, un message à
afficher, une réponse tapée au clavier.

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

Un message est souvent fait de morceaux : une partie fixe et une partie qui change.
C'est comme une lettre type où vous complétez le prénom à la main.

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

**À vous :** écrivez un programme qui range `"Léo"` dans `prenom` et `8` dans `age`,
puis affiche « Léo aura 9 ans l'an prochain. » avec l'interpolation.

> **Solution**
>
> ```ruby
> prenom = "Léo"
> age = 8
> puts "#{prenom} aura #{age + 1} ans l'an prochain."
> ```
>
> ```sortie
> Léo aura 9 ans l'an prochain.
> ```

## La longueur d'un texte

Connaître la longueur sert à vérifier ce que quelqu'un a tapé : un mot de passe trop
court, une case laissée vide.

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

Ligne par ligne :

- `"chat".length` compte c, h, a, t : 4 caractères ;
- `"un chat".length` donne 7, car l'espace entre les deux mots compte aussi ;
- `""` est un texte vide, sans aucun caractère : `empty?` répond donc `true`.

`empty?` répond `true` quand le texte est vide.

## Majuscules et minuscules

Les gens n'écrivent pas tous de la même façon : « alice », « ALICE », « Alice ». Ces
méthodes remettent un texte en forme, pour l'afficher proprement ou le comparer.

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

Un espace en trop ne se voit pas à l'écran. Pourtant, pour Ruby, `"Alice "` et
`"Alice"` sont deux textes différents.

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

Ligne par ligne :

- `saisie` contient « Alice », entouré de trois espaces de chaque côté ;
- la deuxième ligne colle des crochets autour du texte : ils montrent où il commence
  et où il finit, espaces compris ;
- la troisième fait pareil avec `saisie.strip` : les espaces ont disparu, les crochets
  touchent le prénom.

> **Astuce :** utilisez `strip` sur tout texte tapé par quelqu'un : on ajoute souvent
> des espaces sans le vouloir.

**À vous :** écrivez un programme qui range `"  alice  "` dans `saisie`, puis affiche
« Bonjour Alice ! », sans les espaces et avec une majuscule.

> **Solution**
>
> ```ruby
> saisie = "  alice  "
> nom = saisie.strip
> puts "Bonjour #{nom.capitalize} !"
> ```
>
> ```sortie
> Bonjour Alice !
> ```

## Chercher dans un texte

Chercher un morceau de texte sert partout : repérer un mot dans un message, vérifier
qu'une adresse finit par « .fr ».

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

Ligne par ligne :

- `phrase` contient le texte à examiner ;
- `include?("choco")` cherche « choco » n'importe où : il est dans « chocolat », donc
  `true` ;
- `start_with?("J'aime")` regarde seulement le début : `true` ;
- `end_with?(".")` regarde seulement la fin : la phrase n'a pas de point final, donc
  `false`.

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

Un texte regroupe souvent plusieurs informations : les mots d'une phrase, une liste de
courses séparée par des virgules. Pour les traiter une par une, il faut les séparer.

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

Ligne par ligne :

- `phrase.split(" ")` coupe à chaque espace : trois morceaux, rangés dans le tableau
  `fruits` ;
- `p fruits` affiche le tableau, avec ses crochets et ses guillemets ;
- `"rouge,vert,bleu".split(",")` coupe cette fois à chaque virgule ;
- `liste.join(" / ")` recolle les trois couleurs, avec « / » entre chacune.

Les tableaux sont expliqués dans [les tableaux et hashs](collections.md).

**À vous :** écrivez un programme qui découpe `"pain;lait;oeufs"` à chaque
point-virgule, puis recolle les morceaux avec « , » entre eux.

> **Solution**
>
> ```ruby
> courses = "pain;lait;oeufs".split(";")
> puts courses.join(", ")
> ```
>
> ```sortie
> pain, lait, oeufs
> ```

## Remplacer un morceau

C'est la fonction « Rechercher et remplacer » d'un traitement de texte : vous corrigez
un mot, ou vous changez un nom partout d'un coup.

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

Ligne par ligne :

- `texte` contient deux fois « un » ;
- `sub("un", "le")` ne change que le premier « un » ;
- `gsub("un", "le")` change les deux ;
- `texte` lui-même ne change pas : chaque ligne repart du texte d'origine.

> **À retenir :** pensez « g » comme « global » : `gsub` remplace partout.

## Prendre une partie du texte

Parfois, seule une partie du texte vous intéresse : l'initiale d'un prénom, l'année
dans une date.

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

Pour examiner chaque lettre d'un mot, pour les compter ou les transformer, il faut
d'abord les séparer.

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

Ligne par ligne :

- `"abc".chars` donne le tableau `["a", "b", "c"]`, que `p` affiche ;
- `each { |lettre| ... }` prend chaque caractère à tour de rôle, le range dans
  `lettre`, puis exécute `puts lettre.upcase` : A, puis B, puis C ;
- `reverse` lit le texte de la fin vers le début.

`reverse` renvoie le texte à l'envers.

## Un texte sur plusieurs lignes

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

Écrire une lettre entière sur une seule ligne, avec des `\n` partout, serait
illisible. Le heredoc vous laisse écrire le texte tel qu'il s'affichera.

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

Un « 42 » tapé au clavier est un texte, pas un nombre : Ruby refuse de calculer avec.
Il faut d'abord le convertir. Et pour afficher un nombre proprement, on le met en forme.

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

Ligne par ligne :

- `"42".to_i` transforme le texte en nombre 42 : on peut alors ajouter 1, ce qui
  donne 43 ;
- `"2.5".to_f` donne le nombre 2.5 ; fois 2, cela fait 5.0 ;
- `10.to_s` donne le texte `"10"` ; `+ "0"` colle un zéro au bout : `"100"`. Aucun
  calcul n'a lieu.

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

**À vous :** écrivez un programme qui range le texte `"2.5"` (un prix) dans `prix` et
`3` dans `quantite`, puis affiche le total : « Total : 7.50 € ».

> **Solution**
>
> ```ruby
> prix = "2.5"
> quantite = 3
> total = prix.to_f * quantite
> puts format("Total : %.2f €", total)
> ```
>
> ```sortie
> Total : 7.50 €
> ```

## Les textes gelés

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

Geler un texte le protège : une valeur qui ne doit jamais changer, comme le nom de
votre site, ne risque plus d'être modifiée par erreur.

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

Ligne par ligne :

- `"salut".freeze` crée un texte gelé, rangé dans `mot` ;
- `mot.frozen?` vérifie qu'il est bien gelé : `true` ;
- `mot.dup` fabrique une copie, qui, elle, n'est pas gelée ;
- `copie << " !"` ajoute « ! » au bout de la copie. `mot` vaut toujours `"salut"`.

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
