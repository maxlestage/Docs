# Héritage et polymorphisme

Cette page montre comment une classe peut hériter d'une autre, comment des objets différents peuvent répondre au même appel, et comment faire marcher `==`, `<` ou `len` sur vos propres classes.

> **Objectifs :**
>
> - vous saurez écrire une sous-classe et appeler le parent avec `super()` ;
> - vous saurez redéfinir une méthode et tester un type avec `isinstance` ;
> - vous saurez ce que sont le polymorphisme et le *duck typing* ;
> - vous saurez écrire une classe abstraite avec `abc` ;
> - vous saurez choisir entre héritage et composition, et écrire `__eq__`, `__lt__` et
>   `__len__`.

## Une sous-classe

L'[héritage](../debuter/glossaire.md#heritage) permet de créer une classe à partir
d'une autre. La nouvelle classe, la **sous-classe** (ou classe enfant), reçoit tout ce
que possède la **classe parente** : ses attributs et ses méthodes. On l'écrit en mettant
le parent entre parenthèses.

```python
class Animal:
    def __init__(self, nom):
        self.nom = nom

    def manger(self):
        print(f"{self.nom} mange.")

class Chien(Animal):
    def aboyer(self):
        print(f"{self.nom} aboie : Wouf !")

rex = Chien("Rex")
rex.manger()
rex.aboyer()
```

```sortie
Rex mange.
Rex aboie : Wouf !
```

`Chien` n'a pas écrit `__init__` ni `manger` : il les a hérités d'`Animal`. Il ajoute
seulement ce qui lui est propre.

> **À retenir :** l'héritage exprime une relation « **est un** ». Un chien **est un**
> animal. Si la phrase sonne faux, l'héritage n'est sans doute pas le bon outil.

## super() : appeler le parent

Si la sous-classe a besoin d'attributs en plus, elle écrit son propre `__init__`. Pour
ne pas recopier le travail du parent, elle l'appelle avec `super()`.

```python
class Animal:
    def __init__(self, nom):
        self.nom = nom

class Chat(Animal):
    def __init__(self, nom, couleur):
        super().__init__(nom)
        self.couleur = couleur

minou = Chat("Minou", "gris")
print(minou.nom, minou.couleur)
```

```sortie
Minou gris
```

`super().__init__(nom)` veut dire : « fais d'abord ce que ferait le parent avec `nom` ».
Ensuite, `Chat` ajoute `couleur`.

## Redéfinir une méthode

Une sous-classe peut **redéfinir** une méthode du parent : elle écrit une méthode du
même nom, qui remplace celle du parent pour ses objets.

```python
class Animal:
    def __init__(self, nom):
        self.nom = nom

    def parler(self):
        return "..."

class Chien(Animal):
    def parler(self):
        return "Wouf"

class Chat(Animal):
    def parler(self):
        return "Miaou"

print(Animal("Bête").parler())
print(Chien("Rex").parler())
print(Chat("Minou").parler())
```

```sortie
...
Wouf
Miaou
```

### Compléter plutôt que remplacer

Avec `super()`, une méthode redéfinie peut aussi **réutiliser** celle du parent et
l'enrichir.

```python
class Employe:
    def __init__(self, nom, salaire):
        self.nom = nom
        self.salaire = salaire

    def description(self):
        return f"{self.nom}, {self.salaire} €"

class Manager(Employe):
    def __init__(self, nom, salaire, equipe):
        super().__init__(nom, salaire)
        self.equipe = equipe

    def description(self):
        return super().description() + f", équipe de {len(self.equipe)}"

print(Manager("Alice", 4000, ["Bob", "Chloé"]).description())
```

```sortie
Alice, 4000 €, équipe de 2
```

## isinstance et issubclass

`isinstance(objet, Classe)` répond à « cet objet est-il de cette classe, **ou d'une de
ses sous-classes** ? ». `issubclass` pose la même question pour deux classes.

```python
class Animal:
    pass

class Chien(Animal):
    pass

rex = Chien()
print(isinstance(rex, Chien))
print(isinstance(rex, Animal))
print(isinstance(rex, str))
print(issubclass(Chien, Animal))
print(type(rex) == Animal)
```

```sortie
True
True
False
True
False
```

`type(rex) == Animal` est faux : `type` donne la classe exacte, sans regarder les
parents. Préférez `isinstance`.

> **Note :** toutes les classes héritent de `object`, la classe de base de Python. C'est
> elle qui fournit le `__repr__` et le `==` par défaut.

## Le polymorphisme

« Polymorphisme » veut dire « plusieurs formes ». Le même appel, `forme.aire()`, donne un
résultat différent selon l'objet. Le code qui appelle n'a pas besoin de savoir de quelle
classe il s'agit.

```python
import math

class Cercle:
    def __init__(self, rayon):
        self.rayon = rayon

    def aire(self):
        return math.pi * self.rayon ** 2

class Rectangle:
    def __init__(self, largeur, hauteur):
        self.largeur = largeur
        self.hauteur = hauteur

    def aire(self):
        return self.largeur * self.hauteur

formes = [Cercle(1), Rectangle(2, 3), Cercle(2)]
for forme in formes:
    print(type(forme).__name__, round(forme.aire(), 2))
```

