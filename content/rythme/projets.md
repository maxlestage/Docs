# Projets pas à pas

Trois petits programmes complets, construits étape par étape, pour réunir tout ce que vous avez appris : un jeu, une liste de tâches qui se souvient, et un bulletin de notes.

> **Objectifs :**
>
> - construire un programme petit à petit, en le lançant à chaque étape ;
> - répéter avec une fonction récursive, sans boucle ;
> - garder des données dans un fichier JSON, et les relire ;
> - transformer un texte CSV en enregistrements, puis en un tableau bien aligné.

## Comment suivre ces projets

Chaque projet avance par **étapes**. À chaque étape, le programme est
complet : recopiez-le dans un fichier (par exemple `jeu.ry`), lancez-le avec
`rythme run jeu.ry`, et comparez avec la sortie donnée. Puis passez à
l'étape suivante, qui ajoute une seule idée.

C'est la bonne façon de programmer, même pour les experts : un petit pas,
on vérifie, un autre petit pas. Quand quelque chose casse, on sait que c'est
dans la dernière modification.

À la fin de chaque projet, « Pour aller plus loin » propose des idées pour
continuer seul.

## Projet 1 : Le juste nombre

Le programme choisit un nombre secret entre 1 et 100. Le joueur propose des
nombres ; à chaque proposition, le programme répond « trop petit », « trop
grand » ou « gagné ! ».

Pour pouvoir lancer chaque étape sans clavier, les propositions viennent
d'abord d'une **liste fixe**. Le vrai jeu au clavier arrive à la fin.

### Étape 1 : comparer une proposition

Tout le jeu repose sur une fonction : elle reçoit le nombre secret et une
proposition, et donne la réponse.

```rythme
fn indice(secret, proposition) =
  if proposition < secret then "trop petit"
  else if proposition > secret then "trop grand"
  else "gagné !"

print(indice(42, 50))
print(indice(42, 10))
print(indice(42, 42))
```

```sortie
trop grand
trop petit
gagné !
```

On essaie les trois cas possibles : c'est le moment de vérifier que la
fonction ne se trompe pas.

### Étape 2 : essayer une liste de propositions

Le joueur fait plusieurs propositions. `each` les essaie l'une après
l'autre :

```rythme
fn indice(secret, proposition) =
  if proposition < secret then "trop petit"
  else if proposition > secret then "trop grand"
  else "gagné !"

secret = 42
propositions = [50, 25, 37, 43, 42]

propositions.each do |p|
  print("{p} : {indice(secret, p)}")
end
```

```sortie
50 : trop grand
25 : trop petit
37 : trop petit
43 : trop grand
42 : gagné !
```

### Étape 3 : s'arrêter et compter les essais

