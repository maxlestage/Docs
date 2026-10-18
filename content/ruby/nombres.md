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

Le `_` sert seulement à rendre le nombre lisible, comme l'espace dans « 1 000 000 ».
Un entier Ruby n'a pas de limite de taille : `2**100` marche sans problème.

## Les opérateurs

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

Dès qu'un des deux nombres est un `Float`, le résultat est un `Float`. `fdiv` force une
division à virgule, et `divmod` donne le quotient et le reste d'un coup.

> **Attention :** `-7 / 2` vaut `-4`, pas `-3` : Ruby arrondit toujours vers le bas.

## Arrondir

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

- `round` arrondit au plus proche ; `round(1)` garde un chiffre après la virgule.
- `floor` arrondit vers le bas (le « plancher »), `ceil` vers le haut (le « plafond »).
- `truncate` coupe la partie après la virgule.
- Un nombre négatif dans `round` arrondit aux dizaines, centaines…

## Le piège des flottants

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

Même graine, même suite. `srand(42)` fait la même chose pour `rand` tout court. Sans
graine, chaque lancement donne d'autres nombres.

## Convertir

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

### Les chiffres d'un nombre

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
