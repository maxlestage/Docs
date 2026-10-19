# Nombres et calculs

Go propose des entiers et des nombres à virgule : ce chapitre montre comment calculer avec, les convertir, les lire depuis du texte et les afficher proprement.

> **Objectifs :**
>
> - vous saurez choisir entre `int` et `float64` ;
> - vous saurez calculer avec la division entière et le reste `%` ;
> - vous saurez convertir un nombre d'un type à l'autre, et depuis du texte avec `strconv` ;
> - vous saurez utiliser le paquet `math` et tirer des nombres au hasard ;
> - vous saurez afficher un nombre avec le bon nombre de décimales.

## Les entiers

Les entiers servent à compter : des élèves, des années, des points dans un jeu. Dès
qu'une quantité ne se coupe pas en morceaux, c'est un entier.

Un [entier](../debuter/glossaire.md#entier) est un nombre sans virgule : `3`, `-12`,
`2026`. Le type le plus courant est `int`.

```go
package main

import "fmt"

func main() {
	annee := 2026
	eleves := 28
	fmt.Println(annee-2000, eleves*2)
}
```

```sortie
26 56
```

Ligne par ligne :

- `annee := 2026` crée une variable `annee` et y range 2026 ;
- `eleves := 28` fait de même avec 28 ;
- `fmt.Println` calcule d'abord `annee-2000` (26) et `eleves*2` (56), puis affiche les
  deux résultats, séparés par une espace.

### Les tailles d'entiers

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

Go a plusieurs types d'entiers. Leur nom dit deux choses :

- le début : `int` pour un entier **signé** (il peut être négatif), `uint` pour un
  entier **non signé** (*unsigned* : jamais négatif) ;
- le nombre : la taille en [bits](../debuter/glossaire.md#bit). Plus il y a de bits,
  plus le nombre peut être grand.

| Type | Plus petite valeur | Plus grande valeur |
|------|--------------------|--------------------|
| `int8` | -128 | 127 |
| `uint8` (ou `byte`) | 0 | 255 |
| `int16` | -32 768 | 32 767 |
| `int32` (ou `rune`) | -2 147 483 648 | 2 147 483 647 |
| `int64` | environ -9,2 × 10¹⁸ | environ 9,2 × 10¹⁸ |
| `uint64` | 0 | environ 1,8 × 10¹⁹ |
| `int` | comme `int64` sur un ordinateur récent | |

> **À retenir :** utilisez `int` par défaut. Les autres tailles servent dans des cas
> précis : économiser de la mémoire, lire un fichier binaire…

Le tiret bas `_` peut séparer les chiffres pour la lecture :

```go
package main

import "fmt"

func main() {
	population := 8_100_000_000
	fmt.Println(population)
}
```

```sortie
8100000000
```

## Les nombres à virgule

Tout ne se compte pas en nombres entiers : un prix (19,99 €), une taille (1,75 m), une
température (36,6 °C). Pour ces valeurs, il faut des nombres à virgule.

Un nombre à virgule s'appelle un [flottant](../debuter/glossaire.md#flottant). En Go,
on utilise `float64`. Attention : la virgule s'écrit avec un **point**.

```go
package main

import "fmt"

func main() {
	prix := 19.99
	quantite := 3.0
	fmt.Println(prix * quantite)

	a := 0.1
	b := 0.2
	fmt.Println(a + b)
}
```

```sortie
59.97
0.30000000000000004
```

Ligne par ligne :

- `prix := 19.99` range un nombre à virgule dans `prix` : Go lui donne le type
  `float64` ;
- `quantite := 3.0` est aussi un `float64`, grâce au `.0` ;
- le premier `Println` affiche le produit des deux : 59.97 ;
- `a` et `b` valent 0.1 et 0.2, et leur somme donne une petite surprise, expliquée
  juste en dessous.

Le résultat de `a + b` n'est pas tout à fait `0.3`. L'ordinateur range les nombres
à virgule en binaire, et certains nombres n'y tombent pas juste, comme 1/3 en décimal
(0,3333…). Ce n'est pas un bug de Go : tous les langages font pareil.

> **Note :** `fmt.Println(0.1 + 0.2)` affiche bien `0.3`. Les nombres écrits
> directement dans le code sont des **constantes**, que le compilateur calcule de façon
> exacte. Le problème n'apparaît qu'avec des variables.

> **Astuce :** pour de l'argent, comptez en **centimes** avec un `int` : `1999`
> centimes au lieu de `19.99` euros. Les calculs restent exacts.

## Les opérateurs

Un programme calcule sans arrêt : un total à payer, un score, un temps restant. Les
opérateurs sont les touches de sa calculatrice.

| Opérateur | Rôle | Exemple | Résultat |
|-----------|------|---------|----------|
| `+` | addition | `7 + 2` | `9` |
| `-` | soustraction | `7 - 2` | `5` |
| `*` | multiplication | `7 * 2` | `14` |
| `/` | division | `7 / 2` | `3` |
| `%` | reste de la division | `7 % 2` | `1` |

Les règles de priorité sont celles des maths : `*` et `/` avant `+` et `-`. Les
parenthèses passent avant tout.

### La division entière et le reste

Entre deux entiers, `/` donne une **division entière** : la partie après la virgule est
coupée. `%` (prononcé « modulo ») donne le reste.

```go
package main

import "fmt"

func main() {
	fmt.Println(7 / 2)
	fmt.Println(7 % 2)
	fmt.Println(7.0 / 2)

	minutes := 135
	fmt.Println(minutes/60, "h", minutes%60, "min")
}
```

```sortie
3
1
3.5
2 h 15 min
```

Ligne par ligne :

- `7 / 2` : deux entiers, donc une division entière. Le résultat est 3 ;
- `7 % 2` : le reste de cette division, 1 ;
- `7.0 / 2` : `7.0` est un nombre à virgule, donc la division est normale : 3.5 ;
- `minutes/60` donne les heures entières (2), et `minutes%60` les minutes qui restent
  (15).

Imaginez 7 bonbons partagés entre 2 enfants : chacun en a 3 (`7 / 2`), et il en reste
1 (`7 % 2`).

> **Astuce :** `n % 2 == 0` teste si `n` est pair. Le reste est très utile !

**À vous :** écrivez un programme qui range `200` secondes dans une variable, puis
affiche ce temps en minutes et secondes : « 3 min 20 s ».

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	secondes := 200
> 	fmt.Println(secondes/60, "min", secondes%60, "s")
> }
> ```
>
> ```sortie
> 3 min 20 s
> ```

### Raccourcis

```go
package main

import "fmt"

func main() {
	points := 10
	points += 5
	points *= 2
	points--
	fmt.Println(points)
}
```

```sortie
29
```

- `points += 5` est un raccourci pour `points = points + 5` (de même `-=`, `*=`, `/=`) ;
- `points++` ajoute 1, `points--` enlève 1.

> **Note :** en Go, `points++` est une instruction seule sur sa ligne. On ne peut pas
> écrire `x := points++`.

## Le dépassement

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

Un entier a une taille limitée. Si un calcul dépasse la plus grande valeur, le nombre
« fait le tour », comme un compteur kilométrique qui repasse à zéro.

```go
package main

import "fmt"

func main() {
	var n int8 = 127
	n++
	fmt.Println(n)

	var octet uint8 = 255
	octet++
	fmt.Println(octet)
}
```

```sortie
-128
0
```

Go ne vous prévient pas : le résultat est juste faux. Avec un `int` (64 bits), la
limite est si grande que le problème est rare, mais il existe.

> **Attention :** si la valeur est écrite directement dans le code, le compilateur
> vérifie : `var n int8 = 200` est refusé. Le tour du compteur n'arrive que pendant
> l'exécution.

## Les conversions sont explicites

Un programme mélange souvent des quantités entières (3 articles) et des prix à virgule
(2,50 €). Go veut que vous disiez clairement comment les combiner : ainsi, aucun calcul
n'est faussé en silence.

Go ne mélange jamais deux types de nombres tout seul. Pour multiplier un `int` par un
`float64`, vous devez **convertir** l'un des deux, en écrivant le type comme une
fonction : `float64(x)`.

```go
package main

import "fmt"

func main() {
	quantite := 3
	prix := 2.5
	total := float64(quantite) * prix
	fmt.Println(total)

	note := 15.8
	fmt.Println(int(note))
}
```

```sortie
7.5
15
```

- `float64(quantite)` transforme `3` en `3.0`.
- `int(note)` **coupe** la partie décimale : `15.8` devient `15`, pas `16`.

### Calculer une moyenne

C'est le piège classique : la division de deux entiers est entière.

```go
package main

import "fmt"

func main() {
	somme := 47
	nombre := 4
	fmt.Println(somme / nombre)
	fmt.Println(float64(somme) / float64(nombre))
}
```

```sortie
11
11.75
```

**À vous :** écrivez un programme qui calcule la moyenne exacte de quatre notes
entières : 12, 15, 16 et 18.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	somme := 12 + 15 + 16 + 18
> 	fmt.Println(float64(somme) / 4)
> }
> ```
>
> ```sortie
> 15.25
> ```

## Du texte vers un nombre : strconv

Un nombre tapé par l'utilisateur ou lu dans un fichier arrive sous forme de **texte**.
Le paquet `strconv` (*string conversion*) fait les conversions.

- `strconv.Atoi` : texte → `int` (*ASCII to integer*) ;
- `strconv.Itoa` : `int` → texte ;
- `strconv.ParseFloat` : texte → `float64`.

```go
package main

import (
	"fmt"
	"strconv"
)

func main() {
	n, err := strconv.Atoi("42")
	fmt.Println(n+1, err)

	texte := strconv.Itoa(2026)
	fmt.Println("Année " + texte)

	f, err := strconv.ParseFloat("3.75", 64)
	fmt.Println(f*2, err)
}
```

```sortie
43 <nil>
Année 2026
7.5 <nil>
```

Ligne par ligne :

- `strconv.Atoi("42")` transforme le texte `"42"` en nombre : `n` reçoit 42, et `err`
  reçoit `nil` ;
- `n+1` est donc un vrai calcul, qui donne 43 ;
- `strconv.Itoa(2026)` fait le chemin inverse : `texte` contient le texte `"2026"`, que
  `+` colle à `"Année "` ;
- `ParseFloat` lit `"3.75"` comme le nombre 3.75, et `f*2` donne 7.5.

`Atoi` et `ParseFloat` renvoient **deux** valeurs : le nombre, et une erreur. Quand
tout va bien, l'erreur vaut `nil` (« rien »). Le `64` de `ParseFloat` demande un
`float64`.

### Quand le texte n'est pas un nombre

```go
package main

import (
	"fmt"
	"strconv"
)

func main() {
	n, err := strconv.Atoi("douze")
	if err != nil {
		fmt.Println("Erreur :", err)
		return
	}
	fmt.Println(n)
}
```

```sortie
Erreur : strconv.Atoi: parsing "douze": invalid syntax
```

> **À retenir :** testez toujours `err != nil` après une conversion. Les erreurs sont
> détaillées dans [la gestion des erreurs](erreurs.md).

**À vous :** écrivez un programme qui convertit le texte `"17"` en nombre, puis affiche
l'âge de l'année suivante : « L'an prochain : 18 ans ». N'oubliez pas de tester `err`.

> **Solution**
>
> ```go
> package main
>
> import (
> 	"fmt"
> 	"strconv"
> )
>
> func main() {
> 	age, err := strconv.Atoi("17")
> 	if err != nil {
> 		fmt.Println("Erreur :", err)
> 		return
> 	}
> 	fmt.Println("L'an prochain :", age+1, "ans")
> }
> ```
>
> ```sortie
> L'an prochain : 18 ans
> ```

## Le paquet math

Certains calculs sont longs à écrire vous-même : une racine carrée, une puissance, un
arrondi. Le paquet `math` est une boîte à outils déjà remplie : vous prenez l'outil,
sans avoir à le fabriquer.

Le paquet `math` contient les fonctions mathématiques courantes.

```go
package main

import (
	"fmt"
	"math"
)

func main() {
	fmt.Println(math.Sqrt(16))
	fmt.Println(math.Pow(2, 10))
	fmt.Println(math.Abs(-7.5))
	fmt.Println(math.Round(2.5), math.Floor(2.7), math.Ceil(2.1))
	fmt.Println(math.Max(3, 8), math.Min(3, 8))
	fmt.Printf("%.4f\n", math.Pi)
}
```

```sortie
4
1024
7.5
3 2 3
8 3
3.1416
```

| Fonction | Rôle |
|----------|------|
| `math.Sqrt(x)` | racine carrée |
| `math.Pow(x, y)` | x puissance y |
| `math.Abs(x)` | valeur absolue (sans le signe moins) |
| `math.Round(x)` | arrondi au plus proche |
| `math.Floor(x)`, `math.Ceil(x)` | arrondi vers le bas, vers le haut |
| `math.Max(a, b)`, `math.Min(a, b)` | le plus grand, le plus petit |

Ces fonctions travaillent avec des `float64`. Pour deux entiers, Go 1.21 a ajouté les
fonctions `max` et `min`, sans import :

```go
package main

import "fmt"

func main() {
	fmt.Println(max(4, 9, 2), min(4, 9, 2))
}
```

```sortie
9 2
```

## Des nombres au hasard

Un jeu a souvent besoin de surprise : lancer un dé, mélanger des cartes, placer un
ennemi n'importe où. Pour cela, il faut des nombres tirés au hasard.

Le paquet `math/rand` tire des nombres au hasard. `rand.Intn(6)` donne un entier entre
0 et 5 : on ajoute 1 pour simuler un dé.

Un ordinateur ne sait pas vraiment tirer au hasard : il calcule une suite de nombres
qui *semblent* aléatoires, à partir d'un nombre de départ, la **graine** (*seed*).
Avec la même graine, on obtient toujours la même suite. C'est très pratique pour les
tests, et pour ce site : la sortie ci-dessous ne change jamais.

```go
package main

import (
	"fmt"
	"math/rand"
)

func main() {
	r := rand.New(rand.NewSource(1))
	fmt.Println(r.Intn(6)+1, r.Intn(6)+1, r.Intn(6)+1)
	fmt.Println(r.Intn(6)+1, r.Intn(6)+1, r.Intn(6)+1)
	fmt.Printf("%.3f\n", r.Float64())
}
```

```sortie
6 4 6
6 2 1
0.066
```

- `rand.NewSource(1)` crée une source avec la graine `1` ;
- `rand.New(...)` crée un générateur `r` à partir de cette source ;
- `r.Intn(n)` donne un entier de 0 à n-1, `r.Float64()` un nombre entre 0 et 1.

> **Astuce :** dans un vrai jeu, appelez directement `rand.Intn(6)` sans créer de
> source : Go choisit alors une graine différente à chaque lancement.

## Afficher un nombre

Un calcul donne parfois trop de décimales : `3.14159` euros n'a pas de sens sur un
ticket de caisse, où un prix a toujours deux décimales. Vous choisissez donc comment le
nombre s'affiche.

`fmt.Printf` contrôle l'affichage avec ses verbes :

```go
package main

import "fmt"

func main() {
	prix := 3.14159
	fmt.Printf("%.2f\n", prix)
	fmt.Printf("[%8.2f]\n", prix)
	fmt.Printf("[%-6d]\n", 42)
	fmt.Printf("%03d\n", 7)
	fmt.Printf("%e\n", 1234567.0)
	fmt.Printf("%b %x\n", 10, 255)
}
```

```sortie
3.14
[    3.14]
[42    ]
007
1.234567e+06
1010 ff
```

| Format | Effet |
|--------|-------|
| `%.2f` | 2 décimales |
| `%8.2f` | 2 décimales, sur 8 caractères (calé à droite) |
| `%-6d` | entier sur 6 caractères, calé à gauche |
| `%03d` | entier sur 3 chiffres, complété par des zéros |
| `%e` | notation scientifique |
| `%b`, `%x` | binaire, hexadécimal |

Pour obtenir le texte sans l'afficher, utilisez `fmt.Sprintf` avec les mêmes verbes :
`s := fmt.Sprintf("%.2f", prix)`.

**À vous :** écrivez un programme qui calcule le prix de 3 cahiers à 4.5 euros
chacun, puis affiche « Total : 13.50 euros », avec deux décimales.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	quantite := 3
> 	prix := 4.5
> 	total := float64(quantite) * prix
> 	fmt.Printf("Total : %.2f euros\n", total)
> }
> ```
>
> ```sortie
> Total : 13.50 euros
> ```

## Erreurs fréquentes

### Mélanger int et float64

```go norun
package main

