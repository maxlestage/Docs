# Projets pas à pas

Cette page vous guide dans trois petits projets complets, construits étape par étape : un jeu, une liste de tâches enregistrée dans un fichier et un bulletin de notes.

> **Objectifs :**
>
> - découper un programme en petites étapes qui marchent chacune ;
> - réutiliser ce que vous avez appris : conditions, boucles, méthodes, tableaux, hashs ;
> - enregistrer des données dans un fichier JSON ;
> - lire des données en texte, calculer et afficher un rapport bien aligné.

## Comment suivre ces projets

Chaque projet avance par petites étapes. À chaque étape :

1. lisez le code et l'explication ;
2. tapez le code dans un fichier (par exemple `jeu.rb`) et lancez-le avec `ruby jeu.rb` ;
3. comparez avec la sortie affichée ;
4. modifiez une valeur pour voir ce qui change.

> **Astuce :** ne copiez pas tout d'un coup. Un programme se construit morceau par
> morceau, en le lançant souvent. C'est ainsi que travaillent les programmeurs.

## Projet 1 : Le juste nombre

Le programme choisit un nombre secret entre 1 et 100. Le joueur propose des nombres ;
le programme répond « trop petit » ou « trop grand », jusqu'à ce que le joueur trouve.

### Étape 1 : comparer un essai

Commencez par le cœur du jeu : comparer **un** essai au nombre secret.

```ruby
secret = 42
essai = 30

if essai < secret
  puts "Trop petit !"
elsif essai > secret
  puts "Trop grand !"
else
  puts "Gagné !"
end
```

```sortie
Trop petit !
```

Changez `essai` en `50`, puis en `42`, et relancez.

### Étape 2 : une méthode pour l'indice

Rangez cette comparaison dans une méthode. Elle **renvoie** le texte au lieu de
l'afficher : elle sera plus facile à réutiliser.

```ruby
def indice(secret, essai)
  if essai < secret
    "Trop petit !"
  elsif essai > secret
    "Trop grand !"
  else
    "Gagné !"
  end
end

puts indice(42, 30)
puts indice(42, 50)
puts indice(42, 42)
```

```sortie
Trop petit !
Trop grand !
Gagné !
```

### Étape 3 : plusieurs essais et un compteur

Le joueur fait plusieurs essais. Pour l'instant, ses essais sont dans un tableau : le
programme donne ainsi toujours le même résultat, et vous pouvez le tester facilement.
`each_with_index` donne le numéro de l'essai ; `break` arrête la boucle quand c'est
gagné.

```ruby
def indice(secret, essai)
  if essai < secret
    "Trop petit !"
  elsif essai > secret
    "Trop grand !"
  else
    "Gagné !"
  end
end

secret = 42
essais = [50, 25, 40, 42, 10]

essais.each_with_index do |essai, i|
  puts "Essai #{i + 1} : #{essai} -> #{indice(secret, essai)}"
  if essai == secret
    puts "Bravo, trouvé en #{i + 1} essais !"
    break
  end
end
```

```sortie
Essai 1 : 50 -> Trop grand !
Essai 2 : 25 -> Trop petit !
Essai 3 : 40 -> Trop petit !
Essai 4 : 42 -> Gagné !
Bravo, trouvé en 4 essais !
```

Le dernier essai (`10`) n'est jamais lu : `break` a arrêté la boucle avant.

### Étape 4 : limiter le nombre d'essais

Pour corser le jeu, le joueur n'a droit qu'à 5 essais. Rangez la partie dans une
méthode `jouer` qui renvoie `true` si le joueur gagne. `first(5)` ne garde que les
5 premiers essais.

```ruby
def indice(secret, essai)
  if essai < secret
    "Trop petit !"
  elsif essai > secret
    "Trop grand !"
  else
    "Gagné !"
  end
end

def jouer(secret, essais, maximum: 5)
  essais.first(maximum).each_with_index do |essai, i|
    puts "Essai #{i + 1} : #{essai} -> #{indice(secret, essai)}"
    return true if essai == secret
  end
  false
end

if jouer(42, [50, 25, 40, 42])
  puts "Bravo !"
end

puts "---"

unless jouer(7, [50, 25, 12, 10, 8, 7])
  puts "Perdu ! Le nombre était 7."
end
```

