# Les pièges du débutant en Rythme

Cette page rassemble les douze surprises qui bloquent le plus souvent les débutants en Rythme, surtout ceux qui viennent d'un autre langage, avec pour chacune le message réel et la bonne façon d'écrire.

> **Objectifs :**
>
> - vous saurez pourquoi une valeur ne change jamais, et quoi écrire à la place ;
> - vous saurez remplacer une boucle et une valeur `null` ;
> - vous saurez éviter les pièges des nombres : `/`, `//` et `0.1 + 0.2` ;
> - vous saurez pourquoi un texte et un nombre ne se mélangent pas ;
> - vous saurez lire les messages de Rythme quand vous tombez dans un piège.

## Comment lire cette page

Chaque piège suit le même ordre :

```text
1. Ce que vous attendez      (l'habitude d'un autre langage)
2. Ce que Rythme fait        (le code et son vrai message)
3. Pourquoi                  (une phrase ou deux)
4. La correction             (le code qui marche)
```

Vous n'avez pas besoin de tout lire d'un coup. Revenez ici quand un message
vous surprend.

> **À retenir :** Rythme vous prévient presque toujours. Lisez la ligne
> `aide:` du message : elle donne souvent la correction.

## Les pièges des valeurs

### 1. On ne change pas la valeur d'un nom

**Vous attendez :** `total = total + 1` ajoute 1 à `total`, comme dans
presque tous les langages.

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

