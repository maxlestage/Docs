# Les pièges du débutant en Go

Cette page rassemble onze pièges où presque tous les débutants en Go tombent, avec pour chacun ce qui se passe, pourquoi, et comment l'éviter.

> **Objectifs :**
>
> - reconnaître les messages d'erreur les plus courants de Go ;
> - comprendre pourquoi un programme qui compile donne parfois un résultat faux ;
> - savoir corriger chaque piège en une ou deux lignes.

## Comment lire cette page

Chaque piège suit le même chemin :

```text
 Ce que vous pensez  →  Ce qui se passe  →  Pourquoi  →  La correction
```

Il y a deux sortes de pièges :

- ceux que le [compilateur](../debuter/glossaire.md#compilateur) refuse : un message
  d'erreur apparaît, le programme ne se lance pas ;
- ceux qui compilent, mais donnent un **résultat faux**. Ce sont les plus sournois.

Pas besoin de tout lire d'un coup. Revenez ici quand un programme vous surprend.

## Les pièges du compilateur

### 1. Une variable ou un import qui ne sert à rien

**Vous pensez :** « Je prépare une variable, je m'en servirai plus tard. »

**Ce qui se passe :** Go refuse de compiler.

```go norun
package main

import (
	"fmt"
	"strings"
)

func main() {
	nom := "Alice"
	age := 12
	fmt.Println(nom)
}
```

```text
# command-line-arguments
./main.go:5:2: "strings" imported and not used
./main.go:10:2: declared and not used: age
```

**Pourquoi :** en Go, tout ce qui est créé doit servir. Une
[variable](../debuter/glossaire.md#variable) oubliée cache souvent une faute de frappe.
Go préfère vous prévenir tout de suite.

**La correction :** supprimez ce qui ne sert pas. Ou servez-vous-en.

```go
package main

import "fmt"

func main() {
	nom := "Alice"
	age := 12
	fmt.Println(nom, age)
}
```

```sortie
Alice 12
```

> **Astuce :** pour garder une valeur sans l'utiliser, donnez-la à `_` (le « tiroir
> poubelle ») : `_ = age`. C'est pratique pendant un essai, mais à enlever ensuite.

### 2. Confondre := et =

**Vous pensez :** « `:=` et `=` font la même chose. »

**Ce qui se passe :** pas du tout.

- `:=` **crée** une nouvelle variable. C'est coller une étiquette sur une boîte neuve.
- `=` **change** le contenu d'une boîte qui existe déjà.

Premier cas : `:=` deux fois pour le même nom.

```go norun
package main

import "fmt"

func main() {
	total := 0
	total := 10
	fmt.Println(total)
}
```

```text
# command-line-arguments
./main.go:7:8: no new variables on left side of :=
```

Le message veut dire : « à gauche de `:=`, il n'y a rien de nouveau à créer ».

Deuxième cas : `=` pour une variable qui n'existe pas encore.

```go norun
package main

import "fmt"

func main() {
	score = 5
	fmt.Println(score)
}
```

```text
# command-line-arguments
./main.go:6:2: undefined: score
./main.go:7:14: undefined: score
```

`undefined` veut dire « je ne connais pas ce nom ».

**La correction :** `:=` la première fois, `=` les fois suivantes.

```go
package main

import "fmt"

func main() {
	score := 5
	score = score + 1
	fmt.Println(score)
}
```

```sortie
6
```

> **À retenir :** `:=` = nouvelle boîte. `=` = changer ce qu'il y a dans la boîte.

### 3. Changer une lettre d'un texte

**Vous pensez :** « Un texte est une suite de lettres, je peux en changer une. »

```go norun
package main

import "fmt"

func main() {
	mot := "café"
	mot[0] = 'C'
	fmt.Println(mot)
}
```

```text
# command-line-arguments
./main.go:7:2: cannot assign to mot[0] (neither addressable nor a map index expression)
```

**Pourquoi :** en Go, un texte (`string`) ne se modifie jamais. On dit qu'il est
[immuable](../debuter/glossaire.md#immuable).

**La correction :** fabriquez un **nouveau** texte.

```go
package main

import (
	"fmt"
	"strings"
)

func main() {
	mot := "café"
	mot = "C" + mot[1:]
	fmt.Println(mot)
	fmt.Println(strings.ToUpper("café"))
}
```

```sortie
Café
CAFÉ
```

## Les pièges du calcul

### 4. La division entière

**Vous pensez :** « 7 divisé par 2, ça fait 3,5. »

```go
package main

import "fmt"

func main() {
	fmt.Println(7 / 2)
	note1, note2 := 12, 15
	moyenne := (note1 + note2) / 2
	fmt.Println(moyenne)
}
```

```sortie
3
13
```

**Pourquoi :** entre deux [entiers](../debuter/glossaire.md#entier), `/` donne un
entier. Go **coupe** la partie après la virgule (il n'arrondit pas). 27 / 2 donne 13,
pas 13,5.

**La correction :** convertissez en `float64` **avant** de diviser.

```go
package main

import "fmt"

func main() {
	note1, note2 := 12, 15
	moyenne := float64(note1+note2) / 2
	fmt.Println(moyenne)
}
```

```sortie
13.5
```

Essayez : écrivez `float64((note1 + note2) / 2)`. Vous obtenez `13`. La division a
eu lieu **avant** la conversion : trop tard, la partie 0,5 est déjà perdue.

Voir [les nombres](nombres.md#la-division-entiere-et-le-reste).

### 5. Ignorer l'erreur

**Vous pensez :** « La conversion du texte en nombre va bien se passer. »

```go
package main

import (
	"fmt"
	"strconv"
)

func main() {
	n, _ := strconv.Atoi("12a")
	fmt.Println(n * 2)
}
```

```sortie
0
```

**Pourquoi :** `"12a"` n'est pas un nombre. `strconv.Atoi` renvoie alors `0` **et** une
[erreur](../debuter/glossaire.md#erreur). Avec `_`, vous avez jeté l'erreur à la
poubelle. Le programme continue avec un `0` faux, sans rien dire.

**La correction :** testez toujours `err`.

```go
package main

import (
	"fmt"
	"strconv"
)

func main() {
	n, err := strconv.Atoi("12a")
	if err != nil {
		fmt.Println("Erreur :", err)
		return
	}
	fmt.Println(n * 2)
}
```

```sortie
Erreur : strconv.Atoi: parsing "12a": invalid syntax
```

> **À retenir :** quand une fonction renvoie `err`, la ligne suivante est presque
> toujours `if err != nil { ... }`.

## Les pièges des boucles et des blocs

### 6. Une variable qui en cache une autre

**Vous pensez :** « Je change `message` dans le `if`. »

```go
package main

import "fmt"

func main() {
	age := 20
	message := "mineur"
	if age >= 18 {
		message := "majeur"
		fmt.Println("dans le if :", message)
	}
	fmt.Println("après le if :", message)
}
```

```sortie
dans le if : majeur
après le if : mineur
```

**Pourquoi :** dans le `if`, `message :=` crée une **deuxième** variable, avec le même
nom. Elle cache la première, puis disparaît à la fin du bloc `}`. On parle de
**masquage**. La zone où un nom existe s'appelle sa
[portée](../debuter/glossaire.md#portee).

```text
main  ┌─────────────────────────────┐
      │ message = "mineur"          │
      │   if  ┌───────────────────┐ │
      │       │ message = "majeur"│ │  ← une autre boîte
      │       └───────────────────┘ │  ← elle disparaît ici
      │ message vaut toujours "mineur"
      └─────────────────────────────┘
```

**La correction :** un seul caractère. Remplacez `:=` par `=` dans le `if`.

```go
package main

import "fmt"

func main() {
	age := 20
	message := "mineur"
	if age >= 18 {
		message = "majeur"
	}
	fmt.Println(message)
}
```

```sortie
majeur
```

### 7. range donne une copie

**Vous pensez :** « Je multiplie chaque nombre de la slice par 10. »

```go
package main

import "fmt"

func main() {
	nombres := []int{1, 2, 3}
	for _, n := range nombres {
		n = n * 10
	}
	fmt.Println(nombres)
}
```

```sortie
[1 2 3]
```

**Pourquoi :** `n` est une **copie** de chaque case, comme une photocopie. Vous écrivez
sur la photocopie, pas sur l'original.

| Tour | n avant | n après | nombres |
|---|---|---|---|
| 1 | 1 | 10 | [1 2 3] |
| 2 | 2 | 20 | [1 2 3] |
| 3 | 3 | 30 | [1 2 3] |

**La correction :** passez par la position `i` pour écrire dans la vraie case.

```go
package main

import "fmt"

func main() {
	nombres := []int{1, 2, 3}
	for i := range nombres {
		nombres[i] = nombres[i] * 10
	}
	fmt.Println(nombres)
}
```

```sortie
[10 20 30]
```

## Les pièges des slices, des maps et des textes

### 8. Deux slices qui partagent les mêmes cases

**Vous pensez :** « `debut` est une nouvelle slice. Y ajouter un élément ne touche pas à
`base`. »

```go
package main

import "fmt"

func main() {
	base := []int{1, 2, 3, 4}
	debut := base[:2]
	debut = append(debut, 99)
	fmt.Println("debut :", debut)
	fmt.Println("base  :", base)
}
```

```sortie
debut : [1 2 99]
base  : [1 2 99 4]
```

**Pourquoi :** une [slice](../debuter/glossaire.md#slice) est une **fenêtre** sur un
tableau caché. `base[:2]` ne copie rien : c'est une petite fenêtre sur les mêmes cases.
Comme il reste de la place derrière, `append` écrit 99 dans la case 2… qui appartient
aussi à `base`.

```text
tableau caché : [ 1 | 2 | 3 | 4 ]
base          :  └───────────────┘
debut         :  └─────┘
après append  : [ 1 | 2 | 99| 4 ]   ← la case 2 est écrasée
```

**La correction :** faites une vraie copie avec `slices.Clone`.

```go
package main

import (
	"fmt"
	"slices"
)

func main() {
	base := []int{1, 2, 3, 4}
	debut := slices.Clone(base[:2])
	debut = append(debut, 99)
	fmt.Println("debut :", debut)
	fmt.Println("base  :", base)
}
```

```sortie
debut : [1 2 99]
base  : [1 2 3 4]
```

Plus de détails dans [le piège du tableau partagé](collections.md#le-piege-du-tableau-partage).

### 9. Écrire dans une map nil

**Vous pensez :** « J'ai déclaré ma map, je peux la remplir. »

```go norun
package main

import "fmt"

func main() {
	var stock map[string]int
	stock["pommes"] = 5
	fmt.Println(stock)
}
```

```text
panic: assignment to entry in nil map

goroutine 1 [running]:
main.main()
	/chemin/vers/main.go:7 +0x28
exit status 2
```

**Pourquoi :** `var stock map[string]int` crée une étiquette, mais pas de boîte. La map
vaut `nil` (« rien »). Écrire dedans provoque une
[panique](../debuter/glossaire.md#panique) : le programme s'arrête net.

**La correction :** créez la map avec `make` (ou avec `{}`).

```go
package main

import "fmt"

func main() {
	stock := make(map[string]int)
	stock["pommes"] = 5
	fmt.Println(stock)
}
```

```sortie
map[pommes:5]
```

### 10. L'ordre d'une map change à chaque fois

**Vous pensez :** « La map s'affichera dans l'ordre où j'ai mis les clés. »

```go norun
package main

import "fmt"

func main() {
	ages := map[string]int{"Zoé": 8, "Alice": 12, "Marc": 10}
	for prenom, age := range ages {
		fmt.Println(prenom, age)
	}
}
```

Lancez ce programme plusieurs fois. L'ordre change :

```text
1er lancement     2e lancement
Marc 10           Alice 12
Zoé 8             Zoé 8
Alice 12          Marc 10
```

**Pourquoi :** Go mélange **exprès** l'ordre de `range` sur une map. Ainsi, personne ne
compte dessus par erreur.

**La correction :** triez les clés, puis parcourez-les.

```go
package main

import (
	"fmt"
	"maps"
	"slices"
)

func main() {
	ages := map[string]int{"Zoé": 8, "Alice": 12, "Marc": 10}
	for _, prenom := range slices.Sorted(maps.Keys(ages)) {
		fmt.Println(prenom, ages[prenom])
	}
}
```

```sortie
Alice 12
Marc 10
Zoé 8
```

> **Note :** `fmt.Println(ages)` affiche déjà les clés triées. Le piège ne concerne que
> `range`.

### 11. Un texte se mesure en octets, pas en lettres

**Vous pensez :** « `"été"` a 3 lettres, donc `len` vaut 3 et `mot[0]` vaut `é`. »

```go
package main

import "fmt"

func main() {
	mot := "été"
	fmt.Println(len(mot))
	fmt.Println(mot[0])
	fmt.Println(string(mot[0]))
}
```

```sortie
5
195
Ã
```

**Pourquoi :** Go range le texte en [octets](../debuter/glossaire.md#octet) (en UTF-8,
voir [Unicode](../debuter/glossaire.md#unicode)). Une lettre simple comme `t` prend 1
octet. Une lettre accentuée comme `é` en prend **2**. `mot[0]` donne le premier
**octet**, pas la première lettre.

```text
lettres :  é       t    é
octets  : [195 169][116][195 169]   → 5 octets
```

**La correction :** passez par les **runes** (une rune = une lettre).

```go
package main

import "fmt"

func main() {
	mot := "été"
	lettres := []rune(mot)
	fmt.Println(len(lettres))
	fmt.Println(string(lettres[0]))
	for _, r := range mot {
		fmt.Print(string(r), " ")
	}
	fmt.Println()
}
```

```sortie
3
é
é t é 
```

`range` sur un texte donne bien les lettres une par une. Voir
[octets et runes](texte.md#octets-et-runes).

> **À retenir :** `len(texte)` compte des octets. `len([]rune(texte))` compte des
> lettres.

## Erreurs fréquentes

Toute la page parle d'erreurs. Voici encore deux messages que vous verrez souvent.

### Lire une case qui n'existe pas

```go norun
package main

import "fmt"

func main() {
	notes := []int{12, 8, 15}
	fmt.Println(notes[3])
}
```

```text
panic: runtime error: index out of range [3] with length 3

goroutine 1 [running]:
main.main()
	/chemin/vers/main.go:7 +0x15
exit status 2
```

Trois cases, numérotées 0, 1 et 2. La case 3 n'existe pas. La dernière case est
toujours `notes[len(notes)-1]`.

### Mélanger un int et un float64

```go norun
package main

import "fmt"

func main() {
	somme := 27.0
	n := 2
	fmt.Println(somme / n)
}
```

```text
# command-line-arguments
./main.go:8:14: invalid operation: somme / n (mismatched types float64 and int)
```

Go ne mélange jamais deux types de nombres tout seul. Convertissez vous-même :
`somme / float64(n)`.

## Testez-vous

**Question 1 :** qu'affiche ce programme ?

```go norun
package main

import "fmt"

func main() {
	total := 0
	for _, n := range []int{1, 2, 3} {
		total := total + n
		fmt.Print(total, " ")
	}
	fmt.Println(total)
}
```

> **Réponse**
>
> `1 2 3 0`. Dans la boucle, `total :=` crée une nouvelle variable à chaque tour (piège
> 6). Elle vaut `0 + n`. Le `total` du dehors n'a jamais changé : il vaut toujours 0.
> Correction : `total = total + n`, et le programme affiche `1 3 6 6`.

**Question 2 :** pourquoi `fmt.Println(9 / 4)` affiche-t-il `2` ?

> **Réponse**
>
> 9 et 4 sont des entiers : la division est entière, la partie après la virgule est
> coupée. Pour obtenir `2.25`, écrivez `9.0 / 4` ou `float64(9) / 4`.

**Question 3 :** que vaut `len("Noël")` ?

> **Réponse**
>
> `5`. N, o et l prennent 1 octet chacun, `ë` en prend 2. Pour compter les lettres :
> `len([]rune("Noël"))`, qui vaut 4.

## En résumé

- Go refuse les variables et les imports inutilisés : supprimez-les.
- `:=` crée une variable, `=` la change ; un `:=` dans un bloc peut cacher une variable.
- Entre deux entiers, `/` coupe la virgule : convertissez en `float64` avant.
- Testez toujours `err` ; `range` donne une copie ; une map se crée avec `make`.
- Deux slices peuvent partager leurs cases ; l'ordre d'une map est aléatoire.
- Un texte se mesure en octets : utilisez `[]rune` pour compter les lettres.

Suite : [Exercices pour bien démarrer](exercices-faciles.md).
