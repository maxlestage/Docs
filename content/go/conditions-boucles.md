# Les conditions

Cette page montre comment comparer des valeurs et faire des choix avec `if` et `switch`.

> **Objectifs :**
>
> - vous saurez comparer des valeurs et combiner des conditions avec `&&`, `||` et `!` ;
> - vous saurez écrire une condition avec `if`, `else if` et `else` ;
> - vous saurez choisir parmi plusieurs cas avec `switch`.

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

**Question 2 :** quelle est la différence entre `=` et `==` ?

> **Réponse**
>
> `=` range une valeur dans une variable. `==` compare deux valeurs et donne `true`
> ou `false`.

**Question 3 :** qu'affiche `fmt.Println(5 > 3 && 2 > 4)` ?

> **Réponse**
>
> `false`. `5 > 3` est vrai, mais `2 > 4` est faux, et `&&` demande que les deux soient
> vraies.

## En résumé

- `==`, `!=`, `<`, `>`, `<=` et `>=` comparent ; `&&`, `||` et `!` combinent les
  conditions.
- `if`, `else if` et `else` font des choix ; les accolades sont obligatoires.
- `if x := ...; condition {` crée une variable qui ne vit que dans le `if`.
- `switch` choisit parmi plusieurs cas, sans `break` ; sans valeur, il teste des
  conditions.

Suite : [les boucles](boucles.md).
