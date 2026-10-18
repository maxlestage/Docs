# Les canaux

Cette page montre comment des goroutines s'échangent des valeurs avec les canaux, et comment combiner plusieurs canaux avec `select`.

> **Objectifs :**
>
> - vous saurez créer un canal, y envoyer et y recevoir des valeurs ;
> - vous saurez choisir entre un canal sans tampon et un canal avec tampon ;
> - vous saurez fermer un canal et le parcourir avec `range` ;
> - vous saurez attendre plusieurs canaux avec `select` et prévoir un délai maximum ;
> - vous saurez enchaîner des goroutines en pipeline et éviter l'interblocage.

## Un tuyau entre goroutines

Dans [le chapitre précédent](concurrence.md), les goroutines partageaient des variables
protégées par un verrou. Go propose une autre façon de faire, souvent plus simple : les
**canaux** (*channels*). Un proverbe Go dit : « ne communiquez pas en partageant la
mémoire ; partagez la mémoire en communiquant ».

Un canal est un tuyau. Une goroutine glisse une valeur d'un côté, une autre la récupère
de l'autre côté. Le tuyau a un type : un `chan int` ne transporte que des entiers.

- `ch := make(chan int)` crée le canal ;
- `ch <- 5` **envoie** 5 dans le canal (la flèche entre dans `ch`) ;
- `x := <-ch` **reçoit** une valeur (la flèche sort de `ch`).

```go
package main

import "fmt"

func main() {
	ch := make(chan string)

	go func() {
		ch <- "ping"
	}()

	message := <-ch
	fmt.Println(message)
}
```

```sortie
ping
```

`<-ch` **attend** qu'une valeur arrive. Pas besoin de `sync.WaitGroup` ici : la
réception sert aussi de rendez-vous.

## Sans tampon ou avec tampon

### Sans tampon : un passage de main en main

Un canal créé avec `make(chan int)` n'a **pas de tampon** : il ne stocke rien.
L'envoi attend que quelqu'un reçoive, et la réception attend que quelqu'un envoie.
C'est un relais en course à pied : le coureur ne lâche le témoin que lorsque le suivant
l'a attrapé.

```go
package main

import "fmt"

func main() {
	ch := make(chan int)

	go func() {
		for i := 1; i <= 3; i++ {
			fmt.Println("envoi", i)
			ch <- i
		}
	}()

	for i := 0; i < 3; i++ {
		n := <-ch
		fmt.Println("reçu", n)
	}
}
```

Ici, les lignes « envoi » et « reçu » peuvent se croiser différemment d'une exécution
à l'autre : seul l'ordre des nombres reçus est garanti (1, 2, puis 3).

### Avec tampon : une boîte aux lettres

`make(chan int, 3)` crée un canal avec un **tampon** de 3 places. L'envoi ne bloque
pas tant qu'il reste de la place ; la réception ne bloque pas tant qu'il reste une
valeur. C'est une boîte aux lettres : le facteur dépose sans attendre que vous soyez là,
tant que la boîte n'est pas pleine.

```go
package main

import "fmt"

func main() {
	ch := make(chan string, 3)
	ch <- "a"
	ch <- "b"
	ch <- "c"
	fmt.Println("Dans la boîte :", len(ch), "sur", cap(ch))

	fmt.Println(<-ch)
	fmt.Println(<-ch)
	fmt.Println(<-ch)
}
```

```sortie
Dans la boîte : 3 sur 3
a
b
c
```

Les valeurs sortent dans l'ordre où elles sont entrées : premier entré, premier sorti.
Ici, tout se passe dans `main`, sans goroutine : c'est possible grâce au tampon.

> **À retenir :** commencez par des canaux sans tampon. Ajoutez un tampon quand vous
> savez combien de valeurs seront envoyées, ou pour lisser un débit.

## Fermer un canal et le parcourir

Quand l'envoyeur n'a plus rien à envoyer, il **ferme** le canal avec `close(ch)`. Le
receveur peut alors parcourir le canal avec `for ... range` : la boucle s'arrête toute
seule quand le canal est fermé et vide.

```go
package main

import "fmt"

func compter(jusqua int, ch chan<- int) {
	for i := 1; i <= jusqua; i++ {
		ch <- i
	}
	close(ch)
}

func main() {
	ch := make(chan int)
	go compter(4, ch)

	for n := range ch {
		fmt.Println(n)
	}
	fmt.Println("Terminé")
}
```

