# Glossaire

Ce glossaire explique simplement les mots de la programmation que vous croiserez dans cette documentation, classés par ordre alphabétique.

## A

### Algorithme

Une suite d'étapes précises pour résoudre un problème, comme une recette.
Un programme est un algorithme écrit dans un langage de programmation.

### Argument

La valeur que vous donnez à une [fonction](#fonction) quand vous l'appelez.
Dans `double(21)`, l'argument est `21`.

## B

### Bibliothèque

Du code déjà écrit par d'autres, que vous pouvez réutiliser dans votre
programme. Elle vous évite de tout réinventer. Voir aussi [paquet](#paquet).

### Bloc

Un groupe d'instructions qui vont ensemble. En Ruby et en Rythme, un bloc
s'écrit souvent entre `do` et `end` ; en Rust, entre accolades `{ }`.

```ruby
3.times do
  puts "Bonjour"
end
```

```sortie
Bonjour
Bonjour
Bonjour
```

### Booléen

Une valeur qui ne peut être que **vraie** ou **fausse** : `true` ou `false`.
C'est le résultat d'une comparaison comme `10 > 3`.

### Boucle

Une instruction qui répète du code plusieurs fois, par exemple pour chaque
élément d'une liste. Voir aussi [itération](#iteration).

```ruby
[1, 2, 3].each { |n| puts n * 10 }
```

```sortie
10
20
30
```

### Bug

Une erreur dans un programme : il plante, ou il ne fait pas ce que vous
vouliez. Le mot veut dire « insecte » en anglais.

## C

### Chaîne de caractères

Un texte, écrit entre guillemets : `"Bonjour"`. On dit souvent **chaîne**,
ou *string* en anglais.

### Classe

Un modèle pour fabriquer des [objets](#objet) du même genre. La classe
`Chien` décrit ce qu'est un chien ; chaque chien créé est un objet. Ruby
utilise beaucoup les classes ; Rythme n'en a pas.

### Closure

Une fonction sans nom qui se souvient des variables autour d'elle, là où
elle a été créée. En français, on dit parfois **fermeture**.

```ruby
bonus = 5
ajouter_bonus = ->(x) { x + bonus }
puts ajouter_bonus.call(10)
```

```sortie
15
```

### Commentaire

Un texte dans le code que l'ordinateur ignore. Il sert à expliquer le code
aux humains. En Ruby et en Rythme, il commence par `#` ; en Rust, par `//`.

```ruby
# Ceci est un commentaire : rien ne s'affiche.
puts "Seule cette ligne compte"
```

```sortie
Seule cette ligne compte
```

### Compilateur

Un programme qui traduit tout votre code en langage machine **avant** de le
lancer. Rust utilise un compilateur. Voir aussi
[interpréteur](#interpreteur).

### Condition

Un test qui décide quel code exécuter : « **si** il pleut, prendre un
parapluie, **sinon** des lunettes de soleil ». Elle s'écrit avec `if`.

```ruby
meteo = "pluie"
if meteo == "pluie"
  puts "Parapluie"
else
  puts "Lunettes de soleil"
end
```

```sortie
Parapluie
```

### Constante

Une valeur nommée qui ne doit jamais changer, comme `PI = 3.14159`. Par
habitude, son nom s'écrit souvent en majuscules.

## D

### Débogage

Chercher et corriger un [bug](#bug). On lit les messages d'erreur, on
affiche des valeurs, on teste petit à petit. En anglais : *debugging*.

### Dictionnaire

Une collection qui associe une **valeur** à une **clé**, comme un vrai
dictionnaire associe une définition à un mot. En Ruby, on dit *hash* ; en
Rust, *HashMap*.

```ruby
ages = { "Ada" => 36, "Alan" => 41 }
puts ages["Ada"]
```

```sortie
36
```

## E

### Éditeur de code

Le logiciel dans lequel on écrit du code, comme VS Code. Voir
[Installer un éditeur de code](editeur.md).

### Entier

Un nombre sans virgule : `0`, `42`, `-7`. En anglais : *integer*. Voir
aussi [flottant](#flottant).

### Erreur

Un message qui dit que quelque chose s'est mal passé. Il indique souvent le
fichier, la ligne et la cause. Lisez-le : c'est votre meilleur indice.

### Expression

Un morceau de code qui donne une [valeur](#valeur). `2 + 3` est une
expression qui vaut `5` ; `"Ada"` est une expression qui vaut `"Ada"`.

## F

### Fichier source

Le fichier texte qui contient votre code, par exemple `bonjour.rb`,
`main.rs` ou `bonjour.ry`.

### Flottant

Un nombre à virgule : `3.14`, `0.5`. En programmation, la virgule s'écrit
avec un **point**. En anglais : *float*.

### Fonction

Un morceau de code qui porte un nom et qu'on peut réutiliser. Elle reçoit
des [arguments](#argument) et renvoie un résultat.

```rythme
fn double(x) = x * 2
print(double(21))
```

```sortie
42
```

## I

### Immuable

Qui ne peut pas changer. Une valeur immuable reste la même du début à la
fin. En Rythme, toutes les valeurs sont immuables ; en Rust, les variables
le sont par défaut. Voir aussi [mutable](#mutable).

### Indice

La position d'un élément dans une [liste](#liste). On compte à partir de
**zéro** : le premier élément a l'indice `0`.

```ruby
fruits = ["pomme", "poire", "kiwi"]
puts fruits[0]
puts fruits[2]
```

```sortie
pomme
kiwi
```

### Interpréteur

Un programme qui lit votre code et l'exécute tout de suite, ligne après
ligne. Ruby et Rythme utilisent un interpréteur. Voir aussi
[compilateur](#compilateur).

### Itération

Un passage dans une [boucle](#boucle). Itérer sur une liste, c'est traiter
ses éléments un par un.

## L

### Liste

Une suite ordonnée de valeurs : `[1, 2, 3]`. En Ruby, on dit *tableau*
(*Array*) ; en Rust, *Vec*. Chaque élément a un [indice](#indice).

## M

### Méthode

Une fonction attachée à une valeur. On l'appelle avec un point :
`"bonjour".upcase` appelle la méthode `upcase` du texte.

```ruby
puts "bonjour".upcase
```

```sortie
BONJOUR
```

### Module

Un moyen de ranger du code qui va ensemble sous un même nom, souvent dans
un fichier à part. Il aide à organiser un gros programme.

### Mutable

Qui peut changer. Une variable mutable peut recevoir une nouvelle valeur.
En Rust, on l'indique avec `mut`.

```rust
let mut compteur = 1;
compteur = compteur + 1;
println!("{compteur}");
```

```sortie
2
```

## N

### Null

Une valeur spéciale qui veut dire « rien » ou « absent ». Ruby l'appelle
`nil`. Rust et Rythme n'ont pas de `null` : ils utilisent `None`, qui oblige
à prévoir le cas où la valeur manque.

## O

### Objet

Une valeur qui regroupe des données et des [méthodes](#methode). En Ruby,
tout est objet : même le nombre `5`.

### Opérateur

Un symbole qui fait une opération : `+`, `-`, `*`, `/` pour calculer, `==`
pour comparer, `&&` pour « et ».

## P

### Paquet

Une [bibliothèque](#bibliotheque) prête à installer. En Ruby, on dit *gem* ;
en Rust, *crate*.

### Paramètre

Le nom qui reçoit l'[argument](#argument) dans la définition d'une
fonction. Dans `fn double(x) = x * 2`, le paramètre est `x`.

### Portée

La zone du code où une variable existe. Une variable créée dans une fonction
n'existe pas en dehors. En anglais : *scope*.

### Programme

Une liste d'instructions qu'un ordinateur exécute. Voir
[Qu'est-ce que programmer ?](programmer.md).

## R

### Récursion

Quand une fonction s'appelle elle-même, sur un problème plus petit, jusqu'à
un cas simple. Ici, on additionne les nombres de `n` jusqu'à 0 :

```rythme
fn total(0) = 0
fn total(n) = n + total(n - 1)
print(total(4))
```

```sortie
10
```

### REPL

Un mode où l'on tape une ligne de code et où l'on voit le résultat tout de
suite. Le nom vient de l'anglais *Read, Eval, Print, Loop* (lire, évaluer,
afficher, recommencer). Exemples : `irb` pour Ruby, `rythme` pour Rythme.

## S

### Syntaxe

Les règles d'écriture d'un langage : où mettre les parenthèses, les
guillemets, les mots-clés. Une faute de syntaxe empêche le programme de
démarrer.

## T

### Terminal

Une fenêtre où l'on tape des commandes au clavier. Voir
[Le terminal en 10 minutes](terminal.md).

### Test

Un petit morceau de code qui vérifie qu'un autre code marche bien. Si vous
changez votre programme, les tests vous préviennent quand vous cassez
quelque chose.

### Type

La sorte d'une valeur : un [entier](#entier), un texte, un
[booléen](#booleen), une [liste](#liste)... Le type dit ce qu'on peut faire
avec la valeur.

```ruby
puts 42.class
puts "Ada".class
```

```sortie
Integer
String
```

## V

### Valeur

Une donnée : un nombre, un texte, une liste, `true`... C'est ce que
manipule un programme.

### Variable

Un nom qui désigne une [valeur](#valeur), comme une étiquette collée sur
une boîte. On s'en sert pour retrouver la valeur plus tard.

```ruby
prenom = "Ada"
puts prenom
```

```sortie
Ada
```

## En résumé

- Un programme manipule des valeurs : entiers, flottants, textes, booléens, listes.
- On range les valeurs dans des variables, et on réutilise du code avec des fonctions.
- Les conditions choisissent, les boucles répètent.
- Un interpréteur exécute tout de suite, un compilateur traduit d'abord.
- Un bug se corrige en lisant le message d'erreur : c'est le débogage.
