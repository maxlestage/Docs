# Les pointeurs

Cette page explique ce qu'est un pointeur, comment l'utiliser avec `&` et `*`, et quand il vaut mieux s'en servir.

> **Objectifs :**
>
> - vous saurez ce qu'est une adresse et un pointeur ;
> - vous saurez utiliser `&` pour prendre une adresse et `*` pour la suivre ;
> - vous saurez pourquoi une fonction modifie (ou non) une variable ;
> - vous saurez éviter la panique du pointeur `nil` ;
> - vous saurez quand choisir un pointeur.

## Une adresse plutôt qu'une copie

Chaque variable est rangée quelque part dans la
[mémoire vive](../debuter/glossaire.md#memoire-vive), à une **adresse**. Un
[pointeur](../debuter/glossaire.md#pointeur) est une variable qui contient cette adresse.

Pensez à une maison. Vous pouvez donner à un ami **une photo** de votre maison : il peut
la colorier, votre maison ne change pas. Ou vous pouvez lui donner **votre adresse** :
il peut alors venir repeindre la vraie porte. La photo, c'est une copie. L'adresse,
c'est un pointeur.

## & et *

Deux symboles suffisent :

- `&x` donne l'adresse de `x` (« où habite `x` ? ») ;
- `*p` suit l'adresse rangée dans `p` et donne la valeur qui s'y trouve (« va voir à
  cette adresse »).

Le type d'un pointeur vers un `int` s'écrit `*int`.

```go
package main

import "fmt"

func main() {
	x := 10
	p := &x

	fmt.Println("x vaut", x)
	fmt.Println("p pointe vers", *p)

	*p = 20
	fmt.Println("x vaut maintenant", x)
	fmt.Printf("p est de type %T\n", p)
}
```

```sortie
x vaut 10
p pointe vers 10
x vaut maintenant 20
p est de type *int
```

Écrire `*p = 20` change la valeur **à l'adresse** de `x`. C'est donc `x` qui change.

> **Note :** si vous affichez `p` lui-même, Go montre une adresse comme `0xc000012345`.
> Elle change d'une exécution à l'autre : ne comptez jamais dessus.

## Passer par valeur ou par pointeur

En Go, une fonction reçoit toujours une **copie** de ses
[arguments](../debuter/glossaire.md#argument). C'est le passage **par valeur**. La
fonction ne peut donc pas modifier la variable de l'appelant.

Pour qu'elle le puisse, passez l'adresse : la fonction reçoit une copie de l'adresse,
mais cette adresse mène à la vraie variable.

```go
package main

import "fmt"

func doublerCopie(n int) {
	n = n * 2
}

func doubler(n *int) {
	*n = *n * 2
}

func main() {
	x := 5
	doublerCopie(x)
	fmt.Println("Après doublerCopie :", x)

	doubler(&x)
	fmt.Println("Après doubler :", x)
}
```

```sortie
Après doublerCopie : 5
Après doubler : 10
```

### Le cas des structures

Une structure passée par valeur est copiée en entier. Avec un pointeur vers une
structure, Go vous évite d'écrire `(*p).Champ` : `p.Champ` suffit.

```go
package main

import "fmt"

type Joueur struct {
	Nom  string
	Vies int
}

func perdreUneVie(j *Joueur) {
	j.Vies--
}

func main() {
	j := Joueur{Nom: "Léa", Vies: 3}
	perdreUneVie(&j)
	perdreUneVie(&j)
	fmt.Printf("%+v\n", j)
}
```

```sortie
{Nom:Léa Vies:1}
```

## Les récepteurs pointeurs

Une méthode est une fonction comme une autre : son récepteur suit la même règle. Avec
`func (c *Compte)`, la méthode modifie l'original. Vous l'avez vu dans
[structures et méthodes](types.md).

```go
package main

import "fmt"

type Compte struct {
	Solde int
}

func (c *Compte) Deposer(montant int) {
	c.Solde += montant
}

func (c Compte) Afficher() {
	fmt.Println("Solde :", c.Solde)
}

func main() {
	c := Compte{}
	c.Deposer(50)
	c.Deposer(25)
	c.Afficher()
}
```

```sortie
Solde : 75
```

`Deposer` modifie le compte : récepteur pointeur. `Afficher` ne fait que lire : un
récepteur valeur suffit.

## Le pointeur nil

Un pointeur qui ne pointe vers rien vaut `nil`. C'est la valeur zéro des pointeurs :
`var p *int` vaut `nil`. Suivre un pointeur `nil` est impossible : il n'y a pas de
maison à cette adresse.

Testez donc `p != nil` avant de suivre un pointeur qui peut être vide.

```go
package main

import "fmt"

type Joueur struct {
	Nom string
}

func nomDe(j *Joueur) string {
	if j == nil {
		return "(personne)"
	}
	return j.Nom
}

func main() {
	var absent *Joueur
	present := &Joueur{Nom: "Sam"}

	fmt.Println(absent == nil)
	fmt.Println(nomDe(absent))
	fmt.Println(nomDe(present))
}
```

```sortie
true
(personne)
Sam
```

## new et &T{}

`new(T)` réserve une valeur zéro de type `T` et renvoie son adresse. En pratique, on
écrit plus souvent `&T{...}`, qui permet de remplir les champs tout de suite.

```go
package main

import "fmt"

type Point struct {
	X, Y int
}

func main() {
	n := new(int)
	*n = 7
	fmt.Println(*n)

	p := new(Point)
	p.X = 3
	fmt.Println(*p)

	q := &Point{X: 1, Y: 2}
	fmt.Println(*q)
}
```

```sortie
7
{3 0}
{1 2}
```

> **Note :** en Go, une fonction peut renvoyer l'adresse d'une variable locale, comme
> `return &Point{}`. Go garde la valeur en vie tant qu'on s'en sert : il n'y a rien
> à libérer à la main.

## Quand utiliser un pointeur ?

Utilisez un pointeur :

- quand une fonction ou une méthode doit **modifier** la valeur reçue ;
- quand la structure est **grosse** et que la copier coûterait cher ;
- quand « pas de valeur » doit être possible : le pointeur vaut alors `nil`.

Gardez une valeur simple dans les autres cas, surtout pour les petits types (`int`,
`string`, un `Point`). Une valeur ne peut pas être `nil`, donc elle ne peut pas
provoquer de panique.

> **À retenir :** les slices, les maps et les canaux contiennent déjà une sorte de
> pointeur caché. Une fonction qui reçoit une map peut la modifier sans `*` (voir
> [les maps](maps.md)).

```go
package main

import "fmt"

func ajouter(stock map[string]int, fruit string) {
	stock[fruit]++
}

func main() {
	stock := map[string]int{}
	ajouter(stock, "pomme")
	ajouter(stock, "pomme")
	fmt.Println(stock["pomme"])
}
```

```sortie
2
```

## Erreurs fréquentes

### Suivre un pointeur nil

```go norun
package main

import "fmt"

type Joueur struct {
	Nom string
}

func main() {
	var j *Joueur
	fmt.Println(j.Nom)
}
```

```text
panic: runtime error: invalid memory address or nil pointer dereference
[signal SIGSEGV: segmentation violation code=0x1 addr=0x0 pc=0x491af6]

goroutine 1 [running]:
main.main()
	/chemin/vers/main.go:11 +0x16
exit status 2
```

`j` vaut `nil` : il n'y a aucun joueur à cette adresse. Le message « nil pointer
dereference » veut dire « suivre un pointeur nil ». Créez la valeur
(`j := &Joueur{Nom: "Sam"}`) ou testez `j != nil` avant.

### Oublier le &

```go norun
package main

import "fmt"

func doubler(n *int) {
	*n = *n * 2
}

func main() {
	x := 5
	doubler(x)
	fmt.Println(x)
}
```

```text
# command-line-arguments
./main.go:11:10: cannot use x (variable of type int) as *int value in argument to doubler
```

La fonction attend une adresse (`*int`), pas un `int`. Écrivez `doubler(&x)`.

### Faire des calculs sur une adresse

```go norun
package main

import "fmt"

func main() {
	x := 5
	p := &x
	fmt.Println(p + 1)
}
```

```text
# command-line-arguments
./main.go:8:14: invalid operation: p + 1 (mismatched types *int and untyped int)
```

Go interdit les calculs sur les adresses. Vous vouliez sans doute la valeur pointée :
`*p + 1`.

## Testez-vous

**Question 1 :** qu'affiche ce programme ?

```go norun
x := 1
p := &x
*p = *p + 10
fmt.Println(x)
```

> **Réponse**
>
> `11`. `p` contient l'adresse de `x`, donc `*p = ...` modifie `x`.

**Question 2 :** quelle est la valeur zéro d'une variable `var p *string` ?

> **Réponse**
>
> `nil`. Le pointeur ne pointe vers rien ; écrire `*p` provoquerait une panique.

**Question 3 :** écrivez une fonction `echanger(a, b *int)` qui échange les valeurs de
deux variables.

> **Réponse**
>
> ```go
> package main
>
> import "fmt"
>
> func echanger(a, b *int) {
> 	*a, *b = *b, *a
> }
>
> func main() {
> 	x, y := 1, 2
> 	echanger(&x, &y)
> 	fmt.Println(x, y)
> }
> ```
>
> ```sortie
> 2 1
> ```

## En résumé

- Un pointeur contient l'adresse d'une valeur ; son type s'écrit `*T`.
- `&x` prend l'adresse de `x` ; `*p` lit ou modifie la valeur pointée.
- Go passe tout par valeur ; passez un pointeur pour modifier l'original.
- Un pointeur `nil` ne pointe vers rien : le suivre fait paniquer le programme.
- `new(T)` et `&T{...}` créent une valeur et renvoient son adresse.

Suite : [les génériques](generiques.md).
