# Les variables, expliquées simplement

Une variable, c'est un nom donné à une valeur pour s'en souvenir : cette page montre comment en créer une, la lire, la changer, et pourquoi l'ordre des lignes compte.

> **Objectifs :**
>
> - comprendre ce qu'est une variable : un nom et une valeur ;
> - créer une variable, la lire, puis changer sa valeur ;
> - suivre un programme ligne par ligne avec un tableau ;
> - échanger les valeurs de deux variables ;
> - choisir des noms clairs ;
> - reconnaître le même programme en Python, JavaScript, Ruby et Rust.

## Une valeur, un nom

Un programme manipule des **valeurs** : un nombre comme `42`, un texte
comme `"Ada"`. Voir [valeur](glossaire.md#valeur).

Pour se souvenir d'une valeur, on lui donne un **nom**. Le nom et la
valeur, ensemble, forment une **variable**. Voir
[variable](glossaire.md#variable).

```python
age = 30
print(age)
```

```sortie
30
```

Lisez la première ligne de droite à gauche : « prends `30`, et appelle-le
`age` ». La deuxième ligne affiche la valeur qui porte le nom `age`.

> **À retenir :** une variable, c'est un nom collé sur une valeur.

## La boîte avec une étiquette

Une image aide beaucoup : une **boîte** avec une **étiquette**.

```text
   étiquette : age
  +-----------+
  |    30     |   <- la valeur, dans la boîte
  +-----------+
```

- L'étiquette, c'est le **nom** : `age`.
- Ce qu'il y a dedans, c'est la **valeur** : `30`.
- Quand vous écrivez `age`, l'ordinateur ouvre la boîte et prend ce qu'il
  y a dedans.

Une boîte peut contenir un nombre (`30`, `1.65`), un texte (`"Ada"`), ou
autre chose. Le genre de valeur s'appelle le **type** (voir
[type](glossaire.md#type)).

Un texte s'appelle une **chaîne de caractères**, et on l'écrit entre
guillemets (voir [chaîne de caractères](glossaire.md#chaine-de-caracteres)).
Un nombre à virgule s'écrit avec un **point** : `1.65`.

## Les limites de l'image de la boîte

L'image de la boîte est très utile pour débuter. Mais elle n'est pas
tout à fait juste.

En réalité, la valeur vit quelque part dans la mémoire de l'ordinateur. Le
nom est plutôt une **étiquette attachée à la valeur** par une ficelle.

Pourquoi c'est important ? Parce que deux étiquettes peuvent être
attachées à **la même** valeur. Avec des nombres et des textes, vous ne
verrez pas la différence. Avec des listes, si :

```python
courses = ["pain", "lait"]
liste_de_paul = courses
liste_de_paul.append("oeufs")
print(courses)
```

```sortie
['pain', 'lait', 'oeufs']
```

```text
  courses ─────────┐
                   ├──> ["pain", "lait", "oeufs"]
  liste_de_paul ───┘
```

Il n'y a qu'**une seule** liste, avec deux étiquettes. Quand Paul ajoute
des œufs, la liste de `courses` change aussi, puisque c'est la même.
Vous verrez cela en détail dans [les données](donnees.md).

> **À retenir :** pour les nombres et les textes, pensez « boîte ». Pour
> les listes, pensez « étiquette attachée ».

## Créer une variable

On crée une variable en lui donnant une première valeur, avec le signe `=` :
`ville = "Lyon"`.

Attention : ici, `=` ne veut **pas** dire « est égal à », comme en maths.
Il veut dire « range cette valeur sous ce nom ». On l'appelle
l'**affectation**.

Certains langages demandent un mot en plus, comme `let` en JavaScript et
en Rust : vous le verrez plus bas.

## Lire une variable

Lire une variable, c'est utiliser son nom. L'ordinateur remplace le nom
par la valeur.

```python
prix = 4
quantite = 3
total = prix * quantite
print(total)
```

```sortie
12
```

À la ligne 3, l'ordinateur lit `prix` (4) et `quantite` (3), calcule
`4 * 3`, puis range `12` sous le nom `total`.

Lire une variable ne la change pas. Après le calcul, `prix` vaut toujours 4.

Essayez : mettez `quantite = 10`. Le programme affiche `40`.

## Changer une variable

On peut ranger une nouvelle valeur sous un nom qui existe déjà. L'ancienne
valeur est **oubliée**.

Avec `meteo = "pluie"` puis `meteo = "soleil"`, la boîte `meteo` contient
`"soleil"` : `"pluie"` a disparu.

Très souvent, on calcule la nouvelle valeur à partir de l'ancienne :

```python
score = 10
score = score + 5
print(score)
```

```sortie
15
```

`score = score + 5` choque en maths. En programmation, c'est normal :
l'ordinateur calcule **d'abord** la droite (`10 + 5`, soit 15), **puis**
range le résultat dans `score`.

Il existe un raccourci : `score += 5` veut dire `score = score + 5`.

> **À retenir :** l'ordinateur calcule d'abord à droite du `=`, puis range
> le résultat à gauche.

## L'ordre des lignes compte

L'ordinateur lit les lignes **une par une, de haut en bas**. Une variable
vaut ce qu'on y a rangé **en dernier**.

Pour suivre un programme, faites un **tableau** : une ligne par étape, une
colonne par variable.

```python
a = 2
b = 3
a = a + b
b = a * 2
print(a, b)
```

```sortie
5 10
```

| Étape | Ligne | a | b |
|---|---|---|---|
| 1 | `a = 2` | 2 | – |
| 2 | `b = 3` | 2 | 3 |
| 3 | `a = a + b` | 5 | 3 |
| 4 | `b = a * 2` | 5 | 10 |

À l'étape 4, `a` vaut déjà 5 : `b` reçoit donc `10`, pas `4`.

Inversez les lignes 3 et 4 : le programme affiche `6 4`. Cette fois, `b`
est calculé quand `a` vaut encore 2 : `b` reçoit 4, puis `a` devient
`2 + 4`, soit 6. Mêmes lignes, autre ordre : autre résultat.

Essayez : faites le tableau sur papier **avant** de lancer le programme.
Puis comparez avec ce qu'il affiche.

> **À retenir :** en cas de doute, faites un tableau, étape par étape.

## Échanger deux valeurs

Vous avez deux verres : un de jus, un d'eau. Vous voulez échanger leur
contenu. Il vous faut un **troisième verre**, vide.

Le piège classique, sans troisième verre :

```python
x = 1
y = 2
x = y
y = x
print(x, y)
```

```sortie
2 2
```

À la ligne `x = y`, le `1` de `x` est écrasé. Il est perdu, et il est trop tard.

Avec un troisième verre, `temp` (pour « temporaire ») :

```python
x = 1
y = 2
temp = x
x = y
y = temp
print(x, y)
```

```sortie
2 1
```

| Étape | Ligne | x | y | temp |
|---|---|---|---|---|
| 1 | `x = 1` | 1 | – | – |
| 2 | `y = 2` | 1 | 2 | – |
| 3 | `temp = x` | 1 | 2 | 1 |
| 4 | `x = y` | 2 | 2 | 1 |
| 5 | `y = temp` | 2 | 1 | 1 |

Python (et Ruby) ont un raccourci qui fait tout d'un coup :

```python
x = 1
y = 2
x, y = y, x
print(x, y)
```

```sortie
2 1
```

## Nommer clairement

Le nom d'une variable sert aux **humains**, pas à l'ordinateur. Pour lui,
`x` ou `prix_du_pain` c'est pareil. Pour vous, dans un mois, non.

Comparez `t = p * n` avec ceci :

```python
prix_du_pain = 2
nombre_de_pains = 3
total_a_payer = prix_du_pain * nombre_de_pains
print(total_a_payer)
```

```sortie
6
```

Le résultat est le même, mais ce programme se lit comme une phrase.

Quelques règles, valables presque partout :

- pas d'espace dans un nom : `prix du pain` est interdit ;
- on relie les mots par `_` en Python, Ruby et Rust (`prix_du_pain`), et
  par une majuscule en JavaScript (`prixDuPain`) ;
- un nom ne commence pas par un chiffre : `2prix` est interdit ;
- les majuscules comptent : `Age` et `age` sont deux noms différents ;
- évitez les accents : `prenom` plutôt que `prénom`.

> **Astuce :** si vous hésitez sur un nom, demandez-vous « qu'y a-t-il
> dans la boîte ? » et écrivez la réponse.

## Le même programme dans quatre langages

Voici un petit programme : un panier, un prix, une quantité qui change.

En Python :

```python
prix = 4
quantite = 3
print("Total :", prix * quantite)
quantite = quantite + 2
print("Total :", prix * quantite)
```

```sortie
Total : 12
Total : 20
```

En JavaScript, `let` crée une variable qui pourra changer ; `const`, une
variable qui ne changera jamais :

```javascript
const prix = 4;
let quantite = 3;
console.log("Total :", prix * quantite);
quantite = quantite + 2;
console.log("Total :", prix * quantite);
```

```sortie
Total : 12
Total : 20
```

En Ruby, comme en Python, pas de mot en plus :

```ruby
prix = 4
quantite = 3
puts "Total : #{prix * quantite}"
quantite = quantite + 2
puts "Total : #{prix * quantite}"
```

```sortie
Total : 12
Total : 20
```

En Rust, `let` crée la variable. Par défaut, elle ne peut **pas** changer.
Il faut écrire `let mut` (pour « mutable », qui peut changer ; voir
[mutable](glossaire.md#mutable)). Sans `mut`, Rust refuse de compiler et
propose lui-même d'ajouter `mut` :

```rust
let prix = 4;
let mut quantite = 3;
println!("Total : {}", prix * quantite);
quantite = quantite + 2;
println!("Total : {}", prix * quantite);
```

```sortie
Total : 12
Total : 20
```

| | Python | JavaScript | Ruby | Rust |
|---|---|---|---|---|
| Créer | `x = 1` | `let x = 1;` | `x = 1` | `let mut x = 1;` |
| Ne changera pas | – | `const x = 1;` | – | `let x = 1;` |
| Changer | `x = 2` | `x = 2;` | `x = 2` | `x = 2;` |
| Afficher | `print(x)` | `console.log(x);` | `puts x` | `println!("{x}");` |

> **À retenir :** les mots changent d'un langage à l'autre, l'idée reste la
> même : un nom, une valeur, et `=` pour ranger.

## Erreurs fréquentes

### Utiliser une variable avant de la créer

L'ordinateur lit de haut en bas. À la ligne 1, `age` n'existe pas encore.

```python norun
print(age)
age = 30
```

```text
Traceback (most recent call last):
  File "age.py", line 1, in <module>
    print(age)
          ^^^
NameError: name 'age' is not defined
```

`NameError` veut dire « je ne connais pas ce nom ». En JavaScript :

```javascript norun
console.log(age);
let age = 30;
```

```text
console.log(age);
            ^

ReferenceError: Cannot access 'age' before initialization
```

La correction : créez la variable **avant** de l'utiliser. Échangez les
deux lignes : `age = 30` d'abord, `print(age)` ensuite.

### Une faute de frappe dans le nom

Pour l'ordinateur, `prenon` et `prenom` sont deux noms différents.

```python norun
prenom = "Ada"
print(prenon)
```

```text
Traceback (most recent call last):
  File "prenom.py", line 2, in <module>
    print(prenon)
          ^^^^^^
NameError: name 'prenon' is not defined. Did you mean: 'prenom'?
```

Python vous aide : « Did you mean: 'prenom'? » veut dire « vouliez-vous
dire `prenom` ? ». Ruby fait de même.

La correction : écrivez le nom exactement pareil partout, majuscules
comprises. Pour lire ces messages, voyez [lire une erreur](lire-une-erreur.md).

## Testez-vous

**1.** Que vaut `n` à la fin ?

```python
n = 5
n = n * 2
n = n - 1
print(n)
```

> **Réponse**
>
> 9. Étape 1 : `n` vaut 5. Étape 2 : `5 * 2`, donc 10. Étape 3 : `10 - 1`,
> donc 9.

**2.** Pourquoi `x = y` puis `y = x` n'échange-t-il pas deux valeurs ?

> **Réponse**
>
> Parce qu'après `x = y`, l'ancienne valeur de `x` est perdue : les deux
> variables contiennent la même chose. Il faut d'abord la garder dans une
> troisième variable, `temp`.

**3.** Lequel de ces noms est le meilleur pour un prix en euros :
`p`, `prix euros`, `prix_en_euros` ?

> **Réponse**
>
> `prix_en_euros`. `p` ne dit rien, et `prix euros` contient une espace,
> ce qui est interdit.

## En résumé

- Une variable est un nom attaché à une valeur : une boîte avec une étiquette.
- `=` range la valeur de droite sous le nom de gauche ; ce n'est pas « égal ».
- L'ordinateur lit les lignes de haut en bas : une variable vaut sa dernière valeur rangée.
- Pour suivre un programme, faites un tableau étape par étape.
- Pour échanger deux valeurs, gardez l'une d'elles dans une variable temporaire.
- Choisissez des noms qui disent ce que contient la boîte.

Suite : [Vrai, faux et les conditions](logique.md)
