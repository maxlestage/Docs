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
p 7 / 2, 7.0 / 2
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
p t.sort, h
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
  def afficher = puts(to_s)
end

class Animal
  include Affichable
  attr_reader :nom

  def initialize(nom)
    @nom = nom
  end

  def to_s = "Animal #{nom}"
end

class Chat < Animal
  def to_s = "Chat #{nom}"
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
class MonErreur < StandardError; end

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

## Projet et outils

| Commande | Rôle |
| --- | --- |
| `ruby fichier.rb` | lancer un programme |
| `irb` | console interactive |
| `gem install nom` | installer une gem |
| `bundle install` | installer les gems du `Gemfile` |
| `bundle exec …` | lancer avec les gems du projet |
| `rubocop` | vérifier le style |

Détails : [projets, gems et tests](projet.md). Comparez avec
[l'aide-mémoire Rust](../rust/aide-memoire.md) et [l'aide-mémoire Rythme](../rythme/aide-memoire.md).
