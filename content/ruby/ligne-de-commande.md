# Écrire un outil en ligne de commande

Cette page montre comment écrire un petit programme qui se lance dans le terminal, lit ses arguments et ses options, et répond avec une sortie et un code de retour propres.

> **Objectifs :**
>
> - vous saurez lire les arguments d'un programme avec `ARGV` ;
> - vous saurez déclarer des options comme `-v` ou `--nombre 3` avec `OptionParser` ;
> - vous saurez séparer la sortie normale (`$stdout`) des messages d'erreur (`$stderr`) ;
> - vous saurez terminer avec un code de retour grâce à `exit` et `abort` ;
> - vous saurez rendre un script exécutable et écrire un outil complet.

## Un outil en ligne de commande

Un outil en ligne de commande se lance depuis le [terminal](../debuter/terminal.md),
avec des mots en plus après son nom :

```sh
ruby compter.rb -l notes.txt journal.txt
```

Ces mots sont les **arguments** du programme. Ceux qui commencent par `-` sont des
**options** : ils changent le comportement (`-l` : compter aussi les lignes). Les
autres sont souvent des noms de fichiers.

Un bon outil se comporte comme un employé fiable : il comprend les consignes, il
écrit ses résultats d'un côté et ses plaintes de l'autre, et il dit clairement à la
fin s'il a réussi.

## Les arguments avec ARGV

