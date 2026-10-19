# Dictionnaires et ensembles

Cette page approfondit deux collections : le dictionnaire, qui retrouve une valeur par sa clé, et l'ensemble, qui garde chaque valeur une seule fois.

> **Objectifs :**
>
> - vous saurez créer un dictionnaire et y lire, ajouter ou retirer une clé ;
> - vous saurez parcourir un dictionnaire, ses clés et ses valeurs ;
> - vous saurez compter des mots avec un dictionnaire ;
> - vous saurez créer un ensemble et combiner deux ensembles ;
> - vous saurez choisir entre un enregistrement, un dictionnaire et un ensemble.

## Créer un dictionnaire

Un [dictionnaire](../debuter/glossaire.md#dictionnaire) associe une
**valeur** à chaque **clé**. C'est comme un répertoire téléphonique : on
cherche un nom (la clé), on trouve un numéro (la valeur).

Rythme n'a pas d'écriture spéciale pour les dictionnaires : on appelle la
fonction `dict` avec une liste de paires `[clé, valeur]` :

```rythme
capitales = dict([["France", "Paris"], ["Italie", "Rome"], ["Japon", "Tokyo"]])
print(capitales)
print(len(capitales))
```

```sortie
dict("France" => "Paris", "Italie" => "Rome", "Japon" => "Tokyo")
3
```

L'affichage met une flèche `=>` entre chaque clé et sa valeur. Cette forme
sert seulement à l'affichage : dans le code, écrivez toujours
`dict([[clé, valeur], ...])`.

`dict()` est le dictionnaire vide. Quand une clé revient deux fois, la
dernière valeur l'emporte :

```rythme
print(dict(), dict([[1, "un"], [2, "deux"], [1, "UN"]]))
```

```sortie
dict() dict(1 => "UN", 2 => "deux")
```

## Des clés de toute sorte

Les clés ne sont pas forcément des textes. Ce peut être des nombres, des
[symboles](../debuter/glossaire.md#symbole), et même des listes. Ici, une
case d'une grille est repérée par ses coordonnées `[x, y]` :

```rythme
grille = dict([[[0, 0], "départ"], [[2, 1], "arrivée"]])
print(grille[[2, 1]])
stock = dict([[:pomme, 3], [:poire, 0]])
print(stock[:pomme])
```

```sortie
arrivée
3
```

Seules les fonctions ne peuvent pas être des clés.

> **Note :** un dictionnaire est toujours rangé dans l'ordre de ses clés
> (l'ordre alphabétique pour des textes), quel que soit l'ordre dans lequel
> vous l'avez construit.

## Lire une valeur

Il y a deux façons de lire, comme pour les listes :

- `dico[clé]` donne la valeur, et arrête le programme si la clé manque ;
- `get(dico, clé)` donne une `Option` : `Some(valeur)` ou `None`.

`has(dico, clé)` dit si la clé existe :

```rythme
capitales = dict([["France", "Paris"], ["Japon", "Tokyo"]])
print(capitales["Japon"])
print(get(capitales, "Japon"), get(capitales, "Chili"))
print(has(capitales, "France"), has(capitales, "Chili"))
print(unwrap_or(get(capitales, "Chili"), "inconnue"))
```

```sortie
Tokyo
Some("Tokyo") None
true false
inconnue
```

Utilisez `[...]` quand vous êtes sûr que la clé existe, et `get` sinon.
Les options sont expliquées dans [gérer les erreurs](erreurs.md).

## Ajouter et retirer une clé

Un dictionnaire ne change jamais : il est
[immuable](../debuter/glossaire.md#immuable). `put` et `remove` fabriquent
un **nouveau** dictionnaire :

```rythme
capitales = dict([["France", "Paris"], ["Italie", "Rome"]])
plus = put(capitales, "Chili", "Santiago")
moins = remove(capitales, "Italie")
corrige = put(capitales, "France", "PARIS")
print(plus)
print(moins)
print(corrige)
print(capitales)
```

```sortie
dict("Chili" => "Santiago", "France" => "Paris", "Italie" => "Rome")
dict("France" => "Paris")
dict("France" => "PARIS", "Italie" => "Rome")
dict("France" => "Paris", "Italie" => "Rome")
```

`put` ajoute la clé, ou remplace sa valeur si elle existe déjà. Le
dictionnaire `capitales` d'origine n'a pas bougé.

## Clés, valeurs et paires

```rythme
capitales = dict([["France", "Paris"], ["Italie", "Rome"]])
print(keys(capitales))
print(values(capitales))
print(entries(capitales))
```

```sortie
["France", "Italie"]
["Paris", "Rome"]
[["France", "Paris"], ["Italie", "Rome"]]
```

| Fonction | Ce qu'elle donne |
|----------|------------------|
| `keys(d)` | la liste des clés |
| `values(d)` | la liste des valeurs |
| `entries(d)` | la liste des paires `[clé, valeur]` |

Ces trois fonctions donnent des **listes** : vous pouvez ensuite utiliser
`map`, `sort`, `filter` et toutes les fonctions de listes.

## Parcourir un dictionnaire

`each` appelle une fonction pour chaque paire, dans l'ordre des clés. La
fonction reçoit **deux** valeurs : la clé et la valeur.

```rythme
capitales = dict([["France", "Paris"], ["Japon", "Tokyo"]])
each(capitales, fn(pays, ville) = print("{pays} : {ville}"))
capitales.each do |pays, ville|
  print("La capitale de {pays} est {ville}.")
end
```

```sortie
France : Paris
Japon : Tokyo
La capitale de France est Paris.
La capitale de Japon est Tokyo.
```

Le style avec `do |pays, ville|` est celui de Ruby ; les deux écritures font
la même chose.

`filter` garde les paires choisies, et `map_values` transforme chaque
valeur :

```rythme
prix = dict([["pomme", 3], ["poire", 5], ["figue", 9]])
print(filter(prix, fn(fruit, p) = p < 6))
print(map_values(prix, fn(p) = p * 2))
```

```sortie
dict("poire" => 5, "pomme" => 3)
dict("figue" => 18, "poire" => 10, "pomme" => 6)
```

## Compter des mots

C'est l'usage le plus classique d'un dictionnaire : chaque mot est une clé,
et sa valeur est le nombre de fois où on l'a vu.

L'idée : on part d'un dictionnaire vide, et pour chaque mot, on ajoute 1 à
son compteur (0 s'il n'existe pas encore). `fold` fait passer le
dictionnaire d'un mot au suivant (voir [penser fonctionnel](fonctionnel.md)) :

```rythme
fn compter(compte, mot)
  deja = unwrap_or(get(compte, mot), 0)
  put(compte, mot, deja + 1)
end

texte = "le chat et le chien et le chat"
compte = fold(split(texte), dict(), compter)
print(compte)
print(compte["le"])
```

```sortie
dict("chat" => 2, "chien" => 1, "et" => 2, "le" => 3)
3
```

Pour trouver les mots les plus fréquents, on passe par la liste des paires :

```rythme
compte = dict([["chat", 2], ["chien", 1], ["et", 2], ["le", 3]])
podium = entries(compte) |> sort_by(fn(paire) = -paire[1]) |> take(2)
print(podium)
```

```sortie
[["le", 3], ["chat", 2]]
```

Trier selon `-paire[1]` range du plus grand nombre au plus petit.

## Créer un ensemble

Un **ensemble** (`set`) contient des valeurs **sans doublons**, comme un
album de timbres où chaque timbre n'apparaît qu'une fois. On le construit
avec `set` et une liste :

```rythme
couleurs = set(["rouge", "vert", "rouge", "bleu"])
print(couleurs, len(couleurs))
print(set(chars("banane")))
print(set(), is_empty(set()))
```

```sortie
set("bleu", "rouge", "vert") 3
set("a", "b", "e", "n")
set() true
```

Comme un dictionnaire, un ensemble est rangé dans l'ordre de ses valeurs.

## Ajouter, retirer, chercher

Les noms sont les mêmes que pour les listes et les dictionnaires :

```rythme
amis = set(["Ada", "Alan", "Grace"])
print(push(amis, "Linus"))
print(remove(amis, "Ada"))
print(contains(amis, "Ada"), has(amis, "Linus"))
print(values(amis))
```

```sortie
set("Ada", "Alan", "Grace", "Linus")
set("Alan", "Grace")
true false
["Ada", "Alan", "Grace"]
```

- `push(e, x)` donne l'ensemble avec `x` en plus ;
- `remove(e, x)` donne l'ensemble sans `x` ;
- `contains(e, x)` et `has(e, x)` disent si `x` y est ;
- `values(e)` donne ses éléments sous forme de liste.

Chercher dans un ensemble est très rapide, même avec des millions
d'éléments.

## Combiner deux ensembles

Pensez à deux cercles qui se chevauchent : les amis d'Ada et les amis de
Linus.

```rythme
a = set(["Alan", "Grace", "Tim"])
b = set(["Alan", "Linus", "Tim"])
print(union(a, b))
print(intersection(a, b))
print(difference(a, b))
print(is_subset(set(["Alan"]), a))
```

```sortie
set("Alan", "Grace", "Linus", "Tim")
set("Alan", "Tim")
set("Grace")
true
```

| Fonction | Ce qu'elle donne | Avec les cercles |
|----------|------------------|------------------|
| `union(a, b)` | ce qui est dans `a` **ou** dans `b` | les deux cercles entiers |
| `intersection(a, b)` | ce qui est dans `a` **et** dans `b` | la partie commune |
| `difference(a, b)` | ce qui est dans `a` mais **pas** dans `b` | `a` sans la partie commune |
| `is_subset(a, b)` | `true` si tout `a` est dans `b` | `a` est à l'intérieur de `b` |

`union` marche aussi avec deux dictionnaires : pour une clé présente des
deux côtés, la valeur du second l'emporte.

## Enregistrement, dictionnaire ou ensemble ?

Les trois se ressemblent. Une question simple aide à choisir : **connaissez-vous
les noms à l'avance ?**

| Collection | Quand la choisir | Exemple |
|------------|------------------|---------|
| [enregistrement](../debuter/glossaire.md#enregistrement) | une fiche dont les champs sont connus en écrivant le programme | `{nom: "Ada", age: 36}` |
| dictionnaire | des clés qui arrivent avec les données : des mots, des prénoms, des coordonnées | `dict([["Ada", 36], ["Alan", 41]])` |
| ensemble | seulement savoir **si** une valeur est présente, sans doublons | `set(["Ada", "Alan"])` |

Un élève a toujours un `nom` et une `note` : c'est un enregistrement. Les
mots d'un livre ne sont pas connus d'avance : pour les compter, c'est un
dictionnaire. Pour savoir quels mots ont déjà été vus, un ensemble suffit.

## Erreurs fréquentes

### Lire une clé qui n'existe pas

```rythme norun
stock = dict([["pomme", 3], ["poire", 0]])
print(stock["kiwi"])
```

```text
erreur[E010]: ce dictionnaire n'a pas de clé "kiwi"
 --> main.ry:2:13
  |
2 | print(stock["kiwi"])
  |             ^^^^^^
  |
  = aide: utilisez `get(dico, clé)` pour obtenir une Option à la place, ou `has(dico, clé)` pour vérifier d'abord
  = en savoir plus : `rythme explain E010`
```

Si la clé peut manquer, écrivez `get(stock, "kiwi")`, ou
`unwrap_or(get(stock, "kiwi"), 0)` pour avoir 0 par défaut.

### Oublier les crochets autour des paires

```rythme norun
ages = dict(["Ada", 36])
```

```text
erreur[E023]: `dict` a besoin d'une liste de paires [clé, valeur], mais elle contient Text "Ada"
 --> main.ry:1:8
  |
1 | ages = dict(["Ada", 36])
  |        ^^^^^^^^^^^^^^^^^
  |
  = aide: chaque paire est une liste de deux éléments, comme `dict([["Ada", 36], ["Alan", 41]])`
  = en savoir plus : `rythme explain E023`
```

Il faut une liste **de paires** : `dict([["Ada", 36]])`, avec deux paires de
crochets.

### Chercher une fonction add

```rythme norun
vus = set([1, 2])
print(add(vus, 3))
```

```text
erreur[E003]: nom inconnu `add`
 --> main.ry:2:7
  |
2 | print(add(vus, 3))
  |       ^^^ non défini
  |
  = aide: en Rythme, cela s'appelle `push`
  = en savoir plus : `rythme explain E003`
```

Pour un ensemble comme pour une liste, on « ajoute » avec `push`.

### Croire que put modifie le dictionnaire

```rythme norun
stock = dict([["pomme", 3]])
put(stock, "kiwi", 5)
print(stock)
```

```text
avertissement[W008]: le résultat de `put` est perdu
 --> main.ry:2:1
  |
2 | put(stock, "kiwi", 5)
  | ^^^^^^^^^^^^^^^^^^^^^ cette valeur est perdue
  |
  = aide: les valeurs ne changent jamais : `put(record, name, value)` donne une nouvelle valeur, et ne modifie rien.
      Pour la garder, donnez-lui un nom : `nouveau = put(record, name, value)`

dict("pomme" => 3)
```

Le kiwi n'apparaît pas. Gardez le résultat : `stock2 = put(stock, "kiwi", 5)`.

## Testez-vous

**Question 1 :** avec `d = dict([["a", 1], ["b", 2]])`, que valent
`get(d, "b")`, `get(d, "z")` et `keys(d)` ?

> **Réponse**
>
> `Some(2)`, `None` et `["a", "b"]`.

**Question 2 :** qu'affiche `print(len(set([1, 2, 2, 3, 3, 3])))` ?

> **Réponse**
>
> `3` : un ensemble garde chaque valeur une seule fois, il contient donc
> 1, 2 et 3.

**Question 3 :** quels élèves suivent à la fois le cours de piano et celui
de danse ?

```rythme norun
piano = set(["Ada", "Léa", "Tom"])
danse = set(["Léa", "Sam", "Tom"])
```

> **Réponse**
>
> `intersection(piano, danse)`, qui vaut `set("Léa", "Tom")`.

## En résumé

- `dict([[clé, valeur], ...])` crée un dictionnaire ; les clés peuvent être
  des textes, des nombres, des symboles, des listes...
- `d[clé]` lit une valeur sûre, `get(d, clé)` une valeur qui peut manquer,
  `has(d, clé)` vérifie.
- `put` et `remove` donnent un nouveau dictionnaire ; `keys`, `values` et
  `entries` donnent des listes.
- `set(liste)` crée un ensemble sans doublons ; `push`, `remove`,
  `contains`, `union`, `intersection` et `difference` s'en servent.
- Champs connus d'avance : enregistrement ; clés venues des données :
  dictionnaire ; simple présence : ensemble.

Suite : [Suivre un programme pas à pas](pas-a-pas.md)
