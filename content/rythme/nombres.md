# Nombres et calculs

Cette page montre les deux sortes de nombres de Rythme, les entiers et les décimaux, et tout ce qu'on peut calculer avec eux.

> **Objectifs :**
>
> - vous saurez faire la différence entre un `Int` et un `Float` ;
> - vous saurez utiliser les opérateurs `+`, `-`, `*`, `/`, `//`, `%` et `**` ;
> - vous saurez arrondir, convertir et utiliser les fonctions mathématiques ;
> - vous saurez afficher un nombre joliment, avec des décimales et des milliers séparés.

## Deux sortes de nombres

Un programme manipule sans cesse des nombres : un âge, un score, un prix.
Certains se comptent un par un (3 pommes), d'autres ont des décimales
(1,5 kg de pommes).

Rythme connaît deux sortes de nombres :

- les [entiers](../debuter/glossaire.md#entier) (`Int`), sans virgule :
  `42`, `-7`, `0` ;
- les [flottants](../debuter/glossaire.md#flottant) (`Float`), les nombres
  décimaux, écrits avec un **point** : `3.14`, `2.0`, `-0.5`.

`type_of` dit de quelle sorte est un nombre :

```rythme
print(type_of(42), type_of(3.14), type_of(2.0))
print(1_000_000, 0xFF, 0b1010)
```

```sortie
Int Float Float
1000000 255 10
```

`2.0` est un Float, même s'il n'a pas de décimales. Le `_` sépare les
chiffres pour les rendre lisibles : `1_000_000` est un million. `0xFF` est
écrit en hexadécimal et `0b1010` en binaire.

> **Attention :** on écrit `3.14` avec un point, jamais `3,14`. La virgule
> sépare les valeurs : `print(3,14)` affiche `3 14`.

## Des entiers sans limite de taille

Dans beaucoup de langages, un entier a une taille maximale, et un calcul
trop grand donne un résultat faux. En Rythme, un entier grandit autant que
nécessaire, comme un ruban qu'on déroule sans fin :

```rythme
print(2 ** 100)
print(1_000_000 * 1_000_000)
fn factorielle(n) = if n == 0 then 1 else n * factorielle(n - 1)
print(factorielle(30))
```

```sortie
1267650600228229401496703205376
1000000000000
265252859812191058636308480000000
```

Ligne par ligne :

- `print(2 ** 100)` calcule 2 multiplié par lui-même 100 fois : un nombre
  de 31 chiffres ;
- la deuxième ligne multiplie un million par un million : mille milliards ;
- `fn factorielle(n) = …` définit une fonction. La factorielle de `n`, c'est
  1 × 2 × 3 × … × `n` ;
- `factorielle(30)` multiplie donc tous les nombres de 1 à 30 : le résultat
  a 33 chiffres, tous justes.

Tous les chiffres sont exacts. Les Floats, eux, sont des valeurs
**approchées** : ils gardent environ 16 chiffres.

```rythme
print(0.1 + 0.2, 0.1 + 0.2 == 0.3)
```

```sortie
0.30000000000000004 false
```

Ce n'est pas un défaut de Rythme : tous les langages calculent ainsi les
nombres décimaux. Pour comparer des Floats, arrondissez d'abord (voir plus
bas).

## Les quatre opérations

Un programme sert souvent de calculatrice : le total d'un panier, la part
de chacun au restaurant. Ces quatre signes suffisent pour la plupart des
calculs.

`+`, `-` et `*` marchent comme à l'école. Entre deux Int, le résultat est un
Int ; dès qu'il y a un Float, le résultat est un Float :

```rythme
print(7 + 2, 7 - 2, 7 * 2)
print(7 + 2.0, 3 * 1.0)
```

```sortie
9 5 14
9.0 3.0
```

La division `/` donne **toujours un Float**, même quand elle tombe juste :

```rythme
print(7 / 2, 6 / 3, 1 / 3)
```

```sortie
3.5 2.0 0.3333333333333333
```

Ainsi, `7 / 2` vaut bien `3.5`, et non `3` comme dans d'autres langages.

**À vous :** écrivez un programme qui range `prix = 4` et `quantite = 3`,
puis affiche le total à payer, et enfin ce que paie chacun si deux personnes
partagent ce total.

> **Solution**
>
> ```rythme
> prix = 4
> quantite = 3
> total = prix * quantite
> print(total)
> print(total / 2)
> ```
>
> ```sortie
> 12
> 6.0
> ```

## Division entière, reste et puissance

Pour partager des objets entiers, on veut souvent le **quotient** et le
**reste** : 17 bonbons pour 5 enfants, c'est 3 bonbons chacun, et il en
reste 2.

```rythme
print(17 // 5, 17 % 5)
print(2 ** 10, 2 ** -1, 9 ** 0.5)
```

```sortie
3 2
1024 0.5 3.0
```

| Opérateur | Nom | Exemple | Résultat |
|-----------|-----|---------|----------|
| `+` `-` `*` | addition, soustraction, multiplication | `7 * 2` | `14` |
| `/` | division, toujours un Float | `7 / 2` | `3.5` |
| `//` | division entière, arrondie vers le bas | `17 // 5` | `3` |
| `%` | reste de la division | `17 % 5` | `2` |
| `**` | puissance | `2 ** 10` | `1024` |

Les priorités sont celles des mathématiques : `**` d'abord, puis `*`, `/`,
`//`, `%`, puis `+` et `-`. Les parenthèses changent l'ordre :

```rythme
print(2 + 3 * 4, (2 + 3) * 4, -2 ** 2)
```

```sortie
14 20 -4
```

`-2 ** 2` vaut `-(2 ** 2)`, donc `-4`.

> **Astuce :** `n % 2 == 0` dit si `n` est pair. Rythme a aussi
> `is_even(n)` et `is_odd(n)`, plus lisibles.

**À vous :** vous avez 23 œufs et des boîtes de 6. Écrivez un programme qui
affiche le nombre de boîtes pleines, puis le nombre d'œufs qui restent.

> **Solution**
>
> ```rythme
> oeufs = 23
> print(oeufs // 6, oeufs % 6)
> ```
>
> ```sortie
> 3 5
> ```

## Pair ou impair

Savoir si un nombre est pair sert plus souvent qu'on ne le croit : colorier
une ligne sur deux d'un tableau, former des équipes de deux.

```rythme
print(is_even(4), is_odd(4), is_odd(7))
print(filter(1..10, is_even))
```

```sortie
true false true
[2, 4, 6, 8, 10]
```

`is_even` et `is_odd` sont des fonctions comme les autres : on peut les
donner à `filter` ou à `count`.

## Les fonctions de calcul

Certains calculs reviennent tout le temps : trouver le plus grand score,
enlever le signe d'un nombre. Au lieu de les réécrire, vous utilisez des
fonctions toutes prêtes, comme les touches spéciales d'une calculatrice.

Rythme fournit les fonctions mathématiques courantes, sans rien importer :

```rythme
print(abs(-5), abs(-2.5))
print(min(3, 7), max(3, 7.5), max([2, 9, 4]))
print(sqrt(16), sqrt(2))
print(pow(2, 8), pow(2.0, 0.5))
```

```sortie
5 2.5
3 7.5 9
4.0 1.4142135623730951
256 1.4142135623730951
```

- `abs(x)` : la valeur absolue, le nombre sans son signe ;
- `min` et `max` : le plus petit ou le plus grand, de plusieurs valeurs
  ou d'une liste ;
- `sqrt(x)` : la racine carrée, toujours un Float ;
- `pow(x, n)` : la puissance, comme `x ** n`.

## Arrondir

Un résultat comme `0.30000000000000004` est rarement utile tel quel.
Arrondir, c'est ce que vous faites à la caisse quand vous dites « environ
13 euros ».

Quatre fonctions transforment un Float en Int :

```rythme
print(round(2.5), round(2.4), round(-2.5))
print(floor(2.7), floor(-2.5), ceil(2.1))
print(int(3.99), int(-3.99))
print(round(3.14159, 2), round(0.1 + 0.2, 2) == 0.3)
```

```sortie
3 2 -3
2 -3 3
3 -3
3.14 true
```

| Fonction | Ce qu'elle fait | `2.7` | `-2.5` |
|----------|-----------------|-------|--------|
| `round(x)` | au plus proche (un `.5` s'éloigne de zéro) | `3` | `-3` |
| `floor(x)` | vers le bas, le « plancher » | `2` | `-3` |
| `ceil(x)` | vers le haut, le « plafond » | `3` | `-2` |
| `int(x)` | coupe les décimales | `2` | `-2` |

`round(x, chiffres)` garde ce nombre de décimales et donne un Float.

**À vous :** un repas coûte `12.3` €. Écrivez un programme qui affiche ce
prix arrondi à l'euro le plus proche, puis à l'euro au-dessus, puis à l'euro
au-dessous.

> **Solution**
>
> ```rythme
> prix_repas = 12.3
> print(round(prix_repas), ceil(prix_repas), floor(prix_repas))
> ```
>
> ```sortie
> 12 13 12
> ```

## Convertir

Ce que l'utilisateur tape au clavier arrive sous forme de texte : `"42"`
n'est pas encore le nombre 42. Pour calculer avec, il faut d'abord le
convertir.

Rythme ne convertit **jamais** une valeur en silence. `int` et `float`
convertissent quand vous le demandez :

```rythme
print(int("42") + 1, float("2.5") * 2, float(3), int(true))
print(parse_int("42"), parse_int("douze"))
```

```sortie
43 5.0 3.0 1
Ok(42) Err("\"douze\" n'est pas un nombre entier")
```

`int("douze")` arrête le programme. Quand un texte vient de l'utilisateur
et peut ne pas être un nombre, préférez `parse_int` : il donne un `Result`,
expliqué dans [gérer les erreurs](erreurs.md).

## PGCD et PPCM

Ces deux calculs servent pour partager ou pour se retrouver : deux bus qui
passent toutes les 4 et toutes les 6 minutes repassent ensemble toutes les
12 minutes.

`gcd(a, b)` donne le plus grand commun diviseur (PGCD), et `lcm(a, b)` le
plus petit commun multiple (PPCM) :

```rythme
print(gcd(12, 18), lcm(4, 6))
numerateur = 18
denominateur = 24
d = gcd(numerateur, denominateur)
print("{numerateur}/{denominateur} = {numerateur // d}/{denominateur // d}")
```

```sortie
6 12
18/24 = 3/4
```

Ligne par ligne :

- la première ligne affiche le PGCD de 12 et 18 (6), puis le PPCM de 4 et
  6 (12) ;
- `numerateur` et `denominateur` reçoivent 18 et 24 : la fraction 18/24 ;
- `d` reçoit 6, le plus grand nombre qui divise à la fois 18 et 24 ;
- la dernière ligne divise le haut et le bas par `d` : `18 // 6` vaut 3 et
  `24 // 6` vaut 4.

Le PGCD sert par exemple à simplifier une fraction.

## Pi et la trigonométrie

Ces fonctions servent dès qu'il y a des cercles ou des angles : calculer
une aire, dessiner, faire tourner un objet dans un jeu.

`pi` est la constante π. `sin`, `cos` et `tan` prennent un angle en
**radians**. Pour des degrés, multipliez par `pi / 180` :

```rythme
print(pi)
angle = 30 * pi / 180
print(sin(angle), round(sin(angle), 2))
print(cos(0), round(cos(60 * pi / 180), 2))
rayon = 2
print("aire du disque : {round(pi * rayon ** 2, 2)}")
```

```sortie
3.141592653589793
0.49999999999999994 0.5
1.0 0.5
aire du disque : 12.57
```

Ligne par ligne :

- `print(pi)` affiche π, avec 16 chiffres ;
- `angle` reçoit 30 degrés, convertis en radians ;
- la ligne suivante affiche le sinus de cet angle, d'abord tel quel, puis
  arrondi à 2 décimales ;
- `cos(0)` vaut `1.0`, et le cosinus de 60°, arrondi, vaut `0.5` ;
- `rayon` vaut 2 : l'aire du disque, π × 2 × 2, vaut environ 12,57.

Le sinus de 30° vaut 0,5 ; le calcul approché donne `0.49999999999999994`,
et `round` remet les choses au propre.

## Des nombres au hasard

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

Le hasard sert surtout dans les jeux : lancer un dé, choisir qui commence,
tirer une carte.

`random(a, b)` tire un entier de `a` à `b`, inclus, comme un dé.
`random()` donne un Float entre 0 et 1, et `random(liste)` un élément au
hasard. Le résultat change à chaque exécution :

```rythme norun
print(random(1, 6))
print(random(["pile", "face"]))
```

```text
4
face
```

Ce qu'on peut vérifier, c'est que le dé reste entre 1 et 6 :

```rythme
de = random(1, 6)
print(de >= 1 and de <= 6, contains(1..6, de))
```

```sortie
true true
```

## Afficher un nombre joliment

Un prix s'écrit `13.50 €`, pas `13.5 €`. Pour qu'un nombre soit agréable à
lire, vous choisissez son nombre de décimales et la façon d'écrire les
milliers.

`text(x, decimales)` écrit un nombre avec **exactement** ce nombre de
décimales. C'est parfait pour des prix :

```rythme
print(text(3.14159, 2), text(5, 2), text(1 / 3, 3))
prix = 4.5
print("Total : {text(prix * 3, 2)} €")
```

```sortie
3.14 5.00 0.333
Total : 13.50 €
```

Ligne par ligne :

- `text(3.14159, 2)` garde deux décimales : `3.14`. `text(5, 2)` en ajoute :
  `5.00` ;
- `prix` reçoit 4.5 ;
- `prix * 3` vaut 13.5, et `text(…, 2)` l'écrit `13.50`. Les accolades le
  placent dans le texte.

**À vous :** un cahier coûte `2.5` €. Écrivez un programme qui affiche
`4 cahiers : 10.00 €`, en calculant le prix avec `text`.

> **Solution**
>
> ```rythme
> prix_cahier = 2.5
> nombre = 4
> print("{nombre} cahiers : {text(prix_cahier * nombre, 2)} €")
> ```
>
> ```sortie
> 4 cahiers : 10.00 €
> ```

`format_number(x, decimales, langue)` sépare aussi les milliers. Avec
`"fr"`, on obtient l'écriture française : une espace pour les milliers, une
virgule pour les décimales.

```rythme
print(format_number(1234567.891, 2, "fr"))
print(format_number(1234567.891, 2))
print(format_number(-42000, 0, "fr"), format_number(0.5, 2, "fr"))
```

```sortie
1 234 567,89
1,234,567.89
-42 000 0,50
```

Sans langue, c'est l'écriture anglaise : une virgule pour les milliers et
un point pour les décimales.

## Erreurs fréquentes

### Ajouter un nombre à un texte

```rythme norun
age = "36"
print(age + 1)
```

```text
erreur[E005]: impossible d'utiliser `+` avec Text et Int
 --> main.ry:2:11
  |
2 | print(age + 1)
  |           ^ Text + Int 1
  |
  = aide: pour construire un texte contenant une valeur, utilisez des accolades : "total : {x}"
      ou convertissez d'abord la valeur : text(x)
  = en savoir plus : `rythme explain E005`
```

`"36"` est un texte, pas un nombre. Convertissez-le : `int(age) + 1` vaut
`37`.

### Utiliser le résultat de / comme position

```rythme norun
xs = ["a", "b", "c", "d"]
milieu = len(xs) / 2
print(xs[milieu])
```

```text
erreur[E005]: un indice doit être un nombre entier (Int), mais a reçu une valeur de type Float
 --> main.ry:3:10
  |
3 | print(xs[milieu])
  |          ^^^^^^
  |
  = en savoir plus : `rythme explain E005`
```

`/` donne toujours un Float : `4 / 2` vaut `2.0`. Pour une position,
utilisez la division entière : `milieu = len(xs) // 2`.

### Diviser par zéro

```rythme norun
notes = []
print(sum(notes) / len(notes))
```

```text
erreur[E008]: division par zéro
 --> main.ry:2:18
  |
2 | print(sum(notes) / len(notes))
  |                  ^
  |
  = aide: vérifiez d'abord le diviseur : `if b == 0 then ... else a / b`,
      ou renvoyez un Result : `if b == 0 then Err("division par zéro") else Ok(a / b)`
  = en savoir plus : `rythme explain E008`
```

Une liste vide a 0 élément. Testez-la d'abord :
`if is_empty(notes) then 0 else sum(notes) / len(notes)`.

## Testez-vous

**Question 1 :** que valent `7 / 2`, `7 // 2` et `7 % 2` ?

> **Réponse**
>
> `3.5`, `3` et `1`. `/` donne toujours un Float, `//` le quotient entier,
> `%` le reste.

**Question 2 :** que donnent `round(2.5)`, `floor(2.5)` et `int(-2.5)` ?

> **Réponse**
>
> `3`, `2` et `-2`. `round` va au plus proche (un `.5` s'éloigne de zéro),
> `floor` vers le bas, `int` coupe les décimales.

**Question 3 :** comment afficher `1234.5` sous la forme `1 234,50` ?

> **Réponse**
>
> `format_number(1234.5, 2, "fr")` : deux décimales, écriture française.

## En résumé

- `Int` pour les entiers, sans limite de taille ; `Float` pour les
  décimaux, écrits avec un point, et approchés.
- `/` donne toujours un Float ; `//` et `%` donnent le quotient entier et
  le reste ; `**` la puissance.
- `round`, `floor`, `ceil` et `int` arrondissent ; `int`, `float` et
  `parse_int` convertissent.
- `abs`, `min`, `max`, `sqrt`, `pow`, `gcd`, `lcm`, `pi`, `sin`, `cos`
  sont toujours disponibles.
- `text(x, 2)` et `format_number(x, 2, "fr")` affichent un nombre
  joliment.

Suite : [Conditions et répétition](conditions-boucles.md)
