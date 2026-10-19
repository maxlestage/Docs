# Exercices pour bien démarrer

Cette page propose 15 exercices très guidés, tous faciles, pour pratiquer les bases de Python : afficher, calculer, choisir, répéter, écrire des fonctions et manier listes et dictionnaires.

## Comment faire ces exercices

Chaque exercice est petit. Il demande une seule idée nouvelle. Pour chacun :

1. **Lisez** l'énoncé et la sortie attendue. C'est ce que votre programme doit afficher.
2. **Essayez sur papier.** Écrivez les étapes en français : « d'abord je…, puis je… ».
3. **Écrivez le code** dans un fichier `exercice.py`, puis lancez `python3 exercice.py`.
4. **Comparez** ce qui s'affiche avec la sortie attendue, caractère par caractère.
5. **Bloqué ?** Lisez le premier indice. Encore bloqué ? Le deuxième. La solution vient
   en dernier.

> **Astuce :** une erreur rouge n'est pas un échec. Lisez la dernière ligne du message :
> elle dit souvent ce qui ne va pas. La page [lire une erreur](../debuter/lire-une-erreur.md)
> vous aide.

Il existe souvent plusieurs bonnes solutions. Si votre programme affiche exactement la
sortie attendue, c'est gagné.

## Afficher et calculer

### Exercice 1 : Une carte de visite

Niveau : ★☆☆

Rangez votre prénom et votre ville dans deux variables. Affichez ensuite deux lignes,
comme ci-dessous.

```text
Prénom : Alice
Ville : Lyon
```

