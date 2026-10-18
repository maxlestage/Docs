# Ruby : introduction et installation

Cette page présente Ruby, explique comment l'installer et vous fait écrire votre premier programme.

## Qu'est-ce que Ruby ?

Ruby est un langage de programmation créé au Japon par Yukihiro Matsumoto, en 1995.
Son but : rendre les programmeurs heureux. Le code se lit presque comme de l'anglais.

```ruby
3.times { puts "Bonjour" }
```

```sortie
Bonjour
Bonjour
Bonjour
```

Ruby est un langage **interprété** : vous écrivez un fichier, puis vous le lancez
directement. Il n'y a pas d'étape de préparation (de « compilation ») avant.

## Pourquoi apprendre Ruby ?

- La syntaxe est courte et lisible.
- Tout est un objet, même les nombres.
- Il est très utilisé pour le web (avec Ruby on Rails) et pour les scripts.
- Sa communauté partage des milliers de bibliothèques, appelées **gems**.

```ruby
puts 5.even?
puts "ruby".upcase
```

```sortie
false
RUBY
```

> **Note :** ce site présente aussi d'autres langages : comparez avec
> [l'introduction à Rust](../rust/introduction.md) ou [l'introduction à Rythme](../rythme/introduction.md).

## Installer Ruby

### Sur Linux et macOS

Le plus simple est d'utiliser le gestionnaire de paquets de votre système.

```sh
sudo apt install ruby-full     # Debian, Ubuntu
sudo dnf install ruby          # Fedora
brew install ruby              # macOS avec Homebrew
```

### Avec rbenv

`rbenv` permet d'installer plusieurs versions de Ruby et de passer de l'une à l'autre.

```sh
rbenv install 3.3.0
rbenv global 3.3.0
```

> **Astuce :** utilisez rbenv si vous travaillez sur plusieurs projets qui demandent des
> versions différentes de Ruby.

### Sur Windows

Téléchargez **RubyInstaller** sur le site rubyinstaller.org, puis lancez l'installeur.
Gardez les options proposées par défaut.

## Vérifier l'installation

Ouvrez un terminal et tapez la commande suivante. Si le terminal est nouveau pour vous,
lisez d'abord [le terminal](../debuter/terminal.md).

```sh
ruby -v
```

Vous devez voir un numéro de version, par exemple `ruby 3.3.0`.

## Essayer Ruby avec irb

`irb` est une console interactive. Vous tapez une ligne, Ruby répond tout de suite.

```text
$ irb
irb(main):001> 2 + 3
=> 5
irb(main):002> "salut".length
=> 5
irb(main):003> exit
```

> **Astuce :** irb est parfait pour tester une petite idée sans créer de fichier.

## Votre premier programme

Créez un fichier `hello.rb` avec ce contenu :

```ruby
puts "Bonjour, le monde !"
```

```sortie
Bonjour, le monde !
```

`puts` affiche un texte, puis passe à la ligne.

## Lancer un programme

Dans le terminal, placez-vous dans le dossier du fichier (avec la commande `cd`), puis
tapez :

```sh
ruby hello.rb
```

Le programme s'exécute et affiche `Bonjour, le monde !`.

> **À retenir :** un programme Ruby est un fichier `.rb`. Vous le lancez avec
> `ruby nom_du_fichier.rb`.

Le guide [le terminal](../debuter/terminal.md) explique comment ouvrir un terminal et
changer de dossier.

## En résumé

- Ruby est un langage simple et lisible, où tout est un objet.
- Installez-le avec votre gestionnaire de paquets, rbenv ou RubyInstaller.
- `ruby -v` vérifie l'installation, `irb` permet d'essayer du code tout de suite.
- Un programme est un fichier `.rb`, lancé avec `ruby nom_du_fichier.rb`.
- `puts` affiche un texte et passe à la ligne.

Suite : passez aux [premiers pas](premiers-pas.md) pour découvrir les variables, les nombres et
les textes.
