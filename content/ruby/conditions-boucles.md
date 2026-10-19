# Conditions et boucles

Cette page montre comment prendre des décisions avec `if`, `unless` et `case`, et comment répéter des actions avec les boucles.

> **Objectifs :**
>
> - vous saurez exécuter du code seulement si une condition est vraie ;
> - vous saurez combiner des conditions avec `&&`, `||` et `!` ;
> - vous saurez choisir parmi plusieurs cas avec `case` ;
> - vous saurez répéter du code avec `while`, `loop`, `times`, `each` et `step` ;
> - vous saurez arrêter une boucle ou sauter un tour.

## if, elsif et else

Une [condition](../debuter/glossaire.md#condition) est une question à laquelle on répond
par vrai ou faux : « la note est-elle supérieure à 10 ? ». C'est un carrefour : selon
la réponse, le programme prend un chemin ou un autre.

`if` exécute du code seulement si une condition est vraie. Le bloc se termine par `end`.

```ruby
note = 14
if note >= 16
  puts "Très bien"
elsif note >= 10
  puts "Admis"
else
  puts "Refusé"
end
```

```sortie
Admis
```

- `if` (« si ») pose la première question.
- `elsif` (« sinon, si ») pose une autre question, seulement si les précédentes ont
  répondu non. Attention à l'orthographe : `elsif`, sans `e` au milieu.
- `else` (« sinon ») s'exécute si aucune question n'a répondu oui.
- Ruby s'arrête à la **première** branche vraie.

Ligne par ligne :

- `note = 14` range 14 dans la variable `note` ;
- `if note >= 16` demande : « 14 est-il supérieur ou égal à 16 ? ». Non : Ruby saute
  ce bloc ;
- `elsif note >= 10` demande : « 14 est-il supérieur ou égal à 10 ? ». Oui : Ruby
  affiche « Admis » ;
- une branche a répondu oui : Ruby saute le `else` et continue après le `end`.

Les comparaisons (`==`, `!=`, `<`, `>`, `<=`, `>=`) sont présentées dans
[les premiers pas](premiers-pas.md).

**À vous :** écrivez un programme qui range `heure = 15` dans une variable, puis
affiche « Bonjour » si l'heure est avant 12, « Bon après-midi » si elle est avant 18,
et « Bonsoir » sinon.

> **Solution**
>
> ```ruby
> heure = 15
> if heure < 12
>   puts "Bonjour"
> elsif heure < 18
>   puts "Bon après-midi"
> else
>   puts "Bonsoir"
> end
> ```
>
> ```sortie
> Bon après-midi
> ```

### La forme courte

Pour une seule instruction, vous pouvez mettre `if` à la fin de la ligne.

```ruby
temperature = 35
puts "Il fait chaud !" if temperature > 30
```

```sortie
Il fait chaud !
```

### if renvoie une valeur

En Ruby, `if` donne une valeur : celle de la branche choisie. Vous pouvez la ranger
dans une variable.

```ruby
note = 8
resultat = if note >= 10
             "admis"
           else
             "refusé"
           end
puts resultat
```

```sortie
refusé
```

## unless

Parfois, la question se pose plus naturellement à l'envers : « s'il n'est **pas**
connecté, demandez-lui de se connecter ». `unless` permet de l'écrire comme on le dit.

`unless` (« à moins que ») est le contraire de `if` : le code s'exécute si la condition
est **fausse**.

```ruby
connecte = false
puts "Veuillez vous connecter." unless connecte
```

```sortie
Veuillez vous connecter.
```

> **Astuce :** évitez `unless` avec `else` : c'est difficile à lire. Utilisez plutôt `if`.

## Combiner des conditions

Une décision dépend souvent de plusieurs choses à la fois. Pour entrer gratuitement au
musée, il faut être jeune **et** membre. Ces trois signes relient plusieurs tests en un
seul.

`&&` veut dire « et », `||` veut dire « ou », `!` veut dire « non ».

- `a && b` est vrai si `a` **et** `b` sont vrais.
- `a || b` est vrai si au moins l'un des deux est vrai.
- `!a` est vrai si `a` est faux.

```ruby
age = 20
membre = true
puts "Entrée gratuite" if age < 26 && membre
puts "Pas membre" if !membre
puts "Tarif réduit" if age < 18 || age > 65
puts "Plein tarif" unless age < 18 || age > 65
```

```sortie
Entrée gratuite
Plein tarif
```

Ligne par ligne :

- `age` vaut 20 et `membre` vaut `true` ;
- ligne 3 : `age < 26` est vrai **et** `membre` est vrai : « Entrée gratuite » s'affiche ;
- ligne 4 : `!membre` transforme `true` en `false` : rien ne s'affiche ;
- ligne 5 : 20 n'est ni plus petit que 18, ni plus grand que 65 : le `||` est faux ;
- ligne 6 : la même condition est fausse, et `unless` agit justement quand c'est faux.

`&&` passe avant `||`, comme `*` passe avant `+`. Ajoutez des parenthèses pour être
clair :

```ruby
age = 70
etudiant = false
membre = false
puts "Réduction" if (age > 65 || etudiant) && !membre
```

```sortie
Réduction
```

> **Note :** Ruby connaît aussi `and`, `or` et `not`. Ils n'ont pas la même priorité
> que `&&` et `||` : pour débuter, utilisez seulement `&&`, `||` et `!`.

**À vous :** écrivez un programme qui range `age = 70` dans une variable, puis
affiche « Tarif réduit » si l'âge est inférieur à 18 ou supérieur ou égal à 65, et
« Plein tarif » sinon.

> **Solution**
>
> ```ruby
> age = 70
> if age < 18 || age >= 65
>   puts "Tarif réduit"
> else
>   puts "Plein tarif"
> end
> ```
>
> ```sortie
> Tarif réduit
> ```

## Le ternaire

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

`condition ? valeur_si_vrai : valeur_si_faux` choisit une valeur en une ligne.

```ruby
age = 15
statut = age >= 18 ? "majeur" : "mineur"
puts statut
```

```sortie
mineur
```

Lisez-le comme une question : « âge ≥ 18 ? si oui majeur, sinon mineur ».

## case et when

Pensez à un distributeur de boissons : selon le bouton choisi, il sert une boisson
différente. Quand une même valeur peut prendre beaucoup de formes (un jour, un choix
dans un menu), vous voulez une réaction pour chacune.

`case` compare une valeur à plusieurs possibilités. C'est plus lisible qu'une longue
suite de `elsif`.

```ruby
jour = "samedi"
case jour
when "samedi", "dimanche"
  puts "Week-end !"
when "lundi"
  puts "Courage..."
else
  puts "Jour de semaine"
end
```

```sortie
Week-end !
```

- Une virgule dans `when` veut dire « ou ».
- `else` sert si aucun `when` ne correspond.

Ligne par ligne :

- `jour` vaut `"samedi"` ;
- `case jour` annonce la valeur à comparer ;
- premier `when` : `jour` vaut-il `"samedi"` ou `"dimanche"` ? Oui : Ruby affiche
  « Week-end ! » ;
- les autres `when` et le `else` sont sautés, comme avec `elsif`.

`when` accepte aussi des intervalles :

```ruby
note = 12
mention = case note
          when 0...10 then "insuffisant"
          when 10...14 then "passable"
          else "bien"
          end
puts mention
```

```sortie
passable
```

`then` permet d'écrire la réponse sur la même ligne que `when`.

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

### Oublier end

```ruby norun
note = 14
if note >= 10
  puts "Admis"
```

```text
main.rb: --> main.rb
Unmatched keyword, missing `end' ?
> 2  if note >= 10
main.rb:3: syntax error, unexpected end-of-input, expecting `end' or dummy end (SyntaxError)
  puts "Admis"
              ^
```

Chaque `if`, `case`, `while`, `until` et `do` doit être fermé par un `end`. Ajoutez
`end` après `puts "Admis"`. Indentez votre code : un `end` manquant se voit tout de
suite.

### Écrire = au lieu de ==

```ruby norun
note = 14
if note = 10
  puts "dix"
end
```

```text
main.rb:2: warning: found `= literal' in conditional, should be ==
dix
```

`note = 10` **range** 10 dans `note` : la condition est toujours vraie. Ruby prévient
(*warning* : « avertissement »), mais le programme continue. Écrivez `if note == 10`.

### Écrire elseif ou else if

```ruby norun
note = 12
if note >= 10
  puts "Admis"
elseif note >= 5
  puts "Rattrapage"
end
```

```text
Admis
main.rb:4:in `<main>': undefined method `elseif' for main (NoMethodError)

elseif note >= 5
^^^^^^
Did you mean?  elsif
```

Ruby lit `elseif` comme une méthode inconnue. Le mot-clé s'écrit `elsif`.

## Testez-vous

**Question 1 :** qu'affiche ce code ?

```ruby norun
x = 5
puts "A" if x > 3 && x < 5
puts "B" if x > 3 || x < 5
```

> **Réponse**
>
> Seulement `B`. `x < 5` est faux (5 n'est pas plus petit que 5), donc le `&&` est faux.
> Avec `||`, il suffit que `x > 3` soit vrai.

**Question 2 :** combien de nombres contient `(1...10).to_a` ?

> **Réponse**
>
> 9 nombres, de 1 à 9 : avec trois points, la dernière valeur est exclue.

**Question 3 :** comment afficher 10, 8, 6, 4, 2, 0 ?

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

## En résumé

- `if`, `elsif` et `else` choisissent quoi faire ; `unless` est le contraire de `if`.
- `&&` (et), `||` (ou), `!` (non) combinent des conditions.
- `case` et `when` comparent une valeur à plusieurs possibilités.
- `1..5` inclut 5, `1...5` l'exclut.
- `while`, `until` et `loop` répètent ; `times`, `each`, `upto` et `step` sont plus courants.
- `break` arrête la boucle, `next` passe au tour suivant.

Suite : [les méthodes](fonctions.md). Comparez avec
[les conditions et boucles en Rythme](../rythme/conditions-boucles.md).
