# Organiser un projet : fichiers, gems et Bundler

Cette page montre comment organiser un projet Ruby : découper le code en fichiers, utiliser des bibliothèques (les gems) et fixer leurs versions avec Bundler.

> **Objectifs :**
>
> - ranger un projet dans des dossiers clairs (`lib/`, `test/`) ;
> - charger vos fichiers avec `require_relative` et les bibliothèques avec `require` ;
> - installer une gem avec `gem install` ;
> - lister les gems d'un projet dans un `Gemfile` et les installer avec Bundler ;
> - vérifier le style du code avec RuboCop.

## Découper en plusieurs fichiers

Quand le programme grandit, un seul fichier devient difficile à lire. Rangez le code
dans plusieurs fichiers : en général, **un module ou une classe par fichier** (voir
[les modules](modules.md)). `require_relative` charge un fichier situé à côté du
fichier courant.

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

puts Calcul.double(21)
```

```sh
ruby main.rb
```

```text
42
```

> **À retenir :** le chemin de `require_relative` part du dossier du fichier qui
> l'écrit. Depuis `test/test_calcul.rb`, on écrit donc `require_relative "../lib/calcul"`.

## require et require_relative

Les deux chargent du code, mais ne cherchent pas au même endroit :

- `require_relative "chemin"` : un fichier **de votre projet**, à partir du fichier
  courant ;
- `require "nom"` : une bibliothèque **livrée avec Ruby** ou une gem installée.

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

Ruby est livré avec de nombreuses bibliothèques prêtes à charger avec `require` :
`json`, `date`, `set`, `csv`… Par exemple, `date` sait calculer avec les dates :

```ruby
require "date"

rentree = Date.new(2026, 9, 1)
puts rentree.strftime("%d/%m/%Y")
puts rentree + 30
```

```sortie
01/09/2026
2026-10-01
```

> **Note :** l'extension `.rb` est inutile dans `require` et `require_relative`.

## Un projet bien rangé

La plupart des projets Ruby suivent la même organisation. Vous vous y retrouvez ainsi
dans le projet de n'importe qui.

```text
mon_projet/
├── Gemfile          # les gems du projet
├── Gemfile.lock     # leurs versions exactes (créé par Bundler)
├── README.md        # à quoi sert le projet, comment le lancer
├── main.rb          # le programme à lancer
├── lib/             # le code : un fichier par module ou par classe
│   ├── calcul.rb
│   └── bulletin.rb
└── test/            # les tests : un fichier test_… par fichier de lib/
    ├── test_calcul.rb
    └── test_bulletin.rb
```

- Les noms de fichiers s'écrivent en minuscules avec des `_` : `liste_taches.rb`
  contient la classe `ListeTaches`.
- Les tests ont leur propre chapitre : [tester son code](tests.md#ranger-le-code-et-les-tests).

## Les tests du projet

Un projet sérieux a des tests : de petits programmes qui vérifient votre code
automatiquement. Ruby est livré avec **Minitest** pour les écrire. Tout est expliqué
dans le chapitre [Tester son code](tests.md).

## Les gems

Une **gem** est une bibliothèque Ruby prête à l'emploi, écrite par d'autres
programmeurs (voir [le glossaire](../debuter/glossaire.md#paquet)). Elles sont publiées
sur le site [rubygems.org](https://rubygems.org). La commande `gem` les installe.

```sh
gem install colorize    # installe la gem colorize
gem list                # liste les gems installées
```

Ensuite, vous la chargez avec `require`, comme une bibliothèque de Ruby :

```ruby norun
require "colorize"
puts "Succès".green
```

> **Attention :** `gem install` installe la gem sur **votre** ordinateur seulement. Un
> autre programmeur qui récupère votre projet ne l'aura pas : c'est le rôle de Bundler.

## Bundler et le Gemfile

Dans un projet, listez vos gems dans un fichier `Gemfile`, à la racine. **Bundler** lit
ce fichier et installe les bonnes versions, pour tout le monde.

```ruby norun
# Gemfile
source "https://rubygems.org"

gem "colorize", "~> 1.1"

group :test do
  gem "minitest"
