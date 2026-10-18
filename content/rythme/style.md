# Bien écrire du Rythme

Un programme est lu bien plus souvent qu'il n'est écrit : cette page présente le style officiel de Rythme, `rythme fmt` qui l'applique pour vous, et les habitudes qui rendent un programme facile à lire, à documenter et à tester.

> **Objectifs :**
>
> - vous saurez mettre votre code en forme avec `rythme fmt` ;
> - vous saurez choisir des noms clairs, en français et avec accents ;
> - vous saurez découper un programme en petites fonctions, et préférer les enchaînements `|>` aux appels imbriqués ;
> - vous saurez documenter vos fonctions avec `##` et produire leur documentation avec `rythme doc` ;
> - vous saurez placer vos tests à côté de votre code.

## Pourquoi un style commun

Imaginez une bibliothèque où chaque livre aurait sa propre mise en page :
les titres en bas, les chapitres sans numéro. On lirait plus lentement. Le
code, c'est pareil : quand tous les programmes se ressemblent, on lit
directement **ce qu'ils font**, sans être gêné par **comment ils sont
écrits**.

Rythme a donc un style officiel, et un outil qui l'applique : vous n'avez
pas à retenir les règles, ni à en discuter.

## rythme fmt : la mise en forme automatique

Ce brouillon marche, mais il est pénible à lire :

```rythme
fn taille(t)=match t
    t if t>30->"gros"
    _->"petit"
  end
paniers=[[12,5],[30,8],[8,8,8]]
totaux=paniers|>map(sum)
    |>sort
print( totaux ) ; print(map(totaux,taille))
```

```sortie
[17, 24, 38]
["petit", "petit", "gros"]
```

`rythme fmt brouillon.ry` le réécrit ainsi :

```rythme
fn taille(t) = match t
  t if t > 30 -> "gros"
  _ -> "petit"
end

paniers = [[12, 5], [30, 8], [8, 8, 8]]

totaux = paniers
  |> map(sum)
  |> sort

print(totaux)
print(map(totaux, taille))
```

```sortie
[17, 24, 38]
["petit", "petit", "gros"]
```

Le programme fait **exactement** la même chose : `rythme fmt` ne change que
la disposition (les espaces, les lignes vides, les retours à la ligne). Il
relit son résultat, et laisse le fichier tel quel s'il y a le moindre doute.

Les commandes à connaître :

```text
$ rythme fmt --check brouillon.ry
à mettre en forme : brouillon.ry
✗ 1 fichier(s) à mettre en forme, 0 déjà en forme ; `rythme fmt` (sans --check) les met en forme
$ rythme fmt brouillon.ry
mis en forme : brouillon.ry
1 fichier(s) mis en forme, 0 déjà en forme
$ rythme fmt --check brouillon.ry
✓ 1 fichier(s) déjà en forme
```

- `rythme fmt fichier.ry` réécrit le fichier ; `rythme fmt .` traite tout
  le dossier ;
- `--check` ne change rien : il dit seulement ce qui serait changé ;
- `rythme help fmt` résume le style.

> **Astuce :** un [éditeur](../debuter/editeur.md) configuré pour Rythme
> (avec `rythme lsp`) sait mettre le fichier en forme exactement comme
> `rythme fmt`, d'un raccourci clavier.

## Le style officiel en bref

Voici ce qu'applique `rythme fmt`, d'après `rythme help fmt` :

| Règle | Exemple |
|-------|---------|
| deux espaces par niveau d'indentation | `  _ -> "petit"` |
| des espaces autour des opérateurs, de `=`, `->` et `\|>` | `x = a + b * 2` |
| pas d'espace dans les parenthèses et crochets, ni autour de `..` | `f(x)`, `[1, 2]`, `1..10` |
| un enchaînement sur plusieurs lignes : une étape par ligne | `\|> map(sum)` |
| une instruction par ligne, pas de `;` | `print(totaux)` |
| `end` sur sa propre ligne | `end` |
| une ligne vide autour d'une définition de plusieurs lignes | |
| une liste sur plusieurs lignes finit par une virgule | `  "Ada",` |

`rythme fmt` garde vos commentaires, et vos retours à la ligne là où le
langage laisse le choix.

## Des noms qui disent ce qu'ils contiennent

Un bon nom évite un commentaire. Comparez :

```rythme
fn f(l) = if len(l) == 0 then 0 else sum(l) / len(l)

print(f([12, 15, 9]))
```

```sortie
12.0
```

```rythme
fn moyenne([]) = 0
fn moyenne(notes) = sum(notes) / len(notes)

print(moyenne([12, 15, 9]), moyenne([]))
```

```sortie
12.0 0
```

Dans la seconde version, on sait sans réfléchir ce que fait la fonction et
ce qu'elle attend. Quelques règles :

- les mots-clés et les fonctions intégrées sont en anglais, mais **vos
  noms peuvent être en français**, accents compris : `prénom`, `âge`,
  `quantité` ;
