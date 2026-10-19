# Les boucles

Cette page montre comment répéter des actions avec `for`, la seule boucle de Go, et comment parcourir une liste, un texte ou des nombres avec `range`.

> **Objectifs :**
>
> - vous saurez écrire les trois formes de la boucle `for` ;
> - vous saurez parcourir une liste, un texte ou des nombres avec `range` ;
> - vous saurez arrêter ou sauter un tour avec `break` et `continue`.

Ce chapitre suit [les conditions](conditions-boucles.md) : vous y avez appris à poser une question avec `if` ; ici, vous allez répéter des actions tant qu'une condition est vraie.

## La boucle for

Imaginez qu'on vous demande d'afficher les nombres de 1 à 100. Écrire 100
`fmt.Println` serait long et pénible. Une boucle fait ce travail répétitif à votre
place, comme le refrain d'une chanson qu'on reprend plusieurs fois.

Une [boucle](../debuter/glossaire.md#boucle) répète un bloc de code. Go n'a **qu'une
seule** boucle : `for`. Pas de `while`, pas de `do`. Mais `for` s'écrit de trois façons.

### Forme 1 : le compteur

La forme classique a trois parties, séparées par des `;` :

```go
package main

import "fmt"

func main() {
	for i := 1; i <= 3; i++ {
		fmt.Println("Tour", i)
	}
}
```

```sortie
Tour 1
Tour 2
Tour 3
```

1. `i := 1` : le **départ**, exécuté une seule fois ;
2. `i <= 3` : la **condition**, testée avant chaque tour ; si elle est fausse, la boucle
   s'arrête ;
3. `i++` : le **pas**, exécuté à la fin de chaque tour.

### Forme 2 : tant que

Avec seulement une condition, `for` joue le rôle du « tant que » (`while` dans d'autres
langages).

```go
package main

import "fmt"

func main() {
	argent := 100
	annees := 0
	for argent < 150 {
		argent = argent * 110 / 100
		annees++
	}
	fmt.Println(annees, "ans, il y a", argent, "euros")
}
```

```sortie
5 ans, il y a 160 euros
```

Ligne par ligne :

- au départ, `argent` vaut 100 et `annees` vaut 0 ;
- `for argent < 150 {` pose la question avant chaque tour. 100 est plus petit que
  150 : Go entre dans le bloc ;
- `argent * 110 / 100` ajoute 10 % : 100 devient 110, puis 121, 133, 146 et 160 (Go
  calcule avec des nombres entiers et laisse tomber les décimales) ;
- `annees++` ajoute 1 à `annees` à chaque tour : il compte les tours ;
- quand `argent` vaut 160, la réponse est non : la boucle s'arrête et la dernière
  ligne affiche le résultat.

On ne sait pas d'avance combien de tours il faudra : la boucle tourne tant que
`argent < 150`.

**À vous :** écrivez un programme qui part de `n := 1` et qui, tant que `n` est plus
petit que 100, affiche `n` puis le double.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	n := 1
> 	for n < 100 {
> 		fmt.Println(n)
> 		n = n * 2
> 	}
> }
> ```
>
> ```sortie
> 1
> 2
> 4
> 8
> 16
> 32
> 64
> ```

### Forme 3 : la boucle infinie

Sans rien du tout, `for` tourne pour toujours. On en sort avec `break`.

```go
package main

import "fmt"

func main() {
	n := 1
	for {
		n = n * 2
		if n > 50 {
			break
		}
	}
	fmt.Println(n)
}
```

```sortie
64
```

> **Attention :** si la condition ne devient jamais fausse (ou si `break` n'arrive
> jamais), le programme tourne sans fin. Appuyez sur `Ctrl+C` dans le terminal pour
> l'arrêter.

## Parcourir avec range

Très souvent, vous avez une liste de choses à traiter une par une : chaque article
d'une liste de courses, chaque note d'un bulletin. `range` vous les donne dans
l'ordre, sans que vous ayez à compter vous-même.

`range` parcourt les éléments d'une collection, un par un. À chaque tour, il donne
l'**indice** (la position) et la valeur.

### Une slice

```go
package main

import "fmt"

func main() {
	fruits := []string{"pomme", "kiwi", "poire"}
	for i, fruit := range fruits {
		fmt.Println(i, fruit)
	}
}
```

```sortie
0 pomme
1 kiwi
2 poire
```

Ligne par ligne :

- `fruits` contient une liste de trois textes ;
- au premier tour, `range` range la position 0 dans `i` et `"pomme"` dans `fruit` ;
- le bloc affiche les deux. Puis Go revient au `for`, qui donne 1 et `"kiwi"`, puis
  2 et `"poire"` ;
- après `"poire"`, il n'y a plus d'élément : la boucle s'arrête.

`[]string{...}` est une **slice**, une liste de valeurs : vous la découvrirez dans
[tableaux et slices](collections.md). Les positions commencent à 0.

Si vous n'avez pas besoin de l'indice, remplacez-le par `_` :

```go
package main

import "fmt"

