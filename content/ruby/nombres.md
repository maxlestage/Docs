# Nombres et calculs

Cette page montre comment Ruby calcule avec les entiers et les nombres à virgule, comment arrondir, compter de l'argent sans erreur et afficher joliment un nombre.

> **Objectifs :**
>
> - vous saurez faire la différence entre un `Integer` et un `Float` ;
> - vous saurez éviter le piège de la division entière ;
> - vous saurez arrondir, convertir et formater un nombre ;
> - vous saurez pourquoi `0.1 + 0.2` ne vaut pas exactement `0.3`, et quoi faire pour l'argent ;
> - vous saurez tirer des nombres au hasard de façon reproductible.

## Deux sortes de nombres

Un programme compte sans arrêt : un âge, un prix, un score, une température. Parfois il
faut un nombre entier (3 enfants), parfois un nombre à virgule (1,5 litre de lait).

Ruby a deux sortes principales de nombres :

- les [entiers](../debuter/glossaire.md#entier) (`Integer`) : `3`, `-12`, `1_000_000` ;
- les [flottants](../debuter/glossaire.md#flottant) (`Float`), avec une virgule écrite
  comme un point : `3.5`, `-0.25`.

```ruby
puts 42.class
puts 3.14.class
puts 1_000_000
puts 2**100
```

```sortie
Integer
Float
1000000
1267650600228229401496703205376
```

Ligne par ligne :

- `42.class` demande à Ruby la sorte du nombre 42 : c'est un `Integer` ;
- `3.14` contient un point, c'est donc un `Float` ;
- `1_000_000` s'affiche sans les `_` : ils ne servent qu'à vous, pour lire ;
- `2**100` multiplie 2 par lui-même 100 fois : le résultat est énorme, et Ruby le donne
  en entier, chiffre par chiffre.

Le `_` sert seulement à rendre le nombre lisible, comme l'espace dans « 1 000 000 ».
Un entier Ruby n'a pas de limite de taille : `2**100` marche sans problème.

## Les opérateurs

Un ordinateur est d'abord une machine à calculer. Comme sur une calculatrice, vous
écrivez le calcul, et Ruby donne le résultat.

Les [opérateurs](../debuter/glossaire.md#operateur) font les calculs de base.

```ruby
puts 7 + 2
puts 7 - 2
puts 7 * 2
puts 7 / 2
puts 7 % 2
puts 7**2
```

```sortie
9
5
14
3
1
49
```

Ligne par ligne :

- les trois premières lignes font une addition, une soustraction et une multiplication,
  comme à l'école ;
- `7 / 2` donne `3`, et pas `3.5` : la section suivante explique pourquoi ;
- `7 % 2` donne ce qui reste de la division, et `7**2` multiplie 7 par lui-même.

- `%` donne le **reste** de la division : 7 = 2 × 3 + **1**.
- `**` est la puissance : `7**2` vaut 7 × 7.

> **Astuce :** `n % 2 == 0` teste si `n` est pair. Ruby a aussi `n.even?` et `n.odd?`.

### L'ordre des calculs

Comme à l'école, `**` passe avant `*`, `/` et `%`, qui passent avant `+` et `-`. Les
parenthèses passent avant tout.

```ruby
puts 2 + 3 * 4
puts (2 + 3) * 4
puts 2 * 3**2
puts -2**2
```

```sortie
14
20
18
-4
```

Le dernier résultat surprend : `-2**2` se lit `-(2**2)`. Écrivez `(-2)**2` pour obtenir
`4`. Dans le doute, mettez des parenthèses : elles ne coûtent rien.

**À vous :** vous avez 25 jours de vacances. Écrivez un programme qui affiche le nombre
de semaines complètes (7 jours), puis le nombre de jours qui restent en plus.

> **Solution**
>
> ```ruby
> puts 25 / 7
> puts 25 % 7
> ```
>
> ```sortie
> 3
> 4
> ```

## Division entière et division à virgule

Si les deux nombres sont des entiers, `/` donne un entier : la partie après la virgule
est **jetée**. C'est comme partager 7 bonbons entre 2 enfants : chacun en a 3, et il en
reste 1.

```ruby
puts 7 / 2
puts 7.0 / 2
puts 7 / 2.0
puts 7.fdiv(2)
p 7.divmod(2)
```

```sortie
3
3.5
3.5
3.5
[3, 1]
```

Ligne par ligne :

- `7 / 2` : les deux nombres sont des entiers, donc la virgule est jetée : `3` ;
- `7.0 / 2` et `7 / 2.0` : un des deux nombres est un `Float`, le résultat garde sa
  virgule : `3.5` ;
- `7.fdiv(2)` demande une division à virgule, même entre deux entiers ;
- `7.divmod(2)` donne deux nombres d'un coup : le quotient `3` et le reste `1`.

Dès qu'un des deux nombres est un `Float`, le résultat est un `Float`. `fdiv` force une
division à virgule, et `divmod` donne le quotient et le reste d'un coup.

> **Attention :** `-7 / 2` vaut `-4`, pas `-3` : Ruby arrondit toujours vers le bas.

## Arrondir

Un calcul donne souvent trop de chiffres après la virgule. Un prix de 3,756 €, personne
ne peut le payer : arrondir garde seulement les chiffres utiles.

```ruby
x = 3.756
puts x.round
puts x.round(1)
puts x.floor
puts x.ceil
puts x.truncate
puts 1234.round(-2)
```

```sortie
4
3.8
3
4
3
1200
```

Ligne par ligne :

- `x = 3.756` range le nombre dans la variable `x` ;
- `x.round` donne l'entier le plus proche : 3,756 est plus près de 4 que de 3 ;
- `x.round(1)` garde un seul chiffre après la virgule : `3.8` ;
- `floor`, `ceil` et `truncate` donnent `3`, `4` et `3` (voir la liste ci-dessous) ;
- `1234.round(-2)` arrondit à la centaine la plus proche : `1200`.

- `round` arrondit au plus proche ; `round(1)` garde un chiffre après la virgule.
- `floor` arrondit vers le bas (le « plancher »), `ceil` vers le haut (le « plafond »).
- `truncate` coupe la partie après la virgule.
- Un nombre négatif dans `round` arrondit aux dizaines, centaines…

**À vous :** un trajet coûte `12.456` €. Écrivez un programme qui range ce prix dans une
variable, puis affiche le prix arrondi à 2 chiffres après la virgule, puis l'entier
supérieur.

> **Solution**
>
> ```ruby
> prix = 12.456
> puts prix.round(2)
> puts prix.ceil
> ```
>
> ```sortie
> 12.46
> 13
> ```

## Le piège des flottants

Cette partie explique pourquoi un calcul tout simple peut donner un résultat un tout
petit peu faux, et comment l'éviter, surtout quand il s'agit d'argent.

Un `Float` est rangé en binaire dans l'ordinateur. Certains nombres simples pour nous,
comme `0.1`, ne tombent pas juste en binaire, comme `1/3` ne tombe pas juste en décimal
(`0.3333…`). D'où ce résultat étonnant :

```ruby
puts 0.1 + 0.2
puts 0.1 + 0.2 == 0.3
puts (0.1 + 0.2 - 0.3).abs < 1e-9
```

```sortie
0.30000000000000004
false
true
```

Ligne par ligne :

- `0.1 + 0.2` donne `0.30000000000000004` : un tout petit peu plus que `0.3` ;
- `0.1 + 0.2 == 0.3` compare les deux nombres : ils ne sont pas exactement égaux, donc
  `false` ;
- la dernière ligne calcule l'écart entre les deux, sans son signe (`abs`), et vérifie
  qu'il est plus petit que `1e-9`, c'est-à-dire 0,000000001 : c'est vrai.

Pour comparer deux flottants, regardez si leur écart est minuscule plutôt que d'utiliser
`==`.

### Rational : les fractions exactes

Un `Rational` est une vraie fraction. Il s'écrit avec un `r` à la fin.

```ruby
tiers = 1/3r
puts tiers
puts tiers * 3
puts 0.1r + 0.2r == 0.3r
puts (3/4r).to_f
```

```sortie
1/3
1/1
true
0.75
```

### BigDecimal : pour l'argent

Pour des prix, utilisez `BigDecimal`. Il calcule en décimal, comme sur un ticket de
caisse. Il faut le charger avec `require`.

```ruby
require "bigdecimal"

prix = BigDecimal("19.99")
total = prix * 3
puts total.to_s("F")
puts BigDecimal("0.1") + BigDecimal("0.2") == BigDecimal("0.3")
```

```sortie
59.97
true
```

> **À retenir :** créez un `BigDecimal` à partir d'un **texte** (`"19.99"`), pas d'un
> flottant : le flottant serait déjà imprécis. `to_s("F")` l'affiche sans notation
> scientifique.

Une autre solution simple : compter en centimes, avec des entiers (`1999` au lieu de
`19.99`).

## Le module Math

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

Pour des calculs plus savants, comme une racine carrée, inutile de tout écrire
vous-même : Ruby les fournit déjà, rangés dans une même boîte à outils.

`Math` regroupe les fonctions mathématiques.

```ruby
puts Math.sqrt(16)
puts Math::PI.round(4)
puts Math.hypot(3, 4)
puts Math.log10(1000)
puts 10.pow(3)
puts -7.abs
```

```sortie
4.0
3.1416
5.0
3.0
1000
7
```

`Math.sqrt` est la racine carrée, `Math.hypot(3, 4)` l'hypoténuse d'un triangle
rectangle. `abs` donne la valeur sans le signe.

## Le hasard avec rand

Un jeu de dés, un quiz qui mélange ses questions, un tirage au sort : tous ont besoin
de hasard.

`rand(1..6)` tire un nombre entre 1 et 6, comme un dé. Le résultat change à chaque
lancement. Pour obtenir toujours la même suite (utile pour tester), donnez une
**graine** avec `srand` ou créez un générateur `Random`.

```ruby
de = Random.new(42)
p 5.times.map { de.rand(1..6) }

de = Random.new(42)
p 5.times.map { de.rand(1..6) }
```

```sortie
[4, 5, 3, 5, 5]
[4, 5, 3, 5, 5]
```

Ligne par ligne :

- `Random.new(42)` crée un générateur de hasard avec la graine 42, rangé dans `de` ;
- `5.times.map { de.rand(1..6) }` lance le dé 5 fois et rassemble les 5 résultats dans
  une liste, que `p` affiche ;
- les deux dernières lignes recommencent avec la même graine 42 : la liste est identique.

Même graine, même suite. `srand(42)` fait la même chose pour `rand` tout court. Sans
graine, chaque lancement donne d'autres nombres.

## Convertir

Ce qu'une personne tape au clavier, ou ce qu'on lit dans un fichier, arrive toujours
sous forme de texte. Or `"42"` n'est pas un nombre : il faut le convertir avant de
calculer avec.

```ruby
puts "42".to_i + 1
puts "3.5".to_f * 2
puts "12abc".to_i
puts "abc".to_i
puts 7.to_f
puts 7.9.to_i
puts Integer("42")
```

```sortie
43
7.0
12
0
7.0
7
42
```

Ligne par ligne :

- `"42".to_i` transforme le texte `"42"` en nombre 42, puis `+ 1` donne `43` ;
- `"3.5".to_f` donne le flottant 3.5, et `* 2` donne `7.0` ;
- `"12abc".to_i` lit `12` puis s'arrête ; `"abc".to_i` ne trouve aucun chiffre : `0` ;
- `7.to_f` donne `7.0`, et `7.9.to_i` coupe la virgule sans arrondir : `7`.

`to_i` est tolérant : il lit le début du texte et renvoie `0` s'il ne trouve rien.
`Integer("...")` est strict : il lève une erreur si le texte n'est pas un nombre. C'est
mieux pour vérifier une saisie.

```ruby
begin
  Integer("12abc")
rescue ArgumentError => e
  puts "Saisie invalide : #{e.message}"
end
```

```sortie
Saisie invalide : invalid value for Integer(): "12abc"
```

**À vous :** écrivez un programme qui range les textes `"12"` et `"30"` dans deux
variables, puis affiche leur somme, calculée comme des nombres.

> **Solution**
>
> ```ruby
> a = "12"
> b = "30"
> puts a.to_i + b.to_i
> ```
>
> ```sortie
> 42
> ```

### Les chiffres d'un nombre

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

`digits` donne les chiffres, **en commençant par la fin**.

```ruby
p 1234.digits
puts 1234.digits.sum
puts 255.to_s(2)
puts "ff".to_i(16)
```

```sortie
[4, 3, 2, 1]
10
11111111
255
```

`to_s(2)` écrit le nombre en binaire, `to_i(16)` lit un nombre écrit en hexadécimal.

## Formater un nombre

Un résultat comme `3.14159` est trop long pour un ticket de caisse ou un tableau.
Formater, c'est choisir comment le nombre s'affiche, sans changer sa valeur.

`format` construit un texte à partir d'un modèle. `%.2f` veut dire « un flottant avec
2 chiffres après la virgule », `%05d` « un entier sur 5 chiffres, complété par des 0 ».

```ruby
puts format("%.2f", 3.14159)
puts format("%05d", 42)
puts format("%-6s|", "ab")
puts format("%d %% de réussite", 87)
```

```sortie
3.14
00042
ab    |
87 % de réussite
```

Ligne par ligne :

- `%.2f` garde 2 chiffres après la virgule : `3.14` ;
- `%05d` écrit 42 sur 5 chiffres, avec des 0 devant ;
- `%-6s` écrit le texte `"ab"` sur 6 caractères, collé à gauche : les espaces se voient
  avant le `|` ;
- `%%` affiche un vrai signe `%`.

**À vous :** écrivez un programme qui affiche le prix `7.5` avec 2 chiffres après la
virgule, puis le numéro `7` sur 3 chiffres, complété par des 0.

> **Solution**
>
> ```ruby
> puts format("%.2f", 7.5)
> puts format("%03d", 7)
> ```
>
> ```sortie
> 7.50
> 007
> ```

### Séparer les milliers

Ruby n'a pas de méthode toute faite pour écrire `1 234 567`. Une petite expression
régulière le fait :

```ruby
def milliers(n)
  n.to_s.reverse.scan(/\d{1,3}/).join(" ").reverse
end

puts milliers(1234567)
puts milliers(999)
puts format("%.2f €", 1234.5).sub(".", ",")
```

```sortie
1 234 567
999
1234,50 €
```

On retourne le texte, on le coupe par paquets de 3, on recolle avec des espaces, puis on
le retourne à nouveau.

## Erreurs fréquentes

### La division qui perd la virgule

```ruby norun
moyenne = (12 + 15) / 2
puts moyenne
```

```text
13
```

Pas d'erreur, mais le résultat est faux : la vraie moyenne est `13.5`. Les deux nombres
sont des entiers, donc la division est entière. Écrivez `(12 + 15) / 2.0` ou
`(12 + 15).fdiv(2)`.

### Diviser un entier par zéro

```ruby norun
puts 10 / 0
```

```text
main.rb:1:in `/': divided by 0 (ZeroDivisionError)
	from main.rb:1:in `<main>'
```

Vérifiez le diviseur avant de diviser. Notez que `10 / 0.0` ne plante pas : il donne
`Infinity`, ce qui n'est souvent pas mieux.

### Additionner un texte et un nombre

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

`"30"` est un texte. Convertissez-le d'abord : `age.to_i + 1`.

## Testez-vous

**Question 1 :** que valent `17 / 5` et `17 % 5` ?

> **Réponse**
>
> `3` et `2` : 17 = 5 × 3 + 2.
>
> ```ruby
> p 17.divmod(5)
> ```
>
> ```sortie
> [3, 2]
> ```

**Question 2 :** comment afficher `2.5` arrondi à l'entier supérieur ?

> **Réponse**
>
> Avec `ceil`.
>
> ```ruby
> puts 2.5.ceil
> ```
>
> ```sortie
> 3
> ```

**Question 3 :** pourquoi utiliser `BigDecimal("0.1")` plutôt que `0.1` pour un prix ?

> **Réponse**
>
> Parce que `0.1` est un `Float`, rangé en binaire de façon approchée. Les petites
> erreurs s'accumulent (`0.1 + 0.2` donne `0.30000000000000004`). `BigDecimal` calcule
> en décimal, exactement.

## En résumé

- `Integer` pour les entiers, `Float` pour les nombres à virgule.
- Entre deux entiers, `/` jette la virgule : utilisez `2.0` ou `fdiv`.
- `round`, `floor`, `ceil` arrondissent ; `format("%.2f", x)` affiche proprement.
- Les flottants sont approchés : `Rational` ou `BigDecimal` pour les calculs exacts.
- `Integer("...")` convertit en vérifiant ; `to_i` convertit sans se plaindre.
- `Random.new(graine)` donne un hasard reproductible.

Suite : [Conditions et boucles](conditions-boucles.md)
