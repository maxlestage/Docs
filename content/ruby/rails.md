# Un premier site avec Rails, pas à pas

Cette page vous guide dans un premier site Ruby on Rails : les routes, un contrôleur, une vue, un modèle, puis le voyage complet d'une requête, étape par étape.

> **Objectifs :**
>
> - comprendre le rôle de chaque morceau de Rails grâce à l'image du restaurant ;
> - créer un projet avec `rails new` et lancer le serveur ;
> - écrire une route, un contrôleur et une vue ERB ;
> - créer un modèle et sa migration, puis tout générer d'un coup avec un *scaffold* ;
> - suivre une requête du navigateur jusqu'à la page affichée.

## Rails, c'est quoi ?

**Ruby on Rails** (on dit souvent « Rails ») est un *framework* web. Un *framework*,
c'est un cadre de travail : une maison déjà construite, avec des pièces rangées. Vous
ajoutez vos meubles ; vous ne coulez pas les fondations.

La page [Ruby pour le web](web.md) montre Rack et Sinatra. Rails repose aussi sur Rack,
mais il apporte beaucoup plus : base de données, formulaires, sécurité, tests.

Rails range le code en trois rôles, appelés **MVC** : Modèle, Vue, Contrôleur.

### Le restaurant Rails

Imaginez un restaurant :

```text
   Vous (le navigateur)
        |
        |  « Je voudrais la liste des articles »   (la requête)
        v
  +-------------+     « 3 articles, svp »    +-------------+
  |  SERVEUR    | -------------------------> |  CUISINE    |
  | contrôleur  | <------------------------- |  modèle     |
  +-------------+     les 3 articles         +-------------+
        |
        |  il dresse l'assiette selon la carte
        v
  +-------------+
  |  CARTE /    |
  |  ASSIETTE   |   la vue : ce que vous voyez
  +-------------+
        |
        v
   Vous recevez la page HTML                 (la réponse)
```

| Restaurant | Rails | Dossier | Son travail |
|------------|-------|---------|-------------|
| le plan de salle | les **routes** | `config/routes.rb` | dire quelle demande va à quel serveur |
| le serveur | le **contrôleur** | `app/controllers` | prendre la commande, parler à la cuisine |
| la cuisine | le **modèle** | `app/models` | préparer les données (la base de données) |
| la carte et l'assiette | la **vue** | `app/views` | présenter le résultat au client |

Le serveur ne cuisine jamais. La cuisine ne parle jamais au client. Chacun son travail :
c'est ce qui rend un gros site facile à ranger.

### Le restaurant en Ruby ordinaire