- des minuscules, et `_` entre les mots : `prix_total`, `nom_du_jour` ;
- une majuscule au début est réservée aux types et aux constructeurs :
  `Cercle`, `Some` ;
- une fonction qui répond par oui ou non peut finir par `?`, comme en Ruby ;
- un nom d'une seule lettre convient seulement pour une toute petite
  fonction : `fn(x) = x * 2`.

```rythme
fn majeur?(âge) = âge >= 18

élève = {prénom: "Zoé", âge: 17}
print(élève.prénom, majeur?(élève.âge))
```

```sortie
Zoé false
```

## Des enchaînements plutôt que des appels imbriqués

Des appels imbriqués se lisent de l'intérieur vers l'extérieur, en comptant
les parenthèses :

```rythme
notes = [12, 7, 15, 9, 18]
print(sum(map(filter(notes, fn(n) = n >= 10), fn(n) = n * 2)))
```

```sortie
90
```

Avec `|>`, le même calcul se lit de haut en bas, dans l'ordre où il se
fait : garder les notes d'au moins 10, les doubler, faire la somme.

```rythme
notes = [12, 7, 15, 9, 18]

total = notes
  |> filter(fn(n) = n >= 10)
  |> map(fn(n) = n * 2)
  |> sum

print(total)
```

```sortie
90
```

Dès qu'il y a plus de deux appels emboîtés, préférez l'enchaînement, une
étape par ligne. Voir [la programmation fonctionnelle](fonctionnel.md).

## Blocs ou fn : choisir une forme

Rythme accepte trois façons d'écrire la même chose :

```rythme
nombres = [1, 2, 3]

print(map(nombres, fn(x) = x * 2))
print(nombres |> map(fn(x) = x * 2))
print(nombres.map { |x| x * 2 })
```

```sortie
[2, 4, 6]
[2, 4, 6]
[2, 4, 6]
```

La forme avec `fn` et `|>` est celle de ce cours et du manuel. Les blocs
`{ |x| ... }` et `do |x| ... end` sont là pour les habitués de Ruby (voir
[venir de Ruby](venir-de-ruby.md)). Toutes sont justes ; l'important est de
**ne pas mélanger** les styles dans un même fichier. `rythme fmt` garde la
forme que vous avez choisie.

## De petites fonctions

Une fonction qui fait tout est difficile à lire et impossible à tester par
morceaux :

```rythme
fn bulletin(eleves)
  lignes = map(eleves, fn(e) = do
    m = sum(e.notes) / len(e.notes)
    mention = if m >= 16 then "très bien" else if m >= 12 then "bien" else "passable"
    "{e.nom} : {m} ({mention})"
  end)
  join(lignes, "\n")
end

classe = [{nom: "Ada", notes: [16, 18]}, {nom: "Alan", notes: [11, 13]}]
print(bulletin(classe))
```

```sortie
Ada : 17.0 (très bien)
Alan : 12.0 (bien)
```

Découpée, chaque fonction a **un seul travail** et un nom qui le dit. Les
mentions deviennent une fonction définie par cas, avec des gardes, plus
lisible qu'une chaîne de `else if` :

```rythme
fn moyenne(notes) = sum(notes) / len(notes)

fn mention(m) if m >= 16 = "très bien"
fn mention(m) if m >= 12 = "bien"
fn mention(_) = "passable"

fn ligne(eleve)
  m = moyenne(eleve.notes)
  "{eleve.nom} : {m} ({mention(m)})"
end

classe = [{nom: "Ada", notes: [16, 18]}, {nom: "Alan", notes: [11, 13]}]
classe |> map(ligne) |> each(print)
```

```sortie
Ada : 17.0 (très bien)
Alan : 12.0 (bien)
```

Un bon repère : si une fonction ne tient pas sur votre écran, ou si vous
avez du mal à lui trouver un nom, découpez-la.

## Documenter avec ## et rythme doc

Un commentaire qui commence par `##`, juste au-dessus d'une `fn`, documente
la fonction. Les lignes `##` du tout début du fichier, séparées du reste
par une ligne vide, décrivent le fichier entier :

```rythme norun
## Calculs sur un panier d'achats.

## Le prix d'une ligne du panier : le prix unitaire fois la quantité.
fn prix_ligne({prix, quantité}) = prix * quantité

## Le total du panier, en centimes. Voir aussi `prix_ligne`.
fn total(panier) = panier |> map(prix_ligne) |> sum
```

`help(total)` affiche ce commentaire (voir [les étapes](etapes.md)), et
`rythme doc` en fait une page de documentation, en Markdown :

```text
$ rythme doc panier.ry
# panier.ry

Calculs sur un panier d'achats.

## Fonctions

### prix_ligne

~~~rythme
fn prix_ligne({prix, quantité})
~~~

Le prix d'une ligne du panier : le prix unitaire fois la quantité.

### total

~~~rythme
fn total(panier)
~~~

Le total du panier, en centimes. Voir aussi [`prix_ligne`](#prix_ligne).
```

