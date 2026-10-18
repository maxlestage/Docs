# Les expressions régulières

Une expression régulière (une « regex ») décrit la forme d'un morceau de texte : cette page montre comment chercher, extraire, découper et remplacer du texte avec elles.

> **Objectifs :**
>
> - vous saurez lire et écrire une regex simple : chiffres, lettres, répétitions ;
> - vous saurez tester, chercher et extraire avec `matches`, `find_first`, `find_all` et `captures` ;
> - vous saurez découper avec `split_by` et remplacer avec `replace_all` ;
> - vous saurez écrire une regex dans un texte Rythme : `\\` et `\{ \}` ;
> - vous saurez lire l'erreur E026 d'une regex mal écrite.

## Qu'est-ce qu'une regex ?

`contains(texte, "2024")` cherche exactement `2024`. Mais comment chercher
« n'importe quelle année », ou « un code postal » ? Il faut décrire une
**forme** : « quatre chiffres », « cinq chiffres ».

Une regex est un petit texte qui décrit cette forme. C'est comme un
avis de recherche : « un homme, grand, avec un chapeau ». On ne connaît pas
la personne, mais on sait la reconnaître.

```rythme
print(find_all("Nés en 1815, 1912 et 1906.", "[0-9]+"))
```

```sortie
["1815", "1912", "1906"]
```

`[0-9]` veut dire « un chiffre de 0 à 9 », et `+` « une fois ou plus ».
Ensemble : « une suite de chiffres ».

## Les pièces d'une regex

Une regex s'assemble avec quelques pièces. Voici les plus utiles, avec la
façon de les écrire dans un texte Rythme (voir la section suivante pour les
`\\`) :

| Pièce | Dans un texte Rythme | Correspond à |
|-------|----------------------|--------------|
| `abc` | `"abc"` | exactement ces caractères |
| `.` | `"."` | n'importe quel caractère sauf un retour à la ligne |
| `[abc]` | `"[abc]"` | un caractère parmi `a`, `b`, `c` |
| `[a-z]` | `"[a-z]"` | un caractère de `a` à `z` |
| `[^0-9]` | `"[^0-9]"` | un caractère qui n'est **pas** un chiffre |
| `\d` | `"\\d"` | un chiffre, de 0 à 9 |
| `\w` | `"\\w"` | une lettre (accents compris), un chiffre ou `_` |
| `\s` | `"\\s"` | une espace, une tabulation ou un retour à la ligne |
| `\D`, `\W`, `\S` | `"\\D"`... | le contraire de `\d`, `\w`, `\s` |
| `\b` | `"\\b"` | le bord d'un mot |
| `^`, `$` | `"^"`, `"$"` | le début et la fin du texte |
| `\.` | `"\\."` | le signe `.` lui-même (de même `\(`, `\+`...) |
| `a\|b` | `"a\|b"` | `a` ou `b` |
| `(...)` | `"(...)"` | un groupe, dont le texte est capturé |
| `(?:...)` | `"(?:...)"` | un groupe sans capture |
| `(?<nom>...)` | `"(?<nom>...)"` | un groupe capturé, avec un nom |
| `*` | `"*"` | zéro fois ou plus ce qui précède |
| `+` | `"+"` | une fois ou plus |
| `?` | `"?"` | zéro ou une fois |
| `{4}` | `"\{4\}"` | exactement 4 fois |
| `{2,}`, `{2,5}` | `"\{2,\}"`, `"\{2,5\}"` | au moins 2 fois ; de 2 à 5 fois |
| `*?`, `+?` | `"*?"`, `"+?"` | le moins de fois possible |
| `(?i)` | `"(?i)"` | au début : ignorer majuscules et minuscules |

Pas besoin de tout retenir : gardez ce tableau sous la main.

## Écrire une regex dans un texte Rythme

Une regex est donnée à Rythme sous forme de texte. Deux signes ont déjà un
sens dans un texte Rythme, il faut donc les **doubler** ou les protéger :

