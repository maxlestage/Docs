# Les boucles

Cette page montre comment répéter des actions avec les boucles, et comment décrire une suite de valeurs avec un intervalle.

> **Objectifs :**
>
> - vous saurez décrire une suite de valeurs avec un intervalle ;
> - vous saurez répéter du code avec `while`, `loop`, `times`, `each` et `step` ;
> - vous saurez arrêter une boucle ou sauter un tour.

Ce chapitre suit [les conditions](conditions-boucles.md) : une boucle répète du code
tant qu'une condition le demande.

## Les intervalles

Les notes de 10 à 14, les jours de 1 à 31, les lettres de a à e : on parle souvent de
valeurs qui se suivent. Un intervalle les désigne d'un coup, sans les écrire une par une.

Un intervalle (`Range`) va d'une valeur à une autre. `1..5` inclut 5, `1...5` l'exclut.

```ruby
p (1..5).to_a
p (1...5).to_a
puts (1..10).include?(7)
p ("a".."e").to_a
```

```sortie
[1, 2, 3, 4, 5]
[1, 2, 3, 4]
true
["a", "b", "c", "d", "e"]
```

> **Astuce :** pour retenir : le troisième point « pousse » la dernière valeur dehors.

## La boucle while

Imaginez qu'on vous demande d'afficher les nombres de 1 à 100. Écrire 100 `puts` serait
long et pénible. Une boucle fait ce travail répétitif à votre place.