Voici l'idée en Ruby pur, sans Rails. Chaque rôle est une petite
[classe](../debuter/glossaire.md#classe).

```ruby
class Cuisine # le modèle
  def self.plats = ["Soupe", "Gratin", "Tarte"]
end

class Carte # la vue
  def self.afficher(plats) = plats.map { |p| "- #{p}" }.join("\n")
end

class Serveur # le contrôleur
  def commande = Carte.afficher(Cuisine.plats)
end

puts Serveur.new.commande
```

```sortie
- Soupe
- Gratin
- Tarte
```

Essayez : ajoutez `"Salade"` dans `Cuisine.plats`. Seul le modèle change, et pourtant
la page affiche une ligne de plus. Le serveur et la carte n'ont rien vu.

> **À retenir :** MVC = trois rôles séparés. Le contrôleur reçoit la demande, le modèle
> fournit les données, la vue les présente.

## Installer Rails et créer un projet

Rails est une [gem](../debuter/glossaire.md#gem) (une bibliothèque Ruby). Ces commandes
se tapent dans le [terminal](../debuter/glossaire.md#terminal). Elles ont besoin
d'Internet : elles ne sont pas exécutées ici.

```sh
gem install rails
rails new blog
```

```text
Successfully installed rails-8.0.2
      create  README.md
      create  app/controllers/application_controller.rb
      ...
Bundle complete! 22 Gemfile dependencies, 108 gems now installed.
```

Les numéros de version peuvent être différents chez vous. `rails new` construit la maison : des dizaines de dossiers et de fichiers. Voici les
pièces qui comptent pour débuter :

```text
blog/
├── app/
│   ├── controllers/   ← les serveurs (contrôleurs)
│   ├── models/        ← la cuisine (modèles)
│   └── views/         ← les cartes (vues ERB)
├── config/
│   └── routes.rb      ← le plan de salle (routes)
├── db/
│   └── migrate/       ← les plans des tables
└── Gemfile            ← la liste des gems (voir projet.md)
```

### Lancer le serveur

```sh
cd blog
bin/rails server
```

```text
=> Rails 8.0.2 application starting in development
Puma starting in single mode...
* Listening on http://127.0.0.1:3000
Use Ctrl-C to stop
```

Ouvrez `http://localhost:3000` dans votre navigateur : Rails affiche sa page d'accueil.
`Ctrl+C` dans le terminal arrête le serveur.

Essayez : lancez `bin/rails server -p 4000`. Le site répond alors sur le port `4000`
au lieu de `3000`.

> **À retenir :** `rails new nom` crée le projet ; `bin/rails server` le lance sur
> `http://localhost:3000`.

## Les routes : le plan de salle

Une **route** relie une adresse ([URL](../debuter/glossaire.md#url)) à une action d'un
contrôleur. On l'écrit dans `config/routes.rb` :

```ruby norun
# config/routes.rb
Rails.application.routes.draw do
  root "pages#accueil"           # la page d'accueil : /
  get "/articles", to: "articles#index"
end
```

`"articles#index"` se lit : « le contrôleur `articles`, la méthode `index` ».
`bin/rails routes` affiche le plan :

```sh
bin/rails routes
```

```text
  Prefix Verb URI Pattern         Controller#Action
    root GET  /                   pages#accueil
articles GET  /articles(.:format) articles#index
```

### Un routeur en Ruby ordinaire

Un routeur, c'est surtout un [dictionnaire](../debuter/glossaire.md#dictionnaire) : une
adresse donne le nom d'une [méthode](../debuter/glossaire.md#methode). `send` appelle la
méthode dont on lui donne le nom.

```ruby
ROUTES = {
  "/" => :accueil,
  "/articles" => :index
}

def accueil = "Bienvenue sur le blog"
def index = "Liste des articles"

["/", "/articles", "/contact"].each do |chemin|
  action = ROUTES[chemin]
  if action
    puts "#{chemin} -> #{send(action)}"
  else
    puts "#{chemin} -> 404 : page introuvable"
  end
end
```

```sortie
/ -> Bienvenue sur le blog
/articles -> Liste des articles
/contact -> 404 : page introuvable
```

Suivons la boucle pas à pas :

| Étape | chemin | action | Ce qui s'affiche |
|-------|--------|--------|------------------|
| 1 | `"/"` | `:accueil` | `/ -> Bienvenue sur le blog` |
| 2 | `"/articles"` | `:index` | `/articles -> Liste des articles` |
| 3 | `"/contact"` | `nil` (pas dans le hash) | `/contact -> 404 : page introuvable` |

Essayez : ajoutez `"/contact" => :contact` dans `ROUTES` et une méthode
`def contact = "Écrivez-nous"`. La troisième ligne devient `/contact -> Écrivez-nous`.

> **À retenir :** une route dit « telle adresse → tel contrôleur#action ». Une adresse
> sans route donne une erreur 404.

## Le contrôleur : le serveur du restaurant

Un **contrôleur** est une classe. Chaque méthode publique est une **action** : une
commande que le serveur sait prendre.

```ruby norun
# app/controllers/articles_controller.rb
class ArticlesController < ApplicationController
  def index
    @articles = ["Bonjour Rails", "Mes vacances", "Une recette"]
  end
end
```

Deux choses à remarquer :

- `@articles` commence par `@` : c'est une **variable d'instance**. Rails la donne
  automatiquement à la vue.
- Il n'y a pas de `render` : par défaut, Rails affiche la vue qui porte le même nom
  que l'action, ici `app/views/articles/index.html.erb`.

La commande `bin/rails generate controller Articles index` écrit ces deux fichiers
pour vous.

> **À retenir :** une action de contrôleur prépare les données dans des variables `@…`,
> puis Rails affiche la vue du même nom.

## La vue : la page en ERB

Une **vue** est un modèle ERB : du [HTML](../debuter/glossaire.md#html) avec des trous
remplis par Ruby (voir [les modèles ERB](web.md)).

- `<%= ... %>` calcule et **insère** une valeur ;
- `<% ... %>` exécute sans rien insérer (une boucle, une condition).

```text
<%# app/views/articles/index.html.erb %>
<h1>Les articles</h1>
<ul>
  <% @articles.each do |titre| %>
    <li><%= titre %></li>
  <% end %>
</ul>
```

### La même vue avec la bibliothèque erb

ERB est livré avec Ruby. Voici le même travail, sans Rails. `binding` donne au modèle
accès aux variables `@…` de l'objet, comme Rails le fait pour vous.

```ruby
require "erb"

VUE = <<~ERB
  <h1>Les articles</h1>
  <ul>
  <%- @articles.each do |titre| -%>
    <li><%= titre %></li>
  <%- end -%>
  </ul>
ERB

class ArticlesController
  def index
    @articles = ["Bonjour Rails", "Mes vacances"]
    ERB.new(VUE, trim_mode: "-").result(binding)
  end
end

puts ArticlesController.new.index
```

```sortie
<h1>Les articles</h1>
<ul>
  <li>Bonjour Rails</li>
  <li>Mes vacances</li>
</ul>
```

`trim_mode: "-"` et les balises `<%-` `-%>` évitent des lignes vides dans le résultat.
Le modèle se remplit ligne après ligne :

| Étape | Ligne du modèle | titre | Texte produit |
|-------|-----------------|-------|---------------|
| 1 | `<h1>Les articles</h1>` | — | `<h1>Les articles</h1>` |
| 2 | `<ul>` | — | `<ul>` |
| 3 | `@articles.each` (1er tour) | `"Bonjour Rails"` | rien |
| 4 | `<li><%= titre %></li>` | `"Bonjour Rails"` | `  <li>Bonjour Rails</li>` |
| 5 | `@articles.each` (2e tour) | `"Mes vacances"` | rien |
| 6 | `<li><%= titre %></li>` | `"Mes vacances"` | `  <li>Mes vacances</li>` |
| 7 | `</ul>` | — | `</ul>` |

Essayez : remplacez `<%= titre %>` par `<%= titre.upcase %>`. Les titres s'affichent en
majuscules : `BONJOUR RAILS`, `MES VACANCES`.

> **À retenir :** la vue ne fait que présenter. Elle lit les variables `@…` préparées
> par le contrôleur.

## Le modèle et sa migration

Pour l'instant, les articles sont écrits en dur dans le contrôleur. Un vrai site les
range dans une [base de données](base-de-donnees.md). C'est le travail du **modèle**.

```sh
bin/rails generate model Article titre:string contenu:text
```

Cette commande crée `app/models/article.rb` et une migration
`db/migrate/20261019120000_create_articles.rb`. Le long nombre est la date et l'heure de
création : il change chez vous.

### La migration : le plan de la table

Une **migration** décrit une modification de la base. Celle-ci crée une table
`articles` avec deux colonnes :

```ruby norun
# db/migrate/20261019120000_create_articles.rb
class CreateArticles < ActiveRecord::Migration[8.0]
  def change
    create_table :articles do |t|
      t.string :titre
      t.text :contenu

      t.timestamps
    end
  end
end
```

`t.timestamps` ajoute deux colonnes automatiques : `created_at` (créé le) et
`updated_at` (modifié le). Le plan ne fait rien tant qu'on ne l'applique pas :

```sh
bin/rails db:migrate
```

```text
== 20261019120000 CreateArticles: migrating ===================================
-- create_table(:articles)
   -> 0.0021s
== 20261019120000 CreateArticles: migrated (0.0022s) ==========================
```

```text
Table articles (vide pour l'instant)
+----+-------+---------+------------+------------+
| id | titre | contenu | created_at | updated_at |
+----+-------+---------+------------+------------+
```

### Le modèle : une ligne = un objet

Le fichier `app/models/article.rb` est presque vide. Pourtant, `Article` sait déjà
lire et écrire dans la table : c'est **ActiveRecord**.

```ruby norun
# app/models/article.rb
class Article < ApplicationRecord
  validates :titre, presence: true
end
```

`validates` ajoute une règle : un article sans titre est refusé. Dans la console
(`bin/rails console`), `Article.create(titre: "Bonjour")` ajoute une ligne et
`Article.count` répond `1`.

### L'idée en Ruby ordinaire

Voici un modèle de poche, sans base de données : un tableau garde les lignes.

```ruby
Article = Struct.new(:id, :titre)
LIGNES = [] # la « table »

def creer_article(titre)
  return nil if titre.empty? # la règle « titre obligatoire »
  LIGNES << Article.new(LIGNES.size + 1, titre)
end

creer_article("Bonjour Rails")
creer_article("")
creer_article("Mes vacances")

puts "Nombre : #{LIGNES.size}"
LIGNES.each { |a| puts "#{a.id}. #{a.titre}" }
```

```sortie
Nombre : 2
1. Bonjour Rails
2. Mes vacances
```

| Étape | Appel | Titre vide ? | Lignes après |
|-------|-------|--------------|--------------|
| 1 | `creer_article("Bonjour Rails")` | non | 1 ligne |
| 2 | `creer_article("")` | oui → `nil`, rien n'est ajouté | 1 ligne |
| 3 | `creer_article("Mes vacances")` | non | 2 lignes |

Essayez : remplacez `""` par `"Une recette"`. Le compteur passe à `3` et une troisième
ligne s'affiche.

> **À retenir :** une migration crée ou modifie une table ; un modèle ActiveRecord fait
> d'une ligne de la table un objet Ruby.

## Le scaffold : tout d'un coup

*Scaffold* veut dire « échafaudage ». Une seule commande crée le modèle, la migration,
le contrôleur, les vues et les routes pour **lister, voir, créer, modifier et supprimer**
des articles.

```sh
bin/rails generate scaffold Article titre:string contenu:text
bin/rails db:migrate
```

Dans `config/routes.rb`, la ligne `resources :articles` crée sept routes d'un coup :

| Adresse | Verbe | Action | Ce qu'elle fait |
|---------|-------|--------|-----------------|
| `/articles` | GET | `index` | lister les articles |
| `/articles/new` | GET | `new` | afficher le formulaire vide |
| `/articles` | POST | `create` | enregistrer le nouvel article |
| `/articles/1` | GET | `show` | voir l'article n° 1 |
| `/articles/1/edit` | GET | `edit` | afficher le formulaire rempli |
| `/articles/1` | PATCH | `update` | enregistrer les modifications |
| `/articles/1` | DELETE | `destroy` | supprimer l'article |

Ouvrez `http://localhost:3000/articles` : vous pouvez déjà créer des articles avec un
formulaire. Le scaffold est parfait pour apprendre : lisez les fichiers créés, un par un.

> **À retenir :** `generate scaffold` écrit tout le MVC d'une ressource. C'est un point
> de départ à lire et à modifier, pas du code magique.

## Le voyage d'une requête, étape par étape

Vous tapez `http://localhost:3000/articles` et appuyez sur Entrée. Que se passe-t-il ?

| Étape | Qui travaille | Ce qui se passe | Fichier |
|-------|---------------|-----------------|---------|
| 1 | le navigateur | envoie la requête `GET /articles` | — |
| 2 | le serveur Puma | reçoit la requête et la passe à Rails | — |
| 3 | le routeur | trouve `GET /articles` → `articles#index` | `config/routes.rb` |
| 4 | le contrôleur | crée un `ArticlesController` et appelle `index` | `app/controllers/articles_controller.rb` |
| 5 | le modèle | `Article.all` lit la table `articles` | `app/models/article.rb` |
| 6 | le contrôleur | range le résultat dans `@articles` | `app/controllers/articles_controller.rb` |
| 7 | la vue | remplit le modèle ERB avec `@articles` | `app/views/articles/index.html.erb` |
| 8 | Rails | renvoie la réponse `200` avec le HTML | — |
| 9 | le navigateur | affiche la page | — |

### Le voyage complet en Ruby ordinaire

Ce programme rejoue les étapes 3 à 8 et affiche chacune d'elles.

```ruby
require "erb"

class Article # le modèle
  def self.all = ["Bonjour Rails", "Mes vacances"]
end

class ArticlesController # le contrôleur
  VUE = "<ul><% @articles.each do |t| %><li><%= t %></li><% end %></ul>"

  def index
    puts "4. le contrôleur appelle le modèle"
    @articles = Article.all
    puts "5-6. @articles contient #{@articles.size} titres"
    puts "7. la vue remplit le modèle ERB"
    ERB.new(VUE).result(binding)
  end
end

ROUTES = { ["GET", "/articles"] => [ArticlesController, :index] }

def traiter(verbe, chemin)
  puts "3. le routeur cherche #{verbe} #{chemin}"
  classe, action = ROUTES[[verbe, chemin]]
  return [404, "Introuvable"] unless classe

  html = classe.new.send(action)
  puts "8. réponse 200"
  [200, html]
end

code, corps = traiter("GET", "/articles")
puts corps
```

```sortie
3. le routeur cherche GET /articles
4. le contrôleur appelle le modèle
5-6. @articles contient 2 titres
7. la vue remplit le modèle ERB
8. réponse 200
<ul><li>Bonjour Rails</li><li>Mes vacances</li></ul>
```

Essayez : appelez `traiter("GET", "/contact")`. Seule la ligne `3.` s'affiche, puis
`corps` vaut `"Introuvable"` : la requête s'arrête au routeur, comme une erreur 404.

> **À retenir :** une requête suit toujours le même chemin : route → contrôleur →
> modèle → vue → réponse. Quand quelque chose casse, cherchez à quelle étape.

## Erreurs fréquentes

### Une adresse sans route

Vous ouvrez `http://localhost:3000/articlez` (avec une faute de frappe). Rails affiche :

```text
Routing Error
No route matches [GET] "/articlez"
```

Aucune ligne de `config/routes.rb` ne correspond. Vérifiez l'adresse, ou ajoutez la
route. `bin/rails routes` liste toutes les routes existantes.

### Oublier de remplir la variable de la vue

```ruby norun
require "erb"

class ArticlesController
  def index
    # oubli : @articles = [...]
    ERB.new("<% @articles.each do |a| %><%= a %> <% end %>").result(binding)
  end
end

puts ArticlesController.new.index
```

```text
(erb):1:in `index': undefined method `each' for nil (NoMethodError)
	from /opt/rbenv/versions/3.3.6/lib/ruby/3.3.0/erb.rb:429:in `eval'
	from /opt/rbenv/versions/3.3.6/lib/ruby/3.3.0/erb.rb:429:in `result'
	from main.rb:6:in `index'
	from main.rb:10:in `<main>'
```

Une variable `@…` jamais remplie vaut `nil`. Dans Rails, c'est le message le plus
courant dans une vue. Remplissez `@articles` dans l'action du contrôleur, avec le même
nom exactement.

### Une migration pas encore appliquée

Vous avez généré un modèle, mais oublié `bin/rails db:migrate`. La page affiche :

```text
ActiveRecord::PendingMigrationError
Migrations are pending. To resolve this issue, run:

        bin/rails db:migrate
```

Le message donne la solution : lancez `bin/rails db:migrate`, puis rechargez la page.

## Testez-vous

**Question 1 :** dans l'image du restaurant, qui est le contrôleur, et que fait-il ?

> **Réponse**
>
> Le serveur. Il reçoit la commande (la requête), demande les données à la cuisine (le
> modèle) et les fait présenter par la carte (la vue). Il ne cuisine pas lui-même.

**Question 2 :** l'action `index` du contrôleur `ArticlesController` n'appelle pas
`render`. Quelle vue Rails affiche-t-il ?

> **Réponse**
>
> `app/views/articles/index.html.erb` : la vue qui porte le nom du contrôleur et de
> l'action.

**Question 3 :** complétez ce routeur pour que `/bonjour` affiche `Salut !`.

> **Réponse**
>
> ```ruby
> ROUTES = { "/bonjour" => :bonjour }
>
> def bonjour = "Salut !"
>
> puts send(ROUTES["/bonjour"])
> ```
>
> ```sortie
> Salut !
> ```

## En résumé

- Rails range un site en MVC : le contrôleur reçoit, le modèle fournit les données, la
  vue présente.
- `rails new` crée le projet, `bin/rails server` le lance sur le port 3000.
- Une route relie une adresse à `contrôleur#action` dans `config/routes.rb`.
- Une migration crée la table ; un modèle ActiveRecord fait d'une ligne un objet.
- `generate scaffold` écrit tout le MVC d'une ressource d'un coup.
- Une requête suit : route → contrôleur → modèle → vue → réponse.

Suite : [Tester son code](tests.md)