- la barre oblique inverse `\` s'écrit `\\` : la regex `\d` s'écrit
  `"\\d"` ;
- les accolades s'écrivent `\{` et `\}`, car `{...}` insère une valeur : la
  regex `\d{4}` s'écrit `"\\d\{4\}"`.

```rythme
print(find_all("Nés en 1815 et 1912, code 75001.", "\\d\{4\}"))
print(find_all("Nés en 1815 et 1912, code 75001.", "\\b\\d\{4\}\\b"))
```

```sortie
["1815", "1912", "7500"]
["1815", "1912"]
```

La première regex trouve aussi `7500` dans `75001`. Avec `\b` (le bord d'un
mot) de chaque côté, on ne garde que les nombres d'exactement 4 chiffres.

> **Astuce :** entre crochets, un signe n'a pas besoin de barre oblique :
> `"[.]"` est un point, `"[(]"` une parenthèse.

## Tester un texte avec matches

`matches(texte, regex)` dit si le texte **entier** a la forme de la regex :

```rythme
print(matches("75001", "\\d\{5\}"), matches("7500", "\\d\{5\}"))
print(matches("code 75001", "\\d\{5\}"))
print(matches("06 12 34 56 78", "0\\d( \\d\{2\})\{4\}"))
```

```sortie
true false
false
true
```

`"code 75001"` contient un code postal, mais n'en **est** pas un : le texte
entier doit correspondre. C'est parfait pour vérifier ce qu'un utilisateur
a tapé.

## Chercher avec find_first et find_all

`find_first` donne la première correspondance, dans une
[Option](../debuter/glossaire.md#option). `find_all` donne la liste de
toutes les correspondances, de gauche à droite :

```rythme
print(find_first("Rendez-vous à 14h30", "\\d+"))
print(find_first("Pas de chiffre", "\\d+"))
print(find_all("3 chats, 12 chiens et 1 poisson", "\\d+"))
```

```sortie
Some("14")
None
["3", "12", "1"]
```

## Extraire des morceaux avec captures

Les parenthèses forment des **groupes**. `captures` donne la
correspondance entière, puis le texte de chaque groupe :

```rythme
print(captures("version 2.14", "(\\d+)\\.(\\d+)"))
match captures("version 2.14", "(\\d+)\\.(\\d+)")
  Some([_, majeure, mineure]) -> print("majeure {majeure}, mineure {mineure}")
  _ -> print("pas de version")
end
```

```sortie
Some(["2.14", "2", "14"])
majeure 2, mineure 14
```

Le motif `[_, majeure, mineure]` (voir [le filtrage par motif](pattern-matching.md))
ignore la correspondance entière et nomme les deux groupes.

Avec beaucoup de groupes, donnez-leur des noms : `(?<nom>...)`.
`named_captures` donne alors un enregistrement :

```rythme
date = named_captures("2024-02-29", "(?<annee>\\d\{4\})-(?<mois>\\d\{2\})-(?<jour>\\d\{2\})")
print(date)
match date
  Some({jour, mois}) -> print("le {jour}/{mois}")
  None -> print("pas une date")
end
```

```sortie
Some({annee: "2024", mois: "02", jour: "29"})
le 29/02
```

Les morceaux capturés sont des textes : utilisez `int` pour en faire des
nombres.

## Découper avec split_by

`split(texte, ",")` coupe à un séparateur fixe. `split_by` coupe partout où
la regex correspond, ce qui permet des séparateurs variés :

```rythme
print(split_by("pomme, poire;figue  kiwi", "[,; ]+"))
```

```sortie
["pomme", "poire", "figue", "kiwi"]
```

Virgules, points-virgules, une ou plusieurs espaces : tout sépare.

## Remplacer avec replace_all

`replace_all(texte, regex, nouveau)` remplace chaque correspondance. Dans
`nouveau`, `$1`, `$2`... désignent le texte des groupes, `$<nom>` un groupe
nommé, et `$$` un vrai `$` :

```rythme
print(replace_all("29/02/2024", "(\\d+)/(\\d+)/(\\d+)", "$3-$2-$1"))
print(replace_all("Ada Lovelace", "(?<prenom>\\w+) (?<nom>\\w+)", "$<nom>, $<prenom>"))
print(replace_all("a   b    c", "\\s+", " "))
```

```sortie
2024-02-29
Lovelace, Ada
a b c
```

`nouveau` peut aussi être une **fonction** : elle reçoit chaque
correspondance et renvoie le texte qui la remplace.

```rythme
print(replace_all("bonjour le monde", "\\b\\w", upper))
print(replace_all("prix : 10 € et 25 €", "\\d+", fn(n) = text(int(n) * 2)))
```

```sortie
Bonjour Le Monde
prix : 20 € et 50 €
```

`\b\w` est la première lettre de chaque mot ; `upper` la met en majuscule.

## Ignorer les majuscules avec (?i)

Placé au tout début, `(?i)` rend la regex indifférente aux majuscules :

```rythme
print(matches("Bonjour", "bonjour"), matches("Bonjour", "(?i)bonjour"))
print(find_all("OUI, oui, Oui !", "(?i)oui"))
```

```sortie
false true
["OUI", "oui", "Oui"]
```

## Prendre le moins possible

`+` et `*` prennent **le plus** de caractères possible. Ajoutez `?` pour
prendre le moins possible :

```rythme
print(find_all("x<b>y</b>", "<.+>"))
print(find_all("x<b>y</b>", "<.+?>"))
```

```sortie
["<b>y</b>"]
["<b>", "</b>"]
```

## Une regex mal écrite : E026

Une parenthèse non fermée, une répétition sans rien devant... Rythme
vérifie la regex. Quand elle est écrite dans l'appel, l'erreur est trouvée
**avant** de lancer le programme, et montrée à sa place :

```rythme norun
code = "AB-12"
print(matches(code, "[A-Z]+-([0-9]+"))
```

```text
erreur[E026]: regex invalide : cette `(` n'est jamais fermée
 --> main.ry:2:29
  |
