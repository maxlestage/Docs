# Annuler et limiter dans le temps avec context

Cette page montre comment le paquet `context` permet de dire à une goroutine « arrête-toi, plus personne n'attend ton travail », tout de suite ou au bout d'un certain temps.

> **Objectifs :**
>
> - comprendre à quoi sert un `context` ;
> - annuler un travail avec `context.WithCancel` ;
> - fixer un délai maximum avec `context.WithTimeout` ;
> - écouter `ctx.Done()` dans un `select` et lire `ctx.Err()` ;
> - passer `ctx` en premier paramètre de vos fonctions.

Ce chapitre suppose que vous connaissez [les goroutines](concurrence.md) et
[les canaux](canaux.md), surtout `select`.

## Pourquoi annuler un travail ?

Imaginez une pizzeria. Vous commandez une pizza par téléphone, puis vous raccrochez :
vous avez changé d'avis. Si personne ne prévient le pizzaïolo, il continue à cuire une
pizza que personne ne mangera.

Un programme a le même problème. Une [goroutine](../debuter/glossaire.md#goroutine)
lancée pour un travail continue, même quand plus personne n'attend le résultat :

- l'utilisateur a fermé la page web ;
- la réponse arrive trop tard ;
- une autre goroutine a déjà trouvé la réponse.

Le **context** (le « contexte ») est le téléphone qui permet de prévenir le
pizzaïolo : « stop, laisse tomber ».

```text
 main ──── ctx ────► goroutine
   │                    │ travaille…
 cancel() ─────────►  ctx.Done() se ferme
                        │ « on m'a dit d'arrêter »
                        └─ s'arrête
```

> **À retenir :** un `context` transporte un signal « arrête-toi ». Celui qui lance le
> travail l'envoie ; celui qui travaille l'écoute.

## Le point de départ : context.Background

Tout contexte part d'une racine : `context.Background()`. C'est un contexte vide. Il
n'est jamais annulé.

```go
package main

import (
	"context"
	"fmt"
)

func main() {
	ctx := context.Background()
	fmt.Println(ctx)
	fmt.Println(ctx.Err())
}
```

```sortie
context.Background
<nil>
```

`ctx.Err()` dit si le contexte est terminé. `<nil>` veut dire « non, tout va bien ».

> **Note :** par habitude, la variable s'appelle toujours `ctx`.

## Annuler à la main : WithCancel

`context.WithCancel(parent)` renvoie **deux** choses :

- un nouveau contexte `ctx`, enfant de `parent` ;
- une fonction `cancel`. L'appeler annule `ctx`.

```go
package main

import (
	"context"
	"fmt"
)

func main() {
	ctx, cancel := context.WithCancel(context.Background())
	fmt.Println("avant :", ctx.Err())
	cancel()
	fmt.Println("après :", ctx.Err())
}
```

```sortie
avant : <nil>
après : context canceled
```

| Étape | Ce qui se passe | ctx.Err() |
|---|---|---|
| 1 | `WithCancel` crée `ctx` | `<nil>` |
| 2 | `cancel()` | `context canceled` |

Appeler `cancel` une deuxième fois ne fait rien de plus. Ce n'est pas une erreur.

## Écouter ctx.Done() dans un select

`ctx.Done()` est un [canal](../debuter/glossaire.md#canal). Il se **ferme** quand le
contexte est annulé. Recevoir depuis un canal fermé réussit tout de suite : c'est le
signal.

Voici un travailleur qui attend. `main` l'annule, puis attend qu'il ait fini.

```go
package main

import (
	"context"
	"fmt"
)

func travailleur(ctx context.Context, fini chan<- bool) {
	<-ctx.Done()
	fmt.Println("travailleur : on m'a dit d'arrêter :", ctx.Err())
	fini <- true
}

func main() {
	ctx, cancel := context.WithCancel(context.Background())
	fini := make(chan bool)

	go travailleur(ctx, fini)
	fmt.Println("main : j'annule")
	cancel()
	<-fini
	fmt.Println("main : terminé")
}
```

```sortie
main : j'annule
travailleur : on m'a dit d'arrêter : context canceled
main : terminé
```

Le travailleur ne peut rien afficher avant `cancel()` : il est bloqué sur `<-ctx.Done()`.
L'ordre est donc toujours le même.

### Un producteur qu'on arrête

En vrai, un travailleur fait autre chose qu'attendre. Il met alors `ctx.Done()` dans un
`select`, à côté de son travail. Ici, un producteur envoie 1, 2, 3, 4… sans fin. `main`
en lit trois, puis annule.

```go
package main

import (
	"context"
	"fmt"
)

func producteur(ctx context.Context, sortie chan<- int, fini chan<- bool) {
	for n := 1; ; n++ {
		select {
		case sortie <- n:
			// envoyé, on continue avec le suivant
		case <-ctx.Done():
			fmt.Println("producteur : arrêté")
			fini <- true
			return
		}
	}
}

func main() {
	ctx, cancel := context.WithCancel(context.Background())
	nombres := make(chan int)
	fini := make(chan bool)

	go producteur(ctx, nombres, fini)
	for i := 0; i < 3; i++ {
		fmt.Println("reçu :", <-nombres)
	}
	cancel()
	<-fini
}
```

```sortie
reçu : 1
reçu : 2
reçu : 3
producteur : arrêté
```

Pas à pas :

| Étape | main | producteur |
|---|---|---|
| 1 | reçoit 1 | envoie 1 |
| 2 | reçoit 2 | envoie 2 |
| 3 | reçoit 3 | envoie 3 |
| 4 | appelle `cancel()` | attend dans `select` pour envoyer 4 |
| 5 | attend `fini` | `ctx.Done()` est prêt : il s'arrête |

Le 4 n'est jamais envoyé : plus personne ne lit `nombres`. Seul le cas `ctx.Done()`
peut gagner.

Essayez : remplacez `i < 3` par `i < 5`. Vous recevez 1 à 5, puis `producteur : arrêté`.

> **À retenir :** un travailleur qui peut être annulé contient presque toujours
> `case <-ctx.Done():` dans son `select`.

## Toujours appeler cancel avec defer

Un contexte annulable garde un peu de [mémoire](../debuter/glossaire.md#memoire-vive)
tant qu'il n'est pas annulé. La règle est simple : juste après avoir créé le contexte,
écrivez `defer cancel()`.

```go norun
ctx, cancel := context.WithCancel(context.Background())
defer cancel() // annulé à la fin de la fonction, quoi qu'il arrive
```

`defer` repousse l'appel à la fin de la fonction (voir
[defer](fonctions.md#defer-repousser-a-la-fin)). Même si la fonction s'arrête plus tôt,
le contexte sera nettoyé.

## Un délai maximum : WithTimeout

`context.WithTimeout(parent, durée)` crée un contexte qui s'annule **tout seul** au bout
de la durée. C'est un minuteur de cuisine : quand il sonne, on arrête.

Le travailleur ci-dessous attend un résultat sur un canal qui ne reçoit **jamais** rien.
C'est donc toujours le délai qui gagne.

```go
package main

import (
	"context"
	"fmt"
	"time"
)

func main() {
	ctx, cancel := context.WithTimeout(context.Background(), 50*time.Millisecond)
	defer cancel()

	jamais := make(chan int)
	select {
	case r := <-jamais:
		fmt.Println("résultat :", r)
	case <-ctx.Done():
		fmt.Println("abandon :", ctx.Err())
	}
}
```

```sortie
abandon : context deadline exceeded
```

`deadline exceeded` veut dire « délai dépassé ». C'est la deuxième raison possible de
`ctx.Err()`.

À l'inverse, si le résultat est déjà là, il passe avant le délai :

```go
package main

import (
	"context"
	"fmt"
	"time"
)

func main() {
	ctx, cancel := context.WithTimeout(context.Background(), time.Second)
	defer cancel()

	resultat := make(chan int, 1)
	resultat <- 42

	select {
	case r := <-resultat:
		fmt.Println("résultat :", r)
	case <-ctx.Done():
		fmt.Println("abandon :", ctx.Err())
	}
	fmt.Println("ctx.Err() :", ctx.Err())
}
```

```sortie
résultat : 42
ctx.Err() : <nil>
```

Le délai d'une seconde n'est pas écoulé : `ctx.Err()` vaut encore `<nil>`.

> **Note :** `WithTimeout` ressemble à `time.After` (voir
> [un délai maximum avec time.After](canaux.md#un-delai-maximum-avec-time-after)). La
> différence : le contexte se **transmet** à d'autres fonctions, qui s'arrêtent toutes
> en même temps.

## Lire la raison : ctx.Err()

`ctx.Err()` a trois valeurs possibles :

| Situation | ctx.Err() |
|---|---|
| le contexte tourne encore | `nil` |
| quelqu'un a appelé `cancel()` | `context.Canceled` |
| le délai est dépassé | `context.DeadlineExceeded` |

Pour tester la raison, utilisez `errors.Is`.

```go
package main

import (
	"context"
	"errors"
	"fmt"
	"time"
)

func main() {
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Millisecond)
	defer cancel()
	<-ctx.Done()

	err := ctx.Err()
	if errors.Is(err, context.DeadlineExceeded) {
		fmt.Println("trop lent")
	} else if errors.Is(err, context.Canceled) {
		fmt.Println("annulé")
	}
}
```

```sortie
trop lent
```

## Passer ctx en premier paramètre

Une fonction qui peut prendre du temps reçoit le contexte en **premier** paramètre,
nommé `ctx`. C'est une convention suivie dans toute la bibliothèque de Go.

```go
package main

import (
	"context"
	"fmt"
	"time"
)

// chercher attend une réponse qui ne vient jamais : seul ctx peut l'arrêter.
func chercher(ctx context.Context, mot string) (string, error) {
	reponse := make(chan string)
	select {
	case r := <-reponse:
		return r, nil
	case <-ctx.Done():
		return "", ctx.Err()
	}
}

func main() {
	ctx, cancel := context.WithTimeout(context.Background(), 20*time.Millisecond)
	defer cancel()

	_, err := chercher(ctx, "gopher")
	if err != nil {
		fmt.Println("chercher a échoué :", err)
	}
}
```

```sortie
chercher a échoué : context deadline exceeded
```

Le contexte descend de fonction en fonction, comme le téléphone passe de main en main.
Si `chercher` appelait d'autres fonctions, elle leur donnerait le même `ctx`.

> **À retenir :** `func nom(ctx context.Context, …)`. Ne rangez pas un contexte dans
> une structure : passez-le à chaque appel.

## Les enfants suivent le parent

Un contexte créé à partir d'un autre est son **enfant**. Annuler le parent annule tous
ses enfants. L'inverse n'est pas vrai.

```go
package main

import (
	"context"
	"fmt"
)

func main() {
	parent, annulerParent := context.WithCancel(context.Background())
	enfant, annulerEnfant := context.WithCancel(parent)
	defer annulerEnfant()

	annulerParent()
	<-enfant.Done()
	fmt.Println("enfant :", enfant.Err())
}
```

```sortie
enfant : context canceled
```

```text
Background
   └── parent   ← cancel ici…
         └── enfant   ← …annule aussi celui-ci
```

## Transporter une valeur : WithValue (avec modération)

`context.WithValue(parent, clé, valeur)` accroche une petite étiquette au contexte. On
s'en sert pour une information qui suit une requête, comme un numéro de suivi.

```go
package main

import (
	"context"
	"fmt"
)

type cle string

const cleSuivi cle = "suivi"

func traiter(ctx context.Context) {
	numero, ok := ctx.Value(cleSuivi).(string)
	if !ok {
		numero = "inconnu"
	}
	fmt.Println("traitement de la demande", numero)
}

func main() {
	ctx := context.WithValue(context.Background(), cleSuivi, "A-42")
	traiter(ctx)
	traiter(context.Background())
}
```

```sortie
traitement de la demande A-42
traitement de la demande inconnu
```

- La clé a son propre type (`cle`). Ainsi, elle ne peut pas se mélanger avec la clé
  d'un autre paquet.
- `ctx.Value` renvoie une valeur de type `any`. Le `.(string)` vérifie que c'est un
  texte (voir [les interfaces](interfaces.md)).

> **Attention :** n'utilisez pas `WithValue` pour passer les paramètres normaux d'une
> fonction. Un paramètre écrit dans la signature se lit bien mieux.

## Erreurs fréquentes

### Jeter la fonction cancel

```go norun
package main

import (
	"context"
	"fmt"
	"time"
)

func main() {
	ctx, _ := context.WithTimeout(context.Background(), time.Second)
	fmt.Println(ctx.Err())
}
```

Le programme compile, mais `go vet main.go` vous prévient :

```text
# command-line-arguments
# [command-line-arguments]
./main.go:10:7: the cancel function returned by context.WithTimeout should be called, not discarded, to avoid a context leak
```

Gardez `cancel` et écrivez `defer cancel()` juste en dessous.

### Passer un contexte nil

```go norun
package main

import (
	"context"
	"fmt"
	"time"
)

func main() {
	var ctx context.Context
	ctx, cancel := context.WithTimeout(ctx, time.Second)
	defer cancel()
	fmt.Println(ctx.Err())
}
```

```text
panic: cannot create context from nil parent

goroutine 1 [running]:
context.WithDeadlineCause({0x0?, 0x0?}, {0x57dbe0?, 0x3b9aca00?, 0x57dbe0?}, {0x0?, 0x0?})
	/usr/local/go/src/context/context.go:625 +0x39e
…
exit status 2
```

Un contexte déclaré avec `var` vaut `nil`. Partez toujours de `context.Background()`.

### Oublier d'écouter ctx.Done()

```go norun
func producteur(ctx context.Context, sortie chan<- int) {
	for n := 1; ; n++ {
		sortie <- n // ctx n'est jamais regardé
	}
}
```

Ici, rien ne s'affiche comme erreur. Mais annuler `ctx` ne sert à rien : la goroutine
reste bloquée sur `sortie <- n` pour toujours. Recevoir `ctx` ne suffit pas, il faut
l'**écouter** dans un `select`, comme dans [le producteur](#un-producteur-qu-on-arrete).

## Testez-vous

**Question 1 :** que vaut `ctx.Err()` juste après `context.WithCancel(...)`, puis
après `cancel()` ?

> **Réponse**
>
> D'abord `nil` (le contexte tourne). Après `cancel()`, `context canceled`.

**Question 2 :** qu'affiche ce programme ?

```go norun
ctx, cancel := context.WithTimeout(context.Background(), 30*time.Millisecond)
defer cancel()
jamais := make(chan string)
select {
case m := <-jamais:
	fmt.Println(m)
case <-ctx.Done():
	fmt.Println(ctx.Err())
}
```

> **Réponse**
>
> `context deadline exceeded`. Le canal `jamais` ne reçoit rien : au bout de 30 ms, le
> contexte s'annule et le second cas gagne.

**Question 3 :** où place-t-on le contexte dans la liste des paramètres d'une
fonction ?

> **Réponse**
>
> En premier, sous le nom `ctx` : `func telecharger(ctx context.Context, url string)`.

## En résumé

- Un `context` transporte le signal « arrête-toi » vers les goroutines.
- Tout part de `context.Background()`.
- `WithCancel` donne une fonction `cancel` ; `WithTimeout` s'annule tout seul. Écrivez
  toujours `defer cancel()`.
- Le travailleur écoute `case <-ctx.Done():` dans un `select` ; `ctx.Err()` donne la
  raison (`Canceled` ou `DeadlineExceeded`).
- `ctx` se passe en premier paramètre ; `WithValue` reste pour de petites informations.

Suite : [lire et écrire des fichiers](fichiers.md).
