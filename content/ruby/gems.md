# Créer et publier sa propre gem

Cette page montre, pas à pas, comment transformer votre code en gem : l'écrire, la tester, l'emballer, la publier, et l'utiliser dans un autre projet.

> **Objectifs :**
>
> - savoir ce qu'est une gem et ce que contient son gemspec ;
> - créer le squelette d'une gem avec `bundle gem` ;
> - écrire le code dans `lib/`, avec un numéro de version, et le tester ;
> - construire le paquet avec `gem build` et le publier sur rubygems.org ;
> - choisir un numéro de version et utiliser une gem locale avec Bundler.

## Une gem, c'est quoi ?

Une [gem](../debuter/glossaire.md#gem) est une [bibliothèque](../debuter/glossaire.md#bibliotheque)
Ruby emballée pour être partagée. Vous en utilisez déjà : la page
[Organiser un projet](projet.md) montre `gem install` et Bundler. Ici, vous passez de
l'autre côté : vous **fabriquez** la gem.

Pensez à un colis :

```text
  +--------------------------------------------+
  |  ÉTIQUETTE (le gemspec)                    |
  |  nom : salutations    version : 0.1.0      |
  |  auteur : Alice       résumé : Dire bonjour|
  +--------------------------------------------+
  |  CONTENU                                   |
  |    lib/salutations.rb                      |
  |    lib/salutations/version.rb              |
  |    README.md                               |
  +--------------------------------------------+
         |
         |  gem build   (on ferme le colis)
         v
  salutations-0.1.0.gem
         |
         |  gem push    (on l'envoie à la poste : rubygems.org)
         v
  tout le monde peut faire : gem install salutations
```

- Le **contenu**, c'est votre code Ruby, rangé dans `lib/`.
- L'**étiquette**, c'est le *gemspec* : le nom, la version, l'auteur, la liste des
  fichiers.
- Le **bureau de poste**, c'est [rubygems.org](https://rubygems.org), le site qui
  distribue les gems.

> **À retenir :** une gem = du code dans `lib/` + une étiquette (le gemspec). On la
> construit avec `gem build`, on la publie avec `gem push`.

## Le code de la gem : un module et une méthode

Tout le code d'une gem vit dans un [module](../debuter/glossaire.md#module) qui porte son
nom (voir [les modules](modules.md)). Ainsi, ses noms ne se mélangent pas avec ceux des
autres gems. Notre gem `salutations` dit bonjour en trois langues :

```ruby
# lib/salutations.rb
module Salutations
  FORMULES = { fr: "Bonjour", en: "Hello", es: "Hola" }

  def self.bonjour(nom, langue: :fr)
    formule = FORMULES.fetch(langue)
    "#{formule}, #{nom} !"
  end
end

puts Salutations.bonjour("Alice")
puts Salutations.bonjour("Bob", langue: :en)
```

```sortie
Bonjour, Alice !
Hello, Bob !
```

Suivons le deuxième appel, `Salutations.bonjour("Bob", langue: :en)` :

| Étape | Ligne | nom | langue | formule | Résultat |
|-------|-------|-----|--------|---------|----------|
| 1 | entrée dans `bonjour` | `"Bob"` | `:en` | — | — |
| 2 | `FORMULES.fetch(langue)` | `"Bob"` | `:en` | `"Hello"` | — |
| 3 | `"#{formule}, #{nom} !"` | `"Bob"` | `:en` | `"Hello"` | `"Hello, Bob !"` |

- `self.bonjour` : la méthode s'appelle sur le module lui-même, `Salutations.bonjour`.
- `langue: :fr` est un argument nommé, avec `:fr` par défaut (voir
  [les fonctions](fonctions.md)).
- `fetch` lève une erreur si la langue n'existe pas : mieux vaut une erreur claire
  qu'un `nil` silencieux.

Essayez : ajoutez `it: "Ciao"` dans `FORMULES`, puis appelez
`Salutations.bonjour("Léa", langue: :it)`. Vous obtenez `Ciao, Léa !`.

> **À retenir :** le code d'une gem est rangé dans un module du même nom que la gem.

## Le numéro de version

Chaque gem a un numéro de version, rangé dans une [constante](../debuter/glossaire.md#constante)
`VERSION`, dans son propre fichier :

```ruby
# lib/salutations/version.rb
module Salutations
  VERSION = "0.1.0"
end

puts "salutations version #{Salutations::VERSION}"
```

```sortie
salutations version 0.1.0
```

Le fichier principal charge ce fichier avec `require_relative` :

```ruby norun
# lib/salutations.rb
require_relative "salutations/version"

module Salutations
  # ... le code vu plus haut
end
```

Pourquoi un fichier à part ? Parce que le gemspec doit lire la version **sans** charger
tout le code. C'est comme écrire le numéro du colis sur un post-it collé dehors : on le
lit sans ouvrir la boîte.

> **À retenir :** la version est une chaîne dans `Salutations::VERSION`, dans
> `lib/salutations/version.rb`.

## Créer le squelette avec bundle gem

Bundler crée tous les dossiers et fichiers de départ pour vous. Tapez dans le
[terminal](../debuter/glossaire.md#terminal) :

```sh
bundle gem salutations --test=minitest
```

```text
Creating gem 'salutations'...
      create  salutations/Gemfile
      create  salutations/lib/salutations.rb
      create  salutations/lib/salutations/version.rb
      create  salutations/sig/salutations.rbs
      create  salutations/salutations.gemspec
      create  salutations/Rakefile
      create  salutations/README.md
      create  salutations/bin/console
      create  salutations/bin/setup
      create  salutations/.gitignore
      create  salutations/test/test_helper.rb
      create  salutations/test/test_salutations.rb
Gem 'salutations' was successfully created.
```

Bundler pose parfois des questions avant (licence, intégration continue…) : répondez
`n` si vous hésitez. Voici le rangement obtenu :

```text
salutations/
├── lib/                        ← le CONTENU du colis
│   ├── salutations.rb          ← le module Salutations
│   └── salutations/
│       └── version.rb          ← VERSION = "0.1.0"
├── test/                       ← les tests (pas envoyés dans le colis)
│   ├── test_helper.rb
│   └── test_salutations.rb
├── salutations.gemspec         ← l'ÉTIQUETTE
├── Gemfile                     ← les gems utiles pour développer
├── Rakefile                    ← les tâches : rake test, rake build…
└── README.md                   ← le mode d'emploi
```

La règle des noms : la gem `salutations` → le fichier `lib/salutations.rb` → le module
`Salutations`. Pour une gem `bonne_humeur`, ce serait `lib/bonne_humeur.rb` et le module
`BonneHumeur`.

> **À retenir :** `bundle gem nom` crée le squelette ; le code va dans `lib/`, les
> tests dans `test/`.

## Le gemspec : l'étiquette du colis

Le fichier `salutations.gemspec` décrit la gem. `bundle gem` en écrit un plein de
`TODO` à remplacer. Voici une version simple et complète :

```ruby norun
# salutations.gemspec
require_relative "lib/salutations/version"

Gem::Specification.new do |spec|
  spec.name     = "salutations"
  spec.version  = Salutations::VERSION
  spec.authors  = ["Alice Martin"]
  spec.email    = ["alice@example.com"]
  spec.summary  = "Dire bonjour poliment, en plusieurs langues."
  spec.homepage = "https://github.com/alice/salutations"
  spec.license  = "MIT"
  spec.required_ruby_version = ">= 3.0"

  spec.files = Dir["lib/**/*.rb"] + ["README.md"]
  spec.require_paths = ["lib"]
end
```

| Champ | Ce qu'il dit | Obligatoire ? |
|-------|--------------|---------------|
| `name` | le nom de la gem, celui de `gem install` | oui |
| `version` | le numéro de version | oui |
| `authors` | qui l'a écrite | oui |
| `summary` | une phrase de résumé | oui |
| `files` | les fichiers mis dans le colis | oui, sinon le colis est vide |
| `license` | ce que les autres ont le droit d'en faire | conseillé |
| `require_paths` | où chercher le code pour `require` | `lib` par défaut |

`Dir["lib/**/*.rb"]` veut dire : « tous les fichiers `.rb` dans `lib/` et ses
sous-dossiers ».

### Une étiquette, c'est un objet Ruby

`Gem::Specification` est une classe Ruby ordinaire, toujours disponible. Vous pouvez
fabriquer une étiquette et la lire :

```ruby
spec = Gem::Specification.new do |s|
  s.name    = "salutations"
  s.version = "0.1.0"
  s.summary = "Dire bonjour"
  s.authors = ["Alice Martin"]
end

puts spec.name
puts spec.version
puts spec.full_name
puts spec.file_name
```

```sortie
salutations
0.1.0
salutations-0.1.0
salutations-0.1.0.gem
```

`file_name` est le nom du colis que `gem build` va fabriquer.

Essayez : remplacez `"0.1.0"` par `"1.0.0"`. Les deux dernières lignes deviennent
`salutations-1.0.0` et `salutations-1.0.0.gem`.

> **À retenir :** le gemspec est du Ruby : un `Gem::Specification.new` avec au moins
> le nom, la version, les auteurs, le résumé et les fichiers.

## Tester la gem

Avant d'envoyer un colis, on vérifie ce qu'il contient. Pour une gem, ce sont les
[tests](../debuter/glossaire.md#test) (voir [Tester son code](tests.md)).

### Avec de simples vérifications

Le plus simple : comparer le résultat attendu et le résultat obtenu.

```ruby
module Salutations
  FORMULES = { fr: "Bonjour", en: "Hello", es: "Hola" }

  def self.bonjour(nom, langue: :fr)
    "#{FORMULES.fetch(langue)}, #{nom} !"
  end
end

def verifier(attendu, obtenu)
  etat = attendu == obtenu ? "ok" : "ÉCHEC"
  puts "#{etat} : #{obtenu.inspect}"
end

verifier "Bonjour, Alice !", Salutations.bonjour("Alice")
verifier "Hola, Ana !", Salutations.bonjour("Ana", langue: :es)
verifier "Hello, Bob!", Salutations.bonjour("Bob", langue: :en)
```

```sortie
ok : "Bonjour, Alice !"
ok : "Hola, Ana !"
ÉCHEC : "Hello, Bob !"
```

| Vérification | attendu | obtenu | Égaux ? |
|--------------|---------|--------|---------|
| 1 | `"Bonjour, Alice !"` | `"Bonjour, Alice !"` | oui → `ok` |
| 2 | `"Hola, Ana !"` | `"Hola, Ana !"` | oui → `ok` |
| 3 | `"Hello, Bob!"` | `"Hello, Bob !"` | non → `ÉCHEC` |

Le troisième échec vient du **test**, pas du code : il manque une espace avant `!`
dans la valeur attendue. Un test peut se tromper aussi : relisez-le.

Essayez : corrigez la troisième ligne en `"Hello, Bob !"`. Les trois lignes affichent
`ok`.

### Avec Minitest

Dans le squelette, `test/test_salutations.rb` utilise Minitest, livré avec Ruby :

```ruby
require "minitest/autorun"

module Salutations
  FORMULES = { fr: "Bonjour", en: "Hello", es: "Hola" }

  def self.bonjour(nom, langue: :fr)
    "#{FORMULES.fetch(langue)}, #{nom} !"
  end
end

class TestSalutations < Minitest::Test
  def test_bonjour_en_francais_par_defaut
    assert_equal "Bonjour, Alice !", Salutations.bonjour("Alice")
  end

  def test_bonjour_en_espagnol
    assert_equal "Hola, Ana !", Salutations.bonjour("Ana", langue: :es)
  end

  def test_langue_inconnue
    assert_raises(KeyError) { Salutations.bonjour("Max", langue: :de) }
  end
end
```

Dans la vraie gem, le module n'est pas recopié : `test/test_helper.rb` le charge depuis
`lib/`. On lance les tests depuis le dossier de la gem :

```sh
bundle exec rake test
```

```text
Run options: --seed 40213

# Running:

...

Finished in 0.000731s, 4103.9672 runs/s, 4103.9672 assertions/s.

3 runs, 3 assertions, 0 failures, 0 errors, 0 skips
```

Trois points, trois tests réussis. Le nombre après `--seed` et le temps changent à
chaque fois.

> **À retenir :** testez la gem avant de la construire : `bundle exec rake test` doit
> finir par `0 failures, 0 errors`.

## Construire le paquet : gem build

`gem build` lit l'étiquette, met les fichiers dans le colis et le ferme :

```sh
gem build salutations.gemspec
```

```text
  Successfully built RubyGem
  Name: salutations
  Version: 0.1.0
  File: salutations-0.1.0.gem
```

Le fichier `salutations-0.1.0.gem` est une archive. Dedans, trois morceaux :

```text
salutations-0.1.0.gem
├── metadata.gz       ← l'étiquette (le gemspec, en YAML)
├── data.tar.gz       ← le contenu : lib/ et README.md
└── checksums.yaml.gz ← des empreintes pour vérifier que rien n'est abîmé
```

Avant de publier, essayez le colis chez vous. `gem install` accepte un fichier local :

```sh
gem install ./salutations-0.1.0.gem
ruby -e 'require "salutations"; puts Salutations.bonjour("Alice", langue: :es)'
```

```text
Successfully installed salutations-0.1.0
1 gem installed
Hola, Alice !
```

> **À retenir :** `gem build fichier.gemspec` fabrique le fichier `.gem` ;
> `gem install ./fichier.gem` l'essaie sur votre ordinateur.

## Publier sur rubygems.org

Publier, c'est envoyer le colis à la poste. Ces commandes ont besoin d'Internet et
d'un compte : elles ne sont pas exécutées ici.

1. Créez un compte sur [rubygems.org](https://rubygems.org/sign_up).
2. Connectez-vous depuis le terminal, une seule fois :

   ```sh
   gem signin
   ```

3. Envoyez le colis :

   ```sh
   gem push salutations-0.1.0.gem
   ```

   ```text
   Pushing gem to https://rubygems.org...
   Successfully registered gem: salutations (0.1.0)
   ```

Quelques minutes plus tard, n'importe qui peut taper `gem install salutations`.

> **Attention :** le nom doit être libre (cherchez-le d'abord sur rubygems.org), et une
> version publiée ne peut **jamais** être remplacée. Pour corriger, publiez une nouvelle
> version (`0.1.1`). Ne mettez jamais de mot de passe ou de clé secrète dans une gem.

> **À retenir :** `gem signin` une fois, puis `gem push fichier.gem` à chaque nouvelle
> version.

## Les numéros de version

Les gems suivent le **versionnage sémantique** (*semver*) : trois nombres,
`MAJEUR.MINEUR.CORRECTIF`. Chaque nombre est un message aux utilisateurs.

```text
   1   .   4   .   2
   |       |       |
   |       |       +-- CORRECTIF : un bug corrigé, rien d'autre ne change
   |       +---------- MINEUR    : une nouveauté, l'ancien code marche encore
   +------------------ MAJEUR    : un changement qui peut casser l'ancien code
```

| Vous avez fait… | Avant | Après |
|-----------------|-------|-------|
| corrigé une faute dans `"Hola"` | `1.4.2` | `1.4.3` |
| ajouté la langue `:it` | `1.4.3` | `1.5.0` |
| renommé `bonjour` en `saluer` | `1.5.0` | `2.0.0` |
| la toute première version, encore instable | — | `0.1.0` |

Quand un nombre monte, ceux à sa droite repassent à `0`. Une version `0.x` prévient :
« tout peut encore changer ».

### Comparer des versions

Attention : comparer des versions comme des textes donne un faux résultat.
`Gem::Version` compare nombre par nombre :

```ruby
p "1.10.0" > "1.9.0"
p Gem::Version.new("1.10.0") > Gem::Version.new("1.9.0")
p Gem::Version.new("1.4.2").segments
```

```sortie
false
true
[1, 4, 2]
```

Comme textes, `"1.1…"` est avant `"1.9…"`, car le caractère `1` vient avant `9`. Comme
versions, `10` est plus grand que `9`.

### L'opérateur ~>

Dans un `Gemfile`, `gem "salutations", "~> 1.2"` veut dire : « `1.2` ou plus, mais
moins que `2.0` ». Vous acceptez les nouveautés, pas les changements qui cassent.

```ruby
regle = Gem::Requirement.new("~> 1.2")

%w[1.1.0 1.2.0 1.9.3 2.0.0].each do |v|
  puts "#{v} : #{regle.satisfied_by?(Gem::Version.new(v))}"
end
```

```sortie
1.1.0 : false
1.2.0 : true
1.9.3 : true
2.0.0 : false
```

| Étape | v | Au moins 1.2 ? | Moins que 2.0 ? | Accepté ? |
|-------|---|----------------|-----------------|-----------|
| 1 | `1.1.0` | non | oui | `false` |
| 2 | `1.2.0` | oui | oui | `true` |
| 3 | `1.9.3` | oui | oui | `true` |
| 4 | `2.0.0` | oui | non | `false` |

Essayez : remplacez `"~> 1.2"` par `"~> 1.2.0"`. Seule `1.2.0` reste acceptée : la
règle devient « `1.2.0` ou plus, mais moins que `1.3` ».

> **À retenir :** MAJEUR.MINEUR.CORRECTIF. Changez le MAJEUR quand vous cassez quelque
> chose, le MINEUR pour une nouveauté, le CORRECTIF pour un bug.

## Utiliser une gem locale avec Bundler

Pendant que vous développez la gem, vous voulez l'essayer dans un autre projet, sans la
publier. Bundler sait prendre une gem dans un dossier, avec `path:`.

```text
mes-projets/
├── salutations/      ← la gem
└── accueil/          ← le projet qui l'utilise
    ├── Gemfile
    └── main.rb
```

```ruby norun
# accueil/Gemfile
source "https://rubygems.org"

gem "salutations", path: "../salutations"
```

```ruby norun
# accueil/main.rb
require "salutations"

puts Salutations.bonjour("Bob", langue: :en)
```

```sh
cd accueil
bundle install
bundle exec ruby main.rb
```

```text
Resolving dependencies...
Bundle complete! 1 Gemfile dependency, 2 gems now installed.
Use `bundle info [gemname]` to see where a bundled gem is installed.
Hello, Bob !
```

`bundle exec` lance le programme avec les gems du `Gemfile`. Avec `path:`, Bundler lit
directement le dossier : modifiez `lib/salutations.rb`, relancez, le changement est
déjà là. Pas besoin de `gem build`.

Essayez : changez `"Hello"` en `"Hi"` dans la gem, puis relancez
`bundle exec ruby main.rb`. Le programme affiche `Hi, Bob !`.

Une fois la gem publiée, remplacez la ligne par `gem "salutations", "~> 0.1"`.

> **À retenir :** `gem "nom", path: "../dossier"` utilise une gem pas encore publiée,
> et suit vos modifications en direct.

## Erreurs fréquentes

### Un champ obligatoire manque dans le gemspec

```ruby norun
Gem::Specification.new do |spec|
  spec.name    = "salutations"
  spec.version = "0.1.0"
  spec.authors = ["Alice Martin"]
  spec.files   = Dir["lib/**/*.rb"]
end
```

```text
WARNING:  See https://guides.rubygems.org/specification-reference/ for help
ERROR:  While executing gem ... (Gem::InvalidSpecificationException)
    missing value for attribute summary
```

`gem build` refuse un colis sans résumé. Ajoutez `spec.summary = "…"`. Lisez la ligne
`missing value for attribute …` : elle nomme le champ qui manque.

### Une clé absente avec fetch

```ruby norun
FORMULES = { fr: "Bonjour", en: "Hello" }
puts FORMULES.fetch(:de)
```

```text
main.rb:2:in `fetch': key not found: :de (KeyError)
	from main.rb:2:in `<main>'
```

La langue `:de` n'est pas dans le hash. Ajoutez-la (`de: "Hallo"`), ou donnez une
valeur de secours : `FORMULES.fetch(langue, "Bonjour")`.

### Un mauvais chemin dans path:

```ruby norun
# accueil/Gemfile
gem "salutations", path: "../salutation"
```

```text
The path `/home/alice/mes-projets/salutation` does not exist.
```

Il manque un `s` au nom du dossier. Le chemin part du dossier du `Gemfile` : vérifiez-le
avec `ls ../`.

## Testez-vous

**Question 1 :** quel fichier sert d'« étiquette » à une gem, et que contient-il ?

> **Réponse**
>
> Le gemspec (`salutations.gemspec`). Il donne le nom, la version, les auteurs, le
> résumé, la licence et la liste des fichiers mis dans la gem.

**Question 2 :** votre gem est en `2.3.1`. Vous ajoutez une méthode sans rien casser.
Quelle est la nouvelle version ?

> **Réponse**
>
> `2.4.0` : une nouveauté compatible fait monter le MINEUR, et le CORRECTIF repasse à
> `0`.

**Question 3 :** écrivez le code qui affiche le nom du fichier `.gem` d'une gem
`meteo` en version `0.2.0`.

> **Réponse**
>
> ```ruby
> spec = Gem::Specification.new do |s|
>   s.name = "meteo"
>   s.version = "0.2.0"
> end
> puts spec.file_name
> ```
>
> ```sortie
> meteo-0.2.0.gem
> ```

## En résumé

- Une gem, c'est du code dans `lib/` (un module du même nom) et un gemspec, son
  étiquette.
- `bundle gem nom` crée le squelette ; la version vit dans `lib/nom/version.rb`.
- Testez avec Minitest (`bundle exec rake test`) avant de construire.
- `gem build` fabrique le `.gem`, `gem push` le publie sur rubygems.org.
- Les versions suivent MAJEUR.MINEUR.CORRECTIF ; `~> 1.2` accepte tout jusqu'à `2.0`
  exclu.
- `gem "nom", path: "…"` utilise une gem locale avec Bundler.

Suite : [Projets pas à pas](projets.md)