```sortie
Cercle 3.14
Rectangle 6
Cercle 12.57
```

C'est comme une télécommande : le bouton « marche » allume la télévision, la radio ou le
ventilateur. Chaque appareil sait ce que « marche » veut dire pour lui.

### Le duck typing

Dans l'exemple précédent, `Cercle` et `Rectangle` n'ont **aucun parent commun** : ils
ont juste tous les deux une méthode `aire`. Pour Python, cela suffit.

C'est le *duck typing* : « si ça marche comme un canard et que ça cancane comme un
canard, alors c'est un canard ». Python ne vérifie pas le type d'un objet, seulement
qu'il sait faire ce qu'on lui demande.

```python
class Canard:
    def cancaner(self):
        return "Coin coin"

class Robot:
    def cancaner(self):
        return "BIP coin BIP"

def faire_cancaner(truc):
    print(truc.cancaner())

faire_cancaner(Canard())
faire_cancaner(Robot())
```

```sortie
Coin coin
BIP coin BIP
```

## Les classes abstraites

Une **classe abstraite** est un modèle incomplet : elle annonce des méthodes que chaque
sous-classe **doit** écrire, mais on ne peut pas créer d'objet directement à partir
d'elle. On l'écrit avec le module `abc` (*abstract base classes*).

```python
from abc import ABC, abstractmethod

class Forme(ABC):
    @abstractmethod
    def aire(self):
        pass

    def decrire(self):
        return f"{type(self).__name__} d'aire {self.aire()}"

class Carre(Forme):
    def __init__(self, cote):
        self.cote = cote

    def aire(self):
        return self.cote ** 2

print(Carre(3).decrire())
try:
    Forme()
except TypeError as e:
    print(e)
```

```sortie
Carre d'aire 9
Can't instantiate abstract class Forme without an implementation for abstract method 'aire'
```

`decrire` est écrite une seule fois, dans `Forme`, et elle appelle `aire`, que chaque
sous-classe fournit. La classe abstraite garantit qu'aucune sous-classe n'oublie `aire`.

## Composition ou héritage ?

L'héritage n'est pas la seule façon de réutiliser du code. La **composition** consiste à
**ranger un objet dans un autre**. Elle exprime une relation « **a un** » : une voiture
**a un** moteur ; elle n'**est pas** un moteur.

```python
class Moteur:
    def __init__(self, puissance):
        self.puissance = puissance

    def demarrer(self):
        return f"Vroum ({self.puissance} ch)"

class Voiture:
    def __init__(self, marque, puissance):
        self.marque = marque
        self.moteur = Moteur(puissance)

    def demarrer(self):
        return f"{self.marque} : {self.moteur.demarrer()}"

print(Voiture("Zoé", 90).demarrer())
```

```sortie
Zoé : Vroum (90 ch)
```

| Question | Choix |
|----------|-------|
| « X est un Y » est vrai ? | héritage |
| « X a un Y » est vrai ? | composition |
| vous hésitez ? | composition, plus souple |

La composition permet de changer une pièce sans toucher au reste : on peut donner à la
voiture un moteur électrique sans créer une nouvelle classe de voiture.

## Les méthodes spéciales pour vos classes

Les méthodes entourées de doubles soulignés, comme `__init__`, sont appelées
**méthodes spéciales** (ou *dunder*, pour *double underscore*). Python les appelle tout
seul quand vous utilisez un opérateur ou une fonction intégrée.

| Vous écrivez | Python appelle |
|--------------|----------------|
| `a == b` | `a.__eq__(b)` |
| `a < b` | `a.__lt__(b)` |
| `len(a)` | `a.__len__()` |
| `str(a)`, `print(a)` | `a.__str__()` |
| `x in a` | `a.__contains__(x)` |

### Comparer avec __eq__

Par défaut, `==` entre deux objets regarde s'il s'agit du **même** objet. Avec `__eq__`,
vous décidez ce qu'« égal » veut dire.

```python
class Point:
    def __init__(self, x, y):
        self.x = x
        self.y = y

    def __eq__(self, autre):
        if not isinstance(autre, Point):
            return NotImplemented
        return self.x == autre.x and self.y == autre.y

print(Point(1, 2) == Point(1, 2))
print(Point(1, 2) == Point(3, 4))
print(Point(1, 2) == "texte")
```

```sortie
True
False
False
```

`NotImplemented` dit à Python : « je ne sais pas comparer avec ça ». Python essaie alors
autre chose, et finit par répondre `False`.

### Trier avec __lt__

`sorted`, `min` et `max` utilisent `<`. Il suffit d'écrire `__lt__` (*less than*,
« plus petit que »).

```python
class Carte:
    def __init__(self, valeur, couleur):
        self.valeur = valeur
        self.couleur = couleur

    def __lt__(self, autre):
        return self.valeur < autre.valeur

    def __repr__(self):
        return f"{self.valeur}{self.couleur}"

main = [Carte(10, "♥"), Carte(3, "♠"), Carte(7, "♦")]
print(sorted(main))
print(max(main))
```

