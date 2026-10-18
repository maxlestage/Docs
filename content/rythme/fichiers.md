# Lire et écrire : clavier et fichiers

Cette page montre comment un programme Rythme reçoit des informations (le clavier, la ligne de commande) et comment il garde ses résultats dans des fichiers : texte, lignes, CSV et JSON.

> **Objectifs :**
>
> - vous saurez lire une ligne tapée au clavier, et la transformer en nombre ;
> - vous saurez lire les mots donnés sur la ligne de commande ;
> - vous saurez écrire, compléter et lire un fichier texte, en traitant les échecs ;
> - vous saurez traiter un fichier ligne par ligne, et lire un petit fichier CSV ;
> - vous saurez garder des données en JSON.

## Lire au clavier avec input

`input(question)` affiche la question, attend que l'utilisateur tape une
ligne et appuie sur Entrée, puis donne ce qu'il a tapé, sous forme de texte :

```rythme norun
nom = input("Comment vous appelez-vous ? ")
print("Bonjour, {nom} !")
```

Dans le terminal, cela donne :

```text
Comment vous appelez-vous ? Ada
Bonjour, Ada !
```

Ce que donne `input` est toujours un **texte**. Pour un nombre, lisez-le
avec `parse_int`, qui donne un `Result` (voir
[travailler avec du texte](texte.md)) :

```rythme norun
saisie = input("Votre âge ? ")
match parse_int(saisie)
  Ok(age) -> print("Dans dix ans, vous aurez {age + 10} ans.")
  Err(_) -> print("Ce n'est pas un nombre.")
end
```

```text
Votre âge ? 30
Dans dix ans, vous aurez 40 ans.
```

> **Note :** les exemples avec `input` ne sont pas lancés sur ce site : ils
> attendent quelqu'un au clavier. Essayez-les chez vous avec `rythme run`.

## Les mots de la ligne de commande avec args

Les mots écrits après le nom du fichier, dans le terminal, arrivent dans la
liste `args()` :

```sh
rythme run saluer.ry Ada Alan
```

Voici `saluer.ry`. Il prévoit aussi le cas où aucun mot n'est donné :

```rythme
fn saluer([]) = print("Donnez au moins un nom après le nom du fichier.")
fn saluer(noms) = noms.each do |nom|
  print("Bonjour, {nom} !")
end

saluer(args())
```

Lancé sans mot, il affiche :

```sortie
Donnez au moins un nom après le nom du fichier.
```

Et avec `Ada Alan` :

```text
Bonjour, Ada !
Bonjour, Alan !
```

## Écrire un fichier avec write_file

Un **fichier** est un texte rangé sur le disque de l'ordinateur : il reste
là quand le programme s'arrête. Le programme le retrouve par son
**chemin**, son nom et, si besoin, les dossiers où il se trouve
(`notes.txt`, `donnees/notes.txt`).

`write_file(chemin, texte)` crée le fichier, ou remplace tout son contenu.
Écrire peut échouer (un dossier qui n'existe pas, pas le droit d'écrire...),
donc la réponse est un `Result` : `Ok(())` si tout va bien.

```rythme
resultat = write_file("courses.txt", "pain\nlait\nœufs")
print(resultat)
```

```sortie
Ok(())
```

Le fichier `courses.txt` est créé dans le dossier où vous lancez le
programme. `\n` passe à la ligne : le fichier a trois lignes.

> **Attention :** `write_file` remplace **tout** le contenu, sans demander.
> C'est comme réécrire une page au propre : l'ancienne version est perdue.

## Lire un fichier avec read_file

`read_file(chemin)` donne tout le contenu du fichier, dans un `Result` :
`Ok(texte)`, ou `Err(message)` si le fichier n'existe pas.

```rythme
write_file("courses.txt", "pain\nlait\nœufs")

print(read_file("courses.txt"))
print(read_file("oubli.txt"))
```

```sortie
Ok("pain\nlait\nœufs")
Err("fichier introuvable : oubli.txt")
```

On traite les deux cas avec `match` :

```rythme
write_file("courses.txt", "pain\nlait\nœufs")

match read_file("courses.txt")
  Ok(texte) -> print(texte)
  Err(e) -> print("Problème : {e}")
end
```

