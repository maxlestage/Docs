# Lire et écrire : clavier et fichiers

Cette page montre comment lire ce que tape l'utilisateur, et comment écrire puis relire des fichiers.

> **Objectifs :**
>
> - vous saurez lire une ligne tapée au clavier avec `gets` ;
> - vous saurez récupérer les arguments du programme avec `ARGV` ;
> - vous saurez écrire, lire et compléter un fichier texte ;
> - vous saurez lister les fichiers d'un dossier ;
> - vous saurez lire et écrire des données aux formats CSV et JSON.

## Lire le clavier avec gets

`gets` (*get string*, « obtenir un texte ») attend que l'utilisateur tape une ligne et
appuie sur Entrée. Il renvoie le texte tapé. Le programme est en pause pendant ce
temps, comme un serveur qui attend votre commande.

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

`gets` renvoie toujours du texte. Pour un nombre, ajoutez `to_i` (entier) ou `to_f`
(nombre à virgule).

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

Vous pouvez donner des valeurs au programme au moment de le lancer, après le nom du
fichier. Elles arrivent dans le tableau `ARGV` (*argument vector*, « liste des
arguments »), une par case.

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

Un **fichier** est un texte (ou des données) rangé sur le disque, sous un nom. Il
survit à la fin du programme, contrairement aux variables. Un nom simple comme
`"message.txt"` désigne un fichier du **dossier courant** : celui où vous avez lancé
`ruby`. On peut aussi donner un **chemin**, comme `"donnees/notes.txt"`.

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

`File.write` n'ajoute pas de retour à la ligne : écrivez `\n` vous-même à la fin de
chaque ligne.

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

## Ouvrir un fichier avec File.open

`File.open` ouvre un fichier et vous le donne dans un bloc. Le deuxième argument est le
**mode** : ce que vous comptez faire.

| Mode | Sens | Si le fichier existe |
|------|------|----------------------|
| `"r"` | lire (le mode par défaut) | il est lu |
| `"w"` | écrire (*write*) | son contenu est effacé |
| `"a"` | ajouter à la fin (*append*) | on écrit après l'ancien contenu |

Dans le bloc, `f.puts` écrit une ligne dans le fichier, comme `puts` à l'écran.

```ruby
File.open("liste.txt", "w") do |f|
  f.puts "pain"
  f.puts "lait"
end
puts File.read("liste.txt")
```

```sortie
pain
lait
```

## Ajouter à la fin d'un fichier

Pour ajouter des lignes sans effacer les anciennes, ouvrez le fichier en mode `"a"`
(comme « ajouter »).

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

## Les dossiers

`Dir` (*directory*, « dossier ») s'occupe des dossiers :

- `Dir.mkdir` crée un dossier ;
- `Dir.exist?` vérifie qu'il existe ;
- `Dir.glob` liste les fichiers dont le nom correspond à un motif : `*` veut dire
  « n'importe quel texte ».

```ruby
Dir.mkdir("notes") unless Dir.exist?("notes")
File.write("notes/lundi.txt", "vélo")
File.write("notes/mardi.txt", "piscine")
File.write("notes/photo.jpg", "")

p Dir.glob("notes/*.txt").sort
puts File.basename("notes/lundi.txt")
File.delete("notes/photo.jpg")
p Dir.children("notes").sort
```

```sortie
["notes/lundi.txt", "notes/mardi.txt"]
lundi.txt
["lundi.txt", "mardi.txt"]
```

- `File.basename` garde seulement le nom du fichier, sans le dossier.
- `File.delete` supprime un fichier : il ne passe pas par la corbeille !
- `Dir.children` liste tout ce que contient un dossier.

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

**JSON** est un format de texte très répandu pour ranger des données : il ressemble
beaucoup aux hashs et aux tableaux de Ruby. Il fait partie de la **bibliothèque
standard** : il est livré avec Ruby, mais il faut le charger avec `require "json"`.

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

Pour obtenir des clés symboles, ajoutez `symbolize_names: true`. Pour un fichier facile
à lire par un humain, écrivez avec `JSON.pretty_generate`.

```ruby
require "json"

scores = { alice: 42, bob: 17 }
File.write("scores.json", JSON.pretty_generate(scores))
puts File.read("scores.json")

relu = JSON.parse(File.read("scores.json"), symbolize_names: true)
puts relu[:alice]
```

```sortie
{
  "alice": 42,
  "bob": 17
}
42
```

## Erreurs fréquentes

### Lire un fichier qui n'existe pas

```ruby norun
puts File.read("notes.txt")
```

```text
main.rb:1:in `read': No such file or directory @ rb_sysopen - notes.txt (Errno::ENOENT)
	from main.rb:1:in `<main>'
```

Le fichier est introuvable (*No such file or directory*). Vérifiez le nom, et le
dossier depuis lequel vous lancez le programme. Testez avec `File.exist?`, ou attrapez
l'erreur :

```ruby
begin
  puts File.read("notes.txt")
rescue Errno::ENOENT
  puts "Pas encore de notes."
end
```

```sortie
Pas encore de notes.
```

### Oublier require "json"

```ruby norun
joueur = JSON.parse("{}")
```

```text
main.rb:1:in `<main>': uninitialized constant JSON (NameError)

joueur = JSON.parse("{}")
         ^^^^
```

Ruby ne connaît pas `JSON` tant qu'il n'est pas chargé. Ajoutez `require "json"` en
haut du fichier.

### Écrire dans un fichier ouvert en lecture

Ici, le fichier `journal.txt` existe déjà.

```ruby norun
File.open("journal.txt") do |f|
  f.puts "mardi"
end
```

```text
main.rb:2:in `write': not opened for writing (IOError)
	from main.rb:2:in `puts'
	from main.rb:2:in `block in <main>'
	from main.rb:1:in `open'
	from main.rb:1:in `<main>'
```

Sans mode, `File.open` ouvre en lecture seule (`"r"`). Donnez le mode : `"w"` pour
remplacer, `"a"` pour ajouter.

## Testez-vous

**Question 1 :** quelle différence entre `File.write("a.txt", "x")` et
`File.open("a.txt", "a") { |f| f.write("x") }` ?

> **Réponse**
>
> `File.write` remplace tout le contenu. Le mode `"a"` ajoute `x` à la fin, sans
> effacer l'ancien contenu.

**Question 2 :** l'utilisateur tape `12`. Que vaut `gets.chomp` ? Et `gets.chomp.to_i` ?

> **Réponse**
>
> `"12"` (un texte), puis `12` (un entier).

**Question 3 :** comment compter les lignes d'un fichier `courses.txt` ?

> **Réponse**
>
> ```ruby
> File.write("courses.txt", "pain\nlait\n")
> puts File.readlines("courses.txt").length
> ```
>
> ```sortie
> 2
> ```

## En résumé

- `gets.chomp` lit une ligne tapée au clavier ; `ARGV` contient les arguments du programme.
- `File.write` écrit (et remplace), `File.read` lit tout le fichier.
- `File.readlines` et `File.foreach` lisent ligne par ligne.
- `File.open` prend un mode : `"r"` lit, `"w"` remplace, `"a"` ajoute à la fin.
- `File.exist?` vérifie qu'un fichier existe ; `Dir.glob` liste les fichiers d'un dossier.
- Un CSV simple se lit avec `split(",")`, un JSON avec `JSON.parse`.

Suite : [les tests](tests.md). Comparez avec
[les fichiers en Rust](../rust/fichiers.md).
