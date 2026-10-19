# Les conditions

Cette page explique comment un programme Python prend des décisions avec `if`, `elif`, `else` et `match`.

> **Objectifs :**
>
> - vous saurez comparer deux valeurs avec `==`, `!=`, `<` et `>` ;
> - vous saurez écrire une condition avec `if`, `elif` et `else` ;
> - vous saurez combiner des tests avec `and`, `or` et `not` ;
> - vous saurez choisir entre plusieurs cas avec `match`.

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

Ligne par ligne :

- `temperature = 30` range 30 dans la variable `temperature` ;
- `if temperature > 25:` pose une question : « 30 est-il plus grand que 25 ? ». La
  réponse est oui ;
- les deux `print` décalés s'exécutent donc, l'un après l'autre ;
- le dernier `print` n'est pas décalé : il s'exécute dans tous les cas, même s'il
  faisait froid.

Trois règles à retenir :

- la ligne `if` finit par **deux-points** `:` ;
- les lignes qui dépendent du `if` sont **indentées** (décalées de 4 espaces) : elles
  forment un **bloc** ;
- la première ligne non décalée (`print("Bonne journée !")`) est hors du bloc : elle
  s'exécute toujours.

> **À retenir :** en Python, l'indentation n'est pas décorative. C'est elle qui dit
> quelles lignes font partie du bloc. Utilisez toujours 4 espaces.

## else et elif

Souvent, il n'y a pas qu'un seul cas. Une note peut être très bonne, moyenne ou
mauvaise, et vous voulez un message différent pour chacune, comme sur un bulletin.

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

**À vous :** écrivez un programme qui range `heure = 15` dans une variable, puis
affiche « Bonjour » si l'heure est avant 12, « Bon après-midi » si elle est avant 18,
et « Bonsoir » sinon.

> **Solution**
>
> ```python
> heure = 15
> if heure < 12:
>     print("Bonjour")
> elif heure < 18:
>     print("Bon après-midi")
> else:
>     print("Bonsoir")
> ```
>
> ```sortie
> Bon après-midi
> ```

## Comparer des valeurs

Pour décider, un programme pose d'abord une question : « l'âge vaut-il 18 ? »,
« la température dépasse-t-elle 25 ? ». Les opérateurs de comparaison servent à
écrire ces questions.

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

Une décision dépend souvent de plusieurs choses à la fois. Pour entrer au cinéma, il
faut un billet **et** avoir l'âge. Ces trois mots relient plusieurs tests en un seul.

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

Ligne par ligne :

- les trois premières lignes préparent les données : un âge et deux booléens ;
- premier `if` : `age >= 18` est vrai **et** `a_un_billet` est vrai, donc l'entrée est
  autorisée ;
- deuxième `if` : `a_un_billet` est vrai, et avec `or` cela suffit ;
- troisième `if` : `est_vip` vaut `False`, et `not` le transforme en `True`.

**À vous :** écrivez un programme qui range `age = 70` dans une variable, puis
affiche « Tarif réduit » si l'âge est inférieur à 18 ou supérieur ou égal à 65, et
« Plein tarif » sinon.

> **Solution**
>
> ```python
> age = 70
> if age < 18 or age >= 65:
>     print("Tarif réduit")
> else:
>     print("Plein tarif")
> ```
>
> ```sortie
> Tarif réduit
> ```

## Vrai ou faux : la « véracité »

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

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

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

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

**Question 2 :** qu'affiche `print(5 > 3 and not 2 > 4)` ?

> **Réponse**
>
> `True`. `5 > 3` est vrai ; `2 > 4` est faux, et `not` le rend vrai. Vrai `and` vrai
> donne `True`.

**Question 3 :** dans un `match`, à quoi sert `case _:` ?

> **Réponse**
>
> C'est le cas par défaut : il attrape toutes les valeurs qu'aucun `case` précédent
> n'a reconnues. Placez-le en dernier.

## En résumé

- `==` compare, `=` range ; une comparaison donne `True` ou `False`.
- `if`, `elif` et `else` choisissent un chemin ; chaque ligne finit par `:` et le bloc
  est indenté de 4 espaces.
- `and`, `or` et `not` combinent les tests.
- `0`, `None` et les collections vides comptent comme faux.
- `match` choisit parmi plusieurs cas ; `case _:` attrape le reste.

Suite : [les boucles](boucles.md).