Une [boucle](../debuter/glossaire.md#boucle) répète du code. Chaque passage s'appelle un
**tour** (ou une [itération](../debuter/glossaire.md#iteration)).

`while` (« tant que ») répète tant que la condition est vraie.

```ruby
compteur = 1
while compteur <= 3
  puts "Tour #{compteur}"
  compteur += 1
end
```

```sortie
Tour 1
Tour 2
Tour 3
```

Ligne par ligne :

- `compteur = 1` : la variable vaut 1 au départ ;
- `while compteur <= 3` : avant chaque tour, Ruby pose la question. `1 <= 3` est vrai :
  il entre dans la boucle ;
- le bloc affiche « Tour 1 », puis `compteur += 1` ajoute 1 : `compteur` vaut 2 ;
- Ruby remonte au `while` et repose la question, avec 2, puis avec 3 ;
- quand `compteur` vaut 4, `4 <= 3` est faux : Ruby sort de la boucle, après le `end`.

> **Attention :** si la condition reste toujours vraie, la boucle ne s'arrête jamais
> (une **boucle infinie**). Ici, oublier `compteur += 1` ferait tourner le programme
> sans fin. Arrêtez-le avec Ctrl+C.

**À vous :** écrivez un programme qui affiche les nombres de 1 à 5 avec une boucle
`while`, puis « Fini ! ».

> **Solution**
>
> ```ruby
> n = 1
> while n <= 5
>   puts n
>   n += 1
> end
> puts "Fini !"
> ```
>
> ```sortie
> 1
> 2
> 3
> 4
> 5
> Fini !
> ```

## La boucle until

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

`until` répète **jusqu'à** ce que la condition devienne vraie.

```ruby
n = 3
until n.zero?
  puts n
  n -= 1
end
puts "Partez !"
```

```sortie
3
2
1
Partez !
```

## La boucle loop

`loop` répète sans fin. Vous sortez avec `break`. C'est pratique quand la condition
d'arrêt se trouve au milieu du tour.

```ruby
total = 0
loop do
  total += 7
  break if total > 20
  puts "Total : #{total}"
end
puts "Fini : #{total}"
```

```sortie
Total : 7
Total : 14
Fini : 21
```

Ligne par ligne :

- `total` vaut 0 au départ ;
- premier tour : `total += 7` donne 7. `7 > 20` est faux : pas de `break`, et le `puts`
  affiche 7 ;
- deuxième tour : `total` vaut 14, même chemin ;
- troisième tour : `total` vaut 21. `21 > 20` est vrai : `break` sort de la boucle
  avant le `puts` ;
- Ruby continue après le `end` et affiche « Fini : 21 ».

## times et each

En Ruby, on préfère souvent `times` et `each` aux boucles classiques. Vous n'avez pas
de compteur à gérer : impossible d'oublier `+= 1`.

```ruby
3.times { |i| puts "i vaut #{i}" }
(1..3).each { |n| puts n * 10 }
```

```sortie
i vaut 0
i vaut 1
i vaut 2
10
20
30
```

Ligne par ligne :

- `3.times` exécute le bloc trois fois : `i` vaut 0, puis 1, puis 2 ;
- `(1..3).each` prend chaque nombre de l'intervalle, un par un, et le range dans `n` ;
- le bloc affiche `n * 10` : 10, puis 20, puis 30.

Le code entre `{ }` est un **bloc**. Entre les barres `|i|`, vous nommez la valeur que
le bloc reçoit à chaque tour.

> **À retenir :** `times` compte à partir de `0`. `each` parcourt chaque élément d'une
> collection. Pour en savoir plus, lisez [les blocs](blocs.md).

**À vous :** écrivez un programme qui affiche la table de 7, de « 7 x 1 = 7 » à
« 7 x 5 = 35 », une ligne par calcul, avec `each` sur un intervalle.

> **Solution**
>
> ```ruby
> (1..5).each { |i| puts "7 x #{i} = #{7 * i}" }
> ```
>
> ```sortie
> 7 x 1 = 7
> 7 x 2 = 14
> 7 x 3 = 21
> 7 x 4 = 28
> 7 x 5 = 35
> ```

### Compter avec upto, downto et step

- `upto` compte vers le haut, `downto` vers le bas ;
- `step` avance par pas : de 2 en 2, de 5 en 5…

```ruby
1.upto(3) { |n| puts "upto : #{n}" }
3.downto(2) { |n| puts "downto : #{n}" }
0.step(10, 5) { |n| puts "step : #{n}" }
```

```sortie
upto : 1
upto : 2
upto : 3
downto : 3
downto : 2
step : 0
step : 5
step : 10
```

`0.step(10, 5)` veut dire : « de 0 à 10, de 5 en 5 ».

### Avoir la position avec each_with_index

`each_with_index` donne l'élément **et** sa position (son
[indice](../debuter/glossaire.md#indice)), qui commence à `0`.

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

`do ... end` remplace les accolades quand le bloc fait plusieurs lignes.

## break et next

Parfois, vous voulez changer le cours d'une boucle en route : arrêter de chercher dès
que vous avez trouvé, ou ignorer certains éléments. C'est le rôle de ces deux mots.

`break` arrête la boucle. `next` passe directement au tour suivant.

```ruby
(1..10).each do |n|
  next if n.even?
  break if n > 7
  puts n
end
```

```sortie
1
3
5
7
```

- Pour `2`, `4`, `6` : `next` saute le reste du tour.
- Pour `9` : `break` arrête tout.

Ligne par ligne :

- `(1..10).each` donne à `n` les valeurs 1, 2, 3… jusqu'à 10 ;
- `n.even?` demande « `n` est-il pair ? ». Si oui, `next` passe au tour suivant : le
  `puts` n'est pas atteint ;
- `break if n > 7` : dès que `n` dépasse 7 (ici, à 9), la boucle s'arrête : 9 et 10 ne
  sont jamais affichés ;
- sinon, `puts n` affiche le nombre.

## Erreurs fréquentes

### Appeler each sur un nombre

```ruby norun
5.each { |n| puts n }
```

```text
main.rb:1:in `<main>': undefined method `each' for an instance of Integer (NoMethodError)

5.each { |n| puts n }
 ^^^^^
```

Un nombre n'est pas une collection : il n'a pas de méthode `each`. Pour répéter 5 fois,
écrivez `5.times`. Pour parcourir les nombres de 1 à 5, écrivez `(1..5).each`.

### Utiliser la variable du bloc après le bloc

```ruby norun
3.times { |i| puts i }
puts "Dernier : #{i}"
```

```text
0
1
2
main.rb:2:in `<main>': undefined local variable or method `i' for main (NameError)

puts "Dernier : #{i}"
                  ^
```

`i` n'existe qu'à l'intérieur du bloc, entre `{` et `}`. Pour garder une valeur après
la boucle, créez la variable avant le bloc (`dernier = nil`), puis changez-la dans le
bloc (`dernier = i`).

## Testez-vous

**Question 1 :** combien de nombres contient `(1...10).to_a` ?

> **Réponse**
>
> 9 nombres, de 1 à 9 : avec trois points, la dernière valeur est exclue.

**Question 2 :** comment afficher 10, 8, 6, 4, 2, 0 ?

> **Réponse**
>
> Avec `step` et un pas négatif :
>
> ```ruby
> 10.step(0, -2) { |n| puts n }
> ```
>
> ```sortie
> 10
> 8
> 6
> 4
> 2
> 0
> ```

**Question 3 :** quelle est la différence entre `break` et `next` ?

> **Réponse**
>
> `break` arrête toute la boucle : Ruby continue après le `end`. `next` saute seulement
> la fin du tour en cours et passe au tour suivant.

## En résumé

- `1..5` inclut 5, `1...5` l'exclut.
- `while`, `until` et `loop` répètent ; `times`, `each`, `upto` et `step` sont plus courants.
- `each_with_index` donne chaque élément et sa position, à partir de `0`.
- `break` arrête la boucle, `next` passe au tour suivant.

Suite : [les méthodes](fonctions.md). Comparez avec
[les conditions et boucles en Rythme](../rythme/conditions-boucles.md).
