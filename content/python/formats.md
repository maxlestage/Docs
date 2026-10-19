# JSON et CSV

Cette page montre comment enregistrer et relire des données structurées avec les deux formats texte les plus utilisés : JSON et CSV.

> **Objectifs :**
>
> - vous saurez transformer un dictionnaire en texte JSON et l'inverse ;
> - vous saurez écrire et relire un fichier JSON ;
> - vous saurez lire un fichier CSV ligne par ligne ou sous forme de dictionnaires ;
> - vous saurez écrire un fichier CSV ;
> - vous saurez choisir le bon format selon vos données.

## Pourquoi un format ?

Un dictionnaire Python n'existe que dans la [mémoire vive](../debuter/glossaire.md#memoire-vive),
pendant que le programme tourne. Pour le garder dans un fichier, ou l'envoyer à un
autre programme, il faut le transformer en texte. Cette transformation s'appelle la
[sérialisation](../debuter/glossaire.md#serialisation). C'est comme mettre un meuble en
pièces détachées dans un carton, avec une notice pour le remonter ailleurs.

Un **format** est la notice : une règle commune pour écrire les données en texte, que
tous les langages savent relire.

## JSON : des données en texte

[JSON](../debuter/glossaire.md#json) (*JavaScript Object Notation*) ressemble beaucoup
aux dictionnaires et listes de Python. Le module `json` fait la conversion.

### json.dumps : de Python vers le texte

`json.dumps` (*dump string*, « vider dans une chaîne ») renvoie le texte JSON.

```python
import json

eleve = {"nom": "Léa", "age": 12, "notes": [15, 12.5], "inscrite": True, "club": None}
texte = json.dumps(eleve)
print(texte)
print(type(texte))
```

```sortie
{"nom": "L\u00e9a", "age": 12, "notes": [15, 12.5], "inscrite": true, "club": null}
<class 'str'>
```

Remarquez les différences : `True` devient `true`, `None` devient `null`. Et le `é` est
remplacé par un code, `\u00e9`. C'est correct, mais peu lisible.

### Un JSON lisible

Deux options rendent le texte agréable à lire :

- `ensure_ascii=False` garde les accents tels quels ;
- `indent=2` passe à la ligne et décale de 2 espaces à chaque niveau.

```python
import json

eleve = {"nom": "Léa", "age": 12, "notes": [15, 12.5]}
print(json.dumps(eleve, indent=2, ensure_ascii=False))
```

```sortie
{
  "nom": "Léa",
  "age": 12,
  "notes": [
    15,
    12.5
  ]
}
```

### Correspondance des types

| Python | JSON |
|--------|------|
| `dict` | objet `{ ... }` |
| `list`, `tuple` | tableau `[ ... ]` |
| `str` | chaîne `"..."` (toujours des guillemets doubles) |
| `int`, `float` | nombre |
| `True`, `False` | `true`, `false` |
| `None` | `null` |

Les autres types (ensembles, dates, objets de vos classes) ne passent pas directement :
il faut les convertir d'abord, par exemple un ensemble en liste.

### json.loads : du texte vers Python

`json.loads` (*load string*) fait le chemin inverse : il lit un texte JSON et renvoie
des objets Python.

```python
import json

texte = '{"ville": "Lyon", "habitants": 522000, "metro": true}'
donnees = json.loads(texte)

print(donnees["ville"])
print(donnees["habitants"] + 1)
print(donnees["metro"])
```

```sortie
Lyon
522001
True
```

## JSON dans un fichier

Avec les fichiers, on utilise `json.dump` et `json.load`, sans le `s` final : ils
écrivent et lisent directement dans un fichier ouvert.

```python
import json

taches = [
    {"titre": "Faire les courses", "faite": False},
    {"titre": "Appeler Mamie", "faite": True},
]

with open("taches.json", "w", encoding="utf-8") as f:
    json.dump(taches, f, indent=2, ensure_ascii=False)

with open("taches.json", encoding="utf-8") as f:
    relues = json.load(f)

for tache in relues:
    case = "x" if tache["faite"] else " "
    print(f"[{case}] {tache['titre']}")
```

```sortie
[ ] Faire les courses
[x] Appeler Mamie
```

> **À retenir :** avec un `s` (`dumps`, `loads`), on travaille avec une **chaîne**.
> Sans `s` (`dump`, `load`), on travaille avec un **fichier**.

## CSV : des tableaux en texte

CSV (*Comma-Separated Values*, « valeurs séparées par des virgules ») décrit un tableau,
comme une feuille de tableur. Chaque ligne est une ligne du tableau ; les colonnes sont
séparées par des virgules. La première ligne donne souvent le nom des colonnes.

```text
nom,matiere,note
Léa,maths,15
Hugo,maths,11
```

### csv.reader : des listes

`csv.reader` lit le fichier et donne chaque ligne sous forme de liste.

```python
import csv

with open("notes.csv", "w", encoding="utf-8") as f:
    f.write("nom,matiere,note\nLéa,maths,15\nHugo,maths,11\n")

with open("notes.csv", encoding="utf-8", newline="") as f:
    lecteur = csv.reader(f)
    entete = next(lecteur)
    print("Colonnes :", entete)
    for ligne in lecteur:
        print(ligne)
```

```sortie
Colonnes : ['nom', 'matiere', 'note']
['Léa', 'maths', '15']
['Hugo', 'maths', '11']
```

`next(lecteur)` lit la première ligne à part : l'en-tête. Notez `newline=""` : c'est le
réglage recommandé pour les fichiers CSV, il laisse le module gérer les fins de ligne.

> **Attention :** en CSV, **tout est texte**. La note `'15'` est une chaîne. Convertissez
> avec `int` ou `float` avant de calculer.

### Pourquoi ne pas faire split(",") ?

Une valeur peut contenir une virgule. CSV la protège alors avec des guillemets. Le
module `csv` sait gérer ce cas, pas un simple `split`.

```python
import csv

ligne = 'Dupont,"Paris, France",42'
print(ligne.split(","))
print(next(csv.reader([ligne])))
```

```sortie
['Dupont', '"Paris', ' France"', '42']
['Dupont', 'Paris, France', '42']
```

### csv.DictReader : des dictionnaires

`csv.DictReader` utilise l'en-tête pour donner chaque ligne sous forme de dictionnaire.
Le code devient plus lisible : `ligne["note"]` plutôt que `ligne[2]`.

```python
import csv

with open("notes.csv", "w", encoding="utf-8") as f:
    f.write("nom,matiere,note\nLéa,maths,15\nHugo,maths,11\nInès,maths,17\n")

with open("notes.csv", encoding="utf-8", newline="") as f:
    lignes = list(csv.DictReader(f))

print(lignes[0])
total = sum(int(l["note"]) for l in lignes)
print(f"Moyenne : {total / len(lignes):.2f}")
```

```sortie
{'nom': 'Léa', 'matiere': 'maths', 'note': '15'}
Moyenne : 14.33
```

### csv.DictWriter : écrire

`csv.DictWriter` écrit des dictionnaires. Vous lui donnez la liste des colonnes
(`fieldnames`), puis il écrit l'en-tête et les lignes.

```python
import csv

produits = [
    {"nom": "Stylo", "prix": 1.5, "stock": 120},
    {"nom": "Cahier, grand format", "prix": 3.2, "stock": 40},
]

with open("produits.csv", "w", encoding="utf-8", newline="") as f:
    ecrivain = csv.DictWriter(f, fieldnames=["nom", "prix", "stock"])
    ecrivain.writeheader()
    ecrivain.writerows(produits)

with open("produits.csv", encoding="utf-8") as f:
    print(f.read(), end="")
```

```sortie
nom,prix,stock
Stylo,1.5,120
"Cahier, grand format",3.2,40
```

Le module a mis tout seul des guillemets autour de la valeur qui contient une virgule.

> **Astuce :** pour un fichier séparé par des points-virgules (fréquent avec les
> tableurs en français), ajoutez `delimiter=";"` au lecteur ou à l'écrivain.

## Quel format choisir ?

| Besoin | Format conseillé | Pourquoi |
|--------|------------------|----------|
| Un tableau simple (lignes et colonnes) | CSV | s'ouvre dans un tableur, très compact |
| Des données imbriquées (listes dans des dictionnaires) | JSON | garde la structure |
| Échanger avec un site web ou une [API](../debuter/glossaire.md#api) | JSON | c'est la langue commune du web |
| Garder des nombres, booléens, `None` | JSON | CSV ne connaît que du texte |
| Un fichier qu'une personne va modifier à la main dans un tableur | CSV | pas besoin de connaître une syntaxe |
| Les réglages d'un programme | JSON (ou TOML, avec `tomllib`) | lisible et structuré |

## Erreurs fréquentes

### Sérialiser un type que JSON ne connaît pas

```python norun
import json
print(json.dumps({"tags": {"a", "b"}}))
```

```text
Traceback (most recent call last):
  File "main.py", line 2, in <module>
    print(json.dumps({"tags": {"a", "b"}}))
          ~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^
  ...
TypeError: Object of type set is not JSON serializable
```

JSON ne connaît pas les ensembles. Convertissez d'abord :
`json.dumps({"tags": sorted({"a", "b"})})`.

### Des guillemets simples dans le JSON

```python norun
import json
data = json.loads("{'nom': 'Alice'}")
```

```text
Traceback (most recent call last):
  File "main.py", line 2, in <module>
    data = json.loads("{'nom': 'Alice'}")
  ...
json.decoder.JSONDecodeError: Expecting property name enclosed in double quotes: line 1 column 2 (char 1)
```

JSON exige des guillemets **doubles**. Écrivez `'{"nom": "Alice"}'`. Le message donne
la position de l'erreur : colonne 2.

### Calculer avec une valeur CSV

```python norun
ligne = {"nom": "Alice", "age": "12"}
print(ligne["age"] + 1)
```

```text
Traceback (most recent call last):
  File "main.py", line 2, in <module>
    print(ligne["age"] + 1)
          ~~~~~~~~~~~~~^~~
TypeError: can only concatenate str (not "int") to str
```

Les valeurs lues dans un CSV sont des chaînes. Écrivez `int(ligne["age"]) + 1`.

## Testez-vous

**Question 1 :** quelle est la différence entre `json.dump` et `json.dumps` ?

> **Réponse**
>
> `json.dumps` renvoie une chaîne. `json.dump` écrit directement dans un fichier
> ouvert, passé en deuxième argument.

**Question 2 :** que devient `None` en JSON ? Et `False` ?

> **Réponse**
>
> `None` devient `null`, `False` devient `false`.

**Question 3 :** à partir du texte CSV ci-dessous, affichez le nom du produit le plus
cher.

```text
nom,prix
Pomme,0.5
Melon,2.8
Kiwi,0.4
```

> **Réponse**
>
> ```python
> import csv
>
> texte = "nom,prix\nPomme,0.5\nMelon,2.8\nKiwi,0.4\n"
> lignes = list(csv.DictReader(texte.splitlines()))
> plus_cher = max(lignes, key=lambda l: float(l["prix"]))
> print(plus_cher["nom"])
> ```
>
> ```sortie
> Melon
> ```
>
> `csv.DictReader` accepte n'importe quelle suite de lignes, pas seulement un fichier.

## En résumé

- Sérialiser, c'est transformer des données en texte pour les garder ou les envoyer.
- `json.dumps` / `json.loads` travaillent avec des chaînes ; `json.dump` / `json.load`
  avec des fichiers.
- `indent=2` et `ensure_ascii=False` rendent le JSON lisible.
- `csv.reader` donne des listes, `csv.DictReader` des dictionnaires ; tout est texte.
- `csv.DictWriter` écrit un CSV et gère les virgules dans les valeurs.
- CSV pour les tableaux simples, JSON pour les données imbriquées et le web.

Suite : [Une base de données avec sqlite3](sqlite.md)
