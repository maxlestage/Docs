# Vos propres types et match

Cette page montre comment créer vos propres types avec `type`, et comment les examiner avec `match` et ses motifs : listes, enregistrements, gardes et symboles.

## Créer un type

`type` crée un nouveau type, avec ses **variantes** séparées par `|`. Les
noms commencent par une majuscule :

```rythme
type Feu = Rouge | Orange | Vert

feu = Orange
print(feu, type_of(feu))
```

```sortie
Orange Feu
```

Une variante peut porter des **champs** : elle s'utilise alors comme une
fonction qui fabrique une valeur.

```rythme
type Forme = Cercle(rayon) | Rect(largeur, hauteur)

c = Cercle(2)
r = Rect(3, 4)
print(c, r, r.largeur)
```

```sortie
Cercle(2) Rect(3, 4) 3
```

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

`rythme check feu.ry` :

```text
avertissement[W002]: ce `match` ne traite pas tous les cas
 --> feu.ry:3:19
  |
3 | fn suivant(feu) = match feu
  |                   ^^^^^^^^^
  |
  = note: non traité : `Orange`
  = aide: ajoutez les cas manquants, comme `Orange -> ...`, ou un dernier cas `_ -> ...`

✓ feu.ry : aucune erreur, 1 avertissement(s)
```

Il suffit d'ajouter la ligne `Orange -> Rouge`.

## Les motifs de listes

Un motif peut décrire une liste : `[]` pour la liste vide, `[x]` pour un
seul élément, `[premier, ..reste]` pour « au moins un élément ».

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

Plusieurs possibilités s'écrivent avec `|` : `Rouge | Orange -> "attention"`.

## Les symboles

Un **symbole**, comme en Ruby, est un nom utilisé comme valeur : `:ok`,
`:en_attente`. Il est pratique pour un état, sans créer de type :

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

## En résumé

- `type` crée un type avec ses variantes : `type Feu = Rouge | Orange | Vert`.
- Une variante peut porter des champs : `Cercle(rayon)`.
- `match` traite chaque variante, dans l'ordre, et nomme ses champs.
- Les motifs reconnaissent aussi les listes (`[premier, ..reste]`) et les
  enregistrements (`{nom}`).
- `rythme check` signale les cas oubliés dans un `match`.

Suite : deux types déjà prêts, `Option` et `Result`, dans
[gérer les erreurs](erreurs.md). Comparez avec les types de
[Ruby](../ruby/types.md) et de [Rust](../rust/types.md).
