# Suivre un programme pas à pas

Cette page vous apprend à exécuter un programme Python dans votre tête, ligne par ligne, avec un tableau qui suit la valeur de chaque variable, sur cinq petits programmes de plus en plus riches.

> **Objectifs :**
>
> - lire un programme dans l'ordre où Python l'exécute ;
> - remplir un tableau de suivi (une colonne par variable, une ligne par étape) ;
> - deviner la sortie d'un programme **avant** de le lancer ;
> - suivre un appel de fonction : qui appelle qui, et ce qui revient ;
> - faire afficher au programme ses propres étapes avec `print`.

## Pourquoi suivre un programme à la main ?

Un ordinateur ne devine rien. Il lit vos lignes **une par une**, de haut en bas, et fait
exactement ce qui est écrit. Si vous savez faire pareil dans votre tête, vous comprenez
tout programme, et vous trouvez vos [bugs](../debuter/glossaire.md#bug) beaucoup plus vite.

Pensez à une recette de cuisine. Un bon cuisinier ne lit pas la recette « en gros ». Il
suit les étapes dans l'ordre, et il regarde ce qu'il y a dans chaque bol après chaque
étape. Ici, les bols sont les [variables](../debuter/glossaire.md#variable).

```text
   la recette (le code)          les bols (les variables)
   ┌────────────────────┐        ┌─────┐  ┌─────┐
   │ 1. a = 5           │  ───►  │ a 5 │  │ b ? │
   │ 2. b = 3           │        └─────┘  └─────┘
   │ 3. a = a + b       │
   └────────────────────┘
        ▲
        └── le doigt : la ligne qu'on exécute maintenant
```

> **À retenir :** suivre un programme, c'est avancer un doigt sur le code, une ligne à
> la fois, et noter ce que contient chaque variable après chaque ligne.

## Le tableau de suivi

L'outil de base est un tableau. On l'appelle souvent un **tableau de suivi** (en
anglais, une *trace*).

- Une **colonne par variable**, plus une colonne « Ligne » et une colonne « Affiche ».
- Une **ligne du tableau par étape** : chaque fois que Python exécute une ligne du code.
- Dans chaque case, la valeur **après** l'étape. Une case vide veut dire « pas changé ».

Une feuille de papier et un crayon suffisent. Écrire à la main vous force à ne sauter
aucune étape.

Pour suivre facilement, nous écrirons chaque programme deux fois : d'abord avec des
numéros de ligne (`L1`, `L2`…), pour le tableau ; puis en vrai, pour le lancer.

> **À retenir :** une colonne par variable, une ligne par étape, et toujours la valeur
> **après** l'étape.

## Programme 1 : des variables qui changent

Le premier programme n'a ni condition ni boucle. Seulement des variables qui changent.

```text
L1  a = 5
L2  b = 3
L3  a = a + b
L4  b = a - b
L5  print(a, b)
```

**Avant de lancer, devinez ce qui s'affiche.** Écrivez votre réponse sur un papier.

Voici le suivi. À la ligne 3, Python calcule d'abord la partie **à droite** du `=` avec
les valeurs actuelles, puis range le résultat dans la boîte de gauche.

| Étape | Ligne | Calcul | `a` | `b` | Affiche |
|---|---|---|---|---|---|
| 1 | L1 | | 5 | | |
| 2 | L2 | | 5 | 3 | |
| 3 | L3 | `5 + 3` | 8 | 3 | |
| 4 | L4 | `8 - 3` | 8 | 5 | |
| 5 | L5 | | 8 | 5 | `8 5` |

Le piège est à l'étape 4 : `a` ne vaut plus 5, mais 8. Il faut utiliser la valeur
**du moment**, pas celle du début.

Lancez-le pour vérifier :

```python
a = 5
b = 3
a = a + b
b = a - b
print(a, b)
```

```sortie
8 5
```

Essayez : ajoutez une ligne `a = a - b` juste avant le `print`. Suivez à la main :
`a` devient `8 - 5`, donc `3`. Le programme affiche `3 5` : les valeurs de départ ont été
échangées !

> **À retenir :** dans `a = a + b`, la droite est calculée d'abord, avec les valeurs du
> moment. Ensuite seulement, `a` change.

## Programme 2 : une condition

Ajoutons une [condition](../debuter/glossaire.md#condition). Avec un `if`, certaines
lignes sont **sautées**. Le tableau doit le montrer.

```text
L1  temperature = 12
L2  pluie = True
L3  if temperature > 20:
L4      conseil = "t-shirt"
L5  elif pluie:
L6      conseil = "parapluie"
L7  else:
L8      conseil = "pull"
L9  print(conseil)
```

**Avant de lancer, devinez ce qui s'affiche.**

Pour une condition, ajoutez une colonne « Test » : la question posée et sa réponse,
`True` (vrai) ou `False` (faux).

| Étape | Ligne | Test | `temperature` | `pluie` | `conseil` | Affiche |
|---|---|---|---|---|---|---|
| 1 | L1 | | 12 | | | |
| 2 | L2 | | 12 | True | | |
| 3 | L3 | `12 > 20` → False | | | | |
| 4 | L5 | `pluie` → True | | | | |
| 5 | L6 | | | | `"parapluie"` | |
| 6 | L9 | | | | | `parapluie` |

Remarquez les lignes **absentes** du tableau : L4, L7 et L8. Python ne les exécute pas.
C'est comme un aiguillage de train : une seule voie est prise.

```text
            temperature > 20 ?
              /          \
           oui            non
            │               │
        "t-shirt"       pluie ?
                        /     \
                      oui      non
                       │        │
                "parapluie"   "pull"
```

```python
temperature = 12
pluie = True
if temperature > 20:
    conseil = "t-shirt"
elif pluie:
    conseil = "parapluie"
else:
    conseil = "pull"
print(conseil)
```

```sortie
parapluie
```

Essayez : mettez `temperature = 25`. Le premier test devient vrai : Python prend la
première voie, affiche `t-shirt`, et ne regarde même pas `pluie`.

> **À retenir :** dans un `if` / `elif` / `else`, Python teste dans l'ordre et prend
> **la première** voie vraie. Les autres lignes n'apparaissent pas dans le tableau.

## Programme 3 : une boucle qui accumule

Avec une [boucle](../debuter/glossaire.md#boucle), les mêmes lignes reviennent plusieurs
fois. Dans le tableau, elles apparaissent donc plusieurs fois, une par tour.

```text
L1  total = 0
L2  for n in range(1, 5):
L3      total = total + n * 2
L4  print(total)
```

**Avant de lancer, devinez ce qui s'affiche.** Rappel : `range(1, 5)` donne 1, 2, 3, 4
(le 5 est exclu).

`total` est une tirelire. Elle commence vide (0), et chaque tour y glisse une pièce.

| Étape | Ligne | `n` | Calcul | `total` | Affiche |
|---|---|---|---|---|---|
| 1 | L1 | | | 0 | |
| 2 | L2 | 1 | | 0 | |
| 3 | L3 | 1 | `0 + 1 * 2` | 2 | |
| 4 | L2 | 2 | | 2 | |
| 5 | L3 | 2 | `2 + 2 * 2` | 6 | |
| 6 | L2 | 3 | | 6 | |
| 7 | L3 | 3 | `6 + 3 * 2` | 12 | |
| 8 | L2 | 4 | | 12 | |
| 9 | L3 | 4 | `12 + 4 * 2` | 20 | |
| 10 | L2 | fin | | 20 | |
| 11 | L4 | | | 20 | `20` |

À l'étape 10, il n'y a plus de nombre dans `range(1, 5)` : la boucle se termine, et
Python passe à la ligne qui suit le bloc indenté.

```python
total = 0
for n in range(1, 5):
    total = total + n * 2
print(total)
```

```sortie
20
```

Une fois à l'aise, vous pouvez faire un tableau plus court : **une ligne par tour**.

| Tour | `n` | `total` avant | `total` après |
|---|---|---|---|
| 1 | 1 | 0 | 2 |
| 2 | 2 | 2 | 6 |
| 3 | 3 | 6 | 12 |
| 4 | 4 | 12 | 20 |

Essayez : décalez `print(total)` de 4 espaces, pour le mettre **dans** la boucle. Il
s'exécute alors à chaque tour : le programme affiche `2`, `6`, `12` puis `20`.

### Le même avec while

Une boucle `while` teste sa condition **avant chaque tour**. Mettez ce test dans le
tableau.

```python
argent = 10
jours = 0
while argent > 0:
    argent = argent - 3
    jours = jours + 1
print(jours, argent)
```

```sortie
4 -2
```

| Tour | Test `argent > 0` | `argent` après | `jours` après |
|---|---|---|---|
| 1 | `10 > 0` → True | 7 | 1 |
| 2 | `7 > 0` → True | 4 | 2 |
| 3 | `4 > 0` → True | 1 | 3 |
| 4 | `1 > 0` → True | -2 | 4 |
| — | `-2 > 0` → False | fin | |

Beaucoup de gens devinent `3 1`. Le tableau montre pourquoi c'est faux : quand il reste
1, le test `1 > 0` est encore vrai, donc un quatrième tour a lieu.

> **À retenir :** dans une boucle, on note une ligne par tour. La valeur « après » d'un
> tour devient la valeur « avant » du tour suivant.

## Programme 4 : une fonction appelée deux fois

Une [fonction](../debuter/glossaire.md#fonction) est une petite machine : on lui donne
quelque chose, elle travaille dans son coin, puis elle **renvoie** un résultat. Quand
Python rencontre un appel, il **saute** dans la fonction, puis **revient** à l'endroit
de l'appel.

```text
L1  def prix_ttc(prix_ht):
L2      taxe = prix_ht * 20 // 100
L3      return prix_ht + taxe
L4
L5  a = prix_ttc(50)
L6  b = prix_ttc(a)
L7  print(a, b)
```

**Avant de lancer, devinez ce qui s'affiche.**

Première chose : les lignes L1 à L3 **définissent** la fonction, elles ne l'exécutent
pas. C'est comme écrire une recette dans un carnet : on ne cuisine pas encore. Le vrai
travail commence à L5.

Voici qui appelle qui, et ce qui revient :

```text
  programme principal              prix_ttc
  ───────────────────              ────────
  L5  a = prix_ttc(50)  ── 50 ──►  prix_ht = 50
                                   taxe = 10
      a = 60            ◄── 60 ──  return 60

  L6  b = prix_ttc(a)   ── 60 ──►  prix_ht = 60
                                   taxe = 12
      b = 72            ◄── 72 ──  return 72

  L7  print(a, b)  →  60 72
```

Le tableau ajoute une colonne « Où ? » pour savoir si l'on est dans la fonction ou
dehors.

| Étape | Où ? | Ligne | `prix_ht` | `taxe` | `a` | `b` | Affiche |
|---|---|---|---|---|---|---|---|
| 1 | dehors | L5 | | | ? | | |
| 2 | fonction | L1 | 50 | | | | |
| 3 | fonction | L2 | 50 | 10 | | | |
| 4 | fonction | L3 | 50 | 10 | | | renvoie 60 |
| 5 | dehors | L5 | | | 60 | | |
| 6 | dehors | L6 | | | 60 | ? | |
| 7 | fonction | L1 | 60 | | | | |
| 8 | fonction | L2 | 60 | 12 | | | |
| 9 | fonction | L3 | 60 | 12 | | | renvoie 72 |
| 10 | dehors | L6 | | | 60 | 72 | |
| 11 | dehors | L7 | | | 60 | 72 | `60 72` |

Deux détails importants :

- `prix_ht` et `taxe` n'existent que **pendant** l'appel. À chaque nouvel appel, ils
  repartent de zéro. On dit que ce sont des variables locales (voir
  [la portée](../debuter/glossaire.md#portee)).
- Au deuxième appel, le [paramètre](../debuter/glossaire.md#parametre) `prix_ht` reçoit
  la valeur de `a`, c'est-à-dire 60.

```python
def prix_ttc(prix_ht):
    taxe = prix_ht * 20 // 100
    return prix_ht + taxe

a = prix_ttc(50)
b = prix_ttc(a)
print(a, b)
```

```sortie
60 72
```

Essayez : remplacez `return prix_ht + taxe` par `print(prix_ht + taxe)`. La fonction
affiche `60`, puis ne renvoie rien : `a` vaut `None`, et le deuxième appel plante. Voir
[les erreurs fréquentes](#erreurs-frequentes) plus bas.

> **À retenir :** à un appel, le doigt saute dans la fonction ; au `return`, il revient
> à la ligne de l'appel avec la valeur renvoyée.

## Programme 5 : une liste et une condition dans la boucle

Dernier programme : une boucle sur une [liste](../debuter/glossaire.md#liste), avec un
`if` à l'intérieur. Tout ce que vous avez vu se combine.

```text
L1  notes = [12, 7, 15, 9, 18]
L2  reussites = []
L3  for note in notes:
L4      if note >= 10:
L5          reussites.append(note)
L6      else:
L7          print(f"{note} : à retravailler")
L8  print(reussites)
```

**Avant de lancer, devinez ce qui s'affiche.** Attention : il y a des `print` **dans**
la boucle et un `print` **après**.

| Tour | `note` | Test `note >= 10` | Action | `reussites` après | Affiche |
|---|---|---|---|---|---|
| 1 | 12 | True | `append(12)` | `[12]` | |
| 2 | 7 | False | `print` | `[12]` | `7 : à retravailler` |
| 3 | 15 | True | `append(15)` | `[12, 15]` | |
| 4 | 9 | False | `print` | `[12, 15]` | `9 : à retravailler` |
| 5 | 18 | True | `append(18)` | `[12, 15, 18]` | |
| après | | | L8 | `[12, 15, 18]` | `[12, 15, 18]` |

Imaginez un tri de courrier : chaque lettre passe devant vous, une à la fois. Les
bonnes vont dans le panier `reussites`, les autres reçoivent un mot tout de suite.

```text
  notes :  12   7   15   9   18
           │    │   │    │   │
           ▼    ▼   ▼    ▼   ▼
         >=10 ? ── oui ──►  panier : [12, 15, 18]
                └─ non ──►  print « à retravailler »
```

```python
notes = [12, 7, 15, 9, 18]
reussites = []
for note in notes:
    if note >= 10:
        reussites.append(note)
    else:
        print(f"{note} : à retravailler")
print(reussites)
```

```sortie
7 : à retravailler
9 : à retravailler
[12, 15, 18]
```

Les messages sur 7 et 9 sortent **avant** la liste : ils sont affichés pendant la
boucle, alors que la liste n'est affichée qu'à la fin.

Essayez : changez le test en `note > 12`. Suivez à la main avant de lancer : 12 ne passe
plus, donc le programme affiche aussi `12 : à retravailler`, et la liste devient
`[15, 18]`.

> **À retenir :** dans une boucle avec un `if`, chaque tour choisit sa voie. Notez pour
> chaque tour le test, sa réponse, et ce qui change.

## Faire afficher les étapes au programme

Parfois, le programme est trop long pour un tableau sur papier. Alors, demandez à Python
de remplir le tableau lui-même : ajoutez un `print` dans la boucle, avec des
**étiquettes** pour savoir qui est qui.

```python
total = 0
for n in range(1, 5):
    total = total + n * 2
    print(f"tour n={n} : total={total}")
print("fin :", total)
```

```sortie
tour n=1 : total=2
tour n=2 : total=6
tour n=3 : total=12
tour n=4 : total=20
fin : 20
```

C'est exactement le tableau du programme 3, écrit par Python.

Python a un raccourci pratique : dans une f-string, `{x=}` affiche le nom **et** la
valeur.

```python
def prix_ttc(prix_ht):
    taxe = prix_ht * 20 // 100
    print(f"  dans prix_ttc : {prix_ht=} {taxe=}")
    return prix_ht + taxe

a = prix_ttc(50)
print(f"{a=}")
b = prix_ttc(a)
print(f"{b=}")
```

```sortie
  dans prix_ttc : prix_ht=50 taxe=10
a=60
  dans prix_ttc : prix_ht=60 taxe=12
b=72
```

Les deux espaces au début des lignes « dans prix_ttc » montrent qu'on est **dans** la
fonction. On voit l'aller-retour du dessin du programme 4.

Essayez : ajoutez `print(f"{note=}")` comme première ligne de la boucle du programme 5.
Vous verrez passer chaque note, une par ligne, avant les autres messages de ce tour.

> **Astuce :** quand le bug est trouvé, retirez ces `print`. Ils servent à enquêter,
> pas à rester dans le programme final.

> **À retenir :** un `print` avec des étiquettes, placé dans la boucle ou dans la
> fonction, transforme le programme en tableau de suivi automatique.

## Et le débogueur ?

Python a aussi un [débogueur](../debuter/glossaire.md#debogage), `pdb` : écrivez
`breakpoint()` dans votre code, et le programme s'arrête là pour vous laisser regarder
les variables (`p total`) puis avancer d'une ligne (`n`). C'est votre tableau de suivi,
mais en direct ; tout est expliqué dans [Déboguer en Python](debogage.md).

> **À retenir :** papier, `print` ou débogueur : c'est toujours la même idée, regarder
> les variables une étape après l'autre.

## Erreurs fréquentes

Ces erreurs viennent souvent d'une étape mal suivie. Le tableau de suivi aide à les
voir **avant** de lancer.

### Faire un tour de trop

En suivant, on oublie que `range(n)` s'arrête **avant** `n`. Ici, on fait un tour de
trop :

```python norun
notes = [12, 7, 15]
for i in range(len(notes) + 1):
    print(notes[i])
```

```text
12
7
15
Traceback (most recent call last):
  File "main.py", line 3, in <module>
    print(notes[i])
          ~~~~~^^^
IndexError: list index out of range
```

Le tableau montre le problème : au tour 4, `i` vaut 3, mais la liste n'a que les cases
0, 1 et 2.

| Tour | `i` | `notes[i]` |
|---|---|---|
| 1 | 0 | 12 |
| 2 | 1 | 7 |
| 3 | 2 | 15 |
| 4 | 3 | il n'y a pas de case 3 ! |

La correction : parcourez directement la liste, sans compter.

```python
notes = [12, 7, 15]
for note in notes:
    print(note)
```

```sortie
12
7
15
```

### Utiliser une variable qui n'a pas encore de valeur

Au premier tour, on lit `total`. Mais aucune ligne au-dessus ne lui a donné de valeur :
la première case de la colonne `total` est vide.

```python norun
for n in [3, 4, 5]:
    total = total + n
print(total)
```

```text
Traceback (most recent call last):
  File "main.py", line 2, in <module>
    total = total + n
            ^^^^^
NameError: name 'total' is not defined
```

La correction : remplissez la tirelire **avant** la boucle.

```python
total = 0
for n in [3, 4, 5]:
    total = total + n
print(total)
```

```sortie
12
```

### Croire qu'une fonction renvoie ce qu'elle affiche

En suivant, on note « `resultat` vaut 6 » parce que la fonction affiche 6. Mais la
fonction n'a pas de `return` : elle renvoie `None`.

```python norun
def double(x):
    print(x * 2)

resultat = double(3)
print(resultat + 1)
```

```text
6
Traceback (most recent call last):
  File "main.py", line 5, in <module>
    print(resultat + 1)
          ~~~~~~~~~^~~
TypeError: unsupported operand type(s) for +: 'NoneType' and 'int'
```

Dans le dessin d'appel, la flèche du retour porte `None`, pas 6. La correction :

```python
def double(x):
    return x * 2

resultat = double(3)
print(resultat + 1)
```

```sortie
7
```

### Reprendre une ancienne valeur

Cette erreur ne fait pas planter Python : elle se passe dans **votre** tête. Au
programme 1, beaucoup calculent `b = a - b` avec `a = 5` et trouvent `2`. Mais à cette
étape, `a` vaut déjà 8. Règle simple : pour chaque calcul, lisez la valeur dans la
**dernière** case remplie de la colonne, jamais dans la première.

## Testez-vous

**Question 1 :** que vaut `x` à la fin ?

```python
x = 2
x = x * 3
x = x - 1
print(x)
```

```sortie
5
```

> **Réponse**
>
> | Étape | `x` |
> |---|---|
> | `x = 2` | 2 |
> | `x = x * 3` | 6 |
> | `x = x - 1` | 5 |
>
> Le programme affiche `5`.

**Question 2 :** combien de fois le mot `tour` s'affiche-t-il, et que vaut `n` à la
fin ?

```python
n = 1
while n < 20:
    print("tour")
    n = n * 3
print(n)
```

```sortie
tour
tour
tour
27
```

> **Réponse**
>
> | Test `n < 20` | `n` après le tour |
> |---|---|
> | `1 < 20` → True | 3 |
> | `3 < 20` → True | 9 |
> | `9 < 20` → True | 27 |
> | `27 < 20` → False | fin |
>
> Trois tours, donc trois fois `tour`, puis `27`.

**Question 3 :** qu'affiche ce programme ?

```python
def plus_un(n):
    return n + 1

a = plus_un(1)
b = plus_un(plus_un(a))
print(a, b)
```

```sortie
2 4
```

> **Réponse**
>
> Premier appel : `plus_un(1)` renvoie 2, donc `a = 2`. Pour `b`, Python calcule
> d'abord l'appel du **dedans** : `plus_un(2)` renvoie 3. Puis l'appel du dehors :
> `plus_un(3)` renvoie 4. Le programme affiche `2 4`.

## En résumé

- Python exécute les lignes une par une, de haut en bas ; suivez-les avec le doigt.
- Un tableau de suivi a une colonne par variable et une ligne par étape (ou par tour de
  boucle).
- Dans `x = x + 1`, la droite est calculée d'abord, avec la valeur **du moment**.
- Une condition saute des lignes ; une boucle répète des lignes ; un appel de fonction
  saute dans la fonction puis revient avec la valeur du `return`.
- Devinez la sortie avant de lancer, puis comparez : c'est le meilleur entraînement.
- `print(f"{x=}")` fait écrire le tableau par Python ; `breakpoint()` met le programme
  en pause.

Suite : [Les pièges du débutant en Python](pieges.md)
