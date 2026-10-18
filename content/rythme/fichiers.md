# Lire et écrire : clavier et fichiers

Cette page montre comment un programme Rythme reçoit des informations (le clavier, la ligne de commande) et comment il garde ses résultats dans des fichiers : texte, lignes, CSV et JSON.

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

Suite : [projets, modules et tests](projet.md). Comparez avec les fichiers en
[Ruby](../ruby/fichiers.md) et en [Rust](../rust/fichiers.md).
