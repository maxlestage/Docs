# Premiers pas

Cette page présente les bases de Rythme : donner un nom à une valeur, les sortes de valeurs, les textes, l'affichage et les commentaires.

## Donner un nom à une valeur

On écrit un nom, `=`, puis une valeur :

```rythme
nom = "Ada"
naissance = 1815
print(nom, naissance)
```

```sortie
Ada 1815
```

`print` affiche ses valeurs séparées par une espace, puis passe à la ligne.

## Les noms ne changent jamais

En Rythme, une valeur ne change jamais. Un nom désigne une seule valeur. Si
vous voulez une autre valeur, donnez-lui un nouveau nom :

```rythme
total = 10
total2 = total + 5
print(total, total2)
```

```sortie
10 15
```

Redéfinir un nom est une erreur, trouvée avant même de lancer le programme :

```rythme norun
total = 1
total = total + 1
print(total)
```

```text
erreur[E011]: `total` est déjà défini
 --> total.ry:2:1
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

> **À retenir :** pas de `x = x + 1`, pas de `x += 1`. Une nouvelle valeur
> reçoit un nouveau nom.

## Les sortes de valeurs

Voici les quatre sortes de valeurs les plus simples :

| Sorte | Exemples | Pour quoi |
|-------|----------|-----------|
| `Int` | `42`, `-7`, `1_000_000` | les nombres entiers |
| `Float` | `3.14`, `2.0` | les nombres à virgule |
| `Text` | `"bonjour"` | les textes |
| `Bool` | `true`, `false` | vrai ou faux |

`type_of` donne la sorte d'une valeur :

```rythme
print(type_of(42), type_of(3.14), type_of("salut"), type_of(true))
```

```sortie
Int Float Text Bool
```

## Calculer

Les opérateurs habituels marchent. Attention, `/` donne toujours un `Float`,
et `//` donne la division entière :

```rythme
print(2 + 3 * 4, 7 / 2, 7 // 2, 7 % 3, 2 ** 10)
```

```sortie
14 3.5 3 1 1024
```

Les nombres entiers n'ont pas de limite de taille :

```rythme
print(2 ** 100)
```

```sortie
1267650600228229401496703205376
```

## Les textes et l'insertion

Un texte s'écrit entre guillemets droits `"`. Pour mettre une valeur dans un
texte, écrivez-la entre accolades `{...}` :

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

Comme en Ruby, `#{...}` marche aussi :

```rythme
nom = "Grace"
print("Bonjour, #{nom} !")
```

```sortie
Bonjour, Grace !
```

> **Attention :** Rythme ne mélange jamais texte et nombre en silence.
> `"âge : " + 36` est une erreur. Écrivez `"âge : {36}"`, ou convertissez
> avec `text(36)`.

Quelques fonctions utiles sur les textes (la page
[travailler avec du texte](texte.md) en montre bien d'autres) :

```rythme
print(upper("salut"), len("été"), trim("  ok  "), "a,b,c" |> split(","))
```

```sortie
SALUT 3 ok ["a", "b", "c"]
```

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

## Les commentaires

`#` commence un commentaire, jusqu'à la fin de la ligne. Rythme l'ignore.

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

Vos noms peuvent contenir des accents :

```rythme
prénom = "Élise"
âge = 28
print("{prénom} a {âge} ans.")
```

```sortie
Élise a 28 ans.
```

Un nom qui commence par une majuscule est réservé aux types et à leurs
variantes (voir [vos propres types](types.md)).

## En résumé

- `nom = valeur` donne un nom à une valeur, pour toujours : une nouvelle
  valeur reçoit un nouveau nom.
- Les sortes de valeurs simples sont `Int`, `Float`, `Text` et `Bool`.
- `/` donne toujours un nombre à virgule, `//` la division entière.
- `"Bonjour {nom}"` insère une valeur dans un texte.
- `print` (ou `puts`) affiche, et `#` commence un commentaire.

Suite : [conditions et répétition](conditions-boucles.md). Les mêmes bases en
[Ruby](../ruby/premiers-pas.md) et en [Rust](../rust/premiers-pas.md).
