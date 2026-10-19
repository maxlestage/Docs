# JSON, CSV et YAML

Cette page montre comment enregistrer et relire des données dans les trois formats texte les plus courants : JSON, CSV et YAML.

> **Objectifs :**
>
> - vous saurez transformer un hash en JSON et relire du JSON avec `symbolize_names` ;
> - vous saurez lire un CSV avec ses en-têtes et en écrire un ;
> - vous saurez écrire et relire du YAML sans risque avec `safe_load` ;
> - vous saurez enregistrer ces formats dans des fichiers ;
> - vous saurez choisir le bon format selon la situation.

## Pourquoi un format ?

Vos hashs et vos tableaux vivent dans la mémoire du programme. Quand il s'arrête, ils
disparaissent. Pour les garder, ou les envoyer à un autre programme, il faut les écrire
sous forme de **texte**, selon des règles connues de tous : un **format**.

Transformer des données en texte s'appelle **sérialiser**. L'inverse, relire le texte
pour retrouver des données, s'appelle **désérialiser** (ou *parser*). C'est comme
plier une tente pour la ranger dans son sac, puis la déplier au camping.

Les trois formats de cette page sont fournis avec Ruby. Il suffit de les charger avec
`require`. Le chapitre [les fichiers](fichiers.md#le-format-json) en donnait un premier
aperçu.

## JSON

JSON est le format du web : les sites et les API l'utilisent pour échanger des données.
Il connaît les objets (des hashs), les tableaux, les textes, les nombres, `true`,
`false` et `null` (le `nil` de Ruby).

### Écrire du JSON

`JSON.generate` (ou `.to_json`) donne une ligne compacte. `JSON.pretty_generate` donne
un texte indenté, plus lisible pour un humain.

```ruby
require "json"

livre = { titre: "Dune", annee: 1965, genres: ["SF", "aventure"], lu: true, note: nil }

puts JSON.generate(livre)
puts livre.to_json
puts JSON.pretty_generate(livre)
```

```sortie
{"titre":"Dune","annee":1965,"genres":["SF","aventure"],"lu":true,"note":null}
{"titre":"Dune","annee":1965,"genres":["SF","aventure"],"lu":true,"note":null}
{
  "titre": "Dune",
  "annee": 1965,
  "genres": [
    "SF",
    "aventure"
  ],
  "lu": true,
  "note": null
}
```

Les [symboles](../debuter/glossaire.md#symbole) `:titre` deviennent des textes
`"titre"` : JSON ne connaît pas les symboles.

### Lire du JSON

`JSON.parse` fait le chemin inverse. Par défaut, les clés reviennent sous forme de
**textes**. Avec `symbolize_names: true`, elles redeviennent des symboles.

```ruby
require "json"

texte = '{"titre": "Dune", "annee": 1965, "genres": ["SF"]}'

donnees = JSON.parse(texte)
p donnees
puts donnees["titre"]

donnees = JSON.parse(texte, symbolize_names: true)
p donnees
puts donnees[:annee] + 1
```

```sortie
{"titre"=>"Dune", "annee"=>1965, "genres"=>["SF"]}
Dune
{:titre=>"Dune", :annee=>1965, :genres=>["SF"]}
1966
```

Les nombres restent des nombres : `1965` est un vrai entier, pas un texte.

## Écrire et relire un fichier

On écrit souvent ces formats dans des fichiers. Pour ne rien salir, les exemples
travaillent dans un **dossier temporaire** : `Dir.mktmpdir` en crée un, vous donne son
chemin, puis l'efface à la fin du bloc.

```ruby
require "json"
require "tmpdir"

Dir.mktmpdir do |dossier|
  chemin = File.join(dossier, "scores.json")

  scores = { "alice" => 120, "bob" => 95 }
  File.write(chemin, JSON.pretty_generate(scores))

  relu = JSON.parse(File.read(chemin))
  relu["bob"] += 10
  p relu
end
```

```sortie
{"alice"=>120, "bob"=>105}
```

`File.join` assemble un chemin avec le bon séparateur, quel que soit le système.

## CSV

CSV (*comma-separated values*) est un tableau sous forme de texte : une ligne par
enregistrement, des virgules entre les colonnes. Les tableurs (Excel, LibreOffice) savent
l'ouvrir et l'enregistrer.

### Lire un CSV avec ses en-têtes

Avec `headers: true`, la première ligne donne le nom des colonnes. Chaque ligne se lit
alors comme un hash : `ligne["nom"]`.

```ruby
require "csv"

texte = <<~CSV
  nom,ville,age
  Alice,Nantes,30
  Bob,Lyon,25
  Chloé,"Paris, France",41
CSV

CSV.parse(texte, headers: true).each do |ligne|
  puts "#{ligne["nom"]} (#{ligne["age"]} ans) : #{ligne["ville"]}"
end
```

```sortie
Alice (30 ans) : Nantes
Bob (25 ans) : Lyon
Chloé (41 ans) : Paris, France
```

Les guillemets protègent une virgule à l'intérieur d'une valeur : `"Paris, France"`
reste une seule colonne. C'est pour cela qu'il vaut mieux utiliser `csv` plutôt qu'un
simple `split(",")`.

### Convertir les valeurs

En CSV, tout est texte : `"30"` n'est pas un nombre. `converters: :numeric` convertit
les nombres, et `header_converters: :symbol` transforme les en-têtes en symboles.

```ruby
require "csv"

texte = "produit,prix,stock\nstylo,1.5,40\ncahier,3.2,12\n"

lignes = CSV.parse(texte, headers: true, converters: :numeric, header_converters: :symbol)
total = lignes.sum { |l| l[:prix] * l[:stock] }
puts total.round(2)
p lignes.first.to_h
```

```sortie
98.4
{:produit=>"stylo", :prix=>1.5, :stock=>40}
```

### Écrire un CSV

`CSV.generate` construit le texte ligne par ligne avec `<<`. `CSV.open` fait la même
chose directement dans un fichier, et `CSV.read` le relit.

```ruby
require "csv"
require "tmpdir"

puts CSV.generate { |csv| csv << ["nom", "note"]; csv << ["Léa", 14] }

Dir.mktmpdir do |dossier|
  chemin = File.join(dossier, "notes.csv")

  CSV.open(chemin, "w") do |csv|
    csv << ["nom", "note"]
    csv << ["Tom", 9]
    csv << ["Zoé", 17]
  end

  p CSV.read(chemin, headers: true).map { |l| l["nom"] }
end
```

```sortie
nom,note
Léa,14
["Tom", "Zoé"]
```

> **Note :** depuis Ruby 3.4, `csv` est une *gem* livrée avec Ruby mais plus chargée
> d'office. Si vous utilisez un `Gemfile`, ajoutez-y la ligne `gem "csv"` (voir
> [gem](../debuter/glossaire.md#gem)).

## YAML

YAML est fait pour être lu et écrit par des humains : pas d'accolades, l'indentation
montre l'imbrication. On le trouve surtout dans les fichiers de configuration.

### Écrire et relire

`YAML.dump` transforme des données en YAML, `YAML.load` les relit.

```ruby
require "yaml"

config = {
  "nom" => "Mon site",
  "port" => 8080,
  "langues" => ["fr", "en"],
  "base" => { "hote" => "localhost", "nom" => "dev" }
}

texte = YAML.dump(config)
puts texte
p YAML.load(texte)["base"]["hote"]
```

```sortie
---
nom: Mon site
port: 8080
langues:
- fr
- en
base:
  hote: localhost
  nom: dev
"localhost"
```

Le `---` du début marque le commencement d'un document YAML.

### Relire sans risque avec safe_load

Un YAML peut décrire des objets Ruby de n'importe quelle classe. Lire un fichier venu
d'ailleurs pourrait donc créer des objets dangereux. `YAML.safe_load` n'accepte que
les types simples : textes, nombres, booléens, `nil`, tableaux et hashs. Pour en
autoriser d'autres, nommez-les avec `permitted_classes`.

```ruby
require "yaml"
require "tmpdir"

Dir.mktmpdir do |dossier|
  chemin = File.join(dossier, "config.yml")
  File.write(chemin, <<~YAML)
    titre: Mes notes
    actif: true
    taille: 12
    theme: :sombre
  YAML

  config = YAML.safe_load(File.read(chemin), permitted_classes: [Symbol])
  p config
end
```

```sortie
{"titre"=>"Mes notes", "actif"=>true, "taille"=>12, "theme"=>:sombre}
```

> **À retenir :** pour un fichier qui ne vient pas de vous, utilisez toujours
> `YAML.safe_load`. Depuis Ruby 3.1, `YAML.load` est d'ailleurs prudent par défaut.

## Quel format choisir ?

| Format | Points forts | Quand l'utiliser |
|--------|--------------|------------------|
| JSON | universel, strict, imbrication libre | échanger avec un site, une API, un autre langage |
| CSV | ouvert par les tableurs, très compact | des lignes toutes de la même forme (un tableau) |
| YAML | agréable à lire et à modifier à la main | des fichiers de configuration |

## Erreurs fréquentes

### Oublier require "json"

```ruby norun
puts({ nom: "Alice" }.to_json)
```

```text
main.rb:1:in `<main>': undefined method `to_json' for an instance of Hash (NoMethodError)

puts({ nom: "Alice" }.to_json)
                     ^^^^^^^^
Did you mean?  to_s
```

`to_json` n'existe qu'après `require "json"`. Ajoutez cette ligne en haut du fichier.

### Lire une clé symbole dans un JSON relu

```ruby norun
require "json"
data = JSON.parse(%q({"nom": "Alice"}))
puts data[:nom].upcase
```

```text
main.rb:3:in `<main>': undefined method `upcase' for nil (NoMethodError)

puts data[:nom].upcase
               ^^^^^^^
```

Sans option, les clés sont des textes : écrivez `data["nom"]`, ou relisez avec
`JSON.parse(texte, symbolize_names: true)`.

### Un JSON mal formé

```ruby norun
require "json"
JSON.parse("{nom: 1}")
```

```text
/opt/rbenv/versions/3.3.6/lib/ruby/3.3.0/json/common.rb:220:in `parse': unexpected token at '{nom: 1}' (JSON::ParserError)
	from /opt/rbenv/versions/3.3.6/lib/ruby/3.3.0/json/common.rb:220:in `parse'
	from main.rb:2:in `<main>'
```

En JSON, les clés sont **toujours** entre guillemets doubles : `{"nom": 1}`. Si le
texte vient de l'extérieur, entourez `JSON.parse` d'un `rescue JSON::ParserError`
(voir [gérer les erreurs](erreurs.md)).

### Relire une classe non autorisée en YAML

```ruby norun
require "yaml"
require "date"
texte = YAML.dump({ "jour" => Date.new(2024, 3, 1) })
p YAML.load(texte)
```

```text
/opt/rbenv/versions/3.3.6/lib/ruby/3.3.0/psych/class_loader.rb:99:in `find': Tried to load unspecified class: Date (Psych::DisallowedClass)
	from /opt/rbenv/versions/3.3.6/lib/ruby/3.3.0/psych/class_loader.rb:28:in `load'
	(… une vingtaine de lignes from …)
	from main.rb:4:in `<main>'
```

Par prudence, `Date` n'est pas acceptée. Autorisez-la :
`YAML.load(texte, permitted_classes: [Date])`.

## Testez-vous

**Question 1 :** que donne `JSON.parse('{"a": 1}')["a"]` ? Et avec `[:a]` ?

> **Réponse**
>
> `1` avec `["a"]`, et `nil` avec `[:a]`, car les clés relues sont des textes.

**Question 2 :** dans un CSV lu avec `headers: true`, quel est le type de la valeur
`"30"` de la colonne `age` ?

> **Réponse**
>
> Un texte (`String`). Ajoutez `converters: :numeric` pour obtenir un nombre.

**Question 3 :** transformez `{ "ville" => "Lyon", "habitants" => 520000 }` en JSON
indenté, puis relisez-le pour afficher les habitants.

> **Réponse**
>
> ```ruby
> require "json"
>
> texte = JSON.pretty_generate({ "ville" => "Lyon", "habitants" => 520000 })
> puts texte
> puts JSON.parse(texte)["habitants"]
> ```
>
> ```sortie
> {
>   "ville": "Lyon",
>   "habitants": 520000
> }
> 520000
> ```

## En résumé

- `require "json"` : `JSON.generate` ou `pretty_generate` pour écrire, `JSON.parse` pour lire.
- `symbolize_names: true` rend des clés symboles ; sinon ce sont des textes.
- `require "csv"` : `CSV.parse(texte, headers: true)` donne des lignes lisibles par nom de colonne.
- `require "yaml"` : `YAML.dump` pour écrire, `YAML.safe_load` pour relire sans risque.
- JSON pour échanger, CSV pour des tableaux, YAML pour la configuration.

Suite : [Une base de données en Ruby](base-de-donnees.md)
