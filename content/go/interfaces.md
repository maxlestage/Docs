# Les interfaces

Cette page explique comment décrire un comportement avec une interface, pour écrire du code qui accepte plusieurs types différents.

> **Objectifs :**
>
> - vous saurez définir une interface et la satisfaire sans rien déclarer ;
> - vous saurez ranger des valeurs de types différents dans une même slice ;
> - vous saurez rendre un type affichable avec `fmt.Stringer` ;
> - vous saurez retrouver le type caché derrière `any` avec une assertion ou un `switch` ;
> - vous saurez pourquoi Go préfère les petites interfaces.

## Qu'est-ce qu'une interface ?

Une **interface** est une liste de méthodes. Elle dit **ce qu'un type sait faire**, pas
ce qu'il est.

Pensez à une prise électrique. La prise ne demande pas si vous branchez une lampe, un
grille-pain ou un chargeur. Elle demande seulement : « avez-vous la bonne fiche ? ». Une
interface est cette prise : tout type qui a les bonnes méthodes peut s'y brancher.

```go
package main

import "fmt"

type Forme interface {
	Aire() float64
}

type Rectangle struct {
	Largeur, Hauteur float64
}

func (r Rectangle) Aire() float64 {
	return r.Largeur * r.Hauteur
}

func afficherAire(f Forme) {
	fmt.Println("Aire :", f.Aire())
}

func main() {
	r := Rectangle{Largeur: 3, Hauteur: 4}
	afficherAire(r)
}
```

```sortie
Aire : 12
```

La fonction `afficherAire` accepte n'importe quelle `Forme`. Elle ne connaît pas
`Rectangle` : elle sait seulement qu'elle peut appeler `Aire()`.

## Une implémentation implicite

Remarquez que `Rectangle` ne dit nulle part « j'implémente `Forme` ». En Go, c'est
**implicite** : un type satisfait une interface dès qu'il possède toutes ses méthodes,
avec les mêmes noms et les mêmes types.

