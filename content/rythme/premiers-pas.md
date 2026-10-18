# Premiers pas

Cette page présente les bases de Rythme : donner un nom à une valeur, les sortes de valeurs, les calculs, les textes, l'affichage et les commentaires.

> **Objectifs :**
>
> - vous saurez donner un nom à une valeur, et savoir pourquoi ce nom ne change plus ;
> - vous saurez reconnaître les sortes de valeurs simples : `Int`, `Float`, `Text`, `Bool` ;
> - vous saurez calculer avec `+`, `-`, `*`, `/`, `//`, `%` et `**` ;
> - vous saurez insérer une valeur dans un texte et l'afficher ;
> - vous saurez écrire un commentaire.

## Donner un nom à une valeur

Une [valeur](../debuter/glossaire.md#valeur) est une donnée : un nombre, un
texte... Pour la retrouver plus tard, on lui donne un **nom**. On écrit le
nom, `=`, puis la valeur :

```rythme
nom = "Ada"
naissance = 1815
print(nom, naissance)
```

```sortie
Ada 1815
```

Imaginez une étiquette collée sur une boîte : `nom` est l'étiquette, `"Ada"`
est ce qu'il y a dans la boîte. Ailleurs, on appelle cela une
[variable](../debuter/glossaire.md#variable).

`print` affiche ses valeurs séparées par une espace, puis passe à la ligne.

## Les noms ne changent jamais

En Rythme, une valeur ne change jamais : elle est
[immuable](../debuter/glossaire.md#immuable). Un nom désigne une seule
valeur, pour toujours. Si vous voulez une autre valeur, donnez-lui un
nouveau nom :

```rythme
total = 10
total2 = total + 5
print(total, total2)
```

```sortie
10 15
```

L'étiquette est collée pour de bon : on ne la décolle pas, on en colle une
nouvelle sur une nouvelle boîte. C'est plus simple à suivre : quand vous
lisez `total`, vous savez qu'il vaut 10 partout dans le programme.

> **À retenir :** pas de `x = x + 1`, pas de `x += 1`. Une nouvelle valeur
> reçoit un nouveau nom. Rythme refuse les deux (voir
> [les erreurs fréquentes](#erreurs-frequentes)).

## Les sortes de valeurs

Chaque valeur a une sorte, son [type](../debuter/glossaire.md#type). Voici
les quatre plus simples :

| Sorte | Exemples | Pour quoi |
|-------|----------|-----------|
| `Int` | `42`, `-7`, `1_000_000` | les nombres [entiers](../debuter/glossaire.md#entier) |
| `Float` | `3.14`, `2.0` | les nombres à virgule, ou [flottants](../debuter/glossaire.md#flottant) |
| `Text` | `"bonjour"` | les textes, ou [chaînes de caractères](../debuter/glossaire.md#chaine-de-caracteres) |
| `Bool` | `true`, `false` | vrai ou faux, les [booléens](../debuter/glossaire.md#booleen) |

Le `_` dans `1_000_000` sert seulement à lire plus facilement. Un nombre à
virgule s'écrit avec un **point** : `3.14`, pas `3,14`.

`type_of` donne la sorte d'une valeur :

```rythme
print(type_of(42), type_of(3.14), type_of("salut"), type_of(true))
```

```sortie
Int Float Text Bool
```

Un `Float` s'affiche toujours avec un point, même quand il est rond : `2.0`.
On ne le confond donc jamais avec l'`Int` `2`.

## Calculer

Voici les **opérateurs** de calcul :

| Opérateur | Veut dire | Exemple | Résultat |
|-----------|-----------|---------|----------|
| `+` | plus | `7 + 2` | `9` |
| `-` | moins | `7 - 2` | `5` |
| `*` | fois | `7 * 2` | `14` |
| `/` | divisé par | `7 / 2` | `3.5` |
| `//` | division entière | `7 // 2` | `3` |
| `%` | reste de la division | `7 % 2` | `1` |
| `**` | puissance | `7 ** 2` | `49` |

```rythme
print(2 + 3 * 4, (2 + 3) * 4)
print(7 / 2, 6 / 3, 7 // 2, 7 % 3, 2 ** 10)
```

```sortie
14 20
3.5 2.0 3 1 1024
```

Quelques règles à connaître :

- `*`, `/`, `//` et `%` passent avant `+` et `-`, comme à l'école. Les
  parenthèses changent l'ordre.
- `/` donne **toujours** un `Float`, même quand la division tombe juste :
  `6 / 3` vaut `2.0`.
- `//` donne la **division entière** : combien de fois 2 entre dans 7 ?
  3 fois. `%` donne ce qui reste : 7 = 3 × 2 + **1**.
- `%` sert souvent à savoir si un nombre est pair : `n % 2 == 0`.

Pensez à des bonbons : 7 bonbons pour 2 enfants, chacun en a `7 // 2`,
donc 3, et il en reste `7 % 2`, donc 1.

Les nombres entiers n'ont pas de limite de taille :

```rythme
print(2 ** 100)
```

```sortie
1267650600228229401496703205376
```

Pour arrondir ou convertir un nombre :

```rythme
print(round(3.7), round(3.14159, 2), int(3.9), float(2))
```

```sortie
4 3.14 3 2.0
```

`round(x)` arrondit à l'entier le plus proche, `round(x, 2)` garde deux
décimales, `int` coupe les décimales, `float` donne un nombre à virgule.

## Les textes et l'insertion

Un texte s'écrit entre guillemets droits `"`. Pour mettre une valeur dans un
texte, écrivez-la entre accolades `{...}` : c'est l'**insertion**.

```rythme
nom = "Ada"
age = 36
print("{nom} a {age} ans.")
print("Dans dix ans : {age + 10} ans.")
```

```sortie
Ada a 36 ans.
Dans dix ans : 46 ans.
```

Entre les accolades, on peut écrire un calcul, pas seulement un nom. Comme
en Ruby, `#{...}` marche aussi :

```rythme
nom = "Grace"
print("Bonjour, #{nom} !")
```

```sortie
Bonjour, Grace !
```

`+` met deux textes bout à bout. Mais Rythme ne mélange jamais texte et
nombre en silence : `"âge : " + 36` est une erreur. Écrivez `"âge : {36}"`,
ou convertissez d'abord le nombre en texte avec `text(36)`.

```rythme
print("Bon" + "jour", "âge : " + text(36))
```

```sortie
Bonjour âge : 36
```

Quelques fonctions utiles sur les textes (la page
[travailler avec du texte](texte.md) en montre bien d'autres) :

```rythme
print(upper("salut"), len("été"), trim("  ok  "), "a,b,c" |> split(","))
```

```sortie
SALUT 3 ok ["a", "b", "c"]
```

`upper` met en majuscules, `len` compte les caractères, `trim` enlève les
espaces autour, `split` découpe en morceaux.

## Afficher avec print et puts

`print` et `puts` sont la même fonction. `puts` est le nom de Ruby. Au début
d'une ligne, vous pouvez même oublier les parenthèses :

```rythme
print("avec des parenthèses")
puts "sans parenthèses"
puts "deux valeurs :", 1 + 1
```

```sortie
avec des parenthèses
sans parenthèses
deux valeurs : 2
```

Les valeurs données à `print` sont séparées par des virgules. Elles
s'affichent séparées par une espace.

## Les commentaires

`#` commence un **commentaire**, jusqu'à la fin de la ligne. Rythme
l'ignore : c'est une note pour les humains, vous compris dans trois mois.

```rythme
# Un commentaire ordinaire.
prix = 12  # un commentaire après du code
print(prix * 2)
```

```sortie
24
```

Vous croiserez aussi `##`, deux dièses : c'est un commentaire qui décrit la
fonction écrite juste en dessous (voir
[documenter avec rythme doc](projet.md#documenter-avec-rythme-doc)).

## Des accents dans les noms

Un nom commence par une lettre ou `_`, puis contient des lettres, des
chiffres et des `_`. Les accents sont permis :

```rythme
prénom = "Élise"
âge = 28
print("{prénom} a {âge} ans.")
```

```sortie
Élise a 28 ans.
```

Majuscules et minuscules comptent : `age` et `Age` sont deux noms
différents. Un nom qui commence par une majuscule est réservé aux types et
à leurs variantes (voir [vos propres types](types.md)).

## Erreurs fréquentes

### Changer la valeur d'un nom

Dans beaucoup de langages, on écrit `x = x + 1`. Pas en Rythme :

```rythme norun
total = 1
total = total + 1
print(total)
```

```text
erreur[E011]: `total` est déjà défini
 --> main.ry:2:1
  |
2 | total = total + 1
  | ^^^^^ défini à nouveau ici
  |
1 | total = 1
  | ----- défini une première fois ici
  |
  = aide: les valeurs ne changent jamais en Rythme : donnez un nouveau nom à la nouvelle valeur, comme `total2 = total + 1`
  = en savoir plus : `rythme explain E011`
```

L'erreur est trouvée avant même de lancer le programme. Donnez un nouveau
nom : `total2 = total + 1`.

### Utiliser += pour ajouter

```rythme norun
score = 0
score += 10
```

```text
erreur[E011]: les valeurs ne peuvent pas être modifiées en Rythme
 --> main.ry:2:7
  |
2 | score += 10
  |       ^^
  |
  = aide: les valeurs ne changent jamais en Rythme : donnez un nouveau nom à la nouvelle valeur, comme `total2 = total + 1`
  = en savoir plus : `rythme explain E011`
```

`+=` modifierait `score`. Écrivez plutôt `score2 = score + 10`.

### Ajouter un nombre à un texte

```rythme norun
age = 36
print("âge : " + age)
```

```text
erreur[E005]: impossible d'utiliser `+` avec Text et Int
 --> main.ry:2:16
  |
2 | print("âge : " + age)
  |                ^ Text "âge : " + Int
  |
  = aide: pour construire un texte contenant une valeur, utilisez des accolades : "total : {x}"
      ou convertissez d'abord la valeur : text(x)
  = en savoir plus : `rythme explain E005`
```

Rythme ne devine pas si vous voulez un texte ou un nombre. Écrivez
`print("âge : {age}")`.

### Écrire let ou var devant un nom

```rythme norun
let age = 36
print(age)
```

```text
erreur[E001]: `let` n'est pas utilisé en Rythme
 --> main.ry:1:1
  |
1 | let age = 36
  | ^^^ vient d'un autre langage
  |
  = aide: écrivez simplement `nom = valeur` (les valeurs ne changent jamais, aucun mot-clé n'est nécessaire)
  = en savoir plus : `rythme explain E001`
```

Aucun mot n'est nécessaire : `age = 36`.

## Testez-vous

**Question 1 :** que valent `9 / 2`, `9 // 2` et `9 % 2` ?

> **Réponse**
>
> `4.5`, `4` et `1`. `/` donne toujours un nombre à virgule, `//` la
> division entière, `%` le reste.

**Question 2 :** qu'affiche ce programme ?

```rythme norun
prix = 5
quantite = 3
print("Total : {prix * quantite} euros")
```

> **Réponse**
>
> `Total : 15 euros`. Le calcul entre accolades est fait, puis inséré dans
> le texte.

**Question 3 :** vous avez `points = 10` et vous voulez 5 points de plus.
Que faut-il écrire ?

> **Réponse**
>
> Un nouveau nom : `points2 = points + 5`. `points = points + 5` est
> l'erreur E011, car un nom ne change jamais.

## En résumé

- `nom = valeur` donne un nom à une valeur, pour toujours : une nouvelle
  valeur reçoit un nouveau nom.
- Les sortes de valeurs simples sont `Int`, `Float`, `Text` et `Bool` ;
  `type_of` donne la sorte d'une valeur.
- `/` donne toujours un nombre à virgule, `//` la division entière, `%` le
  reste, `**` la puissance.
- `"Bonjour {nom}"` insère une valeur dans un texte ; texte et nombre ne se
  mélangent jamais avec `+`.
- `print` (ou `puts`) affiche, et `#` commence un commentaire.

Suite : [Nombres et calculs](nombres.md)
