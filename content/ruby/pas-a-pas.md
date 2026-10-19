# Suivre un programme pas à pas en Ruby

Cette page vous apprend à exécuter un programme Ruby « dans votre tête », ligne par ligne, avec un tableau qui suit la valeur de chaque variable.

> **Objectifs :**
>
> - lire un programme ligne par ligne, dans l'ordre où Ruby l'exécute ;
> - remplir un tableau de suivi des [variables](../debuter/glossaire.md#variable) ;
> - deviner la sortie d'un programme avant de le lancer ;
> - faire afficher au programme ses propres étapes.

## Pourquoi suivre un programme à la main ?

Un ordinateur ne devine rien. Il lit une ligne, la fait, puis passe à la suivante.
Pour comprendre un programme, faites comme lui : **une ligne à la fois**.

C'est comme suivre une recette de cuisine. Vous ne lisez pas toute la recette d'un coup
en espérant que le gâteau apparaisse. Vous cassez les œufs, puis vous ajoutez le sucre,
puis vous mélangez. Et à chaque étape, vous savez ce qu'il y a dans le saladier.

Savoir faire ça vous aide à :

- comprendre un exemple qui vous paraît magique ;
- trouver un [bug](../debuter/glossaire.md#bug) sans aide ;
- réussir les exercices, car vous pouvez vérifier votre idée sur papier.

## L'outil : le tableau de suivi

Prenez une feuille. Faites une colonne par variable, et une ligne par étape.
À chaque ligne de code qui change une variable, écrivez sa **nouvelle** valeur.

```text
  | Ligne | a | b |      <- une colonne par variable
  |-------|---|---|
  |   1   | 5 | – |      <- « – » : la variable n'existe pas encore
  |   2   | 5 | 3 |
```

Imaginez des boîtes avec une étiquette. Une variable, c'est une boîte. Quand vous
écrivez `a = 8`, vous videz la boîte `a` et vous y mettez `8`. L'ancienne valeur est
**perdue**.

```text
  avant a = 8          après a = 8
  +-------+            +-------+
  |   5   |   a        |   8   |   a
  +-------+            +-------+
```

La méthode, toujours la même :

1. **Avant de lancer**, devinez ce qui s'affiche.
2. Remplissez le tableau, ligne par ligne.
3. Lancez le programme et comparez.

> **À retenir :** une variable ne contient qu'une seule valeur à la fois. Quand on lui
> donne une nouvelle valeur, l'ancienne disparaît.

## Programme 1 : des variables qui changent

Voici le premier programme. Il n'a ni condition, ni boucle.

```ruby
a = 5
b = 3
a = a + b
b = a - b
puts a
puts b
```

Avant de lancer, devinez ce qui s'affiche. Écrivez votre réponse sur un papier.

### Le tableau de suivi

| Ligne | Code | a | b | Ce qui se passe |
|---|---|---|---|---|
| 1 | `a = 5` | 5 | – | la boîte `a` reçoit 5 |
| 2 | `b = 3` | 5 | 3 | la boîte `b` reçoit 3 |
| 3 | `a = a + b` | 8 | 3 | Ruby calcule d'abord `5 + 3`, puis range 8 dans `a` |
| 4 | `b = a - b` | 8 | 5 | `a` vaut **déjà** 8 : `8 - 3` donne 5 |
| 5 | `puts a` | 8 | 5 | affiche 8 |
| 6 | `puts b` | 8 | 5 | affiche 5 |

Le piège est à la ligne 4. Beaucoup de gens utilisent l'**ancienne** valeur de `a`,
c'est-à-dire 5. Mais à la ligne 4, `a` a déjà changé.

### La sortie

```sortie
8
5
```

À la ligne 3, lisez le `=` comme une flèche vers la gauche : « calcule `a + b`, puis
mets le résultat dans `a` ». Ce n'est pas une égalité de mathématiques.

```text
  a = a + b
  ^   \___/
  |     |
  |     +-- 1. on calcule avec les valeurs actuelles : 5 + 3 = 8
  +-------- 2. on range le résultat dans a
```

Essayez : remplacez la ligne 4 par `b = b - a`. Remplissez le tableau avant de lancer.
Vous devez trouver `8` puis `-5`.

> **À retenir :** à droite du `=`, on utilise les valeurs **du moment**. Regardez
> toujours la dernière ligne de votre tableau.

## Programme 2 : une condition

Une [condition](../debuter/glossaire.md#condition) fait **sauter** des lignes. Dans le
tableau, notez quelle branche Ruby choisit.

```ruby
temperature = 18
if temperature > 25
  conseil = "un short"
elsif temperature > 15
  conseil = "un pull léger"
else
  conseil = "un manteau"
end
puts "Il fait #{temperature} degrés : prenez #{conseil}."
```

Avant de lancer, devinez ce qui s'affiche.

### Le tableau de suivi

| Ligne | Code | Test | temperature | conseil |
|---|---|---|---|---|
| 1 | `temperature = 18` | | 18 | – |
| 2 | `if temperature > 25` | `18 > 25` → `false` | 18 | – |
| 4 | `elsif temperature > 15` | `18 > 15` → `true` | 18 | – |
| 5 | `conseil = "un pull léger"` | | 18 | `"un pull léger"` |
| 9 | `puts ...` | | 18 | `"un pull léger"` |

Regardez la colonne « Ligne » : 1, 2, 4, 5, 9. Les lignes 3, 6 et 7 ne sont **jamais**
exécutées. Ruby a trouvé une condition vraie à la ligne 4 : il fait sa branche, puis
saute directement après le `end`.

```text
  temperature > 25 ?  --non-->  temperature > 15 ?  --oui-->  "un pull léger"
        |                              |                             |
       oui                            non                            |
        v                              v                             |
   "un short"                    "un manteau"                        |
                                                                     v
                                                              puts (ligne 9)
```

### La sortie

```sortie
Il fait 18 degrés : prenez un pull léger.
```

Essayez : mettez `temperature = 30`. Le premier test devient vrai, et vous obtenez
« prenez un short ». Puis essayez `temperature = 15` : `15 > 15` est **faux**, donc
c'est le manteau.

> **À retenir :** dans un `if ... elsif ... else`, une seule branche est exécutée : la
> première dont le test est vrai. Notez chaque test et son résultat dans le tableau.

## Programme 3 : une boucle qui accumule

Une [boucle](../debuter/glossaire.md#boucle) répète des lignes. Dans le tableau, on
écrit **une ligne par tour**.

Vous mettez 10 euros dans une tirelire magique. Chaque semaine, son contenu double.
Au bout de combien de semaines dépasse-t-elle 50 euros ?

```ruby
argent = 10
semaine = 0
while argent < 50
  semaine += 1
  argent = argent * 2
end
puts "Semaine #{semaine} : #{argent} euros"
```

Avant de lancer, devinez ce qui s'affiche.

### Le tableau de suivi

| Étape | Test `argent < 50` | semaine | argent |
|---|---|---|---|
| avant la boucle | | 0 | 10 |
| tour 1 | `10 < 50` → `true` | 1 | 20 |
| tour 2 | `20 < 50` → `true` | 2 | 40 |
| tour 3 | `40 < 50` → `true` | 3 | 80 |
| fin | `80 < 50` → `false` | 3 | 80 |

Le test est fait **au début** de chaque tour. Au tour 3, `argent` vaut encore 40 au
moment du test : on entre. Puis il passe à 80. Le test suivant est faux : la boucle
s'arrête.

```text
          +-------------------------+
          v                         |
   argent < 50 ? --oui--> semaine += 1
          |               argent = argent * 2 ---+
         non
          |
          v
        puts
```

### La sortie

```sortie
Semaine 3 : 80 euros
```

Essayez : remplacez `50` par `80`. Au tour 4, le test `80 < 80` est faux. Le résultat
ne change pas : « Semaine 3 : 80 euros ». Puis remplacez `<` par `<=` : il y a un tour
de plus, et vous obtenez « Semaine 4 : 160 euros ».

> **À retenir :** pour une boucle, faites une ligne par tour, et notez le test **avant**
> de faire le tour. La dernière ligne du tableau est le test qui échoue.

## Programme 4 : une méthode appelée deux fois

Une [méthode](../debuter/glossaire.md#methode) est comme une machine à café. Vous
mettez quelque chose dedans (le [paramètre](../debuter/glossaire.md#parametre)), elle
travaille, et elle vous rend un résultat. Vous pouvez l'utiliser plusieurs fois.

```ruby
def prix_ttc(prix_ht)
  taxe = prix_ht * 20 / 100
  prix_ht + taxe
end

livre = prix_ttc(10)
stylo = prix_ttc(5)
puts "Livre : #{livre}"
puts "Stylo : #{stylo}"
puts "Total : #{livre + stylo}"
```

Avant de lancer, devinez ce qui s'affiche.

### Qui appelle qui ?

Les lignes 1 à 4 ne font **rien** tout de suite : elles décrivent la machine. Le
programme commence vraiment à la ligne 6.

```text
  programme principal                     méthode prix_ttc
  -------------------                     ----------------
  ligne 6 : prix_ttc(10)  ---- 10 ---->   prix_ht = 10
                                          taxe = 2
            livre = 12    <--- 12 -----   renvoie 12

  ligne 7 : prix_ttc(5)   ----  5 ---->   prix_ht = 5
                                          taxe = 1
            stylo = 6     <---  6 -----   renvoie 6
```

### Le tableau de suivi

Les variables de la méthode (`prix_ht`, `taxe`) sont neuves à **chaque** appel. Elles
disparaissent quand la méthode a fini.

| Étape | Où ? | prix_ht | taxe | livre | stylo |
|---|---|---|---|---|---|
| ligne 6 : appel | méthode | 10 | – | – | – |
| ligne 2 | méthode | 10 | 2 | – | – |
| ligne 3 : renvoie 12 | méthode | 10 | 2 | – | – |
| retour ligne 6 | principal | – | – | 12 | – |
| ligne 7 : appel | méthode | 5 | – | 12 | – |
| ligne 2 | méthode | 5 | 1 | 12 | – |
| ligne 3 : renvoie 6 | méthode | 5 | 1 | 12 | – |
| retour ligne 7 | principal | – | – | 12 | 6 |

À la ligne 2, Ruby calcule de gauche à droite : `5 * 20` donne 100, puis `100 / 100`
donne 1. La dernière ligne de la méthode (`prix_ht + taxe`) est la valeur renvoyée :
c'est le retour implicite (voir [les méthodes](fonctions.md#le-retour-implicite)).

### La sortie

```sortie
Livre : 12
Stylo : 6
Total : 18
```

Essayez : ajoutez `puts taxe` à la fin du programme. Ruby s'arrête
avec une erreur `NameError` (« undefined local variable or method ») : `taxe` n'existe
que **dans** la méthode.
Retirez ensuite la ligne et appelez `prix_ttc(7)` : `7 * 20 / 100` donne 1 (division
entière), donc le prix est 8.

> **À retenir :** à chaque appel, sautez dans la méthode avec la nouvelle valeur du
> paramètre, faites ses lignes, puis revenez avec le résultat à l'endroit de l'appel.

## Programme 5 : parcourir un tableau avec une condition

Dernier programme : une boucle sur un tableau, avec des tests à l'intérieur. On compte
les notes réussies et on cherche la meilleure.

```ruby
notes = [12, 7, 15, 9, 18]
reussies = 0
meilleure = 0
notes.each do |note|
  if note >= 10
    reussies += 1
  end
  if note > meilleure
    meilleure = note
  end
end
puts "Réussies : #{reussies}"
puts "Meilleure : #{meilleure}"
```

Avant de lancer, devinez ce qui s'affiche.

### Le tableau de suivi

`each` donne les éléments un par un à `note`, de gauche à droite, comme un professeur
qui prend les copies sur la pile.

| Tour | note | `note >= 10` ? | reussies | `note > meilleure` ? | meilleure |
|---|---|---|---|---|---|
| avant | – | | 0 | | 0 |
| 1 | 12 | oui | 1 | `12 > 0` oui | 12 |
| 2 | 7 | non | 1 | `7 > 12` non | 12 |
| 3 | 15 | oui | 2 | `15 > 12` oui | 15 |
| 4 | 9 | non | 2 | `9 > 15` non | 15 |
| 5 | 18 | oui | 3 | `18 > 15` oui | 18 |

Quand un test est faux, la variable **garde** sa valeur : on recopie la ligne du
dessus.

```text
  notes :  [ 12 ,  7 , 15 ,  9 , 18 ]
             ^     ^    ^    ^    ^
  tour :     1     2    3    4    5
  réussie :  ok    -    ok   -    ok    -> 3
  record :   12    -    15   -    18    -> 18
```

### La sortie

```sortie
Réussies : 3
Meilleure : 18
```

Essayez : remplacez le tableau par `[-4, -2, -9]`. La meilleure note affichée est 0,
alors qu'elle n'est pas dans le tableau ! Votre tableau de suivi montre pourquoi :
`meilleure` part de 0 et aucun test `note > meilleure` n'est vrai. La correction :
commencer avec `meilleure = notes[0]`.

> **À retenir :** pour une boucle sur un tableau, une colonne pour l'élément du tour, une
> colonne par test, une colonne par variable qui change. Quand un test est faux, recopiez
> la valeur précédente.

## Faire afficher les étapes par le programme

Remplir un tableau à la main est le meilleur entraînement. Mais vous pouvez aussi
demander au programme d'afficher **son propre** tableau de suivi. Ajoutez un `puts`
avec des étiquettes dans la boucle :

```ruby
argent = 10
semaine = 0
while argent < 50
  semaine += 1
  argent = argent * 2
  puts "tour #{semaine} : semaine = #{semaine}, argent = #{argent}"
end
puts "Fin : semaine #{semaine}, #{argent} euros"
```

```sortie
tour 1 : semaine = 1, argent = 20
tour 2 : semaine = 2, argent = 40
tour 3 : semaine = 3, argent = 80
Fin : semaine 3, 80 euros
```

Les étiquettes (`semaine = `, `argent = `) sont importantes. Sans elles, vous voyez une
colonne de nombres et vous ne savez plus lequel est lequel.

`p` est pratique pour voir **exactement** ce que contient une variable : il montre les
guillemets d'un texte et affiche `nil` en clair.

```ruby
mots = ["chat", "", "chien"]
mots.each do |mot|
  print "mot = "
  p mot
end
```

```sortie
mot = "chat"
mot = ""
mot = "chien"
```

Avec `puts`, le texte vide aurait donné une ligne avec juste `mot = ` : facile à rater.

Essayez : dans le programme 5, ajoutez
`puts "tour : note = #{note}, reussies = #{reussies}, meilleure = #{meilleure}"` juste
avant le `end` de la boucle. Comparez avec votre tableau.

> **À retenir :** un `puts` avec des étiquettes dans la boucle affiche le tableau de
> suivi tout seul. Retirez ces lignes quand le programme marche.

## Le débogueur, en deux phrases

Ruby peut aussi mettre votre programme en pause : écrivez `binding.irb` à une ligne, et
vous pouvez taper le nom de chaque variable pour voir sa valeur à ce moment-là. Le
débogueur `debug` va plus loin et avance ligne par ligne ; tout est expliqué dans
[le chapitre sur le débogage](debogage.md).

> **À retenir :** le débogueur fait le tableau de suivi à votre place, mais il faut
> savoir le faire à la main pour comprendre ce qu'il montre.

## Erreurs fréquentes

### Réutiliser une ancienne valeur

On veut échanger le contenu de deux variables. On écrit deux lignes, et on oublie que
la première a déjà changé `a`.

```ruby
a = "rouge"
b = "bleu"
a = b
b = a
puts a
puts b
```

```sortie
bleu
bleu
```

Le tableau de suivi montre la panne : après la ligne 3, `a` vaut `"bleu"`, et
`"rouge"` est perdu. La ligne 4 recopie donc `"bleu"`. La correction : une boîte de
plus pour garder l'ancienne valeur, ou l'échange de Ruby en une ligne.

```ruby
a = "rouge"
b = "bleu"
a, b = b, a
puts a
puts b
```

```sortie
bleu
rouge
```

### Oublier l'étape « avant la boucle »

On veut additionner des nombres, mais on oublie de créer le total avant la boucle.

```ruby norun
[3, 4].each do |n|
  total += n
end
```

```text
a.rb:2:in `block in <main>': undefined method `+' for nil (NoMethodError)

  total += n
        ^
	from a.rb:1:in `each'
	from a.rb:1:in `<main>'
```

Dans le tableau, la ligne « avant » serait vide : `total` n'a pas de valeur, donc il
vaut `nil`, et on ne peut pas ajouter un nombre à `nil`. La correction : écrire
`total = 0` **avant** la boucle.

### Remettre à zéro à chaque tour

Autre version du même oubli : le total est créé **dans** la boucle.

```ruby
[3, 4, 5].each do |n|
  total = 0
  total += n
  puts total
end
```

```sortie
3
4
5
```

Le tableau de suivi montre que `total` repart de 0 à chaque tour. Sortez `total = 0`
de la boucle : vous obtiendrez 3, 7, 12.

### Un tour de trop ou de moins

`(1..3)` contient 3 nombres, mais `(1...3)` n'en contient que 2. On se trompe souvent
d'un seul tour.

```ruby
(1...3).each do |i|
  puts "tour #{i}"
end
```

```sortie
tour 1
tour 2
```

Pour éviter ça, écrivez dans votre tableau le **premier** et le **dernier** tour avant
de remplir le reste. Voir [les intervalles](intervalles.md).

## Testez-vous

1. Que vaut `x` à la fin ?

   ```ruby
   x = 2
   x = x * 3
   x = x + x
   puts x
   ```

   ```sortie
   12
   ```

> **Réponse**
>
> 12. Ligne 1 : `x` vaut 2. Ligne 2 : `2 * 3`, donc 6. Ligne 3 : `6 + 6`, donc 12. À
> chaque ligne, on utilise la valeur que `x` a **à ce moment-là**.

2. Combien de tours fait cette boucle, et qu'affiche-t-elle ?

   ```ruby
   n = 1
   while n < 20
     n = n + 5
   end
   puts n
   ```

   ```sortie
   21
   ```

> **Réponse**
>
> 4 tours. `n` passe de 1 à 6, 11, 16, puis 21. Le test `21 < 20` est faux : la boucle
> s'arrête et affiche 21.
>
> | Tour | Test | n |
> |---|---|---|
> | avant | | 1 |
> | 1 | `1 < 20` oui | 6 |
> | 2 | `6 < 20` oui | 11 |
> | 3 | `11 < 20` oui | 16 |
> | 4 | `16 < 20` oui | 21 |
> | fin | `21 < 20` non | 21 |

3. Pourquoi les lignes d'une méthode ne s'exécutent-elles pas quand Ruby lit le `def` ?

> **Réponse**
>
> Le `def` décrit seulement la méthode, comme une recette écrite dans un livre. Elle
> est faite seulement quand on l'**appelle**, par exemple `prix_ttc(10)`. Dans le
> tableau de suivi, on saute donc le `def` et on commence à la première ligne en
> dehors.

## En résumé

- Un ordinateur exécute une ligne à la fois : suivez-le de la même façon.
- Le tableau de suivi a une colonne par variable et une ligne par étape (ou par tour).
- À droite du `=`, utilisez toujours la valeur **du moment**, celle de la dernière
  ligne du tableau.
- Pour une condition, notez le test et son résultat ; pour une boucle, notez le test
  **avant** chaque tour ; pour une méthode, sautez dedans puis revenez avec le
  résultat.
- `puts` avec des étiquettes, ou `p`, affiche les étapes ; le débogueur va plus loin.

Suite : [Les pièges du débutant en Ruby](pieges.md)