**Pourquoi :** en Rythme, une [valeur](../debuter/glossaire.md#valeur) est
[immuable](../debuter/glossaire.md#immuable) : elle ne change jamais. Un nom
est une étiquette collée pour de bon sur une boîte.

```text
 total ──► [ 1 ]        total2 ──► [ 2 ]
 (étiquette collée)     (nouvelle étiquette, nouvelle boîte)
```

**La correction :** donnez un nouveau nom à la nouvelle valeur.

```rythme
total = 1
total2 = total + 1
print(total, total2)
```

```sortie
1 2
```

`+=` et `++` n'existent pas non plus, pour la même raison.

Essayez : remplacez `total2` par `total` sur la deuxième ligne. Rythme
refuse le programme avant même de le lancer.

### 2. push et put ne modifient rien

**Vous attendez :** `push(courses, "œufs")` ajoute les œufs à la liste.

```rythme
courses = ["pain", "lait"]
push(courses, "œufs")
print(courses)
```

```sortie
["pain", "lait"]
```

Les œufs ont disparu ! Rythme vous avait pourtant prévenu, avec un
avertissement :

```text
avertissement[W008]: le résultat de `push` est perdu
 --> main.ry:2:1
  |
2 | push(courses, "œufs")
  | ^^^^^^^^^^^^^^^^^^^^^ cette valeur est perdue
  |
  = aide: les valeurs ne changent jamais : `push(list, x)` donne une nouvelle valeur, et ne modifie rien.
      Pour la garder, donnez-lui un nom : `nouveau = push(list, x)`
```

**Pourquoi :** `push` fabrique une **nouvelle** liste. Comme personne ne lui
donne de nom, elle est perdue. C'est pareil pour `put` sur un
[dictionnaire](../debuter/glossaire.md#dictionnaire).

**La correction :** gardez le résultat sous un nouveau nom.

```rythme
courses = ["pain", "lait"]
courses2 = push(courses, "œufs")
print(courses)
print(courses2)
```

```sortie
["pain", "lait"]
["pain", "lait", "œufs"]
```

C'est comme une photocopie de la liste de courses : vous écrivez sur la
copie, l'original ne bouge pas.

Un champ d'[enregistrement](../debuter/glossaire.md#enregistrement) ne se
modifie pas non plus :

```rythme norun
ada = {nom: "Ada", age: 36}
ada.age = 37
```

```text
erreur[E001]: impossible de donner une valeur à ceci
 --> main.ry:2:1
  |
2 | ada.age = 37
  | ^^^^^^^ ceci n'est pas un nom
  |
2 | ada.age = 37
  |         - `=` donne un nom à une valeur
  |
  = aide: les enregistrements ne peuvent pas être modifiés ; construisez-en un nouveau : `nouveau = {..ancien, age: valeur}`
  = en savoir plus : `rythme explain E001`
```

On construit une copie avec un champ changé :

```rythme
ada = {nom: "Ada", age: 36}
ada2 = {..ada, age: 37}
print(ada, ada2)
```

```sortie
{nom: "Ada", age: 36} {nom: "Ada", age: 37}
```

> **À retenir :** en Rythme, rien ne se modifie. On fabrique une nouvelle
> valeur, et on lui donne un nouveau nom.

### 3. Il n'y a pas de boucle for ni while

**Vous attendez :** une [boucle](../debuter/glossaire.md#boucle) `for` pour
répéter.

```rythme norun
for x in [1, 2]
  print(x)
end
```

```text
erreur[E001]: `for` n'est pas utilisé en Rythme
 --> main.ry:1:1
  |
1 | for x in [1, 2]
  | ^^^ vient d'un autre langage
  |
  = aide: Rythme n'a pas de boucles. Travaillez sur toute la liste à la place :
        1..5 |> each(fn(i) = print(i))
        [1, 2, 3] |> map(fn(x) = x * 2)
      ou écrivez une fonction récursive. Voir `rythme help functional`.
  = en savoir plus : `rythme explain E001`
```

**Pourquoi :** une boucle change un compteur à chaque tour. Or rien ne
change en Rythme. On dit plutôt « fais ceci pour toute la liste ».

**La correction :** `each` pour **faire** quelque chose avec chaque élément,
`map` pour **calculer** une nouvelle liste.

```rythme
[1, 2, 3] |> each(fn(x) = print("élément {x}"))
doubles = [1, 2, 3] |> map(fn(x) = x * 2)
print(doubles)
```

```sortie
élément 1
élément 2
élément 3
[2, 4, 6]
```

Voici ce que fait `map`, élément par élément :

| Étape | x | x * 2 | Liste fabriquée |
|-------|---|-------|-----------------|
| 1 | 1 | 2 | `[2]` |
| 2 | 2 | 4 | `[2, 4]` |
| 3 | 3 | 6 | `[2, 4, 6]` |

Pour une boucle « tant que », écrivez une fonction qui s'appelle elle-même
(voir [conditions et répétition](conditions-boucles.md#repeter-avec-la-recursion)).

Essayez : remplacez `x * 2` par `x * x`. Vous obtenez `[1, 4, 9]`.

### 4. Il n'y a pas de null

**Vous attendez :** `null` (ou `nil`, `None` en Python) pour dire « pas de
valeur ».

```rythme norun
reponse = null
```

```text
erreur[E001]: `null` n'est pas utilisé en Rythme
 --> main.ry:1:11
  |
1 | reponse = null
  |           ^^^^ vient d'un autre langage
  |
  = aide: Rythme n'a pas de null : utilisez `None` pour « pas de valeur » et `Some(x)` pour une valeur
  = en savoir plus : `rythme explain E001`
```

**Pourquoi :** un [null](../debuter/glossaire.md#null) oublié est la cause
de nombreux plantages. Rythme range plutôt la valeur dans une boîte, une
[option](../debuter/glossaire.md#option) : `Some(valeur)` quand il y a
quelque chose, `None` quand la boîte est vide.

```rythme
print(first(["a", "b"]), first([]))
```

```sortie
Some("a") None
```

On ne calcule pas directement avec la boîte : `first([]) + 1` est une
erreur. On l'ouvre avec `unwrap_or`, en donnant une valeur de secours :

```rythme
vide = first([]) |> unwrap_or(0)
plein = first([5, 6]) |> unwrap_or(0)
print(vide + 1, plein + 1)
```

```sortie
1 6
```

Les options sont expliquées en détail dans [gérer les erreurs](erreurs.md).

> **À retenir :** pas de `for`, pas de `null`. Rythme vous dit quoi écrire à
> la place, dans la ligne `aide:`.

## Les pièges des nombres et des textes

### 5. La division / donne toujours un nombre à virgule

**Vous attendez :** `6 / 3` vaut `2`.

```rythme
print(6 / 3, 7 / 2, 7 // 2)
```

```sortie
2.0 3.5 3
```

**Pourquoi :** `/` donne **toujours** un nombre à virgule, un
[flottant](../debuter/glossaire.md#flottant) (`Float`), même quand la
division tombe juste. Pour un [entier](../debuter/glossaire.md#entier)
(`Int`), utilisez la division entière `//`.

Le piège arrive quand on se sert du résultat comme position :

```rythme norun
notes = [12, 15, 9, 18]
milieu = len(notes) / 2
print(notes[milieu])
```

```text
erreur[E005]: un indice doit être un nombre entier (Int), mais a reçu une valeur de type Float
 --> main.ry:3:13
  |
3 | print(notes[milieu])
  |             ^^^^^^
  |
  = en savoir plus : `rythme explain E005`
```

**La correction :** `//` donne un entier.

```rythme
notes = [12, 15, 9, 18]
milieu = len(notes) // 2
print(milieu, notes[milieu])
```

```sortie
2 9
```

Essayez : affichez `type_of(4 / 2)` puis `type_of(4 // 2)`. Vous lisez
`Float` puis `Int`.

### 6. 0.1 + 0.2 ne vaut pas tout à fait 0.3

**Vous attendez :** `0.1 + 0.2 == 0.3` vaut `true`.

```rythme
print(0.1 + 0.2, 0.1 + 0.2 == 0.3)
```

```sortie
0.30000000000000004 false
```

**Pourquoi :** l'ordinateur range les nombres à virgule en binaire, avec un
nombre limité de chiffres. Certains nombres, comme 0.1, ne tombent pas
juste, comme 1/3 qui s'écrit 0.3333... en décimal. Ce n'est pas propre à
Rythme : Python, JavaScript et Ruby font pareil.

**La correction :** arrondissez avant de comparer, ou comptez en entiers
(des centimes au lieu des euros).

```rythme
print(round(0.1 + 0.2, 2) == 0.3, 10 + 20 == 30)
```

```sortie
true true
```

### 7. Un texte et un nombre ne s'additionnent pas

**Vous attendez :** `"âge : " + 36` donne `"âge : 36"`.

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

**Pourquoi :** Rythme ne devine jamais. `"3" + 4`, est-ce `"34"` ou `7` ?
Plutôt que de choisir en silence, il refuse. Pour la même raison, un texte
n'est jamais égal à un nombre :

```rythme
print("1" == 1, 1 == 1.0)
```

```sortie
false true
```

**La correction :** mettez la valeur entre accolades, ou convertissez-la
avec `text`.

```rythme
age = 36
print("âge : {age}", "âge : " + text(age))
```

```sortie
âge : 36 âge : 36
```

Dans l'autre sens, `parse_int` lit un nombre dans un texte. Mais il donne
une boîte `Ok(42)`, pas `42`, car le texte n'est peut-être pas un nombre :

```rythme
print(parse_int("42"), parse_int("bonjour"))
n = parse_int("42") |> unwrap_or(0)
print(n + 1)
```

```sortie
Ok(42) Err("\"bonjour\" n'est pas un nombre entier")
43
```

> **À retenir :** un texte reste un texte, un nombre reste un nombre. Pour
> passer de l'un à l'autre : `"{x}"` ou `text(x)` dans un sens,
> `parse_int` dans l'autre.

## Les pièges de l'écriture

### 8. Un if a toujours un else

**Vous attendez :** un `if` sans `else`, qui ne fait rien quand la
condition est fausse.

```rythme norun
x = 5
if x > 0 then print("positif")
```

```text
erreur[E019]: ce `if` n'a pas de `else`
 --> main.ry:2:1
  |
2 | if x > 0 then print("positif")
  | ^^ chaque `if` doit avoir un `else`
  |
3 | 
  | - `else` attendu ici
  |
  = aide: en Rythme, `if` donne toujours une valeur, il lui faut donc les deux cas :
        if age >= 18 then "majeur" else "mineur"
  = en savoir plus : `rythme explain E019`
```

**Pourquoi :** en Rythme, `if` donne une valeur, comme un calcul. Il lui
faut une valeur pour les deux cas.

**La correction :** quand il n'y a rien à faire, donnez la valeur vide `()`.

```rythme
x = 5
if x > 0 then print("positif") else ()
```

```sortie
positif
```

Autre surprise : la [condition](../debuter/glossaire.md#condition) doit
être `true` ou `false`. Une liste vide ou le nombre 0 ne veulent pas dire
« faux » :

```rythme norun
panier = []
print(if panier then "plein" else "vide")
```

```text
erreur[E005]: une condition doit être `true` ou `false`, mais ceci est une valeur de type List
 --> main.ry:2:10
  |
2 | print(if panier then "plein" else "vide")
  |          ^^^^^^ pas un Bool
  |
  = aide: pour tester si c'est vide, utilisez `is_empty(x)` ou `not is_empty(x)`
  = en savoir plus : `rythme explain E005`
```

Écrivez la question en entier : `if is_empty(panier) then "vide" else "plein"`.

### 9. C'est end qui ferme un bloc, pas l'indentation

**Vous attendez (en Python) :** revenir à gauche suffit à finir la fonction.

```rythme norun
fn double(x)
  x * 2
print(double(4))
```

```text
erreur[E020]: il manque `end` pour fermer ce `fn`
 --> main.ry:1:1
  |
1 | fn double(x)
  | ^^ ce `fn` n'est jamais fermé
  |
4 | 
  | - le code se termine ici
  |
  = aide: ajoutez `end` sur sa propre ligne après la dernière ligne du bloc
  = en savoir plus : `rythme explain E020`
```

**Pourquoi :** en Rythme, l'[indentation](../debuter/glossaire.md#indentation)
(les espaces au début des lignes) sert seulement à la lecture. C'est le mot
`end` qui ferme un [bloc](../debuter/glossaire.md#bloc).

**La correction :**

```rythme
fn double(x)
  x * 2
end
print(double(4))
```

```sortie
8
```

```text
fn double(x)      ◄── ouvre le bloc
  x * 2           ◄── le contenu (deux espaces : pour vos yeux)
end               ◄── ferme le bloc
```

Une fonction d'une seule ligne n'a pas besoin de `end` :
`fn double(x) = x * 2`.

> **Astuce :** `rythme fmt main.ry` remet les espaces au bon endroit pour
> vous.

### 10. Les accents et les majuscules comptent

**Vous attendez :** `prénom` et `prenom`, c'est pareil.

```rythme norun
prénom = "Léa"
print(prenom)
```

```text
erreur[E003]: nom inconnu `prenom`
 --> main.ry:2:7
  |
2 | print(prenom)
  |       ^^^^^^ non défini
  |
  = aide: vouliez-vous dire `prénom` ?
  = en savoir plus : `rythme explain E003`
```

**Pourquoi :** les accents sont permis dans les noms. Mais `é` et `e` sont
deux lettres différentes, donc `prénom` et `prenom` sont deux noms.

**La correction :** choisissez une écriture et gardez-la partout. Beaucoup
de gens écrivent leurs noms sans accents, pour ne pas hésiter.

Une majuscule au début d'un nom pose un autre problème :

```rythme norun
Total = 3
```

```text
erreur[E003]: constructeur inconnu `Total`
 --> main.ry:1:1
  |
1 | Total = 3
  | ^^^^^ non défini
  |
  = aide: les constructeurs se définissent avec `type` : `type Nom = Total(champ) | Autre`
  = en savoir plus : `rythme explain E003`
```

Un nom qui commence par une majuscule est réservé aux types et à leurs
variantes (voir [vos propres types](types.md)). Écrivez `total = 3`.

### 11. On compte les positions à partir de 0

**Vous attendez :** le premier élément est à la position 1.

```rythme
notes = [12, 15, 9]
print(notes[0], notes[2], notes[-1])
```

```sortie
12 9 9
```

```text
 position :   0    1    2
 notes    : [ 12 , 15 , 9 ]
                        -1  (le dernier, en comptant depuis la fin)
```

Le dernier élément d'une liste de 3 est donc à la position 2, pas 3 :

```rythme norun
fruits = ["pomme", "poire", "kiwi"]
print(fruits[3])
```

```text
erreur[E007]: l'indice 3 est hors limites : la liste a 3 élément(s)
 --> main.ry:2:14
  |
2 | print(fruits[3])
  |              ^ aucun élément ici
  |
  = aide: les indices valides vont de 0 à 2 (ou -1 pour le dernier élément)
  = aide: pour obtenir une Option au lieu d'une erreur, utilisez `get(x, 3)`
  = en savoir plus : `rythme explain E007`
```

**La correction :** `fruits[-1]` pour le dernier, ou `get(fruits, 3)` qui
donne `None` au lieu de s'arrêter.

### 12. Une ligne qui commence par - est une nouvelle ligne de code

**Vous attendez :** un long calcul peut continuer sur la ligne suivante.

```rythme norun
prix = 10
remise = 3
total = prix
  - remise
print(total)
```

```text
erreur[E024]: cette ligne calcule une valeur mais n'en fait rien
 --> main.ry:4:3
  |
4 |   - remise
  |   ^^^^^^^^ cette valeur est perdue
  |
  = aide: une ligne qui commence par `-` est une nouvelle instruction. Pour continuer la ligne
      précédente, terminez plutôt cette ligne par `-` :
        total = prix -
          remise
  = en savoir plus : `rythme explain E024`
```

**Pourquoi :** `-remise` tout seul est déjà un calcul valable (« moins
remise »). Rythme ne sait donc pas qu'il doit le rattacher à la ligne du
dessus.

**La correction :** laissez l'opérateur à la fin de la première ligne.

```rythme
prix = 10
remise = 3
total = prix -
  remise
print(total)
```

```sortie
7
```

## Erreurs fréquentes

Toute la page parle d'erreurs ; voici encore trois réflexes venus d'autres
langages.

### Des apostrophes autour d'un texte

```rythme norun
nom = 'Ada'
```

```text
erreur[E018]: caractère inattendu `'`
 --> main.ry:1:7
  |
1 | nom = 'Ada'
  |       ^
  |
  = aide: les textes s'écrivent entre guillemets droits : "bonjour"
  = en savoir plus : `rythme explain E018`
```

Un texte s'écrit toujours entre guillemets droits : `nom = "Ada"`.

### Écrire return

```rythme norun
fn double(x)
  return x * 2
end
```

```text
erreur[E001]: `return` n'est pas utilisé en Rythme
 --> main.ry:2:3
  |
2 |   return x * 2
  |   ^^^^^^ vient d'un autre langage
  |
  = aide: pas besoin de `return` : la dernière expression d'une fonction est son résultat.
      Pour arrêter plus tôt avec une erreur, renvoyez `Err(...)` et utilisez `?`.
  = en savoir plus : `rythme explain E001`
```

La dernière ligne d'une fonction est son résultat : écrivez simplement
`x * 2`.

### Comparer avec = au lieu de ==

```rythme norun
x = 3
print(if x = 3 then "trois" else "autre")
```

```text
erreur[E001]: `then` attendu, mais trouvé `=`
 --> main.ry:2:12
  |
2 | print(if x = 3 then "trois" else "autre")
  |            ^ `then` attendu ici
  |
  = aide: pour comparer deux valeurs, utilisez `==`
  = aide: un `if` s'écrit : `if condition then valeur else autre_valeur`
  = en savoir plus : `rythme explain E001`
```

`=` donne un nom, `==` compare : `if x == 3 then ...`.

## Testez-vous

**Question 1 :** ce programme affiche `[1, 2]`. Pourquoi le 3 n'y est-il
pas, et comment le garder ?

```rythme norun
xs = [1, 2]
push(xs, 3)
print(xs)
```

> **Réponse**
>
> `push` ne modifie pas `xs` : il fabrique une nouvelle liste, perdue ici
> car elle n'a pas de nom. Écrivez `xs2 = push(xs, 3)` puis `print(xs2)`.

**Question 2 :** que valent `9 / 3` et `9 // 3` ?

> **Réponse**
>
> `3.0` et `3`. `/` donne toujours un nombre à virgule ; `//` donne un
> entier.

**Question 3 :** comment afficher `Score : 12` quand `score = 12` ?

> **Réponse**
>
> `print("Score : {score}")`, ou `print("Score : " + text(score))`.
> `"Score : " + score` est l'erreur E005 : un texte et un nombre ne
> s'additionnent pas.

## En résumé

- Rien ne se modifie : `push`, `put` et `{..r, champ: v}` donnent une
  nouvelle valeur, à ranger sous un nouveau nom.
- Pas de boucles ni de `null` : `each`, `map` et la récursion d'un côté,
  `Some(x)` et `None` de l'autre.
- `/` donne un nombre à virgule, `//` un entier ; arrondissez avant de
  comparer des nombres à virgule.
- Un texte et un nombre ne se mélangent pas : `"{x}"`, `text(x)` et
  `parse_int`.
- Tout `if` a un `else`, et c'est `end` qui ferme un bloc.
- Les noms tiennent compte des accents et des majuscules, et les positions
  commencent à 0.

Suite : [Exercices pour bien démarrer](exercices-faciles.md)
