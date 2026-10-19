# Une base de données en Ruby

Cette page explique comment un programme Ruby range ses données dans une base de données SQLite, et comment l'écrire pour pouvoir changer de stockage sans tout réécrire.

> **Objectifs :**
>
> - vous saurez ce qu'est une table, une ligne et une colonne ;
> - vous saurez lire du code Ruby qui crée, remplit et interroge une base SQLite ;
> - vous saurez pourquoi on écrit `?` au lieu de coller les valeurs dans le SQL ;
> - vous saurez écrire un petit **dépôt** : une classe qui range et retrouve vos données ;
> - vous saurez changer de stockage sans toucher au reste du programme.

## Pourquoi une base de données ?

Avec [les formats](formats.md), vous savez déjà écrire vos données dans un fichier JSON.
Ça marche très bien pour 10 livres. Pour 100 000 livres, c'est lent : il faut tout
relire pour en trouver un seul.

Une base de données, c'est une **armoire à fiches bien rangée**. Vous demandez « le
livre numéro 42 », et elle vous le donne tout de suite, sans tout relire.

Le principe général est expliqué dans [les bases de données](../debuter/bases-de-donnees.md).
Ici, on voit le côté Ruby.

> **À retenir :** un fichier JSON se lit en entier ; une base de données répond à une
> question précise.

## Tables, lignes et colonnes

Une base range ses données dans des **tables**. Une table ressemble à un tableau de
tableur :

```text
Table « livres »
┌────┬──────────────────────────────────┬───────────────┐
│ id │ titre                            │ auteur        │   ← les colonnes
├────┼──────────────────────────────────┼───────────────┤
│  1 │ Le Petit Prince                  │ Saint-Exupéry │   ← une ligne
│  2 │ Vingt mille lieues sous les mers │ Verne         │   ← une autre ligne
└────┴──────────────────────────────────┴───────────────┘
```

- une **colonne** est une sorte d'information (le titre, l'auteur) ;
- une **ligne** est une fiche complète (un livre) ;
- l'**id** est un numéro unique : il désigne une ligne sans risque d'erreur.

On parle à la base avec un langage à part : **SQL**. Quatre phrases suffisent pour
commencer :

| SQL | En français |
|---|---|
| `INSERT INTO livres ...` | ajoute une ligne |
| `SELECT * FROM livres` | donne-moi les lignes |
| `UPDATE livres SET ...` | modifie des lignes |
| `DELETE FROM livres WHERE id = 1` | supprime la ligne numéro 1 |

## SQLite avec la gem sqlite3

