# Struct, Data et Set

Cette page présente trois outils tout prêts pour ranger des données : `Struct` pour un petit objet modifiable, `Data` pour une valeur qui ne change pas, et `Set` pour un ensemble sans doublons.

> **Objectifs :**
>
> - vous saurez créer une petite classe en une ligne avec `Struct` ;
> - vous saurez créer une valeur immuable avec `Data.define` ;
> - vous saurez reconnaître un `OpenStruct` et pourquoi l'éviter ;
> - vous saurez utiliser un `Set` : ajouter, tester, unir, croiser ;
> - vous saurez choisir le bon outil pour vos données.

## Struct : une classe en une ligne

Souvent, une classe ne sert qu'à regrouper quelques champs : un nom, un âge, une ville.
Écrire `initialize` et `attr_accessor` à chaque fois est répétitif. `Struct` le fait pour
vous, comme un formulaire pré-imprimé qu'il ne reste qu'à remplir.

```ruby
Point = Struct.new(:x, :y)

a = Point.new(3, 4)
puts a.x
a.y = 10
p a
p a.to_a
puts a == Point.new(3, 10)
```

```sortie
3
#<struct Point x=3, y=10>
[3, 10]
true
```

Un `Struct` vous donne :

- `new` avec les champs dans l'ordre ;
- un accesseur en lecture et en écriture pour chaque champ ;
- un affichage lisible avec `p` ;
- `==` qui compare les champs, pas l'identité des objets.

### Avec des arguments nommés

Avec `keyword_init: true`, on donne les champs par leur nom. C'est plus clair quand il
y en a beaucoup, et l'ordre n'a plus d'importance.

```ruby
Contact = Struct.new(:nom, :email, :ville, keyword_init: true)

c = Contact.new(email: "lea@exemple.fr", nom: "Léa", ville: "Lyon")
puts c.nom
puts c.ville
p c.to_h
```

```sortie
Léa
Lyon
{:nom=>"Léa", :email=>"lea@exemple.fr", :ville=>"Lyon"}
```

### Ajouter des méthodes

Un bloc après `Struct.new` permet d'ajouter des méthodes, comme dans une classe.

```ruby
Rectangle = Struct.new(:largeur, :hauteur) do
  def aire = largeur * hauteur

  def to_s = "#{largeur} × #{hauteur}"
end

r = Rectangle.new(3, 5)
puts r
puts r.aire
```

```sortie
3 × 5
15
```

## Data : une valeur qui ne change pas

`Data.define` (depuis Ruby 3.2) ressemble à `Struct`, avec deux différences :

- l'objet est [immuable](../debuter/glossaire.md#immuable) : pas de `x=`, on ne peut
  pas le modifier après sa création ;
- tous les champs sont obligatoires.

C'est l'outil idéal pour une **valeur** : une couleur, une somme d'argent, une
coordonnée. Comme le nombre `5`, une valeur ne change pas : on en crée une nouvelle.

```ruby
Couleur = Data.define(:rouge, :vert, :bleu)

orange = Couleur.new(rouge: 255, vert: 165, bleu: 0)
p orange
puts orange.vert
puts orange == Couleur.new(255, 165, 0)

plus_rouge = orange.with(vert: 100)
p plus_rouge
p orange
```

```sortie
#<data Couleur rouge=255, vert=165, bleu=0>
165
true
#<data Couleur rouge=255, vert=100, bleu=0>
#<data Couleur rouge=255, vert=165, bleu=0>
```

`with` crée une **copie** avec quelques champs changés. L'original reste intact.

On peut aussi ajouter des méthodes et vérifier les valeurs dans `initialize` :

```ruby
Prix = Data.define(:centimes) do
  def initialize(centimes:)
    raise ArgumentError, "prix négatif" if centimes.negative?
    super
  end

  def to_s = format("%.2f €", centimes / 100.0)

  def +(autre) = Prix.new(centimes + autre.centimes)
end

total = Prix.new(1999) + Prix.new(501)
puts total

begin
  Prix.new(-5)
rescue ArgumentError => e
  puts "Refusé : #{e.message}"
end
```

```sortie
25.00 €
Refusé : prix négatif
```

> **À retenir :** `Struct` pour un petit objet que l'on modifie, `Data` pour une valeur
> que l'on ne modifie jamais. Dans le doute, commencez par `Data`.

## OpenStruct : à connaître, à éviter

