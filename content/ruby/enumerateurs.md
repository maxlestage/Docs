# Énumérateurs et paresse

Cette page montre ce qu'est un énumérateur, comment parcourir une collection à votre rythme, par paquets ou par paires, et comment travailler avec des suites infinies grâce à `lazy`.

> **Objectifs :**
>
> - vous saurez ce que renvoie `each` quand on ne lui donne pas de bloc ;
> - vous saurez avancer pas à pas avec `next` ;
> - vous saurez découper une collection avec `each_slice`, `each_cons` et `chunk_while` ;
> - vous saurez traiter une suite infinie avec `lazy` ;
> - vous saurez créer votre propre énumérateur avec `Enumerator.new`.

## each sans bloc

Que se passe-t-il si vous appelez `each` ou `map` **sans** bloc ? Ruby ne plante pas :
il renvoie un **énumérateur** (`Enumerator`).

```ruby
e = [10, 20, 30].each
p e
puts e.class
```

```sortie
#<Enumerator: [10, 20, 30]:each>
Enumerator
```

Un énumérateur est un parcours **en attente**. Pensez à un marque-page dans un livre :
il sait où vous en êtes et peut vous donner la page suivante quand vous la demandez.
C'est un [itérateur](../debuter/glossaire.md#iterateur) que l'on peut ranger dans une
variable.