**SQLite** est une base de données qui tient dans **un seul fichier**. Pas de serveur à
installer : c'est parfait pour apprendre. Ruby lui parle grâce à une
[gem](../debuter/glossaire.md#gem) (un paquet Ruby) :

```sh
gem install sqlite3
```

Les exemples de cette section ont besoin de cette gem : ils sont montrés sans être
lancés, avec leur sortie.

### Créer une table

```ruby norun
require "sqlite3"

db = SQLite3::Database.new("bibliotheque.db")
db.execute(<<~SQL)
  CREATE TABLE IF NOT EXISTS livres (
    id     INTEGER PRIMARY KEY,
    titre  TEXT NOT NULL,
    auteur TEXT NOT NULL
  )
SQL
puts "Table prête"
```

```text
Table prête
```

Ligne par ligne :

1. `require "sqlite3"` charge la gem ;
2. `Database.new` ouvre le fichier `bibliotheque.db` (il est créé s'il n'existe pas) ;
3. `CREATE TABLE` décrit les colonnes ; `IF NOT EXISTS` évite une erreur si la table
   existe déjà ;
4. `INTEGER PRIMARY KEY` : SQLite donne lui-même un `id` à chaque nouvelle ligne.

### Ajouter des lignes avec des ?

```ruby norun
require "sqlite3"

db = SQLite3::Database.new("bibliotheque.db")
db.execute("INSERT INTO livres (titre, auteur) VALUES (?, ?)",
           ["Le Petit Prince", "Saint-Exupéry"])
db.execute("INSERT INTO livres (titre, auteur) VALUES (?, ?)",
           ["Vingt mille lieues sous les mers", "Verne"])
puts db.last_insert_row_id
```

```text
2
```

Chaque `?` est une **place réservée**. La gem y met les valeurs du tableau, dans
l'ordre, en les protégeant. Ne collez **jamais** les valeurs vous-même dans le texte SQL
(voir [Erreurs fréquentes](#erreurs-frequentes)).

### Lire des lignes

```ruby norun
require "sqlite3"

db = SQLite3::Database.new("bibliotheque.db")
db.results_as_hash = true

db.execute("SELECT * FROM livres").each do |livre|
  puts "#{livre["id"]}. #{livre["titre"]} (#{livre["auteur"]})"
end

livre = db.get_first_row("SELECT * FROM livres WHERE id = ?", [2])
puts livre["titre"]
```

```text
1. Le Petit Prince (Saint-Exupéry)
2. Vingt mille lieues sous les mers (Verne)
Vingt mille lieues sous les mers
```

`results_as_hash = true` donne chaque ligne sous forme de hash : on lit une colonne par
son nom. Les clés sont des **textes** (`"titre"`), pas des symboles.

> **À retenir :** `execute` envoie du SQL ; les `?` reçoivent les valeurs ; chaque ligne
> revient sous forme de tableau ou de hash.

## ActiveRecord en deux mots

Dans une application Ruby on Rails, on écrit rarement du SQL à la main. **ActiveRecord**
transforme chaque table en [classe](../debuter/glossaire.md#classe) Ruby, et écrit le SQL
pour vous :

```ruby norun
class Livre < ActiveRecord::Base
end

Livre.create(titre: "Le Petit Prince", auteur: "Saint-Exupéry")
puts Livre.count
puts Livre.find(1).titre
```

```text
1
Le Petit Prince
```

C'est pratique, mais l'idée reste la même : **ajouter, trouver, lister, supprimer**. Ces
quatre actions, vous allez les écrire vous-même.

## Le dépôt : une boîte de rangement

Voici le problème. Si `db.execute(...)` est écrit partout dans votre programme, changer
de base devient un cauchemar : il faut tout relire.

La solution : ranger tout le stockage dans **une seule classe**, le **dépôt**. Le reste
du programme ne parle qu'au dépôt, jamais à la base.

```text
   votre programme                 le dépôt                    le stockage
 ┌─────────────────┐  ajouter   ┌────────────────┐         ┌───────────────┐
 │ afficher,       │ ─────────► │ ajouter        │ ──────► │ un Hash       │
 │ calculer,       │  trouver   │ trouver        │         │ ou un fichier │
 │ menus…          │ ◄───────── │ tous           │ ◄────── │ ou SQLite     │
 └─────────────────┘            │ supprimer      │         └───────────────┘
                                └────────────────┘
```

C'est comme un guichet de bibliothèque : vous demandez un livre au guichet. Vous ne
savez pas si les livres sont à la cave ou au grenier, et vous n'avez pas besoin de le
savoir.

> **Note :** en anglais, on dit *repository*. Ne confondez pas avec le
> [dépôt Git](../debuter/glossaire.md#depot), qui est autre chose.

### Un dépôt en mémoire

Commençons sans base de données : le dépôt range les livres dans un hash. La clé est
l'id, la valeur est le livre.

```ruby
class DepotLivres
  def initialize
    @livres = {}
    @prochain_id = 1
  end

  def ajouter(titre, auteur)
    livre = { id: @prochain_id, titre: titre, auteur: auteur }
    @livres[@prochain_id] = livre
    @prochain_id += 1
    livre
  end

  def trouver(id)
    @livres[id]
  end

  def tous
    @livres.values
  end

  def supprimer(id)
    !@livres.delete(id).nil?
  end
end

depot = DepotLivres.new
depot.ajouter("Le Petit Prince", "Saint-Exupéry")
depot.ajouter("Vingt mille lieues sous les mers", "Verne")

depot.tous.each do |livre|
  puts "#{livre[:id]}. #{livre[:titre]} (#{livre[:auteur]})"
end
puts depot.trouver(2)[:titre]
p depot.trouver(99)
```

```sortie
1. Le Petit Prince (Saint-Exupéry)
2. Vingt mille lieues sous les mers (Verne)
Vingt mille lieues sous les mers
nil
```

Les quatre méthodes, une par une :

| Méthode | Ce qu'elle fait | Ce qu'elle renvoie |
|---|---|---|
| `ajouter` | crée une fiche avec un nouvel id | la fiche créée |
| `trouver(id)` | cherche la fiche par son id | la fiche, ou `nil` |
| `tous` | liste toutes les fiches | un tableau |
| `supprimer(id)` | retire la fiche | `true` si elle existait |

Suivons `@prochain_id` et `@livres` pendant les deux ajouts :

| Étape | @prochain_id | clés de @livres |
|---|---|---|
| après `new` | 1 | (aucune) |
| après le 1er `ajouter` | 2 | 1 |
| après le 2e `ajouter` | 3 | 1, 2 |

> **Essayez :** ajoutez un troisième livre, puis affichez `depot.tous.size`. Vous
> obtenez `3`, et le nouveau livre a l'id `3`.

### Tester le dépôt

Un dépôt sert à tout le programme : vérifions qu'il marche. Pas besoin d'outil : une
petite méthode `verifier` affiche `OK` ou `ÉCHEC`. (Pour de vrais tests, voir
[les tests](tests.md).)

```ruby
class DepotLivres
  def initialize
    @livres = {}
    @prochain_id = 1
  end

  def ajouter(titre, auteur)
    livre = { id: @prochain_id, titre: titre, auteur: auteur }
    @livres[@prochain_id] = livre
    @prochain_id += 1
    livre
  end

  def trouver(id) = @livres[id]
  def tous = @livres.values
  def supprimer(id) = !@livres.delete(id).nil?
end

def verifier(nom, condition)
  puts "#{condition ? 'OK   ' : 'ÉCHEC'} #{nom}"
end

depot = DepotLivres.new
verifier("un dépôt neuf est vide", depot.tous.empty?)

livre = depot.ajouter("Les Misérables", "Hugo")
verifier("le premier id vaut 1", livre[:id] == 1)
verifier("on retrouve le livre", depot.trouver(1)[:titre] == "Les Misérables")
verifier("un id inconnu donne nil", depot.trouver(42).nil?)

verifier("supprimer un livre présent", depot.supprimer(1) == true)
verifier("supprimer deux fois", depot.supprimer(1) == false)
verifier("le dépôt est vide à nouveau", depot.tous.empty?)
```

```sortie
OK    un dépôt neuf est vide
OK    le premier id vaut 1
OK    on retrouve le livre
OK    un id inconnu donne nil
OK    supprimer un livre présent
OK    supprimer deux fois
OK    le dépôt est vide à nouveau
```

Ici, les méthodes courtes sont écrites sur une ligne avec `def nom = ...` (voir
[la méthode sur une ligne](fonctions.md#la-methode-sur-une-ligne)).

> **À retenir :** un dépôt a peu de méthodes, toujours les mêmes : ajouter, trouver,
> lister, supprimer.

## Changer de stockage

Voici l'intérêt du dépôt. Écrivons un **deuxième** dépôt, qui garde les livres dans un
fichier JSON. Il a **les mêmes méthodes**. La méthode `afficher_catalogue` marche avec
les deux, sans changer une ligne.

```ruby
require "json"

class DepotMemoire
  def initialize
    @livres = {}
  end

  def ajouter(titre)
    id = @livres.size + 1
    @livres[id] = { "id" => id, "titre" => titre }
  end

  def tous = @livres.values
end

class DepotJson
  def initialize(chemin)
    @chemin = chemin
    File.write(@chemin, "[]") unless File.exist?(@chemin)
  end

  def ajouter(titre)
    livres = tous
    livre = { "id" => livres.size + 1, "titre" => titre }
    File.write(@chemin, JSON.generate(livres + [livre]))
    livre
  end

  def tous = JSON.parse(File.read(@chemin))
end

def afficher_catalogue(depot)
  depot.ajouter("Le Petit Prince")
  depot.ajouter("Les Misérables")
  depot.tous.each { |livre| puts "  #{livre["id"]}. #{livre["titre"]}" }
end

puts "En mémoire :"
afficher_catalogue(DepotMemoire.new)
puts "Dans un fichier :"
afficher_catalogue(DepotJson.new("livres.json"))
puts File.read("livres.json")
```

```sortie
En mémoire :
  1. Le Petit Prince
  2. Les Misérables
Dans un fichier :
  1. Le Petit Prince
  2. Les Misérables
[{"id":1,"titre":"Le Petit Prince"},{"id":2,"titre":"Les Misérables"}]
```

`afficher_catalogue` ne sait pas où vont les livres. Il demande seulement
`ajouter` et `tous`. Ruby ne vérifie pas la classe de `depot` : il suffit que les
méthodes existent.

Le jour où vous voulez SQLite, vous écrivez un troisième dépôt avec les mêmes méthodes :

```ruby norun
require "sqlite3"

class DepotSqlite
  def initialize(chemin)
    @db = SQLite3::Database.new(chemin)
    @db.results_as_hash = true
    @db.execute("CREATE TABLE IF NOT EXISTS livres (id INTEGER PRIMARY KEY, titre TEXT)")
  end

  def ajouter(titre)
    @db.execute("INSERT INTO livres (titre) VALUES (?)", [titre])
    { "id" => @db.last_insert_row_id, "titre" => titre }
  end

  def tous = @db.execute("SELECT id, titre FROM livres")
end

afficher_catalogue(DepotSqlite.new("bibliotheque.db"))
```

```text
  1. Le Petit Prince
  2. Les Misérables
```

Le reste du programme ne change pas.

> **À retenir :** commencez avec un dépôt en mémoire. Quand vous aurez besoin d'une vraie
> base, vous changerez **une seule classe**.

## Erreurs fréquentes

### La gem sqlite3 n'est pas installée

```ruby norun
require "sqlite3"
```

```text
<internal:/opt/rbenv/versions/3.3.6/lib/ruby/3.3.0/rubygems/core_ext/kernel_require.rb>:136:in `require': cannot load such file -- sqlite3 (LoadError)
	from <internal:/opt/rbenv/versions/3.3.6/lib/ruby/3.3.0/rubygems/core_ext/kernel_require.rb>:136:in `require'
	from main.rb:1:in `<main>'
```

`cannot load such file` : Ruby ne trouve pas la gem. Installez-la avec
`gem install sqlite3`, ou ajoutez `gem "sqlite3"` dans le `Gemfile` du projet.

### Coller les valeurs dans le SQL

Un utilisateur tape un titre avec une apostrophe… ou pire :

```ruby
titre = "x'); DELETE FROM livres; --"
puts "INSERT INTO livres (titre) VALUES ('#{titre}')"
```

```sortie
INSERT INTO livres (titre) VALUES ('x'); DELETE FROM livres; --')
```

Le texte de l'utilisateur est devenu **une commande** : elle effacerait tous les livres.
C'est une **injection SQL** (voir [la sécurité](../debuter/securite.md)). La correction :
toujours des `?`, et les valeurs à part : `db.execute("... VALUES (?)", [titre])`.

### Oublier que trouver peut renvoyer nil

```ruby norun
livre = depot.trouver(42)
puts livre[:titre]
```

```text
main.rb:2:in `<main>': undefined method `[]' for nil (NoMethodError)

puts livre[:titre]
          ^^^^^^^^
```

Aucun livre n'a l'id 42 : `trouver` renvoie `nil`. Testez avant d'utiliser :
`puts livre[:titre] if livre`, ou `puts livre&.dig(:titre) || "Livre introuvable"`.

## Testez-vous

**Question 1 :** pourquoi écrit-on `VALUES (?, ?)` au lieu de mettre les valeurs
directement dans le texte SQL ?

> **Réponse**
>
> Pour que la gem protège les valeurs. Sinon, un texte tapé par un utilisateur peut
> devenir une commande SQL (injection SQL).

**Question 2 :** que renvoie `trouver` dans notre dépôt quand l'id n'existe pas ?

> **Réponse**
>
> `nil`, car c'est ce que renvoie un hash pour une clé absente.
>
> ```ruby
> livres = { 1 => { titre: "Le Petit Prince" } }
> p livres[7]
> ```
>
> ```sortie
> nil
> ```

**Question 3 :** vous passez d'un dépôt en mémoire à un dépôt SQLite. Que faut-il
changer dans le reste du programme ?

> **Réponse**
>
> Seulement la ligne qui **crée** le dépôt (`DepotSqlite.new(...)` au lieu de
> `DepotLivres.new`). Le reste appelle les mêmes méthodes : `ajouter`, `trouver`, `tous`,
> `supprimer`.

## En résumé

- Une base range les données en **tables** : des colonnes (les informations) et des
  lignes (les fiches), chacune avec un **id**.
- SQLite tient dans un fichier ; la gem `sqlite3` lui envoie du SQL avec `execute`.
- Les valeurs passent toujours par des `?`, jamais collées dans le texte SQL.
- Un **dépôt** regroupe tout le stockage : ajouter, trouver, lister, supprimer.
- Commencez avec un dépôt en mémoire (un hash) ; changez de stockage en changeant une
  seule classe.

Suite : [les expressions régulières](regex.md)