```sortie
Essai 1 : 50 -> Trop grand !
Essai 2 : 25 -> Trop petit !
Essai 3 : 40 -> Trop petit !
Essai 4 : 42 -> Gagné !
Bravo !
---
Essai 1 : 50 -> Trop grand !
Essai 2 : 25 -> Trop grand !
Essai 3 : 12 -> Trop grand !
Essai 4 : 10 -> Trop grand !
Essai 5 : 8 -> Trop grand !
Perdu ! Le nombre était 7.
```

> **Note :** `return true` quitte la méthode tout de suite, en pleine boucle. Si la
> boucle se termine sans victoire, la méthode arrive à la dernière ligne et renvoie
> `false`.

### Étape 5 : la vraie version, au clavier

Il reste deux changements pour un vrai jeu :

- `rand(1..100)` tire un nombre au hasard entre 1 et 100 ;
- `gets.chomp.to_i` lit l'essai tapé par le joueur (voir
  [lire le clavier](fichiers.md#lire-le-clavier-avec-gets)).

Ce code attend le clavier : il n'est pas exécuté ici. Lancez-le chez vous.

```ruby norun
# jeu.rb
def indice(secret, essai)
  if essai < secret
    "Trop petit !"
  elsif essai > secret
    "Trop grand !"
  else
    "Gagné !"
  end
end

secret = rand(1..100)
puts "J'ai choisi un nombre entre 1 et 100. Vous avez 5 essais."

gagne = false
5.times do |i|
  print "Essai #{i + 1} : "
  essai = gets.chomp.to_i
  puts indice(secret, essai)
  if essai == secret
    gagne = true
    break
  end
end

puts gagne ? "Bravo !" : "Perdu ! Le nombre était #{secret}."
```

Une partie peut ressembler à ceci :

```text
J'ai choisi un nombre entre 1 et 100. Vous avez 5 essais.
Essai 1 : 50
Trop grand !
Essai 2 : 25
Trop petit !
Essai 3 : 37
Gagné !
Bravo !
```

### Pour aller plus loin

- Demandez au joueur s'il veut rejouer (`gets.chomp == "o"`) et entourez la partie d'une
  boucle `loop do … end`.
- Refusez un essai hors de 1..100 sans le compter.
- Retenez le meilleur score (le plus petit nombre d'essais) dans un fichier.
- Écrivez des tests pour `indice` avec Minitest (voir [tester son code](tests.md)).

## Projet 2 : Une liste de tâches

Une petite application en ligne de commande : ajouter une tâche, afficher la liste,
marquer une tâche comme faite. Les tâches sont enregistrées dans un fichier JSON :
elles sont toujours là au prochain lancement.

### Étape 1 : représenter les tâches

Une tâche est un hash avec deux clés : son `"titre"` et `"faite"` (vrai ou faux). La
liste est un tableau de ces hashs.

```ruby
taches = [
  { "titre" => "Acheter du pain", "faite" => false },
  { "titre" => "Appeler Léa", "faite" => true }
]

taches.each_with_index do |tache, i|
  case_a_cocher = tache["faite"] ? "[x]" : "[ ]"
  puts "#{i + 1}. #{case_a_cocher} #{tache["titre"]}"
end
```

```sortie
1. [ ] Acheter du pain
2. [x] Appeler Léa
```

> **Note :** les clés sont des textes (`"titre"`) et non des symboles (`:titre`). Ainsi,
> elles restent les mêmes après un passage par JSON (voir
> [lire du JSON](fichiers.md#lire-du-json)).

### Étape 2 : ajouter, afficher, terminer

Chaque action devient une méthode. `afficher` gère aussi la liste vide.

```ruby
def ajouter(taches, titre)
  taches << { "titre" => titre, "faite" => false }
end

def terminer(taches, numero)
  tache = taches[numero - 1]
  if tache
    tache["faite"] = true
  else
    puts "Pas de tâche numéro #{numero}."
  end
end

def afficher(taches)
  if taches.empty?
    puts "Aucune tâche."
    return
  end

  taches.each_with_index do |tache, i|
    case_a_cocher = tache["faite"] ? "[x]" : "[ ]"
    puts "#{i + 1}. #{case_a_cocher} #{tache["titre"]}"
  end
end

taches = []
afficher(taches)
ajouter(taches, "Acheter du pain")
ajouter(taches, "Réviser Ruby")
terminer(taches, 1)
terminer(taches, 5)
afficher(taches)
```

```sortie
Aucune tâche.
Pas de tâche numéro 5.
1. [x] Acheter du pain
2. [ ] Réviser Ruby
```

> **Attention :** le joueur compte à partir de 1, Ruby à partir de 0. D'où
> `taches[numero - 1]`.

### Étape 3 : enregistrer dans un fichier JSON

Pour garder les tâches, écrivez-les dans `taches.json`. Au lancement suivant, relisez
le fichier. S'il n'existe pas encore, partez d'une liste vide.

```ruby
require "json"

FICHIER = "taches.json"

def charger
  return [] unless File.exist?(FICHIER)

  JSON.parse(File.read(FICHIER))
end

def sauvegarder(taches)
  File.write(FICHIER, JSON.pretty_generate(taches))
end

# Premier lancement : le fichier n'existe pas encore.
taches = charger
p taches
taches << { "titre" => "Acheter du pain", "faite" => false }
sauvegarder(taches)

# Lancement suivant : la tâche est relue depuis le fichier.
p charger
puts File.read(FICHIER)
```

```sortie
[]
[{"titre"=>"Acheter du pain", "faite"=>false}]
[
  {
    "titre": "Acheter du pain",
    "faite": false
  }
]
```

`JSON.pretty_generate` écrit un JSON aéré, facile à lire et à corriger à la main.

### Étape 4 : le programme complet

Réunissez tout. Une méthode `executer` reçoit une commande (`"ajouter"`, `"liste"`,
`"faite"`) et son argument. Chaque commande charge les tâches, agit, puis enregistre.
Ici, plusieurs commandes sont lancées à la suite, comme si l'utilisateur lançait le
programme plusieurs fois.

```ruby
require "json"

FICHIER = "taches.json"

def charger
  return [] unless File.exist?(FICHIER)

  JSON.parse(File.read(FICHIER))
end

def sauvegarder(taches)
  File.write(FICHIER, JSON.pretty_generate(taches))
end

def afficher(taches)
  if taches.empty?
    puts "Aucune tâche."
    return
  end

  taches.each_with_index do |tache, i|
    case_a_cocher = tache["faite"] ? "[x]" : "[ ]"
    puts "#{i + 1}. #{case_a_cocher} #{tache["titre"]}"
  end
end

def executer(commande, argument = nil)
  taches = charger

  case commande
  when "ajouter"
    taches << { "titre" => argument, "faite" => false }
    puts "Ajouté : #{argument}"
  when "faite"
    tache = taches[argument.to_i - 1]
    if tache
      tache["faite"] = true
      puts "Bravo : #{tache["titre"]}"
    else
      puts "Pas de tâche numéro #{argument}."
    end
  when "liste"
    afficher(taches)
  else
    puts "Commandes : ajouter TITRE, liste, faite NUMÉRO"
  end

  sauvegarder(taches)
end

executer("liste")
executer("ajouter", "Acheter du pain")
executer("ajouter", "Réviser Ruby")
executer("faite", "2")
executer("liste")
executer("aide")
```

```sortie
Aucune tâche.
Ajouté : Acheter du pain
Ajouté : Réviser Ruby
Bravo : Réviser Ruby
1. [ ] Acheter du pain
2. [x] Réviser Ruby
Commandes : ajouter TITRE, liste, faite NUMÉRO
```

### Étape 5 : depuis le terminal avec ARGV

Pour l'utiliser vraiment, remplacez les appels de test de la fin par une seule ligne.
`ARGV` contient les mots tapés après le nom du programme (voir
[les arguments](fichiers.md#les-arguments-avec-argv)).

```ruby norun
# À la fin de taches.rb, à la place des appels executer(...)
executer(ARGV[0], ARGV[1])
```

```sh
ruby taches.rb ajouter "Acheter du pain"
ruby taches.rb ajouter "Réviser Ruby"
ruby taches.rb faite 1
ruby taches.rb liste
```

```text
Ajouté : Acheter du pain
Ajouté : Réviser Ruby
Bravo : Acheter du pain
1. [x] Acheter du pain
2. [ ] Réviser Ruby
```

> **Astuce :** les guillemets autour de `"Acheter du pain"` en font **un seul**
> argument. Sans eux, `ARGV[1]` vaudrait seulement `"Acheter"`.

### Pour aller plus loin

- Ajoutez une commande `supprimer NUMÉRO` avec `taches.delete_at(numero - 1)`.
- Ajoutez une commande `menage` qui retire toutes les tâches faites (`reject!`).
- Affichez à la fin de la liste : « 1 tâche sur 2 terminée ».
- Transformez le tout en une classe `ListeDeTaches` (voir
  [classes et objets](types.md)) et testez-la avec Minitest.

## Projet 3 : Le bulletin de notes

Le programme lit les notes d'une classe, écrites en texte au format CSV. Il calcule la
moyenne de chaque élève et de chaque matière, trouve le meilleur élève, puis affiche un
bulletin bien aligné.

### Étape 1 : lire les données

Les notes sont dans un texte CSV : une ligne de titres, puis une ligne par élève. Un
texte sur plusieurs lignes s'écrit avec `<<~CSV … CSV` (un *heredoc*).

```ruby
donnees = <<~CSV
  nom,maths,francais,histoire
  Alice,15,12,17
  Bob,9,14,11
  Chloé,18,16,13
CSV

lignes = donnees.lines(chomp: true)
titres = lignes.first.split(",")
p titres

eleves = lignes.drop(1).map do |ligne|
  nom, *notes = ligne.split(",")
  { nom: nom, notes: notes.map(&:to_i) }
end
p eleves.first
```

```sortie
["nom", "maths", "francais", "histoire"]
{:nom=>"Alice", :notes=>[15, 12, 17]}
```

- `lines(chomp: true)` découpe le texte en lignes, sans les retours à la ligne.
- `nom, *notes = …` range le premier morceau dans `nom` et **tous les autres** dans le
  tableau `notes`.
- `map(&:to_i)` transforme les textes `"15"` en nombres `15`.

> **Note :** dans un vrai programme, le texte viendrait d'un fichier :
> `donnees = File.read("notes.csv")`.

### Étape 2 : la moyenne de chaque élève

Une moyenne, c'est la somme divisée par le nombre de notes. `to_f` évite la division
entière (`44 / 3` vaut `14`, mais `44.0 / 3` vaut `14.666…`).

```ruby
def moyenne(notes)
  (notes.sum.to_f / notes.size).round(1)
end

eleves = [
  { nom: "Alice", notes: [15, 12, 17] },
  { nom: "Bob", notes: [9, 14, 11] },
  { nom: "Chloé", notes: [18, 16, 13] }
]

eleves.each do |eleve|
  puts "#{eleve[:nom]} : #{moyenne(eleve[:notes])}"
end
```

```sortie
Alice : 14.7
Bob : 11.3
Chloé : 15.7
```

### Étape 3 : le meilleur élève et les moyennes par matière

`max_by` trouve l'élève qui a la plus grande moyenne. Pour une matière, prenez la note
numéro `i` de chaque élève.

```ruby
def moyenne(notes)
  (notes.sum.to_f / notes.size).round(1)
end

matieres = ["maths", "francais", "histoire"]
eleves = [
  { nom: "Alice", notes: [15, 12, 17] },
  { nom: "Bob", notes: [9, 14, 11] },
  { nom: "Chloé", notes: [18, 16, 13] }
]

meilleur = eleves.max_by { |eleve| moyenne(eleve[:notes]) }
puts "Meilleure moyenne : #{meilleur[:nom]}"

matieres.each_with_index do |matiere, i|
  notes = eleves.map { |eleve| eleve[:notes][i] }
  puts "#{matiere} : #{moyenne(notes)}"
end
```

```sortie
Meilleure moyenne : Chloé
maths : 14.0
francais : 14.0
histoire : 13.7
```

### Étape 4 : un bulletin bien aligné

Pour un tableau lisible, chaque colonne doit avoir la même largeur. `ljust(8)` complète
un texte avec des espaces jusqu'à 8 caractères ; `format("%5.1f", x)` écrit un nombre
sur 5 caractères avec un chiffre après la virgule.

```ruby
def moyenne(notes)
  (notes.sum.to_f / notes.size).round(1)
end

def mention(note)
  if note >= 14
    "Bien"
  elsif note >= 10
    "Passable"
  else
    "Insuffisant"
  end
end

donnees = <<~CSV
  nom,maths,francais,histoire
  Alice,15,12,17
  Bob,9,14,11
  Chloé,18,16,13
CSV

lignes = donnees.lines(chomp: true)
eleves = lignes.drop(1).map do |ligne|
  nom, *notes = ligne.split(",")
  { nom: nom, notes: notes.map(&:to_i) }
end

puts "BULLETIN DE LA CLASSE"
puts "=" * 30
eleves.sort_by { |eleve| -moyenne(eleve[:notes]) }.each do |eleve|
  m = moyenne(eleve[:notes])
  puts "#{eleve[:nom].ljust(8)}#{format("%5.1f", m)}   #{mention(m)}"
end
puts "=" * 30

toutes = eleves.flat_map { |eleve| eleve[:notes] }
puts "Moyenne de la classe : #{moyenne(toutes)}"
meilleur = eleves.max_by { |eleve| moyenne(eleve[:notes]) }
puts "Félicitations à #{meilleur[:nom]} !"
```

```sortie
BULLETIN DE LA CLASSE
==============================
Chloé    15.7   Bien
Alice    14.7   Bien
Bob      11.3   Passable
==============================
Moyenne de la classe : 13.9
Félicitations à Chloé !
```

- `sort_by { |eleve| -moyenne(…) }` trie de la plus grande moyenne à la plus petite
  (le signe `-` inverse l'ordre).
- `flat_map` rassemble toutes les notes de tous les élèves dans un seul tableau.
- `"=" * 30` répète le texte `"="` 30 fois.

### Pour aller plus loin

- Lisez les notes depuis un vrai fichier `notes.csv` avec `File.read`.
- Enregistrez le bulletin dans `bulletin.txt` avec `File.write`.
- Ignorez les notes vides (une case `,,` dans le CSV) avec `reject(&:empty?)`.
- Affichez pour chaque matière la meilleure note et le nom de l'élève qui l'a obtenue.

## Erreurs fréquentes

### Oublier to_i après split ou gets

`split` et `gets` donnent du **texte**. Additionner des textes ne fait pas une somme.

```ruby norun
notes = "15,12,17".split(",")
puts notes.sum
```

```text
main.rb:2:in `+': String can't be coerced into Integer (TypeError)
```

Transformez d'abord en nombres : `notes.map(&:to_i).sum`.

### Lire les clés JSON avec des symboles

Après `JSON.parse`, les clés sont des textes. Avec un symbole, vous obtenez `nil`.

```ruby norun
require "json"

tache = JSON.parse('{"titre": "Acheter du pain"}')
puts tache[:titre].upcase
```

```text
main.rb:4:in `<main>': undefined method `upcase' for nil (NoMethodError)
```

Écrivez `tache["titre"]`.

### Un numéro de tâche décalé

L'utilisateur tape `1` pour la première tâche, mais Ruby compte à partir de 0. Sans
`- 1`, vous terminez la mauvaise tâche, ou vous obtenez `nil` pour la dernière.

```ruby norun
taches = [{ "titre" => "Pain", "faite" => false }]
taches[1]["faite"] = true
```

```text
main.rb:2:in `<main>': undefined method `[]=' for nil (NoMethodError)
```

Écrivez `taches[numero - 1]`, et vérifiez que la tâche existe avant de la modifier.

### La division entière

Entre deux entiers, `/` donne un entier : la moyenne est fausse.

```ruby
puts 44 / 3
puts 44.0 / 3
```

```sortie
14
14.666666666666666
```

Ajoutez `to_f` avant de diviser : `notes.sum.to_f / notes.size`.

## Testez-vous

**1.** Dans le juste nombre, pourquoi utiliser un tableau d'essais plutôt que `gets`
pendant la mise au point ?

> **Réponse**
>
> Le programme donne toujours le même résultat, sans rien taper : on peut le relancer
> et le vérifier vite. On passe à `gets` une fois que tout marche.

**2.** Que fait `nom, *notes = "Alice,15,12".split(",")` ?

> **Réponse**
>
> `nom` reçoit `"Alice"` et `notes` reçoit le tableau de tous les autres morceaux :
> `["15", "12"]`.

**3.** Pourquoi la liste de tâches vérifie-t-elle `File.exist?` avant de lire le fichier ?

> **Réponse**
>
> Au tout premier lancement, `taches.json` n'existe pas encore. Le lire provoquerait une
> erreur ; on part alors d'une liste vide.

## En résumé

- Un projet se construit par petites étapes, en lançant le programme à chaque fois.
- Remplacer `gets` par des valeurs fixes rend un programme facile à vérifier.
- Des méthodes courtes (`indice`, `ajouter`, `moyenne`) rendent le code clair et
  testable.
- `JSON.pretty_generate` et `JSON.parse` enregistrent et relisent des données.
- `ljust` et `format` alignent un rapport en colonnes.

Suite : entraînez-vous avec [les exercices corrigés](exercices.md). Comparez avec
[les projets en Rust](../rust/projets.md) et [en Rythme](../rythme/projets.md).
