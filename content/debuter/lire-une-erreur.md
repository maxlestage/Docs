# Lire un message d'erreur sans paniquer

Un message d'erreur n'est pas une punition, c'est une aide : cette page vous apprend à y trouver trois informations, dans six langages, et à savoir quoi faire ensuite.

> **Objectifs :**
>
> - voir un message d'erreur comme une aide ;
> - trouver dans un message : l'endroit, la sorte d'erreur et la phrase qui explique ;
> - lire une trace de bas en haut ;
> - distinguer une erreur de syntaxe, une erreur d'exécution et un mauvais résultat ;
> - savoir quoi faire après avoir lu le message.

## Une erreur est une aide

Un message rouge apparaît. Votre cœur bat plus vite. C'est normal.

Mais regardez ce qui se passe vraiment : l'ordinateur s'est arrêté, et il
vous **explique pourquoi**. C'est comme un GPS qui dit « demi-tour possible » :
il ne vous gronde pas, il vous aide à retrouver la route.

Les programmeurs expérimentés voient des erreurs **toute la journée**. La
différence, c'est qu'ils les lisent calmement. Une
[erreur](glossaire.md#erreur) ne dit rien de vous. Elle dit quelque chose de
**une ligne** de votre code.

> **À retenir :** un message d'erreur est un indice gratuit. Lisez-le
> avant de toucher au code.

## Les trois choses à trouver

Tous les messages, dans tous les langages, contiennent les mêmes trois
informations. Cherchez-les comme on cherche le nom, la date et le lieu sur
une invitation :

```text
  1. OÙ ?            le fichier et la ligne   →  bonjour.py, line 2
  2. QUELLE SORTE ?  le nom de l'erreur       →  NameError
  3. POURQUOI ?      la phrase qui explique   →  name 'prenon' is not defined
```

Le reste (les `~~~`, les `^^^`, les chemins) aide à préciser. Commencez
par ces trois-là.

Voici les mêmes trois choses dans six langages. Les fichiers sont dans le
dossier `/home/ada`.

### En Python

```python norun
prenom = "Ada"
print("Bonjour " + prenon)
```

```text
Traceback (most recent call last):
  File "/home/ada/bonjour.py", line 2, in <module>
    print("Bonjour " + prenon)
                       ^^^^^^
NameError: name 'prenon' is not defined. Did you mean: 'prenom'?
```

| Question | Réponse dans le message |
|---|---|
| Où ? | `bonjour.py`, `line 2` ; les `^^^^^^` soulignent `prenon` |
| Quelle sorte ? | `NameError` : un nom inconnu |
| Pourquoi ? | `name 'prenon' is not defined` : ce nom n'existe pas |

Python propose même la correction : `Did you mean: 'prenom'?` (« vouliez-vous
dire `prenom` ? »). Une faute de frappe, rien de plus.

### En JavaScript

```javascript norun
const prenom = "Ada";
console.log("Bonjour " + prenon);
```

```text
file:///home/ada/prenom.mjs:2
console.log("Bonjour " + prenon);
                         ^

ReferenceError: prenon is not defined
    at file:///home/ada/prenom.mjs:2:26
```

- **Où ?** `prenom.mjs:2`, la ligne 2. Le `^` montre l'endroit exact.
- **Quelle sorte ?** `ReferenceError` : une référence (un nom) introuvable.
- **Pourquoi ?** `prenon is not defined` : `prenon` n'est pas défini.

Node ajoute souvent d'autres lignes `at …` qui parlent de Node lui-même
(`node:internal/…`). Vous pouvez les ignorer.

### En Ruby

```ruby norun
stock = { "pomme" => 3, "poire" => 5 }
quantite = stock["banane"]
puts quantite + 1
```

```text
panier.rb:3:in `<main>': undefined method `+' for nil (NoMethodError)

puts quantite + 1
              ^
```

- **Où ?** `panier.rb:3`, la ligne 3.
- **Quelle sorte ?** `NoMethodError` : une opération qui n'existe pas.
- **Pourquoi ?** `undefined method '+' for nil` : on fait `+` sur `nil`,
  c'est-à-dire sur « rien ».

La ligne 3 est juste. Le vrai problème est à la ligne 2 : il n'y a pas de
banane dans le stock, donc `quantite` vaut `nil`.

### En Rust

```rust norun
fn main() {
    let prix = 12;
    println!("{}", prix + quantite);
}
```

```text
error[E0425]: cannot find value `quantite` in this scope
 --> total.rs:3:27
  |
3 |     println!("{}", prix + quantite);
  |                           ^^^^^^^^ not found in this scope

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0425`.
```

- **Où ?** `total.rs:3:27` : ligne 3, colonne 27 (le 27ᵉ caractère).
- **Quelle sorte ?** `E0425`, un code à chercher avec `rustc --explain E0425`.
- **Pourquoi ?** `cannot find value 'quantite'` : la valeur `quantite`
  est introuvable. Il faut d'abord écrire `let quantite = 3;`.

### En Go

```go norun
package main

import "fmt"

func main() {
	nom := "Ada"
	age := 36
	fmt.Println(nom)
}
```

```text
# command-line-arguments
./main.go:7:2: declared and not used: age
```

- **Où ?** `main.go:7:2` : ligne 7, colonne 2.
- **Quelle sorte ?** Pas de nom ici : c'est une erreur de compilation.
- **Pourquoi ?** `declared and not used: age` : la variable `age` est créée
  mais jamais utilisée. Go refuse. Supprimez-la, ou affichez-la.

### En Rythme

```rythme norun
fn double(x) = x * 2

print(double("dix"))
```

```text
erreur[E005]: impossible d'utiliser `*` avec Text et Int
 --> double.ry:3:7
  |
3 | print(double("dix"))
  |       ^^^^^^^^^^^^^ `double` s'arrête sur cette erreur, à chaque fois
  |
1 | fn double(x) = x * 2
  |                  - dans `double` : Text "dix" * Int 2
  |
  = note: d'après la façon dont `double` utilise `x`, elle a besoin de : `fn double(x: Int|Float) -> Int|Float`
  = aide: pour répéter un texte ou une liste, utilisez `repeat(x, n)`
  = en savoir plus : `rythme explain E005`
```

- **Où ?** `double.ry:3:7`, puis la ligne 1, dans la fonction.
- **Quelle sorte ?** `E005`, expliquée par `rythme explain E005`.
- **Pourquoi ?** On multiplie un texte (`"dix"`) par un nombre. Il faut
  écrire `double(10)`.

> **À retenir :** où, quelle sorte, pourquoi. Trouvez ces trois choses
> avant de chercher plus loin.

## Lire une trace de bas en haut

Quand une fonction appelle une autre fonction, le message montre **tout le
chemin**. On l'appelle une **trace** (*traceback* en Python).

```python norun
def moyenne(notes):
    return sum(notes) / len(notes)

def bulletin(nom, notes):
    print(nom, ":", moyenne(notes))

bulletin("Ada", [12, 15])
bulletin("Léo", [])
```

```text
Ada : 13.5
Traceback (most recent call last):
  File "/home/ada/moyenne.py", line 8, in <module>
    bulletin("Léo", [])
    ~~~~~~~~^^^^^^^^^^^
  File "/home/ada/moyenne.py", line 5, in bulletin
    print(nom, ":", moyenne(notes))
                    ~~~~~~~^^^^^^^
  File "/home/ada/moyenne.py", line 2, in moyenne
    return sum(notes) / len(notes)
           ~~~~~~~~~~~^~~~~~~~~~~~
ZeroDivisionError: division by zero
```

`most recent call last` veut dire « l'appel le plus récent en dernier ». La
ligne la plus importante est donc **tout en bas**. Remontez ensuite :

| Ordre de lecture | Ligne | Ce qu'elle dit |
|---|---|---|
| 1 | `ZeroDivisionError: division by zero` | on a divisé par zéro |
| 2 | `line 2, in moyenne` | dans `moyenne`, `len(notes)` valait 0 |
| 3 | `line 5, in bulletin` | `moyenne` a été appelée par `bulletin` |
| 4 | `line 8, in <module>` | tout a commencé par `bulletin("Léo", [])` |

Léo n'a aucune note : la liste est vide, sa longueur est 0. Comme une pile
d'assiettes, la dernière posée (en bas du message) est la première à
regarder.

Rust, Go et JavaScript font l'inverse : l'erreur est **en haut**. Dans le
doute, cherchez la ligne qui contient `Error`, `error` ou `erreur`.

## Trois moments où ça casse

### Avant de démarrer : l'erreur de syntaxe

La [syntaxe](glossaire.md#syntaxe), c'est la grammaire du langage. Si une
signe manque, rien ne démarre :

```python norun
age = 17
if age >= 18
    print("majeur")
```

```text
  File "/home/ada/age.py", line 2
    if age >= 18
                ^
SyntaxError: expected ':'
```

`expected ':'` : Python attendait deux-points à la fin du `if`.

### Pendant : l'erreur d'exécution

Le programme démarre, puis tombe sur une opération impossible. C'est
l'exemple de la moyenne de Léo : `Ada : 13.5` s'affiche, **puis** l'erreur
arrive.

### Sans message : le mauvais résultat

Le pire cas : tout marche, mais le résultat est faux. On veut le total de
trois prix :

```python
total = 0
for prix in [3, 4, 5]:
    total = prix
print(total)
```

```sortie
5
```

On attendait 12. Il n'y a aucun message : il faut afficher les valeurs pour
comprendre. Ici, il fallait `total = total + prix`.

> **À retenir :** syntaxe = ne démarre pas ; exécution = s'arrête en route ;
> mauvais résultat = aucun message, à vous de chercher.

## Que faire après avoir lu le message

1. **Relisez la ligne indiquée, et celle d'avant.** L'ordinateur signale
   l'endroit où il s'est perdu, pas toujours l'endroit de la faute.
2. **Cherchez le message en ligne.** Copiez la partie générale, sans vos
   noms à vous : `ZeroDivisionError: division by zero`, pas `moyenne.py`.
3. **Affichez les valeurs.** Un `print(notes)` juste avant la ligne fautive
   montre `[]`, et tout s'éclaire.
4. **Corrigez la première erreur**, relancez, puis passez à la suivante.

Les techniques pour chercher un [bug](glossaire.md#bug) sont dans [Trouver
et corriger ses erreurs](deboguer.md).

## Erreurs fréquentes

### Chercher seulement sur la ligne indiquée

```javascript norun
const notes = [12, 15
console.log(notes);
```

```text
file:///home/ada/notes.mjs:2
console.log(notes);
^^^^^^^

SyntaxError: Unexpected identifier 'console'
```

Le message montre la ligne 2, qui est juste. C'est la ligne 1 qui oublie
le `]`. JavaScript ne s'en aperçoit qu'en voyant `console`, qu'il
n'attendait pas (*unexpected*). Correction : `const notes = [12, 15];`.

### Ne lire que le haut du message Python

```python norun
def trouver(nom):
    print("recherche de", nom)

resultat = trouver("Ada")
print(resultat.upper())
```

```text
recherche de Ada
Traceback (most recent call last):
  File "/home/ada/trouver.py", line 5, in <module>
    print(resultat.upper())
          ^^^^^^^^^^^^^^
AttributeError: 'NoneType' object has no attribute 'upper'
```

`Traceback` en haut ne dit rien. L'explication est **en bas** : `resultat`
vaut `None` (« rien »), car `trouver` ne renvoie aucune valeur. Correction :
ajoutez `return nom` à la fin de la fonction.

## Testez-vous

**1.** Quelles sont les trois choses à chercher dans un message d'erreur ?

> **Réponse**
>
> Où (le fichier et la ligne), quelle sorte (le nom de l'erreur, comme
> `TypeError`), et pourquoi (la phrase qui explique).

**2.** Dans une trace Python avec plusieurs `File …`, par où commencer ?

> **Réponse**
>
> Par le bas : la dernière ligne donne l'erreur, la ligne `File` juste
> au-dessus donne l'endroit où elle s'est produite. On remonte ensuite pour
> voir qui a appelé quoi.

**3.** Le message indique la ligne 12, mais elle vous semble parfaite. Que
faites-vous ?

> **Réponse**
>
> Vous regardez la ligne 11, et celles d'avant : une parenthèse, un
> crochet ou un deux-points oublié plus haut est souvent la vraie cause.

## En résumé

- Un message d'erreur est une aide : il dit où, quelle sorte et pourquoi.
- En Python, lisez la trace de bas en haut ; en Rust, Go et JavaScript, l'erreur est en haut.
- Erreur de syntaxe : ne démarre pas ; d'exécution : s'arrête en route ; mauvais résultat : aucun message.
- Relisez la ligne indiquée et celle d'avant.
- Cherchez la partie générale du message en ligne, et affichez les valeurs.

Suite : [Quand on bloque : une méthode pas à pas](bloque.md).