import "fmt"

func main() {
	quantite := 3
	prix := 2.5
	fmt.Println(quantite * prix)
}
```

```text
# command-line-arguments
./main.go:8:14: invalid operation: quantite * prix (mismatched types int and float64)
```

Les types ne correspondent pas (*mismatched types*). Convertissez :
`float64(quantite) * prix`.

### Oublier la deuxième valeur de strconv.Atoi

```go norun
package main

import (
	"fmt"
	"strconv"
)

func main() {
	n := strconv.Atoi("42")
	fmt.Println(n)
}
```

```text
# command-line-arguments
./main.go:9:7: assignment mismatch: 1 variable but strconv.Atoi returns 2 values
```

`Atoi` renvoie 2 valeurs : il faut 2 variables. Écrivez `n, err := strconv.Atoi("42")`,
puis testez `err`.

### Diviser par zéro

```go norun
package main

import "fmt"

func main() {
	a := 10
	b := 0
	fmt.Println(a / b)
}
```

```text
panic: runtime error: integer divide by zero

goroutine 1 [running]:
main.main()
	/tmp/tmp.nGOUtnybpF/main.go:8 +0x9
exit status 2
```

Le programme s'arrête net : c'est une **panique**. Vérifiez le diviseur avant :
`if b != 0 { ... }`.

### Une valeur trop grande pour son type

```go norun
package main

