# Classes et objets

Cette page explique comment créer vos propres types avec `class` : leurs attributs, leurs méthodes, leur affichage et leurs propriétés.

> **Objectifs :**
>
> - vous saurez écrire une classe avec `__init__` et créer des objets ;
> - vous saurez ce que représente `self` ;
> - vous saurez ajouter des méthodes et un affichage avec `__str__` et `__repr__` ;
> - vous saurez utiliser `@property`, `@classmethod` et `@staticmethod` ;
> - vous saurez qu'une variable contient une **référence** vers un objet.

## Pourquoi des classes ?

Jusqu'ici, vous avez rangé vos données dans des listes et des dictionnaires. Pour un
compte bancaire, cela donne :

```python
compte = {"titulaire": "Alice", "solde": 100}
compte["solde"] += 50
print(compte)
```

```sortie
{'titulaire': 'Alice', 'solde': 150}
```

Cela marche, mais rien n'empêche une faute de frappe (`"sold"`), et les actions
(déposer, retirer) sont éparpillées ailleurs dans le code.

Une [classe](../debuter/glossaire.md#classe) regroupe **les données et les actions** au
même endroit. C'est un plan de construction : la classe `Maison` décrit les maisons ;
chaque maison construite à partir du plan est un [objet](../debuter/glossaire.md#objet),
aussi appelé **instance**.

> **Note :** en Python, vous utilisez des objets depuis le début : `"abc"` est un objet
> de la classe `str`, `[1, 2]` un objet de la classe `list`. `type("abc")` affiche
> `<class 'str'>`.

## Une première classe

On écrit `class`, un nom en `CamelCase` (une majuscule à chaque mot), puis deux-points.
On crée un objet en **appelant** la classe comme une fonction.

```python
class Chien:
    pass

rex = Chien()
medor = Chien()
print(type(rex))
print(rex is medor)
```

```sortie
<class '__main__.Chien'>
False
```

`pass` veut dire « rien pour l'instant ». `rex` et `medor` sont deux objets différents,
fabriqués avec le même plan. `__main__` est le nom du fichier principal.

## __init__ et self

La méthode spéciale `__init__` (*initialiser*) s'exécute automatiquement à la création
de l'objet. Elle sert à remplir ses **attributs** : les variables propres à chaque objet.

```python
class Chien:
    def __init__(self, nom, age):
        self.nom = nom
        self.age = age

rex = Chien("Rex", 3)
medor = Chien("Médor", 5)
print(rex.nom, rex.age)
print(medor.nom, medor.age)
```

```sortie
Rex 3
Médor 5
```

Qui est `self` ? C'est **l'objet en cours de construction**. Quand vous écrivez
`Chien("Rex", 3)`, Python :

1. crée un objet vide ;
2. appelle `__init__(objet, "Rex", 3)` : `self` est cet objet ;
3. vous renvoie l'objet rempli.

`self.nom = nom` veut donc dire : « range la valeur `nom` dans l'attribut `nom` de cet
objet ». Sans `self.`, la valeur resterait une simple variable locale, perdue à la fin
de `__init__`.

> **À retenir :** `self` est toujours le **premier paramètre** des méthodes, mais vous ne
> le passez jamais vous-même : Python le fait pour vous.

## Les méthodes

Une [méthode](../debuter/glossaire.md#methode) est une fonction écrite dans la classe.
Elle reçoit `self` et peut donc lire et modifier les attributs de l'objet.

```python
class Compte:
    def __init__(self, titulaire, solde=0):
        self.titulaire = titulaire
        self.solde = solde

    def deposer(self, montant):
        self.solde += montant

    def retirer(self, montant):
        if montant > self.solde:
            print("Solde insuffisant")
            return
        self.solde -= montant

compte = Compte("Alice", 100)
compte.deposer(50)
compte.retirer(500)
compte.retirer(30)
print(compte.solde)
```

```sortie
Solde insuffisant
120
```

`compte.deposer(50)` est un raccourci pour `Compte.deposer(compte, 50)` : l'objet placé
avant le point devient `self`.

## Afficher un objet : __str__ et __repr__

Par défaut, `print` affiche un objet de façon peu utile, avec une adresse mémoire qui
change à chaque exécution (par exemple `<__main__.Point object at 0x7f3a…>`). Deux
méthodes spéciales règlent ce problème :

- `__str__` : le texte pour un **humain**, utilisé par `print` et `str()` ;
- `__repr__` : le texte pour un **développeur**, utilisé dans les listes, par `repr()`
  et dans la console. Idéalement, il ressemble au code qui recrée l'objet.

```python
class Point:
    def __init__(self, x, y):
        self.x = x
        self.y = y

    def __str__(self):
        return f"({self.x}, {self.y})"

    def __repr__(self):
        return f"Point({self.x}, {self.y})"

p = Point(2, 5)
print(p)
print(repr(p))
print([p, Point(0, 0)])
```

```sortie
(2, 5)
Point(2, 5)
[Point(2, 5), Point(0, 0)]
```

> **Astuce :** si vous n'écrivez qu'une seule des deux, écrivez `__repr__` : `print` s'en
> sert aussi quand `__str__` manque.

## Attributs de classe

Un attribut écrit **directement dans la classe** (pas dans `__init__`) est partagé par
tous les objets. Il sert pour une constante commune, ou un compteur.

```python
class Chien:
    espece = "Canis familiaris"
    nombre = 0

    def __init__(self, nom):
        self.nom = nom
        Chien.nombre += 1

a = Chien("Rex")
b = Chien("Médor")
print(a.espece, b.espece)
print(Chien.nombre)
```

```sortie
Canis familiaris Canis familiaris
2
```

> **Attention :** n'utilisez jamais une liste ou un dictionnaire comme attribut de
> classe si chaque objet doit avoir le sien : tous les objets partageraient **la même**
> liste. Créez-la dans `__init__` avec `self.tours = []`.

## Les propriétés avec @property

Parfois, une valeur se **calcule** à partir d'autres attributs. `@property` transforme
une méthode en attribut en lecture : on l'utilise sans parenthèses.

```python
class Rectangle:
    def __init__(self, largeur, hauteur):
        self.largeur = largeur
        self.hauteur = hauteur

    @property
    def aire(self):
        return self.largeur * self.hauteur

r = Rectangle(3, 4)
print(r.aire)
r.largeur = 10
print(r.aire)
```

```sortie
12
40
```

L'aire est toujours à jour, car elle est recalculée à chaque lecture.

### Contrôler une modification

Avec `@nom.setter`, vous pouvez vérifier une valeur avant de l'accepter. Par convention,
la vraie valeur est rangée dans un attribut qui commence par `_` : cela signifie
« interne, ne pas toucher directement ».

```python
class Thermostat:
    def __init__(self, temperature):
        self.temperature = temperature

    @property
    def temperature(self):
        return self._temperature

    @temperature.setter
    def temperature(self, valeur):
        if valeur < 5 or valeur > 30:
            raise ValueError(f"température hors limites : {valeur}")
        self._temperature = valeur

t = Thermostat(20)
t.temperature = 22
print(t.temperature)
try:
    t.temperature = 80
except ValueError as e:
    print(e)
```

```sortie
22
température hors limites : 80
```

`raise` et `try` sont expliqués dans [gérer les erreurs](erreurs.md).

## @classmethod et @staticmethod

Deux autres décorateurs changent ce que reçoit une méthode :

| Décorateur | Premier paramètre | Usage |
|------------|-------------------|-------|
| aucun | `self`, l'objet | méthode normale |
| `@classmethod` | `cls`, la classe | autre façon de **créer** un objet |
| `@staticmethod` | rien | fonction utilitaire rangée dans la classe |

```python
class Date:
    def __init__(self, jour, mois, annee):
        self.jour = jour
        self.mois = mois
        self.annee = annee

    @classmethod
    def depuis_texte(cls, texte):
        jour, mois, annee = texte.split("/")
        return cls(int(jour), int(mois), int(annee))

    @staticmethod
    def est_bissextile(annee):
        return annee % 4 == 0 and (annee % 100 != 0 or annee % 400 == 0)

    def __repr__(self):
        return f"Date({self.jour}, {self.mois}, {self.annee})"

d = Date.depuis_texte("14/07/2024")
print(d)
print(Date.est_bissextile(2024))
```

```sortie
Date(14, 7, 2024)
True
```

`depuis_texte` est un « constructeur alternatif » : il prépare les valeurs puis appelle
`cls(...)`, c'est-à-dire `Date(...)`.

## Les objets sont des références

Une variable ne **contient** pas l'objet : elle contient une **référence** vers lui, comme
une étiquette collée dessus. Deux variables peuvent porter deux étiquettes sur le même
objet.

```python
class Compte:
    def __init__(self, solde):
        self.solde = solde

a = Compte(100)
b = a
b.solde = 0
print(a.solde)
print(a is b)
```

```sortie
0
True
```

`b = a` ne copie pas le compte : `a` et `b` désignent le **même** objet. C'est pareil
quand vous passez un objet à une fonction : la fonction peut le modifier.

```python
class Compte:
    def __init__(self, solde):
        self.solde = solde

def vider(compte):
    compte.solde = 0

c = Compte(100)
vider(c)
print(c.solde)
```

```sortie
0
```

Pour une vraie copie indépendante, utilisez le module `copy` :

```python
import copy

class Compte:
    def __init__(self, solde):
        self.solde = solde

a = Compte(100)
b = copy.copy(a)
b.solde = 0
print(a.solde, b.solde)
```

```sortie
100 0
```

> **Note :** `is` répond à « est-ce le même objet ? », alors que `==` répond à « ont-ils
> la même valeur ? ». Pour vos classes, `==` compare l'identité tant que vous n'avez pas
> écrit `__eq__` (voir [héritage et polymorphisme](heritage.md)).

## Erreurs fréquentes

### Oublier self dans une méthode

```python norun
class Chien:
    def aboyer():
        print("Wouf !")

rex = Chien()
rex.aboyer()
```

```text
Traceback (most recent call last):
  File "main.py", line 6, in <module>
    rex.aboyer()
    ~~~~~~~~~~^^
TypeError: Chien.aboyer() takes 0 positional arguments but 1 was given
```

Python passe toujours l'objet comme premier argument : « 1 was given », c'est `rex`.
Ajoutez `self` : `def aboyer(self):`.

### Oublier self. devant un attribut

```python norun
class Chien:
    def __init__(self, nom):
        nom = nom

rex = Chien("Rex")
print(rex.nom)
```

```text
Traceback (most recent call last):
  File "main.py", line 6, in <module>
    print(rex.nom)
          ^^^^^^^
AttributeError: 'Chien' object has no attribute 'nom'
```

`nom = nom` ne range rien dans l'objet. Écrivez `self.nom = nom`.

### Oublier un argument du constructeur

```python norun
class Chien:
    def __init__(self, nom):
        self.nom = nom

rex = Chien()
```

```text
Traceback (most recent call last):
  File "main.py", line 5, in <module>
    rex = Chien()
TypeError: Chien.__init__() missing 1 required positional argument: 'nom'
```

Les paramètres de `__init__` (sauf `self`) sont ceux qu'on donne à la classe :
`Chien("Rex")`. Ou donnez une valeur par défaut : `def __init__(self, nom="Inconnu")`.

### Modifier une propriété sans setter

```python norun
class Cercle:
    def __init__(self, rayon):
        self.rayon = rayon

    @property
    def diametre(self):
        return self.rayon * 2

c = Cercle(3)
c.diametre = 10
```

```text
Traceback (most recent call last):
  File "main.py", line 10, in <module>
    c.diametre = 10
    ^^^^^^^^^^
AttributeError: property 'diametre' of 'Cercle' object has no setter
```

Une propriété sans `@diametre.setter` est en lecture seule. Modifiez `c.rayon`, ou
ajoutez un setter qui calcule `self.rayon = valeur / 2`.

## Testez-vous

**Question 1 :** quelle différence y a-t-il entre `self.nom` et `Chien.nombre` dans la
classe `Chien` vue plus haut ?

> **Réponse**
>
> `self.nom` est un attribut **d'objet** : chaque chien a le sien. `Chien.nombre` est un
> attribut **de classe** : une seule valeur, partagée par tous les chiens.

**Question 2 :** qu'affiche ce code ?

```python norun
class Boite:
    def __init__(self):
        self.contenu = []

a = Boite()
b = a
b.contenu.append("clé")
print(a.contenu)
```

> **Réponse**
>
> `['clé']`. `b = a` ne crée pas de nouvelle boîte : `a` et `b` désignent le même objet.

**Question 3 :** écrivez une classe `Carre` avec un attribut `cote` et une propriété
`perimetre`.

> **Réponse**
>
> ```python
> class Carre:
>     def __init__(self, cote):
>         self.cote = cote
>
>     @property
>     def perimetre(self):
>         return 4 * self.cote
>
> print(Carre(5).perimetre)
> ```
>
> ```sortie
> 20
> ```

## En résumé

- `class Nom:` définit un type ; `Nom(...)` crée un objet et appelle `__init__`.
- `self` est l'objet courant ; `self.x = ...` crée un attribut propre à l'objet.
- `__str__` donne l'affichage pour `print`, `__repr__` l'affichage pour le développeur.
- `@property` calcule un attribut ; `@classmethod` crée des objets autrement ;
  `@staticmethod` range une fonction utilitaire.
- Une variable est une référence : `b = a` ne copie pas l'objet.

Suite : [héritage et polymorphisme](heritage.md). Comparez avec
[les classes en Ruby](../ruby/types.md).