Il connaît toutes les méthodes de [Enumerable](blocs.md#les-methodes-de-enumerable) :

```ruby
e = [10, 20, 30].each
p e.map { |x| x + 1 }
p e.to_a
```

```sortie
[11, 21, 31]
[10, 20, 30]
```

## Avancer pas à pas avec next

`next` donne l'élément suivant, un à la fois. Quand il n'y en a plus, il lève
`StopIteration`. `loop` attrape cette erreur et s'arrête tout seul.

```ruby
feu = ["vert", "orange", "rouge"].each
puts feu.next
puts feu.next
puts feu.next

feu.rewind
loop do
  puts "Couleur : #{feu.next}"
end
```

```sortie
vert
orange
rouge
Couleur : vert
Couleur : orange
Couleur : rouge
```

`rewind` remet le marque-page au début. `peek` regarde l'élément suivant sans avancer.

## Ajouter la position avec with_index

Comme un énumérateur est un objet, on peut lui ajouter des méthodes en chaîne.
`with_index` ajoute la position à n'importe quel parcours, et peut commencer à 1.

```ruby
fruits = ["pomme", "kiwi", "poire"]

fruits.each.with_index(1) do |fruit, i|
  puts "#{i}. #{fruit}"
end

p fruits.map.with_index { |fruit, i| "#{fruit}#{i}" }
```

```sortie
1. pomme
2. kiwi
3. poire
["pomme0", "kiwi1", "poire2"]
```

`map.with_index` n'est pas une méthode à part : c'est `map` sans bloc (un énumérateur) à qui
on demande `with_index`.

## Par paquets avec each_slice

`each_slice(n)` coupe la collection en paquets de `n` éléments. Le dernier paquet peut
être plus petit.

```ruby
eleves = ["Ana", "Ben", "Cléo", "Dan", "Eva"]
eleves.each_slice(2) { |groupe| p groupe }
```

```sortie
["Ana", "Ben"]
["Cléo", "Dan"]
["Eva"]
```

## Par fenêtres avec each_cons

`each_cons(n)` fait glisser une fenêtre de `n` éléments, un cran à la fois. Pratique pour
comparer chaque valeur à la suivante.

```ruby
temperatures = [12, 15, 14, 18]
temperatures.each_cons(2) do |avant, apres|
  puts "#{avant} -> #{apres} : #{apres - avant}"
end
```

```sortie
12 -> 15 : 3
15 -> 14 : -1
14 -> 18 : 4
```

## Regrouper ce qui se suit avec chunk_while

`chunk_while` regroupe les éléments voisins tant que le bloc répond `true` pour chaque
paire. Ici, on regroupe les nombres qui se suivent.

```ruby
jours = [1, 2, 3, 7, 8, 12]
groupes = jours.chunk_while { |a, b| b == a + 1 }
p groupes.to_a
p groupes.map { |g| g.size > 1 ? "#{g.first}-#{g.last}" : g.first.to_s }
```

```sortie
[[1, 2, 3], [7, 8], [12]]
["1-3", "7-8", "12"]
```

Son contraire, `slice_when`, coupe quand le bloc répond `true`.

## Répéter avec cycle et take

`cycle` répète une collection sans fin. `take(n)` prend les `n` premiers éléments, ce qui
permet de s'arrêter.

```ruby
equipes = ["rouge", "bleu"]
joueurs = ["Ana", "Ben", "Cléo", "Dan", "Eva"]

joueurs.zip(equipes.cycle.take(joueurs.size)).each do |joueur, equipe|
  puts "#{joueur} : #{equipe}"
end
```

```sortie
Ana : rouge
Ben : bleu
Cléo : rouge
Dan : bleu
Eva : rouge
```

## La paresse avec lazy

Normalement, chaque étape d'une chaîne traite **toute** la collection avant de passer à
la suivante. Avec une suite infinie, c'est impossible : `map` ne finirait jamais.

`lazy` rend la chaîne **paresseuse** : chaque élément traverse toutes les étapes l'un
après l'autre, et seulement quand on en a besoin. C'est comme une cuisine qui prépare
un plat à la commande, au lieu de tout cuisiner d'avance.

```ruby
carres_pairs = (1..)
  .lazy
  .map { |n| n * n }
  .select(&:even?)
  .first(5)
p carres_pairs
```

```sortie
[4, 16, 36, 64, 100]
```

`(1..)` est infini, mais seuls les 10 premiers nombres sont calculés. `first(5)` (ou
`take(5).to_a`) déclenche le travail et dit quand s'arrêter.

```ruby
etapes = (1..3).lazy.map { |n| puts "calcul de #{n}"; n * 10 }
puts "Rien n'est encore calculé"
p etapes.first(2)
```

```sortie
Rien n'est encore calculé
calcul de 1
calcul de 2
[10, 20]
```

> **À retenir :** une chaîne `lazy` ne fait rien tant qu'on ne demande pas de résultat
> avec `first`, `to_a`, `each` ou `force`.

## Créer son énumérateur avec Enumerator.new

`Enumerator.new` fabrique une suite à la demande. Dans le bloc, `y << valeur` (le
« yielder ») **donne** une valeur, puis attend qu'on demande la suivante.

```ruby
fibonacci = Enumerator.new do |y|
  a, b = 0, 1
  loop do
    y << a
    a, b = b, a + b
  end
end

p fibonacci.take(10)
p fibonacci.lazy.select(&:odd?).first(5)
puts fibonacci.next
puts fibonacci.next
```

```sortie
[0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
[1, 1, 3, 5, 13]
0
1
```

La boucle `loop` est infinie, mais ce n'est pas grave : le bloc s'arrête à chaque `y <<`
et ne reprend que si on demande une autre valeur.

## Erreurs fréquentes

### Appeler next après la fin

```ruby norun
e = [1, 2].each
e.next
e.next
e.next
```

```text
main.rb:4:in `next': iteration reached an end (StopIteration)
	from main.rb:4:in `<main>'
```

Il n'y a que deux éléments. Utilisez `loop do ... end`, qui s'arrête proprement à la
fin, ou `rewind` pour revenir au début.

### Oublier lazy sur une suite infinie

```ruby norun
p (1..).map { |n| n * 2 }.first(3)
```

```text
(le programme ne s'arrête jamais)
```

`map` veut traiter **tous** les nombres avant de passer à `first`. Ajoutez `lazy` :
`(1..).lazy.map { |n| n * 2 }.first(3)`.

### Afficher une chaîne lazy au lieu de son résultat

```ruby norun
p (1..).lazy.map { |n| n * 2 }.select(&:even?)
```

```text
#<Enumerator::Lazy: #<Enumerator::Lazy: #<Enumerator::Lazy: 1..>:map>:select>
```

Pas d'erreur, mais pas de nombres : la chaîne attend qu'on lui demande quelque chose.
Terminez par `first(n)`.

## Testez-vous

**Question 1 :** que renvoie `[1, 2, 3].map` sans bloc ?

> **Réponse**
>
> Un énumérateur, en attente d'un bloc ou d'une autre méthode.
>
> ```ruby
> p [1, 2, 3].map
> ```
>
> ```sortie
> #<Enumerator: [1, 2, 3]:map>
> ```

**Question 2 :** comment afficher `[1, 2, 3, 4, 5, 6]` par lignes de 3 ?

> **Réponse**
>
> ```ruby
> [1, 2, 3, 4, 5, 6].each_slice(3) { |ligne| puts ligne.join(" ") }
> ```
>
> ```sortie
> 1 2 3
> 4 5 6
> ```

**Question 3 :** comment obtenir les 4 premiers multiples de 7 supérieurs à 100 ?

> **Réponse**
>
> ```ruby
> p (101..).lazy.select { |n| n % 7 == 0 }.first(4)
> ```
>
> ```sortie
> [105, 112, 119, 126]
> ```

## En résumé

- `each`, `map` et leurs cousins renvoient un `Enumerator` quand on ne donne pas de bloc.
- `next` avance pas à pas ; `with_index(1)` ajoute une position.
- `each_slice` coupe en paquets, `each_cons` fait glisser une fenêtre, `chunk_while`
  regroupe ce qui se suit.
- `lazy` traite les éléments un par un, à la demande : idéal pour les suites infinies.
- `Enumerator.new { |y| y << ... }` crée une suite sur mesure.

Suite : [Classes et objets](types.md)
