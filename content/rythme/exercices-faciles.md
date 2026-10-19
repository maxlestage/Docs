# Exercices pour bien démarrer

Quinze petits exercices très guidés, tous faciles, pour vous entraîner sur les bases de Rythme : noms, calculs, textes, conditions, fonctions, listes, enregistrements et dictionnaires.

## Comment faire ces exercices

Tous les exercices ont le niveau ★☆☆ : quelques lignes, une seule idée.
Pour chacun, suivez ces étapes :

```text
1. Lisez l'énoncé et le résultat attendu.
2. Cherchez sur papier : quelles valeurs ? quel calcul ?
3. Écrivez le code dans un fichier, par exemple exo.ry.
4. Lancez-le :  rythme run exo.ry
5. Comparez avec le résultat attendu.
6. Bloqué ? Lisez le premier indice, puis le second.
7. Seulement ensuite, ouvrez la solution.
```

Une erreur n'est pas un échec : lisez le message, surtout la ligne `aide:`.
La page [les pièges du débutant](pieges.md) explique les plus courants.

> **Astuce :** il y a souvent plusieurs bonnes réponses. Si votre programme
> affiche le bon résultat, il est juste, même s'il ne ressemble pas à la
> solution.

## Noms, calculs et textes

### Exercice 1 : Se présenter

Niveau : ★☆☆

Donnez un nom à votre prénom et un autre à votre ville. Affichez ensuite
une phrase qui utilise les deux. Résultat attendu (avec vos valeurs) :

```text
Bonjour, je suis Léa et j'habite à Lyon.
```

> **Indice :** commencez par `prenom = "Léa"`, puis `ville = "Lyon"`.

> **Indice :** dans un texte, `{prenom}` est remplacé par la valeur du nom.

> **Solution**
>
> ```rythme
> prenom = "Léa"
> ville = "Lyon"
> print("Bonjour, je suis {prenom} et j'habite à {ville}.")
> ```
>
> ```sortie
> Bonjour, je suis Léa et j'habite à Lyon.
> ```
>
> Pourquoi ça marche : les accolades insèrent la valeur d'un nom dans le
> texte. L'apostrophe de « j'habite » est un caractère comme un autre, à
> l'intérieur des guillemets.

### Exercice 2 : Le ticket de caisse

Niveau : ★☆☆

Un croissant coûte 3 euros. Vous en achetez 4. Calculez le total et
affichez :

```text
4 croissants à 3 euros : 12 euros
```

> **Indice :** trois noms : `prix`, `quantite` et `total`.

> **Indice :** `total = prix * quantite`. L'étoile `*` veut dire « fois ».

> **Solution**
>
> ```rythme
> prix = 3
> quantite = 4
> total = prix * quantite
> print("{quantite} croissants à {prix} euros : {total} euros")
> ```
>
> ```sortie
> 4 croissants à 3 euros : 12 euros
> ```
>
> Pourquoi ça marche : `total` reçoit le résultat du calcul, puis le texte
> l'insère. Changez `quantite` : le total suit tout seul.

### Exercice 3 : Partager des bonbons

Niveau : ★☆☆

Vous avez 17 bonbons pour 5 enfants. Chacun reçoit le même nombre de
bonbons. Affichez combien chaque enfant reçoit, et combien il en reste :

```text
Chaque enfant reçoit 3 bonbons.
Il en reste 2.
```

> **Indice :** la division entière `//` donne la part de chacun.

> **Indice :** le reste `%` donne ce qui reste : 17 = 3 × 5 + **2**.

> **Solution**
>
> ```rythme
> bonbons = 17
> enfants = 5
> print("Chaque enfant reçoit {bonbons // enfants} bonbons.")
> print("Il en reste {bonbons % enfants}.")
> ```
>
> ```sortie
> Chaque enfant reçoit 3 bonbons.
> Il en reste 2.
> ```
>
> Pourquoi ça marche : `17 // 5` vaut 3 (5 entre 3 fois dans 17) et
> `17 % 5` vaut 2. Avec `/`, vous obtiendriez `3.4`, ce qui n'a pas de sens
> pour des bonbons.

