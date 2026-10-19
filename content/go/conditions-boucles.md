# Conditions et boucles

Cette page montre comment faire des choix avec `if` et `switch`, et comment répéter des actions avec `for`, la seule boucle de Go.

> **Objectifs :**
>
> - vous saurez écrire une condition avec `if`, `else if` et `else` ;
> - vous saurez choisir parmi plusieurs cas avec `switch` ;
> - vous saurez écrire les trois formes de la boucle `for` ;
> - vous saurez parcourir une liste, un texte ou des nombres avec `range` ;
> - vous saurez arrêter ou sauter un tour avec `break` et `continue`.

## Comparer des valeurs

Un programme doit souvent décider : ouvrir la porte ou non, afficher « gagné » ou
« perdu ». Pour décider, il pose d'abord une question, comme vous vous demandez
« ai-je assez d'argent pour ce gâteau ? ».

Une [condition](../debuter/glossaire.md#condition) est une question à laquelle on
répond par vrai (`true`) ou faux (`false`). On la construit avec des opérateurs de
comparaison.

| Opérateur | Sens | Exemple | Résultat |
|-----------|------|---------|----------|
| `==` | égal à | `3 == 3` | `true` |
| `!=` | différent de | `3 != 4` | `true` |
| `<`, `>` | plus petit, plus grand | `2 > 5` | `false` |
| `<=`, `>=` | plus petit ou égal, plus grand ou égal | `5 >= 5` | `true` |

On combine les conditions avec les opérateurs logiques :

- `&&` : « et » (les deux doivent être vraies) ;
- `||` : « ou » (au moins une doit être vraie) ;
- `!` : « non » (inverse vrai et faux).

```go
package main

import "fmt"

func main() {
	age := 15
	avecParent := true
	fmt.Println(age >= 18)
	fmt.Println(age >= 18 || avecParent)
	fmt.Println(age > 12 && age < 18)
	fmt.Println(!avecParent)
}
```

```sortie
false
true
true
false
```

Ligne par ligne :

- `age := 15` range 15 dans `age`, et `avecParent := true` range « vrai » dans
  `avecParent` ;
- `age >= 18` demande « 15 est-il plus grand que 18, ou égal ? ». Non : `false` ;
- avec `||`, une seule réponse vraie suffit : `avecParent` vaut `true`, donc `true` ;
- avec `&&`, il faut les deux : 15 est plus grand que 12 **et** plus petit que 18,
  donc `true` ;
- `!avecParent` inverse `true` : on obtient `false`.

> **Attention :** `=` range une valeur, `==` compare deux valeurs. Ne les confondez pas.

## if et else

Dans la vie, vous faites des choix tout le temps : s'il pleut, vous prenez un
parapluie ; sinon, vous le laissez. `if` donne ce pouvoir à votre programme : faire
une chose ou une autre selon la situation.

`if` (« si ») exécute un bloc seulement si la condition est vraie. `else` (« sinon »)
donne le bloc à exécuter dans le cas contraire.

```go
package main

import "fmt"

func main() {
	temperature := 28
	if temperature > 25 {
		fmt.Println("Il fait chaud.")
	} else {
		fmt.Println("Il fait bon.")
	}
}
```

```sortie
Il fait chaud.
```

Ligne par ligne :

- `temperature := 28` range 28 dans la variable `temperature` ;
- `if temperature > 25 {` pose la question « 28 est-il plus grand que 25 ? ». La
  réponse est oui ;
- Go exécute donc le premier bloc et affiche « Il fait chaud. » ;
- le bloc du `else` est sauté : il ne sert que si la réponse est non.

En Go :

- pas de parenthèses autour de la condition ;
- les accolades `{ }` sont **obligatoires**, même pour une seule ligne ;
- `else` se place sur la même ligne que l'accolade fermante : `} else {`.

### else if

Pour plus de deux cas, enchaînez avec `else if`. Go teste les conditions dans l'ordre et
s'arrête à la première vraie.

```go
package main

import "fmt"

func main() {
	note := 14
	if note >= 16 {
		fmt.Println("Très bien")
	} else if note >= 12 {
		fmt.Println("Bien")
	} else if note >= 10 {
		fmt.Println("Passable")
	} else {
		fmt.Println("Insuffisant")
	}
}
```

```sortie
Bien
```

`note >= 10` est vraie aussi, mais Go ne la teste même pas : il a déjà trouvé
`note >= 12`.

**À vous :** écrivez un programme qui range `10` dans une variable `age`, puis
affiche « Enfant » si l'âge est inférieur à 13, « Adolescent » s'il est inférieur
à 18, et « Adulte » sinon.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	age := 10
> 	if age < 13 {
> 		fmt.Println("Enfant")
> 	} else if age < 18 {
> 		fmt.Println("Adolescent")
> 	} else {
> 		fmt.Println("Adulte")
> 	}
> }
> ```
>
> ```sortie
> Enfant
> ```

### if avec une instruction d'initialisation

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

Go permet de placer une petite instruction **avant** la condition, séparée par un `;`.
La variable créée là n'existe que dans le `if` et ses `else`.

```go
package main

