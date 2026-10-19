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

Un programme a besoin de se souvenir de choses : un prénom, un âge, un score. Comme
un post-it sur le frigo, la variable garde l'information pour plus tard.

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

Ligne par ligne :

- `package main`, `import "fmt"` et `func main()` forment le cadre vu dans
  l'introduction : le programme commence dans `main` ;
- `var prenom string = "Alice"` crée la boîte `prenom` et y range le texte `"Alice"` ;
- `var age int = 12` crée la boîte `age` et y range le nombre 12 ;
- `fmt.Println` affiche ses valeurs dans l'ordre, séparées par des espaces : le
  contenu de `prenom`, le mot « a », le contenu de `age`, puis le mot « ans ».

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

**À vous :** écrivez un programme qui crée une variable `pommes` de type `int` qui
vaut 3 et l'affiche. Ajoutez-lui ensuite 4, puis affichez-la de nouveau.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	var pommes int = 3
> 	fmt.Println(pommes)
> 	pommes = pommes + 4
> 	fmt.Println(pommes)
> }
> ```
>
> ```sortie
> 3
> 7
> ```

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

**À vous :** avec `:=`, créez une variable `ville` qui vaut `"Nice"` et une variable
`habitants` qui vaut `340000`. Avec un seul `fmt.Println`, affichez
`Nice compte 340000 habitants`.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	ville := "Nice"
> 	habitants := 340000
> 	fmt.Println(ville, "compte", habitants, "habitants")
> }
> ```
>
> ```sortie
> Nice compte 340000 habitants
> ```

### Plusieurs variables d'un coup

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

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

Parfois, vous créez une variable avant de connaître sa valeur : un compteur qui
partira de zéro, un nom que vous remplirez plus tard. Go la remplit alors pour vous.

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

Ligne par ligne :

- les quatre `var` créent quatre variables sans valeur : Go range dans chacune la
  valeur zéro de son type ;
- `fmt.Println` affiche `n`, `prix` et `actif` : `0`, `0` et `false` ;
- un texte vide ne se voit pas à l'écran. `Printf` (expliqué plus bas) l'entoure donc
  de crochets : `[]` montre qu'il n'y a rien entre les deux.

| Type | Valeur zéro |
|------|-------------|
| `int`, `float64` | `0` |
| `bool` | `false` |
| `string` | `""` (texte vide) |

> **Note :** en Go, une variable a toujours une valeur. Il n'existe pas de variable
> « non initialisée » qui contiendrait n'importe quoi.

## Les types de base

Go doit savoir quelle sorte de valeur il range, pour savoir ce qu'il peut en faire :
on peut diviser un prix par 2, pas un prénom. C'est comme une cuisine bien rangée,
avec les couverts dans un tiroir et les assiettes dans un autre.

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

Ligne par ligne :

- `note := 15` crée un `int`. `moyenne := 13.5` crée un `float64`, à cause de la
  virgule (un point en Go) ;
- `ville := "Lyon"` crée un `string` : le texte est entre guillemets doubles ;
- `recu := note >= 10` pose une question : « 15 est-il plus grand ou égal à 10 ? ».
  La réponse, `true`, est rangée dans `recu`, qui est donc un `bool` ;
- `initiale := 'L'` crée une `rune` : un seul caractère, entre apostrophes.

Surprise : `initiale` s'affiche `76`. Une `rune` est en fait un nombre, le code
[Unicode](../debuter/glossaire.md#unicode) du caractère. Pour voir la lettre, il faut
`Printf` et `%c`, comme nous allons le voir.

## Les constantes

Une [constante](../debuter/glossaire.md#constante) est une valeur qui ne change jamais.
On la crée avec `const`.
Elle sert pour les valeurs fixes de votre programme : un taux de TVA, le nombre de
jours dans une semaine. C'est un prix gravé sur une plaque, pas écrit au crayon.

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

Ligne par ligne :

- `const TauxTVA = 0.2` est écrit hors de `main`, au niveau du fichier ;
- `const pi = 3.14159` crée une constante à l'intérieur de `main` ;
- `prix := 50.0` est une variable ordinaire ;
- `prix * (1 + TauxTVA)` calcule 50 × 1,2. Go affiche `60`, sans les zéros inutiles.

Une constante peut être déclarée en dehors de `main`, au niveau du fichier : elle est
alors visible partout. Essayer de la modifier (`pi = 3`) est une erreur de compilation.

**À vous :** créez une constante `JoursParSemaine` qui vaut 7, puis une variable
`semaines` qui vaut 3. Affichez le nombre de jours dans ces 3 semaines.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> const JoursParSemaine = 7
>
> func main() {
> 	semaines := 3
> 	fmt.Println(semaines * JoursParSemaine)
> }
> ```
>
> ```sortie
> 21
> ```

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

Afficher, c'est la façon dont votre programme vous parle : il montre un résultat ou
un message. `fmt` vous aide à le présenter proprement, comme un ticket de caisse
bien aligné.

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

Ligne par ligne :

- `nom` vaut `"Alice"` et `age` vaut `12` ;
- dans le modèle `"%s a %d ans.\n"`, Go remplace le premier verbe, `%s`, par la
  première valeur qui suit le modèle : `nom` ;
- puis il remplace le deuxième verbe, `%d`, par la deuxième valeur : `age` ;
- `\n` ne s'affiche pas : il fait passer à la ligne.

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

**À vous :** créez une variable `livre` qui vaut `"Le Petit Prince"` et une variable
`pages` qui vaut 96. Avec `Printf`, affichez `Le Petit Prince compte 96
pages.`, puis, sur une autre ligne, le type de `pages` avec `%T`.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	livre := "Le Petit Prince"
> 	pages := 96
> 	fmt.Printf("%s compte %d pages.\n", livre, pages)
> 	fmt.Printf("%T\n", pages)
> }
> ```
>
> ```sortie
> Le Petit Prince compte 96 pages.
> int
> ```

## Les commentaires

Un [commentaire](../debuter/glossaire.md#commentaire) est une note pour les humains :
Go l'ignore.
Il sert à expliquer **pourquoi** le code est écrit ainsi, à la personne qui le
relira. Souvent, ce sera vous, un mois plus tard. C'est comme une note dans la marge
d'une recette.

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

Un bon nom dit ce que contient la variable. `prixTotal` se comprend tout de suite ;
`x` ou `pt` obligent à chercher. Vous écrivez le code une fois, mais vous le relisez
souvent.

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
