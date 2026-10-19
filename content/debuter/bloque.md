# Quand on bloque : une méthode pas à pas

Être bloqué devant un problème arrive à tout le monde : cette page donne une méthode simple, étape par étape, pour avancer quand même, avec un exemple complet en Python.

> **Objectifs :**
>
> - savoir qu'être bloqué est normal ;
> - reformuler un problème avec vos propres mots ;
> - résoudre un petit exemple à la main, sur papier ;
> - découper un problème et l'écrire une étape à la fois ;
> - utiliser le canard en plastique et la pause ;
> - demander de l'aide de façon efficace.

## Être bloqué, c'est normal

Vous lisez l'énoncé. Vous ne savez pas par où commencer. Vous fixez l'écran
vide. Vous vous dites : « je ne suis pas fait pour ça ».

C'est faux. **Tout le monde** bloque, même après vingt ans de métier. Bloquer
ne veut pas dire que vous êtes nul : cela veut dire que vous êtes en train
d'apprendre quelque chose de nouveau.

La différence entre un débutant et quelqu'un d'expérimenté, ce n'est pas
qu'il ne bloque jamais. C'est qu'il a une **méthode** pour se débloquer. La
voici, en six étapes :

```text
  1. Reformuler        → dire le problème avec mes mots
  2. Faire à la main   → un tout petit exemple, sur papier
  3. Découper          → des étapes en français (pseudo-code)
  4. Coder pas à pas   → une étape, je lance, je vérifie
  5. Expliquer         → au canard en plastique
  6. Faire une pause   → puis demander de l'aide
```

> **À retenir :** bloquer fait partie du travail. Ce qui compte, c'est
> d'avoir une méthode.

## Étape 1 : reformuler avec vos mots

Un énoncé est souvent écrit par quelqu'un d'autre, avec ses mots à lui.
Réécrivez-le avec **les vôtres**, en une ou deux phrases simples.

Prenons cet énoncé :

```text
Écrire une fonction qui détermine le nombre de voyelles d'une chaîne.
```

Reformulé :

```text
Je reçois un mot. Je dois dire combien il a de voyelles.
Les voyelles sont : a, e, i, o, u, y.
```

Posez-vous trois questions :

- **Qu'est-ce que je reçois ?** Un mot (un texte).
- **Qu'est-ce que je dois donner ?** Un nombre.
- **Qu'est-ce qui n'est pas clair ?** Les majuscules comptent-elles ? Oui :
  `"A"` est une voyelle aussi.

Si vous n'arrivez pas à reformuler, le problème n'est pas le code : c'est
la compréhension de l'énoncé. Relisez-le, ou demandez une précision.

## Étape 2 : résoudre un tout petit exemple à la main

Prenez un crayon. Choisissez un mot court : `banane`. Comptez ses voyelles,
**lentement**, en notant tout ce que vous faites.

| Lettre | Voyelle ? | Compteur |
|---|---|---|
| (départ) | | 0 |
| b | non | 0 |
| a | oui | 1 |
| n | non | 1 |
| a | oui | 2 |
| n | non | 2 |
| e | oui | 3 |

Réponse : **3**.

Maintenant, regardez ce que vous avez fait. Vous avez :

1. commencé un compteur à 0 ;
2. regardé chaque lettre, une par une ;
3. ajouté 1 au compteur quand c'était une voyelle ;
4. donné le compteur à la fin.

Vous venez de trouver l'[algorithme](glossaire.md#algorithme). Votre cerveau
le connaissait déjà : il fallait juste le **ralentir** pour le voir.

> **À retenir :** si vous savez le faire à la main, vous pouvez
> l'expliquer à l'ordinateur.

## Étape 3 : découper en pseudo-code

On écrit ces étapes en français, avec la forme d'un programme. C'est le
[pseudo-code](glossaire.md#pseudo-code) (voir [Penser comme un
programmeur](algorithmes.md)) :

```text
compteur ← 0
pour chaque lettre du mot
    si la lettre est une voyelle
        compteur ← compteur + 1
donner compteur
```

