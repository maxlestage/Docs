# Ruby pour le web

Cette page fait le tour de Ruby sur le web : une requête HTTP, une petite application Rack, Sinatra, Rails et les modèles ERB.

> **Objectifs :**
>
> - comprendre en quelques mots comment fonctionne le web ;
> - envoyer une requête HTTP avec `Net::HTTP` ;
> - écrire une application Rack et une application Sinatra minimales ;
> - savoir ce qu'est Rails et ce qu'il apporte ;
> - produire du HTML avec un modèle ERB.

## Le web en quatre lignes

1. Votre navigateur est un **client** : il envoie une **requête** (« donne-moi la page
   `/accueil` »).
2. Un **serveur** reçoit la requête et lance un programme, par exemple écrit en Ruby.
3. Le programme renvoie une **réponse** : un code (`200` = OK, `404` = introuvable), des
   en-têtes et un corps (souvent du HTML).
4. Le navigateur affiche le corps.

Le langage de cet échange s'appelle **HTTP**. C'est comme commander au comptoir : vous
demandez, le serveur prépare, il vous tend une assiette (ou vous dit « plus de frites »).
Pour plus de détails, lisez [le web pour débuter](../debuter/web.md).

## Envoyer une requête avec Net::HTTP

Ruby sait jouer le rôle du client grâce à `Net::HTTP`, livré avec Ruby. Ce code a besoin
d'Internet : il n'est pas exécuté ici.

```ruby norun
require "net/http"

uri = URI("https://example.com/")
reponse = Net::HTTP.get_response(uri)

puts reponse.code
puts reponse["content-type"]
puts reponse.body[0, 15]
```

```text
200
text/html
<!doctype html>
```

- `URI(...)` découpe l'adresse (protocole, domaine, chemin).
- `get_response` envoie une requête `GET` et renvoie la réponse.
- `code`, les en-têtes (`reponse["..."]`) et `body` sont les trois parties de la réponse.