Un nom entre accents graves devient un lien. `rythme doc --html panier.ry
--out doc.html` écrit une page web. Documentez **ce que fait** la fonction
et ce qu'elle attend, pas comment elle le fait : le code le dit déjà.

## Les tests à côté du code

En Rythme, un test s'écrit dans le même fichier que la fonction qu'il
vérifie, juste en dessous (voir [les tests](tests.md)). `rythme fichier.ry`
les ignore ; `rythme test fichier.ry` les lance.

```rythme
fn moyenne(notes) = sum(notes) / len(notes)

fn mention(m) if m >= 16 = "très bien"
fn mention(m) if m >= 12 = "bien"
fn mention(_) = "passable"

test "moyenne de deux notes" = assert_eq(moyenne([10, 20]), 15.0)
test "mention bien" = assert_eq(mention(12.5), "bien")

print(mention(moyenne([16, 18])))
```

```sortie
très bien
```

```text
$ rythme test bulletin.ry
Exécution de 2 test(s)
  ✓ moyenne de deux notes
  ✓ mention bien
2 test(s) : 2 réussi(s), 0 échoué(s) (1 ms)
```

Les petites fonctions de la section précédente se testent une à une ; la
grosse fonction `bulletin`, elle, ne pouvait se tester qu'en entier.

## Erreurs fréquentes

### Séparer le commentaire ## de sa fonction

```rythme norun
## Le double d'un nombre.

fn double(x) = x * 2

help(double)
```

```text
double(x)
  (pas de documentation : écrivez `## description` sur la ligne au-dessus de `fn`)
  définie en main.ry:3:4
```

Une ligne vide détache le commentaire `##` de la fonction. Collez-le à la
ligne de `fn`.

### Commencer un nom par une majuscule

```rythme norun
Total = 3
print(Total)
```

```text
erreur[E003]: constructeur inconnu `Total`
 --> main.ry:1:1
  |
1 | Total = 3
  | ^^^^^ non défini
  |
  = aide: les constructeurs se définissent avec `type` : `type Nom = Total(champ) | Autre`
  = en savoir plus : `rythme explain E003`

erreur[E003]: nom inconnu `Total`
 --> main.ry:2:7
  |
2 | print(Total)
  |       ^^^^^ non défini
  |
  = aide: pour créer votre propre sorte de valeur, définissez un type : `type Nom = Total(champ) | Autre`
  = en savoir plus : `rythme explain E003`
```

Une majuscule au début désigne un constructeur de type. Écrivez `total`.

### Mettre un tiret dans un nom

```rythme norun
prix-ttc = 3
```

```text
erreur[E001]: impossible de donner une valeur à ceci
 --> main.ry:1:1
  |
1 | prix-ttc = 3
  | ^^^^^^^^ ceci n'est pas un nom
  |
1 | prix-ttc = 3
  |          - `=` donne un nom à une valeur
  |
  = aide: pour comparer deux valeurs, utilisez `==`
  = en savoir plus : `rythme explain E001`
```

`prix-ttc` se lit « `prix` moins `ttc` ». Séparez les mots par `_` :
`prix_ttc`.

## Testez-vous

**Question 1 :** `rythme fmt` peut-il changer le résultat de votre
programme ?

> **Réponse**
>
> Non. Il ne change que la disposition : espaces, lignes vides, retours à
> la ligne. Il vérifie que le programme reste le même, sinon il laisse le
> fichier tel quel.

**Question 2 :** réécrivez `print(len(filter(mots, fn(m) = len(m) > 3)))`
avec des enchaînements.

> **Réponse**
>
> ```rythme
> mots = ["un", "chat", "dort", "ici"]
>
> mots
>   |> filter(fn(m) = len(m) > 3)
>   |> len
>   |> print
> ```
>
> ```sortie
> 2
> ```

**Question 3 :** où écrire le commentaire qui documente une fonction ?

> **Réponse**
>
> Sur les lignes juste au-dessus de `fn`, chacune commençant par `##`,
> sans ligne vide entre elles et la fonction.

## En résumé

- `rythme fmt` applique le style officiel ; `--check` vérifie sans rien
  changer.
- Des noms clairs, en minuscules avec `_`, en français et avec accents si
  vous voulez ; les majuscules sont pour les types.
- Préférez `|>`, une étape par ligne, aux appels imbriqués, et gardez une
  seule forme (`fn` ou blocs) par fichier.
- De petites fonctions, avec un seul travail chacune, se lisent et se
  testent mieux.
- `##` documente une fonction pour `help` et `rythme doc` ; les tests
  s'écrivent à côté du code.

Suite : [venir de Ruby (ou de Python)](venir-de-ruby.md). Comparez avec le
style en [Ruby](../ruby/style.md) et en [Rust](../rust/style.md).
