# Tableaux et hashs

Cette page présente les deux collections principales de Ruby : les tableaux (`Array`) et les hashs (`Hash`).

> **Objectifs :**
>
> - vous saurez créer un tableau, lire, ajouter et retirer ses éléments ;
> - vous saurez trier, compter et chercher dans un tableau ;
> - vous saurez créer un hash et retrouver une valeur par sa clé ;
> - vous saurez gérer une clé absente avec `fetch`, une valeur par défaut ou `dig` ;
> - vous saurez parcourir un tableau et un hash avec `each`.

Une **collection** est une valeur qui en contient plusieurs. Ruby en propose deux
principales :

- le **tableau**, une [liste](../debuter/glossaire.md#liste) rangée dans l'ordre, comme
  une file de casiers numérotés ;
- le **hash**, un [dictionnaire](../debuter/glossaire.md#dictionnaire) qui associe une
  étiquette à chaque valeur, comme un répertoire « nom → numéro ».

## Les tableaux

Un tableau est une liste ordonnée de valeurs, entre crochets `[ ]`, séparées par des
virgules. Chaque valeur s'appelle un **élément**.

```ruby
fruits = ["pomme", "banane", "cerise"]
p fruits
puts fruits.length
```

```sortie
["pomme", "banane", "cerise"]
3
```

Un tableau peut contenir n'importe quelles valeurs, même mélangées, et même d'autres
tableaux. `[]` est un tableau vide.

### Lire un élément

Les positions ([indices](../debuter/glossaire.md#indice)) commencent à `0`. Un indice
négatif part de la fin : `-1` est le dernier élément.

```ruby
fruits = ["pomme", "banane", "cerise"]
puts fruits[0]
puts fruits[-1]
puts fruits.first
p fruits[10]
```

```sortie
pomme
cerise
pomme
nil
```

> **Note :** lire un indice qui n'existe pas ne plante pas : vous obtenez `nil`.

### Lire plusieurs éléments

Un intervalle, ou `first(n)` et `last(n)`, donnent un morceau du tableau.

```ruby
lettres = ["a", "b", "c", "d", "e"]
p lettres[1..3]
p lettres.first(2)
p lettres.last(2)
```

```sortie
["b", "c", "d"]
["a", "b"]
["d", "e"]
```

### Ajouter et retirer

- `<<` et `push` ajoutent à la fin ; `unshift` ajoute au début ;
- `pop` retire le dernier élément et le renvoie ; `shift` retire le premier ;
- `delete` retire toutes les cases qui ont cette valeur.

```ruby
nombres = [1, 2]
nombres << 3
nombres.push(4)
p nombres
dernier = nombres.pop
p dernier
p nombres
nombres.delete(1)
p nombres
```

```sortie
[1, 2, 3, 4]
4
[1, 2, 3]
[2, 3]
```

```ruby
file = ["Bob", "Chloé"]
file.unshift("Alice")
p file
premier = file.shift
puts "Au tour de #{premier}"
p file
file[0] = "Zoé"
p file
```

```sortie
["Alice", "Bob", "Chloé"]
Au tour de Alice
["Bob", "Chloé"]
["Zoé", "Chloé"]
```

`file[0] = "Zoé"` remplace l'élément à l'indice 0.

### Méthodes utiles

Ces méthodes renvoient un résultat **sans modifier** le tableau.

```ruby
notes = [12, 8, 15, 8]
p notes.sort
p notes.uniq
puts notes.include?(15)
puts notes.sum
puts notes.max
p notes.reverse
puts notes.join(" - ")
```

```sortie
[8, 8, 12, 15]
[12, 8, 15]
true
43
15
[8, 15, 8, 12]
12 - 8 - 15 - 8
```

D'autres questions utiles :

```ruby
notes = [12, 8, 15, 8]
puts notes.count(8)
p notes.index(15)
puts notes.empty?
puts [].empty?
p notes.sort.reverse
puts notes.sum.fdiv(notes.length)
```

```sortie
2
2
false
true
[15, 12, 8, 8]
10.75
```

- `count(8)` compte les 8 ; `index(15)` donne la position du premier 15.
- `sort.reverse` trie du plus grand au plus petit.
- `sum.fdiv(length)` calcule la moyenne, avec une virgule.

## Parcourir un tableau

`each` exécute un bloc pour chaque élément.

```ruby
["pomme", "banane"].each do |fruit|
  puts "J'aime la #{fruit}"
end
```

```sortie
J'aime la pomme
J'aime la banane
```

Avec `each_with_index`, vous obtenez aussi la position :

```ruby
["or", "argent", "bronze"].each_with_index do |medaille, i|
  puts "#{i + 1}. #{medaille}"
end
```

```sortie
1. or
2. argent
3. bronze
```

## Les hashs

Un hash associe des **clés** à des **valeurs**. Pensez à un vestiaire : vous donnez
le ticket (la clé), on vous rend le manteau (la valeur). Chaque clé est unique. Les
clés sont souvent des [symboles](premiers-pas.md#les-symboles).

```ruby
personne = { nom: "Alice", age: 30 }
puts personne[:nom]
puts personne[:age]
p personne[:ville]
```

```sortie
Alice
30
nil
```

> **À retenir :** `{ nom: "Alice" }` est un raccourci pour `{ :nom => "Alice" }`. Les
> deux écritures donnent le même hash.

### Modifier un hash

- `hash[cle] = valeur` ajoute une clé, ou remplace sa valeur si elle existe déjà ;
- `delete` retire une clé ;
- `key?` demande si une clé existe.

```ruby
stock = { "pommes" => 5 }
stock["poires"] = 3
stock["pommes"] += 1
stock.delete("poires")
p stock
puts stock.key?("pommes")
```

```sortie
{"pommes"=>6}
true
```

### Valeur par défaut avec fetch

`fetch` permet de donner une valeur de secours si la clé manque.

```ruby
config = { langue: "fr" }
puts config.fetch(:langue, "en")
puts config.fetch(:theme, "clair")
```

```sortie
fr
clair
```

### Valeur par défaut avec Hash.new

`Hash.new(0)` crée un hash dont **toutes** les clés absentes valent `0`. C'est idéal
pour compter.

```ruby
votes = Hash.new(0)
["pomme", "kiwi", "pomme"].each { |fruit| votes[fruit] += 1 }
p votes
puts votes["fraise"]
```

```sortie
{"pomme"=>2, "kiwi"=>1}
0
```

Sans le `0`, `votes[fruit] += 1` planterait : on ne peut pas ajouter 1 à `nil`.

> **Attention :** n'écrivez pas `Hash.new([])` : toutes les clés partageraient le même
> tableau. Pour un tableau par clé, utilisez `Hash.new { |h, k| h[k] = [] }`.

### Lire des données imbriquées avec dig

Un hash peut contenir d'autres hashs et des tableaux. `dig` descend niveau par niveau
et renvoie `nil` si un niveau manque, au lieu de planter.

```ruby
eleve = { nom: "Alice", adresse: { ville: "Lyon" }, notes: [12, 15] }
puts eleve[:adresse][:ville]
puts eleve.dig(:adresse, :ville)
puts eleve.dig(:notes, 1)
p eleve.dig(:parents, :mere)
```

```sortie
Lyon
Lyon
15
nil
```

`eleve[:parents][:mere]` planterait, car `eleve[:parents]` vaut `nil`.

## Parcourir un hash

`each` donne la clé et la valeur à chaque tour.

```ruby
prix = { cafe: 2, the: 3 }
prix.each do |boisson, montant|
  puts "#{boisson} : #{montant} €"
end
p prix.keys
p prix.values
```

```sortie
cafe : 2 €
the : 3 €
[:cafe, :the]
[2, 3]
```

> **Astuce :** pour transformer ou filtrer une collection, utilisez `map` et `select`.
> Ils sont expliqués dans [la page sur les blocs](blocs.md#les-methodes-de-enumerable).

## Erreurs fréquentes

### Lire au-delà de la fin du tableau

```ruby norun
fruits = ["pomme", "banane", "cerise"]
puts fruits[3].upcase
```

```text
main.rb:2:in `<main>': undefined method `upcase' for nil (NoMethodError)

puts fruits[3].upcase
              ^^^^^^^
```

Trois éléments ont les indices 0, 1 et 2. `fruits[3]` vaut `nil`. Le dernier est
`fruits[2]`, ou mieux `fruits[-1]` ou `fruits.last`.

### Confondre une clé texte et une clé symbole

```ruby norun
stock = { "pommes" => 5 }
puts stock[:pommes] + 1
```

```text
main.rb:2:in `<main>': undefined method `+' for nil (NoMethodError)

puts stock[:pommes] + 1
                    ^
```

`"pommes"` (un texte) et `:pommes` (un symbole) sont deux clés différentes. Utilisez la
même que dans le hash : `stock["pommes"] + 1`.

### fetch sans valeur de secours

```ruby norun
config = { langue: "fr" }
puts config.fetch(:theme)
```

```text
main.rb:2:in `fetch': key not found: :theme (KeyError)
	from main.rb:2:in `<main>'
```

Sans valeur de secours, `fetch` plante si la clé manque : c'est voulu, pour repérer
une clé oubliée. Donnez une valeur par défaut : `config.fetch(:theme, "clair")`.

## Testez-vous

**Question 1 :** que vaut `[5, 3, 9][1]` ?

> **Réponse**
>
> `3`. Les indices commencent à 0 : l'indice 1 est le deuxième élément.

**Question 2 :** comment compter les lettres de `"abracadabra"` dans un hash ?

> **Réponse**
>
> ```ruby
> compte = Hash.new(0)
> "abracadabra".chars.each { |c| compte[c] += 1 }
> p compte
> ```
>
> ```sortie
> {"a"=>5, "b"=>2, "r"=>2, "c"=>1, "d"=>1}
> ```

**Question 3 :** quelle est la différence entre `h[:x]` et `h.fetch(:x)` quand la clé
`:x` est absente ?

> **Réponse**
>
> `h[:x]` renvoie `nil` (ou la valeur par défaut du hash). `h.fetch(:x)` lève une
> erreur `KeyError`.

## En résumé

- Un tableau est une liste ordonnée : `[1, 2, 3]`. Les indices commencent à `0`.
- `<<` ajoute à la fin, `pop` retire le dernier, `delete` retire une valeur.
- Un hash associe des clés à des valeurs : `{ nom: "Alice" }`.
- Une clé absente donne `nil` ; `fetch`, `Hash.new(0)` et `dig` gèrent les absences.
- `each` parcourt un tableau ou un hash.

Suite : [Les intervalles](intervalles.md)
