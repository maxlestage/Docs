# Lire et écrire : clavier et fichiers

Cette page montre comment lire ce que tape l'utilisateur, et comment écrire puis relire des fichiers.

## Lire le clavier avec gets

`gets` attend que l'utilisateur tape une ligne et appuie sur Entrée. Il renvoie le texte
tapé.

```ruby norun
print "Votre prénom : "
prenom = gets.chomp
puts "Bonjour, #{prenom} !"
```

Dans le terminal, cela donne :

```text
Votre prénom : Alice
Bonjour, Alice !
```

> **À retenir :** `gets` garde le retour à la ligne de la touche Entrée (`"Alice\n"`).
> `chomp` l'enlève. Écrivez presque toujours `gets.chomp`.

### Lire un nombre

`gets` renvoie toujours du texte. Pour un nombre, ajoutez `to_i`.

```ruby norun
print "Votre âge : "
age = gets.chomp.to_i
puts "Dans 10 ans, vous aurez #{age + 10} ans."
```

```text
Votre âge : 30
Dans 10 ans, vous aurez 40 ans.
```

## Les arguments avec ARGV

Vous pouvez donner des valeurs au programme au moment de le lancer. Elles arrivent dans
le tableau `ARGV`.

```ruby norun
# saluer.rb
puts "Bonjour, #{ARGV[0]} !"
```

```sh
ruby saluer.rb Alice
```

```text
Bonjour, Alice !
```

Sans argument, `ARGV` est vide. Prévoyez une valeur par défaut avec `||` :

```ruby
nom = ARGV[0] || "inconnu"
puts "Bonjour, #{nom} !"
puts "Nombre d'arguments : #{ARGV.length}"
```

```sortie
Bonjour, inconnu !
Nombre d'arguments : 0
```

> **Note :** comme `gets`, `ARGV` contient du texte. Utilisez `ARGV[0].to_i` pour un
> nombre.

## Écrire un fichier

`File.write` crée un fichier et y écrit du texte. Si le fichier existe déjà, son contenu
est **remplacé**.

```ruby
File.write("message.txt", "Bonjour !\n")
puts "Fichier écrit."
```

```sortie
Fichier écrit.
```

> **Attention :** `File.write` efface l'ancien contenu sans prévenir.

## Lire un fichier

### Tout d'un coup

`File.read` renvoie tout le contenu du fichier, en un seul texte.

```ruby
File.write("message.txt", "Bonjour !\nÀ bientôt.\n")
contenu = File.read("message.txt")
puts contenu
```

```sortie
Bonjour !
À bientôt.
```

### Ligne par ligne

`File.readlines` renvoie un tableau, avec une case par ligne. L'option `chomp: true`
enlève les retours à la ligne.

```ruby
File.write("courses.txt", "pain\nlait\noeufs\n")
lignes = File.readlines("courses.txt", chomp: true)
p lignes
puts "#{lignes.length} articles"
```

```sortie
["pain", "lait", "oeufs"]
3 articles
```

`File.foreach` lit le fichier une ligne à la fois. Il convient aux très gros fichiers.

```ruby
File.write("courses.txt", "pain\nlait\noeufs\n")
File.foreach("courses.txt") do |ligne|
  puts "- #{ligne.chomp}"
end
```

```sortie
- pain
- lait
- oeufs
```

## Vérifier qu'un fichier existe

Lire un fichier absent provoque une erreur. `File.exist?` permet de vérifier avant.

```ruby
puts File.exist?("absent.txt")
File.write("present.txt", "ok")
puts File.exist?("present.txt")
```

```sortie
false
true
```

```ruby
if File.exist?("notes.txt")
  puts File.read("notes.txt")
else
  puts "Pas encore de notes."
end
```

```sortie
Pas encore de notes.
```

> **Astuce :** vous pouvez aussi attraper l'erreur avec `rescue Errno::ENOENT`. Voir
> [gérer les erreurs](erreurs.md).

## Ajouter à la fin d'un fichier

Pour ajouter des lignes sans effacer les anciennes, ouvrez le fichier en mode `"a"`
(comme « ajouter »). Dans le bloc, `f.puts` écrit une ligne dans le fichier.

```ruby
File.write("journal.txt", "lundi : vélo\n")
File.open("journal.txt", "a") do |f|
  f.puts "mardi : piscine"
  f.puts "mercredi : repos"
end
puts File.read("journal.txt")
```

```sortie
lundi : vélo
mardi : piscine
mercredi : repos
```

> **À retenir :** avec un bloc, `File.open` ferme le fichier tout seul à la fin.

## Un fichier CSV simple

Un fichier **CSV** est un tableau écrit en texte : une ligne par rangée, des virgules
entre les colonnes. Pour un fichier simple, `split(",")` suffit.

```ruby
File.write("notes.csv", "nom,note\nAlice,15\nBob,12\n")

lignes = File.readlines("notes.csv", chomp: true)
lignes.drop(1).each do |ligne|
  nom, note = ligne.split(",")
  puts "#{nom} a eu #{note}/20"
end
```

```sortie
Alice a eu 15/20
Bob a eu 12/20
```

- `drop(1)` saute la première ligne, celle des titres.
- `nom, note = ...` range les deux morceaux dans deux variables.

> **Note :** si une valeur contient elle-même une virgule, `split` ne suffit plus. Ruby
> propose alors la bibliothèque `csv`.

## Le format JSON

**JSON** est un format de texte très répandu pour ranger des données. Chargez-le avec
`require "json"`.

### Écrire du JSON

`JSON.generate` transforme un hash en texte JSON.

```ruby
require "json"

joueur = { "nom" => "Alice", "score" => 42 }
texte = JSON.generate(joueur)
puts texte
File.write("joueur.json", texte)
```

```sortie
{"nom":"Alice","score":42}
```

### Lire du JSON

`JSON.parse` fait l'inverse : il transforme un texte JSON en hash.

```ruby
require "json"

File.write("joueur.json", '{"nom":"Alice","score":42}')
joueur = JSON.parse(File.read("joueur.json"))
puts joueur["nom"]
puts joueur["score"] + 1
```

```sortie
Alice
43
```

> **Attention :** après `JSON.parse`, les clés sont des textes : écrivez `joueur["nom"]`,
> pas `joueur[:nom]`.

## En résumé

- `gets.chomp` lit une ligne tapée au clavier ; `ARGV` contient les arguments du programme.
- `File.write` écrit (et remplace), `File.read` lit tout le fichier.
- `File.readlines` et `File.foreach` lisent ligne par ligne.
- `File.exist?` vérifie qu'un fichier existe ; `File.open(nom, "a")` ajoute à la fin.
- Un CSV simple se lit avec `split(",")`, un JSON avec `JSON.parse`.

Suite : [projets, gems et tests](projet.md). Comparez avec
[les fichiers en Rust](../rust/fichiers.md).
