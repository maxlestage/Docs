# Premiers pas

Cette page montre comment ranger des valeurs dans des variables, quels sont les types de base de Go et comment afficher proprement avec `fmt`.

> **Objectifs :**
>
> - vous saurez créer une variable avec `var` ou `:=` ;
> - vous connaîtrez les types de base et leur valeur zéro ;
> - vous saurez créer des constantes, et des suites de constantes avec `iota` ;
> - vous saurez afficher des valeurs avec `fmt.Println` et `fmt.Printf` ;
> - vous saurez pourquoi certains noms commencent par une majuscule.

## Les variables

Une [variable](../debuter/glossaire.md#variable) est une boîte avec une étiquette. Vous
rangez une valeur dedans, et vous la retrouvez grâce à son nom.

En Go, la façon la plus complète de créer une variable utilise le mot `var` :

```go
package main

import "fmt"

func main() {
	var prenom string = "Alice"
	var age int = 12
	fmt.Println(prenom, "a", age, "ans")
}
```

```sortie
Alice a 12 ans
```

Lisez `var age int = 12` ainsi : « crée une variable `age`, de type `int` (un entier),
qui vaut 12 ».

Le [type](../debuter/glossaire.md#type) dit quelle sorte de valeur la boîte contient :
un nombre, un texte, un vrai/faux… En Go, une variable garde **toujours** le même type.

### Changer la valeur

Le signe `=` range une nouvelle valeur dans une variable qui existe déjà.

```go
package main

import "fmt"

func main() {
	var score int = 10
	fmt.Println(score)
	score = 25
	fmt.Println(score)
	score = score + 5
	fmt.Println(score)
}
```

```sortie
10
25
30
```

`score = score + 5` se lit de droite à gauche : Go calcule `score + 5` (25 + 5), puis
range le résultat dans `score`.

## La déclaration courte :=

Écrire le type à chaque fois est long. Si vous donnez une valeur, Go **devine** le type
tout seul. Dans une fonction, on utilise alors `:=` :

```go
package main

import "fmt"

func main() {
	prenom := "Bob"
	age := 9
	taille := 1.32
	fmt.Println(prenom, age, taille)
}
```

```sortie
Bob 9 1.32
```

- `prenom := "Bob"` crée une variable de type `string` ;
- `age := 9` crée un `int` ;
- `taille := 1.32` crée un `float64` (un nombre à virgule).

> **À retenir :** `:=` **crée** une variable. `=` **change** une variable qui existe
> déjà. C'est la différence la plus importante de ce chapitre.

`:=` ne marche qu'à l'intérieur d'une fonction. En dehors, au niveau du fichier,
utilisez `var`.

### Plusieurs variables d'un coup

```go
package main

import "fmt"

func main() {
	x, y := 3, 4
	fmt.Println(x, y)
	x, y = y, x
	fmt.Println(x, y)
}
```

```sortie
3 4
4 3
```

La ligne `x, y = y, x` échange les deux valeurs, sans variable intermédiaire.

## Les valeurs zéro

Si vous créez une variable avec `var` sans lui donner de valeur, elle n'est pas vide :
elle reçoit la **valeur zéro** de son type.

```go
package main

import "fmt"

func main() {
	var n int
	var prix float64
	var actif bool
	var nom string
	fmt.Println(n, prix, actif)
	fmt.Printf("[%s]\n", nom)
}
```

```sortie
0 0 false
[]
```

| Type | Valeur zéro |
|------|-------------|
| `int`, `float64` | `0` |
| `bool` | `false` |
| `string` | `""` (texte vide) |

> **Note :** en Go, une variable a toujours une valeur. Il n'existe pas de variable
> « non initialisée » qui contiendrait n'importe quoi.

## Les types de base

| Type | Ce qu'il contient | Exemple |
|------|-------------------|---------|
| `int` | un [entier](../debuter/glossaire.md#entier) | `42`, `-7` |
| `float64` | un [nombre à virgule](../debuter/glossaire.md#flottant) | `3.14` |
| `string` | du texte | `"bonjour"` |
| `bool` | un [booléen](../debuter/glossaire.md#booleen) : vrai ou faux | `true`, `false` |
| `rune` | un seul caractère | `'é'` |
| `byte` | un [octet](../debuter/glossaire.md#octet) (de 0 à 255) | `65` |

```go
package main

import "fmt"

func main() {
	note := 15
	moyenne := 13.5
	ville := "Lyon"
	recu := note >= 10
	initiale := 'L'
	fmt.Println(note, moyenne, ville, recu, initiale)
}
```

```sortie
15 13.5 Lyon true 76
```

Surprise : `initiale` s'affiche `76`. Une `rune` est en fait un nombre, le code
[Unicode](../debuter/glossaire.md#unicode) du caractère. Pour voir la lettre, il faut
`Printf` et `%c`, comme nous allons le voir.

## Les constantes

Une [constante](../debuter/glossaire.md#constante) est une valeur qui ne change jamais.
On la crée avec `const`.

```go
package main

import "fmt"

const TauxTVA = 0.2

func main() {
	const pi = 3.14159
	prix := 50.0
	fmt.Println(prix * (1 + TauxTVA))
	fmt.Println(pi * 2)
}
```

```sortie
60
6.28318
```

Une constante peut être déclarée en dehors de `main`, au niveau du fichier : elle est
alors visible partout. Essayer de la modifier (`pi = 3`) est une erreur de compilation.

### Des suites de constantes avec iota

Pour numéroter des constantes, Go propose `iota`. Dans un bloc `const ( ... )`, `iota`
vaut 0 sur la première ligne, 1 sur la suivante, et ainsi de suite.

```go
package main

import "fmt"

const (
	Lundi = iota
	Mardi
	Mercredi
	Jeudi
)

func main() {
	fmt.Println(Lundi, Mardi, Mercredi, Jeudi)
}
```

```sortie
0 1 2 3
```

Les lignes sans valeur répètent la formule de la ligne d'au-dessus, avec `iota` qui
augmente. On peut donc faire un calcul :

```go
package main

import "fmt"

const (
	Bronze = (iota + 1) * 100
	Argent
	Or
)

func main() {
	fmt.Println(Bronze, Argent, Or)
}
```

```sortie
100 200 300
```

## Afficher avec fmt

### Println

`fmt.Println` affiche ses valeurs séparées par des espaces, puis passe à la ligne.
`fmt.Print` fait pareil sans passer à la ligne.

### Printf et les verbes

`fmt.Printf` (*print formatted*, « afficher avec un format ») prend un **modèle** de
texte. Chaque `%` suivi d'une lettre est un trou, appelé **verbe**, remplacé par une
valeur. Ajoutez `\n` à la fin pour passer à la ligne.

```go
package main

import "fmt"

func main() {
	nom := "Alice"
	age := 12
	fmt.Printf("%s a %d ans.\n", nom, age)
}
```

```sortie
Alice a 12 ans.
```

Les verbes les plus utiles :

| Verbe | Affiche | Exemple de résultat |
|-------|---------|---------------------|
| `%d` | un entier | `42` |
| `%s` | un texte | `Alice` |
| `%q` | un texte entre guillemets | `"Alice"` |
| `%v` | n'importe quelle valeur, au format par défaut | `[1 2 3]` |
| `%+v` | comme `%v`, avec le nom des champs d'une structure | `{Nom:Alice Age:12}` |
| `%T` | le type de la valeur | `float64` |
| `%.2f` | un nombre à virgule, avec 2 décimales | `3.14` |
| `%c` | un caractère (une rune) | `L` |
| `%%` | le signe `%` lui-même | `%` |

```go
package main

import "fmt"

type Eleve struct {
	Nom string
	Age int
}

func main() {
	pi := 3.14159
	fmt.Printf("%.2f\n", pi)
	fmt.Printf("%T et %T\n", pi, 42)
	fmt.Printf("%q\n", "salut")
	fmt.Printf("%c\n", 'L')
	fmt.Printf("%v\n", []int{1, 2, 3})
	e := Eleve{Nom: "Alice", Age: 12}
	fmt.Printf("%v\n", e)
	fmt.Printf("%+v\n", e)
	fmt.Printf("Réduction : 20%%\n")
}
```

```sortie
3.14
float64 et int
"salut"
L
[1 2 3]
{Alice 12}
{Nom:Alice Age:12}
Réduction : 20%
```

> **Astuce :** perdu ? Utilisez `%v` : il marche avec tout. Et `%T` vous dit le type
> exact d'une valeur, très pratique pour comprendre ce qui se passe.

`Eleve` est une **structure** : une valeur qui regroupe plusieurs champs. Elles sont
expliquées dans [les types](types.md).

## Les commentaires

Un [commentaire](../debuter/glossaire.md#commentaire) est une note pour les humains :
Go l'ignore.

```go
package main

import "fmt"

// main est le point de départ du programme.
func main() {
	fmt.Println("visible") // ceci aussi est un commentaire
	/* un commentaire
	   sur plusieurs lignes */
}
```

```sortie
visible
```

- `//` commente jusqu'à la fin de la ligne ;
- `/* ... */` commente tout ce qui est entre les deux.

> **Astuce :** par convention, un commentaire juste au-dessus d'une fonction commence
> par son nom : `// main est...`. Les outils de documentation de Go s'en servent.

## Bien nommer

Un nom de variable commence par une lettre, puis des lettres, des chiffres ou `_`.
En Go, on colle les mots avec des majuscules : `prixTotal`, `nombreDEleves`. C'est le
*camelCase* (« la casse du chameau », à cause des bosses).

```go
package main

import "fmt"

func main() {
	prixTotal := 42
	nbEleves := 25
	fmt.Println(prixTotal, nbEleves)
}
```

```sortie
42 25
```

### Majuscule = visible depuis un autre paquet

En Go, la **première lettre** d'un nom a un sens :

- un nom qui commence par une **majuscule** est **exporté** : les autres paquets peuvent
  l'utiliser ;
- un nom qui commence par une **minuscule** reste **privé** à son paquet.

C'est pour cela qu'on écrit `fmt.Println` avec un `P` majuscule : `Println` est exporté
par le paquet `fmt`. Une fonction `fmt.println` (minuscule), même si elle existait,
serait cachée.

> **À retenir :** pas de mot-clé `public` ou `private` en Go. Seule la majuscule compte.

## Une variable inutilisée est une erreur

Le compilateur de Go refuse une variable créée dans une fonction mais jamais utilisée.
C'est voulu : une variable oubliée cache souvent un bug.

Si vous voulez vraiment ignorer une valeur, utilisez le **tiret bas** `_`, la « poubelle »
de Go :

```go
package main

import "fmt"

func main() {
	a, _ := 10, 20
	fmt.Println(a)
}
```

```sortie
10
```

La valeur `20` est jetée : `_` n'est pas une vraie variable.

## Erreurs fréquentes

### Créer une variable sans l'utiliser

```go norun
package main

import "fmt"

func main() {
	age := 12
	nom := "Alice"
	fmt.Println(nom)
}
```

```text
# command-line-arguments
./main.go:6:2: declared and not used: age
```

`age` est déclarée (*declared*) mais jamais utilisée (*not used*). Utilisez-la, ou
supprimez la ligne.

### Utiliser := deux fois pour la même variable

```go norun
package main

import "fmt"

func main() {
	score := 10
	score := 20
	fmt.Println(score)
}
```

```text
# command-line-arguments
./main.go:7:8: no new variables on left side of :=
```

`score` existe déjà : `:=` n'a rien de nouveau à créer. Pour changer la valeur, écrivez
`score = 20`, avec `=`.

### Utiliser := en dehors d'une fonction

```go norun
package main

import "fmt"

ville := "Paris"

func main() {
	fmt.Println(ville)
}
```

```text
# command-line-arguments
./main.go:5:1: syntax error: non-declaration statement outside function body
```

Hors d'une fonction, `:=` est interdit. Écrivez `var ville = "Paris"`.

### Mettre une valeur du mauvais type

```go norun
package main

import "fmt"

func main() {
	var age int
	age = "douze"
	fmt.Println(age)
}
```

```text
# command-line-arguments
./main.go:7:8: cannot use "douze" (untyped string constant) as int value in assignment
```

`age` est un `int` pour toujours : on ne peut pas y ranger du texte. Écrivez `age = 12`.

## Testez-vous

**Question 1 :** qu'affiche ce programme ?

```go norun
package main

import "fmt"

func main() {
	var total int
	var message string
	fmt.Printf("%d [%s]\n", total, message)
}
```

> **Réponse**
>
> `0 []`. Les variables reçoivent leur valeur zéro : `0` pour un `int`, le texte vide
> pour un `string`.

**Question 2 :** avec `Printf`, affichez `Prix : 12.50 €` à partir de la variable
`prix := 12.5`.

> **Réponse**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	prix := 12.5
> 	fmt.Printf("Prix : %.2f €\n", prix)
> }
> ```
>
> ```sortie
> Prix : 12.50 €
> ```

**Question 3 :** quelle est la différence entre `x := 5` et `x = 5` ?

> **Réponse**
>
> `x := 5` **crée** une nouvelle variable `x`. `x = 5` change la valeur d'une variable
> `x` qui existe déjà.

## En résumé

- `var nom type = valeur` crée une variable ; `nom := valeur` fait de même dans une
  fonction, en devinant le type.
- Une variable sans valeur reçoit la valeur zéro de son type (`0`, `""`, `false`).
- `const` crée une valeur fixe ; `iota` numérote une suite de constantes.
- `fmt.Printf` remplace les verbes `%d`, `%s`, `%v`, `%T`, `%.2f`… par des valeurs.
- Un nom en majuscule est exporté ; une variable inutilisée empêche de compiler.

Suite : [nombres et calculs](nombres.md).
