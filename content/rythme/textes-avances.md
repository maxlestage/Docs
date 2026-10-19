# Textes avancés : nombres, colonnes et rapports

Cette page montre comment écrire des nombres lisibles avec `format_number`, aligner des colonnes avec `pad_left` et `pad_right`, puis construire pas à pas un petit rapport à partir d'une liste d'enregistrements.

> **Objectifs :**
>
> - vous saurez écrire un nombre pour les humains : milliers séparés, décimales, virgule à la française ;
> - vous saurez choisir entre `format_number`, `text(x, 2)` et `round` ;
> - vous saurez remplir un texte à gauche ou à droite pour aligner des colonnes ;
> - vous saurez calculer la largeur d'une colonne à partir des données ;
> - vous saurez construire un rapport ligne par ligne, puis recoller les lignes avec `join`.

## Le but : un rapport lisible

Vous avez une liste de ventes. Afficher les
[enregistrements](../debuter/glossaire.md#enregistrement) tels quels donne
ceci :

```text
{nom: "Pommes", qte: 12, prix: 0.85}
{nom: "Ananas", qte: 3, prix: 2.5}
{nom: "Fraises", qte: 40, prix: 1.2}
```

C'est juste, mais difficile à lire. À la fin de cette page, vous saurez
écrire ceci :

```text
Produit     Qté     Montant
---------------------------
Pommes       12     10,20 €
Ananas        3      7,50 €
Fraises      40     48,00 €
---------------------------
Total               65,70 €
```

Pensez à un ticket de caisse : chaque chose a sa colonne, les nombres sont
alignés à droite, et le total est en bas. Nous allons y arriver en petites
étapes. Chaque outil d'abord, le rapport ensuite.

Les bases des textes (`split`, `join`, `pad_left`, `repeat`) sont dans
[travailler avec du texte](texte.md). Cette page va plus loin.

## Écrire un nombre pour les humains

`format_number(nombre, decimales, langue)` transforme un nombre en
[texte](../debuter/glossaire.md#chaine-de-caracteres) facile à lire :

- il sépare les milliers ;
- il écrit **exactement** le nombre de décimales demandé ;
- il suit l'écriture d'une langue : `"fr"` pour le français.

```rythme
print(format_number(1234567.891, 2, "fr"))
print(format_number(1234567.891, 2))
print(format_number(1234567))
```

```sortie
1 234 567,89
1,234,567.89
1,234,567
```

Sans langue, c'est l'écriture anglaise : une virgule entre les milliers, un
point avant les décimales. En français, c'est l'inverse : une espace entre
les milliers, une virgule avant les décimales.

Le résultat est un **texte**, pas un nombre. On ne calcule plus avec. On
l'affiche.

```rythme
joli = format_number(1500, 2, "fr")
print(joli, type_of(joli))
```

```sortie
1 500,00 Text
```

> **À retenir :** calculez avec des nombres, et appelez `format_number`
> tout à la fin, juste avant d'afficher.

### Les trois arguments, un par un

| Appel | Ce qu'il donne | Pourquoi |
|-------|----------------|----------|
| `format_number(7)` | `7` | pas de décimales demandées : le nombre tel quel |
| `format_number(7, 2)` | `7.00` | exactement 2 décimales, des zéros si besoin |
| `format_number(7, 2, "fr")` | `7,00` | la virgule française |
| `format_number(2.5, 0)` | `3` | 0 décimale : on arrondit |
| `format_number(12345, 0, "de")` | `12.345` | en allemand, un point entre les milliers |

Les langues connues sont `"en"` (par défaut), `"fr"`, `"de"`, `"es"`,
`"it"`, `"nl"` et `"pt"`.

Essayez : remplacez `"fr"` par `"de"` dans le premier exemple. Vous
obtenez `1.234.567,89` : des points entre les milliers, une virgule avant
les décimales.

### Il cache les petites erreurs des nombres à virgule

Un [nombre à virgule](../debuter/glossaire.md#flottant) n'est pas toujours
exact dans un ordinateur (voir [les pièges](pieges.md)). `format_number`
arrondit, et le problème disparaît à l'affichage :

```rythme
print(0.1 + 0.2)
print(format_number(0.1 + 0.2, 2, "fr"))
```

```sortie
0.30000000000000004
0,30
```

C'est comme une balance de cuisine qui affiche 300 g : elle ne montre pas
les milligrammes, et c'est très bien ainsi.

## Arrondir : trois outils voisins

Trois fonctions savent arrondir. Elles ne donnent pas la même chose :

| Outil | Exemple | Résultat | C'est... |
|-------|---------|----------|----------|
| `round(x, 2)` | `round(3.14159, 2)` | `3.14` | un **nombre** |
| `text(x, 2)` | `text(3.14159, 2)` | `"3.14"` | un texte, avec un point |
| `format_number(x, 2, "fr")` | `format_number(3.14159, 2, "fr")` | `"3,14"` | un texte, avec milliers et virgule |

```rythme
x = 1234.5
print(round(x, 2), type_of(round(x, 2)))
print(text(x, 2), type_of(text(x, 2)))
print(format_number(x, 2, "fr"), type_of(format_number(x, 2, "fr")))
```

```sortie
1234.5 Float
1234.50 Text
1 234,50 Text
```

Regardez la première ligne : `round` donne un nombre, et un nombre ne
garde pas les zéros de la fin. Pour **afficher** deux décimales, il faut un
texte : `text(x, 2)` ou `format_number`.

Quelle fonction choisir ?

```text
 Je veux continuer à calculer ............ round(x, 2)
 Je veux un texte simple (fichier, JSON) .. text(x, 2)
 Je veux l'afficher à une personne ........ format_number(x, 2, "fr")
```

> **À retenir :** `round` donne un nombre ; `text` et `format_number`
> donnent un texte, prêt à être affiché.

### Les montants en centimes

Pour de l'argent, une bonne habitude : compter en **centimes**, avec des
[entiers](../debuter/glossaire.md#entier). Les entiers sont toujours exacts.
On divise par 100 seulement pour afficher :

```rythme
prix_centimes = [199, 250, 1005]
total = sum(prix_centimes)
print(total)
print(format_number(total / 100, 2, "fr") + " €")
```

```sortie
1454
14,54 €
```

| Étape | Valeur | Sorte |
|-------|--------|-------|
| `sum(prix_centimes)` | `1454` | entier, exact |
| `total / 100` | `14.54` | nombre à virgule |
| `format_number(..., 2, "fr")` | `"14,54"` | texte |
| `+ " €"` | `"14,54 €"` | texte |

## Remplir un texte pour l'aligner

Une colonne bien alignée, c'est des textes qui ont **tous la même
largeur**. On les complète avec des espaces.

- `pad_right(texte, largeur)` ajoute des espaces **à droite** : le texte
  reste collé à gauche. Pour les mots.
- `pad_left(texte, largeur)` ajoute des espaces **à gauche** : le texte est
  poussé à droite. Pour les nombres.

Pour **voir** les espaces, remplissons avec des points (le troisième
argument choisit le caractère) :

```rythme
print(pad_right("Pommes", 10, "."))
print(pad_left("12", 5, "."))
print(pad_left("12", 5) + "|")
```

```sortie
Pommes....
...12
   12|
```

Comme une règle graduée : on part de la longueur du texte, et on ajoute
juste ce qu'il faut pour arriver à la largeur.

```text
pad_right("Pommes", 10)          pad_left("12", 5)
 P o m m e s · · · ·              · · · 1 2
 └─── 6 ───┘ └─ 4 ─┘              └─ 3 ─┘ └2┘
 └────── 10 ───────┘              └─── 5 ───┘
```

### Pourquoi les nombres vont à droite

Alignés à droite, les unités sont sous les unités et les dizaines sous les
dizaines. On compare d'un coup d'œil :

```rythme
nombres = [5, 120, 42]
nombres |> each(fn(n) = print(pad_right(n, 5) + "|"))
print("")
nombres |> each(fn(n) = print(pad_left(n, 5) + "|"))
```

```sortie
5    |
120  |
42   |

    5|
  120|
   42|
```

La deuxième colonne se lit comme une addition posée à l'école.

Essayez : remplacez `5` par `2` dans `pad_left(n, 5)`. Le nombre `120` a
déjà 3 caractères : il dépasse, et la colonne se décale. La section
suivante règle ce problème.

> **À retenir :** les mots à gauche avec `pad_right`, les nombres à droite
> avec `pad_left`.

### Un texte trop long n'est pas coupé

Si le texte est déjà plus long que la largeur, `pad_left` et `pad_right`
le rendent **tel quel**. Ils ne coupent jamais :

```rythme
print("[" + pad_right("Pamplemousses", 8) + "]")
print("[" + take("Pamplemousses", 8) + "]")
```

```sortie
[Pamplemousses]
[Pamplemo]
```

Pour couper, utilisez `take(texte, n)` : il garde les `n` premiers
caractères. Une petite fonction fait les deux, couper puis remplir :

```rythme
fn ajuster(t, largeur) = pad_right(take(t, largeur), largeur)

["Kiwi", "Pamplemousses"] |> each(fn(m) = print("[" + ajuster(m, 8) + "]"))
```

```sortie
[Kiwi    ]
[Pamplemo]
```

## Choisir la largeur d'une colonne

Plutôt que de deviner la largeur, calculons-la : c'est la longueur du
**plus long** texte de la colonne.

```rythme
noms = ["Pommes", "Ananas", "Fraises"]
largeur = noms |> map(len) |> max
print(largeur)
noms |> each(fn(n) = print(pad_right(n, largeur) + " |"))
```

```sortie
7
Pommes  |
Ananas  |
Fraises |
```

Suivons le calcul de `largeur`, une étape à la fois :

| Étape | Valeur |
|-------|--------|
| `noms` | `["Pommes", "Ananas", "Fraises"]` |
| `\|> map(len)` | `[6, 6, 7]` |
| `\|> max` | `7` |

Ajoutez un nom plus long à la liste, et la colonne s'élargit toute seule.

Essayez : ajoutez `"Mirabelles"` à la liste `noms`. La largeur passe à 10,
et les barres `|` restent alignées.

> **Attention :** `len` compte les caractères : `len("Qté")` vaut 3, l'accent
> compte pour un. Les noms très longs ou les emojis peuvent quand même
> décaler un peu l'affichage dans certains terminaux.

> **À retenir :** la largeur d'une colonne = la longueur de son plus long
> texte : `textes |> map(len) |> max`.

## Construire un rapport pas à pas

Passons au rapport du début. On suit une recette en cinq étapes, comme on
monte un meuble : une pièce à la fois, et on vérifie à chaque étape.

```text
 1. une fonction pour UNE ligne
 2. map : une ligne pour chaque vente
 3. le total
 4. l'en-tête, les traits et le pied
 5. join : tout recoller avec "\n", et un seul print
```

### Étape 1 : une seule ligne

On écrit d'abord une [fonction](../debuter/glossaire.md#fonction) qui
transforme **une** vente en **une** ligne de texte. Et une petite fonction
`euros` pour les montants :

```rythme
fn euros(x) = format_number(x, 2, "fr") + " €"

fn ligne(v) =
  pad_right(v.nom, 10) + pad_left(v.qte, 5) + pad_left(euros(v.qte * v.prix), 12)

print(ligne({nom: "Pommes", qte: 12, prix: 0.85}))
print(ligne({nom: "Ananas", qte: 3, prix: 2.5}))
```

```sortie
Pommes       12     10,20 €
Ananas        3      7,50 €
```

Décomposons la première ligne, morceau par morceau :

| Morceau | Calcul | Texte obtenu | Largeur |
|---------|--------|--------------|---------|
| nom | `pad_right("Pommes", 10)` | `"Pommes    "` | 10 |
| quantité | `pad_left(12, 5)` | `"   12"` | 5 |
| montant | `12 * 0.85` → `10.2` → `euros(10.2)` | `"10,20 €"` | 7 |
| montant aligné | `pad_left("10,20 €", 12)` | `"     10,20 €"` | 12 |
| la ligne | les trois collés avec `+` | `"Pommes       12     10,20 €"` | 27 |

### Étape 2 : une ligne pour chaque vente

Il n'y a pas de boucle en Rythme : `map` applique `ligne` à chaque vente et
donne la liste des lignes.

```rythme
fn euros(x) = format_number(x, 2, "fr") + " €"
fn ligne(v) =
  pad_right(v.nom, 10) + pad_left(v.qte, 5) + pad_left(euros(v.qte * v.prix), 12)

ventes = [
  {nom: "Pommes", qte: 12, prix: 0.85},
  {nom: "Ananas", qte: 3, prix: 2.5},
  {nom: "Fraises", qte: 40, prix: 1.2},
]
corps = ventes |> map(ligne)
print(len(corps))
print(corps[2])
```

```sortie
3
Fraises      40     48,00 €
```

| Tour de `map` | `v.nom` | `v.qte * v.prix` | Ligne fabriquée |
|---------------|---------|------------------|-----------------|
| 1 | `"Pommes"` | `10.2` | `Pommes       12     10,20 €` |
| 2 | `"Ananas"` | `7.5` | `Ananas        3      7,50 €` |
| 3 | `"Fraises"` | `48.0` | `Fraises      40     48,00 €` |

### Étape 3 : le total

`sum` avec une fonction additionne ce que la fonction donne pour chaque
vente :

```rythme
ventes = [
  {nom: "Pommes", qte: 12, prix: 0.85},
  {nom: "Ananas", qte: 3, prix: 2.5},
  {nom: "Fraises", qte: 40, prix: 1.2},
]
total = ventes |> sum(fn(v) = v.qte * v.prix)
print(format_number(total, 2, "fr"))
```

```sortie
65,70
```

| Vente | Montant | Total jusqu'ici |
|-------|---------|-----------------|
| (départ) | | `0` |
| Pommes | `10.2` | `10.2` |
| Ananas | `7.5` | `17.7` |
| Fraises | `48.0` | `65.7` |

### Étapes 4 et 5 : assembler et recoller

On fabrique l'en-tête, un trait et le pied avec les **mêmes largeurs**
(10, 5 et 12). Puis on met toutes les lignes dans une seule liste, et
`join(liste, "\n")` les recolle avec un retour à la ligne entre chacune.

```rythme
fn euros(x) = format_number(x, 2, "fr") + " €"
fn ligne(v) =
  pad_right(v.nom, 10) + pad_left(v.qte, 5) + pad_left(euros(v.qte * v.prix), 12)

ventes = [
  {nom: "Pommes", qte: 12, prix: 0.85},
  {nom: "Ananas", qte: 3, prix: 2.5},
  {nom: "Fraises", qte: 40, prix: 1.2},
]
corps = ventes |> map(ligne)
total = ventes |> sum(fn(v) = v.qte * v.prix)

entete = pad_right("Produit", 10) + pad_left("Qté", 5) + pad_left("Montant", 12)
trait = repeat("-", 27)
pied = pad_right("Total", 15) + pad_left(euros(total), 12)

rapport = [entete, trait, ..corps, trait, pied] |> join("\n")
print(rapport)
```

```sortie
Produit     Qté     Montant
---------------------------
Pommes       12     10,20 €
Ananas        3      7,50 €
Fraises      40     48,00 €
---------------------------
Total               65,70 €
```

`..corps` met les lignes de `corps` **à cet endroit** de la liste, une par
une. Sans les deux points, `corps` serait une liste dans la liste.

```text
[ entete, trait, ..corps,                       trait, pied ]
                   │
                   ▼
[ entete, trait, ligne1, ligne2, ligne3,         trait, pied ]
                   │
          join("\n")
                   ▼
"entete\ntrait\nligne1\nligne2\nligne3\ntrait\npied"
```

Le trait fait 27 caractères : 10 + 5 + 12, la somme des largeurs. Le pied
utilise 15 pour le mot « Total » : il prend la place du nom **et** de la
quantité (10 + 5).

Essayez : ajoutez une quatrième vente, `{nom: "Kiwis", qte: 7, prix: 0.4}`.
Une ligne de plus apparaît, et le total devient `68,50 €`. Vous n'avez rien
d'autre à changer.

> **À retenir :** une fonction pour une ligne, `map` pour toutes les
> lignes, `join("\n")` pour en faire un seul texte.

## Recoller des lignes et des mots

`join` est l'inverse de `split` et de `lines`. Le séparateur choisit ce qui
va **entre** les morceaux, jamais au début ni à la fin :

```rythme
morceaux = ["a", "b", "c"]
print(join(morceaux, ", "))
print(join(morceaux, ""))
print(join(morceaux, "\n"))
```

```sortie
a, b, c
abc
a
b
c
```

`join` accepte aussi une liste de nombres : `join([1, 2, 3], " + ")` donne
`"1 + 2 + 3"`.

### Une liste en français : « a, b et c »

En français, on écrit « pommes, kiwis et ananas » : des virgules, sauf
avant le dernier mot. On sépare le cas en trois, avec une
[définition par cas](fonctions.md#definir-par-cas) :

```rythme
fn liste_humaine([]) = ""
fn liste_humaine([seul]) = seul
fn liste_humaine(mots) =
  join(take(mots, len(mots) - 1), ", ") + " et " + unwrap(last(mots))

print(liste_humaine(["pommes", "kiwis", "ananas"]))
print(liste_humaine(["pommes", "kiwis"]))
print(liste_humaine(["pommes"]))
```

```sortie
pommes, kiwis et ananas
pommes et kiwis
pommes
```

Pour `["pommes", "kiwis", "ananas"]` :

| Étape | Calcul | Valeur |
|-------|--------|--------|
| 1 | `take(mots, 2)` | `["pommes", "kiwis"]` |
| 2 | `join(..., ", ")` | `"pommes, kiwis"` |
| 3 | `unwrap(last(mots))` | `"ananas"` |
| 4 | `"pommes, kiwis" + " et " + "ananas"` | `"pommes, kiwis et ananas"` |

`last` donne une [Option](../debuter/glossaire.md#option) (la liste pourrait
être vide) ; ici, on sait qu'elle ne l'est pas, alors `unwrap` l'ouvre.

## Un rapport à partir d'un texte

Souvent, les données arrivent sous forme de texte, une ligne par chose :
un fichier, un copier-coller. Il faut d'abord les **lire**, puis les
afficher. On enchaîne `lines`, `split`, `parse_float` :

```rythme
donnees = "Loyer;850\nCourses;312.4\nTransport;75.5"

fn lire(l) = match split(l, ";")
  [nom, montant] -> {nom: nom, montant: parse_float(montant) |> unwrap_or(0.0)}
  _ -> {nom: "?", montant: 0.0}
end

depenses = donnees |> lines |> map(lire)
largeur = depenses |> map(fn(d) = len(d.nom)) |> max

depenses |> each(fn(d) =
  print(pad_right(d.nom, largeur) + pad_left(format_number(d.montant, 2, "fr"), 10)))
print(pad_right("Total", largeur) + pad_left(format_number(sum(depenses, fn(d) = d.montant), 2, "fr"), 10))
```

```sortie
Loyer        850,00
Courses      312,40
Transport     75,50
Total      1 237,90
```

| Étape | Ce qu'on a |
|-------|-----------|
| `donnees` | un seul texte, avec des `\n` |
| `\|> lines` | `["Loyer;850", "Courses;312.4", "Transport;75.5"]` |
| `\|> map(lire)` | trois enregistrements `{nom, montant}`, montants en nombres |
| `largeur` | `9` (la longueur de `"Transport"`) |
| `each` | une ligne affichée par dépense |

Remarquez le total : `format_number` a ajouté l'espace des milliers, et
`pad_left` l'a quand même bien aligné, car il compte tous les caractères.

Lire un vrai fichier se fait avec `read_file` : voir
[les fichiers](fichiers.md).

## Erreurs fréquentes

### Donner un texte à format_number

Les morceaux d'un `split` sont des **textes**, même s'ils contiennent des
chiffres :

```rythme norun
print(format_number("1234"))
```

```text
erreur[E005]: `format_number` attend un nombre comme argument 1, mais a reçu Text "1234"
 --> main.ry:1:21
  |
1 | print(format_number("1234"))
  |                     ^^^^^^ mauvaise sorte de valeur
  |
  = aide: utilisation : `format_number(x, decimals, language)`
  = en savoir plus : `rythme explain E005`
```

Lisez d'abord le nombre : `parse_float("1234") |> unwrap_or(0.0)`, puis
donnez-le à `format_number`.

### Remplir avec plusieurs caractères

```rythme norun
print(pad_left(7, 3, "00"))
```

```text
erreur[E023]: `pad_left` a besoin d'un seul caractère, mais "00" en a 2
 --> main.ry:1:7
  |
1 | print(pad_left(7, 3, "00"))
  |       ^^^^^^^^^^^^^^^^^^^^
  |
  = en savoir plus : `rythme explain E023`
```

Le remplissage est **un** caractère, répété autant qu'il faut :
`pad_left(7, 3, "0")` donne `"007"`.

### La plus grande largeur d'une liste vide

```rythme norun
noms = []
largeur = noms |> map(len) |> max
print(largeur)
```

```text
erreur[E023]: `max` d'une liste vide
 --> main.ry:2:11
  |
2 | largeur = noms |> map(len) |> max
  |           ^^^^^^^^^^^^^^^^^^^^^^^
  |
  = en savoir plus : `rythme explain E023`
```

Une liste vide n'a pas de plus grand élément. Ajoutez une valeur de
départ, par exemple la longueur du titre de la colonne :
`[len("Produit"), ..(noms |> map(len))] |> max` vaut 7 même si `noms` est
vide. Le titre doit tenir dans
la colonne, de toute façon.

## Testez-vous

**Question 1 :** que donne `format_number(9876.5, 2, "fr")` ? Et est-ce un
nombre ?

> **Réponse**
>
> Le texte `"9 876,50"` : une espace entre les milliers, une virgule, et
> exactement deux décimales. C'est un **texte** (`Text`) : on l'affiche, on
> ne calcule plus avec.

**Question 2 :** que donnent `pad_left("abc", 6, "*")` et
`pad_right("abc", 2)` ?

> **Réponse**
>
> `"***abc"` : trois étoiles à gauche pour arriver à 6 caractères. Et
> `"abc"` : le texte est déjà plus long que 2, il est rendu tel quel, sans
> être coupé.

**Question 3 :** vous avez une liste `lignes` de textes. Comment l'afficher
avec une ligne par élément, en un seul `print` ?

> **Réponse**
>
> `print(join(lignes, "\n"))` : `join` met un retour à la ligne **entre**
> chaque élément, puis `print` affiche le tout.

## En résumé

- `format_number(x, decimales, "fr")` écrit un nombre pour les humains :
  `1 234,50`. Il donne un texte : appelez-le juste avant d'afficher.
- `round` donne un nombre ; `text(x, 2)` et `format_number` donnent un
  texte. Pour l'argent, comptez en centimes et divisez par 100 à la fin.
- `pad_right` aligne les mots à gauche, `pad_left` aligne les nombres à
  droite ; ils ne coupent jamais : `take` coupe.
- La largeur d'une colonne est la longueur de son plus long texte :
  `map(len) |> max`.
- Un rapport : une fonction pour une ligne, `map` pour toutes, puis
  `join("\n")` et un seul `print`.

Suite : [les expressions régulières](regex.md).
