# Gérer les erreurs

Cette page explique comment lire un message d'erreur Python, comment attraper les erreurs avec `try` et `except`, comment en lever avec `raise` et comment `with` garantit le nettoyage.

> **Objectifs :**
>
> - vous saurez lire une trace d'erreur ligne par ligne ;
> - vous saurez attraper une erreur précise avec `try` et `except` ;
> - vous saurez utiliser `else` et `finally` ;
> - vous saurez lever vos propres erreurs avec `raise` et des classes d'exception ;
> - vous saurez utiliser `with` et choisir entre « demander pardon » et « demander la
>   permission ».

## Une erreur arrête le programme

Quand Python rencontre un problème, il lève une
[exception](../debuter/glossaire.md#exception) : un objet qui décrit
l'[erreur](../debuter/glossaire.md#erreur). « Lever » veut dire : déclencher l'alarme.
Si personne ne s'en occupe, le programme s'arrête et affiche une **trace** (*traceback*).

```python norun
def moyenne(notes):
    return sum(notes) / len(notes)

def bulletin(eleve, notes):
    m = moyenne(notes)
    return f"{eleve} : {m:.1f}"

print(bulletin("Alice", [12, 15]))
print(bulletin("Bob", []))
```

```text
Alice : 13.5
Traceback (most recent call last):
  File "main.py", line 9, in <module>
    print(bulletin("Bob", []))
          ~~~~~~~~^^^^^^^^^^^
  File "main.py", line 5, in bulletin
    m = moyenne(notes)
  File "main.py", line 2, in moyenne
    return sum(notes) / len(notes)
           ~~~~~~~~~~~^~~~~~~~~~~~
ZeroDivisionError: division by zero
```

La première ligne s'est bien affichée : le programme s'arrête **au moment** de l'erreur,
pas avant.

### Lire la trace ligne par ligne

Une trace se lit **de bas en haut** : la cause est à la fin.

1. `ZeroDivisionError: division by zero` : la **classe** de l'erreur (son nom de
   famille), puis sa description, « division par zéro ». Commencez toujours par là.
2. `File "main.py", line 2, in moyenne` : l'endroit exact, dans la fonction `moyenne`.
   Les `~~~^~~~` soulignent l'opération fautive : la division.
3. `line 5, in bulletin` : `moyenne` avait été appelée par `bulletin`, ligne 5.
4. `line 9, in <module>` : et `bulletin` par le programme principal (`<module>`),
   ligne 9.

`Traceback (most recent call last)` le rappelle : l'appel le plus récent est en dernier.
Ici, on comprend vite : `bulletin` a reçu une liste vide, et `len([])` vaut 0.

> **Note :** sur votre ordinateur, `File` affiche le chemin complet, par exemple
> `File "/home/vous/projet/main.py"`. Il est raccourci en `main.py` dans ce guide.

## try et except

`try` (« essayer ») et `except` (« sauf si ») attrapent une erreur. Le programme
continue au lieu de s'arrêter. C'est le filet sous un trapéziste : la chute est
rattrapée, le spectacle continue.

- Le code risqué va dans le bloc `try`.
- Le code de secours va dans le bloc `except`, avec la classe d'erreur attendue.

```python
try:
    resultat = 10 / 0
    print("Cette ligne n'est jamais atteinte")
except ZeroDivisionError:
    print("On ne divise pas par zéro !")
print("Le programme continue.")
```

```sortie
On ne divise pas par zéro !
Le programme continue.
```

Dès qu'une erreur arrive dans `try`, Python saute directement au `except` : la suite du
bloc `try` est abandonnée.

### Récupérer le message

Avec `as e`, vous récupérez l'objet exception. `str(e)` (ou `print(e)`) donne son
message.

```python
try:
    age = int("douze")
except ValueError as e:
    print("Erreur :", e)
```

```sortie
Erreur : invalid literal for int() with base 10: 'douze'
```

### Plusieurs except

On peut écrire plusieurs `except`, un par classe d'erreur, ou en grouper plusieurs dans
un tuple.

```python
def diviser(texte_a, texte_b):
    try:
        return int(texte_a) / int(texte_b)
    except ValueError:
        return "Ce n'est pas un nombre."
    except ZeroDivisionError:
        return "Division par zéro."

print(diviser("10", "4"))
print(diviser("dix", "4"))
print(diviser("10", "0"))
```

```sortie
2.5
Ce n'est pas un nombre.
Division par zéro.
```

> **Attention :** attrapez des erreurs **précises**. Un `except:` tout seul, ou
> `except Exception:`, attrape tout, y compris vos fautes de frappe : un bug devient
> alors invisible.

## else et finally

Deux blocs facultatifs complètent `try` :

- `else` s'exécute seulement s'il n'y a **pas** eu d'erreur ;
- `finally` s'exécute **toujours**, erreur ou pas. Il sert à nettoyer.

```python
def lire_age(texte):
    try:
        age = int(texte)
    except ValueError:
        print(f"{texte!r} : pas un nombre")
    else:
        print(f"{texte!r} : {age} ans")
    finally:
        print("  (vérification terminée)")

lire_age("42")
lire_age("vieux")
```