Vous croiserez peut-être `OpenStruct` dans du code ancien. Il accepte n'importe quel
champ, même ajouté plus tard, comme un [dictionnaire](../debuter/glossaire.md#dictionnaire)
déguisé en objet.

```ruby
require "ostruct"

reglages = OpenStruct.new(theme: "sombre")
reglages.taille = 14
puts reglages.theme
puts reglages.taille
p reglages.couleur
```

```sortie
sombre
14
nil
```

Une faute de frappe (`reglages.thme`) donne `nil` au lieu d'une erreur, et `OpenStruct`
est lent. Préférez un `Hash`, un `Struct` ou un `Data`.

## Set : un ensemble sans doublons

Un `Set` est un ensemble, comme en mathématiques : chaque élément y est **au plus une
fois**, et l'ordre ne compte pas. Pensez à une collection de timbres : un deuxième
exemplaire du même timbre n'ajoute rien.

```ruby
require "set"

tags = Set.new(["ruby", "web"])
tags << "ruby"
tags.add("débutant")
p tags
puts tags.size
puts tags.include?("web")
p [3, 1, 3, 2, 1].to_set
```

```sortie
#<Set: {"ruby", "web", "débutant"}>
3
true
#<Set: {3, 1, 2}>
```

`"ruby"` ajouté une deuxième fois est ignoré. `include?` est très rapide sur un `Set`,
même avec des millions d'éléments, alors qu'un tableau doit tout parcourir.

> **Note :** depuis Ruby 3.2, `Set` est disponible sans `require`. L'écrire ne gêne pas
> et fonctionne avec les versions plus anciennes.

### Unir, croiser, retrancher

Les opérations sur les ensembles répondent à des questions de tous les jours.

```ruby
require "set"

alice = Set["cinéma", "piano", "vélo"]
bruno = Set["vélo", "échecs", "cinéma"]

p alice | bruno
p alice & bruno
p alice - bruno
p alice ^ bruno
```

```sortie
#<Set: {"cinéma", "piano", "vélo", "échecs"}>
#<Set: {"vélo", "cinéma"}>
#<Set: {"piano"}>
#<Set: {"échecs", "piano"}>
```

- `|` (union) : tous les loisirs d'Alice **ou** de Bruno ;
- `&` (intersection) : les loisirs **communs** ;
- `-` (différence) : ceux d'Alice que Bruno n'a pas ;
- `^` : ceux qui ne sont que chez l'un des deux.

Les noms longs existent aussi : `union`, `intersection`, `difference`.

### Inclusion

```ruby
require "set"

requis = Set["nom", "email"]
recus = Set["nom", "email", "age"]

puts requis.subset?(recus)
puts recus.superset?(requis)
puts requis <= recus
p requis - Set["nom"]
```

```sortie
true
true
true
#<Set: {"email"}>
```

`subset?` (ou `<=`) répond : « tous mes éléments sont-ils dans l'autre ? » Ici : tous les
champs requis ont-ils été reçus ?

## Lequel choisir ?

| Besoin | Outil | Exemple |
| --- | --- | --- |
| Quelques champs, modifiables | `Struct` | un joueur avec son score |
| Une valeur qui ne change pas | `Data` | une couleur, un prix, un point |
| Des clés inconnues à l'avance | `Hash` | les réglages lus dans un fichier |
| Des éléments uniques, tester l'appartenance | `Set` | des tags, des mots déjà vus |
| Des éléments dans l'ordre, avec doublons | `Array` | une liste de courses |
| Un objet avec beaucoup de comportement | une `class` | un compte en banque |

## Erreurs fréquentes

### Modifier un objet Data

```ruby norun
Point = Data.define(:x, :y)
pt = Point.new(x: 1, y: 2)
pt.x = 5
```

```text
main.rb:3:in `<main>': undefined method `x=' for an instance of Point (NoMethodError)

pt.x = 5
  ^^^^
```

Un `Data` est immuable : il n'a pas de méthode `x=`. Créez une copie modifiée avec
`pt.with(x: 5)`.

### Oublier un champ de Data

```ruby norun
Point = Data.define(:x, :y)
Point.new(x: 1)
```

```text
main.rb:2:in `initialize': missing keyword: :y (ArgumentError)

Point.new(x: 1)
          ^^^^
	from main.rb:2:in `new'
	from main.rb:2:in `<main>'
```

Tous les champs d'un `Data` sont obligatoires. Donnez une valeur à chacun.

### Lire un Set par sa position

```ruby norun
require "set"
couleurs = Set["rouge", "vert"]
puts couleurs[0]
```

```text
main.rb:3:in `<main>': undefined method `[]' for an instance of Set (NoMethodError)

puts couleurs[0]
             ^^^
```

Un `Set` n'a pas de positions. Utilisez `first`, parcourez-le avec `each`, ou
transformez-le avec `to_a` si l'ordre compte.

## Testez-vous

**Question 1 :** comment créer un `Struct` `Livre` avec `titre` et `pages`, puis un
livre de 300 pages ?

> **Réponse**
>
> ```ruby
> Livre = Struct.new(:titre, :pages)
> livre = Livre.new("Dune", 300)
> puts livre.pages
> ```
>
> ```sortie
> 300
> ```

**Question 2 :** combien d'éléments contient `Set[1, 2, 2, 3, 3, 3]` ?

> **Réponse**
>
> Trois : `1`, `2` et `3`. Les doublons sont ignorés.
>
> ```ruby
> require "set"
> puts Set[1, 2, 2, 3, 3, 3].size
> ```
>
> ```sortie
> 3
> ```

**Question 3 :** vous voulez représenter une date d'anniversaire (jour et mois) qui ne
doit jamais changer par erreur. `Struct` ou `Data` ?

> **Réponse**
>
> `Data` : il est immuable et oblige à donner le jour et le mois.

## En résumé

- `Struct.new(:a, :b)` crée une petite classe modifiable ; `keyword_init: true` pour
  des arguments nommés.
- `Data.define(:a, :b)` crée une valeur immuable ; `with` en fait une copie modifiée.
- `OpenStruct` accepte tout, mais cache les fautes de frappe : évitez-le.
- `Set` garde des éléments uniques ; `|`, `&`, `-` et `subset?` comparent des ensembles.
- Choisissez l'outil selon le besoin : modifiable, immuable, unique ou ordonné.

Suite : [comparer des objets](comparaison.md).