```sortie
1
2
3
4
Terminé
```

Le type `chan<- int` veut dire « canal dans lequel on peut seulement envoyer ». De même,
`<-chan int` veut dire « seulement recevoir ». Le compilateur vérifie alors que chaque
fonction utilise le canal dans le bon sens.

### Savoir si un canal est fermé

Recevoir sur un canal fermé ne bloque pas : on obtient la valeur zéro. La forme
`v, ok := <-ch` dit si la valeur est vraie (`ok` vaut `true`) ou si le canal est fermé.

```go
package main

import "fmt"

func main() {
	ch := make(chan int, 1)
	ch <- 7
	close(ch)

	v, ok := <-ch
	fmt.Println(v, ok)
	v, ok = <-ch
	fmt.Println(v, ok)
}
```

```sortie
7 true
0 false
```

> **Attention :** seul l'envoyeur ferme le canal, et une seule fois. Envoyer dans un
> canal fermé provoque une panique.

## select : attendre plusieurs canaux

`select` ressemble à un `switch`, mais pour les canaux. Il attend que **l'un** des cas
soit prêt, puis exécute celui-là. C'est un standardiste devant plusieurs téléphones : il
décroche celui qui sonne.

```go
package main

import "fmt"

func main() {
	messages := make(chan string)
	fini := make(chan bool)

	go func() {
		for _, m := range []string{"un", "deux", "trois"} {
			messages <- m
		}
		close(fini)
	}()

	for {
		select {
		case m := <-messages:
			fmt.Println("message :", m)
		case <-fini:
			fmt.Println("plus de messages")
			return
		}
	}
}
```

```sortie
message : un
message : deux
message : trois
plus de messages
```

Si plusieurs cas sont prêts en même temps, `select` en choisit un **au hasard**. Ici,
ce n'est jamais le cas : le canal `messages` est sans tampon, donc chaque message est
reçu avant que la goroutine passe au suivant, puis ferme `fini`.

## Un délai maximum avec time.After

`time.After(d)` renvoie un canal qui reçoit une valeur au bout de la durée `d`. Placé
dans un `select`, il fixe un **délai maximum** (*timeout*) : si la réponse n'arrive pas à
temps, on abandonne.

```go
package main

import (
	"fmt"
	"time"
)

func calcul(duree time.Duration) <-chan int {
	ch := make(chan int, 1)
	go func() {
		time.Sleep(duree)
		ch <- 42
	}()
	return ch
}

func attendre(ch <-chan int) {
	select {
	case r := <-ch:
		fmt.Println("Résultat :", r)
	case <-time.After(100 * time.Millisecond):
		fmt.Println("Trop long, on abandonne.")
	}
}

func main() {
	attendre(calcul(10 * time.Millisecond))
	attendre(calcul(500 * time.Millisecond))
}
```

```sortie
Résultat : 42
Trop long, on abandonne.
```

Le canal de `calcul` a un tampon de 1 : même si personne n'écoute plus, la goroutine
peut déposer son résultat et se terminer.

## Le pipeline

Un **pipeline** est une chaîne de goroutines reliées par des canaux, comme une chaîne de
montage : chaque poste fait une seule chose et passe le résultat au suivant.

Voici trois étapes : générer des nombres, les mettre au carré, puis les additionner.

```go
package main

import "fmt"

func generer(nombres ...int) <-chan int {
	sortie := make(chan int)
	go func() {
		for _, n := range nombres {
			sortie <- n
		}
		close(sortie)
	}()
	return sortie
}

func carres(entree <-chan int) <-chan int {
	sortie := make(chan int)
	go func() {
		for n := range entree {
			sortie <- n * n
		}
		close(sortie)
	}()
	return sortie
}

func main() {
	total := 0
	for c := range carres(generer(1, 2, 3, 4)) {
		fmt.Println("carré :", c)
		total += c
	}
	fmt.Println("total :", total)
}
```

```sortie
carré : 1
carré : 4
carré : 9
carré : 16
total : 30
```

Chaque étape ferme son canal de sortie quand son entrée est épuisée. La fermeture se
propage ainsi d'un bout à l'autre, et la boucle de `main` s'arrête toute seule.

### Des travailleurs avec des canaux

Le groupe de travailleurs du [chapitre précédent](concurrence.md) s'écrit naturellement
avec deux canaux : un pour les tâches, un pour les résultats.