func main() {
	notes := []int{12, 15, 9}
	total := 0
	for _, note := range notes {
		total += note
	}
	fmt.Println("Total :", total)
}
```

```sortie
Total : 36
```

**À vous :** écrivez un programme qui parcourt la liste de courses
`[]string{"pain", "lait", "œufs"}` et affiche chaque article précédé d'un tiret.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	courses := []string{"pain", "lait", "œufs"}
> 	for _, article := range courses {
> 		fmt.Println("-", article)
> 	}
> }
> ```
>
> ```sortie
> - pain
> - lait
> - œufs
> ```

### Un texte

Sur une chaîne, `range` donne la position (en octets) et chaque caractère, sous forme de
`rune`.

```go
package main

import "fmt"

func main() {
	for i, c := range "Go!" {
		fmt.Printf("%d:%c ", i, c)
	}
	fmt.Println()
}
```

```sortie
0:G 1:o 2:! 
```

### Des entiers

Depuis Go 1.22, `range` accepte un entier : `range 3` donne 0, 1, puis 2. C'est le plus
court pour répéter une action.

```go
package main

import "fmt"

func main() {
	for i := range 3 {
		fmt.Println("Bip", i)
	}
	for range 2 {
		fmt.Println("Hourra !")
	}
}
```

```sortie
Bip 0
Bip 1
Bip 2
Hourra !
Hourra !
```

## break et continue

Parfois, vous voulez changer le cours d'une boucle en route : arrêter de chercher dès
que vous avez trouvé vos clés, ou ignorer certains éléments. C'est le rôle de ces
deux mots.

- `break` sort de la boucle tout de suite ;
- `continue` saute la fin du tour et passe au suivant.

```go
package main

import "fmt"

func main() {
	for i := range 10 {
		if i%2 == 0 {
			continue
		}
		if i > 7 {
			break
		}
		fmt.Println(i)
	}
}
```

```sortie
1
3
5
7
```

Ligne par ligne :

- `for i := range 10 {` donne à `i` les valeurs 0, 1, 2… jusqu'à 9 ;
- si `i` est pair (le reste de la division par 2, `i%2`, vaut 0), `continue` passe
  tout de suite au tour suivant : le `fmt.Println` n'est pas atteint ;
- si `i` dépasse 7, `break` arrête toute la boucle ;
- sinon, `fmt.Println(i)` affiche le nombre.

Les nombres pairs sont sautés par `continue`. Quand `i` vaut 9, `break` arrête tout.

### Les étiquettes

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

Dans deux boucles l'une dans l'autre, `break` ne sort que de la boucle intérieure. Pour
sortir des deux, donnez un nom (une **étiquette**) à la boucle extérieure.

```go
package main

import "fmt"

func main() {
recherche:
	for ligne := 1; ligne <= 3; ligne++ {
		for col := 1; col <= 3; col++ {
			if ligne*col == 6 {
				fmt.Println("Trouvé :", ligne, "x", col)
				break recherche
			}
		}
	}
	fmt.Println("Fin")
}
```

```sortie
Trouvé : 2 x 3
Fin
```

`break recherche` sort de la boucle nommée `recherche`. `continue recherche` existe
aussi : il passe au tour suivant de la boucle extérieure.

## Erreurs fréquentes

### Écrire while

```go norun
package main

import "fmt"

func main() {
	i := 0
	while i < 3 {
		fmt.Println(i)
		i++
	}
}
```

```text
# command-line-arguments
./main.go:7:8: syntax error: unexpected name i at end of statement
./main.go:11:1: syntax error: non-declaration statement outside function body
```

`while` n'existe pas en Go. Écrivez `for i < 3 {`.

### Utiliser le compteur après la boucle

```go norun
package main

import "fmt"

func main() {
	for i := 0; i < 3; i++ {
		fmt.Println(i)
	}
	fmt.Println(i)
}
```

```text
# command-line-arguments
./main.go:9:14: undefined: i
```

`i` n'existe que dans la boucle. Pour la garder, créez-la avant :
`i := 0` puis `for ; i < 3; i++ {`.

## Testez-vous

**Question 1 :** affichez la table de 7, de `7 x 1 = 7` à `7 x 5 = 35`.

> **Réponse**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	for i := 1; i <= 5; i++ {
> 		fmt.Printf("7 x %d = %d\n", i, 7*i)
> 	}
> }
> ```
>
> ```sortie
> 7 x 1 = 7
> 7 x 2 = 14
> 7 x 3 = 21
> 7 x 4 = 28
> 7 x 5 = 35
> ```

**Question 2 :** comment écrire en Go une boucle « tant que `n` est plus grand que 1 » ?

> **Réponse**
>
> `for n > 1 { ... }`. Le mot `while` n'existe pas : `for` avec une seule condition
> le remplace.

**Question 3 :** qu'affiche ce programme ?

```go norun
package main

import "fmt"

func main() {
	for i := range 3 {
		fmt.Println(i)
	}
}
```

> **Réponse**
>
> `0`, `1` puis `2`, chacun sur sa ligne. `range 3` commence à 0 et s'arrête avant 3.

## En résumé

- `for` est la seule boucle : avec compteur, avec une condition, ou infinie.
- `range` parcourt une slice, un texte ou les entiers de 0 à n-1.
- `break` sort de la boucle, `continue` passe au tour suivant ; une étiquette vise une
  boucle extérieure.

Suite : [les fonctions](fonctions.md).