2 | print(matches(code, "[A-Z]+-([0-9]+"))
  |                             ^ ouverte ici
  |
  = aide: ajoutez une `)` là où le groupe finit ; pour le signe `(` lui-même, écrivez `[(]`
  = en savoir plus : `rythme explain E026`
```

Le message dit quoi corriger : fermer le groupe, ou écrire `[(]` pour une
vraie parenthèse.

## Erreurs fréquentes

### Une seule barre oblique inverse

```rythme norun
print(find_all("3 chats et 12 chiens", "\d+"))
```

```text
erreur[E018]: séquence d'échappement inconnue dans le texte
 --> main.ry:1:41
  |
1 | print(find_all("3 chats et 12 chiens", "\d+"))
  |                                         ^^
  |
  = aide: dans une regex, écrivez la barre oblique inverse deux fois : "\\d" est le texte \d
  = aide: échappements valides : \n (nouvelle ligne), \t (tabulation), \r (retour chariot), \0 (caractère nul), \\ (barre oblique),
      \" (guillemet), \{ et \} (accolades), \# (un `#` devant une accolade), \u{e9} (un caractère par son numéro Unicode : é)
  = en savoir plus : `rythme explain E018`
```

Doublez la barre : `"\\d+"`.

### Oublier de protéger les accolades

```rythme
print(matches("2024", "\\d{4}"))
```

```sortie
false
```

Aucun message, mais un résultat faux ! `{4}` a **inséré** la valeur 4 dans
le texte : la regex est devenue `\d4`, « un chiffre puis un 4 ». Écrivez
`"\\d\{4\}"`.

### Utiliser matches pour chercher

```rythme norun
print(matches("prix : 12 €", "\\d+"))
```

```text
false
```

`matches` teste le texte **entier**, qui n'est pas fait que de chiffres.
Pour savoir si le texte **contient** un nombre, utilisez
`is_some(find_first(texte, "\\d+"))`.

## Testez-vous

**Question 1 :** que donne `find_all("a1b22c333", "\\d+")` ?

> **Réponse**
>
> `["1", "22", "333"]` : chaque suite de chiffres, de gauche à droite.

**Question 2 :** comment écrire, dans un texte Rythme, la regex qui
reconnaît exactement trois lettres minuscules, `[a-z]{3}` ?

> **Réponse**
>
> `"[a-z]\{3\}"` : les accolades doivent être protégées par `\`.

**Question 3 :** que donne `replace_all("Dupont Jean", "(\\w+) (\\w+)", "$2 $1")` ?

> **Réponse**
>
> `"Jean Dupont"` : `$2` est le second groupe, `$1` le premier.

## En résumé

- Une regex décrit une forme de texte : `\d` un chiffre, `[a-z]` une
  lettre, `+` une fois ou plus, `\{4\}` quatre fois, `(...)` un groupe.
- Dans un texte Rythme, `\` s'écrit `\\`, et les accolades `\{` et `\}`.
- `matches` teste le texte entier ; `find_first`, `find_all`, `captures` et
  `named_captures` cherchent et extraient.
- `split_by` découpe, `replace_all` remplace avec `$1` ou une fonction ;
  `(?i)` ignore les majuscules.
- Une regex mal écrite donne l'erreur E026, qui montre où est le problème.

Suite : [dates et heures](dates.md).
