# Projets, gems et tests

Cette page montre comment organiser un projet Ruby : découper le code en fichiers, utiliser des gems avec Bundler et écrire des tests.

## Découper en plusieurs fichiers

Quand le programme grandit, rangez le code dans plusieurs fichiers. `require_relative`
charge un fichier situé à côté du fichier courant.

```text
mon_projet/
├── main.rb
└── lib/
    └── calcul.rb
```

```ruby norun
# lib/calcul.rb
module Calcul
  def self.double(x)
    x * 2
  end
end
```

```ruby norun
# main.rb
require_relative "lib/calcul"

puts Calcul.double(21)   # => 42
```

## require et require_relative

- `require_relative "chemin"` : un fichier **de votre projet**.
- `require "nom"` : une bibliothèque de Ruby ou une gem installée.

```ruby
require "json"

donnees = { nom: "Alice", age: 30 }
puts donnees.to_json
p JSON.parse('{"ok": true}')
```

```sortie
{"nom":"Alice","age":30}
{"ok"=>true}
```

> **Note :** l'extension `.rb` est inutile dans `require` et `require_relative`.

## Les gems

Une **gem** est une bibliothèque Ruby prête à l'emploi. La commande `gem` les installe.

```sh
gem install colorize
gem list
```

Ensuite, vous la chargez avec `require` :

```ruby norun
require "colorize"
puts "Succès".green
```

## Bundler et le Gemfile

Dans un projet, listez vos gems dans un fichier `Gemfile`. **Bundler** installe les
bonnes versions pour tout le monde.

```ruby norun
# Gemfile
source "https://rubygems.org"

gem "colorize", "~> 1.1"

group :test do
  gem "minitest"
end
```

```sh
gem install bundler     # une seule fois
bundle install          # installe les gems du Gemfile
bundle exec ruby main.rb
```

> **À retenir :** `bundle install` crée un fichier `Gemfile.lock` avec les versions
> exactes. Gardez-le avec votre code.

## Écrire des tests avec Minitest

Minitest est livré avec Ruby. Une classe de test hérite de `Minitest::Test`, et chaque
méthode qui commence par `test_` est un test.

```ruby
require "minitest/autorun"

def double(x)
  x * 2
end

class TestDouble < Minitest::Test
  def test_double_positif
    assert_equal 8, double(4)
  end

  def test_double_zero
    assert_equal 0, double(0)
  end
end
```

Lancez le fichier comme un programme. Ruby affiche un résumé (la graine et le temps
changent à chaque fois) :

```text
Run options: --seed 12345

# Running:

..

Finished in 0.001s, 2000.0 runs/s, 2000.0 assertions/s.

2 runs, 2 assertions, 0 failures, 0 errors, 0 skips
```

Chaque `.` est un test réussi. Un `F` signale un échec.

> **Astuce :** les assertions les plus courantes sont `assert`, `assert_equal`,
> `assert_nil`, `refute` et `assert_raises`.

## RuboCop : un code propre

**RuboCop** vérifie le style de votre code et signale les erreurs courantes.

```sh
gem install rubocop
rubocop                 # analyse le projet
rubocop -a              # corrige automatiquement ce qui peut l'être
```

## Et ensuite ?

Gardez [l'aide-mémoire](aide-memoire.md) sous la main. Comparez avec
[les projets en Rust](../rust/projet.md).