Avec `each`, le jeu continuerait même après « gagné ! ». Il faut
**s'arrêter** au bon moment, et compter les essais. Rythme n'a pas de
boucle `while` : on écrit une fonction **récursive** (voir
[répéter avec la récursion](conditions-boucles.md#repeter-avec-la-recursion)).

`jouer` reçoit le secret, les propositions qui restent, et le numéro de
l'essai :

```rythme
fn indice(secret, proposition) =
  if proposition < secret then "trop petit"
  else if proposition > secret then "trop grand"
  else "gagné !"

fn jouer(secret, [], _) = print("Perdu ! Le nombre était {secret}.")
fn jouer(secret, [p, ..reste], essai)
  print("Essai {essai} : {p}, {indice(secret, p)}")
  if p == secret then print("Trouvé en {essai} essais.")
  else jouer(secret, reste, essai + 1)
end

jouer(42, [50, 25, 37, 43, 42, 99], 1)
print("---")
jouer(42, [10, 20], 1)
```

```sortie
Essai 1 : 50, trop grand
Essai 2 : 25, trop petit
Essai 3 : 37, trop petit
Essai 4 : 43, trop grand
Essai 5 : 42, gagné !
Trouvé en 5 essais.
---
Essai 1 : 10, trop petit
Essai 2 : 20, trop petit
Perdu ! Le nombre était 42.
```

Lisons les deux cas de `jouer` :

- s'il ne reste **aucune** proposition (`[]`), la partie est perdue ;
- sinon, on prend la première, `p`, et le `reste`. On affiche la réponse.
  Si c'est gagné, on s'arrête ; sinon, on rejoue avec le reste, et un essai
  de plus.

Le `99`, après le `42`, n'est jamais essayé : la partie s'arrête dès que le
nombre est trouvé.

### Étape 4 : l'ordinateur devine tout seul

Et si l'ordinateur jouait ? La meilleure stratégie : proposer **le milieu**
de l'intervalle possible. « Trop grand » ? Le nombre est dans la moitié
basse. « Trop petit » ? Dans la moitié haute. À chaque essai, on élimine la
moitié des nombres.

```rythme
fn indice(secret, proposition) =
  if proposition < secret then "trop petit"
  else if proposition > secret then "trop grand"
  else "gagné !"

fn deviner(secret, bas, haut, essai)
  milieu = (bas + haut) // 2
  print("Essai {essai} : {milieu}, {indice(secret, milieu)}")
  if milieu == secret then essai
  else if milieu < secret then deviner(secret, milieu + 1, haut, essai + 1)
  else deviner(secret, bas, milieu - 1, essai + 1)
end

essais = deviner(42, 1, 100, 1)
print("L'ordinateur a trouvé en {essais} essais.")
```

```sortie
Essai 1 : 50, trop grand
Essai 2 : 25, trop petit
Essai 3 : 37, trop petit
Essai 4 : 43, trop grand
Essai 5 : 40, trop petit
Essai 6 : 41, trop petit
Essai 7 : 42, gagné !
L'ordinateur a trouvé en 7 essais.
```

`//` est la division entière : `(1 + 100) // 2` vaut `50`. Avec cette
méthode, 7 essais suffisent toujours entre 1 et 100.

### Étape 5 : jouer au clavier

Voici enfin le vrai jeu. Le secret est tiré au hasard avec `random(1, 100)`,
et les propositions sont lues au clavier avec `input` (voir
[lire au clavier](fichiers.md#lire-au-clavier-avec-input)). Ce programme
attend quelqu'un au clavier : il n'est pas lancé sur ce site.

```rythme norun
fn indice(secret, proposition) =
  if proposition < secret then "trop petit"
  else if proposition > secret then "trop grand"
  else "gagné !"

fn tour(secret, essai) = match parse_int(input("Votre proposition ? "))
  Ok(p) -> reagir(secret, p, essai)
  Err(_) -> pas_un_nombre(secret, essai)
end

fn reagir(secret, p, essai)
  print(indice(secret, p))
  if p == secret then print("Trouvé en {essai} essais.")
  else tour(secret, essai + 1)
end

fn pas_un_nombre(secret, essai)
  print("Tapez un nombre entier, par exemple 50.")
  tour(secret, essai)
end

print("J'ai choisi un nombre entre 1 et 100.")
tour(random(1, 100), 1)
```

Une partie, dans le terminal :

```text
J'ai choisi un nombre entre 1 et 100.
Votre proposition ? 50
trop grand
Votre proposition ? abc
Tapez un nombre entier, par exemple 50.
Votre proposition ? 25
trop petit
Votre proposition ? 42
gagné !
Trouvé en 3 essais.
```

Trois fonctions se renvoient la balle : `tour` lit une proposition,
`reagir` répond puis relance `tour`, et `pas_un_nombre` explique l'erreur
puis relance `tour` sans compter d'essai.

### Pour aller plus loin

- Limitez la partie à 7 essais : au-delà, affichez « Perdu ! ».
- Ajoutez « tout près ! » quand la proposition est à 3 ou moins du secret
  (`abs(proposition - secret) <= 3`).
- Refusez les nombres hors de 1 à 100, comme `pas_un_nombre` refuse les mots.
- Écrivez des tests pour `indice` (voir [tester son code](tests.md)).

## Projet 2 : Une liste de tâches

Une liste de tâches, comme « Acheter du pain » ou « Réviser Rythme », que
l'on coche quand c'est fait. La liste est **gardée dans un fichier** : elle
est toujours là au lancement suivant.

### Étape 1 : afficher les tâches

Une tâche est un enregistrement avec un titre et une case à cocher, `faite`.
La liste est une liste d'enregistrements :

```rythme
taches = [
  {titre: "Acheter du pain", faite: true},
  {titre: "Réviser Rythme", faite: false},
]

fn afficher(taches) = taches.each_with_index do |tache, i|
  marque = if tache.faite then "[x]" else "[ ]"
  print("{i + 1}. {marque} {tache.titre}")
end

afficher(taches)
```

```sortie
1. [x] Acheter du pain
2. [ ] Réviser Rythme
```

`each_with_index` donne chaque tâche et sa position, à partir de 0 : on
affiche `i + 1` pour numéroter à partir de 1.

### Étape 2 : ajouter et terminer une tâche

Les valeurs ne changent jamais en Rythme : `ajouter` et `terminer` ne
modifient pas la liste, elles donnent une **nouvelle** liste.

```rythme
fn afficher(taches) = taches.each_with_index do |tache, i|
  marque = if tache.faite then "[x]" else "[ ]"
  print("{i + 1}. {marque} {tache.titre}")
end

fn ajouter(taches, titre) = push(taches, {titre: titre, faite: false})

fn terminer(taches, numero) if numero < 1 or numero > len(taches) = taches
fn terminer(taches, numero)
  i = numero - 1
  set_at(taches, i, {..taches[i], faite: true})
end

liste = []
  |> ajouter("Acheter du pain")
  |> ajouter("Réviser Rythme")
  |> ajouter("Arroser les plantes")
  |> terminer(1)
  |> terminer(9)

afficher(liste)
```

```sortie
1. [x] Acheter du pain
2. [ ] Réviser Rythme
3. [ ] Arroser les plantes
```

- `push(taches, tache)` donne une nouvelle liste, avec la tâche à la fin.
- `{..tache, faite: true}` copie la tâche en changeant un seul champ, et
  `set_at` la remet à sa place.
- La première version de `terminer` a une **garde** (voir
  [les gardes](fonctions.md#les-gardes)) : pour un numéro qui n'existe pas,
  comme `9`, la liste reste la même au lieu de provoquer une erreur.

### Étape 3 : sauver et relire en JSON

Pour garder la liste, on l'écrit dans un fichier au format JSON (voir
[le format JSON](fichiers.md#le-format-json)). `to_json(taches, 2)` écrit
un élément par ligne : le fichier reste lisible pour un humain.

```rythme
fn ajouter(taches, titre) = push(taches, {titre: titre, faite: false})

fn sauver(taches) = write_file("taches.json", to_json(taches, 2))

sauver(ajouter([], "Acheter du pain"))
print(read_file("taches.json") |> unwrap_or(""))
```

```sortie
[
  {
    "titre": "Acheter du pain",
    "faite": false
  }
]
```

Pour relire, deux cas : la toute première fois, le fichier n'existe pas
encore, et la liste est vide ; ensuite, on lit le fichier et on le
transforme avec `parse_json`. Lire et transformer peuvent échouer : `charger`
donne un `Result`.

```rythme
fichier = "taches.json"

fn ajouter(taches, titre) = push(taches, {titre: titre, faite: false})

fn sauver(taches) = write_file(fichier, to_json(taches, 2))

fn lire()
  texte = read_file(fichier)?
  parse_json(texte)
end

fn charger() = if file_exists(fichier) then lire() else Ok([])

print(charger())
sauver(ajouter([], "Acheter du pain"))
print(charger())
```

```sortie
Ok([])
Ok([{titre: "Acheter du pain", faite: false}])
```

Les objets JSON redeviennent des enregistrements : on retrouve exactement la
liste sauvée.

### Étape 4 : le programme complet

Il reste à tout assembler. Chaque lancement du programme fait quatre
choses : **charger** la liste, **exécuter** une commande, **sauver** le
résultat, **afficher** la liste.

Une commande est une liste de mots, comme `["ajouter", "Acheter du pain"]`
ou `["finir", "1"]`. Les motifs de listes choisissent le bon cas (voir
[les motifs de listes](types.md#les-motifs-de-listes)). Ici, on simule trois
lancements à la suite :

```rythme
fichier = "taches.json"

fn afficher(taches) = taches.each_with_index do |tache, i|
  marque = if tache.faite then "[x]" else "[ ]"
  print("{i + 1}. {marque} {tache.titre}")
end

fn ajouter(taches, titre) = push(taches, {titre: titre, faite: false})

fn terminer(taches, numero) if numero < 1 or numero > len(taches) = taches
fn terminer(taches, numero)
  i = numero - 1
  set_at(taches, i, {..taches[i], faite: true})
end

fn sauver(taches) = write_file(fichier, to_json(taches, 2))

fn lire()
  texte = read_file(fichier)?
  parse_json(texte)
end

fn charger() = if file_exists(fichier) then lire() else Ok([])

fn executer(taches, ["ajouter", titre]) = ajouter(taches, titre)
fn executer(taches, ["finir", numero]) = terminer(taches, parse_int(numero) |> unwrap_or(0))
fn executer(taches, _) = taches

fn lancer(commande)
  taches = charger() |> unwrap_or([])
  nouvelles = executer(taches, commande)
  sauver(nouvelles)
  afficher(nouvelles)
end

lancer(["ajouter", "Acheter du pain"])
print("---")
lancer(["ajouter", "Réviser Rythme"])
print("---")
lancer(["finir", "1"])
```

```sortie
1. [ ] Acheter du pain
---
1. [ ] Acheter du pain
2. [ ] Réviser Rythme
---
1. [x] Acheter du pain
2. [ ] Réviser Rythme
```

Chaque appel à `lancer` relit le fichier écrit par le précédent : la liste
se souvient. Le dernier cas de `executer`, avec `_`, accepte n'importe quelle
autre commande (ou aucune) et garde la liste telle quelle.

### Étape 5 : des commandes depuis le terminal

Pour un vrai programme, la commande vient des mots tapés après le nom du
fichier : `args()` (voir
[les mots de la ligne de commande](fichiers.md#les-mots-de-la-ligne-de-commande-avec-args)).
Dans le programme de l'étape 4, remplacez les trois `lancer(...)` et les
`print("---")` par une seule ligne :

```rythme norun
lancer(args())
```

Dans le terminal, avec le programme enregistré sous `taches.ry` :

```sh
rythme run taches.ry ajouter "Acheter du pain"
rythme run taches.ry ajouter "Réviser Rythme"
rythme run taches.ry finir 1
rythme run taches.ry
```

```text
1. [ ] Acheter du pain
1. [ ] Acheter du pain
2. [ ] Réviser Rythme
1. [x] Acheter du pain
2. [ ] Réviser Rythme
1. [x] Acheter du pain
2. [ ] Réviser Rythme
```

Les guillemets autour de `"Acheter du pain"` en font un seul mot pour le
terminal. Sans commande, le programme affiche simplement la liste.

### Pour aller plus loin

- Ajoutez une commande `retirer 2`, qui enlève une tâche avec `remove_at`.
- Ajoutez une commande `vider`, qui garde seulement les tâches pas encore
  faites (`reject`).
- Affichez un message d'aide quand la commande est inconnue.
- Affichez un bilan à la fin : « 1 tâche faite sur 2 » (`count`).

## Projet 3 : Le bulletin de notes

Un professeur a les notes de sa classe dans un tableau, au format CSV : une
ligne par élève, des virgules entre les colonnes. Le programme calcule la
moyenne de chacun, trouve le meilleur élève, et affiche un bulletin bien
aligné.

### Étape 1 : découper le texte CSV

Le CSV est ici un texte écrit dans le programme ; la première ligne donne le
nom des colonnes. `lines` découpe en lignes, `drop(lignes, 1)` enlève la
première, et `split(ligne, ",")` découpe une ligne en colonnes :

```rythme
csv = "nom,maths,français,histoire
Ada,18,15,17
Alan,14,12,16
Grace,19,17,15
Edsger,9,13,11"

lignes = lines(csv)
print(lignes[0])
print(drop(lignes, 1))
print(split("Ada,18,15,17", ","))
```

```sortie
nom,maths,français,histoire
["Ada,18,15,17", "Alan,14,12,16", "Grace,19,17,15", "Edsger,9,13,11"]
["Ada", "18", "15", "17"]
```

Un texte entre guillemets peut tenir sur plusieurs lignes. Attention : les
notes sont encore des **textes** (`"18"`), pas des nombres.

### Étape 2 : des enregistrements

Chaque ligne devient un enregistrement `{nom, notes}`. Le motif
`[nom, ..notes]` met la première colonne dans `nom`, et toutes les autres
dans la liste `notes`. Puis `parse_int` lit chaque note comme un nombre :

```rythme
csv = "nom,maths,français,histoire
Ada,18,15,17
Alan,14,12,16
Grace,19,17,15
Edsger,9,13,11"

fn lire_eleve(ligne)
  [nom, ..notes] = split(ligne, ",")
  {nom: nom, notes: map(notes, fn(n) = parse_int(n) |> unwrap_or(0))}
end

eleves = lines(csv) |> drop(1) |> map(lire_eleve)
eleves.each do |e|
  print(e)
end
```

```sortie
{nom: "Ada", notes: [18, 15, 17]}
{nom: "Alan", notes: [14, 12, 16]}
{nom: "Grace", notes: [19, 17, 15]}
{nom: "Edsger", notes: [9, 13, 11]}
```

`parse_int` donne un `Result` : `unwrap_or(0)` donne 0 pour une note mal
écrite.

### Étape 3 : moyennes et meilleur élève

On ajoute à chaque élève un champ `moyenne`, avec `{..e, moyenne: ...}`.
Puis `max_by` trouve l'élève qui a la plus grande moyenne :

```rythme
csv = "nom,maths,français,histoire
Ada,18,15,17
Alan,14,12,16
Grace,19,17,15
Edsger,9,13,11"

fn lire_eleve(ligne)
  [nom, ..notes] = split(ligne, ",")
  {nom: nom, notes: map(notes, fn(n) = parse_int(n) |> unwrap_or(0))}
end

fn moyenne(nombres) = sum(nombres) / len(nombres)

eleves = lines(csv)
  |> drop(1)
  |> map(lire_eleve)
  |> map(fn(e) = {..e, moyenne: moyenne(e.notes)})

eleves.each do |e|
  print("{e.nom} : {text(e.moyenne, 1)}")
end

premier = max_by(eleves, fn(e) = e.moyenne)
print("Meilleure moyenne : {premier.nom}")
print("Moyenne de la classe : {text(moyenne(map(eleves, fn(e) = e.moyenne)), 1)}")
```

```sortie
Ada : 16.7
Alan : 14.0
Grace : 17.0
Edsger : 11.0
Meilleure moyenne : Grace
Moyenne de la classe : 14.7
```

`text(x, 1)` écrit un nombre avec exactement 1 décimale : `16.666...`
devient `16.7`, et `14` devient `14.0`.

### Étape 4 : un bulletin aligné

Dernière étape : un vrai tableau. `pad_right` complète un texte avec des
espaces à droite, `pad_left` à gauche (voir
[répéter et aligner](texte.md#repeter-et-aligner)). La fonction `rangee`
écrit une ligne : le nom sur 8 caractères, puis chaque case sur 10,
alignée à droite. Les élèves sont classés de la meilleure moyenne à la
moins bonne :

```rythme
csv = "nom,maths,français,histoire
Ada,18,15,17
Alan,14,12,16
Grace,19,17,15
Edsger,9,13,11"

fn lire_eleve(ligne)
  [nom, ..notes] = split(ligne, ",")
  {nom: nom, notes: map(notes, fn(n) = parse_int(n) |> unwrap_or(0))}
end

fn moyenne(nombres) = sum(nombres) / len(nombres)

fn rangee(nom, cases) = pad_right(nom, 8) + join(map(cases, fn(c) = pad_left(c, 10)), "")

[entete, ..lignes] = lines(csv)
[_, ..matieres] = split(entete, ",")

eleves = lignes
  |> map(lire_eleve)
  |> map(fn(e) = {..e, moyenne: moyenne(e.notes)})
  |> sort_by(fn(e) = -e.moyenne)

print(rangee("Élève", [..matieres, "moyenne"]))
print(repeat("-", 48))
eleves.each do |e|
  print(rangee(e.nom, [..map(e.notes, text), text(e.moyenne, 1)]))
end
```

```sortie
Élève        maths  français  histoire   moyenne
------------------------------------------------
Grace           19        17        15      17.0
Ada             18        15        17      16.7
Alan            14        12        16      14.0
Edsger           9        13        11      11.0
```

Quelques nouveautés :

- `[entete, ..lignes] = lines(csv)` sépare la première ligne des autres ;
  `[_, ..matieres]` laisse de côté `nom` et garde le nom des matières.
- `sort_by(fn(e) = -e.moyenne)` classe par moyenne **décroissante** : le
  signe `-` inverse l'ordre.
- `map(e.notes, text)` transforme chaque note en texte, pour l'aligner.
- `[..matieres, "moyenne"]` est une nouvelle liste : les matières, plus une
  case.

### Pour aller plus loin

- Lisez les notes depuis un fichier `notes.csv`, avec `read_file` (voir
  [un petit fichier CSV](fichiers.md#un-petit-fichier-csv)).
- Ajoutez une colonne « mention » : « très bien » à partir de 16, « bien »
  à partir de 14, etc.
- Affichez la moyenne de la classe **par matière**, sous le tableau.
- Rangez les fonctions dans un module `bulletin.ry`, et écrivez des tests
  pour `moyenne` et `lire_eleve`.

## Erreurs fréquentes

### Oublier qu'une lecture donne un Result

`read_file` donne `Ok(texte)` ou `Err(message)`, pas directement le texte :

```rythme norun
write_file("taches.json", "[]")
taches = parse_json(read_file("taches.json"))
```

```text
erreur[E005]: `parse_json` attend un texte comme argument 1, mais a reçu une valeur de type Result
 --> taches.ry:2:21
  |
2 | taches = parse_json(read_file("taches.json"))
  |                     ^^^^^^^^^^^^^^^^^^^^^^^^ mauvaise sorte de valeur
  |
  = aide: utilisation : `parse_json(text)`
  = en savoir plus : `rythme explain E005`
```

Correction : dans une fonction, `texte = read_file(chemin)?`, comme `lire`
à l'étape 3 ; au premier niveau, `read_file(chemin) |> unwrap_or("")`.

### Calculer avec des notes encore en texte

`split` donne des textes. Sans `parse_int`, les notes restent `"18"`, et
`sum` refuse de les additionner :

```rythme norun
fn lire_eleve(ligne)
  [nom, ..notes] = split(ligne, ",")
  {nom: nom, notes: notes}
end

fn moyenne(nombres) = sum(nombres) / len(nombres)

eleve = lire_eleve("Ada,18,15,17")
print(moyenne(eleve.notes))
```

```text
erreur[E023]: `sum` a besoin d'une liste de nombres, mais elle contient Text "18"
 --> bulletin.ry:6:23
  |
6 | fn moyenne(nombres) = sum(nombres) / len(nombres)
  |                       ^^^^^^^^^^^^
  |
  = appels en cours (le plus récent d'abord) :
      dans `moyenne` appelée en bulletin.ry:9:7
  = en savoir plus : `rythme explain E023`
```

Correction : `notes: map(notes, fn(n) = parse_int(n) |> unwrap_or(0))`.

### Un numéro de tâche qui n'existe pas

Sans la garde de l'étape 2, `terminer` plante sur un numéro trop grand :

```rythme norun
fn terminer(taches, numero)
  i = numero - 1
  set_at(taches, i, {..taches[i], faite: true})
end

taches = [{titre: "Acheter du pain", faite: false}]
print(terminer(taches, 3))
```

```text
erreur[E007]: l'indice 2 est hors limites : la liste a 1 élément(s)
 --> taches.ry:3:31
  |
3 |   set_at(taches, i, {..taches[i], faite: true})
  |                               ^ aucun élément ici
  |
  = aide: les indices valides vont de 0 à 0 (ou -1 pour le dernier élément)
  = aide: pour obtenir une Option au lieu d'une erreur, utilisez `get(x, 2)`
  = appels en cours (le plus récent d'abord) :
      dans `terminer` appelée en taches.ry:7:7
  = en savoir plus : `rythme explain E007`
```

Correction : ajoutez la garde
`fn terminer(taches, numero) if numero < 1 or numero > len(taches) = taches`
avant l'autre cas.

## Testez-vous

1. Dans le juste nombre, pourquoi `jouer` est-elle une fonction récursive,
   et pas un simple `each` ?

> **Réponse**
>
> `each` essaie toujours **tous** les éléments. La fonction récursive peut
> s'arrêter dès que le nombre est trouvé, et compter les essais grâce à son
> paramètre `essai`.

2. Pourquoi `charger` donne-t-elle `Ok([])` quand le fichier n'existe pas ?

> **Réponse**
>
> La toute première fois, il n'y a encore aucune tâche : ce n'est pas une
> erreur, c'est une liste vide. Le fichier sera créé par le premier
> `sauver`.

3. Que donne `pad_left("7", 4)` ?

> **Réponse**
>
> `"   7"` : le texte complété par 3 espaces à gauche, pour faire 4
> caractères. C'est ce qui aligne les nombres à droite dans le bulletin.

## En résumé

- On construit un programme par petites étapes, en le lançant à chaque fois.
- Pour répéter jusqu'à une condition, une fonction récursive : un cas pour
  s'arrêter, un cas qui s'appelle avec la suite.
- Les valeurs ne changent pas : `push`, `set_at` et `{..r, champ: x}`
  donnent de nouvelles valeurs.
- `to_json` et `write_file` gardent des données ; `read_file` et
  `parse_json` les relisent, dans des `Result`.
- `lines`, `split` et `parse_int` transforment un CSV en enregistrements ;
  `pad_left` et `pad_right` alignent un tableau.

Suite : [les exercices corrigés](exercices.md). Comparez avec les projets en
[Ruby](../ruby/projets.md) et en [Rust](../rust/projets.md).