```sortie
'42' : 42 ans
  (vérification terminée)
'vieux' : pas un nombre
  (vérification terminée)
```

Pourquoi `else` plutôt que mettre le `print` dans `try` ? Pour garder dans `try` **seulement**
la ligne risquée : ainsi, on n'attrape pas par erreur un problème venu d'ailleurs.

## Lever une erreur avec raise

`raise` déclenche une erreur vous-même, par exemple pour refuser une valeur absurde.
Donnez la classe de l'erreur et un message clair.

```python
def retirer(solde, montant):
    if montant <= 0:
        raise ValueError(f"montant invalide : {montant}")
    if montant > solde:
        raise ValueError("solde insuffisant")
    return solde - montant

print(retirer(100, 30))
try:
    retirer(100, -5)
except ValueError as e:
    print("Refusé :", e)
```

```sortie
70
Refusé : montant invalide : -5
```

Lever une erreur plutôt que renvoyer une valeur bizarre (`-1`, `None`…) oblige
l'appelant à s'en occuper : l'erreur ne peut pas passer inaperçue.

### Relancer une erreur

Dans un `except`, un `raise` seul relance l'erreur attrapée, après avoir fait quelque
chose (par exemple, noter le problème).

```python
try:
    try:
        int("abc")
    except ValueError:
        print("Je note le problème, puis je le transmets.")
        raise
except ValueError as e:
    print("Attrapée plus haut :", e)
```

```sortie
Je note le problème, puis je le transmets.
Attrapée plus haut : invalid literal for int() with base 10: 'abc'
```

## Vos propres exceptions

Une exception est un objet comme un autre, d'une [classe](types.md) qui hérite
d'`Exception` (voir [l'héritage](heritage.md)). Créer votre propre classe donne un nom
clair à vos erreurs, que l'appelant peut attraper précisément.

```python
class SoldeInsuffisantError(Exception):
    def __init__(self, solde, montant):
        super().__init__(f"il manque {montant - solde} €")
        self.manque = montant - solde

class Compte:
    def __init__(self, solde):
        self.solde = solde

    def retirer(self, montant):
        if montant > self.solde:
            raise SoldeInsuffisantError(self.solde, montant)
        self.solde -= montant

compte = Compte(50)
try:
    compte.retirer(80)
except SoldeInsuffisantError as e:
    print("Refusé :", e)
    print("Manque :", e.manque)
```

```sortie
Refusé : il manque 30 €
Manque : 30
```

> **À retenir :** héritez d'`Exception`, et terminez le nom par `Error`. Pour une
> exception toute simple, `class MonError(Exception): pass` suffit.

## assert : vérifier une hypothèse

`assert condition, "message"` lève une `AssertionError` si la condition est fausse.
Elle sert à vérifier ce qui **doit** être vrai si votre code est correct : c'est un
garde-fou pour vous, le développeur.

```python
def pourcentage(part, total):
    assert total > 0, "total doit être positif"
    return part / total * 100

print(pourcentage(1, 4))
try:
    pourcentage(1, 0)
except AssertionError as e:
    print("Bug détecté :", e)
```

```sortie
25.0
Bug détecté : total doit être positif
```

> **Attention :** ne validez pas les saisies de l'utilisateur avec `assert` : les
> assertions peuvent être désactivées (`python3 -O`). Pour cela, utilisez `raise`.

## Les gestionnaires de contexte : with

Certaines ressources doivent être **libérées** après usage : un fichier ouvert doit être
fermé, même si une erreur survient. `with` s'en charge pour vous.

```python
with open("notes.txt", "w") as f:
    f.write("15\n12\n")

with open("notes.txt") as f:
    print(f.read().split())
print(f.closed)
```

```sortie
['15', '12']
True
```

À la sortie du bloc `with`, le fichier est fermé automatiquement, qu'il y ait eu une
erreur ou non. C'est un `try … finally` tout fait. Les fichiers sont détaillés dans
[lire et écrire des fichiers](fichiers.md).

### Écrire son propre gestionnaire

Le module `contextlib` permet d'en écrire un avec un générateur (voir
[itérateurs et générateurs](iterateurs.md)) : le code avant `yield` s'exécute à
l'entrée, le code après, dans `finally`, à la sortie.

```python
from contextlib import contextmanager

@contextmanager
def section(titre):
    print(f"[début {titre}]")
    try:
        yield
    finally:
        print(f"[fin {titre}]")

with section("calcul"):
    print(2 + 2)

try:
    with section("risqué"):
        1 / 0
except ZeroDivisionError:
    print("erreur attrapée")
```

```sortie
[début calcul]
4
[fin calcul]
[début risqué]
[fin risqué]
erreur attrapée
```

Même avec l'erreur, `[fin risqué]` s'affiche : le nettoyage est garanti.

## Demander pardon ou demander la permission ?

Deux façons d'éviter une erreur :

- **LBYL** (*Look Before You Leap*, « regarder avant de sauter ») : on vérifie d'abord
  avec un `if` ;
- **EAFP** (*Easier to Ask Forgiveness than Permission*, « plus facile de demander
  pardon que la permission ») : on essaie, et on attrape l'erreur si ça rate.

