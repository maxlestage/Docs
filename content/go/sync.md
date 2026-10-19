# sync : Once, RWMutex, atomic et errgroup

Cette page présente les autres outils de coordination de Go : `sync.Once` pour faire une chose une seule fois, `sync.RWMutex` pour laisser lire à plusieurs, `sync/atomic` pour les compteurs et `errgroup` pour attendre des goroutines qui peuvent échouer.

> **Objectifs :**
>
> - reconnaître une situation de compétition et la faire signaler par `go run -race` ;
> - choisir entre `sync.Mutex` et `sync.RWMutex` ;
> - lancer une initialisation une seule fois avec `sync.Once` ;
> - compter sans verrou avec `atomic.Int64` ;
> - attendre des goroutines et récupérer la première erreur avec `errgroup`.

Ce chapitre suppose que vous connaissez [les goroutines](concurrence.md),
`sync.WaitGroup` et `sync.Mutex`. On les rappelle rapidement.

## La boîte à outils du paquet sync

Plusieurs [goroutines](../debuter/glossaire.md#goroutine) qui partagent des données,
c'est comme plusieurs cuisiniers dans la même cuisine. Il faut des règles. Chaque outil
est une règle différente :

| Outil | La règle, en cuisine | Sert à |
|---|---|---|
| `sync.WaitGroup` | le chef compte les cuisiniers qui ont fini | attendre la fin |
| `sync.Mutex` | une seule clé pour la chambre froide | un seul à la fois |
| `sync.RWMutex` | tout le monde lit la recette, un seul la corrige | lecteurs nombreux, un écrivain |
| `sync.Once` | on allume le four une seule fois le matin | faire une chose une fois |
| `atomic.Int64` | un compteur à cliquet à l'entrée | compter vite et sans verrou |
| `errgroup` | le chef attend tout le monde et note le premier problème | attendre et récupérer une erreur |

> **À retenir :** chaque outil répond à une question précise. Commencez par vous
> demander : « qu'est-ce qui est partagé, et qui y touche ? »

## Rappel : attendre avec WaitGroup

`wg.Add(1)` avant de lancer, `defer wg.Done()` dans la goroutine, `wg.Wait()` pour
attendre. Chaque goroutine écrit dans **sa propre case** : il n'y a rien à protéger.

```go
package main

import (
	"fmt"
	"sync"
)

func main() {
	nombres := []int{1, 2, 3, 4}
	carres := make([]int, len(nombres))

	var wg sync.WaitGroup
	for i, n := range nombres {
		wg.Add(1)
		go func() {
			defer wg.Done()
			carres[i] = n * n
		}()
	}
	wg.Wait()
	fmt.Println(carres)
}
```

```sortie
[1 4 9 16]
```

```text
goroutine 0 ──► carres[0]
goroutine 1 ──► carres[1]      chacun sa case :
goroutine 2 ──► carres[2]      pas de dispute
goroutine 3 ──► carres[3]
```

> **À retenir :** si chaque goroutine a sa propre case, un `WaitGroup` suffit. Les
> verrous ne servent que quand plusieurs goroutines touchent la **même** donnée.

## Une situation de compétition, pas à pas

Quand deux goroutines modifient la même variable sans règle, on a une **situation de
compétition** (en anglais *data race*). Voici le plus petit exemple : deux goroutines
font `compteur++`.

```go norun
compteur := 0
var wg sync.WaitGroup
for i := 0; i < 2; i++ {
	wg.Add(1)
	go func() {
		defer wg.Done()
		compteur++
	}()
}
wg.Wait()
fmt.Println(compteur)
```

`compteur++` a l'air d'une seule action. En vrai, c'est trois petites étapes : **lire**,
**ajouter 1**, **écrire**. Voici un ordre possible, où un ajout se perd :

| Étape | Goroutine A | Goroutine B | compteur en mémoire |
|---|---|---|---|
| 1 | lit 0 | | 0 |
| 2 | | lit 0 | 0 |
| 3 | calcule 0 + 1 = 1 | | 0 |
| 4 | | calcule 0 + 1 = 1 | 0 |
| 5 | écrit 1 | | 1 |
| 6 | | écrit 1 | **1** au lieu de 2 |

C'est comme deux personnes qui notent les entrées d'un musée sur le même cahier : elles
lisent « 41 » en même temps, écrivent toutes les deux « 42 », et un visiteur disparaît.

### Le détecteur -race

Le bug est rare et change d'une fois à l'autre : on ne le voit pas toujours. Go a un
détecteur. Ajoutez `-race` à `go run` (ou à `go test`) :

```sh
go run -race main.go
```

```text
==================
WARNING: DATA RACE
Read at 0x00c000122078 by goroutine 7:
  main.main.func1()
      /home/lea/compteur/main.go:15 +0x84

Previous write at 0x00c000122078 by goroutine 8:
  main.main.func1()
      /home/lea/compteur/main.go:15 +0x96

Goroutine 7 (running) created at:
  main.main()
      /home/lea/compteur/main.go:13 +0x78

Goroutine 8 (finished) created at:
  main.main()
      /home/lea/compteur/main.go:13 +0x78
==================
2
Found 1 data race(s)
exit status 66
```

Comment le lire :

- `WARNING: DATA RACE` : une compétition a été vue ;
- `Read at … main.go:15` puis `Previous write at … main.go:15` : une lecture et une
  écriture de la même case mémoire, sur la ligne 15 (`compteur++`), par deux
  goroutines différentes ;
- `created at … main.go:13` : la ligne où ces goroutines sont lancées (`go func()`) ;
- le `2` affiché ne prouve rien : cette fois, on a eu de la chance.

Essayez : lancez le programme sans `-race`, puis avec. Sans, il affiche `2` et se tait.
Avec, il affiche le rapport et termine avec le code de sortie 66.

> **À retenir :** lancez vos programmes et vos [tests](tests.md) avec `-race` dès qu'il
> y a des goroutines. Le détecteur trouve les bugs que vos yeux ne voient pas.

## Rappel : un seul à la fois avec Mutex

Un `sync.Mutex` est un verrou. `mu.Lock()` prend la clé ; si quelqu'un l'a déjà, on
attend. `mu.Unlock()` rend la clé.

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

Avec le verrou, le tableau de tout à l'heure devient :

| Étape | Goroutine A | Goroutine B | compteur |
|---|---|---|---|
| 1 | `Lock()` : prend la clé | | 0 |
| 2 | lit 0, écrit 1 | `Lock()` : attend… | 1 |
| 3 | `Unlock()` | …attend encore | 1 |
| 4 | | prend la clé, lit 1, écrit 2 | 2 |
| 5 | | `Unlock()` | 2 |

> **À retenir :** tout ce qui est entre `Lock()` et `Unlock()` est fait par une seule
> goroutine à la fois.

## Lire à plusieurs : RWMutex

Un `Mutex` fait attendre tout le monde, même ceux qui veulent seulement **lire**. C'est
dommage : lire ne casse rien.

Pensez au tableau d'affichage d'une école. Vingt élèves peuvent le lire en même temps.
Mais quand le directeur change une affiche, il demande à tout le monde de s'écarter.

Un `sync.RWMutex` a donc deux sortes de verrous :

| Méthode | Pour qui | Combien à la fois |
|---|---|---|
| `RLock()` / `RUnlock()` | un lecteur | autant qu'on veut |
| `Lock()` / `Unlock()` | un écrivain | un seul, et aucun lecteur pendant ce temps |

```go
package main

import (
	"fmt"
	"sync"
)

type Annuaire struct {
	mu      sync.RWMutex
	numeros map[string]string
}

func (a *Annuaire) Chercher(nom string) string {
	a.mu.RLock()
	defer a.mu.RUnlock()
	return a.numeros[nom]
}

func (a *Annuaire) Ajouter(nom, numero string) {
	a.mu.Lock()
	defer a.mu.Unlock()
	a.numeros[nom] = numero
}

func main() {
	a := &Annuaire{numeros: map[string]string{}}
	a.Ajouter("Léa", "06 11")
	a.Ajouter("Tom", "06 22")

	noms := []string{"Léa", "Tom", "Léa", "Zoé"}
	trouves := make([]string, len(noms))

	var wg sync.WaitGroup
	for i, nom := range noms {
		wg.Add(1)
		go func() {
			defer wg.Done()
			trouves[i] = nom + " → " + a.Chercher(nom)
		}()
	}
	wg.Wait()

	for _, t := range trouves {
		fmt.Println(t)
	}
}
```

```sortie
Léa → 06 11
Tom → 06 22
Léa → 06 11
Zoé → 
```

Les quatre recherches peuvent se faire **en même temps** : elles prennent toutes un
`RLock`. `Zoé` n'est pas dans la [map](maps.md) : on reçoit le texte vide.

```text
 lecteurs :  Léa ─┐
             Tom ─┼──► RLock  RLock  RLock  RLock    (tous ensemble)
             Léa ─┤
             Zoé ─┘
 écrivain :  Ajouter ──► Lock   (attend que les lecteurs aient fini,
                                 puis passe seul)
```

`defer a.mu.RUnlock()` rend le verrou à la sortie de la méthode, même avec un `return`
au milieu (voir [defer](fonctions-avancees.md#defer-repousser-a-la-fin)).

Essayez : remplacez `RLock`/`RUnlock` par `Lock`/`Unlock` dans `Chercher`. Le résultat
est le même, mais les recherches passent une par une.

> **À retenir :** prenez un `RWMutex` quand on lit **beaucoup plus** qu'on n'écrit
> (un cache, une configuration). Sinon, un simple `Mutex` suffit et reste plus simple.

## Une seule fois : sync.Once

Certaines choses ne doivent se faire qu'une fois : lire un fichier de configuration,
ouvrir une connexion. C'est comme allumer le four le matin : le premier cuisinier
l'allume, les suivants le trouvent déjà chaud.

`once.Do(f)` appelle `f` la première fois, et **plus jamais** ensuite.

```go
package main

import (
	"fmt"
	"sync"
)

var (
	once   sync.Once
	config map[string]string
)

func charger() {
	fmt.Println("chargement de la configuration…")
	config = map[string]string{"langue": "fr", "theme": "sombre"}
}

func lire(cle string) string {
	once.Do(charger)
	return config[cle]
}

func main() {
	fmt.Println(lire("langue"))
	fmt.Println(lire("theme"))
	fmt.Println(lire("langue"))
}
```

```sortie
chargement de la configuration…
fr
sombre
fr
```

| Appel | once déjà fait ? | charger() appelé ? | Affiche |
|---|---|---|---|
| `lire("langue")` | non | oui | le message, puis `fr` |
| `lire("theme")` | oui | non | `sombre` |
| `lire("langue")` | oui | non | `fr` |

### Avec plusieurs goroutines

`sync.Once` marche aussi quand dix goroutines arrivent **en même temps**. Une seule
lance la fonction ; les autres attendent qu'elle ait fini, puis continuent.

```go
package main

import (
	"fmt"
	"sync"
)

func main() {
	var once sync.Once
	var wg sync.WaitGroup

	for i := 0; i < 10; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			once.Do(func() { fmt.Println("le four est allumé") })
		}()
	}
	wg.Wait()
	fmt.Println("10 cuisiniers servis")
}
```

```sortie
le four est allumé
10 cuisiniers servis
```

### sync.OnceValue : une fois, avec un résultat

Depuis Go 1.21, `sync.OnceValue(f)` renvoie une fonction. Le premier appel calcule
la valeur ; les suivants la redonnent sans recalculer.

```go
package main

import (
	"fmt"
	"sync"
)

func main() {
	prix := sync.OnceValue(func() int {
		fmt.Println("calcul du prix…")
		return 12 * 3
	})

	fmt.Println(prix())
	fmt.Println(prix())
}
```

```sortie
calcul du prix…
36
36
```

Essayez : ajoutez un troisième `fmt.Println(prix())`. Le message `calcul du prix…`
n'apparaît toujours qu'une fois.

> **À retenir :** `once.Do(f)` = « fais `f` la première fois, ignore les fois
> suivantes ». C'est sûr même avec beaucoup de goroutines.

## Compter sans verrou : sync/atomic

Pour un simple compteur, un `Mutex` est un peu lourd. Le paquet `sync/atomic` propose
des nombres dont les opérations sont **indivisibles** (« atomiques ») : lire, ajouter
et écrire se font en une seule étape que personne ne peut couper.

C'est le compteur à cliquet du portier de boîte de nuit : clic, +1. Deux portiers qui
cliquent en même temps ne perdent jamais un clic.

```go
package main

import (
	"fmt"
	"sync"
	"sync/atomic"
)

func main() {
	var visites atomic.Int64
	var wg sync.WaitGroup

	for i := 0; i < 1000; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			visites.Add(1)
		}()
	}
	wg.Wait()
	fmt.Println("visites :", visites.Load())
}
```

```sortie
visites : 1000
```

Les méthodes principales :

| Méthode | Ce qu'elle fait |
|---|---|
| `v.Add(n)` | ajoute `n` et renvoie la nouvelle valeur |
| `v.Load()` | lit la valeur |
| `v.Store(n)` | remplace la valeur |
| `v.CompareAndSwap(ancien, nouveau)` | remplace seulement si la valeur vaut encore `ancien` |

Suivons quelques appels, sans goroutine, pour voir les valeurs renvoyées :

```go
package main

import (
	"fmt"
	"sync/atomic"
)

func main() {
	var n atomic.Int64
	fmt.Println(n.Add(5))
	fmt.Println(n.Add(-2))
	n.Store(10)
	fmt.Println(n.Load())
	fmt.Println(n.CompareAndSwap(10, 20), n.Load())
	fmt.Println(n.CompareAndSwap(10, 99), n.Load())
}
```

```sortie
5
3
10
true 20
false 20
```

| Étape | Appel | n après | Renvoie |
|---|---|---|---|
| 1 | `Add(5)` | 5 | 5 |
| 2 | `Add(-2)` | 3 | 3 |
| 3 | `Store(10)` | 10 | rien |
| 4 | `CompareAndSwap(10, 20)` | 20 | `true` : n valait bien 10 |
| 5 | `CompareAndSwap(10, 99)` | 20 | `false` : n vaut 20, pas 10 |

Il existe aussi `atomic.Int32`, `atomic.Uint64` et `atomic.Bool`. Leur valeur de départ
est zéro (ou `false`) : pas besoin de les créer.

Essayez : remplacez `visites.Add(1)` par `visites.Add(2)`. Le programme affiche
`visites : 2000`.

> **À retenir :** pour **un** nombre partagé, `atomic.Int64` suffit. Dès qu'il faut
> modifier plusieurs choses ensemble (une map et un compteur), revenez à un `Mutex`.

## Attendre et récupérer une erreur : errgroup

Un `WaitGroup` attend, mais ne sait rien des erreurs. Si une goroutine échoue,
comment le savoir ? Le paquet `errgroup` répond à cette question.

Il ne fait pas partie de la bibliothèque standard : il est dans le module
`golang.org/x/sync`, maintenu par l'équipe de Go. Il faut l'ajouter à votre projet
(voir [les modules](modules.md)) :

```sh
go get golang.org/x/sync/errgroup
```

Le principe :

- `g.Go(f)` lance `f` dans une goroutine ; `f` renvoie une `error` ;
- `g.Wait()` attend toutes les goroutines et renvoie la **première** erreur (ou `nil`).

```go norun
package main

import (
	"errors"
	"fmt"

	"golang.org/x/sync/errgroup"
)

func telecharger(nom string) (int, error) {
	if nom == "c.txt" {
		return 0, errors.New("c.txt : introuvable")
	}
	return len(nom) * 100, nil
}

func main() {
	fichiers := []string{"a.txt", "b.txt", "c.txt"}
	tailles := make([]int, len(fichiers))

	var g errgroup.Group
	for i, nom := range fichiers {
		g.Go(func() error {
			t, err := telecharger(nom)
			if err != nil {
				return err
			}
			tailles[i] = t
			return nil
		})
	}
	if err := g.Wait(); err != nil {
		fmt.Println("erreur :", err)
		return
	}
	fmt.Println("tailles :", tailles)
}
```

```text
erreur : c.txt : introuvable
```

| Goroutine | Fichier | Renvoie |
|---|---|---|
| 1 | `a.txt` | `nil` (taille 500) |
| 2 | `b.txt` | `nil` (taille 500) |
| 3 | `c.txt` | `c.txt : introuvable` |
| `g.Wait()` | | la première erreur : `c.txt : introuvable` |

Essayez : remplacez `"c.txt"` par `"d.txt"` dans la liste. Plus rien n'échoue, et le
programme affiche `tailles : [500 500 500]`.

Comparez avec un `WaitGroup` : plus de `Add`, plus de `Done`. `errgroup` les fait pour
vous.

### Arrêter les autres avec WithContext

`errgroup.WithContext(ctx)` donne un groupe **et** un [context](context.md). Dès qu'une
goroutine renvoie une erreur, ce contexte est annulé : les autres peuvent s'arrêter.
`g.SetLimit(n)` limite le nombre de goroutines en marche en même temps.

```go norun
g, ctx := errgroup.WithContext(context.Background())
g.SetLimit(3) // au plus 3 téléchargements à la fois
for _, url := range urls {
	g.Go(func() error {
		return telechargerAvec(ctx, url) // s'arrête si ctx est annulé
	})
}
err := g.Wait()
```

> **À retenir :** `errgroup` = un `WaitGroup` qui rapporte la première erreur. Avec
> `WithContext`, une erreur prévient aussi toutes les autres goroutines.

## Quel outil choisir ?

```text
Des goroutines partagent-elles une donnée ?
├── non ─► WaitGroup (ou errgroup si elles peuvent échouer)
└── oui
    ├── c'est un simple nombre ? ─► atomic.Int64
    ├── on lit beaucoup, on écrit peu ? ─► RWMutex
    ├── on l'initialise une seule fois ? ─► sync.Once
    └── sinon ─► Mutex (ou un canal, voir la page canaux)
```

N'oubliez pas les [canaux](canaux.md) : au lieu de partager une donnée, une goroutine
peut l'**envoyer** à une autre. Le proverbe de Go dit : « ne communiquez pas en
partageant la mémoire ; partagez la mémoire en communiquant ».

> **À retenir :** prenez l'outil le plus simple qui suffit. Et vérifiez avec `-race`.

## Erreurs fréquentes

### Copier une structure qui contient un Mutex

```go norun
package main

import (
	"fmt"
	"sync"
)

type Compteur struct {
	mu sync.Mutex
	n  int
}

func afficher(c Compteur) {
	fmt.Println(c.n)
}

func main() {
	var c Compteur
	afficher(c)
}
```

Le programme compile, mais `go vet main.go` proteste :

```text
# command-line-arguments
# [command-line-arguments]
./main.go:13:17: afficher passes lock by value: command-line-arguments.Compteur contains sync.Mutex
./main.go:19:11: call of afficher copies lock value: command-line-arguments.Compteur contains sync.Mutex
```

Une copie du verrou est un **autre** verrou : il ne protège plus rien. Passez un
pointeur : `func afficher(c *Compteur)` et `afficher(&c)` (voir
[les pointeurs](pointeurs.md)).

### Rendre une clé qu'on n'a pas prise

```go norun
var mu sync.Mutex
mu.Lock()
fmt.Println("dans la section protégée")
mu.Unlock()
mu.Unlock()
```

```text
dans la section protégée
fatal error: sync: unlock of unlocked mutex

goroutine 1 [running]:
…
exit status 2
```

Un `Unlock` de trop arrête le programme. Écrivez `defer mu.Unlock()` juste après
`mu.Lock()` : il y aura toujours exactement un `Unlock`.

### Prendre deux fois le même verrou

```go norun
var mu sync.RWMutex
mu.RLock()
fmt.Println("lecture en cours")
mu.Lock()
fmt.Println("jamais affiché")
```

```text
lecture en cours
fatal error: all goroutines are asleep - deadlock!

goroutine 1 [sync.RWMutex.Lock]:
…
exit status 2
```

`Lock()` attend que tous les lecteurs aient fini… mais le lecteur, c'est vous. Personne
ne peut avancer : c'est un **interblocage** (*deadlock*). Les verrous de Go ne sont pas
« réentrants » : une goroutine qui tient déjà le verrou ne peut pas le reprendre. Rendez
d'abord le `RLock`, puis prenez le `Lock`.

## Testez-vous

**Question 1 :** pourquoi `compteur++` dans 1000 goroutines peut-il donner moins de
1000 ? Comment le voir ?

> **Réponse**
>
> `compteur++` se fait en trois étapes (lire, ajouter, écrire). Deux goroutines peuvent
> lire la même valeur et écrire le même résultat : un ajout se perd. `go run -race`
> affiche alors `WARNING: DATA RACE`.

**Question 2 :** qu'affiche ce programme ?

```go norun
var once sync.Once
for i := 0; i < 3; i++ {
	once.Do(func() { fmt.Println("tour", i) })
}
```

> **Réponse**
>
> Seulement `tour 0`. La fonction passée à `once.Do` n'est appelée qu'au premier tour ;
> les deux suivants l'ignorent.

**Question 3 :** une liste de prix est lue des milliers de fois par seconde et mise à
jour une fois par heure. Quel verrou choisir ?

> **Réponse**
>
> Un `sync.RWMutex` : `RLock` pour les lectures, qui passent toutes ensemble, et `Lock`
> pour la mise à jour, rare.

## En résumé

- Une situation de compétition perd des données sans prévenir : utilisez `-race`.
- `Mutex` : un seul à la fois. `RWMutex` : plusieurs lecteurs **ou** un seul écrivain.
- `sync.Once` (et `sync.OnceValue`) fait une initialisation une seule fois.
- `atomic.Int64` compte sans verrou avec `Add` et `Load`.
- `errgroup` (module `golang.org/x/sync`) attend des goroutines et renvoie la première
  erreur.
- Ne copiez jamais un verrou ; rendez-le avec `defer`.

Suite : [lire et écrire des fichiers](fichiers.md).
