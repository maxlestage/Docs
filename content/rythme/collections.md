# Listes, enregistrements et dictionnaires

Cette page présente les façons de regrouper des valeurs : les listes, les intervalles, les enregistrements, les dictionnaires et les ensembles.

> **Objectifs :**
>
> - vous saurez créer une liste, lire un élément par sa position, et « ajouter » un élément ;
> - vous saurez utiliser les fonctions de listes courantes : `sort`, `sum`, `filter`, `map`... ;
> - vous saurez décrire une fiche avec un enregistrement, et en fabriquer une version modifiée ;
> - vous saurez retrouver une valeur par sa clé dans un dictionnaire ;
> - vous saurez choisir la bonne collection pour vos données.

## Les listes

Quand vous avez beaucoup de valeurs du même genre (les notes d'une classe, les
articles d'un panier), une variable par valeur deviendrait vite ingérable. Une liste
les range toutes sous un seul nom.

Une [liste](../debuter/glossaire.md#liste) est une suite de valeurs, dans un
ordre précis, comme une liste de courses. Elle s'écrit entre crochets, avec
des virgules entre les éléments.

Chaque élément a une **position**, son
[indice](../debuter/glossaire.md#indice). Les positions commencent à 0, et
`-1` désigne le dernier élément :

```rythme
fruits = ["pomme", "poire", "figue"]
print(fruits[0], fruits[-1], len(fruits))
```

```sortie
pomme figue 3
```

`fruits[0]` est le premier fruit, `fruits[2]` le troisième. `len` donne le
nombre d'éléments.

## Ajouter un élément, sans rien modifier

Vos données grandissent : un article de plus dans le panier, un élève de plus dans
la classe. Il faut donc savoir allonger une liste.

Une liste ne change jamais. Pour « ajouter » un élément, on fabrique une
**nouvelle** liste, et l'ancienne reste telle quelle :

```rythme
fruits = ["pomme", "poire"]
plus = push(fruits, "kiwi")
print(fruits)
print(plus)
print(fruits + ["melon"], [..fruits, "prune"], ["cerise", ..fruits])
```

```sortie
["pomme", "poire"]
["pomme", "poire", "kiwi"]
["pomme", "poire", "melon"] ["pomme", "poire", "prune"] ["cerise", "pomme", "poire"]
```

Ligne par ligne :

- `fruits = ["pomme", "poire"]` range une liste de deux fruits sous le nom `fruits` ;
- `push(fruits, "kiwi")` fabrique une nouvelle liste de trois fruits, nommée `plus` ;
- les deux `print` suivants le prouvent : `fruits` a toujours deux éléments, `plus`
  en a trois ;
- la dernière ligne fabrique trois autres listes, sans jamais toucher à `fruits`.

- `push(liste, x)` donne la liste avec `x` à la fin ;
- `+` met deux listes bout à bout ;
- `..fruits` entre crochets insère tous les éléments de `fruits`, au début,
  au milieu ou à la fin.

C'est comme photocopier la liste de courses et écrire sur la copie :
l'original ne bouge pas.

**À vous :** écrivez un programme qui range `["pain", "lait"]` sous le nom
`courses`, fabrique une nouvelle liste avec `"beurre"` à la fin, puis affiche cette
nouvelle liste et son nombre d'éléments.

> **Solution**
>
> ```rythme
> courses = ["pain", "lait"]
> nouvelles = push(courses, "beurre")
> print(nouvelles, len(nouvelles))
> ```
>
> ```sortie
> ["pain", "lait", "beurre"] 3
> ```

## Lire sans risque avec get

`fruits[10]` arrête le programme : l'élément n'existe pas. `get` et `first`
donnent plutôt une `Option` : `Some(valeur)` s'il y a une valeur, `None`
sinon.

```rythme
fruits = ["pomme", "poire"]
print(get(fruits, 1), get(fruits, 10), first(fruits), first([]))
```

```sortie
Some("poire") None Some("pomme") None
```

Pour obtenir la valeur, donnez une valeur par défaut avec `unwrap_or` :

```rythme
fruits = ["pomme", "poire"]
print(get(fruits, 10) |> unwrap_or("rien"))
```

```sortie
rien
```

Les options sont expliquées dans [gérer les erreurs](erreurs.md).

## Les intervalles

Écrire `[1, 2, 3, ..., 100]` à la main serait long. Un intervalle fabrique d'un
coup une suite de nombres, comme les numéros des pages d'un livre.

`a..b` est la liste des nombres entiers de `a` à `b`, les deux compris :

```rythme
print(1..5, len(1..100), 5..1)
print(0..20 |> step_by(5), reverse(1..3))
```

```sortie
[1, 2, 3, 4, 5] 100 []
[0, 5, 10, 15, 20] [3, 2, 1]
```

`5..1` est vide. Pour compter par pas, utilisez `step_by` ; pour compter à
rebours, `reverse`.

## Les fonctions de listes courantes

Trier, additionner, garder certains éléments : ces tâches reviennent sans cesse.
Rythme vous donne des fonctions toutes prêtes, comme des outils rangés dans une boîte.

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

| Fonction | Ce qu'elle donne |
|----------|------------------|
| `sort`, `reverse`, `unique` | la liste triée, retournée, sans doublons |
| `sum`, `max`, `min` | la somme, le plus grand, le plus petit |
| `take(xs, n)`, `drop(xs, n)` | les `n` premiers, ou tout sauf les `n` premiers |
| `contains(xs, x)` | `true` si `x` est dans la liste |
| `filter(xs, f)` | les éléments pour lesquels `f` répond `true` |
| `map(xs, f)` | le résultat de `f` pour chaque élément |
| `is_empty(xs)` | `true` si la liste est vide |

> **Astuce :** `rythme help lists` liste toutes les fonctions de listes, et
> `rythme help sort_by` explique l'une d'elles.

**À vous :** écrivez un programme qui range les notes `[12, 8, 15, 10]` sous le
nom `notes`, puis affiche la meilleure note, la somme des notes et les notes triées.

> **Solution**
>
> ```rythme
> notes = [12, 8, 15, 10]
> print(max(notes), sum(notes), sort(notes))
> ```
>
> ```sortie
> 15 45 [8, 10, 12, 15]
> ```

## Les enregistrements

Une personne a un nom, une année de naissance, une ville. Dans une liste, ces
valeurs seraient mélangées, sans dire laquelle est laquelle. Un enregistrement donne
un nom à chacune.

Un **enregistrement** regroupe des valeurs nommées, ses **champs**. C'est
une fiche : une case « nom », une case « naissance »... On l'écrit entre
accolades, et on lit un champ avec un point :

```rythme
ada = {nom: "Ada", naissance: 1815, ville: "Londres"}
print(ada.nom, ada.naissance)
print(ada)
```

```sortie
Ada 1815
{nom: "Ada", naissance: 1815, ville: "Londres"}
```

Ligne par ligne :

- la première ligne crée une fiche à trois champs et lui donne le nom `ada` ;
- `ada.nom` lit le champ `nom`, qui vaut `"Ada"` ; `ada.naissance` lit `1815` ;
- `print(ada)` affiche la fiche entière, avec tous ses champs.

Rythme n'a pas de classes : un enregistrement suffit pour décrire une
personne, un produit ou un point. `keys` donne les noms des champs, `has`
dit si un champ existe, et `get` lit un champ qui peut manquer :

```rythme
ada = {nom: "Ada", age: 36}
print(keys(ada), has(ada, "age"), get(ada, "ville"))
```

```sortie
["nom", "age"] true None
```

## Mettre à jour un enregistrement

Une information change : une adresse, un âge, une note corrigée. Il vous faut
alors une version à jour de la fiche.

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

Ligne par ligne :

- `ada` est une fiche avec deux champs ;
- `corrigee` recopie `ada`, mais son champ `naissance` vaut 1816 ;
- `avec_ville` recopie `ada` et ajoute un champ `ville` ;
- les trois `print` montrent trois fiches différentes : la première n'a pas bougé.

**À vous :** écrivez un programme qui crée un enregistrement `livre` avec les
champs `titre` (`"Le Petit Prince"`) et `pages` (96), fabrique une version avec 300
pages, puis affiche les deux.

> **Solution**
>
> ```rythme
> livre = {titre: "Le Petit Prince", pages: 96}
> epais = {..livre, pages: 300}
> print(livre)
> print(epais)
> ```
>
> ```sortie
> {titre: "Le Petit Prince", pages: 96}
> {titre: "Le Petit Prince", pages: 300}
> ```

## Décomposer une valeur

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

On peut nommer d'un coup les morceaux d'une liste ou d'un enregistrement :

```rythme
[premier, ..reste] = [10, 20, 30]
{nom, naissance} = {nom: "Ada", naissance: 1815}
print(premier, reste, nom, naissance)
```

```sortie
10 [20, 30] Ada 1815
```

`[premier, ..reste]` veut dire « le premier élément, puis tout le reste ».
`{nom, naissance}` prend les champs de ces noms. La même écriture sert de
motif dans `match` (voir [vos propres types](types.md)).

## Une liste d'enregistrements

C'est la forme la plus courante des données, comme les lignes d'un
tableau :

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

Ligne par ligne :

- `eleves` est une liste de trois fiches, une par élève ;
- `fn(e) = e.note` est une petite fonction : elle reçoit un élève `e` et donne
  sa note ;
- `max_by` garde l'élève qui a la plus grande note : la fiche de Grace ;
- `map(fn(e) = e.nom)` remplace chaque fiche par le nom de l'élève ;
- la dernière ligne trie les élèves par note, puis ne garde que les notes.

`max_by` et `sort_by` reçoivent une fonction qui dit **quoi** comparer : ici,
la note de chaque élève.

## Les dictionnaires

Un dictionnaire sert à retrouver vite une valeur à partir d'un nom : l'âge d'une
personne, le prix d'un article, comme dans un répertoire téléphonique.

Un [dictionnaire](../debuter/glossaire.md#dictionnaire) (`dict`) associe
une valeur à chaque **clé**, comme un vrai dictionnaire associe une
définition à chaque mot. Les clés peuvent être de toute sorte : textes,
nombres, symboles... On le construit à partir d'une liste de paires
`[clé, valeur]` :

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

Ligne par ligne :

- `dict(...)` construit le dictionnaire à partir de deux paires : Bob a 25 ans, Ada 36 ;
- `ages["Ada"]` cherche la clé `"Ada"` et donne 36 ; `get(ages, "Eve")` donne `None`,
  car Eve n'y est pas ;
- `put(ages, "Bob", 26)` fabrique un nouveau dictionnaire où Bob a 26 ans ;
- le dernier `print` montre que `ages` n'a pas bougé : Bob a toujours 25 ans.

`put` donne un **nouveau** dictionnaire, et `ages` n'a pas changé. Un
dictionnaire est toujours rangé dans l'ordre de ses clés.

Les clés sont souvent des **symboles**, des noms précédés de `:`. `each`
donne au bloc la clé et la valeur de chaque paire :

```rythme
stock = dict([[:pomme, 3], [:poire, 0]])
print(stock[:pomme], has(stock, :kiwi), keys(stock))
stock.each do |fruit, n|
  print(fruit, n)
end
```

```sortie
3 false [:poire, :pomme]
:poire 0
:pomme 3
```

> **Note :** quand les noms sont connus à l'avance (`nom`, `age`), prenez
> un enregistrement. Quand les clés arrivent avec les données (des prénoms,
> des mots), prenez un dictionnaire.

**À vous :** écrivez un programme qui construit un dictionnaire `prix` où
`"pomme"` coûte 2 et `"pain"` coûte 1, affiche le prix du pain, puis affiche une
version où la pomme coûte 3.

> **Solution**
>
> ```rythme
> prix = dict([["pomme", 2], ["pain", 1]])
> print(prix["pain"])
> print(put(prix, "pomme", 3))
> ```
>
> ```sortie
> 1
> dict("pain" => 1, "pomme" => 3)
> ```

## Les ensembles

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

Un **ensemble** (`set`) garde chaque valeur une seule fois, comme une
collection de timbres sans doublons :

```rythme
vus = set([3, 1, 3, 2, 1])
print(vus, len(vus), contains(vus, 2))
```

```sortie
set(1, 2, 3) 3 true
```

## Quelle collection choisir

Chaque collection a son usage, comme chaque tiroir d'une cuisine. Ce tableau vous
aide à choisir.

| Besoin | Choix | Exemple |
|--------|-------|---------|
| des valeurs dans un ordre | liste | `[1, 2, 3]` |
| des nombres qui se suivent | intervalle | `1..10` |
| une « fiche » avec des champs fixes | enregistrement | `{nom: "Ada", age: 36}` |
| retrouver une valeur par sa clé | dictionnaire | `dict([["Ada", 36]])` |
| des valeurs sans doublons | ensemble | `set([1, 2])` |

## Erreurs fréquentes

### Lire après le dernier élément

```rythme norun
fruits = ["pomme", "poire", "figue"]
print(fruits[3])
```

```text
erreur[E007]: l'indice 3 est hors limites : la liste a 3 élément(s)
 --> main.ry:2:14
  |
2 | print(fruits[3])
  |              ^ aucun élément ici
  |
  = aide: les indices valides vont de 0 à 2 (ou -1 pour le dernier élément)
  = aide: pour obtenir une Option au lieu d'une erreur, utilisez `get(x, 3)`
  = en savoir plus : `rythme explain E007`
```

Les positions commencent à 0 : avec 3 éléments, la dernière est 2. Écrivez
`fruits[2]`, `fruits[-1]`, ou `get(fruits, 3)` si l'élément peut manquer.

### Croire que push modifie la liste

```rythme norun
fruits = ["pomme", "poire"]
push(fruits, "kiwi")
print(fruits)
```

```text
avertissement[W008]: le résultat de `push` est perdu
 --> main.ry:2:1
  |
2 | push(fruits, "kiwi")
  | ^^^^^^^^^^^^^^^^^^^^ cette valeur est perdue
  |
  = aide: les valeurs ne changent jamais : `push(list, x)` donne une nouvelle valeur, et ne modifie rien.
      Pour la garder, donnez-lui un nom : `nouveau = push(list, x)`
```

Le programme affiche encore `["pomme", "poire"]`. Gardez la nouvelle liste :
`fruits2 = push(fruits, "kiwi")`.

### Modifier un champ avec =

```rythme norun
ada = {nom: "Ada", age: 36}
ada.age = 37
```

```text
erreur[E001]: impossible de donner une valeur à ceci
 --> main.ry:2:1
  |
2 | ada.age = 37
  | ^^^^^^^ ceci n'est pas un nom
  |
2 | ada.age = 37
  |         - `=` donne un nom à une valeur
  |
  = aide: les enregistrements ne peuvent pas être modifiés ; construisez-en un nouveau : `nouveau = {..ancien, age: valeur}`
  = en savoir plus : `rythme explain E001`
```

Fabriquez un nouvel enregistrement : `ada2 = {..ada, age: 37}`.

### Lire un champ qui n'existe pas

```rythme norun
ada = {nom: "Ada", age: 36}
print(ada.ville)
```

```text
erreur[E010]: cet enregistrement n'a pas de champ `ville`
 --> main.ry:2:11
  |
2 | print(ada.ville)
  |           ^^^^^ champ inconnu
  |
  = note: ses champs sont : `nom`, `age`
  = aide: pour obtenir une Option au lieu d'une erreur, utilisez `get(enregistrement, "ville")`
  = en savoir plus : `rythme explain E010`
```

Vérifiez l'orthographe du champ, ou utilisez `get(ada, "ville")` si le
champ peut manquer.

## Testez-vous

**Question 1 :** avec `xs = [4, 8, 15, 16]`, que valent `xs[1]`, `xs[-1]`
et `len(xs)` ?

> **Réponse**
>
> `8`, `16` et `4`. La position 1 est le deuxième élément, `-1` le
> dernier.

**Question 2 :** qu'affiche ce programme ?

```rythme norun
p = {x: 1, y: 2}
q = {..p, y: 5}
print(p.y, q.y)
```

> **Réponse**
>
> `2 5`. `q` est un nouvel enregistrement ; `p` n'a pas changé.

**Question 3 :** vous voulez compter combien de fois chaque prénom apparaît.
Quelle collection choisir ?

> **Réponse**
>
> Un dictionnaire : chaque prénom est une clé, et sa valeur est le nombre
> de fois où il apparaît.

## En résumé

- Une liste s'écrit `[1, 2, 3]` ; `xs[0]` est le premier élément, `xs[-1]`
  le dernier.
- `get` et `first` donnent une `Option`, sans risque d'arrêter le programme.
- Un enregistrement `{nom: "Ada", age: 36}` regroupe des champs, lus avec
  un point : `ada.nom`.
- Un dictionnaire retrouve une valeur par sa clé ; un ensemble garde chaque
  valeur une seule fois.
- Aucune collection ne change : `push`, `put` et `{..r, champ: valeur}`
  fabriquent une nouvelle valeur.

Suite : [Dictionnaires et ensembles](dictionnaires.md)