end
```

- `source` : où télécharger les gems.
- `gem "colorize", "~> 1.1"` : la gem et sa version. `~> 1.1` accepte `1.1`, `1.2`,
  `1.9`… mais pas `2.0`, qui pourrait tout changer.
- `group :test` : des gems utiles seulement pour les tests.

```sh
gem install bundler     # une seule fois (souvent déjà installé avec Ruby)
bundle install          # installe les gems du Gemfile
bundle exec ruby main.rb
```

`bundle exec` lance le programme avec **exactement** les versions du projet.

> **À retenir :** `bundle install` crée un fichier `Gemfile.lock` avec les versions
> exactes. Gardez-le avec votre code : tout le monde aura les mêmes versions.

### Ajouter une gem plus tard

`bundle add` ajoute la ligne au `Gemfile` et installe la gem en une seule commande :

```sh
bundle add colorize
```

## RuboCop : un code propre

**RuboCop** est une gem qui vérifie le style de votre code (indentation, noms,
longueur des lignes…) et signale les erreurs courantes.

```sh
gem install rubocop
rubocop                 # analyse le projet
rubocop -a              # corrige automatiquement ce qui peut l'être
```

Il affiche chaque remarque avec le fichier et la ligne, par exemple :

```text
main.rb:3:5: C: Layout/IndentationWidth: Use 2 (not 4) spaces for indentation.
```

## Erreurs fréquentes

### Utiliser une gem qui n'est pas installée

```ruby norun
require "colorize"
puts "Succès".green
```

```text
<internal:…/rubygems/core_ext/kernel_require.rb>:136:in `require': cannot load such file -- colorize (LoadError)
```

Installez la gem (`gem install colorize`), ou ajoutez-la au `Gemfile` et lancez
`bundle install`.

### Charger un fichier du projet avec require

`require` cherche dans les bibliothèques installées, pas dans votre projet.

```ruby norun
require "lib/calcul"
```

```text
<internal:…/rubygems/core_ext/kernel_require.rb>:136:in `require': cannot load such file -- lib/calcul (LoadError)
```

Pour un fichier de votre projet, écrivez `require_relative "lib/calcul"`.

### Un mauvais chemin dans require_relative

Le chemin part du fichier qui contient `require_relative`. Depuis `test/test_calcul.rb`,
`"lib/calcul"` désigne `test/lib/calcul.rb`, qui n'existe pas.

```ruby norun
# test/test_calcul.rb
require_relative "lib/calcul"
```

```text
test/test_calcul.rb:2:in `require_relative': cannot load such file -- /home/alice/mon_projet/test/lib/calcul (LoadError)
```

Le message montre le chemin complet essayé : remontez d'un dossier avec
`require_relative "../lib/calcul"`.

### Lancer Bundler hors du projet

`bundle install` cherche un `Gemfile` dans le dossier courant.

```sh
bundle install
```

```text
Could not locate Gemfile
```

Placez-vous dans le dossier du projet (`cd mon_projet`), ou créez le `Gemfile`.

## Testez-vous

**1.** Quand utiliser `require` et quand utiliser `require_relative` ?

> **Réponse**
>
> `require_relative` pour un fichier de votre projet ; `require` pour une bibliothèque
> livrée avec Ruby (`json`, `date`…) ou une gem installée.

**2.** À quoi sert le fichier `Gemfile.lock`, et faut-il le garder ?

> **Réponse**
>
> Il note les versions exactes des gems installées par `bundle install`. Oui, gardez-le
> avec votre code : tout le monde utilisera les mêmes versions.

**3.** Dans quel dossier range-t-on le code, et dans lequel les tests ?

> **Réponse**
>
> Le code dans `lib/`, les tests dans `test/`, avec un fichier `test_….rb` par fichier
> de `lib/`.

## En résumé

- `require_relative` charge un fichier de votre projet, `require` une bibliothèque ou
  une gem.
- Un projet range son code dans `lib/` et ses tests dans `test/`.
- Une gem est une bibliothèque ; `gem install` l'installe sur votre ordinateur.
- Le `Gemfile` liste les gems du projet ; `bundle install` les installe et crée
  `Gemfile.lock`.
- RuboCop vérifie le style du code.

Suite : [Créer et publier sa propre gem](gems.md)