> **Astuce :** pour lire une API qui renvoie du JSON, passez `reponse.body` à
> `JSON.parse` (voir [le format JSON](fichiers.md#le-format-json)).

## Rack : la base de tout

Côté serveur, presque tous les outils web Ruby reposent sur **Rack**. Rack fixe une règle
très simple : une application est un objet qui répond à `call(env)`.

- `env` est un hash qui décrit la requête (chemin, méthode…).
- `call` renvoie un tableau de trois éléments : le code, les en-têtes, le corps.

Cette règle est du Ruby ordinaire : vous pouvez appeler l'application à la main, sans
serveur.

```ruby
class Bonjour
  def call(env)
    chemin = env["PATH_INFO"]
    if chemin == "/"
      [200, { "content-type" => "text/plain" }, ["Bonjour !"]]
    else
      [404, { "content-type" => "text/plain" }, ["Introuvable"]]
    end
  end
end

app = Bonjour.new
p app.call({ "PATH_INFO" => "/" })
p app.call({ "PATH_INFO" => "/secret" })
```

```sortie
[200, {"content-type"=>"text/plain"}, ["Bonjour !"]]
[404, {"content-type"=>"text/plain"}, ["Introuvable"]]
```

### Lancer un vrai serveur

Pour servir l'application dans un navigateur, installez les
[gems](../debuter/glossaire.md#gem) `rack` et `rackup`, puis écrivez un fichier
`config.ru` (*ru* pour *rackup*) :

```ruby norun
# config.ru
require_relative "bonjour"

run Bonjour.new
```

```sh
gem install rack rackup
rackup
```

```text
Puma starting in single mode...
* Listening on http://127.0.0.1:9292
```

Ouvrez `http://localhost:9292` : la page affiche « Bonjour ! ». Le port et le serveur
affichés peuvent varier selon votre installation. `Ctrl+C` arrête le serveur.

## Sinatra : un site en quelques lignes

**Sinatra** est une petite [bibliothèque](../debuter/glossaire.md#bibliotheque) construite
sur Rack. Chaque **route** associe une méthode HTTP et un chemin à un bloc ; la valeur du
bloc est la réponse.

```ruby norun
# app.rb
require "sinatra"

get "/" do
  "Bonjour depuis Sinatra !"
end

get "/bonjour/:nom" do
  "Bonjour, #{params[:nom]} !"
end

post "/messages" do
  "Message reçu : #{params[:texte]}"
end
```

```sh
gem install sinatra rackup puma
ruby app.rb
```

Ouvrez `http://localhost:4567/bonjour/Alice` : la page affiche « Bonjour, Alice ! ».

- `get` répond aux pages que l'on consulte ; `post` aux formulaires envoyés.
- `:nom` dans le chemin est un morceau variable, retrouvé dans `params[:nom]`.

## Rails en bref

**Ruby on Rails** est le grand cadre de travail (*framework*) web de Ruby. GitHub,
Shopify ou Basecamp l'utilisent. Là où Sinatra donne une boîte à outils, Rails donne une
maison entière, avec des pièces déjà rangées.

### Créer un projet

```sh
gem install rails
rails new blog
cd blog
bin/rails server
```

`rails new` crée des dizaines de dossiers et de fichiers. Le site tourne sur
`http://localhost:3000`.

### Le modèle MVC

Rails range le code en trois rôles, le modèle **MVC** :

| Rôle | Dossier | Il s'occupe de… |
|------|---------|-----------------|
| **M**odèle | `app/models` | les données et leurs règles |
| **V**ue | `app/views` | le HTML affiché |
| **C**ontrôleur | `app/controllers` | recevoir la requête et choisir quoi faire |

Au restaurant : le contrôleur est le serveur qui prend la commande, le modèle est la
cuisine qui prépare, la vue est la présentation de l'assiette.

### Les générateurs

Les **générateurs** écrivent le code de départ à votre place :

```sh
bin/rails generate model Article titre:string contenu:text
bin/rails db:migrate
```

La première commande crée la classe `Article` et une migration (le plan de la table) ;
la seconde crée la table dans la base de données.

### ActiveRecord

**ActiveRecord** relie une classe Ruby à une table : chaque objet est une ligne. Vous
manipulez la base de données sans écrire de SQL.

```ruby norun
# app/models/article.rb
class Article < ApplicationRecord
  validates :titre, presence: true
end

# Dans la console : bin/rails console
article = Article.create(titre: "Bonjour", contenu: "Mon premier article")
Article.count                        # => 1
Article.find_by(titre: "Bonjour")    # => l'article
Article.where("titre LIKE ?", "B%")  # => tous les articles en B
```

## Les modèles ERB

Une page web mélange du HTML fixe et des valeurs qui changent. **ERB** (*Embedded Ruby*,
« Ruby intégré ») glisse du Ruby dans du texte. C'est le moteur des vues de Rails, et il
est livré avec Ruby : ces exemples s'exécutent.

- `<%= ... %>` calcule une valeur et l'**insère** dans le texte ;
- `<% ... %>` exécute du code **sans rien insérer** (une boucle, une condition).

```ruby
require "erb"

modele = ERB.new("Bonjour <%= nom %>, vous avez <%= age %> ans.")
puts modele.result_with_hash(nom: "Alice", age: 30)
```

```sortie
Bonjour Alice, vous avez 30 ans.
```

`result_with_hash` remplit le modèle avec les valeurs du hash.

### Une boucle dans un modèle

Avec `trim_mode: "-"`, les balises `<%-` et `-%>` suppriment les lignes vides laissées
par le code.

```ruby
require "erb"

modele = ERB.new(<<~MODELE, trim_mode: "-")
  <ul>
  <%- fruits.each do |fruit| -%>
    <li><%= fruit %></li>
  <%- end -%>
  </ul>
MODELE

puts modele.result_with_hash(fruits: %w[pomme poire kiwi])
```

```sortie
<ul>
  <li>pomme</li>
  <li>poire</li>
  <li>kiwi</li>
</ul>
```

### Échapper le HTML

Un texte venu d'un visiteur peut contenir des balises dangereuses. `ERB::Util.html_escape`
les rend inoffensives. Rails le fait automatiquement dans ses vues.

```ruby
require "erb"

commentaire = "<b>Super</b> & pas cher"
puts ERB::Util.html_escape(commentaire)
```

```sortie
&lt;b&gt;Super&lt;/b&gt; &amp; pas cher
```

## Erreurs fréquentes

### Oublier d'installer la gem

```ruby norun
require "sinatra"
```

```text
<internal:/opt/rbenv/versions/3.3.6/lib/ruby/3.3.0/rubygems/core_ext/kernel_require.rb>:136:in `require': cannot load such file -- sinatra (LoadError)
	from <internal:/opt/rbenv/versions/3.3.6/lib/ruby/3.3.0/rubygems/core_ext/kernel_require.rb>:136:in `require'
	from main.rb:1:in `<main>'
```

`cannot load such file` : la gem n'est pas installée. Lancez `gem install sinatra`, ou
ajoutez-la au `Gemfile` puis lancez `bundle install`.

### Une variable absente du modèle

```ruby norun
require "erb"
modele = ERB.new("Bonjour <%= nom %> !")
puts modele.result
```

```text
(erb):1:in `<main>': undefined local variable or method `nom' for main (NameError)
Did you mean?  not
	from /opt/rbenv/versions/3.3.6/lib/ruby/3.3.0/erb.rb:429:in `eval'
	from /opt/rbenv/versions/3.3.6/lib/ruby/3.3.0/erb.rb:429:in `result'
	from main.rb:3:in `<main>'
```

`(erb):1` indique la ligne 1 du **modèle**. Le modèle ne connaît pas `nom`. Donnez-lui
la valeur : `modele.result_with_hash(nom: "Alice")`.

### Confondre <% et <%=

```ruby norun
require "erb"
puts ERB.new("Bonjour <% nom %> !").result_with_hash(nom: "Alice")
```

```text
Bonjour  !
```

Pas d'erreur, mais le nom a disparu : `<% %>` exécute sans insérer. Pour afficher une
valeur, écrivez `<%= nom %>`.

## Testez-vous

**Question 1 :** que doit renvoyer la méthode `call` d'une application Rack ?

> **Réponse**
>
> Un tableau de trois éléments : le code HTTP (`200`), un hash d'en-têtes, et le corps
> (un objet qui répond à `each`, souvent un tableau de chaînes).

**Question 2 :** dans Rails, quel rôle reçoit la requête et décide quoi faire ?

> **Réponse**
>
> Le contrôleur (le « C » de MVC), rangé dans `app/controllers`.

**Question 3 :** écrivez un modèle ERB qui affiche « 3 articles dans le panier » à partir
d'un tableau `articles`.

> **Réponse**
>
> ```ruby
> require "erb"
> modele = ERB.new("<%= articles.size %> articles dans le panier")
> puts modele.result_with_hash(articles: %w[pain lait oeufs])
> ```
>
> ```sortie
> 3 articles dans le panier
> ```

## En résumé

- Le web est un échange de requêtes et de réponses HTTP entre un client et un serveur.
- `Net::HTTP` envoie des requêtes depuis Ruby.
- Une application Rack est un objet dont `call(env)` renvoie `[code, en-têtes, corps]`.
- Sinatra associe des routes à des blocs ; Rails organise tout un site en MVC avec
  ActiveRecord.
- ERB insère du Ruby dans du texte : `<%= %>` affiche, `<% %>` exécute seulement.

Suite : [Tester son code](tests.md)