```go
package main

import (
	"fmt"
	"sort"
	"sync"
)

func main() {
	taches := make(chan int)
	resultats := make(chan int)

	var wg sync.WaitGroup
	for t := 1; t <= 3; t++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			for n := range taches {
				resultats <- n * 10
			}
		}()
	}

	go func() {
		for n := 1; n <= 5; n++ {
			taches <- n
		}
		close(taches)
	}()

	go func() {
		wg.Wait()
		close(resultats)
	}()

	var tous []int
	for r := range resultats {
		tous = append(tous, r)
	}
	sort.Ints(tous)
	fmt.Println(tous)
}
```

```sortie
[10 20 30 40 50]
```

Les résultats arrivent dans le désordre, d'où le tri. Une goroutine ferme `resultats`
quand tous les travailleurs ont fini.

## Erreurs fréquentes

### Envoyer sans personne pour recevoir

```go norun
package main

import "fmt"

func main() {
	ch := make(chan int)
	ch <- 1
	fmt.Println(<-ch)
}
```

```text
fatal error: all goroutines are asleep - deadlock!

goroutine 1 [chan send]:
main.main()
	/chemin/vers/main.go:7 +0x36
exit status 2
```

C'est un **interblocage** (*deadlock*) : l'envoi attend un receveur, mais le seul
receveur possible est la ligne suivante, qui n'arrivera jamais. Go s'en rend compte et
arrête tout. Envoyez depuis une goroutine, ou donnez un tampon au canal :
`make(chan int, 1)`.

### Oublier de fermer le canal

```go norun
package main

import "fmt"

func main() {
	ch := make(chan int)
	go func() {
		for i := 1; i <= 3; i++ {
			ch <- i
		}
	}()
	for n := range ch {
		fmt.Println(n)
	}
}
```

```text
1
2
3
fatal error: all goroutines are asleep - deadlock!

goroutine 1 [chan receive]:
main.main()
	/chemin/vers/main.go:12 +0xc5
exit status 2
```

`range` attend une quatrième valeur qui ne viendra jamais. Ajoutez `close(ch)` après la
boucle d'envoi.

### Envoyer dans un canal fermé

```go norun
package main

import "fmt"

func main() {
	ch := make(chan int, 1)
	close(ch)
	ch <- 1
	fmt.Println("fin")
}
```

```text
panic: send on closed channel

goroutine 1 [running]:
main.main()
	/chemin/vers/main.go:8 +0x3a
exit status 2
```

Un canal fermé ne reçoit plus rien. Fermez le canal seulement quand plus aucun envoi
n'aura lieu, et toujours du côté de l'envoyeur.

## Testez-vous

**Question 1 :** quelle différence entre `make(chan int)` et `make(chan int, 5)` ?

> **Réponse**
>
> Le premier n'a pas de tampon : chaque envoi attend un receveur. Le second peut garder
> jusqu'à 5 valeurs : les envois ne bloquent que lorsqu'il est plein.

**Question 2 :** que se passe-t-il dans un `select` si aucun cas n'est prêt et qu'il n'y
a pas de `default` ?

> **Réponse**
>
> Le `select` attend qu'un des cas devienne prêt. Avec un cas `time.After`, l'attente
> est limitée dans le temps.

**Question 3 :** écrivez une fonction `lettres(mot string) <-chan string` qui envoie
chaque lettre du mot dans un canal puis le ferme. Affichez-les avec `range`.

> **Réponse**
>
> ```go
> package main
>
> import "fmt"
>
> func lettres(mot string) <-chan string {
> 	ch := make(chan string)
> 	go func() {
> 		for _, r := range mot {
> 			ch <- string(r)
> 		}
> 		close(ch)
> 	}()
> 	return ch
> }
>
> func main() {
> 	for l := range lettres("Go!") {
> 		fmt.Println(l)
> 	}
> }
> ```
>
> ```sortie
> G
> o
> !
> ```

## En résumé

- Un canal transporte des valeurs entre goroutines : `ch <- v` envoie, `<-ch` reçoit.
- Sans tampon, l'envoi et la réception se donnent rendez-vous ; avec tampon, le canal
  garde quelques valeurs.
- L'envoyeur ferme le canal avec `close` ; `range` le lit jusqu'à la fermeture.
- `select` attend plusieurs canaux ; `time.After` fixe un délai maximum.
- Un pipeline enchaîne des étapes reliées par des canaux ; un envoi sans receveur
  provoque `all goroutines are asleep - deadlock!`.

Suite : [lire et écrire des fichiers](fichiers.md).