> **Indice :** une [variable](../debuter/glossaire.md#variable) se crée avec `=` :
> `prenom = "Alice"`. N'oubliez pas les guillemets autour du texte.

> **Indice :** une f-string colle du texte et une variable : `f"Prénom : {prenom}"`.

> **Solution**
>
> ```python
> prenom = "Alice"
> ville = "Lyon"
> print(f"Prénom : {prenom}")
> print(f"Ville : {ville}")
> ```
>
> ```sortie
> Prénom : Alice
> Ville : Lyon
> ```
>
> Pourquoi ça marche : dans une f-string, Python remplace `{prenom}` par ce que contient
> la boîte `prenom`. Chaque `print` affiche une ligne.

### Exercice 2 : Le prix total

Niveau : ★☆☆

Un cahier coûte 4 euros. Vous en achetez 3. Rangez le prix et la quantité dans deux
variables, calculez le total, puis affichez-le.

```text
Total : 12 euros
```

> **Indice :** le signe de la multiplication est `*`.

> **Indice :** créez une troisième variable : `total = prix * quantite`.

> **Solution**
>
> ```python
> prix = 4
> quantite = 3
> total = prix * quantite
> print(f"Total : {total} euros")
> ```
>
> ```sortie
> Total : 12 euros
> ```
>
> Pourquoi ça marche : Python calcule d'abord `prix * quantite`, c'est-à-dire `4 * 3`,
> puis range `12` dans `total`.

### Exercice 3 : Des minutes en heures

Niveau : ★☆☆

Un film dure 135 minutes. Affichez sa durée en heures et minutes.

```text
2 h 15 min
```

> **Indice :** dans 135 minutes, combien de fois 60 ? C'est la division entière `//`
> (voir [les nombres](nombres.md)).

> **Indice :** ce qui reste après avoir retiré les heures, c'est le reste `%` :
> `135 % 60`.

> **Solution**
>
> ```python
> duree = 135
> heures = duree // 60
> minutes = duree % 60
> print(f"{heures} h {minutes} min")
> ```
>
> ```sortie
> 2 h 15 min
> ```
>
> Pourquoi ça marche : `135 // 60` vaut `2` (deux paquets de 60 entiers). Il reste
> `135 - 120 = 15` minutes, et c'est exactement `135 % 60`.

## Choisir

### Exercice 4 : Majeur ou mineur

Niveau : ★☆☆

Rangez l'âge `15` dans une variable. Affichez `mineur` si l'âge est inférieur à 18,
sinon `majeur`.

```text
mineur
```

> **Indice :** utilisez `if` et `else` (voir
> [conditions et boucles](conditions-boucles.md)). N'oubliez pas les deux-points et les
> 4 espaces.

> **Solution**
>
> ```python
> age = 15
> if age < 18:
>     print("mineur")
> else:
>     print("majeur")
> ```
>
> ```sortie
> mineur
> ```
>
> Pourquoi ça marche : `15 < 18` vaut `True`, donc Python exécute le bloc du `if` et
> saute celui du `else`.
>
> Essayez : mettez `age = 18`. Le programme affiche `majeur`, car `18 < 18` est faux.

### Exercice 5 : La météo

Niveau : ★☆☆

Rangez une température dans une variable. Affichez `froid` en dessous de 10 degrés,
`doux` de 10 à 24, et `chaud` à partir de 25. Testez avec `18`.

```text
doux
```

> **Indice :** trois cas, donc `if`, `elif` et `else`.

> **Indice :** Python teste les conditions dans l'ordre et s'arrête à la première
> vraie. Commencez par `if temperature < 10:`.

> **Solution**
>
> ```python
> temperature = 18
> if temperature < 10:
>     print("froid")
> elif temperature < 25:
>     print("doux")
> else:
>     print("chaud")
> ```
>
> ```sortie
> doux
> ```
>
> Pourquoi ça marche : `18 < 10` est faux, Python passe au `elif`. `18 < 25` est vrai :
> il affiche `doux` et ignore le reste.

## Répéter

### Exercice 6 : Compter jusqu'à 5

Niveau : ★☆☆

Affichez les nombres de 1 à 5, un par ligne.

```text
1
2
3
4
5
```

> **Indice :** une boucle `for` avec `range`.

> **Indice :** `range(1, 6)` donne 1, 2, 3, 4, 5. Le 6 est exclu.

> **Solution**
>
> ```python
> for i in range(1, 6):
>     print(i)
> ```
>
> ```sortie
> 1
> 2
> 3
> 4
> 5
> ```
>
> Pourquoi ça marche : à chaque tour, `i` prend la valeur suivante de `range(1, 6)`, et
> `print(i)` l'affiche.

### Exercice 7 : La somme de 1 à 10

Niveau : ★☆☆

Calculez `1 + 2 + 3 + … + 10` avec une boucle, puis affichez le résultat.

```text
55
```

> **Indice :** créez une variable `total = 0` **avant** la boucle. C'est une tirelire
> vide.

> **Indice :** dans la boucle, ajoutez `i` à la tirelire : `total = total + i`.

> **Solution**
>
> ```python
> total = 0
> for i in range(1, 11):
>     total = total + i
> print(total)
> ```
>
> ```sortie
> 55
> ```
>
> Pourquoi ça marche : voici les premiers tours.
>
> | Tour | `i` | `total` après le tour |
> |---|---|---|
> | 1 | 1 | 1 |
> | 2 | 2 | 3 |
> | 3 | 3 | 6 |
> | … | … | … |
> | 10 | 10 | 55 |
>
> Le `print` est **hors** de la boucle (pas d'espaces devant) : il n'affiche qu'une fois,
> à la fin.

### Exercice 8 : Le compte à rebours

Niveau : ★☆☆

Avec une boucle `while`, affichez 3, 2, 1, puis `Partez !`.

```text
3
2
1
Partez !
```

> **Indice :** partez de `n = 3`. La boucle tourne tant que `n > 0`.

> **Indice :** dans la boucle, affichez `n`, puis enlevez 1 : `n = n - 1`. Sans cette
> ligne, la boucle ne s'arrête jamais.

> **Solution**
>
> ```python
> n = 3
> while n > 0:
>     print(n)
>     n = n - 1
> print("Partez !")
> ```
>
> ```sortie
> 3
> 2
> 1
> Partez !
> ```
>
> Pourquoi ça marche : `n` passe de 3 à 2, puis 1, puis 0. Quand `n` vaut 0, `n > 0` est
> faux : la boucle s'arrête et Python passe à la ligne suivante.

## Fonctions

### Exercice 9 : Dire bonjour

Niveau : ★☆☆

Écrivez une fonction `bonjour(prenom)` qui **renvoie** la phrase `Bonjour Alice !`.
Affichez le résultat pour `"Alice"` et pour `"Bob"`.

```text
Bonjour Alice !
Bonjour Bob !
```

> **Indice :** une [fonction](../debuter/glossaire.md#fonction) commence par
> `def bonjour(prenom):` (voir [les fonctions](fonctions.md)).

> **Indice :** la fonction utilise `return`, et c'est vous qui faites `print` en dehors :
> `print(bonjour("Alice"))`.

> **Solution**
>
> ```python
> def bonjour(prenom):
>     return f"Bonjour {prenom} !"
>
> print(bonjour("Alice"))
> print(bonjour("Bob"))
> ```
>
> ```sortie
> Bonjour Alice !
> Bonjour Bob !
> ```
>
> Pourquoi ça marche : à chaque appel, le [paramètre](../debuter/glossaire.md#parametre)
> `prenom` reçoit une valeur différente. `return` renvoie la phrase, et `print`
> l'affiche.

### Exercice 10 : Le plus grand des deux

Niveau : ★☆☆

Écrivez une fonction `plus_grand(a, b)` qui renvoie le plus grand des deux nombres,
**sans** utiliser `max`. Affichez `plus_grand(4, 9)` et `plus_grand(7, 2)`.

```text
9
7
```

> **Indice :** dans la fonction, comparez `a` et `b` avec un `if`.

> **Indice :** une fonction peut avoir deux `return`. Le premier atteint arrête la
> fonction.

> **Solution**
>
> ```python
> def plus_grand(a, b):
>     if a > b:
>         return a
>     return b
>
> print(plus_grand(4, 9))
> print(plus_grand(7, 2))
> ```
>
> ```sortie
> 9
> 7
> ```
>
> Pourquoi ça marche : pour `(7, 2)`, `7 > 2` est vrai, la fonction renvoie `7` et
> s'arrête. Pour `(4, 9)`, le test est faux : on arrive au second `return b`.

## Texte

### Exercice 11 : Compter les lettres

Niveau : ★☆☆

Rangez le mot `"python"` dans une variable. Affichez-le en majuscules, suivi de son
nombre de lettres.

```text
PYTHON a 6 lettres
```

> **Indice :** `len(mot)` donne la longueur d'un texte (voir [le texte](texte.md)).

> **Indice :** `mot.upper()` renvoie le texte en majuscules.

> **Solution**
>
> ```python
> mot = "python"
> print(f"{mot.upper()} a {len(mot)} lettres")
> ```
>
> ```sortie
> PYTHON a 6 lettres
> ```
>
> Pourquoi ça marche : entre les accolades d'une f-string, vous pouvez écrire n'importe
> quel calcul, même un appel de méthode.

## Listes et dictionnaires

### Exercice 12 : La liste de courses

Niveau : ★☆☆

Partez de la liste `["pain", "lait"]`. Ajoutez `"pommes"` à la fin. Affichez chaque
article précédé d'un tiret, puis le nombre d'articles.

```text
- pain
- lait
- pommes
3 articles
```

> **Indice :** `courses.append("pommes")` ajoute à la fin (voir
> [les collections](collections.md)).

> **Indice :** `for article in courses:` parcourt la liste, `len(courses)` la compte.

> **Solution**
>
> ```python
> courses = ["pain", "lait"]
> courses.append("pommes")
> for article in courses:
>     print(f"- {article}")
> print(f"{len(courses)} articles")
> ```
>
> ```sortie
> - pain
> - lait
> - pommes
> 3 articles
> ```
>
> Pourquoi ça marche : `append` modifie la liste elle-même. La boucle voit donc bien les
> trois articles.

### Exercice 13 : La meilleure note

Niveau : ★☆☆

Trouvez la plus grande note de la liste `[12, 17, 9, 15]` avec une boucle, **sans**
utiliser `max`.

```text
17
```

> **Indice :** gardez dans une variable `meilleure` la plus grande note vue jusqu'ici.
> Au départ, prenez la première : `meilleure = notes[0]`.

> **Indice :** à chaque note, si elle est plus grande que `meilleure`, elle prend sa
> place.

> **Solution**
>
> ```python
> notes = [12, 17, 9, 15]
> meilleure = notes[0]
> for note in notes:
>     if note > meilleure:
>         meilleure = note
> print(meilleure)
> ```
>
> ```sortie
> 17
> ```
>
> Pourquoi ça marche :
>
> | `note` | `note > meilleure` ? | `meilleure` après |
> |---|---|---|
> | 12 | non | 12 |
> | 17 | oui | 17 |
> | 9 | non | 17 |
> | 15 | non | 17 |

### Exercice 14 : Les prix des fruits

Niveau : ★☆☆

Partez du dictionnaire `{"pomme": 2, "kiwi": 3}`. Ajoutez la clé `"melon"` avec le prix
`5`. Affichez chaque fruit avec son prix.

```text
pomme : 2 €
kiwi : 3 €
melon : 5 €
```

> **Indice :** on ajoute une clé comme on range une valeur :
> `prix["melon"] = 5` (voir [les dictionnaires](dictionnaires.md)).

> **Indice :** `for fruit, montant in prix.items():` donne la clé et la valeur à chaque
> tour.

> **Solution**
>
> ```python
> prix = {"pomme": 2, "kiwi": 3}
> prix["melon"] = 5
> for fruit, montant in prix.items():
>     print(f"{fruit} : {montant} €")
> ```
>
> ```sortie
> pomme : 2 €
> kiwi : 3 €
> melon : 5 €
> ```
>
> Pourquoi ça marche : un [dictionnaire](../debuter/glossaire.md#dictionnaire) range des
> paires « clé → valeur ». Il garde l'ordre dans lequel vous avez ajouté les clés.

### Exercice 15 : Les carrés

Niveau : ★☆☆

Construisez la liste des carrés des nombres de 1 à 5 en **une seule ligne**, avec une
compréhension, puis affichez-la.

```text
[1, 4, 9, 16, 25]
```

> **Indice :** le carré de `n` s'écrit `n * n`.

> **Indice :** une compréhension a la forme `[ce_qu_on_ajoute for n in ...]` (voir
> [les compréhensions](comprehensions.md)).

> **Solution**
>
> ```python
> carres = [n * n for n in range(1, 6)]
> print(carres)
> ```
>
> ```sortie
> [1, 4, 9, 16, 25]
> ```
>
> Pourquoi ça marche : lisez à voix haute « la liste des `n * n`, pour chaque `n` de 1 à
> 5 ». La même chose avec une boucle demanderait trois lignes et un `append`.

## Et ensuite ?

Bravo ! Si ces exercices vous semblent faciles, passez aux
[exercices corrigés](exercices.md), qui montent jusqu'au niveau ★★★. Si l'un d'eux vous
a résisté, relisez [les pièges du débutant](pieges.md) : la cause s'y trouve souvent.

Suite : [Classes et objets](types.md)