```sortie
pain
lait
œufs
```

## Transmettre l'erreur avec ?

Dans une fonction, `?` évite un `match` à chaque étape : si la lecture
échoue, la fonction s'arrête et renvoie l'erreur (voir
[gérer les erreurs](erreurs.md#l-operateur-pour-transmettre-l-erreur)).

```rythme
fn nombre_de_lignes(chemin)
  texte = read_file(chemin)?
  Ok(len(lines(texte)))
end

write_file("courses.txt", "pain\nlait\nœufs")
print(nombre_de_lignes("courses.txt"))
print(nombre_de_lignes("oubli.txt"))
```

```sortie
Ok(3)
Err("fichier introuvable : oubli.txt")
```

> **Attention :** `?` ne marche que dans une fonction. Au premier niveau du
> programme, utilisez `match` ou `unwrap_or`.

## Ajouter à la fin avec append_file

`write_file` remplace tout le contenu. `append_file` ajoute à la fin, et crée
le fichier s'il n'existe pas encore. C'est pratique pour un journal :

```rythme
append_file("journal.txt", "lundi : 5 km\n")
append_file("journal.txt", "mardi : repos\n")
append_file("journal.txt", "mercredi : 8 km\n")

print(read_file("journal.txt") |> unwrap_or(""))
```

```sortie
lundi : 5 km
mardi : repos
mercredi : 8 km
```

N'oubliez pas le `\n` à la fin de chaque ligne ajoutée.

## Savoir si un fichier existe

`file_exists(chemin)` répond simplement `true` ou `false` :

```rythme
write_file("notes.txt", "17")

print(file_exists("notes.txt"), file_exists("absent.txt"))
message = if file_exists("reglages.txt") then "réglages trouvés" else "réglages par défaut"
print(message)
```

```sortie
true false
réglages par défaut
```

## Traiter un fichier ligne par ligne

Souvent, on lit le fichier, on le découpe en lignes avec `lines`, puis on
travaille sur la liste avec `map`, `filter` ou `each` :

```rythme
write_file("courses.txt", "pain\nlait\nœufs")

texte = read_file("courses.txt") |> unwrap_or("")
lines(texte) |> each_with_index(fn(article, i) = print("{i + 1}. {article}"))
```

```sortie
1. pain
2. lait
3. œufs
```

`each_with_index` donne aussi la position de chaque élément, à partir de 0.

## Un petit fichier CSV

Un fichier **CSV** est un tableau écrit en texte : une ligne par rangée, des
virgules entre les colonnes. On le lit avec `lines`, puis `split(ligne, ",")`
pour chaque ligne :

```rythme
write_file("notes.csv", "Ada,17\nAlan,12\nGrace,19")

fn lire_eleve(ligne)
  [nom, note] = split(ligne, ",")
  {nom: nom, note: parse_int(note) |> unwrap_or(0)}
end

eleves = read_file("notes.csv")
  |> unwrap_or("")
  |> lines
  |> map(lire_eleve)

eleves.each do |e|
  print("{e.nom} : {e.note}")
end
print("Moyenne :", sum(map(eleves, fn(e) = e.note)) / len(eleves))
```

```sortie
Ada : 17
Alan : 12
Grace : 19
Moyenne : 16.0
```

`[nom, note] = split(...)` donne un nom à chacun des deux morceaux (voir
[décomposer une valeur](collections.md#decomposer-une-valeur)).

## Le format JSON

**JSON** est un format de texte très répandu pour ranger des données.
`to_json` transforme une valeur en texte JSON, et `parse_json` relit ce
texte. Les enregistrements deviennent des objets JSON, et inversement.

```rythme
ada = {nom: "Ada", notes: [17, 19]}
print(to_json(ada))
```

```sortie
{"nom": "Ada", "notes": [17, 19]}
```

Pour garder des données d'une fois sur l'autre, on écrit le JSON dans un
fichier, puis on le relit. `parse_json` donne un `Result`, car le texte peut
ne pas être du JSON correct :

```rythme
fn sauver(chemin, valeur) = write_file(chemin, to_json(valeur))

fn charger(chemin)
  texte = read_file(chemin)?
  parse_json(texte)
end

sauver("ada.json", {nom: "Ada", notes: [17, 19]})

match charger("ada.json")
  Ok(eleve) -> print("{eleve.nom} a {len(eleve.notes)} notes.")
  Err(e) -> print("Problème : {e}")
end
print(parse_json("pas du json") |> is_err)
```

```sortie
Ada a 2 notes.
true
```

> **Astuce :** `to_json(valeur, 2)` écrit un élément par ligne, décalé de
> 2 espaces : le fichier est plus facile à lire pour un humain.

## Erreurs fréquentes

### Calculer avec ce que tape l'utilisateur

```rythme norun
age = input("Votre âge ? ")
print("Dans dix ans : {age + 10}")
```

```text
erreur[E005]: impossible d'utiliser `+` avec Text et Int
 --> main.ry:2:28
  |
2 | print("Dans dix ans : {age + 10}")
  |                            ^ Text + Int 10
  |
  = aide: pour construire un texte contenant une valeur, utilisez des accolades : "total : {x}"
      ou convertissez d'abord la valeur : text(x)
  = en savoir plus : `rythme explain E005`
```

`input` donne toujours un texte, même si l'on tape `30`. Lisez-le d'abord
avec `parse_int`, comme dans [lire au clavier](#lire-au-clavier-avec-input).
L'erreur est trouvée avant même de poser la question.

### Oublier que read_file donne un Result

```rythme norun
write_file("courses.txt", "pain\nlait")
texte = read_file("courses.txt")
print(lines(texte))
```

```text
erreur[E005]: `lines` attend un texte comme argument 1, mais a reçu une valeur de type Result
 --> main.ry:3:13
  |
3 | print(lines(texte))
  |             ^^^^^ mauvaise sorte de valeur
  |
  = aide: utilisation : `lines(text)`
  = en savoir plus : `rythme explain E005`
```

`texte` est `Ok("pain\nlait")`, pas le texte lui-même : la lecture aurait
pu échouer. Ouvrez le `Result` avec `match`, `unwrap_or("")`, ou `?` dans
une fonction.

### Se tromper de séparateur

```rythme norun
ligne = "Ada;17"
[nom, note] = split(ligne, ",")
print(nom, note)
```

```text
erreur[E006]: la valeur List ["Ada;17"] ne correspond pas à ce motif
 --> main.ry:2:1
  |
2 | [nom, note] = split(ligne, ",")
  | ^^^^^^^^^^^
  |
  = aide: utilisez `match` pour traiter les différentes formes possibles d'une valeur
  = en savoir plus : `rythme explain E006`
```

La ligne est séparée par des `;`, pas des `,` : `split` donne un seul
morceau, et `[nom, note]` en attend deux. Regardez votre fichier et
écrivez `split(ligne, ";")`.

## Testez-vous

**Question 1 :** quelle est la différence entre `write_file` et
`append_file` ?

> **Réponse**
>
> `write_file` remplace tout le contenu du fichier ; `append_file` ajoute
> à la fin, sans effacer ce qui y est déjà.

**Question 2 :** que donne `read_file("absent.txt")` si le fichier n'existe
pas ?

> **Réponse**
>
> `Err("fichier introuvable : absent.txt")`. Le programme ne s'arrête pas :
> c'est à vous de traiter ce cas.

**Question 3 :** le programme `notes.ry` est lancé avec
`rythme run notes.ry maths 2024`. Que vaut `args()` ?

> **Réponse**
>
> `["maths", "2024"]` : une liste de textes, sans le nom du fichier.

## En résumé

- `input("question ")` lit une ligne au clavier, toujours sous forme de
  texte ; `args()` donne les mots de la ligne de commande.
- `write_file` remplace le contenu d'un fichier, `append_file` ajoute à la
  fin, `file_exists` dit s'il existe.
- `read_file` donne un `Result` : `Ok(texte)` ou `Err(message)`. Dans une
  fonction, `?` transmet l'erreur.
- Pour traiter un fichier, on le découpe avec `lines`, puis `split` pour les
  colonnes d'un CSV.
- `to_json` et `parse_json` écrivent et relisent des données en JSON.

Suite : [Données JSON](formats.md)
