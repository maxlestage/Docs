# Analyser des données

Cette page montre comment lire un tableau de données CSV, calculer des statistiques simples (moyenne, médiane, mode, écart type), regrouper, compter, trier et présenter un petit rapport, avec la bibliothèque standard puis avec pandas.

> **Objectifs :**
>
> - vous saurez lire des données CSV et convertir les nombres ;
> - vous saurez calculer moyenne, médiane, mode et écart type avec `statistics` ;
> - vous saurez regrouper et compter avec `Counter` et `defaultdict` ;
> - vous saurez trier des lignes selon une colonne ;
> - vous saurez afficher un rapport bien aligné ;
> - vous saurez à quoi ressemble le même travail avec pandas.

## Analyser, c'est répondre à une question

Imaginez le carnet d'une boulangerie. Chaque soir, la boulangère note ce qu'elle a vendu.
Au bout d'une semaine, elle se pose des questions :

- « Combien de baguettes je vends par jour, en moyenne ? »
- « Quel produit me rapporte le plus ? »
- « Quel jour est le plus calme ? »

Analyser des données, c'est ça : partir d'une **question**, faire un **calcul** sur le
tableau, obtenir une **réponse**.

```text
  question  ──►  lire les données  ──►  calculer  ──►  réponse
 « combien ? »    (le fichier CSV)      (moyenne,      « 41,7 par jour »
                                          total…)
```

Dans toute la page, nous utilisons le même petit tableau de ventes :

| jour | produit | quantite | prix |
|---|---|---|---|
| lundi | baguette | 40 | 1.10 |
| lundi | croissant | 25 | 1.20 |
| lundi | tarte | 3 | 15.00 |
| mardi | baguette | 35 | 1.10 |
| mardi | croissant | 30 | 1.20 |
| mercredi | baguette | 50 | 1.10 |
| mercredi | croissant | 20 | 1.20 |
| mercredi | tarte | 5 | 15.00 |

> **À retenir :** commencez toujours par écrire votre question en une phrase. Elle vous
> dit quelles colonnes regarder et quel calcul faire.

## Lire des données CSV