Chaque ligne est assez petite pour devenir une ligne de code.

## Étape 4 : coder une étape à la fois

Ne tapez pas tout d'un coup. Écrivez **une étape**, lancez, vérifiez.
Puis la suivante. Comme quand on monte un meuble : on vérifie chaque vis
avant de passer à la suivante.

### Essai 1 : parcourir les lettres

D'abord, est-ce que je sais regarder chaque lettre ?

```python
mot = "banane"
for lettre in mot:
    print(lettre)
```

```sortie
b
a
n
a
n
e
```

Oui. Étape suivante.

### Essai 2 : reconnaître une voyelle

`lettre in "aeiouy"` demande : « cette lettre est-elle dans le texte
`aeiouy` ? ». La réponse est `True` (vrai) ou `False` (faux).

```python
mot = "banane"
for lettre in mot:
    print(lettre, lettre in "aeiouy")
```

```sortie
b False
a True
n False
a True
n False
e True
```

Le tableau correspond à celui fait à la main. Étape suivante.

### Essai 3 : compter… et se tromper

```python
mot = "banane"
for lettre in mot:
    compteur = 0
    if lettre in "aeiouy":
        compteur = compteur + 1
print(compteur)
```

```sortie
1
```

On attendait 3. Pas de panique : on a lancé tôt, donc l'erreur est dans les
3 lignes qu'on vient d'ajouter. Suivons la variable à la main :

| Lettre | `compteur = 0` | voyelle ? | compteur après |
|---|---|---|---|
| b | 0 | non | 0 |
| a | 0 | oui | 1 |
| n | 0 | non | 0 |
| a | 0 | oui | 1 |
| n | 0 | non | 0 |
| e | 0 | oui | 1 |

Le compteur repart à 0 **à chaque lettre**. Dans le pseudo-code, `compteur
← 0` était **avant** la boucle. On l'a mis dedans.

### Essai 4 : la correction

```python
mot = "banane"
compteur = 0
for lettre in mot:
    if lettre in "aeiouy":
        compteur = compteur + 1
print(compteur)
```

```sortie
3
```

Ça marche pour `banane`. Mais un seul exemple ne suffit pas.

### Essai 5 : tester d'autres mots

On essaie un mot avec une majuscule, et un mot sans voyelle :

```python
for mot in ["banane", "Avion", "pff"]:
    compteur = 0
    for lettre in mot:
        if lettre in "aeiouy":
            compteur = compteur + 1
    print(mot, compteur)
```

```sortie
banane 3
Avion 2
pff 0
```

`Avion` a 3 voyelles (A, i, o), pas 2. Le `A` majuscule n'est pas dans
`"aeiouy"`. On avait pourtant noté la question à l'étape 1 ! On met chaque
lettre en minuscule avec `lower()`.

### Essai 6 : la version finale

