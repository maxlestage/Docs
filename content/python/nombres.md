# Nombres et calculs

Cette page explique comment Python calcule : les opérateurs, les entiers et les flottants, les conversions, les modules `math` et `random`, et l'affichage soigné des nombres.

> **Objectifs :**
>
> - vous saurez utiliser les opérateurs, dont `/`, `//`, `%` et `**` ;
> - vous saurez pourquoi `0.1 + 0.2` ne vaut pas exactement `0.3`, et quoi faire ;
> - vous saurez convertir un texte en nombre, et l'inverse ;
> - vous saurez utiliser les modules `math` et `random` ;
> - vous saurez afficher un nombre avec 2 décimales ou des séparateurs de milliers.

## Les opérateurs de base

Un programme passe son temps à calculer : le total d'un panier, l'âge d'une personne, le
score d'un joueur. Avant tout le reste, il faut donc savoir écrire un calcul.

Un [opérateur](../debuter/glossaire.md#operateur) est un symbole qui fait un calcul.
Python sait faire tout ce que fait une calculatrice.

| Opérateur | Sens | Exemple | Résultat |
| --- | --- | --- | --- |
| `+` | addition | `7 + 2` | `9` |
| `-` | soustraction | `7 - 2` | `5` |
| `*` | multiplication | `7 * 2` | `14` |
| `/` | division | `7 / 2` | `3.5` |
| `//` | division entière | `7 // 2` | `3` |
| `%` | reste (modulo) | `7 % 2` | `1` |
| `**` | puissance | `7 ** 2` | `49` |

```python
print(7 + 2, 7 - 2, 7 * 2)
print(7 / 2, 7 // 2, 7 % 2)
print(7 ** 2, 2 ** 10)
```

```sortie
9 5 14
3.5 3 1
49 1024
```

Ligne par ligne :

- chaque `print` affiche plusieurs résultats, séparés par des virgules dans le code et
  par des espaces à l'écran ;
- la première ligne fait les trois calculs de l'école : `9`, `5` et `14` ;
- la deuxième montre trois façons de diviser 7 par 2 : la section suivante les détaille ;
- la troisième calcule des puissances : `7 ** 2` veut dire 7 × 7, et `2 ** 10` veut dire
  2 multiplié 10 fois par lui-même.

## Trois divisions : /, // et %

Imaginez que vous partagez 17 bonbons entre 5 enfants.

- `/` est la division « exacte » : `17 / 5` vaut `3.4`. Elle donne **toujours** un
  `float`, même quand le compte est rond.
- `//` est la division **entière** : chaque enfant reçoit `17 // 5`, soit `3` bonbons.
- `%` est le **reste** : il reste `17 % 5`, soit `2` bonbons dans le sac.

```python
bonbons = 17
enfants = 5
print(bonbons / enfants)
print(bonbons // enfants)
print(bonbons % enfants)
print(10 / 2)
```

```sortie
3.4
3
2
5.0
```

Ligne par ligne :

- `bonbons` vaut 17 et `enfants` vaut 5 ;
- `bonbons / enfants` fait le partage « exact » : `3.4` bonbons chacun, ce qui n'a pas
  beaucoup de sens pour des bonbons ;
- `bonbons // enfants` compte les bonbons entiers par enfant : `3` ;
- `bonbons % enfants` compte ce qui reste dans le sac : `2`. En effet, 3 × 5 = 15, et
  17 - 15 = 2.

Remarquez `10 / 2` qui donne `5.0` et non `5` : le `/` renvoie un `float`.

Le reste `%` est très utile pour savoir si un nombre est pair : un nombre pair a un
reste de 0 quand on le divise par 2.

```python
print(14 % 2 == 0)
print(15 % 2 == 0)
```

```sortie
True
False
```

> **Astuce :** pour convertir des secondes en minutes et secondes, combinez `//` et
> `%` : `125 // 60` donne `2` minutes, `125 % 60` donne `5` secondes.

**À vous :** vous rangez 50 œufs dans des boîtes de 12. Écrivez un programme qui range
50 dans une variable `oeufs`, puis affiche le nombre de boîtes pleines et le nombre
d'œufs qui restent.

> **Solution**
>
> ```python
> oeufs = 50
> print(oeufs // 12)
> print(oeufs % 12)
> ```
>
> ```sortie
> 4
> 2
> ```

## L'ordre des opérations

Un même calcul peut se lire de deux façons : `2 + 3 * 4`, est-ce 20 ou 14 ? Pour qu'il n'y
ait jamais de doute, Python suit des règles fixes, les mêmes qu'en classe.

Python suit les règles des maths : d'abord `**`, puis `*`, `/`, `//`, `%`, puis `+` et
`-`. À priorité égale, il calcule de gauche à droite.

```python
print(2 + 3 * 4)
print((2 + 3) * 4)
print(2 * 3 ** 2)
print(10 - 4 - 3)
```

```sortie
14
20
18
3
```

Ligne par ligne :

- `2 + 3 * 4` : la multiplication passe avant, donc 3 × 4 = 12, puis 2 + 12 = 14 ;
- `(2 + 3) * 4` : les parenthèses passent en premier, donc 5, puis 5 × 4 = 20 ;
- `2 * 3 ** 2` : la puissance passe avant tout, donc 3 × 3 = 9, puis 2 × 9 = 18 ;
- `10 - 4 - 3` : de gauche à droite, donc 10 - 4 = 6, puis 6 - 3 = 3.

> **À retenir :** en cas de doute, mettez des parenthèses. Elles ne coûtent rien et
> rendent le calcul évident pour le lecteur.

## Des entiers sans limite

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

Dans beaucoup de langages, un entier a une taille maximale. Pas en Python : un `int`
grandit autant que nécessaire. Seule la mémoire de l'ordinateur le limite.

```python
print(2 ** 100)
print(1_000_000 * 1_000_000)
```

```sortie
1267650600228229401496703205376
1000000000000
```

Le tiret bas `_` dans un nombre sert seulement à le rendre lisible : `1_000_000` est
le même nombre que `1000000`.

## Les flottants sont approximatifs

Un nombre à virgule, comme un prix ou une température, s'appelle un flottant. Il faut
connaître sa petite faiblesse, sinon un calcul « évident » vous surprendra un jour.

Un [flottant](../debuter/glossaire.md#flottant) est stocké en
[binaire](../debuter/glossaire.md#binaire). Certains nombres simples en décimal, comme
`0.1`, ne tombent pas juste en binaire. C'est comme `1/3` en décimal : `0.3333...` ne
s'arrête jamais.

```python
print(0.1 + 0.2)
print(0.1 + 0.2 == 0.3)
```

```sortie
0.30000000000000004
False
```

Ce n'est pas un bug de Python : tous les langages font pareil. Deux solutions simples :

- **arrondir** pour l'affichage, avec `round(nombre, décimales)` ;
- **comparer avec une tolérance**, avec `math.isclose`.

```python
import math

total = 0.1 + 0.2
print(round(total, 2))
print(math.isclose(total, 0.3))
```

```sortie
0.3
True
```

Ligne par ligne :

- `import math` ouvre la boîte à outils `math`, présentée plus bas ;
- `total` reçoit `0.1 + 0.2`, c'est-à-dire `0.30000000000000004` ;
- `round(total, 2)` arrondit à 2 chiffres après la virgule : on obtient `0.3` ;
- `math.isclose(total, 0.3)` demande « ces deux nombres sont-ils presque égaux ? » :
  la réponse est `True`.

> **Attention :** `round` arrondit « au pair le plus proche » quand on tombe pile au
> milieu : `round(2.5)` donne `2`, et `round(3.5)` donne `4`. C'est l'arrondi des
> banquiers, qui évite de toujours arrondir vers le haut.

```python
print(round(2.5), round(3.5), round(2.567, 1))
```

```sortie
2 4 2.6
```

## Decimal et Fraction : des calculs exacts

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

Pour de l'argent, une petite erreur d'arrondi peut poser problème. Le module `decimal`
calcule en **décimal**, comme sur papier. Créez les nombres à partir de **textes** :

```python
from decimal import Decimal

print(Decimal("0.1") + Decimal("0.2"))
prix = Decimal("19.99")
print(prix * 3)
```

```sortie
0.3
59.97
```

Le module `fractions` calcule avec des fractions exactes, comme en cours de maths :

```python
from fractions import Fraction

tiers = Fraction(1, 3)
print(tiers + tiers)
print(tiers * 3)
print(Fraction(3, 4) + Fraction(1, 6))
```

```sortie
2/3
1
11/12
```

La fraction est toujours simplifiée : `1/3 * 3` donne bien `1`.

> **Note :** `Decimal` et `Fraction` sont plus lents que les `float`. Utilisez-les
> quand l'exactitude compte vraiment : les montants, les calculs de précision.

## Le module math

Un [module](../debuter/glossaire.md#module) est une boîte à outils. Le module `math`
contient les fonctions mathématiques. On l'ouvre avec `import math`, puis on écrit
`math.` devant chaque outil.

```python
import math

print(math.sqrt(16))
print(math.pi)
print(math.floor(3.7), math.ceil(3.2))
print(math.gcd(12, 18))
print(abs(-5), max(3, 9, 4), min(3, 9, 4))
```

```sortie
4.0
3.141592653589793
3 4
6
5 9 3
```

- `sqrt` : racine carrée (*square root*) ;
- `floor` arrondit vers le bas (le plancher), `ceil` vers le haut (le plafond) ;
- `gcd` : le plus grand diviseur commun ;
- `abs`, `max` et `min` n'ont pas besoin d'`import` : ils sont toujours disponibles.

## Le module random

Le module `random` tire des nombres au hasard : parfait pour un dé ou un jeu. Mais un
programme dont la sortie change à chaque fois est difficile à tester. `random.seed(n)`
fixe le point de départ du hasard : avec la même graine, on obtient toujours les mêmes
tirages.

```python
import random

random.seed(42)
print(random.randint(1, 6))
print(random.randint(1, 6))
print(random.choice(["pierre", "feuille", "ciseaux"]))
```

```sortie
6
1
pierre
```

Ligne par ligne :

- `import random` ouvre la boîte à outils du hasard ;
- `random.seed(42)` fixe la graine : les tirages qui suivent seront toujours les mêmes ;
- les deux `randint(1, 6)` lancent un dé deux fois : `6`, puis `1` ;
- `choice` pioche un mot dans la liste : ici, `"pierre"`.

- `randint(1, 6)` : un entier entre 1 et 6, **bornes comprises** ;
- `choice(liste)` : un élément de la liste ;
- `random()` : un flottant entre 0 et 1.

> **Astuce :** retirez `random.seed(42)` dans un vrai jeu, sinon le dé donnera toujours
> la même suite. Gardez-le pour les tests.

## Convertir entre texte et nombres

Un nombre écrit entre guillemets, comme `"42"`, est un texte : Python ne peut pas faire
de calcul avec. C'est souvent le cas des données tapées au clavier ou lues dans un
fichier. Il faut alors les convertir, comme on change des euros en dollars avant de
payer.

Trois fonctions convertissent une valeur d'un type à l'autre :

- `int(x)` : en entier ;
- `float(x)` : en flottant ;
- `str(x)` : en texte.

```python
texte = "42"
nombre = int(texte)
print(nombre + 1)
print(float("3.5") * 2)
print(str(100) + " points")
print(int(7.9))
```

```sortie
43
7.0
100 points
7
```

Ligne par ligne :

- `texte` contient le texte `"42"`, pas le nombre 42 ;
- `int(texte)` le transforme en nombre : `nombre` vaut 42, et `nombre + 1` donne `43` ;
- `float("3.5")` donne le nombre `3.5`, et `3.5 * 2` donne `7.0` ;
- `str(100)` donne le texte `"100"`, qu'on peut coller à `" points"` avec `+`.

Attention à `int(7.9)` : il **coupe** la partie décimale, il n'arrondit pas. Pour
arrondir, utilisez `round(7.9)`, qui donne `8`.

**À vous :** la variable `age` contient le texte `"17"`. Écrivez un programme qui
affiche « L'an prochain, vous aurez 18 ans. » en calculant 18 à partir de `age`.

> **Solution**
>
> ```python
> age = "17"
> print("L'an prochain, vous aurez " + str(int(age) + 1) + " ans.")
> ```
>
> ```sortie
> L'an prochain, vous aurez 18 ans.
> ```

## Afficher joliment les nombres

Un prix affiché `3.14159 €` ou une population écrite `68373433` sont difficiles à lire.
Un ticket de caisse montre toujours 2 chiffres après la virgule : les formats servent à
cela.

Dans une f-string, ajoutez `:` puis un **format** après la valeur :

- `:.2f` : 2 chiffres après la virgule (*f* pour *fixed*, virgule fixe) ;
- `:,` : un séparateur de milliers ;
- `:%` : un pourcentage ;
- `:5` : une largeur de 5 caractères.

```python
prix = 3.14159
population = 68373433
taux = 0.256

print(f"{prix:.2f}")
print(f"{population:,}")
print(f"{population:_}")
print(f"{taux:.1%}")
print(f"[{42:5}]")
print(f"{1234567.891:,.2f}")
```

```sortie
3.14
68,373,433
68_373_433
25.6%
[   42]
1,234,567.89
```

> **Note :** le format `,` utilise la virgule anglaise. Pour un affichage à la
> française (`68 373 433`), remplacez-la : `f"{population:,}".replace(",", " ")`.

**À vous :** un article coûte `12.5` euros et vous en achetez 3. Écrivez un programme
qui affiche « Total : 37.50 € », avec 2 chiffres après la virgule.

> **Solution**
>
> ```python
> prix = 12.5
> quantite = 3
> print(f"Total : {prix * quantite:.2f} €")
> ```
>
> ```sortie
> Total : 37.50 €
> ```

## Erreurs fréquentes

### Convertir un texte à virgule avec int

```python norun
n = int("3.5")
```

```text
Traceback (most recent call last):
  File "/home/vous/main.py", line 1, in <module>
    n = int("3.5")
ValueError: invalid literal for int() with base 10: '3.5'
```

`int` n'accepte qu'un texte qui contient un entier. Pour `"3.5"`, utilisez
`float("3.5")`. Et pour l'entier : `int(float("3.5"))`.

### Diviser par zéro

```python norun
print(10 / 0)
```

```text
Traceback (most recent call last):
  File "/home/vous/main.py", line 1, in <module>
    print(10 / 0)
          ~~~^~~
ZeroDivisionError: division by zero
```

Diviser par zéro n'a pas de sens. Vérifiez le diviseur avant :
`if diviseur != 0:`. Les conditions sont expliquées dans
[conditions et boucles](conditions-boucles.md).

### Oublier import math

```python norun
print(math.sqrt(16))
```

```text
Traceback (most recent call last):
  File "/home/vous/main.py", line 1, in <module>
    print(math.sqrt(16))
          ^^^^
NameError: name 'math' is not defined. Did you forget to import 'math'?
```

Python ne connaît `math` qu'après `import math`. Ajoutez cette ligne en haut du
fichier.

### Comparer deux flottants avec ==

```python norun
print(0.1 * 3 == 0.3)
```

```text
False
```

Pas d'erreur, mais un résultat surprenant : `0.1 * 3` vaut `0.30000000000000004`.
Comparez avec `math.isclose(0.1 * 3, 0.3)`, qui répond `True`.

## Testez-vous

**Question 1 :** que valent `20 / 4`, `20 // 6` et `20 % 6` ?

> **Réponse**
>
> `5.0` (un `float`, toujours avec `/`), `3` (division entière) et `2` (le reste :
> 20 = 3 × 6 + 2).

**Question 2 :** convertissez 200 secondes en minutes et secondes, et affichez
`3 min 20 s`.

> **Réponse**
>
> ```python
> secondes = 200
> print(f"{secondes // 60} min {secondes % 60} s")
> ```
>
> ```sortie
> 3 min 20 s
> ```

**Question 3 :** comment afficher `1/3` avec 3 décimales ?

> **Réponse**
>
> ```python
> print(f"{1 / 3:.3f}")
> ```
>
> ```sortie
> 0.333
> ```

## En résumé

- `/` donne toujours un `float` ; `//` donne la division entière et `%` le reste.
- `**` est la puissance ; les parenthèses fixent l'ordre des calculs.
- Les `int` n'ont pas de limite ; les `float` sont approximatifs : arrondissez avec
  `round` ou comparez avec `math.isclose`.
- `Decimal` et `Fraction` calculent de façon exacte.
- `int()`, `float()` et `str()` convertissent ; `import math` et `import random`
  ouvrent des boîtes à outils.
- `f"{x:.2f}"` et `f"{n:,}"` affichent les nombres proprement.

Suite : [conditions et boucles](conditions-boucles.md).
