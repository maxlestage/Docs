# Les types de données : nombres, texte, listes

Chaque valeur a un type, comme un nombre, un texte ou une liste : cette page présente les types de base, en Python d'abord, puis en JavaScript et en Ruby.

> **Objectifs :**
>
> - comprendre ce qu'est un type, et pourquoi il compte ;
> - utiliser des nombres entiers et décimaux ;
> - écrire du texte, le coller à un autre et mesurer sa longueur ;
> - convertir un texte en nombre, et un nombre en texte ;
> - ranger des valeurs dans une liste et dans un dictionnaire ;
> - reconnaître la valeur « rien ».

## Qu'est-ce qu'un type

Dans une cuisine, on ne range pas la farine avec les fourchettes. Chaque
chose a sa **sorte** : un ingrédient, un ustensile, une assiette.

En programmation, c'est pareil. Chaque [valeur](glossaire.md#valeur) a une
sorte : on l'appelle son [type](glossaire.md#type).

- `12` est un nombre ;
- `"bonjour"` est un texte ;
- `[12, 15, 9]` est une liste.

Le type dit **ce qu'on peut faire** avec la valeur. On additionne deux
nombres. On ne peut pas additionner un mot et un nombre : `"chat" + 3`, ça
ne veut rien dire.

Python donne le type d'une valeur avec `type` :

```python
print(type(12))
print(type("bonjour"))
print(type([12, 15, 9]))
```

```sortie
<class 'int'>
<class 'str'>
<class 'list'>
```

`int` veut dire nombre entier, `str` veut dire texte, `list` veut dire liste.

> **À retenir :** chaque valeur a un type. Le type décide ce qu'on peut
> faire avec elle.

## Les nombres entiers et décimaux

Il y a deux sortes de nombres :

