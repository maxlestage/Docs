# Conditions et boucles

Cette page explique comment un programme Python prend des décisions avec `if` et `match`, et répète des actions avec `while` et `for`.

> **Objectifs :**
>
> - vous saurez écrire une condition avec `if`, `elif` et `else` ;
> - vous saurez combiner des tests avec `and`, `or` et `not` ;
> - vous saurez choisir entre plusieurs cas avec `match` ;
> - vous saurez répéter du code avec `while` et `for` ;
> - vous saurez parcourir une liste avec `enumerate` et `zip`.

## La condition if

Une [condition](../debuter/glossaire.md#condition) permet de n'exécuter du code que si
quelque chose est vrai. C'est comme un panneau : « si la route est mouillée, ralentissez ».

```python
temperature = 30
if temperature > 25:
    print("Il fait chaud.")
    print("Pensez à boire.")
print("Bonne journée !")
```

```sortie
Il fait chaud.
Pensez à boire.
Bonne journée !
```

Trois règles à retenir :

- la ligne `if` finit par **deux-points** `:` ;
- les lignes qui dépendent du `if` sont **indentées** (décalées de 4 espaces) : elles
  forment un **bloc** ;
- la première ligne non décalée (`print("Bonne journée !")`) est hors du bloc : elle
  s'exécute toujours.

> **À retenir :** en Python, l'indentation n'est pas décorative. C'est elle qui dit
> quelles lignes font partie du bloc. Utilisez toujours 4 espaces.

## else et elif

`else` (« sinon ») donne le chemin à suivre quand la condition est fausse. `elif`
(contraction de *else if*, « sinon, si ») ajoute d'autres tests.

```python
note = 13
if note >= 16:
    print("Très bien")
elif note >= 12:
    print("Bien")
elif note >= 10:
    print("Passable")
else:
    print("Insuffisant")
```

```sortie
Bien
```

Python teste les conditions **dans l'ordre** et s'arrête à la première vraie. Ici,
`note >= 16` est faux, `note >= 12` est vrai : il affiche « Bien » et saute le reste.

## Comparer des valeurs

| Opérateur | Sens |
| --- | --- |
| `==` | égal à |
| `!=` | différent de |
| `<`, `>` | plus petit, plus grand |
| `<=`, `>=` | plus petit ou égal, plus grand ou égal |

Une comparaison donne un [booléen](../debuter/glossaire.md#booleen) : `True` ou
`False`.

```python
age = 17
print(age == 18)
print(age != 18)
print(age < 18)
print("chat" == "chat")
print(10 <= age < 20)
```

```sortie
False
True
True
True
True
```

La dernière ligne montre une facilité de Python : `10 <= age < 20` veut dire « `age` est
entre 10 (compris) et 20 (non compris) », comme en maths.

> **Attention :** `=` range une valeur dans une variable. `==` compare deux valeurs.
> Ne les confondez pas.

## Combiner : and, or, not

- `a and b` est vrai si `a` **et** `b` sont vrais ;
- `a or b` est vrai si **au moins un** des deux est vrai ;
- `not a` inverse : vrai devient faux, et faux devient vrai.

```python
age = 25
a_un_billet = True
est_vip = False

if age >= 18 and a_un_billet:
    print("Entrée autorisée")
if a_un_billet or est_vip:
    print("Accès à la salle")
if not est_vip:
    print("Pas d'accès au salon VIP")
```

```sortie
Entrée autorisée
Accès à la salle
Pas d'accès au salon VIP
```

## Vrai ou faux : la « véracité »

Dans un `if`, Python accepte n'importe quelle valeur, pas seulement `True` ou `False`.
Certaines valeurs comptent comme **fausses** :

- `False` et `None` ;
- le zéro : `0` et `0.0` ;
- tout ce qui est vide : `""`, `[]`, `{}`.

Tout le reste compte comme **vrai**.

```python
panier = []
if panier:
    print("Vous avez des articles.")
else:
    print("Votre panier est vide.")

nom = "Alice"
if nom:
    print(f"Bonjour, {nom} !")
```

```sortie
Votre panier est vide.
Bonjour, Alice !
```

`if panier:` se lit « si le panier contient quelque chose ». C'est plus court que
`if len(panier) > 0:`.

## L'expression conditionnelle

Pour choisir entre deux valeurs sur une seule ligne, écrivez
`valeur_si_vrai if condition else valeur_si_faux`.

```python
age = 15
statut = "majeur" if age >= 18 else "mineur"
print(statut)

n = 7
print("pair" if n % 2 == 0 else "impair")
```

```sortie
mineur
impair
```

> **Astuce :** réservez cette forme aux choix simples. Dès que c'est plus long, un
> vrai `if` sur plusieurs lignes est plus lisible.

## Choisir parmi plusieurs cas : match

Depuis Python 3.10, `match` compare une valeur à plusieurs **motifs** (*patterns*).
C'est comme un aiguillage de train : la valeur part sur la première voie qui
correspond.

```python
def decrire(commande):
    match commande:
        case "nord" | "sud":
            return "vertical"
        case "est" | "ouest":
            return "horizontal"
        case "quitter":
            return "au revoir"
        case _:
            return "commande inconnue"

print(decrire("nord"))
print(decrire("ouest"))
print(decrire("sauter"))
```

```sortie
vertical
horizontal
commande inconnue
```

- `|` veut dire « ou » : `"nord" | "sud"` accepte l'un ou l'autre ;
- `case _:` attrape tout le reste : c'est le cas par défaut.

`match` sait aussi découper une valeur. Ici, il reconnaît une liste de deux éléments et
range chaque morceau dans une variable :

```python
for action in [["aller", "nord"], ["prendre", "clé"], ["dormir"]]:
    match action:
        case ["aller", direction]:
            print(f"Vous allez vers le {direction}.")
        case ["prendre", objet]:
            print(f"Vous prenez : {objet}.")
        case _:
            print("Je ne comprends pas.")
```

```sortie
Vous allez vers le nord.
Vous prenez : clé.
Je ne comprends pas.
```

Les fonctions (`def`) sont expliquées dans [les fonctions](fonctions.md).

## La boucle while

Une [boucle](../debuter/glossaire.md#boucle) répète du code. `while` (« tant que »)
répète son bloc **tant que** la condition est vraie.

```python
compte = 3
while compte > 0:
    print(compte)
    compte = compte - 1
print("Décollage !")
```

```sortie
3
2
1
Décollage !
```

À chaque tour, Python vérifie `compte > 0`. Quand `compte` vaut 0, la condition est
fausse : la boucle s'arrête.

> **Attention :** si la condition ne devient jamais fausse, la boucle tourne pour
> toujours. Vérifiez qu'une variable change à chaque tour. Pour arrêter un programme
> bloqué, appuyez sur Ctrl+C dans le terminal.

## La boucle for

`for` parcourt les éléments d'une collection, un par un. C'est la boucle la plus
utilisée en Python.

```python
fruits = ["pomme", "banane", "cerise"]
for fruit in fruits:
    print(f"J'aime la {fruit}.")
```

```sortie
J'aime la pomme.
J'aime la banane.
J'aime la cerise.
```

À chaque tour, la variable `fruit` reçoit l'élément suivant de la liste. Chaque passage
s'appelle une [itération](../debuter/glossaire.md#iteration).

`for` parcourt aussi un texte, lettre par lettre :

```python
for lettre in "Oui":
    print(lettre)
```

```sortie
O
u
i
```

### Compter avec range

`range` produit une suite de nombres. Il est parfait pour répéter une action un nombre
précis de fois.

- `range(5)` : de 0 à 4 (5 nombres, **sans** le 5) ;
- `range(1, 6)` : de 1 à 5 ;
- `range(0, 10, 2)` : de 0 à 8, de 2 en 2.

```python
for i in range(3):
    print("Tour", i)

print(list(range(1, 6)))
print(list(range(0, 10, 2)))
print(list(range(10, 0, -3)))
```

```sortie
Tour 0
Tour 1
Tour 2
[1, 2, 3, 4, 5]
[0, 2, 4, 6, 8]
[10, 7, 4, 1]
```

`list(...)` transforme le `range` en liste, pour voir tous ses nombres d'un coup. Un pas
négatif (`-3`) compte à rebours.

> **À retenir :** la fin d'un `range` est toujours **exclue**. `range(1, 6)` s'arrête
> à 5.

## break et continue

- `break` sort de la boucle immédiatement ;
- `continue` saute la fin du tour et passe au suivant.

```python
for n in range(1, 10):
    if n % 2 == 0:
        continue
    if n > 7:
        break
    print(n)
```

```sortie
1
3
5
7
```

Les nombres pairs sont sautés (`continue`). Quand `n` dépasse 7, la boucle s'arrête
(`break`).

### else sur une boucle

Une boucle peut avoir un `else`. Il s'exécute si la boucle va **jusqu'au bout**, sans
`break`. C'est utile pour une recherche : « je n'ai rien trouvé ».

```python
nombres = [3, 7, 11, 15]
for n in nombres:
    if n % 2 == 0:
        print(f"Premier nombre pair : {n}")
        break
else:
    print("Aucun nombre pair.")
```

```sortie
Aucun nombre pair.
```

## enumerate : le numéro et l'élément

Pour avoir à la fois la position et l'élément, utilisez `enumerate`. À chaque tour, il
donne deux valeurs : le numéro (l'[indice](../debuter/glossaire.md#indice)) et
l'élément.

```python
podium = ["Alice", "Bob", "Chloé"]
for rang, nom in enumerate(podium, start=1):
    print(f"{rang}. {nom}")
```

```sortie
1. Alice
2. Bob
3. Chloé
```

Sans `start=1`, la numérotation commence à 0.

## zip : parcourir deux listes ensemble

`zip` avance dans plusieurs listes en même temps, comme une fermeture éclair qui
assemble deux côtés, dent par dent.

```python
prenoms = ["Alice", "Bob", "Chloé"]
notes = [15, 12, 18]
for prenom, note in zip(prenoms, notes):
    print(f"{prenom} : {note}/20")
```

```sortie
Alice : 15/20
Bob : 12/20
Chloé : 18/20
```

> **Note :** `zip` s'arrête à la fin de la liste la plus courte.

## Erreurs fréquentes

### Oublier les deux-points

```python norun
age = 20
if age >= 18
    print("majeur")
```

```text
  File "/home/vous/main.py", line 2
    if age >= 18
                ^
SyntaxError: expected ':'
```

Python attendait `:` (*expected ':'*). Toute ligne `if`, `elif`, `else`, `while`, `for`
et `match` finit par deux-points.

### Écrire = au lieu de ==

```python norun
age = 20
if age = 18:
    print("pile")
```

```text
  File "/home/vous/main.py", line 2
    if age = 18:
       ^^^^^^^^
SyntaxError: invalid syntax. Maybe you meant '==' or ':=' instead of '='?
```

`=` range une valeur ; pour comparer, il faut `==`. Python le devine et vous le
suggère. Écrivez `if age == 18:`.

### Oublier l'indentation

```python norun
age = 20
if age >= 18:
print("majeur")
```

```text
  File "/home/vous/main.py", line 3
    print("majeur")
    ^^^^^
IndentationError: expected an indented block after 'if' statement on line 2
```

Après `if ... :`, Python attend un bloc décalé. Ajoutez 4 espaces devant
`print("majeur")`.

### Parcourir un nombre avec for

```python norun
for i in 5:
    print(i)
```

```text
Traceback (most recent call last):
  File "/home/vous/main.py", line 1, in <module>
    for i in 5:
             ^
TypeError: 'int' object is not iterable
```

On ne peut pas parcourir un nombre (*not iterable*). Pour répéter 5 fois, écrivez
`for i in range(5):`.

## Testez-vous

**Question 1 :** qu'affiche ce code ?

```python norun
x = 0
if x:
    print("A")
elif x == 0:
    print("B")
else:
    print("C")
```

> **Réponse**
>
> `B`. `0` compte comme faux, donc le premier test échoue. `x == 0` est vrai :
> Python affiche `B` et ignore le `else`.

**Question 2 :** combien de fois s'affiche « coucou » avec `for i in range(2, 8):` ?

> **Réponse**
>
> 6 fois : `i` prend les valeurs 2, 3, 4, 5, 6 et 7. La fin (8) est exclue.

**Question 3 :** écrivez une boucle qui affiche les multiples de 3 de 3 à 15, sur une
ligne.

> **Réponse**
>
> ```python
> for n in range(3, 16, 3):
>     print(n, end=" ")
> print("fin")
> ```
>
> ```sortie
> 3 6 9 12 15 fin
> ```

## En résumé

- `if`, `elif` et `else` choisissent un chemin ; chaque ligne finit par `:` et le bloc
  est indenté de 4 espaces.
- `==` compare, `=` range ; `and`, `or` et `not` combinent les tests.
- `0`, `None` et les collections vides comptent comme faux.
- `match` choisit parmi plusieurs cas ; `case _:` attrape le reste.
- `while` répète tant qu'une condition est vraie ; `for` parcourt une collection ou un
  `range`.
- `break` sort, `continue` saute un tour ; `enumerate` et `zip` simplifient les
  parcours.

Suite : [les fonctions](fonctions.md).
