# Premiers pas

Cette page présente les briques de base de Python : les variables, les types de valeurs simples, l'affichage avec `print` et les f-strings.

> **Objectifs :**
>
> - vous saurez créer une variable et changer sa valeur ;
> - vous saurez reconnaître les types `int`, `float`, `str`, `bool` et `None` ;
> - vous saurez afficher des valeurs avec `print` et les f-strings ;
> - vous saurez écrire des commentaires et bien nommer vos variables ;
> - vous saurez lire ce que tape l'utilisateur avec `input`.

## Les variables

Une [variable](../debuter/glossaire.md#variable) est un nom posé sur une
[valeur](../debuter/glossaire.md#valeur). Imaginez une étiquette collée sur une boîte :
l'étiquette, c'est le nom ; le contenu de la boîte, c'est la valeur.

On crée une variable avec le signe `=`, qui veut dire « reçoit » (et non « est égal à ») :

```python
prenom = "Alice"
age = 30
print(prenom)
print(age)
```

```sortie
Alice
30
```

Une variable peut recevoir une nouvelle valeur. L'ancienne est oubliée.

```python
score = 10
print(score)
score = 25
print(score)
score = score + 5
print(score)
```

```sortie
10
25
30
```

La ligne `score = score + 5` se lit de droite à gauche : Python calcule `score + 5`
(soit `30`), puis range le résultat dans `score`.

> **Astuce :** `score += 5` est un raccourci pour `score = score + 5`. Il existe aussi
> `-=`, `*=` et `/=`.

## Les types de valeurs

Chaque valeur a un [type](../debuter/glossaire.md#type) : sa nature. Un nombre ne se
manipule pas comme un texte. Voici les cinq types de base.

| Type | Nom complet | Exemples | Sert à |
| --- | --- | --- | --- |
| `int` | *integer*, entier | `42`, `-7`, `0` | compter |
| `float` | nombre à virgule | `3.14`, `-0.5`, `2.0` | mesurer |
| `str` | *string*, texte | `"chat"`, `'Bonjour'` | écrire |
| `bool` | booléen | `True`, `False` | répondre oui ou non |
| `NoneType` | rien | `None` | dire « pas de valeur » |

```python
nombre = 42
prix = 9.99
nom = "Python"
est_actif = True
resultat = None

print(nombre, prix, nom, est_actif, resultat)
```

```sortie
42 9.99 Python True None
```

Quelques précisions :

- Un [entier](../debuter/glossaire.md#entier) n'a pas de virgule. Un
  [flottant](../debuter/glossaire.md#flottant) s'écrit avec un **point** : `3.14`, pas
  `3,14`.
- Une [chaîne de caractères](../debuter/glossaire.md#chaine-de-caracteres) (`str`) se
  met entre guillemets doubles `"..."` ou simples `'...'`. Les deux se valent.
- Un [booléen](../debuter/glossaire.md#booleen) vaut `True` (vrai) ou `False` (faux),
  avec une majuscule.
- `None` veut dire « rien, pas de valeur pour l'instant ». C'est l'équivalent de
  [null](../debuter/glossaire.md#null) dans d'autres langages.

## Connaître le type d'une valeur

La fonction `type()` donne le type d'une valeur. Elle est très utile dans la console
pour comprendre ce que vous manipulez.

```python
print(type(42))
print(type(3.14))
print(type("bonjour"))
print(type(True))
print(type(None))
print(type("42"))
```

```sortie
<class 'int'>
<class 'float'>
<class 'str'>
<class 'bool'>
<class 'NoneType'>
<class 'str'>
```

Regardez la dernière ligne : `"42"` est un **texte**, pas un nombre. Les guillemets
changent tout.

## Le typage dynamique

En Python, une variable n'a pas de type fixé. C'est la **valeur** qui a un type. La
même étiquette peut passer d'un nombre à un texte : on dit que le typage est
**dynamique**.

```python
x = 5
print(type(x))
x = "cinq"
print(type(x))
```

```sortie
<class 'int'>
<class 'str'>
```

C'est pratique, mais attention : Python ne vous préviendra pas si vous mélangez tout
par erreur. Gardez un seul type par variable, c'est plus clair.

> **Note :** d'autres langages, comme [Rust](../rust/premiers-pas.md), fixent le type
> d'une variable une fois pour toutes. On parle de typage **statique**.

## Afficher avec print

`print` affiche une ou plusieurs valeurs. Séparées par des virgules, elles sont
affichées avec un espace entre elles, puis un retour à la ligne.

```python
print("Il y a", 3, "chats")
print()
print("fin")
```

```sortie
Il y a 3 chats

fin
```

`print()` sans rien affiche une ligne vide.

### Les options sep et end

Deux options changent ce comportement :

- `sep` (*separator*) : ce qui sépare les valeurs (un espace par défaut) ;
- `end` : ce qui est ajouté à la fin (un retour à la ligne par défaut).

```python
print("2026", "10", "18", sep="-")
print("a", "b", "c", sep="")
print("Chargement", end="...")
print("terminé")
```

```sortie
2026-10-18
abc
Chargement...terminé
```

Avec `end="..."`, le `print` suivant continue sur la même ligne.

## Les f-strings

Pour mettre des valeurs **dans** un texte, utilisez une **f-string** : un texte précédé
de la lettre `f`. Entre accolades `{ }`, vous écrivez une variable ou un calcul.

```python
prenom = "Alice"
age = 30
print(f"Bonjour, {prenom} !")
print(f"{prenom} a {age} ans.")
print(f"Dans 10 ans, elle aura {age + 10} ans.")
```

```sortie
Bonjour, Alice !
Alice a 30 ans.
Dans 10 ans, elle aura 40 ans.
```

C'est comme un formulaire à trous : Python remplit chaque `{ }` avec la valeur.

> **Astuce :** écrivez `{variable=}` pour afficher le nom **et** la valeur. Très pratique
> pour déboguer : `print(f"{age=}")` affiche `age=30`.

```python
age = 30
print(f"{age=}")
```

```sortie
age=30
```

## Les commentaires

Un [commentaire](../debuter/glossaire.md#commentaire) est une note pour les humains.
Python l'ignore. Il commence par `#` et va jusqu'à la fin de la ligne.

```python
# Calcul du prix total
prix = 12  # prix d'un article, en euros
quantite = 3
print(prix * quantite)  # affiche le total
```

```sortie
36
```

> **À retenir :** un bon commentaire explique **pourquoi**, pas **quoi**. Le code dit
> déjà ce qu'il fait.

## Bien nommer ses variables

Un nom de variable :

- contient des lettres, des chiffres et `_`, mais ne commence pas par un chiffre ;
- distingue majuscules et minuscules : `age` et `Age` sont deux noms différents ;
- ne peut pas être un mot réservé de Python (`if`, `for`, `def`, `class`, `True`...).

En Python, la convention est le **snake_case** : tout en minuscules, les mots séparés
par `_`, comme un serpent qui rampe au sol.

```python
nombre_de_joueurs = 4
prix_total_ttc = 59.90
est_connecte = False
print(nombre_de_joueurs, prix_total_ttc, est_connecte)
```

```sortie
4 59.9 False
```

Choisissez des noms qui disent ce que contient la variable. `nombre_de_joueurs` est
bien plus clair que `n` ou `x`.

> **Note :** les accents sont permis (`prénom = "Alice"`), mais l'usage est d'écrire les
> noms sans accents, en français ou en anglais.

## Les constantes

Une [constante](../debuter/glossaire.md#constante) est une valeur qui ne doit pas
changer : un taux, une limite, un nom d'application. Python n'a pas de vraies
constantes. Par **convention**, on les écrit en MAJUSCULES : c'est un message aux
autres programmeurs, « ne touchez pas à cette valeur ».

```python
TAUX_TVA = 0.2
SCORE_MAX = 100

prix_ht = 50
print(prix_ht * (1 + TAUX_TVA))
print(f"Score maximum : {SCORE_MAX}")
```

```sortie
60.0
Score maximum : 100
```

## Lire ce que tape l'utilisateur

`input()` affiche une question, attend que l'utilisateur tape une réponse puis appuie
sur Entrée, et renvoie cette réponse.

```python norun
prenom = input("Comment vous appelez-vous ? ")
print(f"Enchanté, {prenom} !")
```

```text
Comment vous appelez-vous ? Alice
Enchanté, Alice !
```

`input` renvoie **toujours** un texte (`str`), même si l'utilisateur tape un nombre.
Pour calculer avec, convertissez-le avec `int()` :

```python norun
age = int(input("Quel âge avez-vous ? "))
print(f"L'an prochain, vous aurez {age + 1} ans.")
```

```text
Quel âge avez-vous ? 29
L'an prochain, vous aurez 30 ans.
```

Les conversions sont détaillées dans [nombres et calculs](nombres.md).

## Erreurs fréquentes

### Coller un texte et un nombre avec +

```python norun
age = 30
print("Âge : " + age)
```

```text
Traceback (most recent call last):
  File "/home/vous/main.py", line 2, in <module>
    print("Âge : " + age)
          ~~~~~~~~~^~~~~
TypeError: can only concatenate str (not "int") to str
```

Python refuse d'additionner un texte et un nombre (« on ne peut coller qu'un `str` à un
`str` »). Utilisez une f-string : `print(f"Âge : {age}")`. Ou convertissez :
`"Âge : " + str(age)`.

### Utiliser une variable avant de la créer

```python norun
print(prenom)
prenom = "Alice"
```

```text
Traceback (most recent call last):
  File "/home/vous/main.py", line 1, in <module>
    print(prenom)
          ^^^^^^
NameError: name 'prenom' is not defined
```

Python lit le fichier de haut en bas. À la ligne 1, `prenom` n'existe pas encore.
Créez la variable avant de l'utiliser. Vérifiez aussi l'orthographe : `Prenom` ou
`prénom` seraient d'autres noms.

### Calculer avec le résultat d'input

```python norun
age = input("Âge ? ")
print(age + 1)
```

```text
Âge ? 20
Traceback (most recent call last):
  File "/home/vous/main.py", line 2, in <module>
    print(age + 1)
          ~~~~^~~
TypeError: can only concatenate str (not "int") to str
```

`input` renvoie le texte `"20"`, pas le nombre `20`. Convertissez-le :
`age = int(input("Âge ? "))`.

### Oublier le f devant la chaîne

```python norun
prenom = "Alice"
print("Bonjour, {prenom} !")
```

```text
Bonjour, {prenom} !
```

Pas d'erreur, mais les accolades sont affichées telles quelles. Sans `f` devant, c'est
un texte ordinaire. Écrivez `print(f"Bonjour, {prenom} !")`.

## Testez-vous

**Question 1 :** qu'affiche ce code ?

```python norun
a = 5
b = a
a = 8
print(a, b)
```

> **Réponse**
>
> `8 5`. La ligne `b = a` donne à `b` la valeur de `a` à ce moment-là (`5`). Changer
> `a` ensuite ne change pas `b`.

**Question 2 :** quel est le type de `"3.5"` ? Et de `3.5` ?

> **Réponse**
>
> `"3.5"` est un `str` (un texte, à cause des guillemets). `3.5` est un `float`.

**Question 3 :** écrivez un programme qui range `"Paris"` dans une variable et affiche
`Je vis à Paris.` avec une f-string.

> **Réponse**
>
> ```python
> ville = "Paris"
> print(f"Je vis à {ville}.")
> ```
>
> ```sortie
> Je vis à Paris.
> ```

## En résumé

- `nom = valeur` crée une variable ; on peut lui donner une nouvelle valeur ensuite.
- Les types de base sont `int`, `float`, `str`, `bool` et `None` ; `type()` les révèle.
- `print` affiche ses valeurs séparées par `sep` et terminées par `end`.
- Une f-string (`f"... {x} ..."`) insère des valeurs dans un texte.
- Les noms s'écrivent en snake_case ; les constantes en MAJUSCULES.
- `input()` renvoie toujours un texte : convertissez-le avant de calculer.

Suite : [nombres et calculs](nombres.md).
