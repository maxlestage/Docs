# Ruby : introduction et installation

Cette page présente Ruby, explique comment l'installer et vous fait écrire votre premier programme.

> **Objectifs :**
>
> - vous saurez dire ce qu'est Ruby et à quoi il sert ;
> - vous saurez installer Ruby et vérifier qu'il marche ;
> - vous saurez essayer une ligne de code dans `irb` ;
> - vous saurez écrire un fichier `.rb` et le lancer.

## Qu'est-ce que Ruby ?

Ruby est un langage de programmation créé au Japon par Yukihiro Matsumoto, en 1995.
Son but : rendre les programmeurs heureux. Le code se lit presque comme de l'anglais.

Un **langage de programmation** est une langue précise, que l'ordinateur sait suivre.
Un **programme** est un texte écrit dans cette langue : une suite d'ordres, comme une
recette de cuisine.

```ruby
3.times { puts "Bonjour" }
```

```sortie
Bonjour
Bonjour
Bonjour
```

Lisez cette ligne à voix haute : « 3 fois, affiche Bonjour ». `puts` (de l'anglais
*put string*, « pose le texte ») affiche un texte.

Ruby est un langage **interprété** : vous écrivez un fichier, puis vous le lancez
directement. Un programme appelé [interpréteur](../debuter/glossaire.md#interpreteur)
lit votre fichier et exécute les ordres un par un. Il n'y a pas d'étape de préparation
(de « compilation ») avant.

## Pourquoi apprendre Ruby ?

- La syntaxe (la façon d'écrire le code) est courte et lisible.
- Tout est un [objet](../debuter/glossaire.md#objet), même les nombres : chaque valeur
  sait faire des choses.
- Il est très utilisé pour le web (avec Ruby on Rails) et pour les scripts.
- Sa communauté partage des milliers de bibliothèques, appelées **gems**.

« Tout est un objet » veut dire que vous pouvez poser une question à un nombre ou à un
texte. Le point `.` sert à appeler une [méthode](../debuter/glossaire.md#methode),
c'est-à-dire une action que la valeur sait faire.

```ruby
puts 5.even?
puts "ruby".upcase
```

```sortie
false
RUBY
```

- `5.even?` demande au nombre 5 : « es-tu pair ? ». Il répond `false` (faux).
- `"ruby".upcase` demande au texte de se mettre en majuscules.

> **Note :** ce site présente aussi d'autres langages : comparez avec
> [l'introduction à Rust](../rust/introduction.md) ou [l'introduction à Rythme](../rythme/introduction.md).

## Installer Ruby

Ce site utilise **Ruby 3.3**. Une version plus récente marche aussi.

### Sur Linux et macOS

Le plus simple est d'utiliser le gestionnaire de paquets de votre système : un
programme qui installe les autres programmes pour vous.

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

Vous devez voir un numéro de version, par exemple `ruby 3.3.0`. Si le terminal répond
« command not found » (commande introuvable), Ruby n'est pas installé, ou le terminal
a été ouvert avant l'installation : fermez-le et rouvrez-le.

## Essayer Ruby avec irb

`irb` (*interactive Ruby*) est une console interactive. Vous tapez une ligne, Ruby
répond tout de suite. C'est comme une calculatrice qui comprend tout Ruby.

```text
$ irb
irb(main):001> 2 + 3
=> 5
irb(main):002> "salut".length
=> 5
irb(main):003> exit
```

- La flèche `=>` montre le résultat de la ligne.
- `exit` quitte irb.

> **Astuce :** irb est parfait pour tester une petite idée sans créer de fichier.

## Votre premier programme

Ouvrez votre [éditeur de code](../debuter/editeur.md). Créez un fichier `hello.rb` avec
ce contenu :

```ruby
puts "Bonjour, le monde !"
```

```sortie
Bonjour, le monde !
```

- `puts` affiche un texte, puis passe à la ligne.
- Le texte est entre guillemets `"..."`. Sans guillemets, Ruby croirait que c'est un
  nom de code.

Un programme peut avoir plusieurs lignes. Ruby les exécute dans l'ordre, de haut en bas.

```ruby
puts "Ligne 1"
puts "Ligne 2"
puts 2 + 3
```

```sortie
Ligne 1
Ligne 2
5
```

`puts 2 + 3` affiche `5` : sans guillemets, Ruby calcule.

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

## Lire un message d'erreur

Quand Ruby ne comprend pas votre code, il s'arrête et affiche une
[erreur](../debuter/glossaire.md#erreur). Ce n'est pas grave : le message vous aide.

```text
main.rb:1:in `<main>': undefined method `put' for main (NoMethodError)
```

Lisez-le de gauche à droite :

- `main.rb:1` : le fichier et le numéro de ligne du problème ;
- ``undefined method `put'`` : la description (« méthode `put` inconnue ») ;
- `(NoMethodError)` : le nom du type d'erreur.

Les messages sont en anglais. Quelques mots reviennent souvent : *undefined* (non
défini), *unexpected* (inattendu), *expecting* (j'attendais), *missing* (il manque).

## Erreurs fréquentes

### Une faute de frappe dans puts

```ruby norun
put "Bonjour"
```

```text
main.rb:1:in `<main>': undefined method `put' for main (NoMethodError)

put "Bonjour"
^^^
Did you mean?  puts
               putc
```

Ruby ne connaît pas `put`. Il propose même la bonne orthographe (*Did you mean?* :
« vouliez-vous dire ? »). Corrigez en `puts "Bonjour"`.

### Oublier les guillemets

```ruby norun
puts Bonjour
```

```text
main.rb:1:in `<main>': uninitialized constant Bonjour (NameError)

puts Bonjour
     ^^^^^^^
```

Sans guillemets, `Bonjour` est lu comme un nom de code, pas comme un texte. Écrivez
`puts "Bonjour"`.

### Oublier le guillemet de fin

```ruby norun
puts "Bonjour
```

```text
main.rb: --> main.rb
expected a closing delimiter for the string literal
> 1  puts "Bonjour
main.rb:1: unterminated string meets end of file (SyntaxError)
puts "Bonjour
             ^
```

Le texte n'est jamais fermé (*unterminated string*). Ajoutez le guillemet manquant :
`puts "Bonjour"`.

## Testez-vous

**Question 1 :** quelle commande vérifie que Ruby est installé ?

> **Réponse**
>
> `ruby -v`. Elle affiche le numéro de version.

**Question 2 :** qu'affiche ce programme ?

```ruby norun
puts "2 + 3"
puts 2 + 3
```

> **Réponse**
>
> `2 + 3`, puis `5`. Entre guillemets, c'est un texte affiché tel quel. Sans
> guillemets, Ruby calcule.

**Question 3 :** vous avez écrit `jeu.rb`. Comment le lancer ?

> **Réponse**
>
> Placez-vous dans son dossier avec `cd`, puis tapez `ruby jeu.rb`.

## En résumé

- Ruby est un langage simple et lisible, où tout est un objet.
- Installez-le avec votre gestionnaire de paquets, rbenv ou RubyInstaller.
- `ruby -v` vérifie l'installation, `irb` permet d'essayer du code tout de suite.
- Un programme est un fichier `.rb`, lancé avec `ruby nom_du_fichier.rb`.
- `puts` affiche un texte et passe à la ligne.
- Un message d'erreur donne le fichier, la ligne et la cause : lisez-le.

Suite : [premiers pas](premiers-pas.md).
