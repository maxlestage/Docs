# Les collections avancées

Cette page présente les boîtes spécialisées de Python : `namedtuple`, `deque`, `heapq`, `bisect`, `Counter` et `defaultdict`, et vous aide à choisir la bonne pour chaque problème.

> **Objectifs :**
>
> - vous saurez donner des noms aux cases d'un tuple avec `namedtuple` ;
> - vous saurez gérer une file d'attente avec `deque` ;
> - vous saurez toujours sortir l'élément le plus prioritaire avec `heapq` ;
> - vous saurez chercher et insérer dans une liste triée avec `bisect` ;
> - vous saurez compter avec `Counter` et regrouper avec `defaultdict` ;
> - vous saurez choisir la bonne collection grâce à un tableau.

## Pourquoi d'autres collections ?

Vous connaissez déjà [les listes](collections.md), [les tuples](collections.md#les-tuples)
et [les dictionnaires](dictionnaires.md). Ils font presque tout. Mais pour certains
travaux, il existe un outil mieux adapté.

Pensez à une cuisine. Un couteau de cuisine coupe tout. Mais pour éplucher une pomme de
terre, un économe va plus vite et coupe moins les doigts. Les collections de cette page
sont des économes : chacune fait **une** chose, très bien.

```text
Le problème                          L'outil
─────────────────────────────────    ─────────────
« des cases avec des noms »      →   namedtuple
« premier arrivé, premier servi » →  deque
« le plus urgent d'abord »       →   heapq
« garder une liste triée »       →   bisect
« compter »                      →   Counter
« ranger par groupes »           →   defaultdict
```

Presque tous viennent du [module](../debuter/glossaire.md#module) `collections`.
`heapq` et `bisect` sont deux petits modules à part. Tout est livré avec Python : rien à
installer.

> **À retenir :** ces outils ne font rien qu'une liste ou un dictionnaire ne sache faire.
> Ils le font plus **clairement**, et souvent plus **vite**.

## namedtuple : un tuple avec des noms

Un [tuple](../debuter/glossaire.md#tuple) range des valeurs à des places numérotées.
`point[0]`, c'est quoi ? Le x ? Le y ? Il faut s'en souvenir.

Un `namedtuple` est un tuple dont chaque case porte une **étiquette**. C'est une fiche
d'identité : on lit « Nom », « Prénom », pas « case 0 », « case 1 ».

```python
from collections import namedtuple

Point = namedtuple("Point", ["x", "y"])

p = Point(3, 4)
print(p)
print(p.x)
print(p.y)
print(p[0])
```

```sortie
Point(x=3, y=4)
3
4
3
```

Ligne par ligne :

| Code | Ce qu'il fait |
|---|---|
| `namedtuple("Point", ["x", "y"])` | fabrique un nouveau **modèle** de fiche, avec deux cases |
| `Point(3, 4)` | remplit une fiche : `x` reçoit 3, `y` reçoit 4 |
| `p.x` | lit la case nommée `x` |
| `p[0]` | marche toujours : c'est un vrai tuple |

```text
         Point(3, 4)
   ┌──────────┬──────────┐
   │ x  =  3  │ y  =  4  │
   └──────────┴──────────┘
     p.x ou p[0]  p.y ou p[1]
```

> **Essayez :** créez `Point(y=10, x=2)`. L'ordre des arguments nommés n'a pas
> d'importance : `print` affiche `Point(x=2, y=10)`.

### Découper et changer une case

Un `namedtuple` se découpe comme un tuple. Il est [immuable](../debuter/glossaire.md#immuable) :
pour « changer » une case, `_replace` fabrique une **nouvelle** fiche.

```python
from collections import namedtuple

Eleve = namedtuple("Eleve", ["nom", "note"])

e = Eleve("Léa", 12)
nom, note = e
print(nom, note)

e2 = e._replace(note=15)
print(e)
print(e2)
print(e2._asdict())
```

```sortie
Léa 12
Eleve(nom='Léa', note=12)
Eleve(nom='Léa', note=15)
{'nom': 'Léa', 'note': 15}
```

`e` n'a pas bougé. `e2` est une copie avec une note différente. `_asdict()` transforme la
fiche en dictionnaire.

> **À retenir :** prenez un `namedtuple` pour un petit groupe fixe de valeurs (un point,
> une couleur, une ligne de tableau). Le code lit `e.note` au lieu de `e[1]` : il se
> comprend tout seul.

## deque : une file d'attente

Imaginez la file à la boulangerie. Les gens arrivent **à la fin**. On sert celui qui est
**au début**. Premier arrivé, premier servi.

Avec une liste, retirer le premier élément (`liste.pop(0)`) oblige Python à décaler tous
les autres d'une place. Avec 10 personnes, peu importe. Avec un million, c'est lent.

Une `deque` (on prononce « dèque », pour *double-ended queue*, « file à deux bouts »)
ajoute et retire **aux deux bouts** sans rien décaler.

```python
from collections import deque

file = deque()
file.append("Alice")
file.append("Bob")
file.append("Chloé")
print(file)

servi = file.popleft()
print("On sert", servi)
print(file)
```

```sortie
deque(['Alice', 'Bob', 'Chloé'])
On sert Alice
deque(['Bob', 'Chloé'])
```

Suivons la file à chaque étape :

| Étape | Code | File (début → fin) |
|---|---|---|
| 1 | `deque()` | *(vide)* |
| 2 | `append("Alice")` | Alice |
| 3 | `append("Bob")` | Alice, Bob |
| 4 | `append("Chloé")` | Alice, Bob, Chloé |
| 5 | `popleft()` → `"Alice"` | Bob, Chloé |

```text
             popleft()                    append(x)
   sortie  ◄──────────  [ Bob | Chloé ]  ◄──────────  entrée
            (début)                         (fin)
```

### Les quatre gestes

| Geste | Bout gauche (début) | Bout droit (fin) |
|---|---|---|
| ajouter | `appendleft(x)` | `append(x)` |
| retirer | `popleft()` | `pop()` |

```python
from collections import deque

d = deque([2, 3])
d.appendleft(1)
d.append(4)
print(d)
print(d.pop(), d.popleft())
print(d)
```

```sortie
deque([1, 2, 3, 4])
4 1
deque([2, 3])
```

### Garder les N derniers : maxlen

Avec `maxlen`, la deque a une taille maximale. Quand elle est pleine, ajouter à droite
fait tomber l'élément de gauche. C'est l'historique « 3 dernières pages visitées ».

```python
from collections import deque

historique = deque(maxlen=3)
for page in ["accueil", "produits", "panier", "paiement", "merci"]:
    historique.append(page)
    print(list(historique))
```

```sortie
['accueil']
['accueil', 'produits']
['accueil', 'produits', 'panier']
['produits', 'panier', 'paiement']
['panier', 'paiement', 'merci']
```

À la 4e page, la deque est pleine : `"accueil"` tombe par la gauche.

> **Essayez :** remplacez `maxlen=3` par `maxlen=2`. Chaque ligne a alors au plus deux
> pages, et la dernière affiche `['paiement', 'merci']`.

### Un exemple : servir les clients

```python
from collections import deque

clients = deque(["Alice", "Bob", "Chloé"])
tour = 1
while clients:
    nom = clients.popleft()
    print(f"Tour {tour} : je sers {nom}, il reste {len(clients)}")
    tour += 1
```

```sortie
Tour 1 : je sers Alice, il reste 2
Tour 2 : je sers Bob, il reste 1
Tour 3 : je sers Chloé, il reste 0
```

| Tour | `nom` | `clients` après `popleft` |
|---|---|---|
| 1 | Alice | Bob, Chloé |
| 2 | Bob | Chloé |
| 3 | Chloé | *(vide)* → `while` s'arrête |

`while clients:` continue tant que la deque n'est pas vide, comme pour une liste.

> **À retenir :** pour une file d'attente, utilisez `deque` avec `append` (entrer) et
> `popleft` (sortir). Jamais `liste.pop(0)` sur une grosse liste.

## heapq : le plus urgent d'abord

Aux urgences de l'hôpital, on ne soigne pas dans l'ordre d'arrivée. On soigne **le plus
grave d'abord**. C'est une **file de priorité**.

Le module `heapq` transforme une liste ordinaire en **tas** (*heap*). Un tas garde
toujours le **plus petit** élément à la place 0. Le reste n'est pas trié : c'est juste
assez rangé pour retrouver vite le plus petit.

```python
import heapq

tas = []
heapq.heappush(tas, 5)
heapq.heappush(tas, 1)
heapq.heappush(tas, 8)
heapq.heappush(tas, 3)
print("le plus petit :", tas[0])

print(heapq.heappop(tas))
print(heapq.heappop(tas))
print("reste :", sorted(tas))
```

```sortie
le plus petit : 1
1
3
reste : [5, 8]
```

| Code | Ce qu'il fait |
|---|---|
| `heappush(tas, x)` | ajoute `x` dans le tas |
| `tas[0]` | regarde le plus petit, sans le retirer |
| `heappop(tas)` | retire et renvoie le plus petit |

```text
On verse des nombres dans un entonnoir :
le plus petit tombe toujours en premier.

   5  1  8  3
   ╲        ╱
    ╲      ╱
     ╲    ╱
      ╲  ╱
       1      ← heappop() le sort
```

### Des tâches avec une priorité

On range des tuples `(priorité, tâche)`. Python compare les tuples **case par case** :
d'abord la priorité. Plus le nombre est petit, plus c'est urgent.

```python
import heapq

taches = []
heapq.heappush(taches, (3, "arroser les plantes"))
heapq.heappush(taches, (1, "éteindre le four"))
heapq.heappush(taches, (2, "répondre au facteur"))

while taches:
    priorite, tache = heapq.heappop(taches)
    print(priorite, tache)
```

```sortie
1 éteindre le four
2 répondre au facteur
3 arroser les plantes
```

L'ordre d'ajout était 3, 1, 2. L'ordre de sortie est 1, 2, 3.

| Tour | `heappop` sort | Il reste |
|---|---|---|
| 1 | (1, éteindre le four) | (2, …), (3, …) |
| 2 | (2, répondre au facteur) | (3, …) |
| 3 | (3, arroser les plantes) | *(vide)* |

> **Essayez :** ajoutez `(0, "appeler les pompiers")`. Elle sort en premier.

### Les N plus grands ou plus petits

Pour juste obtenir le podium, `nlargest` et `nsmallest` suffisent :

```python
import heapq

scores = [42, 17, 99, 63, 8, 75]
print(heapq.nlargest(3, scores))
print(heapq.nsmallest(2, scores))
```

```sortie
[99, 75, 63]
[8, 17]
```

> **À retenir :** `heapq` sort toujours le **plus petit** en premier. Rangez des tuples
> `(priorité, valeur)` pour une file de priorité. Le tas n'est **pas** une liste triée :
> n'affichez pas `tas` en espérant un ordre.

## bisect : chercher dans une liste triée

Pour trouver un mot dans le dictionnaire papier, vous ne lisez pas toutes les pages.
Vous l'ouvrez au milieu, vous regardez si le mot est avant ou après, et vous recommencez
avec la bonne moitié. C'est la **recherche par dichotomie**.

Le module `bisect` fait exactement ça, sur une liste **déjà triée**.

```python
import bisect

notes = [5, 8, 12, 15, 18]
print(bisect.bisect(notes, 13))
```

```sortie
3
```

`bisect(notes, 13)` répond : « pour garder la liste triée, 13 irait à la place 3 »
(juste avant 15). Voici la recherche pas à pas :

| Étape | Morceau regardé | Milieu | 13 est… |
|---|---|---|---|
| 1 | `[5, 8, 12, 15, 18]` (places 0–4) | place 2 → 12 | après 12 → on garde la droite |
| 2 | `[15, 18]` (places 3–4) | place 4 → 18 | avant 18 → on garde la gauche |
| 3 | `[15]` (place 3) | place 3 → 15 | avant 15 → c'est ici : **3** |

Trois coups d'œil pour 5 éléments. Pour un million d'éléments, une vingtaine suffit.

### insort : insérer au bon endroit

`insort` insère un élément **à sa place**. La liste reste triée, sans appeler `sort`.

```python
import bisect

notes = [5, 8, 15, 18]
bisect.insort(notes, 12)
bisect.insort(notes, 1)
print(notes)
```

```sortie
[1, 5, 8, 12, 15, 18]
```

### Un barème avec bisect

Un usage classique : transformer une note en mention. Les seuils sont triés ; `bisect`
dit dans quelle « tranche » tombe la note.

```python
import bisect

seuils = [10, 12, 14, 16]
mentions = ["Échec", "Passable", "Assez bien", "Bien", "Très bien"]

def mention(note):
    return mentions[bisect.bisect(seuils, note)]

for note in [7, 10, 13, 17]:
    print(note, "→", mention(note))
```

```sortie
7 → Échec
10 → Passable
13 → Assez bien
17 → Très bien
```

```text
seuils :        10        12        14        16
         ───────┼─────────┼─────────┼─────────┼────────
place :     0   │    1    │    2    │    3    │    4
         Échec  │Passable │Assez b. │  Bien   │Très bien
                ▲            ▲                     ▲
              note 10     note 13               note 17
```

| `note` | `bisect(seuils, note)` | `mentions[...]` |
|---|---|---|
| 7 | 0 | Échec |
| 10 | 1 | Passable |
| 13 | 2 | Assez bien |
| 17 | 4 | Très bien |

Une note égale à un seuil (10) va **après** lui : elle a la mention de la tranche du
dessus.

> **Essayez :** testez `mention(16)`. Vous obtenez `Très bien`. Avec
> `bisect.bisect_left`, une note égale au seuil irait **avant** : 16 donnerait `Bien`.

> **À retenir :** `bisect` ne marche que sur une liste **triée**. Sur une liste en
> désordre, il ne plante pas : il donne une réponse fausse.

## Counter en détail

`Counter` a déjà été présenté dans [les dictionnaires](dictionnaires.md#avec-collections-counter).
C'est un dictionnaire dont les valeurs sont des **nombres d'apparitions**. Pensez au
pointage des votes au tableau : un trait par voix.

```python
from collections import Counter

votes = ["pizza", "sushi", "pizza", "tacos", "pizza", "sushi"]
c = Counter(votes)
print(c)
print(c["pizza"], c["crêpes"])
print(c.most_common(1))
```

```sortie
Counter({'pizza': 3, 'sushi': 2, 'tacos': 1})
3 0
[('pizza', 3)]
```

Pas à pas, Counter fait ceci pour vous :

| Vote lu | pizza | sushi | tacos |
|---|---|---|---|
| pizza | 1 | 0 | 0 |
| sushi | 1 | 1 | 0 |
| pizza | 2 | 1 | 0 |
| tacos | 2 | 1 | 1 |
| pizza | 3 | 1 | 1 |
| sushi | 3 | 2 | 1 |

### Ajouter, retirer, additionner

`update` ajoute des comptes. `subtract` en retire. Deux `Counter` s'additionnent avec
`+`.

```python
from collections import Counter

stock = Counter(pommes=5, poires=2)
stock.update({"pommes": 3, "kiwis": 4})
print(stock)

stock.subtract({"pommes": 6})
print(stock["pommes"])

lundi = Counter(cafe=3, the=1)
mardi = Counter(cafe=2, chocolat=1)
print(lundi + mardi)
print(sum(lundi.values()))
```

```sortie
Counter({'pommes': 8, 'kiwis': 4, 'poires': 2})
2
Counter({'cafe': 5, 'the': 1, 'chocolat': 1})
4
```

> **Essayez :** retirez 10 pommes au lieu de 6. `stock["pommes"]` vaut alors `-2` :
> `Counter` accepte les comptes négatifs.

> **À retenir :** dès que vous écrivez `compte[x] = compte.get(x, 0) + 1`, pensez
> `Counter`. Une clé absente vaut 0, sans erreur.

## defaultdict en détail

`defaultdict` a lui aussi été vu dans [les dictionnaires](dictionnaires.md#avec-defaultdict).
C'est un dictionnaire qui, quand on lit une clé absente, **crée** une valeur de départ
au lieu de lever `KeyError`.

C'est un classeur magique : vous demandez l'intercalaire « B », il n'existe pas, le
classeur le fabrique tout seul, vide, et vous le tend.

On lui donne une **fonction sans argument** qui fabrique la valeur de départ :

| On écrit | Valeur créée pour une clé absente |
|---|---|
| `defaultdict(int)` | `0` |
| `defaultdict(list)` | `[]` |
| `defaultdict(set)` | `set()` |
| `defaultdict(str)` | `""` |

### Regrouper

```python
from collections import defaultdict

eleves = [("Léa", "CM1"), ("Hugo", "CM2"), ("Inès", "CM1"), ("Tom", "CM2"), ("Zoé", "CM1")]

classes = defaultdict(list)
for nom, classe in eleves:
    classes[classe].append(nom)

for classe, noms in classes.items():
    print(classe, noms)
```

```sortie
CM1 ['Léa', 'Inès', 'Zoé']
CM2 ['Hugo', 'Tom']
```

| Tour | `nom`, `classe` | La clé existe ? | `classes` après le tour |
|---|---|---|---|
| 1 | Léa, CM1 | non → `[]` créé | CM1: [Léa] |
| 2 | Hugo, CM2 | non → `[]` créé | CM1: [Léa], CM2: [Hugo] |
| 3 | Inès, CM1 | oui | CM1: [Léa, Inès], CM2: [Hugo] |
| 4 | Tom, CM2 | oui | CM1: [Léa, Inès], CM2: [Hugo, Tom] |
| 5 | Zoé, CM1 | oui | CM1: [Léa, Inès, Zoé], CM2: [Hugo, Tom] |

### Additionner par groupe

Avec `int`, la valeur de départ est 0 : parfait pour faire des totaux.

```python
from collections import defaultdict

achats = [("Léa", 12.5), ("Hugo", 8.0), ("Léa", 4.5), ("Hugo", 2.0), ("Inès", 10.0)]

total = defaultdict(int)
for nom, prix in achats:
    total[nom] += prix

print(dict(total))
```

```sortie
{'Léa': 17.0, 'Hugo': 10.0, 'Inès': 10.0}
```

### Le piège : lire suffit à créer

Avec `defaultdict`, **regarder** une clé absente la crée. Pour tester sans créer,
utilisez `in`.

```python
from collections import defaultdict

d = defaultdict(list)
print("x" in d)
print(d["x"])
print("x" in d)
print(dict(d))
```

```sortie
False
[]
True
{'x': []}
```

> **À retenir :** `defaultdict(list)` pour regrouper, `defaultdict(int)` pour
> additionner. Testez une clé avec `in`, pas avec `d[cle]`.

## Quelle collection choisir ?

Voici le tableau de choix. Lisez la colonne « Vous voulez… » et prenez l'outil en face.

| Vous voulez… | Outil | Geste principal | Exemple de la vie |
|---|---|---|---|
| une suite ordonnée, modifiable | `list` | `append`, `[i]` | une liste de courses |
| des cases nommées, fixes | `namedtuple` | `p.x` | une fiche d'identité |
| servir dans l'ordre d'arrivée | `deque` | `append` + `popleft` | la file à la boulangerie |
| garder les N derniers | `deque(maxlen=N)` | `append` | l'historique du navigateur |
| sortir le plus urgent | `heapq` | `heappush` + `heappop` | les urgences de l'hôpital |
| le podium des N meilleurs | `heapq.nlargest` | un seul appel | les 3 meilleurs scores |
| chercher dans une liste triée | `bisect` | `bisect`, `insort` | le dictionnaire papier |
| compter des apparitions | `Counter` | `Counter(liste)`, `most_common` | le dépouillement des votes |
| ranger par groupes | `defaultdict(list)` | `d[cle].append(x)` | les intercalaires d'un classeur |
| retrouver par une clé | `dict` | `d[cle]` | un annuaire |
| sans doublons | `set` | `add`, `in` | la liste des invités |

```text
Je dois…
 ├─ compter ?                          → Counter
 ├─ regrouper ?                        → defaultdict(list)
 ├─ traiter dans l'ordre d'arrivée ?   → deque
 ├─ traiter le plus important d'abord ?→ heapq
 ├─ garder une liste toujours triée ?  → bisect
 ├─ nommer les cases d'un tuple ?      → namedtuple
 └─ rien de tout ça                    → list ou dict, tout simplement
```

> **À retenir :** commencez toujours par une liste ou un dictionnaire. Passez à un outil
> spécialisé quand votre code ressemble à l'une des lignes du tableau.

## Erreurs fréquentes

### Modifier un namedtuple

```python norun
from collections import namedtuple
Point = namedtuple("Point", ["x", "y"])
p = Point(1, 2)
p.x = 5
```

```text
Traceback (most recent call last):
  File "main.py", line 4, in <module>
    p.x = 5
    ^^^
AttributeError: can't set attribute
```

Un `namedtuple` est immuable, comme un tuple. Fabriquez une nouvelle fiche :
`p = p._replace(x=5)`. Si vous devez souvent modifier, prenez plutôt une
[dataclass](typage.md#les-dataclasses).

### Retirer d'une deque vide

```python norun
from collections import deque
file = deque()
file.popleft()
```

```text
Traceback (most recent call last):
  File "main.py", line 3, in <module>
    file.popleft()
    ~~~~~~~~~~~~^^
IndexError: pop from an empty deque
```

Il n'y a personne dans la file. Vérifiez avant : `if file: file.popleft()`, ou bouclez
avec `while file:`.

### Deux priorités égales avec des valeurs non comparables

```python norun
import heapq
taches = []
heapq.heappush(taches, (2, {"nom": "laver"}))
heapq.heappush(taches, (2, {"nom": "ranger"}))
```

```text
Traceback (most recent call last):
  File "main.py", line 4, in <module>
    heapq.heappush(taches, (2, {"nom": "ranger"}))
    ~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
TypeError: '<' not supported between instances of 'dict' and 'dict'
```

Les deux priorités valent 2. Python compare alors la case suivante : deux dictionnaires,
qu'il ne sait pas comparer. Ajoutez un numéro d'ordre unique au milieu :
`(2, numero, {"nom": "ranger"})`, avec `numero` qui augmente de 1 à chaque ajout.

### bisect sur une liste pas triée

```python
import bisect

notes = [15, 5, 18, 8]
print(bisect.bisect(notes, 10))
```

```sortie
2
```

Pas d'erreur, mais la réponse n'a pas de sens : la place 2 tombe entre 5 et 18. Dans la
liste triée `[5, 8, 15, 18]`, 10 irait à la place 2, entre 8 et 15 : c'est un hasard. Triez
d'abord avec `notes.sort()`, puis gardez la liste triée avec `insort`.

## Testez-vous

**Question 1 :** vous écrivez un jeu où les joueurs attendent leur tour, dans l'ordre
d'arrivée. Quelle collection prenez-vous, et avec quelles méthodes ?

> **Réponse**
>
> Une `deque`. On ajoute un joueur avec `append` et on sert le suivant avec `popleft`.
> Contrairement à `liste.pop(0)`, `popleft` ne décale rien, même avec beaucoup de
> joueurs.

**Question 2 :** qu'affiche ce programme ?

```python norun
import heapq
tas = []
for x in [7, 2, 9, 4]:
    heapq.heappush(tas, x)
print(heapq.heappop(tas), heapq.heappop(tas))
```

> **Réponse**
>
> ```python
> import heapq
> tas = []
> for x in [7, 2, 9, 4]:
>     heapq.heappush(tas, x)
> print(heapq.heappop(tas), heapq.heappop(tas))
> ```
>
> ```sortie
> 2 4
> ```
>
> `heappop` sort toujours le plus petit : d'abord 2, puis 4.

**Question 3 :** quelle est la différence entre `Counter` et `defaultdict(int)` ?

> **Réponse**
>
> Les deux donnent 0 pour une clé absente. Mais `Counter` compte une liste entière d'un
> coup (`Counter(votes)`) et offre `most_common`. De plus, lire une clé absente d'un
> `Counter` ne la crée pas, alors qu'un `defaultdict` l'ajoute.

## En résumé

- `namedtuple` donne des noms aux cases d'un tuple : `p.x` au lieu de `p[0]`.
- `deque` est une file à deux bouts : `append` pour entrer, `popleft` pour sortir ;
  `maxlen` garde les N derniers.
- `heapq` sort toujours le plus petit : rangez des tuples `(priorité, valeur)`.
- `bisect` cherche et `insort` insère dans une liste **triée**, très vite.
- `Counter` compte, `defaultdict(list)` regroupe, `defaultdict(int)` additionne.
- En cas de doute, commencez par une liste ou un dictionnaire.

Suite : [Les décorateurs](decorateurs.md)