import (
	"fmt"
	"strconv"
)

func main() {
	if n, err := strconv.Atoi("42"); err != nil {
		fmt.Println("Pas un nombre :", err)
	} else {
		fmt.Println("Le double vaut", n*2)
	}
}
```

```sortie
Le double vaut 84
```

Cette forme est très courante en Go, surtout pour tester une erreur. Elle garde les
variables temporaires là où elles servent.

## switch

Pensez à un distributeur de boissons : selon le bouton choisi, il sert une boisson
différente. `switch` fonctionne pareil : selon la valeur, il choisit un bloc.

Quand une même valeur peut prendre plusieurs formes, `switch` est plus lisible qu'une
longue suite de `else if`.

```go
package main

import "fmt"

func main() {
	jour := "samedi"
	switch jour {
	case "samedi", "dimanche":
		fmt.Println("C'est le week-end !")
	case "mercredi":
		fmt.Println("Pas d'école l'après-midi.")
	default:
		fmt.Println("Au travail.")
	}
}
```

```sortie
C'est le week-end !
```

Ligne par ligne :

- `jour` contient le texte `"samedi"` ;
- `switch jour {` compare la valeur de `jour` à chaque `case`, de haut en bas ;
- le premier `case` liste `"samedi"` et `"dimanche"` : `"samedi"` correspond,
  donc Go affiche « C'est le week-end ! » ;
- les autres cas, et `default`, sont ignorés.

Retenez trois choses sur `switch` :

- Chaque `case` donne une ou plusieurs valeurs, séparées par des virgules.
- `default` (« par défaut ») s'exécute si aucun cas ne correspond.
- Go s'arrête au **premier** cas qui correspond, puis sort du `switch`.

> **Note :** contrairement au C ou à Java, pas besoin de `break` à la fin de chaque
> cas : Go ne « tombe » pas dans le cas suivant. Pour le forcer, il existe le mot
> `fallthrough`, mais on ne s'en sert presque jamais.

**À vous :** écrivez un programme qui range `"citron"` dans une variable `fruit`,
puis, avec un `switch`, affiche « Fruit du verger » pour une pomme ou une poire,
« Agrume » pour un citron ou une orange, et « Fruit inconnu » sinon.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	fruit := "citron"
> 	switch fruit {
> 	case "pomme", "poire":
> 		fmt.Println("Fruit du verger")
> 	case "citron", "orange":
> 		fmt.Println("Agrume")
> 	default:
> 		fmt.Println("Fruit inconnu")
> 	}
> }
> ```
>
> ```sortie
> Agrume
> ```

### switch sans valeur

Un `switch` sans valeur teste des conditions, comme un `switch true`. C'est une belle
façon d'écrire une suite de `else if`.

```go
package main

import "fmt"

func main() {
	heure := 14
	switch {
	case heure < 12:
		fmt.Println("Bonjour")
	case heure < 18:
		fmt.Println("Bon après-midi")
	default:
		fmt.Println("Bonsoir")
	}
}
```

```sortie
Bon après-midi
```

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

### Mettre un nombre comme condition

```go norun
package main

import "fmt"

func main() {
	n := 3
	if n {
		fmt.Println("vrai")
	}
}
```

```text
# command-line-arguments
./main.go:7:5: non-boolean condition in if statement
```

En Go, une condition doit être un vrai booléen. `3` n'est ni vrai ni faux. Écrivez la
comparaison : `if n != 0 {`.

### Placer else sur une nouvelle ligne

```go norun
package main

import "fmt"

func main() {
	age := 20
	if age >= 18 {
		fmt.Println("majeur")
	}
	else {
		fmt.Println("mineur")
	}
}
```

```text
# command-line-arguments
./main.go:10:2: syntax error: unexpected keyword else, expected }
```

`else` doit suivre directement l'accolade fermante : `} else {`.

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

**Question 1 :** qu'affiche ce programme ?

```go norun
package main

import "fmt"

func main() {
	x := 7
	switch {
	case x > 5:
		fmt.Println("grand")
	case x > 2:
		fmt.Println("moyen")
	}
}
```

> **Réponse**
>
> `grand`, seulement. Go s'arrête au premier cas vrai et ne passe pas au suivant.

**Question 2 :** affichez la table de 7, de `7 x 1 = 7` à `7 x 5 = 35`.

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

**Question 3 :** comment écrire en Go une boucle « tant que `n` est plus grand que 1 » ?

> **Réponse**
>
> `for n > 1 { ... }`. Le mot `while` n'existe pas : `for` avec une seule condition
> le remplace.

## En résumé

- `if`, `else if` et `else` font des choix ; les accolades sont obligatoires.
- `if x := ...; condition {` crée une variable qui ne vit que dans le `if`.
- `switch` choisit parmi plusieurs cas, sans `break` ; sans valeur, il teste des
  conditions.
- `for` est la seule boucle : avec compteur, avec une condition, ou infinie.
- `range` parcourt une slice, un texte ou les entiers de 0 à n-1.
- `break` sort de la boucle, `continue` passe au tour suivant ; une étiquette vise une
  boucle extérieure.

Suite : [les fonctions](fonctions.md).
