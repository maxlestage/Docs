# Les conditions

Cette page montre comment poser une question avec une comparaison, puis choisir entre plusieurs cas avec `if`, `unless` et `match`.

> **Objectifs :**
>
> - vous saurez écrire une condition avec `==`, `<`, `and`, `or` et `not` ;
> - vous saurez choisir une valeur avec `if ... then ... else`, et pourquoi le `else` est obligatoire ;
> - vous saurez comparer une valeur à plusieurs cas avec `match`.

## Vrai ou faux : les comparaisons

Un programme doit souvent décider : faut-il un parapluie ? la personne est-elle
majeure ? Pour décider, il pose d'abord une question, et la réponse est oui ou non.

Une [condition](../debuter/glossaire.md#condition) est une question dont la
réponse est `true` (vrai) ou `false` (faux), un
[booléen](../debuter/glossaire.md#booleen). On la pose avec un opérateur de
comparaison :

| Opérateur | Question | Exemple | Résultat |
|-----------|----------|---------|----------|
| `==` | égal ? | `3 == 3` | `true` |
| `!=` | différent ? | `3 != 3` | `false` |
| `<`, `>` | plus petit, plus grand ? | `2 < 5` | `true` |
| `<=`, `>=` | plus petit ou égal, plus grand ou égal ? | `5 >= 5` | `true` |

> **Attention :** `=` donne un nom, `==` compare. Ce sont deux choses
> différentes.

Pour combiner des conditions, on utilise des mots :

- `a and b` : vrai si les **deux** sont vraies ;
- `a or b` : vrai si **au moins une** est vraie ;
- `not a` : le contraire de `a`.

```rythme
x = 7
print(x > 0 and x < 10, x < 0 or x == 7, not (x == 7))
```

```sortie
true true false
```

## if donne une valeur

Dans la vie, vous choisissez sans cesse selon la situation : s'il pleut, vous prenez
un manteau, sinon une veste. `if` permet au programme de faire le même genre de choix.

`if` choisit entre deux valeurs selon une condition. En Rythme, `if` n'est
pas un ordre : c'est une **expression**. Il donne une valeur, comme un
calcul. On peut donc la ranger dans un nom :

```rythme
age = 20
statut = if age >= 18 then "majeur" else "mineur"
print(statut)
```

```sortie
majeur
```

Ligne par ligne :

- `age = 20` donne le nom `age` à la valeur 20 ;
- à droite du `=`, `if` pose la question « 20 est-il au moins 18 ? ». La réponse est
  `true`, donc le `if` donne la valeur qui suit `then` : `"majeur"` ;
- cette valeur reçoit le nom `statut` ;
- `print(statut)` affiche `majeur`.

La forme est toujours `if condition then valeur else autre_valeur`. Lisez :
« si l'âge est au moins 18, alors "majeur", sinon "mineur" ».

## Le else est obligatoire

Comme `if` donne toujours une valeur, il lui faut les deux cas : que
vaudrait `statut` si la condition était fausse ? Un `if` sans `else` est donc
une erreur (voir [les erreurs fréquentes](#erreurs-frequentes)).

Quand il n'y a rien à faire dans un cas, donnez la valeur vide `()`, qui
veut dire « rien d'intéressant » :

```rythme
x = 5
if x > 0 then print("positif") else ()
```

```sortie
positif
```

Une branche contient une seule expression. Pour faire plusieurs choses,
regroupez-les dans un bloc `do ... end` : sa dernière ligne donne la valeur.

```rythme
fn classer(n) =
  if n > 100 then do
    print("un grand nombre !")
    "grand"
  end else "petit"

print(classer(500), classer(3))
```

```sortie
un grand nombre !
grand petit
```

## Enchaîner les conditions

Souvent, il n'y a pas que deux cas. Un nombre peut être négatif, nul ou positif, et
vous voulez un mot différent pour chacun, comme sur un bulletin de notes.

`else if` enchaîne plusieurs conditions. `then` et `else` peuvent commencer
une nouvelle ligne :

```rythme
fn signe(n) =
  if n < 0 then "négatif"
  else if n == 0 then "zéro"
  else "positif"

print(signe(-3), signe(0), signe(8))
```

```sortie
négatif zéro positif
```

Ligne par ligne :

- `fn signe(n) =` définit une fonction : elle reçoit un nombre, nommé `n` ;
- pour `signe(-3)`, la première question `-3 < 0` est vraie : la fonction donne
  `"négatif"` et ne lit pas la suite ;
- pour `signe(0)`, `0 < 0` est faux, on passe au `else if` : `0 == 0` est vrai, donc
  `"zéro"` ;
- pour `signe(8)`, les deux questions sont fausses : on arrive au dernier `else`, qui
  donne `"positif"`.

Les conditions sont essayées de haut en bas, et la première vraie gagne.

La condition doit être `true` ou `false`. `if 1 then ...` est une erreur :
rien n'est « vrai » par défaut, ni un nombre, ni un texte, ni une liste.

**À vous :** écrivez une fonction `salut(heure)` qui donne « Bonjour » si l'heure est
avant 12, « Bon après-midi » si elle est avant 18, et « Bonsoir » sinon. Affichez
`salut(15)`.

> **Solution**
>
> ```rythme
> fn salut(heure) =
>   if heure < 12 then "Bonjour"
>   else if heure < 18 then "Bon après-midi"
>   else "Bonsoir"
>
> print(salut(15))
> ```
>
> ```sortie
> Bon après-midi
> ```

## unless, le contraire de if

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

Comme en Ruby, `unless` (« à moins que ») est le contraire de `if` : il
donne la première valeur quand la condition est **fausse**. Lui aussi a
toujours un `else` :

```rythme
stock = 0
print(unless stock > 0 then "épuisé" else "en stock")
```

```sortie
épuisé
```

`unless stock > 0 then a else b` est exactement
`if stock > 0 then b else a`. Utilisez celui qui se lit le mieux.

## Choisir avec match

Quand il y a beaucoup de cas, `match` est plus clair qu'une suite de
`else if`. Il compare une valeur à plusieurs **motifs**, dans l'ordre. Le
premier qui correspond donne le résultat. `_` correspond à tout le reste :

```rythme
fn jour(n) = match n
  6 -> "samedi"
  7 -> "dimanche"
  _ -> "un jour de semaine"
end

print(jour(6), jour(2))
```

```sortie
samedi un jour de semaine
```

Ligne par ligne :

- `fn jour(n) = match n` : la fonction compare le nombre `n` aux motifs qui suivent ;
- pour `jour(6)`, le premier motif `6` correspond : le résultat est `"samedi"` ;
- pour `jour(2)`, ni `6` ni `7` ne correspondent. `_` accepte tout : le résultat est
  `"un jour de semaine"` ;
- `print` affiche les deux résultats, séparés par une espace.

Chaque ligne s'écrit `motif -> résultat`, et `end` ferme le `match`.
Pensez à un tri de courrier : chaque lettre va dans la première case qui
lui convient.

Un motif peut aussi être un intervalle de nombres (`16..20`, les deux bouts
compris), ou plusieurs possibilités séparées par `|` :

```rythme
fn mention(note) = match note
  16..20 -> "très bien"
  12..15 -> "bien"
  10 | 11 -> "passable"
  _ -> "à retravailler"
end

print(mention(17), mention(13), mention(10), mention(8))
```

```sortie
très bien bien passable à retravailler
```

Un motif peut même reconnaître le début d'un texte :

```rythme
fn saluer("Dr " + nom) = "Bonjour docteur {nom}"
fn saluer(nom) = "Bonjour {nom}"

print(saluer("Dr Grace"), "/", saluer("Ada"))
```

```sortie
Bonjour docteur Grace / Bonjour Ada
```

`match` sait faire beaucoup plus : voir [vos propres types et match](types.md).

**À vous :** écrivez une fonction `saison(mois)` qui donne « hiver » pour les mois 12,
1 et 2, « printemps » de 3 à 5, « été » de 6 à 8, et « automne » sinon. Affichez
`saison(1)`, `saison(7)` et `saison(10)`.

> **Solution**
>
> ```rythme
> fn saison(mois) = match mois
>   12 | 1 | 2 -> "hiver"
>   3..5 -> "printemps"
>   6..8 -> "été"
>   _ -> "automne"
> end
>
> print(saison(1), saison(7), saison(10))
> ```
>
> ```sortie
> hiver été automne
> ```

## Erreurs fréquentes

### Oublier le else

```rythme norun
age = 20
print(if age >= 18 then "majeur")
```

```text
erreur[E019]: ce `if` n'a pas de `else`
 --> main.ry:2:7
  |
2 | print(if age >= 18 then "majeur")
  |       ^^ chaque `if` doit avoir un `else`
  |
2 | print(if age >= 18 then "majeur")
  |                                 - `else` attendu ici
  |
  = aide: en Rythme, `if` donne toujours une valeur, il lui faut donc les deux cas :
        if age >= 18 then "majeur" else "mineur"
  = en savoir plus : `rythme explain E019`
```

Ajoutez l'autre cas : `else "mineur"`, ou `else ()` s'il n'y a rien à faire.

### Comparer avec = au lieu de ==

```rythme norun
x = 5
if x = 5 then print("cinq") else ()
```

```text
erreur[E001]: `then` attendu, mais trouvé `=`
 --> main.ry:2:6
  |
2 | if x = 5 then print("cinq") else ()
  |      ^ `then` attendu ici
  |
  = aide: pour comparer deux valeurs, utilisez `==`
  = aide: un `if` s'écrit : `if condition then valeur else autre_valeur`
  = en savoir plus : `rythme explain E001`
```

Pour comparer, deux signes égal : `if x == 5 then ...`.

### Enchaîner deux comparaisons

```rythme norun
note = 15
print(if 10 <= note <= 20 then "ok" else "non")
```

```text
erreur[E001]: les comparaisons ne peuvent pas s'enchaîner
 --> main.ry:2:21
  |
2 | print(if 10 <= note <= 20 then "ok" else "non")
  |                     ^^
  |
  = aide: combinez-les avec `and` : `a < b and b < c`
  = en savoir plus : `rythme explain E001`
```

Écrivez les deux questions et reliez-les : `10 <= note and note <= 20`.
`&&` et `||` n'existent pas non plus : écrivez `and` et `or`.

## Testez-vous

**Question 1 :** que vaut `unless 3 > 5 then "a" else "b"` ?

> **Réponse**
>
> `"a"`. La condition `3 > 5` est fausse, et `unless` donne la première
> valeur quand la condition est fausse.

**Question 2 :** qu'affiche ce programme ?

```rythme norun
fn taille(n) = match n
  0 -> "rien"
  1..9 -> "petit"
  _ -> "grand"
end
print(taille(0), taille(7), taille(42))
```

> **Réponse**
>
> `rien petit grand`. Chaque nombre va au premier motif qui lui convient.

**Question 3 :** que vaut `if 4 >= 4 and not (4 == 5) then "oui" else "non"` ?

> **Réponse**
>
> `"oui"`. `4 >= 4` est vrai ; `4 == 5` est faux, donc `not (4 == 5)` est vrai.
> Les deux côtés du `and` sont vrais : le `if` donne la valeur qui suit `then`.

## En résumé

- `==`, `!=`, `<`, `>`, `<=`, `>=` comparent ; `and`, `or` et `not`
  combinent les conditions.
- `if condition then a else b` donne une valeur, et le `else` est
  obligatoire ; `unless` est son contraire.
- Avec `else if`, les conditions sont essayées de haut en bas : la première
  vraie gagne.
- `match` compare une valeur à des motifs, dans l'ordre ; `_` attrape tout
  le reste.

Suite : [les boucles](boucles.md). Comparez avec les conditions de
[Ruby](../ruby/conditions-boucles.md) et de [Rust](../rust/conditions-boucles.md).
