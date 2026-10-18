# Aide-mémoire

Cette page résume en un coup d'œil la syntaxe Ruby vue dans toute la documentation.

## Bases

| Élément | Exemple |
| --- | --- |
| Variable | `age = 30` |
| Constante | `TAUX = 0.2` |
| Chaîne | `"Bonjour #{nom}"` |
| Symbole | `:rouge` |
| Booléens | `true`, `false` |
| Rien | `nil` |
| Commentaire | `# texte` |
| Afficher | `puts`, `print`, `p` |

```ruby
nom = "Alice"
puts "Bonjour #{nom}"
p :ok
p 7 / 2
p 7.0 / 2
```

```sortie
Bonjour Alice
:ok
3
3.5
```

Détails : [premiers pas](premiers-pas.md).

## Conditions et boucles

| Besoin | Syntaxe |
| --- | --- |
| Condition | `if … elsif … else … end` |
| Condition inverse | `unless … end` |
| En une ligne | `puts "ok" if x > 0` |
| Ternaire | `x > 0 ? "oui" : "non"` |
| Plusieurs cas | `case x when 1 then … end` |
| Boucles | `while`, `until`, `n.times`, `each` |
| Intervalles | `1..5` (inclus), `1...5` (exclu) |
| Contrôle | `break`, `next` |

```ruby
(1..5).each do |n|
  next if n == 2
  puts n.even? ? "#{n} pair" : "#{n} impair"
end
```

```sortie
1 impair
3 impair
4 pair
5 impair
```

Détails : [conditions et boucles](conditions-boucles.md).

## Méthodes

```ruby
def saluer(nom, salut = "Bonjour", poli: false)
  texte = "#{salut} #{nom}"
  poli ? "#{texte}, enchanté." : texte
end

puts saluer("Alice")
puts saluer("Bob", "Salut", poli: true)
```

```sortie
Bonjour Alice
Salut Bob, enchanté.
```

| Convention | Sens |
| --- | --- |
| `vide?` | renvoie vrai ou faux |
| `trier!` | modifie l'objet |
| dernière ligne | valeur renvoyée |
| `yield` | exécute le bloc reçu |

Détails : [les méthodes](fonctions.md).

## Texte

| Besoin | Exemple |
| --- | --- |
| Insérer une valeur | `"Bonjour #{nom}"` |
| Longueur | `t.length` |
| Majuscules, minuscules | `t.upcase`, `t.downcase`, `t.capitalize` |
| Enlever les espaces autour | `t.strip` |
| Chercher | `t.include?("a")`, `t.start_with?("B")` |
| Découper, recoller | `t.split(",")`, `tableau.join(" ")` |
| Remplacer | `t.sub("a", "b")` (une fois), `t.gsub("a", "b")` (partout) |
| Une partie | `t[0]`, `t[0..2]`, `t[-1]` |
| Arrondir, formater | `x.round(2)`, `format("%.2f", x)` |

```ruby
t = "  bonjour ruby  ".strip
puts t.capitalize
puts t[0..6]
p t.split(" ")
puts t.gsub("o", "0")
puts format("%.2f", 3.14159)
```

```sortie
Bonjour ruby
bonjour
["bonjour", "ruby"]
b0nj0ur ruby
3.14
```

Détails : [travailler avec du texte](texte.md).

## Tableaux et hashs

| Action | Tableau | Hash |
| --- | --- | --- |
| Créer | `[1, 2]` | `{ a: 1 }` |
| Lire | `t[0]` | `h[:a]` |
| Ajouter | `t << 3` | `h[:b] = 2` |
| Supprimer | `t.delete(3)` | `h.delete(:b)` |
| Taille | `t.length` | `h.size` |
| Contient | `t.include?(1)` | `h.key?(:a)` |

```ruby
t = [3, 1, 2]
h = { a: 1 }
h[:b] = 2
p t.sort
p h
```

```sortie
[1, 2, 3]
{:a=>1, :b=>2}
```

Détails : [tableaux et hashs](collections.md).

## Blocs et Enumerable

| Méthode | Rôle |
| --- | --- |
| `each` | parcourir |
| `map` | transformer |
| `select` / `reject` | garder / enlever |
| `find` | premier qui correspond |
| `reduce` | combiner en une valeur |
| `sort_by` | trier selon un critère |
| `group_by` | regrouper dans un hash |
| `each_with_index` | parcourir avec l'indice |

```ruby
nombres = [1, 2, 3, 4, 5]
p nombres.map { |n| n * 10 }
p nombres.select(&:odd?)
p nombres.reduce(:+)
carre = ->(x) { x * x }
p carre.call(4)
```

```sortie
[10, 20, 30, 40, 50]
[1, 3, 5]
15
16
```

Détails : [les blocs](blocs.md).

## Classes et objets

```ruby
module Affichable
  def afficher
    puts to_s
  end
end

class Animal
  include Affichable
  attr_reader :nom

  def initialize(nom)
    @nom = nom
  end

  def to_s
    "Animal #{nom}"
  end
end

class Chat < Animal
  def to_s
    "Chat #{nom}"
  end
end

Chat.new("Félix").afficher
Point = Struct.new(:x, :y)
p Point.new(1, 2)
```

```sortie
Chat Félix
#<struct Point x=1, y=2>
```

Détails : [classes et objets](types.md).

## Erreurs

| Mot-clé | Rôle |
| --- | --- |
| `begin … rescue … end` | attraper une erreur |
| `rescue Type => e` | lire l'erreur dans `e` |
| `ensure` | toujours exécuté |
| `raise` | lever une erreur |
| `retry` | recommencer le `begin` |
| `&.` | appel sûr si `nil` |

```ruby
class MonErreur < StandardError
end

begin
  raise MonErreur, "aïe"
rescue MonErreur => e
  puts "Attrapé : #{e.message}"
ensure
  puts "Fini"
end
p nil&.length
```

```sortie
Attrapé : aïe
Fini
nil
```

Détails : [gérer les erreurs](erreurs.md).

## Clavier et fichiers

| Besoin | Syntaxe |
| --- | --- |
| Lire une ligne au clavier | `gets.chomp` |
| Arguments du programme | `ARGV[0]` |
| Écrire (remplace tout) | `File.write("f.txt", texte)` |
| Lire tout | `File.read("f.txt")` |
| Lire les lignes | `File.readlines("f.txt", chomp: true)` |
| Ligne par ligne | `File.foreach("f.txt")` avec un bloc |
| Le fichier existe ? | `File.exist?("f.txt")` |
| Ajouter à la fin | `File.open("f.txt", "a")` avec un bloc, puis `f.puts` |
| JSON | `require "json"`, `JSON.generate(h)`, `JSON.parse(t)` |

```ruby
File.write("f.txt", "un\n")
File.open("f.txt", "a") { |f| f.puts "deux" }
p File.readlines("f.txt", chomp: true)
puts File.exist?("f.txt")
```

```sortie
["un", "deux"]
true
```

Détails : [clavier et fichiers](fichiers.md).

## Projet et outils

| Commande | Rôle |
| --- | --- |
| `ruby fichier.rb` | lancer un programme |
| `irb` | console interactive |
| `gem install nom` | installer une gem |
| `bundle install` | installer les gems du `Gemfile` |
| `bundle exec …` | lancer avec les gems du projet |
| `rubocop` | vérifier le style |

Détails : [projets, gems et tests](projet.md). Pour pratiquer :
[les exercices corrigés](exercices.md). Comparez avec
[l'aide-mémoire Rust](../rust/aide-memoire.md) et [l'aide-mémoire Rythme](../rythme/aide-memoire.md).
