# Les fonctions

Cette page explique comment écrire des fonctions en Go avec `func`, leurs paramètres, leurs valeurs de retour, et des outils plus avancés comme les closures et `defer`.

> **Objectifs :**
>
> - vous saurez écrire une fonction avec `func` et l'appeler ;
> - vous saurez donner des paramètres typés et renvoyer une ou plusieurs valeurs ;
> - vous saurez écrire une fonction qui accepte un nombre variable d'arguments ;
> - vous saurez ranger une fonction dans une variable et créer une closure ;
> - vous saurez repousser une action à la fin avec `defer`.

## Définir une fonction

Une [fonction](../debuter/glossaire.md#fonction) est un morceau de code nommé, que vous
pouvez réutiliser. C'est comme une recette : vous l'écrivez une fois, puis vous la
suivez autant de fois que vous voulez. Elle commence par le mot `func`.

```go
package main

import "fmt"

func saluer() {
	fmt.Println("Bonjour !")
}

func main() {
	saluer()
	saluer()
}
```

```sortie
Bonjour !
Bonjour !
```

Écrire `func saluer()` ne lance rien : cela enregistre la recette. Écrire `saluer()`
**appelle** la fonction, c'est-à-dire l'exécute.

> **Note :** les fonctions se définissent en dehors de `main`, au niveau du fichier.
> L'ordre n'a pas d'importance : `main` peut appeler une fonction écrite plus bas.

## Les paramètres

Les [paramètres](../debuter/glossaire.md#parametre) sont les valeurs que la fonction
reçoit. Ce sont des cases vides, remplies au moment de l'appel. Les valeurs données à
l'appel s'appellent les [arguments](../debuter/glossaire.md#argument).

En Go, chaque paramètre a un **type**, écrit **après** son nom.

```go
package main

import "fmt"

func saluer(prenom string) {
	fmt.Println("Bonjour,", prenom, "!")
}

func main() {
	saluer("Alice")
	saluer("Bob")
}
```

```sortie
Bonjour, Alice !
Bonjour, Bob !
```

Plusieurs paramètres se séparent par des virgules. S'ils ont le même type, vous pouvez
ne l'écrire qu'une fois, à la fin.

```go
package main

import "fmt"

func presenter(nom string, age int) {
	fmt.Printf("%s a %d ans.\n", nom, age)
}

func aire(largeur, hauteur int) {
	fmt.Println("Aire :", largeur*hauteur)
}

func main() {
	presenter("Alice", 12)
	aire(3, 4)
}
```

```sortie
Alice a 12 ans.
Aire : 12
```

`largeur, hauteur int` veut dire : `largeur` et `hauteur` sont deux `int`.

## Renvoyer une valeur

Pour qu'une fonction **donne** un résultat, indiquez son type après les parenthèses, et
utilisez `return`.

```go
package main

import "fmt"

func carre(x int) int {
	return x * x
}

func main() {
	fmt.Println(carre(4))
	resultat := carre(3) + 1
	fmt.Println(resultat)
}
```

```sortie
16
10
```

La valeur renvoyée remplace l'appel : `carre(3) + 1` devient `9 + 1`.

`return` arrête aussi la fonction tout de suite. On s'en sert pour sortir plus tôt :

```go
package main

import "fmt"

func mention(note int) string {
	if note >= 16 {
		return "très bien"
	}
	if note >= 10 {
		return "admis"
	}
	return "refusé"
}

func main() {
	fmt.Println(mention(17))
	fmt.Println(mention(11))
	fmt.Println(mention(4))
}
```

```sortie
très bien
admis
refusé
```

> **À retenir :** une fonction qui annonce un type de retour doit **toujours** finir par
> un `return`, quel que soit le chemin suivi. Sinon, le compilateur refuse.

## Plusieurs valeurs de retour

Une fonction Go peut renvoyer **plusieurs** valeurs. Écrivez leurs types entre
parenthèses.

```go
package main

import "fmt"

func diviser(a, b int) (int, int) {
	return a / b, a % b
}

func main() {
	quotient, reste := diviser(17, 5)
	fmt.Println(quotient, reste)

	q, _ := diviser(20, 3)
	fmt.Println(q)
}
```

```sortie
3 2
6
```

Le `_` jette la valeur dont on n'a pas besoin.

C'est ainsi que Go signale les erreurs : la dernière valeur renvoyée est souvent une
`error`, qui vaut `nil` si tout va bien.

```go
package main

import (
	"errors"
	"fmt"
)

func racineEntiere(n int) (int, error) {
	if n < 0 {
		return 0, errors.New("nombre négatif")
	}
	r := 0
	for (r+1)*(r+1) <= n {
		r++
	}
	return r, nil
}

func main() {
	r, err := racineEntiere(20)
	fmt.Println(r, err)
	r, err = racineEntiere(-4)
	fmt.Println(r, err)
}
```

```sortie
4 <nil>
0 nombre négatif
```

Les erreurs ont leur propre chapitre : [la gestion des erreurs](erreurs.md).

### Les résultats nommés

Vous pouvez donner un nom aux valeurs de retour. Ce sont alors des variables, créées
avec leur valeur zéro. Un `return` seul renvoie leurs valeurs actuelles.

```go
package main

import "fmt"

func minMax(nombres []int) (petit, grand int) {
	petit, grand = nombres[0], nombres[0]
	for _, n := range nombres {
		petit = min(petit, n)
		grand = max(grand, n)
	}
	return
}

func main() {
	p, g := minMax([]int{4, 9, 1, 7})
	fmt.Println(p, g)
}
```

```sortie
1 9
```

> **Astuce :** les noms aident à documenter ce que renvoie la fonction. Mais un `return`
> seul dans une longue fonction devient vite difficile à lire : gardez-le pour les
> fonctions courtes.

## Un nombre variable d'arguments

Avec `...` devant le type, le dernier paramètre accepte autant d'arguments que vous
voulez. Dans la fonction, il devient une slice. On parle de fonction **variadique**.

```go
package main

import "fmt"

func somme(nombres ...int) int {
	total := 0
	for _, n := range nombres {
		total += n
	}
	return total
}

func main() {
	fmt.Println(somme(1, 2))
	fmt.Println(somme(1, 2, 3, 4))
	fmt.Println(somme())

	notes := []int{12, 15, 9}
	fmt.Println(somme(notes...))
}
```

```sortie
3
10
0
36
```

Pour passer une slice existante, ajoutez `...` après elle : `somme(notes...)`.
`fmt.Println` est d'ailleurs variadique : c'est pour cela qu'elle accepte autant de
valeurs.

## Les fonctions sont des valeurs

En Go, une fonction est une valeur comme une autre. Vous pouvez la ranger dans une
variable, ou la passer en argument à une autre fonction.

```go
package main

import "fmt"

func double(x int) int {
	return x * 2
}

func appliquer(nombres []int, f func(int) int) []int {
	resultat := []int{}
	for _, n := range nombres {
		resultat = append(resultat, f(n))
	}
	return resultat
}

func main() {
	operation := double
	fmt.Println(operation(5))
	fmt.Println(appliquer([]int{1, 2, 3}, double))
}
```

```sortie
10
[2 4 6]
```

Le type `func(int) int` se lit : « une fonction qui prend un `int` et renvoie un
`int` ».

### Les fonctions anonymes

Une fonction peut être écrite sans nom, directement là où on en a besoin : c'est une
fonction **anonyme**.

```go
package main

import "fmt"

func main() {
	triple := func(x int) int {
		return x * 3
	}
	fmt.Println(triple(7))

	func() {
		fmt.Println("Exécutée tout de suite !")
	}()
}
```

```sortie
21
Exécutée tout de suite !
```

Les parenthèses `()` à la fin de la deuxième fonction l'appellent aussitôt.

## Les closures

Une fonction anonyme peut utiliser les variables qui l'entourent. Elle les **emporte**
avec elle, même quand elle sort de l'endroit où elle est née. On l'appelle alors une
[closure](../debuter/glossaire.md#closure) (« fermeture »).

```go
package main

import "fmt"

func compteur() func() int {
	n := 0
	return func() int {
		n++
		return n
	}
}

func main() {
	suivant := compteur()
	fmt.Println(suivant())
	fmt.Println(suivant())
	fmt.Println(suivant())

	autre := compteur()
	fmt.Println(autre())
}
```

```sortie
1
2
3
1
```

Imaginez un sac à dos : la fonction renvoyée garde `n` dans son sac. Chaque appel de
`suivant` augmente **son** `n`. `autre` a son propre sac, avec son propre `n`.

## defer : repousser à la fin

`defer` (« différer ») repousse un appel de fonction au moment où la fonction en cours
se termine.

```go
package main

import "fmt"

func travailler() {
	fmt.Println("Ouverture")
	defer fmt.Println("Fermeture")
	fmt.Println("Travail 1")
	fmt.Println("Travail 2")
}

func main() {
	travailler()
}
```

```sortie
Ouverture
Travail 1
Travail 2
Fermeture
```

C'est très utile pour « ranger » : on écrit la fermeture d'un fichier juste à côté de
son ouverture, et on est sûr qu'elle aura lieu, même si la fonction sort plus tôt.

Plusieurs `defer` s'exécutent dans l'ordre **inverse**, comme une pile d'assiettes : la
dernière posée est la première reprise.

```go
package main

import "fmt"

func main() {
	for i := range 3 {
		defer fmt.Println("defer", i)
	}
	fmt.Println("fin de main")
}
```

```sortie
fin de main
defer 2
defer 1
defer 0
```

## Une fonction qui s'appelle elle-même

Une fonction peut s'appeler elle-même : c'est la
[récursion](../debuter/glossaire.md#recursion). Il faut un **cas d'arrêt**, sinon elle
ne s'arrête jamais.

```go
package main

import "fmt"

func factorielle(n int) int {
	if n <= 1 {
		return 1
	}
	return n * factorielle(n-1)
}

func main() {
	fmt.Println(factorielle(5))
}
```

```sortie
120
```

`factorielle(5)` vaut `5 * factorielle(4)`, qui vaut `4 * factorielle(3)`… jusqu'à
`factorielle(1)`, le cas d'arrêt, qui vaut 1. Le résultat est 5 × 4 × 3 × 2 × 1 = 120.

## Erreurs fréquentes

### Oublier un return

```go norun
package main

import "fmt"

func signe(n int) string {
	if n >= 0 {
		return "positif"
	}
}

func main() {
	fmt.Println(signe(3))
}
```

```text
# command-line-arguments
./main.go:9:1: missing return
```

Si `n` est négatif, la fonction n'a rien à renvoyer. Ajoutez un `return "négatif"`
avant l'accolade finale.

### Oublier un argument

```go norun
package main

import "fmt"

func aire(largeur, hauteur int) int {
	return largeur * hauteur
}

func main() {
	fmt.Println(aire(3))
}
```

```text
# command-line-arguments
./main.go:10:19: not enough arguments in call to aire
	have (number)
	want (int, int)
```

La fonction veut deux `int` (*want*), elle reçoit un seul nombre (*have*). Donnez-les
tous : `aire(3, 4)`.

### Recevoir une seule valeur au lieu de deux

```go norun
package main

import "fmt"

func diviser(a, b int) (int, int) {
	return a / b, a % b
}

func main() {
	q := diviser(7, 2)
	fmt.Println(q)
}
```

```text
# command-line-arguments
./main.go:10:7: assignment mismatch: 1 variable but diviser returns 2 values
```

Il faut une variable par valeur renvoyée : `q, r := diviser(7, 2)`, ou `q, _ :=` pour
ignorer le reste.

### Passer un argument du mauvais type

```go norun
package main

import "fmt"

func double(n int) int {
	return n * 2
}

func main() {
	fmt.Println(double("4"))
}
```

```text
# command-line-arguments
./main.go:10:21: cannot use "4" (untyped string constant) as int value in argument to double
```

`"4"` est du texte, pas un nombre. Écrivez `double(4)`, ou convertissez le texte avec
`strconv.Atoi`.

## Testez-vous

**Question 1 :** qu'affiche ce programme ?

```go norun
package main

import "fmt"

func main() {
	defer fmt.Println("A")
	fmt.Println("B")
	defer fmt.Println("C")
}
```

> **Réponse**
>
> `B`, puis `C`, puis `A`. `B` s'affiche tout de suite ; les `defer` attendent la fin
> de `main` et passent dans l'ordre inverse.

**Question 2 :** écrivez une fonction `moyenne(a, b int) float64` qui renvoie la
moyenne de deux entiers.

> **Réponse**
>
> ```go
> package main
>
> import "fmt"
>
> func moyenne(a, b int) float64 {
> 	return float64(a+b) / 2
> }
>
> func main() {
> 	fmt.Println(moyenne(3, 4))
> }
> ```
>
> ```sortie
> 3.5
> ```
>
> La conversion `float64(...)` évite la division entière.

**Question 3 :** quelle est la différence entre un paramètre et un argument ?

> **Réponse**
>
> Le paramètre est le nom écrit dans `func` (`prenom string`). L'argument est la
> valeur donnée à l'appel (`"Alice"`).

## En résumé

- Une fonction s'écrit `func nom(paramètre type) typeRetour { ... }` et renvoie avec
  `return`.
- Une fonction peut renvoyer plusieurs valeurs ; la dernière est souvent une `error`.
- `...type` accepte un nombre variable d'arguments, reçus dans une slice.
- Une fonction est une valeur : on la range dans une variable, on la passe en argument.
- Une closure garde les variables qui l'entourent.
- `defer` repousse un appel à la fin de la fonction, dans l'ordre inverse.

Suite : [travailler avec du texte](texte.md). Comparez avec
[les fonctions en Rust](../rust/fonctions.md).