- les **entiers**, sans virgule : `3`, `-12`, `2026` (type `int`, voir
  [entier](glossaire.md#entier)) ;
- les **décimaux**, avec une virgule : `3.5`, `-0.25` (type `float`, voir
  [flottant](glossaire.md#flottant)).

Attention : en programmation, la virgule s'écrit avec un **point**. On écrit
`3.5`, jamais `3,5`.

```python
print(7 + 2, 7 - 2, 7 * 2)
print(7 / 2, 7 // 2, 7 % 2)
```

```sortie
9 5 14
3.5 3 1
```

| Signe | Opération | Exemple | Résultat |
|---|---|---|---|
| `+` | addition | `7 + 2` | `9` |
| `-` | soustraction | `7 - 2` | `5` |
| `*` | multiplication | `7 * 2` | `14` |
| `/` | division | `7 / 2` | `3.5` |
| `//` | division entière | `7 // 2` | `3` |
| `%` | reste de la division | `7 % 2` | `1` |

Pensez à 7 bonbons pour 2 enfants : chacun en reçoit 3 (`7 // 2`), et il en
reste 1 (`7 % 2`).

Essayez : remplacez `7` par `10`. Le reste `10 % 2` vaut `0` : 10 est pair.

### Le mystère de 0.1 + 0.2

Faites ce calcul :

```python
print(0.1 + 0.2)
```

```sortie
0.30000000000000004
```

Ce n'est pas un bug de Python. L'ordinateur range les décimaux en binaire
(avec des 0 et des 1). Certains nombres, comme 0.1, ne tombent pas juste,
comme 1/3 qui s'écrit 0.3333… sans fin. Il reste une toute petite erreur.

Pour afficher un résultat propre, on arrondit avec `round` :

```python
print(round(0.1 + 0.2, 2))
```

```sortie
0.3
```

Le `2` veut dire « deux chiffres après la virgule ». JavaScript et Ruby
donnent exactement le même `0.30000000000000004`.

> **À retenir :** les décimaux sont parfois un tout petit peu faux.
> Arrondissez avant d'afficher, et ne comptez jamais l'argent avec eux sans
> précaution.

## Le texte

Un texte s'appelle une [chaîne de caractères](glossaire.md#chaine-de-caracteres),
`str` en Python. On l'écrit entre **guillemets** : `"bonjour"` ou
`'bonjour'`. Les guillemets disent à l'ordinateur : « ceci est un texte, pas
un nom de variable ».

On colle deux textes avec `+` : on dit qu'on les **concatène**. On mesure
leur longueur avec `len` :

```python
prenom = "Ada"
salut = "Bonjour " + prenom + " !"
print(salut)
print(len(prenom))
print(len(salut))
```

```sortie
Bonjour Ada !
3
13
```

`len` compte **tous** les caractères, espaces et `!` compris.

Essayez : écrivez `print("prenom")` au lieu de `print(prenom)`. Python
affiche le mot `prenom` lui-même, pas `Ada` : avec des guillemets, c'est un
texte.

## Convertir d'un type à l'autre

`"3"` et `3` se ressemblent, mais ce n'est pas la même chose :

```text
  "3"  →  un texte qui contient le caractère 3   (comme une étiquette)
   3   →  le nombre trois                        (on peut calculer avec)
```

Avec `+`, la différence se voit tout de suite :

```python
print("3" + "3")
print(3 + 3)
```

```sortie
33
6
```

Deux textes sont collés : `"33"`. Deux nombres sont additionnés : `6`.

Pour passer de l'un à l'autre, on **convertit** :

- `int("3")` transforme le texte en nombre entier ;
- `float("3.5")` transforme le texte en nombre décimal ;
- `str(3)` transforme le nombre en texte.

```python
texte = "12"
nombre = int(texte)
print(nombre + 1)
print(str(nombre) + " ans")
```

```sortie
13
12 ans
```

C'est très utile avec `input`, qui lit le clavier : il donne **toujours** un
texte, même si l'utilisateur tape un nombre.

| Langage | Texte → nombre | Nombre → texte |
|---|---|---|
| Python | `int("3")` | `str(3)` |
| JavaScript | `Number("3")` | `String(3)` |
| Ruby | `"3".to_i` | `3.to_s` |

> **À retenir :** un nombre entre guillemets est un texte. Convertissez-le
> avant de calculer.

## Vrai ou faux

Une question fermée a deux réponses : oui ou non. En programmation, c'est
`True` (vrai) ou `False` (faux) : le type [booléen](glossaire.md#booleen),
`bool` en Python.

Une comparaison donne un booléen :

```python
age = 15
print(age >= 18)
print(age == 15)
```

```sortie
False
True
```

En JavaScript et en Ruby, on écrit `true` et `false` en minuscules. Les
booléens servent surtout dans les conditions : voyez [les
conditions](logique.md).

## Les listes

Une [liste](glossaire.md#liste) range plusieurs valeurs dans l'ordre, comme
une liste de courses. On l'écrit entre crochets `[ ]`, avec des virgules.

Chaque valeur a une **position**, qu'on appelle un
[indice](glossaire.md#indice). Attention : on compte à partir de **0**.

```text
fruits = ["pomme", "poire", "kiwi"]

   ┌─────────┬─────────┬─────────┐
   │ "pomme" │ "poire" │ "kiwi"  │
   └─────────┴─────────┴─────────┘
       0         1         2        ← les positions
```

Il y a 3 cases, numérotées 0, 1 et 2. La case 3 n'existe pas.

```python
fruits = ["pomme", "poire", "kiwi"]
print(fruits[0])
print(fruits[2])
print(len(fruits))
```

```sortie
pomme
kiwi
3
```

On ajoute à la fin avec `append`, et on change une case en lui donnant une
nouvelle valeur :

```python
fruits = ["pomme", "poire", "kiwi"]
fruits.append("fraise")
fruits[1] = "banane"
print(fruits)
```

```sortie
['pomme', 'banane', 'kiwi', 'fraise']
```

| Étape | Instruction | La liste après |
|---|---|---|
| 1 | départ | `pomme, poire, kiwi` |
| 2 | `append("fraise")` | `pomme, poire, kiwi, fraise` |
| 3 | `fruits[1] = "banane"` | `pomme, banane, kiwi, fraise` |

Essayez : affichez `fruits[-1]`. Python donne la **dernière** case :
`fraise`.

En JavaScript, c'est presque pareil :

```javascript
const fruits = ["pomme", "poire", "kiwi"];
fruits.push("fraise");
console.log(fruits[0], fruits.length);
```

```sortie
pomme 4
```

| Action | Python | JavaScript | Ruby |
|---|---|---|---|
| première case | `fruits[0]` | `fruits[0]` | `fruits[0]` |
| ajouter à la fin | `fruits.append(x)` | `fruits.push(x)` | `fruits.push(x)` |
| longueur | `len(fruits)` | `fruits.length` | `fruits.length` |

> **À retenir :** la première case d'une liste est la case **0**. La
> dernière est la case `longueur - 1`.

## Les dictionnaires

Un annuaire associe un **nom** à un **numéro**. On ne cherche pas « la
troisième ligne », on cherche « Ada ».

Un [dictionnaire](glossaire.md#dictionnaire) fait la même chose : il
associe des **clés** à des **valeurs**. On l'écrit entre accolades `{ }`,
avec `clé: valeur`.

```text
annuaire
   ┌───────┬──────────────────┐
   │ "Ada" │ "06 12 34 56 78" │
   │ "Léo" │ "07 98 76 54 32" │
   └───────┴──────────────────┘
     clé         valeur
```

```python
annuaire = {"Ada": "06 12 34 56 78", "Léo": "07 98 76 54 32"}
print(annuaire["Ada"])
annuaire["Zoé"] = "06 00 00 00 01"
print(len(annuaire))
print("Léo" in annuaire)
```

```sortie
06 12 34 56 78
3
True
```

- `annuaire["Ada"]` lit la valeur de la clé `"Ada"` ;
- `annuaire["Zoé"] = …` ajoute une nouvelle entrée ;
- `"Léo" in annuaire` demande : « cette clé existe-t-elle ? ».

En Ruby, on parle de **hash**, et on écrit `=>` entre la clé et la
valeur :

```ruby
annuaire = { "Ada" => "06 12 34 56 78" }
puts annuaire["Ada"]
```

```sortie
06 12 34 56 78
```

En JavaScript, on parle d'**objet** : `{ Ada: "06 12 34 56 78" }`.

> **À retenir :** une liste se lit par position (0, 1, 2…), un
> dictionnaire se lit par clé (`"Ada"`).

## « Rien » : None, null, nil

Parfois, il n'y a **pas de valeur** : une case vide, une personne absente
de l'annuaire. Les langages ont une valeur spéciale qui veut dire « rien »
(voir [null](glossaire.md#null)) :

| Langage | « Rien » |
|---|---|
| Python | `None` |
| JavaScript | `null` et `undefined` |
| Ruby | `nil` |

En Python, `get` cherche une clé et donne `None` si elle n'existe pas :

```python
annuaire = {"Ada": "06 12 34 56 78"}
print(annuaire.get("Léo"))
print(annuaire.get("Léo") is None)
```

```sortie
None
True
```

On teste « rien » avec `is None`. « Rien » n'est ni `0`, ni `""` : c'est
l'absence de valeur. On ne peut presque rien faire avec : ni calcul, ni
majuscules. Vérifiez donc d'abord qu'une valeur n'est pas « rien ».

## Erreurs fréquentes

### Additionner un texte et un nombre

```python norun
age = "12"
print(age + 1)
```

```text
Traceback (most recent call last):
  File "/home/ada/age.py", line 2, in <module>
    print(age + 1)
          ~~~~^~~
TypeError: can only concatenate str (not "int") to str
```

`age` est un texte (il a des guillemets). Python ne sait pas coller un
nombre à un texte. Correction : `int(age) + 1`, ou `age = 12` sans
guillemets.

### Lire une case qui n'existe pas

```python norun
fruits = ["pomme", "poire", "kiwi"]
print(fruits[3])
```

```text
Traceback (most recent call last):
  File "/home/ada/fruits.py", line 2, in <module>
    print(fruits[3])
          ~~~~~~^^^
IndexError: list index out of range
```

La liste a 3 cases : 0, 1 et 2. La case 3 est « hors de la liste » (*out of
range*). Correction : `fruits[2]`, ou `fruits[-1]` pour la dernière.

### Chercher une clé absente

```python norun
annuaire = {"Ada": "06 12 34 56 78"}
print(annuaire["Léo"])
```

```text
Traceback (most recent call last):
  File "/home/ada/annuaire.py", line 2, in <module>
    print(annuaire["Léo"])
          ~~~~~~~~^^^^^^^
KeyError: 'Léo'
```

`KeyError` veut dire « clé inconnue ». Correction : `annuaire.get("Léo")`,
qui donne `None` au lieu de s'arrêter.

## Testez-vous

**1.** Que vaut `"5" + "5"` en Python ? Et `5 + 5` ?

> **Réponse**
>
> `"5" + "5"` vaut `"55"` : deux textes collés. `5 + 5` vaut `10` : deux
> nombres additionnés.

**2.** Dans `notes = [12, 15, 9, 18]`, que donne `notes[1]` ? Et quelle est
la position de `18` ?

> **Réponse**
>
> `notes[1]` vaut `15`, car on compte à partir de 0. `18` est à la
> position 3.

**3.** Vous voulez ranger l'âge de chaque élève, retrouvé par son prénom.
Liste ou dictionnaire ?

> **Réponse**
>
> Un dictionnaire : la clé est le prénom, la valeur est l'âge.
> `ages = {"Ada": 12, "Léo": 11}`, puis `ages["Ada"]`.

## En résumé

- Chaque valeur a un type : il décide ce qu'on peut faire avec elle.
- Nombres entiers (`3`) et décimaux (`3.5`, avec un point) ; les décimaux sont parfois un peu faux, arrondissez.
- Le texte s'écrit entre guillemets ; `+` le colle, `len` le mesure.
- `int("3")` et `str(3)` passent du texte au nombre et inversement.
- Une liste se lit par position, à partir de 0 ; un dictionnaire se lit par clé.
- « Rien » s'écrit `None` en Python, `null` en JavaScript, `nil` en Ruby.

Suite : [Lire un message d'erreur sans paniquer](lire-une-erreur.md).
