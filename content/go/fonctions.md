# Les fonctions

Cette page explique comment écrire des fonctions en Go avec `func`, leur donner des paramètres et leur faire renvoyer une ou plusieurs valeurs.

> **Objectifs :**
>
> - vous saurez écrire une fonction avec `func` et l'appeler ;
> - vous saurez donner des paramètres typés et renvoyer une ou plusieurs valeurs ;
> - vous saurez sortir d'une fonction plus tôt avec `return`.

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

Ligne par ligne :

- `func saluer() { ... }` enregistre la recette sous le nom `saluer`. Rien ne
  s'affiche encore ;
- le programme commence toujours par `main` ;
- le premier `saluer()` exécute la recette : « Bonjour ! » s'affiche ;
- le deuxième `saluer()` l'exécute une seconde fois.

Écrire `func saluer()` ne lance rien : cela enregistre la recette. Écrire `saluer()`
**appelle** la fonction, c'est-à-dire l'exécute.

> **Note :** les fonctions se définissent en dehors de `main`, au niveau du fichier.
> L'ordre n'a pas d'importance : `main` peut appeler une fonction écrite plus bas.

## Les paramètres

Une recette de crêpes peut servir pour 4 ou pour 8 personnes : c'est la même recette,
avec une quantité différente. Les paramètres permettent à une fonction de travailler
avec des valeurs différentes à chaque appel.

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

Ligne par ligne :

- `func saluer(prenom string)` annonce une case vide nommée `prenom`, qui recevra du
  texte ;
- `saluer("Alice")` remplit la case : dans la fonction, `prenom` vaut `"Alice"` ;
- `saluer("Bob")` relance la fonction, et cette fois `prenom` vaut `"Bob"`.

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

**À vous :** écrivez une fonction `annoncer(plat string, prix int)` qui affiche
« Plat du jour : soupe, 6 euros. ». Appelez-la deux fois depuis `main`, avec deux plats
différents.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func annoncer(plat string, prix int) {
> 	fmt.Printf("Plat du jour : %s, %d euros.\n", plat, prix)
> }
>
> func main() {
> 	annoncer("soupe", 6)
> 	annoncer("lasagnes", 12)
> }
> ```
>
> ```sortie
> Plat du jour : soupe, 6 euros.
> Plat du jour : lasagnes, 12 euros.
> ```

## Renvoyer une valeur

Afficher un résultat ne suffit pas toujours. Souvent, vous voulez le **garder** pour
vous en servir ensuite, comme le nombre que vous lisez sur une calculatrice avant de
continuer votre calcul.

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

Ligne par ligne :

- `func carre(x int) int` : la fonction reçoit un `int` et promet de rendre un `int` ;
- `return x * x` calcule le résultat et le rend à celui qui a appelé ;
- `carre(4)` vaut donc 16, et `fmt.Println` l'affiche ;
- `carre(3) + 1` donne `9 + 1` : la variable `resultat` vaut 10.

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

**À vous :** écrivez une fonction `perimetre(cote int) int` qui renvoie le périmètre
d'un carré (4 fois le côté). Dans `main`, affichez `perimetre(5)`, puis
`perimetre(2) + 1`.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func perimetre(cote int) int {
> 	return 4 * cote
> }
>
> func main() {
> 	fmt.Println(perimetre(5))
> 	fmt.Println(perimetre(2) + 1)
> }
> ```
>
> ```sortie
> 20
> 9
> ```

## Plusieurs valeurs de retour

Certaines questions ont deux réponses. Partager 17 bonbons entre 5 enfants donne un
nombre de bonbons par enfant, et un reste. Go peut rendre les deux d'un coup.

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

Ligne par ligne :

- `(int, int)` annonce que `diviser` rend deux entiers ;
- `return a / b, a % b` rend le quotient puis le reste, séparés par une virgule ;
- `quotient, reste := diviser(17, 5)` range 3 dans `quotient` et 2 dans `reste` ;
- `q, _ := diviser(20, 3)` garde le quotient, 6, et jette le reste.

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

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

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

**Question 1 :** écrivez une fonction `moyenne(a, b int) float64` qui renvoie la
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

**Question 2 :** quelle est la différence entre un paramètre et un argument ?

> **Réponse**
>
> Le paramètre est le nom écrit dans `func` (`prenom string`). L'argument est la
> valeur donnée à l'appel (`"Alice"`).

**Question 3 :** que fait le `_` dans `q, _ := diviser(20, 3)` ?

> **Réponse**
>
> Il jette la deuxième valeur renvoyée, le reste. Go demande une variable par valeur
> renvoyée : `_` prend la place de celle dont vous n'avez pas besoin.

## En résumé

- Une fonction s'écrit `func nom(paramètre type) typeRetour { ... }` et renvoie avec
  `return`.
- Le type d'un paramètre s'écrit après son nom ; `largeur, hauteur int` donne le même
  type aux deux.
- `return` rend la valeur et arrête la fonction tout de suite.
- Une fonction peut renvoyer plusieurs valeurs ; la dernière est souvent une `error`.

Suite : [aller plus loin avec les fonctions](fonctions-avancees.md). Comparez avec
[les fonctions en Rust](../rust/fonctions.md).