### Exercice 4 : Crier

Niveau : ★☆☆

Écrivez une fonction `crier(phrase)` qui met la phrase en majuscules et
ajoute ` !` à la fin. Résultat attendu pour `crier("on a gagné")` :

```text
ON A GAGNÉ !
```

> **Indice :** `upper` met un texte en majuscules.

> **Indice :** `+` colle deux textes : `upper(phrase) + " !"`.

> **Solution**
>
> ```rythme
> fn crier(phrase) = upper(phrase) + " !"
> print(crier("on a gagné"))
> ```
>
> ```sortie
> ON A GAGNÉ !
> ```
>
> Pourquoi ça marche : `upper` donne un nouveau texte en majuscules, accents
> compris, et `+` colle les deux textes bout à bout.

### Exercice 5 : Le mot à l'envers

Niveau : ★☆☆

Affichez le mot `bonjour` écrit à l'envers :

```text
ruojnob
```

> **Indice :** cherchez une fonction qui « retourne » une valeur, comme une
> crêpe.

> **Indice :** `reverse` marche sur les listes, et aussi sur les textes.

> **Solution**
>
> ```rythme
> print(reverse("bonjour"))
> ```
>
> ```sortie
> ruojnob
> ```
>
> Pourquoi ça marche : `reverse` lit les caractères du dernier au premier.
> Le mot `"bonjour"` lui-même ne change pas : `reverse` en fabrique un
> nouveau.

## Conditions et fonctions

### Exercice 6 : Majeur ou mineur

Niveau : ★☆☆

Écrivez une fonction `statut(age)` qui donne `"majeur"` à partir de 18 ans,
et `"mineur"` sinon. Résultat attendu pour 20, 12 et 18 :

```text
majeur mineur majeur
```

> **Indice :** `if condition then valeur else autre_valeur`.

> **Indice :** la condition est `age >= 18` : « au moins 18 ».

> **Solution**
>
> ```rythme
> fn statut(age) = if age >= 18 then "majeur" else "mineur"
> print(statut(20), statut(12), statut(18))
> ```
>
> ```sortie
> majeur mineur majeur
> ```
>
> Pourquoi ça marche : `>=` inclut 18. Avec `>`, `statut(18)` donnerait
> `"mineur"`. Pensez toujours à tester la valeur limite.

### Exercice 7 : Le plus grand

Niveau : ★☆☆

Écrivez une fonction `plus_grand(a, b)` qui donne le plus grand des deux
nombres. Résultat attendu pour `(3, 8)`, `(10, 2)` et `(5, 5)` :

```text
8 10 5
```

> **Indice :** comparez `a` et `b` avec `>`.

> **Indice :** si `a > b`, la réponse est `a` ; sinon, c'est `b`.

> **Solution**
>
> ```rythme
> fn plus_grand(a, b) = if a > b then a else b
> print(plus_grand(3, 8), plus_grand(10, 2), plus_grand(5, 5))
> ```
>
> ```sortie
> 8 10 5
> ```
>
> Pourquoi ça marche : quand les deux sont égaux, la condition est fausse et
> on donne `b`, qui vaut aussi 5. Rythme a déjà une fonction pour cela :
> `max([a, b])`.

### Exercice 8 : Le feu tricolore

Niveau : ★☆☆

Écrivez une fonction `consigne(couleur)` : `"vert"` donne `"passez"`,
`"orange"` donne `"ralentissez"`, `"rouge"` donne `"arrêtez-vous"`, et toute
autre couleur donne `"couleur inconnue"`.

```text
arrêtez-vous
passez
couleur inconnue
```

> **Indice :** plusieurs cas : utilisez `match couleur`, un cas par ligne,
> puis `end`.

> **Indice :** chaque ligne s'écrit `"vert" -> "passez"`. Le dernier cas,
> `_`, attrape tout le reste.

