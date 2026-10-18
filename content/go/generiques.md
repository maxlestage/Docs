# Les génériques

Cette page montre comment écrire une seule fonction ou un seul type qui marche avec plusieurs types, grâce aux paramètres de type.

> **Objectifs :**
>
> - vous saurez écrire une fonction générique avec `[T any]` ;
> - vous saurez limiter les types acceptés avec une contrainte (`comparable`, `cmp.Ordered`) ;
> - vous saurez écrire votre propre contrainte avec `~int | ~float64` ;
> - vous saurez créer un type générique, comme une pile ;
> - vous saurez quand les génériques ne sont pas utiles.

## Le problème : écrire deux fois la même chose

Imaginez une fonction qui renvoie le plus grand de deux entiers. Puis vous avez besoin de
la même chose pour des nombres à virgule, puis pour des textes. Sans génériques, il faut
écrire trois fonctions presque identiques :

```go norun
func MaxInt(a, b int) int { ... }
func MaxFloat(a, b float64) float64 { ... }
func MaxString(a, b string) string { ... }
```

Un [générique](../debuter/glossaire.md#generique) règle ce problème : on écrit le code
une fois, avec un **paramètre de type** à la place du vrai type. C'est comme un moule
à gâteaux : la forme est la même, que vous y versiez du chocolat ou de la vanille.

## Une première fonction générique

Le paramètre de type s'écrit entre crochets, juste après le nom de la fonction :
`[T any]`. `T` est un nom de type provisoire ; `any` dit que `T` peut être n'importe quel
type.

```go
package main

import "fmt"

func Premier[T any](elements []T) T {
	return elements[0]
}

func main() {
	fmt.Println(Premier([]int{4, 8, 15}))
	fmt.Println(Premier([]string{"pomme", "poire"}))
	fmt.Println(Premier[float64]([]float64{1.5, 2.5}))
}
```

```sortie
4
pomme
1.5
```

Go **devine** `T` à partir des arguments : avec `[]int`, `T` vaut `int`. Vous pouvez
aussi le donner vous-même, comme dans `Premier[float64](...)`.

## Les contraintes

Avec `any`, Go ne sait rien de `T`. Vous ne pouvez donc pas comparer deux `T` avec `>` :
certains types ne se comparent pas. Une **contrainte** limite les types acceptés, et en
échange elle autorise des opérations.

### comparable

`comparable` accepte les types qui supportent `==` et `!=` : nombres, textes,
booléens, structures simples…

```go
package main

import "fmt"

func Contient[T comparable](elements []T, cherche T) bool {
	for _, e := range elements {
		if e == cherche {
			return true
		}
	}
	return false
}

func main() {
	fmt.Println(Contient([]int{1, 2, 3}, 2))
	fmt.Println(Contient([]string{"rouge", "vert"}, "bleu"))
}
```

```sortie
true
false
```

### cmp.Ordered

Le package `cmp` fournit `cmp.Ordered` : les types qui supportent `<`, `<=`, `>` et
`>=`. Ce sont les entiers, les flottants et les textes.

```go
package main

import (
	"cmp"
	"fmt"
)

func Max[T cmp.Ordered](a, b T) T {
	if a > b {
		return a
	}
	return b
}

func main() {
	fmt.Println(Max(3, 7))
	fmt.Println(Max(2.5, 1.5))
	fmt.Println(Max("abricot", "banane"))
}
```

```sortie
7
2.5
banane
```

Les textes se comparent dans l'ordre alphabétique : `"banane"` vient après `"abricot"`.

> **Note :** depuis Go 1.21, `max` et `min` existent déjà dans le langage, et
> `slices.Max` trouve le plus grand élément d'une slice. `Max` est ici un exercice.

## Écrire sa propre contrainte

Une contrainte est une interface. Elle peut lister des types séparés par `|`
(« ou »). Le `~` devant un type veut dire : « ce type, ou tout type construit dessus ».
Ainsi `~int` accepte `int`, mais aussi un type `type Age int`.

```go
package main

import "fmt"

type Nombre interface {
	~int | ~float64
}

func Somme[T Nombre](valeurs []T) T {
	var total T
	for _, v := range valeurs {
		total += v
	}
	return total
}

type Euros float64

func main() {
	fmt.Println(Somme([]int{1, 2, 3}))
	fmt.Println(Somme([]float64{0.5, 0.25}))
	fmt.Println(Somme([]Euros{9.99, 0.01}))
}
```

```sortie
6
0.75
10
```

`var total T` crée la valeur zéro de `T` : `0` pour un nombre. Sans le `~`, la ligne
avec `Euros` serait refusée.

## Map et Filter

Les génériques brillent pour les fonctions qui travaillent sur des
[collections](collections.md). `Map` transforme chaque élément ; `Filter` garde ceux qui
passent un test. Elles reçoivent une fonction en paramètre (voir
[les fonctions](fonctions.md)).

```go
package main

import (
	"fmt"
	"strings"
)

func Map[T, U any](elements []T, f func(T) U) []U {
	resultat := make([]U, 0, len(elements))
	for _, e := range elements {
		resultat = append(resultat, f(e))
	}
	return resultat
}

func Filter[T any](elements []T, garder func(T) bool) []T {
	var resultat []T
	for _, e := range elements {
		if garder(e) {
			resultat = append(resultat, e)
		}
	}
	return resultat
}

func main() {
	nombres := []int{1, 2, 3, 4, 5, 6}

	carres := Map(nombres, func(n int) int { return n * n })
	fmt.Println(carres)

	pairs := Filter(nombres, func(n int) bool { return n%2 == 0 })
	fmt.Println(pairs)

	mots := Map([]string{"go", "est", "simple"}, strings.ToUpper)
	fmt.Println(mots)

	longueurs := Map(mots, func(s string) int { return len(s) })
	fmt.Println(longueurs)
}
```

```sortie
[1 4 9 16 25 36]
[2 4 6]
[GO EST SIMPLE]
[2 3 6]
```

`Map` a **deux** paramètres de type : `T` pour l'entrée et `U` pour la sortie. Ils
peuvent être différents, comme dans le dernier appel (`string` vers `int`).

## Les types génériques

Un type peut aussi avoir un paramètre de type. Voici une **pile** : une liste où l'on
ajoute et retire toujours par le haut, comme une pile d'assiettes. Le dernier élément
posé est le premier repris.

```go
package main

import "fmt"

type Pile[T any] struct {
	elements []T
}

func (p *Pile[T]) Empiler(v T) {
	p.elements = append(p.elements, v)
}

func (p *Pile[T]) Depiler() (T, bool) {
	var zero T
	if len(p.elements) == 0 {
		return zero, false
	}
	dernier := p.elements[len(p.elements)-1]
	p.elements = p.elements[:len(p.elements)-1]
	return dernier, true
}

func (p *Pile[T]) Taille() int {
	return len(p.elements)
}

func main() {
	var assiettes Pile[string]
	assiettes.Empiler("bleue")
	assiettes.Empiler("rouge")
	assiettes.Empiler("verte")
	fmt.Println("Taille :", assiettes.Taille())

	for assiettes.Taille() > 0 {
		a, _ := assiettes.Depiler()
		fmt.Println(a)
	}

	_, ok := assiettes.Depiler()
	fmt.Println("Encore une ?", ok)

	var nombres Pile[int]
	nombres.Empiler(42)
	n, _ := nombres.Depiler()
	fmt.Println(n + 1)
}
```

```sortie
Taille : 3
verte
rouge
bleue
Encore une ? false
43
```

Dans les méthodes, on écrit `Pile[T]` pour le récepteur. `Depiler` renvoie aussi un
booléen : la pile peut être vide, et il faut le dire à l'appelant.

## Quand ne pas utiliser les génériques

Les génériques sont utiles, mais ils rendent le code plus abstrait. Évitez-les :

- quand votre fonction ne sert qu'à **un seul** type : écrivez ce type, tout simplement ;
- quand une **interface** suffit : si vous appelez seulement des méthodes, comme
  `Aire()`, prenez une interface (voir [les interfaces](interfaces.md)) ;
- quand le code de chaque type est **différent** : un générique sert à écrire le même
  code pour tous.

> **À retenir :** commencez par du code simple pour un type. Passez aux génériques
> quand vous vous surprenez à copier la même fonction pour un deuxième type.

## Erreurs fréquentes

### Comparer avec la contrainte any

```go norun
package main

import "fmt"

func Max[T any](a, b T) T {
	if a > b {
		return a
	}
	return b
}

func main() {
	fmt.Println(Max(3, 7))
}
```

```text
# command-line-arguments
./main.go:6:5: invalid operation: a > b (type parameter T cannot use operator >)
```

`any` accepte tous les types, y compris ceux qui ne se comparent pas. Utilisez la
contrainte `cmp.Ordered` et importez `cmp`.

### Mélanger deux types pour un même T

```go norun
package main

import (
	"cmp"
	"fmt"
)

func Max[T cmp.Ordered](a, b T) T {
	if a > b {
		return a
	}
	return b
}

func main() {
	x := 3
	y := 2.5
	fmt.Println(Max(x, y))
}
```

```text
# command-line-arguments
./main.go:18:21: in call to Max, type float64 of y does not match inferred type int for T
```

`a` et `b` ont le même type `T`. Go a choisi `int` d'après `x`, puis trouve un `float64`.
Convertissez : `Max(float64(x), y)`.

### Go ne peut pas deviner T

```go norun
package main

import "fmt"

func Vide[T any]() []T {
	return []T{}
}

func main() {
	s := Vide()
	fmt.Println(s)
}
```

```text
# command-line-arguments
./main.go:10:11: in call to Vide, cannot infer T (declared at ./main.go:5:11)
```

Aucun argument ne permet de deviner `T`. Donnez-le entre crochets : `Vide[int]()`.

## Testez-vous

**Question 1 :** quelle contrainte choisir pour une fonction qui utilise `==` sur ses
éléments ?

> **Réponse**
>
> `comparable`. Elle accepte tous les types qui supportent `==` et `!=`.

**Question 2 :** que veut dire le `~` dans `~int` ?

> **Réponse**
>
> « `int` ou tout type dont le type de base est `int` ». Par exemple `type Age int`
> satisfait `~int`, mais pas `int` tout court.

**Question 3 :** écrivez une fonction générique `Compter[T any](s []T, f func(T) bool) int`
qui compte les éléments pour lesquels `f` renvoie `true`.

> **Réponse**
>
> ```go
> package main
>
> import "fmt"
>
> func Compter[T any](s []T, f func(T) bool) int {
> 	n := 0
> 	for _, e := range s {
> 		if f(e) {
> 			n++
> 		}
> 	}
> 	return n
> }
>
> func main() {
> 	notes := []int{12, 8, 15, 9, 18}
> 	fmt.Println(Compter(notes, func(n int) bool { return n >= 10 }))
> }
> ```
>
> ```sortie
> 3
> ```

## En résumé

- `[T any]` déclare un paramètre de type ; Go le devine souvent à partir des arguments.
- Une contrainte limite les types : `comparable` pour `==`, `cmp.Ordered` pour `<` et `>`.
- Une contrainte maison s'écrit comme une interface : `~int | ~float64`.
- Les types aussi peuvent être génériques : `Pile[T]`, avec des méthodes sur `*Pile[T]`.
- N'utilisez les génériques que pour éviter de vraies répétitions.

Suite : [gérer les erreurs](erreurs.md).
