# Le filtrage par motif

Un motif décrit la forme d'une valeur : cette page passe en revue toutes les sortes de motifs de Rythme, dans `match` comme dans les paramètres des fonctions.

> **Objectifs :**
>
> - vous saurez reconnaître un nombre, un intervalle, un texte ou le début d'un texte ;
> - vous saurez découper une liste ou un enregistrement avec un motif ;
> - vous saurez combiner les motifs avec `|`, des gardes et des motifs imbriqués ;
> - vous saurez lire les avertissements W002 et W004 du vérificateur ;
> - vous saurez utiliser des motifs dans les paramètres d'une fonction.

## Rappel : match

[`match`](../debuter/glossaire.md#match) compare une valeur à une suite de
**motifs**, dans l'ordre. Le premier qui correspond donne le résultat. C'est
comme un trieur de pièces de monnaie : chaque pièce tombe dans le premier
trou à sa taille.

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

Chaque cas s'écrit `motif -> résultat`, et `end` ferme le `match`. Un motif
ne calcule rien : il décrit une **forme**, et donne des noms aux morceaux.

## Les valeurs exactes

Le motif le plus simple est une valeur écrite telle quelle : un nombre, un
texte, `true` ou `false`, `()`, un symbole. `_` correspond à tout, sans
donner de nom.

```rythme
fn reponse(x) = match x
  0 -> "zéro"
  2.5 -> "deux et demi"
  "oui" -> "d'accord"
  true -> "vrai"
  () -> "rien"
  :ok -> "le symbole ok"
  _ -> "autre chose"
end

print([0, 0.0, 2.5, "oui", true, (), :ok, 99] |> map(reponse))
```

```sortie
["zéro", "zéro", "deux et demi", "d'accord", "vrai", "rien", "le symbole ok", "autre chose"]
```

Le motif `0` accepte aussi `0.0` : les nombres égaux se valent.

Un **nom** correspond aussi à tout, mais il garde la valeur :

```rythme
fn decrire(x) = match x
  0 -> "rien"
  n -> "le nombre {n}"
end

print(decrire(0), "/", decrire(42))
```

```sortie
rien / le nombre 42
```

## Les intervalles

`a..b` correspond à un nombre de `a` à `b`, **les deux bouts compris**. Les
bornes sont des nombres écrits dans le code.

```rythme
fn mention(note) = match note
  16..20 -> "très bien"
  12..15 -> "bien"
  10..11 -> "passable"
  0..9 -> "insuffisant"
  _ -> "pas une note"
end

print([17, 12, 10, 3, 25] |> map(mention))
```

```sortie
["très bien", "bien", "passable", "insuffisant", "pas une note"]
```

Un intervalle accepte aussi les Floats : `15.5` n'est dans aucun de ces
intervalles, il irait donc dans `_`. Le dernier cas `_` est indispensable,
car aucun intervalle ne couvre tous les nombres.

## Le début d'un texte

`"M. " + nom` correspond à un texte qui **commence** par `"M. "`, et nomme
la suite `nom`. C'est très pratique pour lire des commandes :

```rythme
fn commande("/aide") = "voici l'aide"
fn commande("/dire " + message) = "vous dites : {message}"
fn commande("/" + _) = "commande inconnue"
fn commande(texte) = "message : {texte}"

print(commande("/aide"))
print(commande("/dire bonjour"))
print(commande("/danse"))
print(commande("salut"))
```

```sortie
voici l'aide
vous dites : bonjour
commande inconnue
message : salut
```

Seul le **début** d'un texte peut servir de motif. Pour la fin, utilisez une
garde (voir plus bas) avec `ends_with`.

## Les listes

Un motif de liste décrit sa longueur et ses éléments :

| Motif | Correspond à |
|-------|--------------|
| `[]` | la liste vide |
| `[x]` | une liste d'exactement un élément |
| `[x, y]` | une liste d'exactement deux éléments |
| `[premier, ..reste]` | une liste d'au moins un élément ; `reste` est la suite |
| `[a, b, ..]` | au moins deux éléments ; la suite n'a pas de nom |

```rythme
fn decrire(liste) = match liste
  [] -> "vide"
  [x] -> "un seul : {x}"
  [x, y] -> "deux : {x} et {y}"
  [premier, ..reste] -> "{premier} puis {len(reste)} autres"
end

print(decrire([]), "/", decrire([7]), "/", decrire([7, 8]), "/", decrire([1, 2, 3, 4]))
```

```sortie
vide / un seul : 7 / deux : 7 et 8 / 1 puis 3 autres
```

`..reste` doit venir en dernier. C'est ce motif qui fait marcher
[la récursion](recursion.md) sur les listes.

## Les enregistrements

`{nom}` correspond à un enregistrement qui a **au moins** un champ `nom`, et
donne ce champ sous le nom `nom`. `{age: 0}` exige que le champ `age` vaille
0. L'enregistrement peut avoir d'autres champs.

```rythme
fn fiche(p) = match p
  {nom, age: 0} -> "{nom} vient de naître"
  {nom, ville: "Paris"} -> "{nom} habite Paris"
  {nom, age} -> "{nom} a {age} ans"
  {} -> "pas une personne"
end

print(fiche({nom: "Léo", age: 0}))
print(fiche({nom: "Ada", age: 36, ville: "Paris"}))
print(fiche({nom: "Alan", age: 41}))
print(fiche({titre: "Livre"}))
```

```sortie
Léo vient de naître
Ada habite Paris
Alan a 41 ans
pas une personne
```

`{}` correspond à n'importe quel enregistrement.

## Les constructeurs

Les variantes d'un type (voir [vos propres types](types.md)), et `Some`,
`None`, `Ok`, `Err`, se reconnaissent par leur nom. Les champs sont à
leur tour des motifs :

```rythme
type Forme = Cercle(rayon) | Rect(largeur, hauteur) | Point

fn aire(f) = match f
  Cercle(r) -> round(pi * r ** 2, 2)
  Rect(l, h) -> l * h
  Point -> 0
end

print(map([Cercle(1), Rect(2, 3), Point], aire))
```

```sortie
[3.14, 6, 0]
```

## Plusieurs possibilités avec |

`m1 | m2` correspond si l'un des deux motifs correspond :

```rythme
fn genre(jour) = match jour
  "samedi" | "dimanche" -> "week-end"
  _ -> "semaine"
end

print(genre("samedi"), genre("lundi"))
```

```sortie
week-end semaine
```

Les motifs reliés par `|` ne peuvent pas donner de noms (voir les erreurs
fréquentes).

## Les gardes

Une **garde** est une condition ajoutée après le motif avec `if`. Le cas
n'est choisi que si le motif correspond **et** que la condition vaut
`true`. Les noms du motif sont utilisables dans la garde.

```rythme
type Forme = Cercle(rayon) | Rect(largeur, hauteur) | Point

fn nature(f) = match f
  Rect(l, h) if l == h -> "un carré de côté {l}"
  Rect(l, h) -> "un rectangle {l} × {h}"
  Cercle(_) | Point -> "rond ou point"
end

print(nature(Rect(2, 2)), "/", nature(Rect(2, 5)), "/", nature(Point))
```

```sortie
un carré de côté 2 / un rectangle 2 × 5 / rond ou point
```

Une garde sert aussi pour la fin d'un texte, ou pour comparer avec une
valeur qui n'est pas écrite dans le code :

```rythme
fn ton(phrase) = match phrase
  t if ends_with(t, "?") -> "question"
  t if ends_with(t, "!") -> "exclamation"
  _ -> "affirmation"
end

print(ton("Vraiment ?"), ton("Bravo !"), ton("Il pleut."))
```

```sortie
question exclamation affirmation
```

## Des motifs imbriqués

Les motifs s'emboîtent comme des boîtes dans des boîtes : un enregistrement
qui contient un enregistrement et une liste, une liste de `Result`...

```rythme
fn resume(commande) = match commande
  {client: {nom}, articles: [premier, ..]} -> "{nom} commande d'abord {premier}"
  {client: {nom}, articles: []} -> "{nom} n'a rien commandé"
  _ -> "commande illisible"
end

print(resume({client: {nom: "Ada", ville: "Londres"}, articles: ["thé", "scones"]}))
print(resume({client: {nom: "Alan"}, articles: []}))

fn premier([Ok(v), ..]) = "premier résultat : {v}"
fn premier([Err(e), ..]) = "erreur : {e}"
fn premier([]) = "rien"
print(premier([Ok(3), Err("x")]), "/", premier([Err("oups")]))
```

```sortie
Ada commande d'abord thé
Alan n'a rien commandé
premier résultat : 3 / erreur : oups
```

## Des motifs dans les paramètres

Les paramètres d'une fonction sont, eux aussi, des motifs. Une fonction
définie par cas est donc un `match` caché :

```rythme
fn distance({x: x1, y: y1}, {x: x2, y: y2}) = sqrt((x2 - x1) ** 2 + (y2 - y1) ** 2)
print(distance({x: 0, y: 0}, {x: 3, y: 4}))

fn tete([x, ..]) = Some(x)
fn tete([]) = None
print(tete([5, 6]), tete([]))
```

```sortie
5.0
Some(5) None
```

Les motifs servent enfin à nommer plusieurs morceaux d'un coup :

```rythme
[a, b, ..] = [10, 20, 30]
{nom} = {nom: "Ada", age: 36}
print(a, b, nom)
```

```sortie
10 20 Ada
```

## Le vérificateur vous aide

Avant de lancer le programme, Rythme relit vos `match`. Il signale deux
oublis courants.

**W002 : un cas oublié.** Ici, le feu orange n'est pas traité :

```rythme norun
type Feu = Rouge | Orange | Vert
fn action(feu) = match feu
  Rouge -> "stop"
  Vert -> "passez"
end
print(action(Vert))
```

```text
avertissement[W002]: ce `match` ne traite pas tous les cas
 --> main.ry:2:18
  |
2 | fn action(feu) = match feu
  |                  ^^^^^^^^^
  |
  = note: non traité : `Orange`
  = aide: ajoutez les cas manquants, comme `Orange -> ...`, ou un dernier cas `_ -> ...`

passez
```

Le programme s'exécute quand même, mais `action(Orange)` l'arrêterait avec
l'erreur E006. Ajoutez le cas `Orange -> "ralentissez"`.

**W004 : un cas jamais atteint.** Les cas sont essayés dans l'ordre : un
`_` placé en premier prend tout.

```rythme norun
fn taille(n) = match n
  _ -> "inconnue"
  0 -> "nulle"
end
print(taille(0))
```

```text
avertissement[W004]: ce cas ne peut jamais être atteint
 --> main.ry:3:3
  |
3 |   0 -> "nulle"
  |   ^
  |
2 |   _ -> "inconnue"
  |   - ce cas traite déjà tout
  |
  = aide: les cas sont essayés dans l'ordre : mettez `_` en dernier

inconnue
```

Mettez les cas les plus précis en haut, et `_` tout en bas.

> **À retenir :** un avertissement n'arrête pas le programme, mais il
> signale presque toujours un vrai problème. Lisez-les.

## Erreurs fréquentes

### Utiliser deux fois le même nom

```rythme norun
fn egaux([a, a]) = true
fn egaux(_) = false
print(egaux([1, 1]))
```

```text
erreur[E011]: `a` apparaît deux fois dans ce motif
 --> main.ry:1:14
  |
1 | fn egaux([a, a]) = true
  |              ^
  |
1 | fn egaux([a, a]) = true
  |           - une première fois ici
  |
  = aide: utilisez deux noms différents, puis comparez-les avec `==` dans une condition : `[a, b] if a == b -> ...`
  = en savoir plus : `rythme explain E011`
```

Un motif ne compare pas deux morceaux entre eux. Écrivez
`fn egaux([a, b]) if a == b = true`.

### Reconnaître la fin d'un texte

```rythme norun
fn ton(t) = match t
  _ + "!" -> "exclamation"
  _ -> "autre"
end
print(ton("oh!"))
```

```text
erreur[E001]: seul un texte peut venir avant `+` dans un motif
 --> main.ry:2:3
  |
2 |   _ + "!" -> "exclamation"
  |   ^^^
  |
  = aide: un texte qui commence par un texte s'écrit `"M. " + nom` : `nom` est le reste du texte
  = aide: pour regarder la fin d'un texte, utilisez une condition : `s if ends_with(s, "!") -> ...`
  = en savoir plus : `rythme explain E001`
```

Seul le début d'un texte se reconnaît. Pour la fin : `t if ends_with(t, "!") -> ...`.

### Donner un nom dans un motif avec |

```rythme norun
fn debut(liste) = match liste
  [x] | [x, _] -> x
  _ -> 0
end
print(debut([4, 5]))
```

```text
erreur[E001]: des motifs reliés par `|` ne peuvent pas donner de noms (ici `x`)
 --> main.ry:2:4
  |
2 |   [x] | [x, _] -> x
  |    ^
  |
  = aide: écrivez plutôt des cas séparés
  = en savoir plus : `rythme explain E001`
```

Écrivez deux cas : `[x] -> x` puis `[x, _] -> x`.

## Testez-vous

**Question 1 :** quelles valeurs acceptent les motifs `[x]`, `[x, ..]` et
`[x, ..reste]` ?

> **Réponse**
>
> `[x]` : une liste d'exactement un élément. `[x, ..]` et `[x, ..reste]` :
> une liste d'au moins un élément ; le second nomme aussi la suite.

**Question 2 :** que donne `classer(15)` ?

```rythme norun
fn classer(n) = match n
  0..9 -> "petit"
  10..99 if n % 5 == 0 -> "rond"
  10..99 -> "moyen"
  _ -> "grand"
end
```

> **Réponse**
>
> `"rond"` : 15 est dans `10..99`, et la garde `15 % 5 == 0` vaut `true`.

**Question 3 :** que signale l'avertissement W004 ?

> **Réponse**
>
> Un cas qui ne peut jamais être choisi, parce qu'un cas placé avant lui
> accepte déjà les mêmes valeurs (souvent un `_` mis trop tôt).

## En résumé

- Un motif décrit une forme : valeur exacte, intervalle `a..b`, début de
  texte `"M. " + nom`, liste, enregistrement, constructeur.
- `[x, ..reste]` découpe une liste ; `{nom}` prend un champ ; `_` accepte
  tout sans nom.
- `|` offre plusieurs possibilités (sans noms) ; `if` ajoute une garde.
- Les motifs s'imbriquent, et servent aussi dans les paramètres des
  fonctions et dans `[a, b] = liste`.
- W002 signale un cas oublié, W004 un cas jamais atteint.

Suite : [Gérer les erreurs](erreurs.md)