```sortie
[3♠, 7♦, 10♥]
10♥
```

### Mesurer avec __len__

`__len__` permet d'utiliser `len()`. En bonus, un objet de longueur 0 est considéré comme
faux dans un `if`.

```python
class Panier:
    def __init__(self):
        self.articles = []

    def ajouter(self, article):
        self.articles.append(article)

    def __len__(self):
        return len(self.articles)

    def __contains__(self, article):
        return article in self.articles

panier = Panier()
if not panier:
    print("Panier vide")
panier.ajouter("pain")
panier.ajouter("lait")
print(len(panier))
print("pain" in panier)
```

```sortie
Panier vide
2
True
```

> **Astuce :** écrire toutes ces méthodes à la main est vite répétitif. Les *dataclasses*
> les génèrent pour vous : voir [les annotations de type](typage.md).

## Erreurs fréquentes

### Oublier super().__init__

```python norun
class Animal:
    def __init__(self, nom):
        self.nom = nom

class Chat(Animal):
    def __init__(self, nom, couleur):
        self.couleur = couleur

minou = Chat("Minou", "gris")
print(minou.nom)
```

```text
Traceback (most recent call last):
  File "main.py", line 10, in <module>
    print(minou.nom)
          ^^^^^^^^^
AttributeError: 'Chat' object has no attribute 'nom'
```

Le `__init__` de `Chat` remplace celui d'`Animal`, qui n'est donc jamais appelé.
Ajoutez `super().__init__(nom)` au début du `__init__` de `Chat`.

### Oublier une méthode abstraite

```python norun
from abc import ABC, abstractmethod

class Forme(ABC):
    @abstractmethod
    def aire(self):
        pass

class Carre(Forme):
    def __init__(self, cote):
        self.cote = cote

c = Carre(3)
```

```text
Traceback (most recent call last):
  File "main.py", line 12, in <module>
    c = Carre(3)
TypeError: Can't instantiate abstract class Carre without an implementation for abstract method 'aire'
```

`Carre` n'a pas écrit `aire` : elle reste abstraite. Ajoutez la méthode dans `Carre`.

### Trier des objets sans __lt__

```python norun
class Carte:
    def __init__(self, valeur):
        self.valeur = valeur

cartes = [Carte(7), Carte(2)]
print(sorted(cartes))
```

```text
Traceback (most recent call last):
  File "main.py", line 6, in <module>
    print(sorted(cartes))
          ~~~~~~^^^^^^^^
TypeError: '<' not supported between instances of 'Carte' and 'Carte'
```

Python ne sait pas quelle carte est la plus petite. Écrivez `__lt__`, ou indiquez une
clé de tri : `sorted(cartes, key=lambda c: c.valeur)`.

### Appeler len sans __len__

```python norun
class Panier:
    def __init__(self):
        self.articles = ["pain", "lait"]

print(len(Panier()))
```

```text
Traceback (most recent call last):
  File "main.py", line 5, in <module>
    print(len(Panier()))
          ~~~^^^^^^^^^^
TypeError: object of type 'Panier' has no len()
```

Ajoutez une méthode `__len__` qui renvoie `len(self.articles)`.

## Testez-vous

**Question 1 :** un `Velo` a deux `Roue`. Héritage ou composition ?

> **Réponse**
>
> Composition : un vélo **a** des roues, il n'**est** pas une roue. On écrira
> `self.roues = [Roue(), Roue()]` dans `Velo`.

**Question 2 :** que renvoie `isinstance(True, int)` ? (Indice : `bool` est une
sous-classe de `int`.)

> **Réponse**
>
> `True`. `isinstance` accepte aussi les sous-classes, et `bool` hérite de `int`.

**Question 3 :** écrivez une classe `Note` avec un attribut `valeur`, triable avec
`sorted`.

> **Réponse**
>
> ```python
> class Note:
>     def __init__(self, valeur):
>         self.valeur = valeur
>
>     def __lt__(self, autre):
>         return self.valeur < autre.valeur
>
>     def __repr__(self):
>         return f"Note({self.valeur})"
>
> print(sorted([Note(15), Note(8), Note(12)]))
> ```
>
> ```sortie
> [Note(8), Note(12), Note(15)]
> ```

## En résumé

- `class Enfant(Parent):` hérite des attributs et des méthodes du parent.
- `super()` appelle la version du parent, notamment dans `__init__`.
- Le polymorphisme : le même appel donne un comportement adapté à chaque objet ; avec le
  *duck typing*, seule compte la présence de la méthode.
- `ABC` et `@abstractmethod` obligent les sous-classes à écrire certaines méthodes.
- Préférez la composition (« a un ») quand « est un » ne sonne pas juste.
- `__eq__`, `__lt__` et `__len__` font marcher `==`, `sorted` et `len` sur vos objets.

Suite : [les annotations de type](typage.md). Comparez avec
[l'héritage en Ruby](../ruby/heritage.md).
