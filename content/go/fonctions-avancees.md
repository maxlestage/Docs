# Aller plus loin avec les fonctions

Cette page présente des outils plus avancés pour les fonctions Go : un nombre variable d'arguments, les fonctions rangées dans des variables, les closures, `defer` et la récursion.

> **Objectifs :**
>
> - vous saurez écrire une fonction qui accepte un nombre variable d'arguments ;
> - vous saurez ranger une fonction dans une variable et créer une closure ;
> - vous saurez repousser une action à la fin avec `defer`.

Ce chapitre suit [les fonctions](fonctions.md) : vous y retrouverez `func`, les paramètres et `return`, cette fois pour des usages plus souples.

## Un nombre variable d'arguments

Vous ne savez pas toujours combien de valeurs vous allez recevoir : deux notes
aujourd'hui, dix demain. Plutôt que d'écrire une fonction par cas, vous en écrivez une
seule, qui accepte toutes les quantités.

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

Ligne par ligne :

- `nombres ...int` : tous les nombres donnés à l'appel arrivent dans la slice
  `nombres` ;
- `total` part de 0, puis la boucle lui ajoute chaque nombre, un par un ;
- `somme(1, 2)` rend 3, `somme(1, 2, 3, 4)` rend 10 ;
- `somme()` ne reçoit rien : la boucle ne tourne pas, et `total` reste à 0.

Pour passer une slice existante, ajoutez `...` après elle : `somme(notes...)`.
`fmt.Println` est d'ailleurs variadique : c'est pour cela qu'elle accepte autant de
valeurs.

**À vous :** écrivez une fonction `saluerTous(prenoms ...string)` qui affiche
« Bonjour, … ! » pour chaque prénom reçu. Appelez-la avec trois prénoms.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func saluerTous(prenoms ...string) {
> 	for _, prenom := range prenoms {
> 		fmt.Println("Bonjour,", prenom, "!")
> 	}
> }
>
> func main() {
> 	saluerTous("Alice", "Bob", "Chloé")
> }
> ```
>
> ```sortie
> Bonjour, Alice !
> Bonjour, Bob !
> Bonjour, Chloé !
> ```

## Les fonctions sont des valeurs

Parfois, vous voulez écrire une fonction générale, puis lui dire **quoi faire**. C'est
comme une perceuse : la même machine perce ou visse, selon l'embout que vous y mettez.

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

Ligne par ligne :

- `operation := double` range la fonction elle-même dans une variable, sans
  l'appeler (pas de parenthèses) ;
- `operation(5)` appelle donc `double(5)`, qui rend 10 ;
- `appliquer` reçoit une slice et une fonction `f`. Elle appelle `f` sur chaque
  nombre et range les résultats dans une nouvelle slice ;
- avec `double`, `[1 2 3]` devient `[2 4 6]`.

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

Il arrive qu'une fonction doive se **souvenir** de quelque chose entre deux appels,
comme le compteur à l'entrée d'un musée, qui ajoute 1 à chaque visiteur.

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

Ligne par ligne :

- `compteur()` crée une variable `n` qui vaut 0, puis rend une fonction anonyme ;
- `suivant` reçoit cette fonction. Elle garde accès à `n`, même après la fin de
  `compteur` ;
- chaque appel de `suivant()` ajoute 1 à `n` et le rend : 1, puis 2, puis 3 ;
- `autre := compteur()` relance `compteur`, qui crée un **nouveau** `n` à 0 : le
  premier appel rend 1.

Imaginez un sac à dos : la fonction renvoyée garde `n` dans son sac. Chaque appel de
`suivant` augmente **son** `n`. `autre` a son propre sac, avec son propre `n`.

## defer : repousser à la fin

Quand vous ouvrez quelque chose, il faut penser à le refermer. C'est comme un pense-bête
« éteindre la lumière » collé sur la porte : vous l'écrivez en arrivant, et il sert
au moment de partir.

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

Ligne par ligne :

- `main` appelle `travailler`, qui affiche d'abord « Ouverture » ;
- la ligne `defer` n'affiche rien tout de suite : elle note « afficher Fermeture » pour
  plus tard ;
- « Travail 1 » puis « Travail 2 » s'affichent ;
- `travailler` arrive à sa fin : l'appel mis de côté s'exécute, et « Fermeture »
  s'affiche en dernier.

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

**À vous :** écrivez un `main` qui affiche « Au revoir » en dernier, alors que vous
écrivez cette ligne en premier. Ajoutez ensuite deux affichages : « Bonjour » et
« Comment allez-vous ? ».

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	defer fmt.Println("Au revoir")
> 	fmt.Println("Bonjour")
> 	fmt.Println("Comment allez-vous ?")
> }
> ```
>
> ```sortie
> Bonjour
> Comment allez-vous ?
> Au revoir
> ```

## Une fonction qui s'appelle elle-même

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

Certains problèmes contiennent une version plus petite d'eux-mêmes, comme des poupées
russes : en ouvrant une poupée, on en trouve une autre, plus petite, jusqu'à la
dernière.

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

### Oublier les ... pour passer une slice

```go norun
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
	notes := []int{12, 15, 9}
	fmt.Println(somme(notes))
}
```

```text
# command-line-arguments
./main.go:15:20: cannot use notes (variable of type []int) as int value in argument to somme
```

`somme` attend des `int` un par un, pas une slice. Ajoutez `...` après la slice :
`somme(notes...)`.

### Appeler une fonction au lieu de la passer

```go norun
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
	fmt.Println(appliquer([]int{1, 2, 3}, double()))
}
```

```text
# command-line-arguments
./main.go:18:40: not enough arguments in call to double
	have ()
	want (int)
```

Avec les parenthèses, `double()` **appelle** la fonction, ici sans argument. Pour
passer la fonction elle-même, écrivez son nom seul : `appliquer([]int{1, 2, 3}, double)`.

### Placer le paramètre variadique avant les autres

```go norun
package main

import "fmt"

func presenter(notes ...int, nom string) {
	fmt.Println(nom, notes)
}

func main() {
	presenter(12, 15, "Alice")
}
```

```text
# command-line-arguments
./main.go:5:22: can only use ... with final parameter in list
```

Le paramètre avec `...` doit être le dernier. Écrivez
`func presenter(nom string, notes ...int)`, puis `presenter("Alice", 12, 15)`.

## Testez-vous

**Question 1 :** vous avez `notes := []int{12, 15, 9}` et une fonction
`somme(nombres ...int) int`. Comment passer `notes` à `somme` ?

> **Réponse**
>
> Ajoutez `...` après la slice : `somme(notes...)`. Sans les `...`, Go refuse : il
> attend des `int`, pas une slice.

**Question 2 :** qu'affiche ce programme ?

```go norun
package main

import "fmt"

func main() {
	total := 0
	ajouter := func(n int) {
		total += n
	}
	ajouter(3)
	ajouter(4)
	fmt.Println(total)
}
```

> **Réponse**
>
> `7`. La fonction anonyme utilise la variable `total` qui l'entoure : chaque appel
> la modifie vraiment.

**Question 3 :** qu'affiche ce programme ?

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

## En résumé

- `...type` accepte un nombre variable d'arguments, reçus dans une slice.
- Une fonction est une valeur : on la range dans une variable, on la passe en argument.
- Une closure garde les variables qui l'entourent.
- `defer` repousse un appel à la fin de la fonction, dans l'ordre inverse.
- Une fonction récursive s'appelle elle-même et a besoin d'un cas d'arrêt.

Suite : [travailler avec du texte](texte.md).