Le format CSV a été présenté dans [les formats de fichiers](formats.md#csv-des-tableaux-en-texte).
D'habitude, on lit un fichier. Ici, pour que chaque exemple tienne en un bloc, le texte
CSV est **dans** le programme. `io.StringIO` le fait passer pour un fichier : c'est un
faux fichier, en [mémoire vive](../debuter/glossaire.md#memoire-vive).

```python
import csv
import io

TEXTE = """jour,produit,quantite,prix
lundi,baguette,40,1.10
lundi,croissant,25,1.20
"""

lignes = list(csv.DictReader(io.StringIO(TEXTE)))
print(len(lignes), "lignes")
print(lignes[0])
```

```sortie
2 lignes
{'jour': 'lundi', 'produit': 'baguette', 'quantite': '40', 'prix': '1.10'}
```

Chaque ligne devient un [dictionnaire](../debuter/glossaire.md#dictionnaire). Les clés
viennent de la première ligne du CSV.

> **Note :** avec un vrai fichier, remplacez `io.StringIO(TEXTE)` par un fichier ouvert :
> `with open("ventes.csv", encoding="utf-8", newline="") as f:` puis `csv.DictReader(f)`.
> Le reste du code ne change pas.

### Convertir les nombres

Regardez bien : `'40'` a des guillemets. En CSV, **tout est texte**. Avant de calculer,
il faut convertir chaque colonne dans le bon type.

```python
import csv
import io

TEXTE = """jour,produit,quantite,prix
lundi,baguette,40,1.10
lundi,croissant,25,1.20
lundi,tarte,3,15.00
"""

ventes = []
for ligne in csv.DictReader(io.StringIO(TEXTE)):
    ventes.append({
        "jour": ligne["jour"],
        "produit": ligne["produit"],
        "quantite": int(ligne["quantite"]),
        "prix": float(ligne["prix"]),
    })

print(ventes[2])
print(ventes[2]["quantite"] * ventes[2]["prix"])
```

```sortie
{'jour': 'lundi', 'produit': 'tarte', 'quantite': 3, 'prix': 15.0}
45.0
```

| Colonne | Texte lu | Conversion | Valeur utilisable |
|---|---|---|---|
| jour | `'lundi'` | aucune | `'lundi'` |
| quantite | `'3'` | `int` | `3` |
| prix | `'15.00'` | `float` | `15.0` |

> **Essayez :** retirez le `int(...)` autour de `ligne["quantite"]`. La dernière ligne
> plante : `TypeError: can't multiply sequence by non-int of type 'float'`. On ne
> multiplie pas un texte par un nombre à virgule.

> **À retenir :** lire, puis **convertir tout de suite**. Ensuite, vous ne manipulez
> plus que des vrais nombres.

## Les statistiques de base

Le module `statistics` calcule les mesures classiques. Prenons les baguettes vendues
sur six jours.

```python
import statistics

baguettes = [40, 35, 50, 35, 42, 48]

print("moyenne :", round(statistics.mean(baguettes), 2))
print("médiane :", statistics.median(baguettes))
print("mode    :", statistics.mode(baguettes))
print("écart type :", round(statistics.stdev(baguettes), 2))
print("min, max :", min(baguettes), max(baguettes))
```

```sortie
moyenne : 41.67
médiane : 41.0
mode    : 35
écart type : 6.35
min, max : 35 50
```

Voyons ce que veut dire chaque mesure.

### La moyenne : partager équitablement

La moyenne, c'est le total partagé en parts égales. Si on mettait toutes les baguettes
en tas et qu'on les répartissait sur les six jours, chaque jour en aurait combien ?

| Étape | Valeur lue | Total |
|---|---|---|
| 1 | 40 | 40 |
| 2 | 35 | 75 |
| 3 | 50 | 125 |
| 4 | 35 | 160 |
| 5 | 42 | 202 |
| 6 | 48 | 250 |

Total 250, divisé par 6 jours : 41,67.

### La médiane : la valeur du milieu

On **trie** les valeurs, puis on prend celle du milieu. Comme une file d'enfants rangés
par taille : la médiane, c'est l'enfant au centre.

```text
triées :   35   35   40 │ 42   48   50
                        ▲
         6 valeurs : pas de milieu unique,
         on prend la moyenne des deux du centre :
         (40 + 42) / 2 = 41.0
```

Avec un nombre impair de valeurs, il y a un seul élément au milieu, et c'est lui.

Pourquoi la médiane ? Elle ne se laisse pas tromper par une valeur extrême :

```python
import statistics

salaires = [1800, 1900, 2000, 2100, 25000]
print("moyenne :", statistics.mean(salaires))
print("médiane :", statistics.median(salaires))
```

```sortie
moyenne : 6560
médiane : 2000
```

Un seul gros salaire fait grimper la moyenne bien au-dessus de ce que gagnent presque
tous. La médiane, elle, reste à 2000 : la moitié gagne moins, la moitié gagne plus.

### Le mode : le plus fréquent

Le mode est la valeur qui revient le plus souvent. Dans `[40, 35, 50, 35, 42, 48]`,
35 apparaît deux fois : c'est le mode. S'il y a égalité, `statistics.multimode` donne
toutes les gagnantes.

```python
import statistics

pointures = [38, 40, 38, 42, 40, 39]
print(statistics.mode(pointures))
print(statistics.multimode(pointures))
```

```sortie
38
[38, 40]
```

### L'écart type : est-ce que ça bouge beaucoup ?

L'écart type mesure à quel point les valeurs s'**éloignent** de la moyenne, en général.

- petit écart type : les valeurs sont serrées autour de la moyenne ;
- grand écart type : elles sont très dispersées.

```python
import statistics

regulier = [40, 41, 39, 40, 40]
irregulier = [10, 70, 25, 60, 35]
print(statistics.mean(regulier), round(statistics.stdev(regulier), 2))
print(statistics.mean(irregulier), round(statistics.stdev(irregulier), 2))
```

```sortie
40 0.71
40 24.75
```

```text
régulier    :  39 ● 40 ●●● 41 ●              (tout près de 40)
irrégulier  :  10 ●   25 ●  35 ●    60 ●  70 ●   (partout)
                         moyenne 40
```

Même moyenne, 40, mais des journées très différentes. Le premier boulanger sait combien
préparer ; le second ne sait jamais.

| Mesure | Fonction | Question à laquelle elle répond |
|---|---|---|
| moyenne | `statistics.mean` | « combien, en partageant équitablement ? » |
| médiane | `statistics.median` | « quelle est la valeur du milieu ? » |
| mode | `statistics.mode` | « quelle valeur revient le plus ? » |
| écart type | `statistics.stdev` | « est-ce que ça varie beaucoup ? » |

> **Essayez :** dans `salaires`, remplacez 25000 par 2200. La moyenne redescend à 2000
> et devient proche de la médiane : quand il n'y a pas de valeur extrême, les deux se
> ressemblent.

> **À retenir :** regardez la moyenne **et** la médiane. Si elles sont très
> différentes, une valeur extrême tire la moyenne.

## Regrouper et compter

La plupart des questions ont la forme « … **par** … » : les ventes **par** produit, le
chiffre d'affaires **par** jour. Il faut regrouper les lignes. Les outils sont ceux des
[collections avancées](collections-avancees.md).

### Compter les lignes avec Counter

« Combien de fois chaque produit apparaît-il dans le carnet ? »

```python
from collections import Counter

produits = ["baguette", "croissant", "tarte", "baguette",
            "croissant", "baguette", "croissant", "tarte"]
print(Counter(produits))
```

```sortie
Counter({'baguette': 3, 'croissant': 3, 'tarte': 2})
```

### Additionner par groupe avec defaultdict

« Combien d'unités de chaque produit ont été vendues ? » Il ne s'agit plus de compter
les lignes, mais d'**additionner** une colonne par groupe.

```python
import csv
import io
from collections import defaultdict

TEXTE = """jour,produit,quantite,prix
lundi,baguette,40,1.10
lundi,croissant,25,1.20
lundi,tarte,3,15.00
mardi,baguette,35,1.10
mardi,croissant,30,1.20
"""

total = defaultdict(int)
for ligne in csv.DictReader(io.StringIO(TEXTE)):
    total[ligne["produit"]] += int(ligne["quantite"])

print(dict(total))
```

```sortie
{'baguette': 75, 'croissant': 55, 'tarte': 3}
```

Suivons le dictionnaire `total` ligne après ligne :

| Ligne lue | produit | quantite | baguette | croissant | tarte |
|---|---|---|---|---|---|
| 1 | baguette | 40 | 40 | — | — |
| 2 | croissant | 25 | 40 | 25 | — |
| 3 | tarte | 3 | 40 | 25 | 3 |
| 4 | baguette | 35 | 75 | 25 | 3 |
| 5 | croissant | 30 | 75 | 55 | 3 |

Un tiret veut dire « la clé n'existe pas encore ». À sa première apparition,
`defaultdict(int)` la crée avec 0, puis on ajoute.

```text
Les lignes du carnet               Les boîtes par produit
 baguette  40 ──────────────►  ┌ baguette ┐  40 + 35 = 75
 croissant 25 ─────────┐       └──────────┘
 tarte      3 ──┐      └────►  ┌ croissant┐  25 + 30 = 55
 baguette  35 ──┼──► (baguette)└──────────┘
 croissant 30 ──┼──► (croissant)
                └──────────►   ┌  tarte   ┐  3
                               └──────────┘
```

> **Essayez :** remplacez `int(ligne["quantite"])` par
> `int(ligne["quantite"]) * float(ligne["prix"])`, et `defaultdict(int)` par
> `defaultdict(float)`. Vous obtenez le chiffre d'affaires par produit au lieu des
> quantités.

> **À retenir :** « compter les lignes par … » → `Counter`. « additionner une colonne
> par … » → `defaultdict(int)` avec `+=`.

## Trier

« Quel produit se vend le mieux ? » Il faut trier les groupes selon leur total. On trie
les paires `(produit, total)` avec `sorted` et une clé (voir
[trier selon un critère](collections.md#trier-selon-un-critere-avec-key)).

```python
total = {"baguette": 125, "croissant": 75, "tarte": 8}

par_total = sorted(total.items(), key=lambda paire: paire[1], reverse=True)
print(par_total)

for rang, (produit, quantite) in enumerate(par_total, start=1):
    print(rang, produit, quantite)
```

```sortie
[('baguette', 125), ('croissant', 75), ('tarte', 8)]
1 baguette 125
2 croissant 75
3 tarte 8
```

| Code | Ce qu'il fait |
|---|---|
| `total.items()` | donne les paires `('baguette', 125)`, `('croissant', 75)`… |
| `key=lambda paire: paire[1]` | trie selon la 2e case de la paire : le total |
| `reverse=True` | du plus grand au plus petit |
| `enumerate(..., start=1)` | numérote les rangs à partir de 1 |

### Trier des lignes selon deux colonnes

On peut trier les lignes elles-mêmes. Avec un tuple comme clé, Python trie d'abord
selon la première case, puis départage avec la seconde.

```python
ventes = [
    {"jour": "lundi", "produit": "tarte", "quantite": 3},
    {"jour": "mardi", "produit": "baguette", "quantite": 35},
    {"jour": "lundi", "produit": "baguette", "quantite": 40},
    {"jour": "mardi", "produit": "croissant", "quantite": 30},
]

for v in sorted(ventes, key=lambda v: (v["produit"], -v["quantite"])):
    print(v["produit"], v["jour"], v["quantite"])
```

```sortie
baguette lundi 40
baguette mardi 35
croissant mardi 30
tarte lundi 3
```

Le signe `-` devant la quantité inverse l'ordre pour cette colonne seulement : les
produits sont dans l'ordre alphabétique, et pour un même produit, la plus grosse vente
vient d'abord.

> **À retenir :** `sorted(..., key=...)` trie selon ce que vous voulez. `reverse=True`
> pour du plus grand au plus petit.

## Un petit rapport en tableau

Une réponse se lit mieux quand elle est bien alignée. Les
[f-strings](texte.md) savent réserver une largeur : `:<12` aligne à gauche sur 12
caractères, `:>6` à droite sur 6, `:>8.2f` à droite sur 8 avec 2 chiffres après la
virgule.

```python
import csv
import io
from collections import defaultdict

TEXTE = """jour,produit,quantite,prix
lundi,baguette,40,1.10
lundi,croissant,25,1.20
lundi,tarte,3,15.00
mardi,baguette,35,1.10
mardi,croissant,30,1.20
mercredi,baguette,50,1.10
mercredi,croissant,20,1.20
mercredi,tarte,5,15.00
"""

quantites = defaultdict(int)
recettes = defaultdict(float)
for ligne in csv.DictReader(io.StringIO(TEXTE)):
    q = int(ligne["quantite"])
    quantites[ligne["produit"]] += q
    recettes[ligne["produit"]] += q * float(ligne["prix"])

print(f"{'Produit':<12}{'Qté':>6}{'Recette':>10}")
print("-" * 28)
for produit in sorted(recettes, key=recettes.get, reverse=True):
    print(f"{produit:<12}{quantites[produit]:>6}{recettes[produit]:>10.2f}")
print("-" * 28)
print(f"{'Total':<12}{sum(quantites.values()):>6}{sum(recettes.values()):>10.2f}")
```

```sortie
Produit        Qté   Recette
----------------------------
baguette       125    137.50
tarte            8    120.00
croissant       75     90.00
----------------------------
Total          208    347.50
```

```text
 {produit:<12}         {quantité:>6}  {recette:>10.2f}
 ├───── 12 ─────┤├─── 6 ──┤├────── 10 ──────┤
 baguette             125          137.50
```

`key=recettes.get` trie les noms de produits selon leur recette : `recettes.get("tarte")`
vaut 120.0, etc.

> **Essayez :** remplacez `:<12` par `:>12`. Les noms de produits s'alignent alors à
> droite, collés aux nombres.

> **À retenir :** pour un rapport, une même largeur pour l'en-tête et pour les lignes.
> Les nombres à droite, le texte à gauche.

## Et pandas ?

Pour de gros tableaux, les analystes utilisent **pandas**, une
[bibliothèque](../debuter/glossaire.md#bibliotheque) externe. Elle n'est pas livrée avec
Python : on l'installe avec [pip](../debuter/glossaire.md#pip) (`pip install pandas`),
de préférence dans un [environnement virtuel](../debuter/glossaire.md#environnement-virtuel).
Les blocs ci-dessous ne sont donc pas exécutés sur ce site, mais leurs sorties sont
réelles.

pandas range les données dans un **DataFrame** : un tableau, comme une feuille de
tableur, avec des colonnes nommées. Les types sont devinés tout seuls.

```python norun
import io
import pandas as pd

TEXTE = """jour,produit,quantite,prix
lundi,baguette,40,1.10
lundi,croissant,25,1.20
lundi,tarte,3,15.00
mardi,baguette,35,1.10
mardi,croissant,30,1.20
mercredi,baguette,50,1.10
mercredi,croissant,20,1.20
mercredi,tarte,5,15.00
"""

df = pd.read_csv(io.StringIO(TEXTE))
print(df.head(3))
print(df["quantite"].mean())
```

```text
    jour    produit  quantite  prix
0  lundi   baguette        40   1.1
1  lundi  croissant        25   1.2
2  lundi      tarte         3  15.0
26.0
```

Regrouper, additionner et trier tient en une ligne :

```python norun
df["recette"] = df["quantite"] * df["prix"]
rapport = df.groupby("produit")[["quantite", "recette"]].sum()
print(rapport.sort_values("recette", ascending=False))
```

```text
           quantite  recette
produit
baguette        125    137.5
tarte             8    120.0
croissant        75     90.0
```

| Avec la bibliothèque standard | Avec pandas |
|---|---|
| `csv.DictReader` + conversions à la main | `pd.read_csv` (types devinés) |
| `statistics.mean(liste)` | `df["colonne"].mean()` |
| `defaultdict` + boucle | `df.groupby("produit").sum()` |
| `sorted(..., key=...)` | `df.sort_values("colonne")` |
| f-strings pour aligner | affichage aligné automatique |

> **À retenir :** pour quelques centaines de lignes, `csv` et `statistics` suffisent et
> ne demandent rien à installer. pandas devient utile quand les tableaux sont gros ou
> les questions nombreuses. Comprendre la version « à la main » d'abord aide à
> comprendre ce que pandas fait pour vous.

## Erreurs fréquentes

### Calculer avec des textes

```python norun
import statistics
notes = ["12", "15", "9"]
print(statistics.mean(notes))
```

```text
Traceback (most recent call last):
  ...
  File "/usr/lib/python3.13/statistics.py", line 334, in _exact_ratio
    raise TypeError(msg)
TypeError: can't convert type 'str' to numerator/denominator
```

Les valeurs lues dans un CSV sont des textes. Convertissez d'abord :
`statistics.mean([int(n) for n in notes])`.

### Un écart type avec une seule valeur

```python norun
import statistics
print(statistics.stdev([12]))
```

```text
Traceback (most recent call last):
  File "main.py", line 2, in <module>
    print(statistics.stdev([12]))
          ~~~~~~~~~~~~~~~~^^^^^^
  File "/usr/lib/python3.13/statistics.py", line 1207, in stdev
    raise StatisticsError('stdev requires at least two data points')
statistics.StatisticsError: stdev requires at least two data points
```

Avec une seule valeur, on ne peut pas mesurer de variation. De même,
`statistics.mean([])` lève `StatisticsError: mean requires at least one data point`.
Vérifiez la taille de la liste avant de calculer : `if len(valeurs) >= 2:`.

### Trier des nombres restés en texte

```python
quantites = ["40", "5", "125", "8"]
print(sorted(quantites))
print(sorted(quantites, key=int))
```

```sortie
['125', '40', '5', '8']
['5', '8', '40', '125']
```

Pas d'erreur, mais un ordre étrange : les textes sont triés **lettre par lettre**, comme
dans un dictionnaire. `"125"` commence par `1`, donc il passe avant `"40"`. Convertissez
en nombres, ou triez avec `key=int`.

## Testez-vous

**Question 1 :** les notes d'une classe sont `[2, 14, 15, 15, 16]`. Pourquoi la moyenne
et la médiane sont-elles différentes ?

> **Réponse**
>
> La moyenne vaut 12,4, la médiane vaut 15. La note 2, très basse, tire la moyenne vers
> le bas. La médiane, elle, prend seulement la valeur du milieu des notes triées.

**Question 2 :** qu'affiche ce programme ?

```python norun
from collections import defaultdict
achats = [("Léa", 5), ("Tom", 3), ("Léa", 2)]
total = defaultdict(int)
for nom, n in achats:
    total[nom] += n
print(dict(total))
```

> **Réponse**
>
> ```python
> from collections import defaultdict
> achats = [("Léa", 5), ("Tom", 3), ("Léa", 2)]
> total = defaultdict(int)
> for nom, n in achats:
>     total[nom] += n
> print(dict(total))
> ```
>
> ```sortie
> {'Léa': 7, 'Tom': 3}
> ```
>
> Léa : 0 + 5 + 2 = 7. Tom : 0 + 3 = 3.

**Question 3 :** vous lisez un CSV avec `csv.DictReader`. Pourquoi
`ligne["quantite"] * 2` donne-t-il `'4040'` au lieu de `80` ?

> **Réponse**
>
> Parce que `ligne["quantite"]` est le texte `'40'`. Multiplier un texte par 2 le
> répète. Il faut convertir : `int(ligne["quantite"]) * 2`.

## En résumé

- Partez d'une question ; elle dit quelles colonnes regarder et quoi calculer.
- `csv.DictReader` lit un CSV (fichier ou `io.StringIO`) ; tout est texte : convertissez
  avec `int` et `float` tout de suite.
- `statistics` donne `mean`, `median`, `mode` et `stdev` ; comparez moyenne et médiane.
- `Counter` compte les lignes par groupe, `defaultdict(int)` additionne une colonne par
  groupe.
- `sorted(..., key=..., reverse=True)` classe les résultats ; les f-strings `:<12` et
  `:>8.2f` alignent un rapport.
- pandas fait tout cela en quelques lignes, pour les gros tableaux.

Suite : [Les expressions régulières](regex.md)