> **Solution**
>
> ```rythme
> fn consigne(couleur) = match couleur
>   "vert" -> "passez"
>   "orange" -> "ralentissez"
>   "rouge" -> "arrêtez-vous"
>   _ -> "couleur inconnue"
> end
> print(consigne("rouge"))
> print(consigne("vert"))
> print(consigne("bleu"))
> ```
>
> ```sortie
> arrêtez-vous
> passez
> couleur inconnue
> ```
>
> Pourquoi ça marche : `match` essaie les cas de haut en bas. `"bleu"` ne
> correspond à aucun texte, donc il tombe sur `_`, toujours en dernier.

### Exercice 9 : La moyenne

Niveau : ★☆☆

Calculez la moyenne des notes `[12, 15, 10]`. Affichez-la telle quelle,
puis arrondie à un chiffre après la virgule :

```text
12.333333333333334
12.3
```

> **Indice :** la moyenne, c'est la somme divisée par le nombre de notes :
> `sum` et `len`.

> **Indice :** `round(x, 1)` garde un chiffre après la virgule.

> **Solution**
>
> ```rythme
> notes = [12, 15, 10]
> moyenne = sum(notes) / len(notes)
> print(moyenne)
> print(round(moyenne, 1))
> ```
>
> ```sortie
> 12.333333333333334
> 12.3
> ```
>
> Pourquoi ça marche : `sum(notes)` vaut 37 et `len(notes)` vaut 3. `/`
> donne un nombre à virgule, que `round` raccourcit.

### Exercice 10 : Le compte à rebours

Niveau : ★☆☆

Affichez un compte à rebours de 3 à 1, puis `Partez !` :

```text
3
2
1
Partez !
```

> **Indice :** pas de boucle en Rythme. `reverse(1..3)` donne `[3, 2, 1]`.

> **Indice :** `each` appelle une fonction pour chaque élément :
> `each(fn(n) = print(n))`.

> **Solution**
>
> ```rythme
> reverse(1..3) |> each(fn(n) = print(n))
> print("Partez !")
> ```
>
> ```sortie
> 3
> 2
> 1
> Partez !
> ```
>
> Pourquoi ça marche : `1..3` est la liste `[1, 2, 3]`, `reverse` la
> retourne, et `each` affiche chaque nombre. Attention : `3..1` serait une
> liste vide.
>
> | Étape | n | Affiché |
> |-------|---|---------|
> | 1 | 3 | `3` |
> | 2 | 2 | `2` |
> | 3 | 1 | `1` |
> | fin | | `Partez !` |

## Listes, enregistrements et dictionnaires

### Exercice 11 : Les carrés

Niveau : ★☆☆

Fabriquez la liste des carrés des nombres de 1 à 6 (un carré, c'est un
nombre multiplié par lui-même) :

```text
[1, 4, 9, 16, 25, 36]
```

> **Indice :** `1..6` donne les nombres de 1 à 6.

> **Indice :** `map` fabrique une nouvelle liste : `map(fn(x) = x * x)`.

> **Solution**
>
> ```rythme
> print(1..6 |> map(fn(x) = x * x))
> ```
>
> ```sortie
> [1, 4, 9, 16, 25, 36]
> ```
>
> Pourquoi ça marche : `map` applique `x * x` à chaque nombre et range les
> résultats dans une nouvelle liste, dans le même ordre.

### Exercice 12 : La liste de courses

Niveau : ★☆☆

Partez de `["pain", "lait"]`. Ajoutez `"œufs"`, puis affichez le nombre
d'articles et la liste numérotée à partir de 1 :

```text
3 articles
1. pain
2. lait
3. œufs
```

> **Indice :** `push(courses, "œufs")` donne une **nouvelle** liste :
> donnez-lui un nouveau nom.

> **Indice :** `each_with_index` donne l'article et sa position, qui
> commence à 0 : affichez `{i + 1}`.

