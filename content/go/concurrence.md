# Les goroutines

Cette page montre comment lancer plusieurs tâches en même temps avec les goroutines, les attendre avec `sync.WaitGroup` et protéger les données partagées avec `sync.Mutex`.

> **Objectifs :**
>
> - vous saurez lancer une goroutine avec le mot-clé `go` ;
> - vous saurez attendre la fin de plusieurs goroutines avec `sync.WaitGroup` ;
> - vous saurez reconnaître une situation de compétition et la corriger avec `sync.Mutex` ;
> - vous saurez obtenir une sortie toujours identique malgré le désordre ;
> - vous saurez organiser le travail avec un groupe de travailleurs.

## Faire plusieurs choses à la fois

Jusqu'ici, vos programmes faisaient une chose après l'autre. La
[concurrence](../debuter/glossaire.md#concurrence) permet d'avancer sur plusieurs tâches
en même temps : télécharger dix fichiers sans attendre que chacun finisse, répondre à
plusieurs visiteurs d'un site…

Pensez à une cuisine. Un seul cuisinier fait cuire les pâtes, **puis** la sauce, **puis**
le dessert. Une brigade de cuisiniers fait tout en même temps, et le repas est prêt bien
plus tôt. Mais il faut s'organiser : savoir quand tout le monde a fini, et ne pas se
disputer la même casserole.

## Le mot-clé go

Une **goroutine** est une fonction qui s'exécute en parallèle du reste du programme.
Pour en lancer une, écrivez `go` devant un appel de fonction. C'est tout. Une goroutine
coûte très peu : un programme peut en lancer des milliers. Elle ressemble à un
[thread](../debuter/glossaire.md#thread), en beaucoup plus léger.

```go norun
go preparer("pâtes")
```

`go` lance la fonction et passe **tout de suite** à la ligne suivante, sans attendre.
Problème : quand `main` se termine, le programme s'arrête, même si des goroutines
travaillent encore. Il faut donc les attendre.

## Attendre avec sync.WaitGroup

Un `sync.WaitGroup` est un compteur de tâches en cours, comme le chef qui compte ses
cuisiniers :

- `wg.Add(1)` : « une tâche de plus » (avant de lancer la goroutine) ;
- `wg.Done()` : « une tâche finie » (à la fin de la goroutine) ;
- `wg.Wait()` : « j'attends que le compteur revienne à zéro ».

```go
package main

import (
	"fmt"
	"sync"
)

func main() {
	plats := []string{"pâtes", "sauce", "dessert"}
	prets := make([]string, len(plats))

	var wg sync.WaitGroup
	for i, plat := range plats {
		wg.Add(1)
		go func() {
			defer wg.Done()
			prets[i] = plat + " : prêt"
		}()
	}
	wg.Wait()

	for _, p := range prets {
		fmt.Println(p)
	}
}
```

```sortie
pâtes : prêt
sauce : prêt
dessert : prêt
```

`go func() { ... }()` lance une fonction anonyme écrite sur place. `defer wg.Done()`
garantit que `Done` est appelé à la fin, même si la fonction s'arrête plus tôt.

### Pourquoi ranger les résultats ?

Les goroutines finissent dans un ordre **imprévisible** : ce n'est pas forcément la
première lancée qui termine en premier. Si chacune affichait son résultat, l'ordre
changerait d'une exécution à l'autre.

Ici, chaque goroutine écrit dans **sa propre case** `prets[i]`. Le programme affiche
ensuite les cases dans l'ordre, après `wg.Wait()`. La sortie est toujours la même.

> **Note :** depuis Go 1.22, chaque tour de boucle a ses propres variables `i` et
> `plat`. Dans du code plus ancien, vous verrez `go func(i int) { ... }(i)` : c'était
> nécessaire avant.

## Le problème des accès simultanés

Que se passe-t-il si plusieurs goroutines modifient **la même** variable ? Prenons un
compteur que 1000 goroutines augmentent de 1.

```go norun
compteur := 0
var wg sync.WaitGroup
for i := 0; i < 1000; i++ {
	wg.Add(1)
	go func() {
		defer wg.Done()
		compteur++
	}()
}
wg.Wait()
fmt.Println(compteur)
```

On attend `1000`, mais le résultat peut être plus petit, et différent à chaque fois.
`compteur++` se fait en trois étapes : lire la valeur, ajouter 1, écrire. Deux
goroutines peuvent lire `41` en même temps, puis écrire toutes les deux `42` : un ajout
est perdu.

C'est une **situation de compétition** (*data race*) : deux cuisiniers qui salent la
même casserole sans se parler.

### Le détecteur de compétition

Go sait repérer ces bugs. Ajoutez `-race` :

```sh
go run -race main.go
```

Le programme tourne plus lentement, mais affiche `WARNING: DATA RACE` avec les lignes
en cause dès qu'une compétition se produit. Pensez-y quand vous écrivez du code
concurrent, et dans vos [tests](tests.md) (`go test -race`).

## Protéger avec sync.Mutex

Un `sync.Mutex` (« exclusion mutuelle ») est un verrou. C'est la clé unique des
toilettes d'un café : une seule personne à la fois, les autres attendent.

- `mu.Lock()` prend le verrou (ou attend qu'il soit libre) ;
- `mu.Unlock()` le rend.

Le code entre les deux n'est exécuté que par une goroutine à la fois.

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

	for i := 0; i < 1000; i++ {
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
1000
```

Le résultat est maintenant toujours `1000`.

### Un type protégé

Le plus propre est de ranger le verrou **avec** les données qu'il protège, dans une
[structure](types.md). Les méthodes prennent un récepteur pointeur : un verrou ne doit
jamais être copié.

```go
package main

import (
	"fmt"
	"sort"
	"sync"
)

type Votes struct {
	mu     sync.Mutex
	compte map[string]int
}

func (v *Votes) Voter(choix string) {
	v.mu.Lock()
	defer v.mu.Unlock()
	v.compte[choix]++
}

func main() {
	v := &Votes{compte: map[string]int{}}
	bulletins := []string{"chat", "chien", "chat", "lapin", "chat", "chien"}

	var wg sync.WaitGroup
	for _, b := range bulletins {
		wg.Add(1)
		go func() {
			defer wg.Done()
			v.Voter(b)
		}()
	}
	wg.Wait()

	noms := make([]string, 0, len(v.compte))
	for nom := range v.compte {
		noms = append(noms, nom)
	}
	sort.Strings(noms)
	for _, nom := range noms {
		fmt.Println(nom, v.compte[nom])
	}
}
```

```sortie
chat 3
chien 2
lapin 1
```

L'ordre d'une [map](maps.md) change à chaque parcours : on trie les clés avant
d'afficher.

## Un groupe de travailleurs

Lancer une goroutine par tâche marche bien pour quelques tâches. Pour des milliers de
fichiers à traiter, on préfère un **groupe de travailleurs** (*worker pool*) : un
nombre fixe de goroutines qui se partagent la liste. C'est une équipe de trois caissiers
pour toute une file de clients.

Ici, chaque travailleur prend la tâche suivante grâce à un indice partagé, protégé par
un verrou.

```go
package main

import (
	"fmt"
	"sort"
	"sync"
)

func main() {
	nombres := []int{3, 1, 4, 1, 5, 9, 2, 6}
	var resultats []string

	var mu sync.Mutex
	suivant := 0

	var wg sync.WaitGroup
	for t := 1; t <= 3; t++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			for {
				mu.Lock()
				if suivant >= len(nombres) {
					mu.Unlock()
					return
				}
				n := nombres[suivant]
				suivant++
				mu.Unlock()

				carre := fmt.Sprintf("%d² = %d", n, n*n)

				mu.Lock()
				resultats = append(resultats, carre)
				mu.Unlock()
			}
		}()
	}
	wg.Wait()

	sort.Strings(resultats)
	fmt.Println(len(resultats), "résultats")
	for _, r := range resultats {
		fmt.Println(r)
	}
}
```

```sortie
8 résultats
1² = 1
1² = 1
2² = 4
3² = 9
4² = 16
5² = 25
6² = 36
9² = 81
```

Le calcul se fait **hors** du verrou : on garde le verrou le moins longtemps possible,
sinon les travailleurs s'attendent les uns les autres. Le chapitre suivant,
[les canaux](canaux.md), montre une façon plus naturelle de distribuer les tâches.

## Erreurs fréquentes

### Ne pas attendre les goroutines

```go norun
package main

import "fmt"

func main() {
	go fmt.Println("Bonjour depuis une goroutine")
	fmt.Println("Fin")
}
```

```text
Fin
```

`main` se termine avant que la goroutine ait eu le temps d'afficher : son message est
perdu. Attendez-la avec un `sync.WaitGroup`.

### Appeler Done plus souvent que Add

```go norun
package main

import (
	"fmt"
	"sync"
)

func main() {
	var wg sync.WaitGroup
	wg.Done()
	fmt.Println("fin")
}
```

```text
panic: sync: negative WaitGroup counter

goroutine 1 [running]:
sync.(*WaitGroup).Add(0x0?, 0x68?)
	/usr/local/go1.24.7/src/sync/waitgroup.go:64 +0xcd
sync.(*WaitGroup).Done(...)
	/usr/local/go1.24.7/src/sync/waitgroup.go:89
main.main()
	/chemin/vers/main.go:10 +0x27
exit status 2
```

Le compteur est passé sous zéro. Chaque `wg.Done()` doit correspondre à un `wg.Add(1)`
fait **avant** de lancer la goroutine.

### Copier un WaitGroup

```go norun
package main

import (
	"fmt"
	"sync"
)

func travailler(id int, wg sync.WaitGroup) {
	defer wg.Done()
	fmt.Println("travail", id)
}

func main() {
	var wg sync.WaitGroup
	wg.Add(1)
	go travailler(1, wg)
	wg.Wait()
}
```

```text
travail 1
fatal error: all goroutines are asleep - deadlock!

goroutine 1 [sync.WaitGroup.Wait]:
sync.runtime_SemacquireWaitGroup(0xc00001c240?)
	/usr/local/go1.24.7/src/runtime/sema.go:110 +0x25
sync.(*WaitGroup).Wait(0x0?)
	/usr/local/go1.24.7/src/sync/waitgroup.go:118 +0x48
main.main()
	/chemin/vers/main.go:17 +0x72
exit status 2
```

La fonction a reçu une **copie** du `WaitGroup` : son `Done` décrémente la copie, et
l'original attend pour toujours. Passez un pointeur : `wg *sync.WaitGroup` et
`travailler(1, &wg)`. La commande `go vet` signale ce piège.

## Testez-vous

**Question 1 :** que fait `go f()` ?

> **Réponse**
>
> Il lance `f` dans une nouvelle goroutine et continue tout de suite à la ligne
> suivante, sans attendre la fin de `f`.

**Question 2 :** pourquoi `compteur++` dans 1000 goroutines ne donne-t-il pas toujours
1000 sans verrou ?

> **Réponse**
>
> Parce que `compteur++` lit, ajoute puis écrit. Deux goroutines peuvent lire la même
> valeur en même temps : un ajout est perdu. C'est une situation de compétition ; un
> `sync.Mutex` l'empêche.

**Question 3 :** calculez la longueur de chaque mot de `[]string{"go", "rapide", "et",
"simple"}` dans des goroutines, et affichez les résultats dans l'ordre des mots.

> **Réponse**
>
> ```go
> package main
>
> import (
> 	"fmt"
> 	"sync"
> )
>
> func main() {
> 	mots := []string{"go", "rapide", "et", "simple"}
> 	longueurs := make([]int, len(mots))
> 	var wg sync.WaitGroup
> 	for i, m := range mots {
> 		wg.Add(1)
> 		go func() {
> 			defer wg.Done()
> 			longueurs[i] = len(m)
> 		}()
> 	}
> 	wg.Wait()
> 	for i, m := range mots {
> 		fmt.Println(m, longueurs[i])
> 	}
> }
> ```
>
> ```sortie
> go 2
> rapide 6
> et 2
> simple 6
> ```
>
> Chaque goroutine écrit dans sa propre case : pas besoin de verrou.

## En résumé

- `go f()` lance `f` dans une goroutine, sans attendre.
- `sync.WaitGroup` attend la fin des goroutines : `Add` avant, `Done` à la fin, `Wait`.
- Plusieurs goroutines qui modifient la même donnée créent une compétition ;
  `sync.Mutex` la protège, `go run -race` la détecte.
- Rangez les résultats dans des cases séparées ou triez-les pour une sortie stable.
- Un groupe de travailleurs limite le nombre de goroutines.

Suite : [les canaux](canaux.md).
