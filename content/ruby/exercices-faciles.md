# Exercices pour bien démarrer

Cette page propose 15 exercices très guidés, tous faciles, pour pratiquer les bases de Ruby : variables, calculs, conditions, boucles, méthodes, textes, tableaux, hashs et intervalles.

## Comment faire ces exercices

Prenez-les dans l'ordre. Pour chaque exercice :

1. **Lisez** l'énoncé et la sortie attendue.
2. **Essayez sur papier** : quelles étapes ? quelles variables ?
3. **Écrivez** votre programme dans un fichier `exercice.rb`.
4. **Lancez-le** avec `ruby exercice.rb` (voir [l'introduction](introduction.md#lancer-un-programme)).
5. **Comparez** avec la sortie attendue, caractère par caractère.
6. Bloqué ? Lisez le **premier indice**, puis le deuxième. Ouvrez la **solution** en
   dernier.

> **Astuce :** une erreur n'est pas un échec. Lisez le message : il donne la ligne et
> la cause. Voir [lire un message d'erreur](introduction.md#lire-un-message-d-erreur).

## Afficher et calculer

### Exercice 1 : Dire bonjour

Niveau : ★☆☆

Rangez votre prénom dans une variable `prenom`. Affichez ensuite une phrase de
bienvenue qui contient ce prénom.

Sortie attendue (avec `"Léa"`) :

```text
Bonjour Léa, bienvenue !
```

> **Indice :** créez la variable avec `prenom = "Léa"`.

> **Indice :** dans un texte entre guillemets doubles, `#{prenom}` est remplacé par la
> valeur de la variable.

> **Solution**
>
> ```ruby
> prenom = "Léa"
> puts "Bonjour #{prenom}, bienvenue !"
> ```
>
> ```sortie
> Bonjour Léa, bienvenue !
> ```
>
> Pourquoi ça marche : `#{...}` s'appelle l'interpolation. Ruby calcule ce qu'il y a
> entre les accolades et le met dans le texte.

### Exercice 2 : Dans dix ans

Niveau : ★☆☆

Rangez un âge dans une variable `age`. Calculez l'âge dans 10 ans et affichez-le.

Sortie attendue (avec `age = 25`) :

```text
Aujourd'hui : 25 ans
Dans 10 ans : 35 ans
```

> **Indice :** créez une deuxième variable : `plus_tard = age + 10`.

> **Solution**
>
> ```ruby
> age = 25
> plus_tard = age + 10
> puts "Aujourd'hui : #{age} ans"
> puts "Dans 10 ans : #{plus_tard} ans"
> ```
>
> ```sortie
> Aujourd'hui : 25 ans
> Dans 10 ans : 35 ans
> ```
>
> Pourquoi ça marche : `age + 10` calcule une nouvelle valeur sans changer `age`. On la
> range sous un autre nom pour l'afficher.

### Exercice 3 : Le prix avec la TVA

Niveau : ★☆☆

Un article coûte 40 euros hors taxe. La TVA est de 20 %. Affichez le prix toutes taxes
comprises.

Sortie attendue :

```text
Prix TTC : 48.0 euros
```

> **Indice :** 20 % de plus, c'est multiplier par `1.2`.

> **Solution**
>
> ```ruby
> prix_ht = 40
> prix_ttc = prix_ht * 1.2
> puts "Prix TTC : #{prix_ttc} euros"
> ```
>
> ```sortie
> Prix TTC : 48.0 euros
> ```
>
> Pourquoi ça marche : `1.2` est un nombre à virgule, donc le résultat aussi : Ruby
> affiche `48.0`. Voir [les nombres](nombres.md).

### Exercice 4 : Partager des bonbons

Niveau : ★☆☆

17 bonbons sont partagés entre 5 enfants. Combien chacun en reçoit-il ? Combien en
reste-t-il ?

Sortie attendue :

```text
Chacun : 3
Reste : 2
```

> **Indice :** entre deux entiers, `/` donne le quotient sans virgule.

> **Indice :** `%` donne le reste de la division.

> **Solution**
>
> ```ruby
> bonbons = 17
> enfants = 5
> puts "Chacun : #{bonbons / enfants}"
> puts "Reste : #{bonbons % enfants}"
> ```
>
> ```sortie
> Chacun : 3
> Reste : 2
> ```
>
> Pourquoi ça marche : 5 × 3 = 15, et 17 − 15 = 2. Ici, la division entière est
> exactement ce qu'on veut (voir [les pièges](pieges.md)).

## Conditions et boucles

### Exercice 5 : Positif, négatif ou zéro

Niveau : ★☆☆

Pour chaque nombre de la liste `[4, -2, 0]`, affichez s'il est positif, négatif ou nul.

Sortie attendue :

```text
4 est positif
-2 est négatif
0 est nul
```

> **Indice :** commencez par un seul nombre, `n = 4`, avec `if`, `elsif` et `else`.

> **Indice :** ensuite, entourez votre `if` d'une boucle `[4, -2, 0].each do |n| ... end`.

> **Solution**
>
> ```ruby
> [4, -2, 0].each do |n|
>   if n > 0
>     puts "#{n} est positif"
>   elsif n < 0
>     puts "#{n} est négatif"
>   else
>     puts "#{n} est nul"
>   end
> end
> ```
>
> ```sortie
> 4 est positif
> -2 est négatif
> 0 est nul
> ```
>
> Pourquoi ça marche : Ruby teste les conditions dans l'ordre et s'arrête à la première
> vraie. Si aucune n'est vraie, il prend le `else`.

### Exercice 6 : Compter jusqu'à 5

Niveau : ★☆☆

Affichez les nombres de 1 à 5, un par ligne.

Sortie attendue :

```text
1
2
3
4
5
```

> **Indice :** `(1..5)` décrit les nombres de 1 à 5, 5 compris.

> **Solution**
>
> ```ruby
> (1..5).each do |n|
>   puts n
> end
> ```
>
> ```sortie
> 1
> 2
> 3
> 4
> 5
> ```
>
> Pourquoi ça marche : `each` donne chaque valeur à `n`, une par tour. `1.upto(5)` marche
> aussi.

### Exercice 7 : Le compte à rebours

Niveau : ★☆☆

Affichez un compte à rebours de 3 à 1, puis `Décollage !`.

Sortie attendue :

```text
3...
2...
1...
Décollage !
```

> **Indice :** `(3..1)` est vide ! Utilisez `3.downto(1)`.

> **Solution**
>
> ```ruby
> 3.downto(1) do |n|
>   puts "#{n}..."
> end
> puts "Décollage !"
> ```
>
> ```sortie
> 3...
> 2...
> 1...
> Décollage !
> ```
>
> Pourquoi ça marche : `downto` descend de 3 à 1. Le dernier `puts` est après `end` : il
> ne s'exécute qu'une fois, à la fin.

### Exercice 8 : La somme de 1 à 10

Niveau : ★☆☆

Additionnez les nombres de 1 à 10 avec une boucle, puis affichez le total.

Sortie attendue :

```text
Total : 55
```

> **Indice :** créez `total = 0` **avant** la boucle.

> **Indice :** dans la boucle, ajoutez le nombre du tour : `total += n`.

> **Solution**
>
> ```ruby
> total = 0
> (1..10).each do |n|
>   total += n
> end
> puts "Total : #{total}"
> ```
>
> ```sortie
> Total : 55
> ```
>
> Pourquoi ça marche : `total` grandit à chaque tour.
>
> | Étape | n | total |
> |---|---|---|
> | avant | – | 0 |
> | tour 1 | 1 | 1 |
> | tour 2 | 2 | 3 |
> | tour 3 | 3 | 6 |
> | … | … | … |
> | tour 10 | 10 | 55 |
>
> En Ruby, `(1..10).sum` fait la même chose en une fois.

## Méthodes

### Exercice 9 : Le carré

Niveau : ★☆☆

Écrivez une méthode `carre` qui reçoit un nombre et renvoie ce nombre multiplié par
lui-même. Affichez `carre(4)` et `carre(9)`.

Sortie attendue :

```text
16
81
```

> **Indice :** une méthode commence par `def carre(n)` et finit par `end`.

> **Indice :** la dernière ligne de la méthode est la valeur renvoyée : `n * n`.

> **Solution**
>
> ```ruby
> def carre(n)
>   n * n
> end
>
> puts carre(4)
> puts carre(9)
> ```
>
> ```sortie
> 16
> 81
> ```
>
> Pourquoi ça marche : quand vous appelez `carre(4)`, `n` vaut 4 dans la méthode. Elle
> renvoie `4 * 4`, que `puts` affiche. Voir [les méthodes](fonctions.md).

### Exercice 10 : Le plus grand des deux

Niveau : ★☆☆

Écrivez une méthode `plus_grand(a, b)` qui renvoie le plus grand des deux nombres, avec
un `if`.

Sortie attendue :

```text
8
12
```

pour `plus_grand(8, 3)` puis `plus_grand(5, 12)`.

> **Indice :** si `a > b`, la réponse est `a`. Sinon, c'est `b`.

> **Solution**
>
> ```ruby
> def plus_grand(a, b)
>   if a > b
>     a
>   else
>     b
>   end
> end
>
> puts plus_grand(8, 3)
> puts plus_grand(5, 12)
> ```
>
> ```sortie
> 8
> 12
> ```
>
> Pourquoi ça marche : un `if` renvoie la valeur de la branche choisie, et c'est la
> dernière chose de la méthode. Ruby a aussi `[a, b].max`.

## Textes

### Exercice 11 : Crier

Niveau : ★☆☆

Écrivez une méthode `crier(texte)` qui renvoie le texte en majuscules, suivi de `!`.

Sortie attendue :

```text
BONJOUR !
ATTENTION !
```

> **Indice :** `"bonjour".upcase` donne `"BONJOUR"`.

> **Solution**
>
> ```ruby
> def crier(texte)
>   "#{texte.upcase} !"
> end
>
> puts crier("bonjour")
> puts crier("attention")
> ```
>
> ```sortie
> BONJOUR !
> ATTENTION !
> ```
>
> Pourquoi ça marche : `upcase` renvoie un nouveau texte en majuscules. On le place dans
> un texte qui ajoute ` !` à la fin.

### Exercice 12 : Les initiales

Niveau : ★☆☆

À partir de `"Marie Curie"`, affichez les initiales et le nombre de lettres du prénom.

Sortie attendue :

```text
Initiales : M.C.
Le prénom a 5 lettres
```

> **Indice :** `"Marie Curie".split` donne `["Marie", "Curie"]`.

> **Indice :** `mot[0]` donne la première lettre, `mot.length` le nombre de caractères.

> **Solution**
>
> ```ruby
> nom_complet = "Marie Curie"
> prenom, nom = nom_complet.split
> puts "Initiales : #{prenom[0]}.#{nom[0]}."
> puts "Le prénom a #{prenom.length} lettres"
> ```
>
> ```sortie
> Initiales : M.C.
> Le prénom a 5 lettres
> ```
>
> Pourquoi ça marche : `split` coupe aux espaces. `prenom, nom = ...` range le premier
> morceau dans `prenom` et le second dans `nom`. Voir [les textes](texte.md).

## Tableaux, hashs et intervalles

### Exercice 13 : La liste de courses

Niveau : ★☆☆

Rangez `pain`, `lait` et `pommes` dans un tableau. Affichez-les numérotés à partir de 1,
puis le nombre d'articles.

Sortie attendue :

```text
1. pain
2. lait
3. pommes
3 articles
```

> **Indice :** `each_with_index` donne l'élément **et** sa position (qui commence à 0).

> **Indice :** affichez `i + 1` pour commencer à 1.

> **Solution**
>
> ```ruby
> courses = ["pain", "lait", "pommes"]
> courses.each_with_index do |article, i|
>   puts "#{i + 1}. #{article}"
> end
> puts "#{courses.size} articles"
> ```
>
> ```sortie
> 1. pain
> 2. lait
> 3. pommes
> 3 articles
> ```
>
> Pourquoi ça marche : les positions vont de 0 à 2. En ajoutant 1, on affiche 1 à 3,
> comme sur une vraie liste.

### Exercice 14 : La moyenne des notes

Niveau : ★☆☆

Avec les notes `[12, 15, 9, 16]`, affichez la plus haute, la plus basse et la moyenne.

Sortie attendue :

```text
Plus haute : 16
Plus basse : 9
Moyenne : 13.0
```

> **Indice :** un tableau connaît `max`, `min`, `sum` et `size`.

> **Indice :** `sum / size` ferait une division entière. Utilisez `sum.fdiv(size)`.

> **Solution**
>
> ```ruby
> notes = [12, 15, 9, 16]
> puts "Plus haute : #{notes.max}"
> puts "Plus basse : #{notes.min}"
> puts "Moyenne : #{notes.sum.fdiv(notes.size)}"
> ```
>
> ```sortie
> Plus haute : 16
> Plus basse : 9
> Moyenne : 13.0
> ```
>
> Pourquoi ça marche : 12 + 15 + 9 + 16 = 52, et 52 ÷ 4 = 13. `fdiv` garde la virgule,
> même quand le résultat tombe juste.

### Exercice 15 : Le prix des fruits

Niveau : ★☆☆

Rangez les prix dans un hash : pommes 2, kiwis 3, poires 4. Affichez chaque prix, puis
la mention d'un panier de 9 euros : `petit` jusqu'à 5, `moyen` de 6 à 10, `gros` au-delà.

Sortie attendue :

```text
pommes : 2 €
kiwis : 3 €
poires : 4 €
Panier de 9 € : moyen
```

> **Indice :** `prix.each do |fruit, montant| ... end` donne chaque clé et sa valeur.

> **Indice :** pour la mention, un `case` avec des intervalles : `when 0..5`, `when 6..10`.

> **Solution**
>
> ```ruby
> prix = { "pommes" => 2, "kiwis" => 3, "poires" => 4 }
> prix.each do |fruit, montant|
>   puts "#{fruit} : #{montant} €"
> end
>
> panier = 9
> mention = case panier
>           when 0..5 then "petit"
>           when 6..10 then "moyen"
>           else "gros"
>           end
> puts "Panier de #{panier} € : #{mention}"
> ```
>
> ```sortie
> pommes : 2 €
> kiwis : 3 €
> poires : 4 €
> Panier de 9 € : moyen
> ```
>
> Pourquoi ça marche : un hash garde l'ordre dans lequel vous avez rangé les clés. `9`
> est dans `6..10`, donc `case` choisit `"moyen"`. Voir [les intervalles](intervalles.md).

## Et ensuite ?

Bravo si vous êtes arrivé ici ! D'autres exercices, plus variés, vous attendent dans
[les exercices corrigés](exercices.md). Mais d'abord, découvrez les blocs : ils sont
partout en Ruby.

Suite : [Blocs, itérateurs et Enumerable](blocs.md)
