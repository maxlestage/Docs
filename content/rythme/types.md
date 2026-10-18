# Vos propres types et match

Cette page montre comment créer vos propres types avec `type`, et comment les examiner avec `match` et ses motifs : listes, enregistrements, gardes et symboles.

> **Objectifs :**
>
> - vous saurez créer un type avec ses variantes, avec ou sans champs ;
> - vous saurez traiter chaque variante avec `match`, ou avec une définition par cas ;
> - vous saurez reconnaître des listes et des enregistrements avec des motifs ;
> - vous saurez ajouter une garde à un cas de `match` ;
> - vous saurez quand préférer un symbole ou un `type`.

## Créer un type

Un [type](../debuter/glossaire.md#type) est une sorte de valeur. Rythme en a
déjà (`Int`, `Text`...), et vous pouvez créer les vôtres.

`type` crée un nouveau type, avec la liste de ses **variantes**, les formes
possibles de ses valeurs, séparées par `|` (lisez « ou »). Les noms du type
et des variantes commencent par une majuscule :

```rythme
type Feu = Rouge | Orange | Vert

feu = Orange
print(feu, type_of(feu))
```

```sortie
Orange Feu
```

Lisez : « un `Feu`, c'est `Rouge`, ou `Orange`, ou `Vert` ». Un feu ne peut
pas être autre chose : pas de faute de frappe possible, comme avec un texte
`"orang"`.

Une variante peut porter des **champs** : elle s'utilise alors comme une
fonction qui fabrique une valeur. On lit un champ avec un point, comme dans
un enregistrement :

```rythme
type Forme = Cercle(rayon) | Rect(largeur, hauteur)

c = Cercle(2)
r = Rect(3, 4)
print(c, r, r.largeur)
```

```sortie
Cercle(2) Rect(3, 4) 3
```

Pensez à un formulaire à plusieurs modèles : le modèle « cercle » a une
case `rayon`, le modèle « rectangle » deux cases, `largeur` et `hauteur`.

## Examiner une valeur avec match

`match` reconnaît chaque variante, et donne un nom à ses champs :

```rythme
type Forme = Cercle(rayon) | Rect(largeur, hauteur)

fn aire(forme) = match forme
  Cercle(r) -> 3 * r * r
  Rect(l, h) -> l * h
end

print(aire(Cercle(2)), aire(Rect(3, 4)))
```

```sortie
12 12
```

Dans le cas `Rect(l, h)`, `l` reçoit la largeur et `h` la hauteur : vous
choisissez les noms.

On peut aussi écrire une définition par cas, sans `match` :

```rythme
type Forme = Cercle(rayon) | Rect(largeur, hauteur)

fn perimetre(Cercle(r)) = 2 * pi * r
fn perimetre(Rect(l, h)) = 2 * (l + h)

print(round(perimetre(Cercle(1)), 2), perimetre(Rect(3, 4)))
```

```sortie
6.28 14
```

`pi` est la constante 3.14159..., et `round(x, 2)` garde deux décimales.

## Le vérificateur repère les cas oubliés

Si vous oubliez une variante, Rythme vous prévient **avant** de lancer le
programme. C'est un avertissement : le programme tourne quand même.

```rythme norun
type Feu = Rouge | Orange | Vert

fn suivant(feu) = match feu
  Rouge -> Vert
  Vert -> Orange
end

print(suivant(Rouge))
```

`rythme check main.ry` :

```text
avertissement[W002]: ce `match` ne traite pas tous les cas
 --> main.ry:3:19
  |
3 | fn suivant(feu) = match feu
  |                   ^^^^^^^^^
  |
  = note: non traité : `Orange`
  = aide: ajoutez les cas manquants, comme `Orange -> ...`, ou un dernier cas `_ -> ...`

✓ main.ry : aucune erreur, 1 avertissement(s)
```

Il suffit d'ajouter la ligne `Orange -> Rouge`. C'est un grand avantage
d'un `type` : Rythme connaît tous ses cas, et vous rappelle ceux que vous
avez oubliés.

## Les motifs de listes

Un **motif** décrit la forme d'une valeur. Un motif peut décrire une liste :
`[]` pour la liste vide, `[x]` pour un seul élément, `[premier, ..reste]`
pour « au moins un élément ».

```rythme
fn decrire(liste) = match liste
  [] -> "vide"
  [x] -> "seulement {x}"
  [a, b] -> "{a} puis {b}"
  [premier, ..reste] -> "{premier}, et {len(reste)} autres"
end

print(decrire([]))
print(decrire([1]))
print(decrire([1, 2]))
print(decrire([1, 2, 3, 4]))
```

```sortie
vide
seulement 1
1 puis 2
1, et 3 autres
```

## Les motifs d'enregistrements

`{nom}` reconnaît un enregistrement qui a un champ `nom`, et le nomme.
`{age: 0}` exige une valeur précise. Les autres champs sont permis :

```rythme
fn saluer(personne) = match personne
  {nom, age: 0} -> "Bienvenue au monde, {nom} !"
  {nom, ville} -> "Bonjour {nom}, de {ville}."
  {nom} -> "Bonjour {nom}."
end

print(saluer({nom: "Léa", age: 0}))
print(saluer({nom: "Ada", ville: "Londres", age: 36}))
print(saluer({nom: "Alan", age: 41}))
```

```sortie
Bienvenue au monde, Léa !
Bonjour Ada, de Londres.
Bonjour Alan.
```

Alan n'a pas de champ `ville` : le deuxième cas ne lui correspond pas, il
prend donc le troisième.

## Les gardes dans match

Après un motif, `if condition` ajoute une condition, une **garde** :

```rythme
fn classer(n) = match n
  0 -> "zéro"
  x if x < 0 -> "négatif"
  x if x % 2 == 0 -> "pair"
  _ -> "impair"
end

print([0, -4, 6, 7] |> map(classer))
```

```sortie
["zéro", "négatif", "pair", "impair"]
```

Plusieurs possibilités s'écrivent avec `|` :

```rythme
type Feu = Rouge | Orange | Vert

fn consigne(feu) = match feu
  Rouge | Orange -> "attention"
  Vert -> "passez"
end

print(consigne(Orange), consigne(Vert))
```

```sortie
attention passez
```

## Les symboles

Un **symbole**, comme en Ruby, est un nom utilisé comme valeur : `:ok`,
`:en_attente`. Il s'écrit avec `:` devant. Il est pratique pour un état,
sans créer de type :

```rythme
fn etiquette(etat) = match etat
  :ok -> "terminé"
  :en_attente -> "en attente"
  _ -> "inconnu"
end

commande = {numero: 12, etat: :en_attente}
print(commande.etat, etiquette(commande.etat), etiquette(:perdu))
```

```sortie
:en_attente en attente inconnu
```

> **Note :** un symbole n'est jamais égal à un texte : `:ok == "ok"` vaut
> `false`. Pour un ensemble fixe de cas, un `type` est plus sûr, car le
> vérificateur repère les cas oubliés.

## Erreurs fréquentes

### Un nom de type en minuscules

```rythme norun
type feu = rouge | vert
```

```text
erreur[E001]: les noms de types commencent par une majuscule
 --> main.ry:1:6
  |
1 | type feu = rouge | vert
  |      ^^^
  |
  = aide: écrivez `type Feu = ...`
  = en savoir plus : `rythme explain E001`
```

Le type et ses variantes prennent une majuscule : `type Feu = Rouge | Vert`.

### Un cas oublié qui arrive vraiment

```rythme norun
type Feu = Rouge | Orange | Vert

fn suivant(feu) = match feu
  Rouge -> Vert
  Vert -> Orange
end

print(suivant(Orange))
```

```text
avertissement[W002]: ce `match` ne traite pas tous les cas
 --> main.ry:3:19
  |
3 | fn suivant(feu) = match feu
  |                   ^^^^^^^^^
  |
  = note: non traité : `Orange`
  = aide: ajoutez les cas manquants, comme `Orange -> ...`, ou un dernier cas `_ -> ...`

erreur[E006]: aucun cas de ce `match` ne traite Feu Orange
 --> main.ry:3:25
  |
3 | fn suivant(feu) = match feu
  |                         ^^^ cette valeur
  |
  = aide: ajoutez un cas pour cette valeur, ou un dernier cas `_ -> ...` qui traite tout le reste
  = appels en cours (le plus récent d'abord) :
      dans `suivant` appelée en main.ry:8:7
  = en savoir plus : `rythme explain E006`
```

L'avertissement arrive d'abord ; puis, quand `Orange` arrive vraiment, le
programme s'arrête. Ne laissez pas d'avertissement W002 : ajoutez
`Orange -> Rouge`.

### Oublier un champ d'une variante

```rythme norun
type Forme = Cercle(rayon) | Rect(largeur, hauteur)
r = Rect(3)
```

```text
erreur[E004]: `Rect` attend 2 arguments, mais en a reçu 1
 --> main.ry:2:5
  |
2 | r = Rect(3)
  |     ^^^^^^^ 1 argument(s) ici
  |
1 | type Forme = Cercle(rayon) | Rect(largeur, hauteur)
  |                              ---------------------- définie ici
  |
  = aide: utilisation : `Rect(largeur, hauteur)`
  = en savoir plus : `rythme explain E004`
```

Une variante avec champs est comme une fonction : donnez une valeur par
champ, `Rect(3, 4)`.

### Mettre _ en premier

```rythme norun
fn decrire(n) = match n
  _ -> "autre"
  0 -> "zéro"
end
print(decrire(0))
```

```text
avertissement[W004]: ce cas ne peut jamais être atteint
 --> main.ry:3:3
  |
3 |   0 -> "zéro"
  |   ^
  |
2 |   _ -> "autre"
  |   - ce cas traite déjà tout
  |
  = aide: les cas sont essayés dans l'ordre : mettez `_` en dernier
```

Le programme affiche `autre` : `_` attrape tout, même 0. Mettez toujours
`_` en dernier.

## Testez-vous

**Question 1 :** écrivez un type `Piece` qui vaut `Pile` ou `Face`.

> **Réponse**
>
> `type Piece = Pile | Face`.

**Question 2 :** qu'affiche ce programme ?

```rythme norun
fn tete(liste) = match liste
  [] -> "rien"
  [x, ..] -> "commence par {x}"
end
print(tete([7, 8, 9]), tete([]))
```

> **Réponse**
>
> `commence par 7 rien`. `[x, ..]` reconnaît une liste d'au moins un
> élément, et nomme le premier `x`.

**Question 3 :** pourquoi un `type` est-il plus sûr qu'un symbole pour
décrire l'état d'une commande ?

> **Réponse**
>
> Parce que Rythme connaît toutes les variantes d'un `type` : il signale
> les cas oubliés dans un `match` (W002), et un nom mal écrit est une
> erreur. Un symbole mal écrit, comme `:en_atente`, passe inaperçu.

## En résumé

- `type` crée un type avec ses variantes : `type Feu = Rouge | Orange | Vert`.
- Une variante peut porter des champs : `Cercle(rayon)`, lus avec un point.
- `match` traite chaque variante, dans l'ordre, et nomme ses champs ; `_`
  va en dernier.
- Les motifs reconnaissent aussi les listes (`[premier, ..reste]`) et les
  enregistrements (`{nom}`) ; une garde `if` ajoute une condition.
- `rythme check` signale les cas oubliés dans un `match`.

Suite : deux types déjà prêts, `Option` et `Result`, dans
[gérer les erreurs](erreurs.md). Comparez avec les types de
[Ruby](../ruby/types.md) et de [Rust](../rust/types.md).