Ruby range les arguments dans le tableau `ARGV`, sous forme de **textes**. Le chapitre
[les fichiers](fichiers.md#les-arguments-avec-argv) l'a présenté rapidement.

```ruby norun
# salut.rb
nom = ARGV[0] || "inconnu"
fois = (ARGV[1] || "1").to_i
fois.times { puts "Bonjour, #{nom} !" }
```

```sh
ruby salut.rb Alice 2
```

```text
Bonjour, Alice !
Bonjour, Alice !
```

`ARGV[1]` vaut `"2"`, un texte : `to_i` le transforme en nombre. `|| "inconnu"` donne
une valeur par défaut quand l'argument manque.

### Essayer sans terminal

Pour lancer un script depuis Ruby lui-même, `Open3.capture3` exécute une commande et
renvoie trois choses : sa sortie, ses messages d'erreur et son **statut**.
`RbConfig.ruby` donne le chemin de Ruby. Les exemples de la page utilisent cette
méthode pour être exécutables tels quels.

```ruby
require "open3"

File.write("salut.rb", <<~RUBY)
  nom = ARGV[0] || "inconnu"
  puts "Bonjour, \#{nom} !"
  p ARGV
RUBY

sortie, erreurs, statut = Open3.capture3(RbConfig.ruby, "salut.rb", "Alice", "42")
puts sortie

sortie, erreurs, statut = Open3.capture3(RbConfig.ruby, "salut.rb")
puts sortie
```

```sortie
Bonjour, Alice !
["Alice", "42"]
Bonjour, inconnu !
[]
```

Le `\#{nom}` est échappé pour que l'interpolation se fasse dans `salut.rb`, et non au
moment d'écrire le fichier.

## Les options avec OptionParser

Analyser `-v`, `--nombre 3` ou `--help` à la main devient vite pénible.
`OptionParser` (avec `require "optparse"`) le fait pour vous :

- `on` déclare une option : forme courte, forme longue, type, description ;
- le bloc de `on` reçoit la valeur ;
- `parse` analyse un tableau d'arguments et renvoie ce qui n'est pas une option.

Pour que l'exemple tourne ici, on analyse un tableau fixe. Dans un vrai script, on
écrit `parser.parse!(ARGV)`.

```ruby
require "optparse"

options = { nombre: 1, verbeux: false }

parser = OptionParser.new do |o|
  o.banner = "Usage : ruby salut.rb [options] NOM..."
  o.on("-n", "--nombre N", Integer, "Répéter N fois") { |n| options[:nombre] = n }
  o.on("-v", "--verbeux", "Afficher plus de détails") { options[:verbeux] = true }
end

noms = parser.parse(["-v", "--nombre", "2", "Alice", "Bob"])

p options
p noms
puts parser
```

```sortie
{:nombre=>2, :verbeux=>true}
["Alice", "Bob"]
Usage : ruby salut.rb [options] NOM...
    -n, --nombre N                   Répéter N fois
    -v, --verbeux                    Afficher plus de détails
```

`puts parser` affiche l'aide, construite à partir de vos descriptions. Le type
`Integer` convertit `"2"` en `2` : pas besoin de `to_i`.

> **Astuce :** `--help` (et `-h`) est ajouté automatiquement : il affiche l'aide puis
> arrête le programme.

### Une option invalide

Si l'utilisateur tape une option inconnue, `parse` lève `OptionParser::InvalidOption`.
Attrapez-la pour afficher un message clair.

```ruby
require "optparse"

parser = OptionParser.new { |o| o.on("-v", "--verbeux") }

begin
  parser.parse(["--couleur"])
rescue OptionParser::InvalidOption => e
  puts "Erreur : #{e.message}"
end
```

```sortie
Erreur : invalid option: --couleur
```

## Deux sorties : $stdout et $stderr

Un programme a deux canaux de sortie :

- `$stdout`, la **sortie standard** : les résultats. `puts` écrit ici.
- `$stderr`, la **sortie d'erreur** : les avertissements et les erreurs. `warn` écrit
  ici.

C'est comme un cuisinier qui envoie les plats en salle et les remarques en cuisine :
les deux ne se mélangent pas. Celui qui redirige les résultats dans un fichier voit
toujours les erreurs à l'écran.

```ruby
require "open3"

File.write("deux.rb", <<~RUBY)
  puts "résultat"
  warn "attention"
  $stderr.puts "problème"
  $stdout.puts "autre résultat"
RUBY

sortie, erreurs, statut = Open3.capture3(RbConfig.ruby, "deux.rb")
puts "stdout : #{sortie.inspect}"
puts "stderr : #{erreurs.inspect}"
```

```sortie
stdout : "résultat\nautre résultat\n"
stderr : "attention\nproblème\n"
```

Dans le terminal, `>` envoie la sortie standard dans un fichier, `2>` la sortie
d'erreur :

```sh
ruby deux.rb > resultats.txt 2> erreurs.txt
```

## Le code de retour : exit et abort

En se terminant, chaque programme renvoie un **code de retour** : un nombre. `0` veut
dire « tout va bien », un autre nombre veut dire « échec ». Les scripts et les outils
d'automatisation lisent ce code pour savoir s'il faut continuer.

- `exit` termine avec le code `0` ; `exit 2` avec le code `2` ;
- `abort("message")` écrit le message sur `$stderr` et termine avec le code `1`.

```ruby
require "open3"

File.write("verif.rb", <<~RUBY)
  abort("Il manque un nom de fichier.") if ARGV.empty?
  puts "Traitement de \#{ARGV.first}"
  exit 0
RUBY

sortie, erreurs, statut = Open3.capture3(RbConfig.ruby, "verif.rb")
puts "erreur : #{erreurs}"
puts "code : #{statut.exitstatus}"

sortie, erreurs, statut = Open3.capture3(RbConfig.ruby, "verif.rb", "notes.txt")
puts sortie
puts "code : #{statut.exitstatus}"
puts statut.success?
```

```sortie
erreur : Il manque un nom de fichier.
code : 1
Traitement de notes.txt
code : 0
true
```

Dans le terminal, `$?` contient le code du dernier programme :

```sh
ruby verif.rb
echo $?
```

```text
Il manque un nom de fichier.
1
```

## Lire l'entrée standard

Un outil peut aussi lire ce qu'on lui envoie par un **tuyau** (`|`) : la sortie d'un
programme devient l'entrée du suivant. En Ruby, cette entrée est `$stdin`.

```ruby norun
# majuscules.rb
$stdin.each_line do |ligne|
  puts ligne.upcase
end
```

```sh
echo "bonjour le monde" | ruby majuscules.rb
```

```text
BONJOUR LE MONDE
```

> **Note :** `ARGF` est un raccourci très pratique : il lit les fichiers nommés dans
> `ARGV`, ou `$stdin` s'il n'y en a aucun. `ARGF.read` donne tout le texte d'un coup.

## Rendre un script exécutable

Pour lancer `./compter.rb` sans taper `ruby` devant, il faut deux choses.

1. Une première ligne spéciale, le **shebang** : elle dit au système quel programme doit
   lire le fichier.

   ```ruby norun
   #!/usr/bin/env ruby
   puts "Je suis exécutable !"
   ```

2. Le droit d'exécution, donné avec `chmod +x` (sur Linux et macOS) :

   ```sh
   chmod +x compter.rb
   ./compter.rb
   ```

`/usr/bin/env ruby` cherche Ruby là où il est installé : le script marche sur
d'autres ordinateurs. Pour taper simplement `compter`, renommez le fichier sans `.rb`
et placez-le dans un dossier de votre `PATH`, comme `~/bin`.

## Un outil complet : compter les mots

Voici un petit outil qui compte les mots des fichiers donnés en arguments, comme la
commande `wc`. Il accepte l'option `-l` pour compter aussi les lignes, signale les
fichiers absents sur `$stderr` et renvoie le code `1` en cas de problème.

```ruby
require "open3"

File.write("compter.rb", <<~'RUBY')
  #!/usr/bin/env ruby
  require "optparse"

  options = { lignes: false }
  parser = OptionParser.new do |o|
    o.banner = "Usage : compter [-l] FICHIER..."
    o.on("-l", "--lignes", "Compter aussi les lignes") { options[:lignes] = true }
  end

  begin
    fichiers = parser.parse(ARGV)
  rescue OptionParser::ParseError => e
    abort("compter : #{e.message}\n#{parser}")
  end
  abort(parser.to_s) if fichiers.empty?

  total = 0
  echec = false
  fichiers.each do |nom|
    unless File.exist?(nom)
      warn "compter : #{nom} : fichier introuvable"
      echec = true
      next
    end
    texte = File.read(nom)
    mots = texte.split.size
    total += mots
    ligne = "#{mots.to_s.rjust(5)} mots"
    ligne += "  #{texte.lines.size.to_s.rjust(3)} lignes" if options[:lignes]
    puts "#{ligne}  #{nom}"
  end
  puts "#{total.to_s.rjust(5)} mots  total" if fichiers.size > 1

  exit(echec ? 1 : 0)
RUBY

File.write("poeme.txt", "Le vent se lève\nil faut tenter de vivre\n")
File.write("liste.txt", "pain beurre\nconfiture\n")

sortie, erreurs, statut = Open3.capture3(RbConfig.ruby, "compter.rb", "-l", "poeme.txt", "liste.txt")
puts sortie
puts "code : #{statut.exitstatus}"

sortie, erreurs, statut = Open3.capture3(RbConfig.ruby, "compter.rb", "poeme.txt", "absent.txt")
puts sortie
puts erreurs
puts "code : #{statut.exitstatus}"
```

```sortie
    9 mots    2 lignes  poeme.txt
    3 mots    2 lignes  liste.txt
   12 mots  total
code : 0
    9 mots  poeme.txt
    9 mots  total
compter : absent.txt : fichier introuvable
code : 1
```

Le texte du script est écrit avec `<<~'RUBY'` (entre apostrophes) : les `#{...}`
restent tels quels dans `compter.rb` et ne sont calculés que lorsqu'il s'exécute.

Dans un terminal, vous l'utiliseriez ainsi :

```sh
chmod +x compter.rb
./compter.rb -l poeme.txt liste.txt
```

## Erreurs fréquentes

### Utiliser un argument absent

```ruby norun
nom = ARGV.first
puts nom.upcase
```

```text
main.rb:2:in `<main>': undefined method `upcase' for nil (NoMethodError)

puts nom.upcase
        ^^^^^^^
```

Lancé sans argument, `ARGV.first` vaut `nil`. Vérifiez avant :
`abort("Usage : ruby main.rb NOM") if ARGV.empty?`.

### Lire le clavier avec gets quand ARGV n'est pas vide

```ruby norun
# lancé avec : ruby main.rb --verbeux
ligne = gets
```

```text
main.rb:2:in `gets': No such file or directory @ rb_sysopen - --verbeux (Errno::ENOENT)
	from main.rb:2:in `gets'
	from main.rb:2:in `<main>'
```

Quand `ARGV` contient des mots, `gets` croit que ce sont des fichiers à lire. Pour lire
le clavier, écrivez `$stdin.gets`.

### Une option inconnue non rattrapée

```ruby norun
require "optparse"
OptionParser.new { |o| o.on("-v", "--verbeux") }.parse!(["--couleur"])
```

```text
main.rb:2:in `<main>': invalid option: --couleur (OptionParser::InvalidOption)
```

L'utilisateur voit une trace d'erreur peu aimable. Entourez `parse!` d'un
`rescue OptionParser::ParseError => e` et terminez avec `abort(e.message)`.

### Lancer un script sans le droit d'exécution

```sh
./compter.rb
```

```text
bash: ./compter.rb: Permission denied
```

Le fichier n'est pas encore exécutable. Donnez-lui le droit avec `chmod +x compter.rb`,
ou lancez-le avec `ruby compter.rb`.

## Testez-vous

**Question 1 :** lancé avec `ruby prog.rb 3 4`, que vaut `ARGV[0] + ARGV[1]` ?

> **Réponse**
>
> `"34"` : les arguments sont des textes, `+` les colle. Pour additionner, écrivez
> `ARGV[0].to_i + ARGV[1].to_i`, qui vaut `7`.

**Question 2 :** quelle est la différence entre `puts "x"` et `warn "x"` ?

> **Réponse**
>
> `puts` écrit sur la sortie standard (`$stdout`), `warn` sur la sortie d'erreur
> (`$stderr`). Une redirection `>` ne capture que la première.

**Question 3 :** déclarez une option `-t` / `--taille N` entière, et analysez le tableau
`["-t", "12", "photo.jpg"]`.

> **Réponse**
>
> ```ruby
> require "optparse"
>
> taille = nil
> parser = OptionParser.new do |o|
>   o.on("-t", "--taille N", Integer, "Taille voulue") { |n| taille = n }
> end
> fichiers = parser.parse(["-t", "12", "photo.jpg"])
> p taille
> p fichiers
> ```
>
> ```sortie
> 12
> ["photo.jpg"]
> ```

## En résumé

- `ARGV` contient les arguments sous forme de textes ; prévoyez le cas où il est vide.
- `OptionParser` déclare les options avec `on`, convertit leurs valeurs et écrit l'aide.
- `puts` écrit les résultats sur `$stdout`, `warn` les problèmes sur `$stderr`.
- `exit` et `abort` terminent avec un code de retour : `0` pour un succès.
- Un shebang `#!/usr/bin/env ruby` et `chmod +x` rendent un script exécutable.

Suite : [Faire plusieurs choses à la fois](concurrence.md)