```python
stock = {"pomme": 3}

# LBYL : on vérifie avant
if "poire" in stock:
    print(stock["poire"])
else:
    print("pas de poire (LBYL)")

# EAFP : on essaie
try:
    print(stock["poire"])
except KeyError:
    print("pas de poire (EAFP)")
```

```sortie
pas de poire (LBYL)
pas de poire (EAFP)
```

Le style EAFP est très courant en Python, surtout quand la vérification serait
compliquée (un texte est-il un nombre valide ?) ou quand la situation peut changer entre
le test et l'action (un fichier supprimé entre-temps). Pour un cas simple comme une clé
de dictionnaire, `stock.get("poire", 0)` est encore plus court.

## Les exceptions courantes

| Classe | Quand ? |
|--------|---------|
| `NameError` | variable inconnue (souvent une faute de frappe) |
| `TypeError` | mauvais type, par exemple `"a" + 1` |
| `ValueError` | bon type, mauvaise valeur : `int("abc")` |
| `IndexError` | indice hors de la liste : `[1, 2][5]` |
| `KeyError` | clé absente d'un dictionnaire |
| `AttributeError` | attribut ou méthode inconnus : `(5).upper()` |
| `ZeroDivisionError` | division par zéro |
| `FileNotFoundError` | fichier introuvable |
| `ImportError` / `ModuleNotFoundError` | module introuvable |
| `StopIteration` | itérateur épuisé |
| `AssertionError` | `assert` dont la condition est fausse |

Toutes héritent d'`Exception`. Attraper une classe parente attrape aussi ses enfants :
`except LookupError` attrape à la fois `IndexError` et `KeyError`.

## Erreurs fréquentes

### Attraper la mauvaise classe d'erreur

```python norun
try:
    age = int("douze")
except ZeroDivisionError:
    print("Problème")
```

```text
Traceback (most recent call last):
  File "main.py", line 2, in <module>
    age = int("douze")
ValueError: invalid literal for int() with base 10: 'douze'
```

`except ZeroDivisionError` n'attrape que les divisions par zéro : la `ValueError` passe
à travers. Recopiez la classe écrite en bas de la trace : `except ValueError:`.

### Lever une chaîne au lieu d'une exception

```python norun
def retirer(solde, montant):
    if montant > solde:
        raise "solde insuffisant"
    return solde - montant

retirer(10, 50)
```

```text
Traceback (most recent call last):
  File "main.py", line 6, in <module>
    retirer(10, 50)
    ~~~~~~~^^^^^^^^
  File "main.py", line 3, in retirer
    raise "solde insuffisant"
TypeError: exceptions must derive from BaseException
```

On ne lève que des objets exception. Écrivez `raise ValueError("solde insuffisant")`.

### Ouvrir un fichier qui n'existe pas

```python norun
with open("notes.txt") as f:
    print(f.read())
```

```text
Traceback (most recent call last):
  File "main.py", line 1, in <module>
    with open("notes.txt") as f:
         ~~~~^^^^^^^^^^^^^
FileNotFoundError: [Errno 2] No such file or directory: 'notes.txt'
```

Le fichier n'est pas dans le dossier où vous lancez le programme. Vérifiez le nom et le
dossier, ou attrapez `FileNotFoundError` pour afficher un message clair.

## Testez-vous

**Question 1 :** qu'affiche ce code ?

```python norun
try:
    print("A")
    int("x")
    print("B")
except ValueError:
    print("C")
else:
    print("D")
finally:
    print("E")
```

> **Réponse**
>
> `A`, `C`, puis `E`. `B` est sauté à cause de l'erreur, et `D` ne s'affiche que s'il
> n'y a pas d'erreur. `finally` s'exécute toujours.

**Question 2 :** dans une trace, où trouver la cause de l'erreur ?

> **Réponse**
>
> Sur la **dernière** ligne (classe et message), puis juste au-dessus : le fichier et la
> ligne où l'erreur s'est produite.

**Question 3 :** écrivez une fonction `age_valide(n)` qui lève une `ValueError` si `n`
est négatif, et renvoie `n` sinon.

> **Réponse**
>
> ```python
> def age_valide(n):
>     if n < 0:
>         raise ValueError("âge négatif")
>     return n
>
> print(age_valide(12))
> try:
>     age_valide(-1)
> except ValueError as e:
>     print(e)
> ```
>
> ```sortie
> 12
> âge négatif
> ```

## En résumé

- Une trace se lit de bas en haut : classe et message, puis la ligne fautive.
- `try` / `except Classe as e` attrape une erreur précise ; `else` s'exécute sans erreur,
  `finally` toujours.
- `raise` lève une erreur ; vos exceptions héritent d'`Exception`.
- `assert` vérifie vos hypothèses de développeur, pas les saisies.
- `with` libère une ressource automatiquement, même en cas d'erreur.
- En Python, on essaie souvent d'abord et on attrape l'erreur ensuite (EAFP).

Suite : [modules, paquets et pip](modules.md). Comparez avec
[les erreurs en Ruby](../ruby/erreurs.md).