> **Solution**
>
> ```rythme
> courses = ["pain", "lait"]
> courses2 = push(courses, "œufs")
> print("{len(courses2)} articles")
> courses2.each_with_index do |article, i|
>   print("{i + 1}. {article}")
> end
> ```
>
> ```sortie
> 3 articles
> 1. pain
> 2. lait
> 3. œufs
> ```
>
> Pourquoi ça marche : `courses` n'a pas changé, `courses2` a trois
> articles. Les positions vont de 0 à 2, d'où le `+ 1` pour un humain.

### Exercice 13 : La fiche d'un livre

Niveau : ★☆☆

Rangez un livre dans un enregistrement avec un titre, un auteur et un
nombre de pages. Affichez une phrase, puis fabriquez une nouvelle fiche
avec 120 pages, et affichez les deux nombres de pages :

```text
Le Petit Prince, de Saint-Exupéry (96 pages)
96 120
```

> **Indice :** un enregistrement s'écrit `{titre: "...", auteur: "...", pages: 96}`
> et un champ se lit avec un point : `livre.titre`.

> **Indice :** `{..livre, pages: 120}` copie la fiche en changeant un seul
> champ.

> **Solution**
>
> ```rythme
> livre = {titre: "Le Petit Prince", auteur: "Saint-Exupéry", pages: 96}
> print("{livre.titre}, de {livre.auteur} ({livre.pages} pages)")
> livre2 = {..livre, pages: 120}
> print(livre.pages, livre2.pages)
> ```
>
> ```sortie
> Le Petit Prince, de Saint-Exupéry (96 pages)
> 96 120
> ```
>
> Pourquoi ça marche : `livre.pages = 120` serait une erreur, car rien ne se
> modifie. `{..livre, pages: 120}` fabrique une copie, et l'original garde
> ses 96 pages.

### Exercice 14 : Les capitales

Niveau : ★☆☆

Rangez trois pays et leur capitale dans un dictionnaire. Affichez la
capitale de l'Italie, puis cherchez celle du Pérou, qui n'y est pas :

```text
Rome
inconnue
```

> **Indice :** `dict([["France", "Paris"], ["Italie", "Rome"], ...])`, puis
> `capitales["Italie"]`.

> **Indice :** `capitales["Pérou"]` arrêterait le programme. `get` donne
> une option, et `unwrap_or("inconnue")` une valeur de secours.

> **Solution**
>
> ```rythme
> capitales = dict([["France", "Paris"], ["Italie", "Rome"], ["Japon", "Tokyo"]])
> print(capitales["Italie"])
> print(get(capitales, "Pérou") |> unwrap_or("inconnue"))
> ```
>
> ```sortie
> Rome
> inconnue
> ```
>
> Pourquoi ça marche : `get` donne `None` pour une clé absente, et
> `unwrap_or` remplace `None` par `"inconnue"`.

### Exercice 15 : Les lettres différentes

Niveau : ★☆☆

Combien de lettres différentes y a-t-il dans `abracadabra` ? Affichez-les,
puis leur nombre :

```text
set("a", "b", "c", "d", "r") 5
```

> **Indice :** `chars(mot)` donne la liste des caractères.

> **Indice :** un ensemble (`set`) garde chaque valeur une seule fois.

> **Solution**
>
> ```rythme
> mot = "abracadabra"
> lettres = set(chars(mot))
> print(lettres, len(lettres))
> ```
>
> ```sortie
> set("a", "b", "c", "d", "r") 5
> ```
>
> Pourquoi ça marche : `chars` donne 11 caractères, avec des répétitions.
> `set` enlève les doublons et range les lettres dans l'ordre.

## Et ensuite ?

Bravo si vous êtes arrivé jusqu'ici ! Des exercices plus variés, du facile
au difficile, vous attendent dans [les exercices corrigés](exercices.md).
`rythme exercise` propose aussi 30 exercices dans le terminal.

Suite : [Penser fonctionnel](fonctionnel.md)