C'est très pratique : vous pouvez créer une interface pour des types écrits par
quelqu'un d'autre, sans les modifier. En Rust, au contraire, il faut écrire
`impl Trait for Type` (voir [les traits en Rust](../rust/types.md#les-traits)).

## Le polymorphisme

Ajoutons un deuxième type, `Cercle`. Il a lui aussi une méthode `Aire()` : c'est donc
aussi une `Forme`. Une slice `[]Forme` peut contenir des rectangles **et** des cercles.

Ce mot savant, **polymorphisme** (« plusieurs formes »), veut dire simplement : le même
appel `f.Aire()` fait un travail différent selon le type réel de `f`.

```go
package main

import (
	"fmt"
	"math"
)

type Forme interface {
	Aire() float64
}

type Rectangle struct {
	Largeur, Hauteur float64
}

func (r Rectangle) Aire() float64 {
	return r.Largeur * r.Hauteur
}

type Cercle struct {
	Rayon float64
}

func (c Cercle) Aire() float64 {
	return math.Pi * c.Rayon * c.Rayon
}

func main() {
	formes := []Forme{
		Rectangle{Largeur: 3, Hauteur: 4},
		Cercle{Rayon: 1},
		Rectangle{Largeur: 2, Hauteur: 2},
	}

	total := 0.0
	for _, f := range formes {
		fmt.Printf("%.2f\n", f.Aire())
		total += f.Aire()
	}
	fmt.Printf("Total : %.2f\n", total)
}
```

```sortie
12.00
3.14
4.00
Total : 19.14
```

## fmt.Stringer : choisir l'affichage

Le package `fmt` définit une toute petite interface :

```go norun
type Stringer interface {
	String() string
}
```

Si votre type a une méthode `String() string`, `fmt.Println` et `%v` l'utilisent pour
l'afficher. C'est comme donner sa carte de visite.

```go
package main

import "fmt"

type Temperature struct {
	Degres float64
}

func (t Temperature) String() string {
	return fmt.Sprintf("%.1f °C", t.Degres)
}

func main() {
	t := Temperature{Degres: 21.456}
	fmt.Println(t)
	fmt.Printf("Il fait %v dehors.\n", t)
}
```

```sortie
21.5 °C
Il fait 21.5 °C dehors.
```

## L'interface vide : any

Une interface sans aucune méthode, `interface{}`, est satisfaite par **tous** les types.
Go lui donne un nom plus court : `any` (« n'importe quoi »).

```go
package main

import "fmt"

func main() {
	var boite any
	boite = 42
	fmt.Println(boite)
	boite = "bonjour"
	fmt.Println(boite)

	melange := []any{1, "deux", 3.0, true}
	fmt.Println(len(melange), "éléments")
}
```

```sortie
42
bonjour
4 éléments
```

> **Attention :** `any` fait perdre l'aide du compilateur : il ne sait plus quel type est
> rangé dedans. Utilisez-le seulement quand c'est vraiment nécessaire. `fmt.Println`
> en est un bon exemple : il accepte des `any` pour tout afficher.

## Les assertions de type

Pour récupérer la valeur rangée dans une interface, on écrit une **assertion de type** :
`x.(string)`. Elle veut dire : « je pense que `x` contient un `string` ».

Si vous vous trompez, le programme s'arrête. Utilisez donc la forme à deux résultats,
dite **virgule ok** : `s, ok := x.(string)`. Si le type est le bon, `ok` vaut `true` ;
sinon `ok` vaut `false` et `s` vaut la valeur zéro.

```go
package main

import "fmt"

func main() {
	var x any = "bonjour"

	s, ok := x.(string)
	fmt.Println(s, ok)

	n, ok := x.(int)
	fmt.Println(n, ok)

	if texte, ok := x.(string); ok {
		fmt.Println("Longueur :", len(texte))
	}
}
```

```sortie
bonjour true
0 false
Longueur : 7
```

## Le switch de type

Quand il y a plusieurs types possibles, un **switch de type** est plus lisible. On écrit
`x.(type)`, uniquement dans un `switch`. Dans chaque `case`, la variable a le bon type.

```go
package main

import "fmt"

func decrire(x any) string {
	switch v := x.(type) {
	case int:
		return fmt.Sprintf("un entier, son double vaut %d", v*2)
	case string:
		return fmt.Sprintf("un texte de %d lettres", len(v))
	case bool:
		return "un booléen"
	default:
		return fmt.Sprintf("un type inconnu : %T", v)
	}
}

func main() {
	fmt.Println(decrire(21))
	fmt.Println(decrire("Go"))
	fmt.Println(decrire(true))
	fmt.Println(decrire(3.5))
}
```

```sortie
un entier, son double vaut 42
un texte de 2 lettres
un booléen
un type inconnu : float64
```

`%T` affiche le nom du type d'une valeur.

## Les petites interfaces

En Go, les bonnes interfaces sont **petites** : souvent une seule méthode. Leur nom
finit alors souvent par `-er` : `Stringer`, `Reader`, `Writer`. Un proverbe Go dit :
« plus l'interface est grande, plus l'abstraction est faible ».

Une petite interface est facile à satisfaire, donc beaucoup de types peuvent s'y
brancher. Le meilleur exemple est `io.Writer`, « quelque chose dans quoi on peut
écrire » :

```go norun
type Writer interface {
	Write(p []byte) (n int, err error)
}
```

Un fichier, la sortie standard `os.Stdout`, une connexion réseau et un
`strings.Builder` sont tous des `io.Writer`. Une fonction qui écrit dans un `io.Writer`
marche donc avec chacun d'eux.

```go
package main

import (
	"fmt"
	"io"
	"os"
	"strings"
)

func ecrireListe(w io.Writer, courses []string) {
	for i, c := range courses {
		fmt.Fprintf(w, "%d. %s\n", i+1, c)
	}
}

func main() {
	courses := []string{"pain", "lait", "pommes"}

	ecrireListe(os.Stdout, courses)

	var b strings.Builder
	ecrireListe(&b, courses)
	fmt.Println("Le builder contient", len(b.String()), "octets.")
}
```

```sortie
1. pain
2. lait
3. pommes
Le builder contient 26 octets.
```

`fmt.Fprintf` est comme `fmt.Printf`, mais il écrit dans le `io.Writer` donné. Le même
code écrit une fois à l'écran, une fois en mémoire.

> **Astuce :** quand vous écrivez une fonction, demandez la plus petite interface dont
> elle a besoin. Ici, `ecrireListe` demande un `io.Writer`, pas un fichier : elle est
> plus facile à réutiliser et à [tester](tests.md).

## Erreurs fréquentes

### Une méthode manque

```go norun
package main

import "fmt"

type Forme interface {
	Aire() float64
	Perimetre() float64
}

type Carre struct {
	Cote float64
}

func (c Carre) Aire() float64 {
	return c.Cote * c.Cote
}

func main() {
	var f Forme = Carre{Cote: 2}
	fmt.Println(f.Aire())
}
```

```text
# command-line-arguments
./main.go:19:16: cannot use Carre{…} (value of struct type Carre) as Forme value in variable declaration: Carre does not implement Forme (missing method Perimetre)
```

Le message donne le nom de la méthode qui manque : `Perimetre`. Ajoutez-la à `Carre`.

### Une méthode avec un récepteur pointeur

```go norun
package main

import "fmt"

type Forme interface {
	Aire() float64
}

type Carre struct {
	Cote float64
}

func (c *Carre) Aire() float64 {
	return c.Cote * c.Cote
}

func main() {
	var f Forme = Carre{Cote: 2}
	fmt.Println(f.Aire())
}
```

```text
# command-line-arguments
./main.go:18:16: cannot use Carre{…} (value of struct type Carre) as Forme value in variable declaration: Carre does not implement Forme (method Aire has pointer receiver)
```

La méthode est définie sur `*Carre`, donc seul `*Carre` est une `Forme`. Donnez un
pointeur : `var f Forme = &Carre{Cote: 2}`. Ou bien, si la méthode ne modifie rien,
prenez un récepteur valeur : `func (c Carre) Aire()`.

### Une assertion sans virgule ok

```go norun
package main

import "fmt"

func main() {
	var x any = "bonjour"
	n := x.(int)
	fmt.Println(n)
}
```

```text
panic: interface conversion: interface {} is string, not int

goroutine 1 [running]:
main.main()
	/chemin/vers/main.go:7 +0x28
exit status 2
```

`x` contient un `string`, pas un `int` : le programme
[panique](../debuter/glossaire.md#panique). Utilisez `n, ok := x.(int)` et vérifiez `ok`.

## Testez-vous

**Question 1 :** un type `Chat` a une méthode `String() string`. Que fait
`fmt.Println(monChat)` ?

> **Réponse**
>
> Il appelle `monChat.String()` et affiche le texte renvoyé, car `Chat` satisfait
> l'interface `fmt.Stringer`.

**Question 2 :** que valent `v` et `ok` après `v, ok := x.(int)` si `x` contient `"7"` ?

> **Réponse**
>
> `v` vaut `0` (la valeur zéro des entiers) et `ok` vaut `false` : `x` contient un
> texte, pas un entier. Le programme ne s'arrête pas.

**Question 3 :** écrivez une interface `Animal` avec une méthode `Cri() string`, deux
types `Chien` et `Chat`, puis affichez le cri de chacun dans une boucle.

> **Réponse**
>
> ```go
> package main
>
> import "fmt"
>
> type Animal interface {
> 	Cri() string
> }
>
> type Chien struct{}
>
> func (Chien) Cri() string { return "Wouf" }
>
> type Chat struct{}
>
> func (Chat) Cri() string { return "Miaou" }
>
> func main() {
> 	animaux := []Animal{Chien{}, Chat{}}
> 	for _, a := range animaux {
> 		fmt.Println(a.Cri())
> 	}
> }
> ```
>
> ```sortie
> Wouf
> Miaou
> ```

## En résumé

- Une interface est une liste de méthodes ; un type la satisfait sans le déclarer.
- Une slice d'interfaces range des types différents qui partagent un comportement.
- `String() string` rend un type affichable par `fmt` (interface `fmt.Stringer`).
- `any` accepte tout ; `v, ok := x.(T)` et `switch x.(type)` retrouvent le type réel.
- Préférez les petites interfaces, comme `io.Writer`.

Suite : [les pointeurs](pointeurs.md).
