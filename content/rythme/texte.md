# Travailler avec du texte

Cette page montre les outils de Rythme pour les textes : insérer des valeurs, changer les majuscules, découper, chercher, remplacer, aligner des colonnes, lire un nombre, et une première rencontre avec les expressions régulières.

> **Objectifs :**
>
> - vous saurez construire un texte avec des valeurs insérées et des caractères spéciaux ;
> - vous saurez transformer un texte : majuscules, espaces, remplacement ;
> - vous saurez découper un texte en morceaux et recoller une liste ;
> - vous saurez lire un nombre tapé sous forme de texte, sans planter ;
> - vous saurez chercher des morceaux de texte avec une expression régulière simple.

## Insérer une valeur dans un texte

Un texte, qu'on appelle aussi une
[chaîne de caractères](../debuter/glossaire.md#chaine-de-caracteres), est
une suite de caractères : lettres, chiffres, espaces, signes.

Vous le savez depuis les [premiers pas](premiers-pas.md) : une valeur entre
accolades `{...}` est insérée dans le texte. On peut y mettre un calcul ou un
appel de fonction :

```rythme
nom = "Ada"
fruits = ["pomme", "poire"]
print("Bonjour {nom} !")
print("{nom} a {len(fruits)} fruits.")
print("Le double de 21 est {21 * 2}.")
```

```sortie
Bonjour Ada !
Ada a 2 fruits.
Le double de 21 est 42.
```

> **Astuce :** pour transformer n'importe quelle valeur en texte, utilisez
> `text` : `text(42)` vaut `"42"`.

## Les caractères spéciaux

Certains caractères ne se tapent pas directement dans un texte. On les écrit
avec une barre oblique inverse `\` devant, une **séquence d'échappement** :

| Écrire | Pour obtenir |
|--------|--------------|
| `\n` | un retour à la ligne |
| `\t` | une tabulation (un grand espace, pour aligner) |
| `\"` | un guillemet, sans fermer le texte |
| `\\` | une barre oblique inverse |
| `\{` et `\}` | des accolades, sans insertion |

```rythme
print("Elle a dit : \"oui\".")
print("ligne 1\nligne 2")
print("Des accolades : \{ et \}")
```

```sortie
Elle a dit : "oui".
ligne 1
ligne 2
Des accolades : { et }
```

## Longueur et majuscules

`len` compte les caractères. `upper` met tout en majuscules, `lower` tout en
minuscules, et `capitalize` seulement la première lettre :

```rythme
mot = "élise"
print(len(mot))
print(upper(mot), lower("BONJOUR"), capitalize(mot))
```

```sortie
5
ÉLISE bonjour Élise
```

Les accents sont comptés comme un seul caractère : `len("été")` vaut 3.

## Lire un caractère par sa position

Chaque caractère a une position, à partir de **0**. `mot[0]` est le premier
caractère, `mot[-1]` le dernier. `slice(mot, debut, fin)` prend un morceau,
de `debut` compris à `fin` non comprise :

```rythme
mot = "bonjour"
print(mot[0], mot[-1], slice(mot, 0, 3), slice(mot, 3, 7))
```

```sortie
b r bon jour
```

Comme une règle graduée : le morceau de 0 à 3 contient les caractères aux
positions 0, 1 et 2.

## Enlever les espaces avec trim

Un texte tapé par quelqu'un a souvent des espaces en trop au début ou à la
fin. `trim` les enlève :

```rythme
saisie = "   Ada   "
print("[{saisie}]")
print("[{trim(saisie)}]")
```

```sortie
[   Ada   ]
[Ada]
```

## Découper et recoller

`split` découpe un texte en une liste de morceaux. Sans séparateur, il coupe
aux espaces. `join` fait l'inverse : il recolle une liste avec un séparateur.

```rythme
phrase = "le chat dort"
mots = split(phrase)
print(mots, len(mots))

print(split("rouge;vert;bleu", ";"))
print(join(["rouge", "vert", "bleu"], ", "))
```

```sortie
["le", "chat", "dort"] 3
["rouge", "vert", "bleu"]
rouge, vert, bleu
```

Avec `|>`, on enchaîne : on découpe, on transforme chaque mot, on recolle.

```rythme
"le chat dort"
  |> split
  |> map(capitalize)
  |> join(" ")
  |> print
```

```sortie
Le Chat Dort
```

## Chercher dans un texte

Trois questions, trois réponses `true` ou `false` :

- `contains` : le texte contient-il ce morceau ?
- `starts_with` : commence-t-il par ce morceau ?
- `ends_with` : finit-il par ce morceau ?

```rythme
fichier = "vacances.jpg"
print(contains(fichier, "cance"))
print(starts_with(fichier, "vac"), ends_with(fichier, ".jpg"))
print(ends_with(fichier, ".png"))
```

```sortie
true
true true
false
```

> **Attention :** les majuscules comptent : `contains("Bonjour", "bon")`
> vaut `false`. Passez d'abord le texte en minuscules avec `lower` si besoin.

Pour comparer deux textes entiers, `==` marche aussi, et les majuscules
comptent de même. `equals_ignore_case` compare sans en tenir compte :

```rythme
print("abc" == "abc", "Abc" == "abc", equals_ignore_case("Été", "ÉTÉ"))
```

```sortie
true false true
```

## Remplacer un morceau

`replace(texte, ancien, nouveau)` remplace **chaque** apparition de `ancien` :

```rythme
print(replace("le chat et le chien", "le", "un"))
print(replace("2024-05-17", "-", "/"))
```

```sortie
un chat et un chien
2024/05/17
```

Comme toujours en Rythme, le texte de départ ne change pas : `replace` donne
un nouveau texte.

## Les caractères et les lignes

`chars` donne la liste des caractères. `lines` découpe un texte en lignes
(`\n` veut dire « retour à la ligne ») :

```rythme
print(chars("abc"))

poeme = "Un\nDeux\nTrois"
print(lines(poeme))
print(len(lines(poeme)), "lignes")
```

```sortie
["a", "b", "c"]
["Un", "Deux", "Trois"]
3 lignes
```

Avec une liste de caractères, toutes les fonctions de listes marchent. Par
exemple, compter les voyelles :

```rythme
voyelles = chars("aeiouy")
nombre = chars("programmation") |> filter(fn(c) = contains(voyelles, c)) |> len
print(nombre)
```

```sortie
5
```

## Répéter et aligner

`repeat(texte, n)` répète un texte `n` fois. Pratique pour un trait :

```rythme
print(repeat("-", 20))
print(repeat("ha", 3))
```

```sortie
--------------------
hahaha
```

`pad_left(texte, largeur)` ajoute des espaces **à gauche** jusqu'à la largeur
voulue, et `pad_right` **à droite**. Cela aligne des colonnes :

```rythme
produits = [
  {nom: "pomme", prix: 3},
  {nom: "kiwi", prix: 12},
  {nom: "ananas", prix: 150},
]

produits.each do |p|
  print(pad_right(p.nom, 8) + pad_left(text(p.prix), 5) + " €")
end
```

```sortie
pomme       3 €
kiwi       12 €
ananas    150 €
```

Un troisième argument choisit le caractère de remplissage :

```rythme
print(pad_left("7", 3, "0"), pad_left("42", 3, "0"))
```

```sortie
007 042
```

## Lire un nombre dans un texte

Un texte comme `"42"` n'est pas un nombre : on ne peut pas faire de calcul
avec. C'est le cas de tout ce qu'un utilisateur tape au clavier. `parse_int` (nombre entier) et `parse_float` (nombre à virgule) le
lisent. Le texte peut ne pas être un nombre, alors ils donnent un `Result` :
`Ok(nombre)` ou `Err(message)`.

```rythme
print(parse_int("42"), parse_int(" 7 "))
print(parse_float("3.5"))
print(parse_int("douze"))
```

```sortie
Ok(42) Ok(7)
Ok(3.5)
Err("\"douze\" n'est pas un nombre entier")
```

Pour obtenir le nombre, traitez les deux cas avec `match`, ou donnez une
valeur par défaut avec `unwrap_or` :

```rythme
fn doubler(saisie) = match parse_int(saisie)
  Ok(n) -> "le double est {n * 2}"
  Err(_) -> "« {saisie} » n'est pas un nombre"
end

print(doubler("21"))
print(doubler("vingt"))
print(parse_int("??") |> unwrap_or(0))
```

```sortie
le double est 42
« vingt » n'est pas un nombre
0
```

`Result` est expliqué en détail dans [gérer les erreurs](erreurs.md).

> **Note :** `int("42")` lit aussi un nombre, mais arrête le programme
> (erreur E022) si le texte n'en est pas un. Gardez-le pour un texte écrit
> par vous ; pour une saisie, préférez `parse_int`.

## Écrire un grand nombre lisiblement

`format_number(nombre, decimales, langue)` sépare les milliers et arrondit.
Avec `"fr"`, il écrit le nombre à la française :

```rythme
print(format_number(1234567.891, 2, "fr"))
print(format_number(1234567.891, 2))
print(text(3.14159, 2))
```

```sortie
1 234 567,89
1,234,567.89
3.14
```

Sans langue, c'est l'écriture anglaise. Et `text(nombre, decimales)` donne
simplement un nombre avec ce nombre de décimales.

## Les expressions régulières

Une **expression régulière** (une « regex ») est un petit modèle qui décrit
des morceaux de texte. Quelques signes suffisent pour commencer :

| Signe | Veut dire |
|-------|-----------|
| `\d` | un chiffre, de 0 à 9 |
| `\w` | une lettre, un chiffre ou `_` |
| `[aeiou]` | un de ces caractères |
| `+` | le signe d'avant, une fois ou plus |

Ainsi, `\d+` veut dire « un ou plusieurs chiffres ».

> **Attention :** dans un texte Rythme, la barre oblique inverse s'écrit deux
> fois : la regex `\d+` s'écrit `"\\d+"`.

Trois fonctions pour commencer :

- `matches(texte, regex)` : le texte **entier** correspond-il au modèle ?
- `find_all(texte, regex)` : la liste de tous les morceaux qui correspondent ;
- `replace_all(texte, regex, nouveau)` : remplace tous ces morceaux.

```rythme
print(matches("2024", "\\d+"), matches("20h24", "\\d+"))
print(find_all("3 pommes et 12 poires", "\\d+"))
print(replace_all("Mon code : 1234", "\\d", "*"))
```

```sortie
true false
["3", "12"]
Mon code : ****
```

Par exemple, additionner tous les nombres d'une phrase :

```rythme
phrase = "J'ai 3 pommes, 12 poires et 5 kiwis."
total = find_all(phrase, "\\d+")
  |> map(fn(t) = parse_int(t) |> unwrap_or(0))
  |> sum
print(total)
```

```sortie
20
```

> **Note :** les regex peuvent faire beaucoup plus. `rythme help find_all`
> donne des exemples, et la référence de Rythme liste tous les signes.

## Exercices

### Exercice 1 : Initiales

Écrivez une fonction `initiales` qui donne les initiales d'un nom, en
majuscules : `initiales("ada lovelace")` donne `"AL"`.

> **Indice :** découpez avec `split`, prenez le premier caractère de chaque
> mot avec `m[0]`, puis recollez avec `join(liste, "")`.

> **Solution**
>
> ```rythme
> fn initiales(nom) = nom
>   |> split
>   |> map(fn(m) = upper(m[0]))
>   |> join("")
>
> print(initiales("ada lovelace"))
> print(initiales("grace brewster hopper"))
> ```
>
> ```sortie
> AL
> GBH
> ```

### Exercice 2 : Un cadre

Écrivez une fonction `cadre` qui affiche un mot entouré d'étoiles, comme
ci-dessous.

```text
**********
* Rythme *
**********
```

> **Indice :** la ligne d'étoiles a la longueur du mot plus 4. Utilisez
> `repeat` et `len`.

> **Solution**
>
> ```rythme
> fn cadre(mot)
>   ligne = repeat("*", len(mot) + 4)
>   print(ligne)
>   print("* {mot} *")
>   print(ligne)
> end
>
> cadre("Rythme")
> ```
>
> ```sortie
> **********
> * Rythme *
> **********
> ```

## Erreurs fréquentes

### Calculer directement avec parse_int

```rythme norun
saisie = "41"
print(parse_int(saisie) + 1)
```

```text
erreur[E005]: impossible d'utiliser `+` avec Result et Int
 --> main.ry:2:25
  |
2 | print(parse_int(saisie) + 1)
  |                         ^ Result + Int 1
  |
  = en savoir plus : `rythme explain E005`
```

`parse_int` ne donne pas un nombre, mais un `Result` : une boîte qui
contient soit le nombre, soit une erreur. Ouvrez-la d'abord :
`(parse_int(saisie) |> unwrap_or(0)) + 1`, ou avec `match`.

### Oublier que le texte ne change pas

```rythme norun
nom = "ada"
upper(nom)
print(nom)
```

```text
avertissement[W008]: le résultat de `upper` est perdu
 --> main.ry:2:1
  |
2 | upper(nom)
  | ^^^^^^^^^^ cette valeur est perdue
  |
  = aide: les valeurs ne changent jamais : `upper(text)` donne une nouvelle valeur, et ne modifie rien.
      Pour la garder, donnez-lui un nom : `nouveau = upper(text)`
```

C'est un avertissement : le programme tourne, mais affiche `ada`. `upper`
fabrique un **nouveau** texte. Gardez-le : `nom_maj = upper(nom)`.

### Une seule barre oblique dans une regex

```rythme norun
print(matches("2024", "\d+"))
```

```text
erreur[E018]: séquence d'échappement inconnue dans le texte
 --> main.ry:1:24
  |
1 | print(matches("2024", "\d+"))
  |                        ^^
  |
  = aide: dans une regex, écrivez la barre oblique inverse deux fois : "\\d" est le texte \d
  = aide: échappements valides : \n (nouvelle ligne), \t (tabulation), \r (retour chariot), \0 (caractère nul), \\ (barre oblique),
      \" (guillemet), \{ et \} (accolades), \# (un `#` devant une accolade), \u{e9} (un caractère par son numéro Unicode : é)
  = en savoir plus : `rythme explain E018`
```

Dans un texte, `\` commence un caractère spécial. Pour la regex `\d+`,
écrivez `"\\d+"`.

## Testez-vous

**Question 1 :** que vaut `"  Bonjour  " |> trim |> lower` ?

> **Réponse**
>
> `"bonjour"` : `trim` enlève les espaces autour, puis `lower` met en
> minuscules.

**Question 2 :** que donne `split("12/05/2024", "/")` ?

> **Réponse**
>
> La liste `["12", "05", "2024"]`. Ce sont encore des textes : pour
> calculer, lisez-les avec `parse_int`.

**Question 3 :** pourquoi `parse_int` donne-t-il un `Result`, et pas
directement un nombre ?

> **Réponse**
>
> Parce que le texte peut ne pas être un nombre (`"douze"`). Le `Result`
> vous oblige à prévoir ce cas : `Ok(nombre)` ou `Err(message)`.

## En résumé

- Une valeur entre accolades entre dans un texte : `"total : {x}"` ;
  `\n`, `\"` et `\{` écrivent des caractères spéciaux.
- `upper`, `lower`, `capitalize`, `trim` et `replace` donnent un nouveau
  texte ; `contains`, `starts_with` et `ends_with` répondent `true` ou
  `false`.
- `split` découpe un texte en liste, `join` recolle une liste ; `chars` et
  `lines` donnent les caractères et les lignes.
- `pad_left`, `pad_right` et `repeat` servent à aligner et décorer.
- `parse_int` et `parse_float` donnent un `Result` : le texte peut ne pas
  être un nombre.
- Une regex décrit des morceaux de texte : `matches`, `find_all` et
  `replace_all` s'en servent.

Suite : [les listes, enregistrements et dictionnaires](collections.md).
Comparez avec les textes en [Ruby](../ruby/texte.md) et en
[Rust](../rust/texte.md).
