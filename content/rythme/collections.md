# Listes, enregistrements et dictionnaires

Cette page présente les façons de regrouper des valeurs : les listes, les intervalles, les enregistrements, les dictionnaires et les ensembles.

## Les listes

Une liste s'écrit entre crochets. Les positions commencent à 0, et `-1`
désigne le dernier élément :

```rythme
fruits = ["pomme", "poire", "figue"]
print(fruits[0], fruits[-1], len(fruits))
```

```sortie
pomme figue 3
```

Une liste ne change jamais. Pour « ajouter » un élément, on fabrique une
nouvelle liste :

```rythme
fruits = ["pomme", "poire"]
plus = push(fruits, "kiwi")
print(fruits, plus, fruits + ["melon"], [..fruits, "prune"])
```

```sortie
["pomme", "poire"] ["pomme", "poire", "kiwi"] ["pomme", "poire", "melon"] ["pomme", "poire", "prune"]
```

`..fruits` entre crochets insère tous les éléments de `fruits`.

## Lire sans risque avec get

`fruits[10]` arrête le programme : l'élément n'existe pas. `get` et `first`
donnent plutôt une `Option` : `Some(valeur)` ou `None`.

```rythme
fruits = ["pomme", "poire"]
print(get(fruits, 1), get(fruits, 10), first(fruits), first([]))
```

```sortie
Some("poire") None Some("pomme") None
```

Les options sont expliquées dans [gérer les erreurs](erreurs.md).

## Les intervalles

`a..b` est la liste des nombres entiers de `a` à `b`, les deux compris :

```rythme
print(1..5, len(1..100), 5..1)
```

```sortie
[1, 2, 3, 4, 5] 100 []
```

## Les fonctions de listes courantes

```rythme
nombres = [3, 1, 4, 1, 5, 9, 2, 6]
print(sort(nombres), reverse(nombres), unique(nombres))
print(sum(nombres), max(nombres), min(nombres))
print(take(nombres, 3), drop(nombres, 5), contains(nombres, 9))
print(filter(nombres, is_even), map(nombres, fn(x) = x * 10))
```

```sortie
[1, 1, 2, 3, 4, 5, 6, 9] [6, 2, 9, 5, 1, 4, 1, 3] [3, 1, 4, 5, 9, 2, 6]
31 9 1
[3, 1, 4] [9, 2, 6] true
[4, 2, 6] [30, 10, 40, 10, 50, 90, 20, 60]
```

> **Astuce :** `rythme help lists` liste toutes les fonctions de listes, et
> `rythme help sort_by` explique l'une d'elles.

## Les enregistrements

Un enregistrement regroupe des valeurs nommées, ses **champs**. On lit un
champ avec un point :

```rythme
ada = {nom: "Ada", naissance: 1815, ville: "Londres"}
print(ada.nom, ada.naissance)
print(ada)
```

```sortie
Ada 1815
{nom: "Ada", naissance: 1815, ville: "Londres"}
```

Rythme n'a pas de classes : un enregistrement suffit pour décrire une
personne, un produit ou un point.

## Mettre à jour un enregistrement

On ne modifie pas un enregistrement : on en fabrique un nouveau.
`{..ada, naissance: 1816}` recopie tous les champs de `ada`, puis remplace
`naissance` :

```rythme
ada = {nom: "Ada", naissance: 1815}
corrigee = {..ada, naissance: 1816}
avec_ville = {..ada, ville: "Londres"}
print(ada)
print(corrigee)
print(avec_ville)
```

```sortie
{nom: "Ada", naissance: 1815}
{nom: "Ada", naissance: 1816}
{nom: "Ada", naissance: 1815, ville: "Londres"}
```

`ada` n'a pas changé.

## Décomposer une valeur

On peut nommer d'un coup les morceaux d'une liste ou d'un enregistrement :

```rythme
[premier, ..reste] = [10, 20, 30]
{nom, naissance} = {nom: "Ada", naissance: 1815}
print(premier, reste, nom, naissance)
```

```sortie
10 [20, 30] Ada 1815
```

## Une liste d'enregistrements

C'est la forme la plus courante des données :

```rythme
eleves = [
  {nom: "Ada", note: 17},
  {nom: "Alan", note: 12},
  {nom: "Grace", note: 19},
]
meilleur = max_by(eleves, fn(e) = e.note)
print(meilleur.nom)
print(eleves |> map(fn(e) = e.nom))
print(sort_by(eleves, fn(e) = e.note) |> map(fn(e) = e.note))
```

```sortie
Grace
["Ada", "Alan", "Grace"]
[12, 17, 19]
```

## Les dictionnaires

Un **dictionnaire** (`dict`) associe une valeur à chaque clé. Les clés
peuvent être de toute sorte : textes, nombres, symboles... On le construit
à partir d'une liste de paires `[clé, valeur]` :

```rythme
ages = dict([["Bob", 25], ["Ada", 36]])
print(ages["Ada"], get(ages, "Eve"), len(ages))
plus_vieux = put(ages, "Bob", 26)
print(plus_vieux)
print(ages)
```

```sortie
36 None 2
dict("Ada" => 36, "Bob" => 26)
dict("Ada" => 36, "Bob" => 25)
```

Un dictionnaire est toujours rangé dans l'ordre de ses clés. Et, comme
toujours, `put` donne un nouveau dictionnaire.

## Les ensembles

Un **ensemble** (`set`) garde chaque valeur une seule fois :

```rythme
vus = set([3, 1, 3, 2, 1])
print(vus, len(vus), contains(vus, 2))
```

```sortie
set(1, 2, 3) 3 true
```

## Quelle collection choisir

| Besoin | Choix | Exemple |
|--------|-------|---------|
| des valeurs dans un ordre | liste | `[1, 2, 3]` |
| des nombres qui se suivent | intervalle | `1..10` |
| une « fiche » avec des champs fixes | enregistrement | `{nom: "Ada", age: 36}` |
| retrouver une valeur par sa clé | dictionnaire | `dict([["Ada", 36]])` |
| des valeurs sans doublons | ensemble | `set([1, 2])` |

## En résumé

- Une liste s'écrit `[1, 2, 3]` ; `xs[0]` est le premier élément.
- `get` et `first` donnent une `Option`, sans risque d'arrêter le programme.
- Un enregistrement `{nom: "Ada", age: 36}` regroupe des champs, lus avec
  un point : `ada.nom`.
- Un dictionnaire retrouve une valeur par sa clé ; un ensemble garde chaque
  valeur une seule fois.
- Aucune collection ne change : `push`, `put` et `{..r, champ: valeur}`
  fabriquent une nouvelle valeur.

Suite : [penser fonctionnel](fonctionnel.md). Comparez avec les collections
de [Ruby](../ruby/collections.md) et de [Rust](../rust/collections.md).