import "fmt"

func main() {
	var petit int8 = 200
	fmt.Println(petit)
}
```

```text
# command-line-arguments
./main.go:6:19: cannot use 200 (untyped int constant) as int8 value in variable declaration (overflows)
```

Un `int8` va jusqu'à 127 : 200 déborde (*overflows*). Prenez un type plus grand, comme
`int`.

## Testez-vous

**Question 1 :** que valent `17 / 5` et `17 % 5` ?

> **Réponse**
>
> `3` et `2`. 17 = 5 × 3 + 2 : le quotient est 3, le reste est 2.

**Question 2 :** convertissez le texte `"250"` en nombre, puis affichez sa moitié avec
une décimale.

> **Réponse**
>
> ```go
> package main
>
> import (
> 	"fmt"
> 	"strconv"
> )
>
> func main() {
> 	n, err := strconv.Atoi("250")
> 	if err != nil {
> 		fmt.Println(err)
> 		return
> 	}
> 	fmt.Printf("%.1f\n", float64(n)/2)
> }
> ```
>
> ```sortie
> 125.0
> ```

**Question 3 :** pourquoi `int(9.99)` vaut-il `9` ?

> **Réponse**
>
> La conversion vers `int` coupe la partie décimale, sans arrondir. Pour arrondir,
> utilisez `int(math.Round(9.99))`, qui donne `10`.

## En résumé

- `int` pour les entiers, `float64` pour les nombres à virgule.
- Entre deux entiers, `/` est une division entière ; `%` donne le reste.
- Go ne convertit jamais tout seul : écrivez `float64(x)` ou `int(x)`.
- `strconv.Atoi`, `strconv.Itoa` et `strconv.ParseFloat` passent du texte aux nombres,
  et renvoient une erreur à vérifier.
- `math` fournit racines, puissances et arrondis ; `math/rand` tire au hasard.
- `%.2f`, `%03d` ou `%8.2f` règlent l'affichage.

Suite : [conditions et boucles](conditions-boucles.md).