On range le tout dans une [fonction](glossaire.md#fonction) :

```python
def compter_voyelles(mot):
    compteur = 0
    for lettre in mot.lower():
        if lettre in "aeiouy":
            compteur = compteur + 1
    return compteur

print(compter_voyelles("banane"))
print(compter_voyelles("Avion"))
print(compter_voyelles("pff"))
```

```sortie
3
3
0
```

Six petits essais, deux erreurs trouvées tout de suite. Si on avait tout
écrit d'un coup, les deux erreurs seraient arrivées ensemble, et on n'aurait
pas su par où commencer.

Essayez : appelez `compter_voyelles("")`, avec un mot vide. Le résultat est
`0` : la boucle ne tourne pas du tout.

> **À retenir :** une étape, on lance, on vérifie. Plus on lance souvent,
> plus l'erreur est facile à trouver.

## Étape 5 : expliquer au canard en plastique

Posez un canard en plastique (ou une plante, ou une tasse) à côté de
l'écran. Expliquez-lui votre code **à voix haute**, ligne par ligne : « ici,
je mets le compteur à zéro ; ici, pour chaque lettre… ».

Très souvent, en plein milieu d'une phrase, vous entendez vous-même
l'erreur : « … et je remets le compteur à zéro à chaque lettre. Ah. ».

Parler oblige à ralentir. On ne peut pas sauter une ligne en l'expliquant.

## Étape 6 : faire une pause

Après 20 minutes bloqué sur la même chose, votre cerveau tourne en rond.
Levez-vous. Marchez, buvez un verre d'eau, faites autre chose dix minutes.

Ce n'est pas abandonner. Le cerveau continue à travailler en arrière-plan.
Beaucoup de programmeurs trouvent la solution sous la douche ou en
marchant, jamais devant l'écran.

## Demander de l'aide

Demander de l'aide n'est pas tricher. Mais une bonne question reçoit une
réponse bien plus vite. Elle contient quatre choses :

```text
1. Un exemple minimal : le plus petit code qui montre le problème.
2. Ce que j'attendais :  compter_voyelles("Avion") devrait donner 3.
3. Ce que j'obtiens :    il donne 2 (ou : le message d'erreur complet).
4. Ce que j'ai essayé :  j'ai affiché chaque lettre, le A n'est pas compté.
```

**Minimal** veut dire : enlevez tout ce qui ne sert pas à montrer le
problème. Souvent, en réduisant le code, vous trouvez l'erreur tout seul.

Copiez le code et le message **en texte**, pas en photo d'écran : la
personne qui vous aide pourra le lancer. Pour aller plus loin, voyez
[Trouver et corriger ses erreurs](deboguer.md).

## Erreurs fréquentes

### Recopier le pseudo-code sans le décalage

```python norun
mot = "banane"
for lettre in mot:
if lettre in "aeiouy":
    print(lettre)
```

```text
  File "/home/ada/voyelles.py", line 3
    if lettre in "aeiouy":
    ^^
IndentationError: expected an indented block after 'for' statement on line 2
```

En Python, ce qui est **dans** la boucle doit être décalé vers la droite
(l'[indentation](glossaire.md#indentation)). Le message le dit : un bloc
décalé était attendu après le `for` de la ligne 2. Correction : décalez le
`if` et tout ce qui suit de 4 espaces.

### Oublier une étape du pseudo-code

```python norun
mot = "banane"
for lettre in mot:
    if lettre in "aeiouy":
        compteur = compteur + 1
print(compteur)
```

```text
Traceback (most recent call last):
  File "/home/ada/voyelles.py", line 4, in <module>
    compteur = compteur + 1
               ^^^^^^^^
NameError: name 'compteur' is not defined. Did you mean: 'complex'?
```

La ligne `compteur ← 0` du pseudo-code a été oubliée. Python ne connaît
pas `compteur` au moment d'ajouter 1. Sa suggestion (`complex`) est fausse
ici : une suggestion est une devinette, pas un ordre. Correction : ajoutez
`compteur = 0` avant la boucle.

## Testez-vous

**1.** Vous ne savez pas par où commencer un exercice. Quelle est la
première chose à faire ?

> **Réponse**
>
> Reformuler l'énoncé avec vos mots : qu'est-ce que je reçois, qu'est-ce
> que je dois donner ? Puis résoudre un tout petit exemple à la main.

**2.** Pourquoi lancer le programme après chaque petite étape ?

> **Réponse**
>
> Parce que si quelque chose ne marche pas, l'erreur est forcément dans les
> quelques lignes qu'on vient d'ajouter. Elle est donc facile à trouver.

**3.** Quelles sont les quatre parties d'une bonne demande d'aide ?

> **Réponse**
>
> Un exemple minimal, ce que vous attendiez, ce que vous obtenez (ou le
> message d'erreur complet), et ce que vous avez déjà essayé.

## En résumé

- Bloquer est normal : ce qui compte, c'est la méthode.
- Reformulez avec vos mots, puis résolvez un petit exemple à la main.
- Écrivez le pseudo-code, puis codez une étape à la fois, en lançant après chacune.
- Testez plusieurs exemples, dont des cas bizarres (majuscule, mot vide).
- Expliquez au canard, faites une pause, puis demandez de l'aide avec un exemple minimal.

Suite : [Le terminal en 10 minutes](terminal.md).
