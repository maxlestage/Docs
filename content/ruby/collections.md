# Tableaux et hashs

Cette page présente les deux collections principales de Ruby : les tableaux (`Array`) et les hashs (`Hash`).

## Les tableaux

Un tableau est une liste ordonnée de valeurs, entre crochets.

```ruby
fruits = ["pomme", "banane", "cerise"]
p fruits
puts fruits.length
```

```sortie
["pomme", "banane", "cerise"]
3
```

### Lire un élément

Les positions (indices) commencent à `0`. Un indice négatif part de la fin.

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

### Ajouter et retirer

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

### Méthodes utiles

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

Un hash associe des **clés** à des **valeurs**. Les clés sont souvent des symboles.

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

## En résumé

- Un tableau est une liste ordonnée : `[1, 2, 3]`. Les indices commencent à `0`.
- `<<` ajoute à la fin, `pop` retire le dernier, `delete` retire une valeur.
- Un hash associe des clés à des valeurs : `{ nom: "Alice" }`.
- Une clé absente donne `nil` ; `fetch` permet une valeur de secours.
- `each` parcourt un tableau ou un hash.

Suite : lisez [les blocs et itérateurs](blocs.md). Comparez avec
[les collections en Rust](../rust/collections.md).
