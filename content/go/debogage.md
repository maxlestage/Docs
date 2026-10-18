# Déboguer en Go

Cette page montre comment trouver et corriger un bug en Go : lire les erreurs du compilateur, comprendre une panique, afficher les valeurs avec `%#v`, utiliser `go vet`, le débogueur Delve et le détecteur de courses.

> **Objectifs :**
>
> - lire une erreur de compilation et trouver la ligne fautive ;
> - lire une panique et sa trace d'appels ;
> - afficher l'état d'un programme avec `fmt.Printf("%#v")` et le paquet `log` ;
> - repérer le code suspect avec `go vet` et le détecteur de courses ;
> - avancer pas à pas avec le débogueur Delve.

## Qu'est-ce que déboguer ?

Un [bug](../debuter/glossaire.md#bug) est une erreur dans un programme. Le
[débogage](../debuter/glossaire.md#debogage) consiste à le trouver, puis à le corriger.

Déboguer, c'est mener une enquête : observer les indices, faire une hypothèse, la
vérifier. On ne devine pas au hasard. Pour les bases communes à tous les langages, voyez
aussi [déboguer pour débuter](../debuter/deboguer.md).

En Go, les bugs se montrent à deux moments :

- **avant** l'exécution : le [compilateur](../debuter/glossaire.md#compilateur) refuse
  le programme ;
- **pendant** l'exécution : le programme s'arrête sur une
  [panique](../debuter/glossaire.md#panique), ou donne un résultat faux.

Le premier cas est le plus confortable : le compilateur vous dit exactement où regarder.

## Lire une erreur de compilation

Le compilateur Go est strict. Ce programme contient quatre fautes :

```go norun
package main

import (
	"fmt"
	"os"
)

func main() {
	total := 0
	prix := 12
	var nom string = 42
	fmt.Println(prix)
}
```

```text
# command-line-arguments
./main.go:5:2: "os" imported and not used
./main.go:9:2: declared and not used: total
./main.go:11:6: declared and not used: nom
./main.go:11:19: cannot use 42 (untyped int constant) as string value in variable declaration
```

Chaque ligne suit le même modèle : `fichier:ligne:colonne: message`. Ainsi
`./main.go:9:2` veut dire « fichier `main.go`, ligne 9, colonne 2 ».

Lisez les erreurs **dans l'ordre**, et corrigez la première d'abord : une faute en
entraîne parfois d'autres.

### Variable ou import inutilisé

En Go, une variable déclarée mais jamais lue est une **erreur**, pas un simple
avertissement. Pareil pour un paquet importé mais pas utilisé. C'est voulu : du code
inutile cache souvent un oubli.

Deux corrections possibles : utiliser la variable, ou la supprimer. Pour garder une
valeur sans l'utiliser, affectez-la à `_`, le « trou noir » de Go.

```go
package main

import "fmt"

func main() {
	prix := 12
	_ = prix * 2 // calcul gardé pour plus tard
	fmt.Println(prix)
}
```

```sortie
12
```

### Des types qui ne correspondent pas

Go ne convertit jamais un type en un autre tout seul. Ici, la fonction promet un
`float64`, mais le calcul donne un `int` :

```go norun
func moyenne(notes []int) float64 {
	total := 0
	for _, n := range notes {
		total += n
	}
	return total / len(notes)
}
```

```text
./main.go:10:9: cannot use total / len(notes) (value of type int) as float64 value in return statement
```

Le message se lit ainsi : « impossible d'utiliser `total / len(notes)` (une valeur de
type `int`) comme une valeur `float64` ». La correction : convertir **avant** de diviser.

```go
package main

import "fmt"

func moyenne(notes []int) float64 {
	total := 0
	for _, n := range notes {
		total += n
	}
	return float64(total) / float64(len(notes))
}

func main() {
	fmt.Println(moyenne([]int{12, 15}))
}
```

```sortie
13.5
```

## Lire une panique

Une **panique** arrête le programme pendant l'exécution. Go affiche alors la cause et
la **trace d'appels** (*stack trace*). Voici une fonction qui veut le dernier élément
d'une liste :

```go norun
package main

import "fmt"

func dernier(notes []int) int {
	return notes[len(notes)]
}

func main() {
	notes := []int{12, 15, 16}
	fmt.Println(dernier(notes))
}
```

```text
panic: runtime error: index out of range [3] with length 3

goroutine 1 [running]:
main.dernier(...)
	/home/alice/notes/main.go:6
main.main()
	/home/alice/notes/main.go:11 +0x16
exit status 2
```

Lisez-la dans cet ordre :

1. **La ligne `panic:`** : la cause. On a demandé l'indice `3` d'une liste de longueur
   `3`. Les indices vont de `0` à `2` : le dernier est `len(notes) - 1`.
2. **`goroutine 1 [running]`** : la goroutine qui a paniqué (la 1 est le programme
   principal, voir [la concurrence](concurrence.md)).
3. **Les paires de lignes** : chaque fonction, puis son fichier et sa ligne. La plus
   haute est la plus profonde : `main.main` (ligne 11) a appelé `main.dernier`
   (ligne 6), où la panique a eu lieu.

Le `+0x16` est une position dans le code machine : ignorez-le. `exit status 2` est le
code de sortie du programme, renvoyé par `go run`.

> **Astuce :** dans une longue trace, cherchez la première ligne qui parle de **vos**
> fichiers. Les lignes qui viennent de la bibliothèque standard sont rarement en cause.

## Afficher les valeurs avec fmt.Printf

`fmt.Println` affiche une valeur « pour les humains ». Pour déboguer, il faut voir la
valeur **exacte**. `fmt.Printf` propose plusieurs verbes :

| Verbe | Affiche |
|-------|---------|
| `%v` | la valeur, simplement |
| `%+v` | une structure avec le nom des champs |
| `%#v` | la valeur écrite comme du code Go |
| `%T` | le type de la valeur |
| `%q` | une chaîne entre guillemets |

```go
package main

import "fmt"

type Eleve struct {
	Nom   string
	Notes []int
	Actif bool
}

func main() {
	e := Eleve{Nom: "Alice", Notes: []int{12, 15}}
	fmt.Printf("%v\n", e)
	fmt.Printf("%+v\n", e)
	fmt.Printf("%#v\n", e)
	fmt.Printf("%T\n", e)
}
```

```sortie
{Alice [12 15] false}
{Nom:Alice Notes:[12 15] Actif:false}
main.Eleve{Nom:"Alice", Notes:[]int{12, 15}, Actif:false}
main.Eleve
```

`%#v` est le plus précis : on voit les guillemets des chaînes et le type de chaque
morceau. Il distingue des valeurs que `%v` confond :

```go
package main

import "fmt"

func main() {
	var texte string
	var notes []int
	fmt.Printf("%v|%#v\n", texte, texte)
	fmt.Printf("%v %#v\n", "42", "42")
	fmt.Printf("%v %#v\n", notes, notes)
}
```

```sortie
|""
42 "42"
[] []int(nil)
```

Avec `%v`, une chaîne vide ne se voit pas, et `"42"` ressemble à `42`. Avec `%#v`, tout
est clair. `[]int(nil)` indique une tranche jamais créée (voir
[les collections](collections.md)).

> **Astuce :** commencez vos lignes de débogage par `DEBUG` :
> `fmt.Printf("DEBUG prix=%#v\n", prix)`. Vous les retrouverez et les supprimerez
> facilement avant de partager votre code.

## Le paquet log

Le paquet `log` affiche des messages de suivi. Par défaut, il écrit sur la **sortie
d'erreur** et ajoute la date et l'heure devant chaque ligne :

```text
2026/10/18 21:01:54 démarrage
```

On peut régler son comportement : `SetFlags(0)` enlève la date, `SetPrefix` ajoute une
étiquette, `SetOutput` choisit où écrire.

```go
package main

import (
	"log"
	"os"
)

func main() {
	log.SetOutput(os.Stdout)
	log.SetFlags(0)
	log.SetPrefix("[prix] ")
	prix := 12
	log.Printf("calcul avec prix=%d", prix)
	log.Println("terminé")
}
```

```sortie
[prix] calcul avec prix=12
[prix] terminé
```

> **Attention :** `log.Fatal(...)` affiche le message puis **arrête** le programme avec
> le code 1. Les `defer` ne sont pas exécutés. Réservez-le au `main`, pour les erreurs
> dont on ne peut pas se remettre.

## Repérer le code suspect avec go vet

Certains programmes compilent mais contiennent une faute probable. `go vet` les
cherche. Ici, les arguments de `Printf` sont inversés, et `Println` reçoit un `%d` qu'il
ne comprend pas :

```go norun
package main

import "fmt"

func main() {
	nom := "Alice"
	age := 30
	fmt.Printf("%s a %d ans\n", age, nom)
	fmt.Println("total : %d", 3)
}
```

```sh
go vet main.go
```

```text
# command-line-arguments
# [command-line-arguments]
./main.go:8:2: fmt.Printf format %s has arg age of wrong type int
./main.go:9:2: fmt.Println call has possible Printf formatting directive %d
```

Sans `go vet`, le programme tourne et affiche n'importe quoi :

```text
%!s(int=30) a %!d(string=Alice) ans
total : %d 3
```

`%!s(int=30)` signifie : « le verbe `%s` a reçu un `int` qui vaut 30 ». Si vous voyez
`%!` dans une sortie, un verbe ne correspond pas à son argument.

> **Note :** `go test` lance automatiquement une partie des vérifications de `go vet`
> (voir [les tests](tests.md)).

## Le débogueur Delve

Afficher des valeurs oblige à deviner lesquelles. Un **débogueur** fait mieux : il
arrête le programme à un endroit choisi, et vous laisse l'explorer. Le débogueur de Go
s'appelle **Delve**, et sa commande `dlv`. Installez-le une fois :

```sh
go install github.com/go-delve/delve/cmd/dlv@latest
```

Puis, dans le dossier d'un [module](projet.md), lancez `dlv debug`. Voici une session
réelle sur la fonction `moyenne` (version qui renvoie un `int`) :

```text
$ dlv debug
Type 'help' for list of commands.
(dlv) break moyenne
Breakpoint 1 set at 0x4b1ae4 for main.moyenne() ./main.go:5
(dlv) continue
> [Breakpoint 1] main.moyenne() ./main.go:5 (hits goroutine(1):1 total:1) (PC: 0x4b1ae4)
=>   5:	func moyenne(notes []int) int {
     6:		total := 0
     7:		for _, n := range notes {
     8:			total += n
     9:		}
    10:		return total / len(notes)
(dlv) print notes
[]int len: 3, cap: 3, [12,15,16]
(dlv) break main.go:10
Breakpoint 2 set at 0x4b1b5e for main.moyenne() ./main.go:10
(dlv) continue
> [Breakpoint 2] main.moyenne() ./main.go:10 (hits goroutine(1):1 total:1) (PC: 0x4b1b5e)
(dlv) locals
total = 43
(dlv) print total / len(notes)
14
(dlv) exit
```

Un **point d'arrêt** (*breakpoint*) est un endroit où le programme se met en pause.
Les commandes principales :

| Commande | Effet |
|----------|-------|
| `break moyenne` ou `b main.go:10` | pose un point d'arrêt |
| `continue` (ou `c`) | reprend jusqu'au prochain arrêt |
| `next` (ou `n`) | exécute la ligne et passe à la suivante |
| `step` (ou `s`) | entre dans la fonction appelée |
| `print x` (ou `p x`) | affiche une valeur ou une expression |
| `locals` | liste les variables locales |
| `stack` | affiche la trace d'appels |
| `exit` | quitte |

> **Astuce :** VS Code (avec l'extension Go) utilise Delve : un clic dans la marge pose
> un point d'arrêt, sans taper de commande.

## Le détecteur de courses

Avec les [goroutines](concurrence.md), un bug sournois apparaît : la **course de
données** (*data race*). Deux goroutines touchent la même variable en même temps, et
le résultat change d'une exécution à l'autre.

```go norun
package main

import (
	"fmt"
	"sync"
)

func main() {
	compteur := 0
	var wg sync.WaitGroup
	for i := 0; i < 100; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			compteur++
		}()
	}
	wg.Wait()
	fmt.Println(compteur)
}
```

Le programme affiche souvent `100`... mais pas toujours. L'option `-race` active le
**détecteur de courses**, qui surveille les accès à la mémoire :

```sh
go run -race main.go
```

```text
==================
WARNING: DATA RACE
Read at 0x00c000012178 by goroutine 7:
  main.main.func1()
      /home/alice/compteur/main.go:15 +0x84

Previous write at 0x00c000012178 by goroutine 106:
  main.main.func1()
      /home/alice/compteur/main.go:15 +0x96
...
==================
100
Found 1 data race(s)
exit status 66
```

Le rapport désigne la ligne 15 : `compteur++`, lu et écrit par plusieurs goroutines.
La correction : protéger la variable avec un verrou, `sync.Mutex`.

```go
package main

import (
	"fmt"
	"sync"
)

func main() {
	compteur := 0
	var mu sync.Mutex
	var wg sync.WaitGroup
	for i := 0; i < 100; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			mu.Lock()
			compteur++
			mu.Unlock()
		}()
	}
	wg.Wait()
	fmt.Println(compteur)
}
```

```sortie
100
```

> **À retenir :** lancez vos [tests](tests.md) avec `go test -race` dès que votre
> programme utilise des goroutines.

## Une enquête pas à pas

Voici un compte bancaire. On dépose 100, puis on affiche le solde.

```go
package main

import "fmt"

type Compte struct {
	Solde int
}

func (c Compte) Deposer(montant int) {
	c.Solde += montant
}

func main() {
	compte := Compte{}
	compte.Deposer(100)
	fmt.Println(compte.Solde)
}
```

```sortie
0
```

**1. Le symptôme.** On attend `100`, le programme affiche `0`.

**2. Observer.** On affiche le compte dans la méthode et dans `main`, avec des
étiquettes et `%p`, qui montre l'**adresse** de la valeur en mémoire :

```go norun
func (c Compte) Deposer(montant int) {
	c.Solde += montant
	fmt.Printf("DEBUG dans Deposer : %+v à %p\n", c, &c)
}

// dans main :
fmt.Printf("DEBUG dans main : %+v à %p\n", compte, &compte)
```

```text
DEBUG dans Deposer : {Solde:100} à 0xc000010108
DEBUG dans main : {Solde:0} à 0xc000010100
```

**3. L'hypothèse.** Dans la méthode, le solde vaut bien 100. Mais les adresses sont
différentes : ce ne sont pas les mêmes comptes ! Le récepteur `(c Compte)` reçoit une
**copie** du compte. On modifie la copie, puis elle disparaît.

**4. Corriger.** Un récepteur **pointeur** `(c *Compte)` travaille sur l'original (voir
[les pointeurs](pointeurs.md)).

```go
package main

import "fmt"

type Compte struct {
	Solde int
}

func (c *Compte) Deposer(montant int) {
	c.Solde += montant
}

func main() {
	compte := Compte{}
	compte.Deposer(100)
	compte.Deposer(50)
	fmt.Println(compte.Solde)
}
```

```sortie
150
```

**5. Protéger.** Écrivez un [test](tests.md) qui vérifie le solde après un dépôt : si
quelqu'un remet un jour `(c Compte)`, le test échouera tout de suite.

> **À retenir :** symptôme, observation, hypothèse, correction, test. Une méthode qui
> **modifie** son récepteur doit avoir un récepteur pointeur.

## Erreurs fréquentes

### Ignorer la variable err

```go norun
n, err := strconv.Atoi("12")
fmt.Println(n)
```

```text
./main.go:9:5: declared and not used: err
```

Go vous oblige à faire quelque chose de `err`. Testez-la avec
`if err != nil { ... }` (voir [les erreurs](erreurs.md)). Ne la remplacez par `_` que
si vous êtes certain que l'erreur est impossible.

### Écrire dans une map nil

```go norun
var m map[string]int
m["a"] = 1
```

```text
panic: assignment to entry in nil map

goroutine 1 [running]:
main.main()
	/home/alice/map/main.go:7 +0x28
exit status 2
```

`var m map[string]int` déclare une map, mais ne la crée pas : elle vaut `nil`. Lire
dedans marche, écrire panique. Créez-la avec `m := map[string]int{}` ou
`make(map[string]int)` (voir [les maps](maps.md)).

### Utiliser un pointeur nil

```go norun
var p *Point
fmt.Println(p.X)
```

```text
panic: runtime error: invalid memory address or nil pointer dereference
[signal SIGSEGV: segmentation violation code=0x1 addr=0x0 pc=0x491af6]
```

`p` ne pointe vers rien. Avant d'utiliser un pointeur qui peut être `nil`, testez-le
(`if p != nil`), ou créez la valeur avec `p := &Point{}`.

### Un verbe %d dans Println

```go norun
fmt.Println("total : %d", 3)
```

```text
total : %d 3
```

`Println` ne connaît pas les verbes : il affiche le texte tel quel. Utilisez
`fmt.Printf("total : %d\n", 3)`. `go vet` signale cette faute.

## Testez-vous

**Question 1 :** que veut dire `./main.go:9:2` dans une erreur de compilation ?

> **Réponse**
>
> Fichier `main.go`, ligne 9, colonne 2. C'est l'endroit exact de la faute.

**Question 2 :** pourquoi préférer `%#v` à `%v` pour déboguer ?

> **Réponse**
>
> `%#v` affiche la valeur comme du code Go : on voit les guillemets des chaînes, le
> type, et la différence entre une tranche vide et une tranche `nil`.

**Question 3 :** un programme avec des goroutines donne parfois un résultat faux.
Quelle commande lancer en premier ?

> **Réponse**
>
> `go run -race main.go` (ou `go test -race`). Le détecteur de courses montre les
> variables partagées sans protection.

## En résumé

- Le compilateur indique `fichier:ligne:colonne` : corrigez la première erreur d'abord.
- Une panique se lit de haut en bas : la cause, puis les fonctions jusqu'à `main.main`.
- `fmt.Printf("%#v")` montre la valeur exacte ; `%T` montre son type.
- `go vet` trouve le code suspect ; `-race` trouve les courses de données.
- Delve (`dlv debug`) met le programme en pause et l'explore ligne par ligne.
- Enquêtez avec méthode : symptôme, observation, hypothèse, correction, test.

Suite : [bien écrire du Go](style.md).
