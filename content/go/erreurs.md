# Gérer les erreurs

Cette page explique comment Go traite les erreurs comme des valeurs ordinaires, avec `error`, `if err != nil`, `fmt.Errorf` et `errors.Is`.

> **Objectifs :**
>
> - vous saurez lire et vérifier une erreur avec `if err != nil` ;
> - vous saurez créer une erreur avec `errors.New` et `fmt.Errorf` ;
> - vous saurez emballer une erreur avec `%w` et la retrouver avec `errors.Is` et `errors.As` ;
> - vous saurez écrire vos propres types d'erreur ;
> - vous saurez quand utiliser `panic` et `recover` (rarement).

## Une erreur est une valeur

Beaucoup de langages lèvent une [exception](../debuter/glossaire.md#exception) qui
interrompt le programme (voir [les erreurs en Ruby](../ruby/erreurs.md)). Go fait
autrement : une fonction qui peut échouer **renvoie** l'[erreur](../debuter/glossaire.md#erreur)
comme un résultat de plus, en dernière position.

Pensez à un livreur. Il ne crie pas dans la rue quand le colis est abîmé : il vous
tend le colis **et** une note « colis abîmé ». À vous de lire la note.

Le type des erreurs s'appelle `error`. Une erreur absente vaut `nil`.

```go
package main

import (
	"fmt"
	"strconv"
)

func main() {
	n, err := strconv.Atoi("42")
	fmt.Println(n, err)

	n, err = strconv.Atoi("douze")
	fmt.Println(n, err)
}
```

```sortie
42 <nil>
0 strconv.Atoi: parsing "douze": invalid syntax
```

`strconv.Atoi` transforme un texte en entier. Quand le texte n'est pas un nombre, il
renvoie `0` et une erreur qui explique le problème.

## if err != nil

Après chaque appel qui peut échouer, on vérifie l'erreur tout de suite. C'est la
ligne la plus célèbre de Go :

```go
package main

import (
	"fmt"
	"strconv"
)

func main() {
	saisies := []string{"12", "abc", "7"}
	for _, s := range saisies {
		n, err := strconv.Atoi(s)
		if err != nil {
			fmt.Println("Ignoré :", err)
			continue
		}
		fmt.Println("Le double vaut", n*2)
	}
}
```

```sortie
Le double vaut 24
Ignoré : strconv.Atoi: parsing "abc": invalid syntax
Le double vaut 14
```

On traite le cas d'erreur **d'abord**, puis le code normal continue sans être
indenté. Cela rend le chemin heureux facile à suivre.

## Créer une erreur

### errors.New

Votre propre fonction peut renvoyer une erreur. `errors.New` en crée une avec un
message fixe. Par habitude, le message commence par une minuscule et ne finit pas par
un point.

```go
package main

import (
	"errors"
	"fmt"
)

func diviser(a, b float64) (float64, error) {
	if b == 0 {
		return 0, errors.New("division par zéro")
	}
	return a / b, nil
}

func main() {
	r, err := diviser(10, 4)
	if err != nil {
		fmt.Println("Erreur :", err)
		return
	}
	fmt.Println("Résultat :", r)

	_, err = diviser(1, 0)
	if err != nil {
		fmt.Println("Erreur :", err)
	}
}
```

```sortie
Résultat : 2.5
Erreur : division par zéro
```

Quand tout va bien, la fonction renvoie `nil` comme erreur.

### fmt.Errorf

`fmt.Errorf` fabrique une erreur comme `fmt.Sprintf` fabrique un texte : vous pouvez y
glisser des valeurs.

```go
package main

import "fmt"

func verifierAge(age int) error {
	if age < 0 {
		return fmt.Errorf("âge invalide : %d", age)
	}
	return nil
}

func main() {
	fmt.Println(verifierAge(12))
	fmt.Println(verifierAge(-3))
}
```

```sortie
<nil>
âge invalide : -3
```

## Emballer une erreur avec %w

Une erreur remonte souvent plusieurs fonctions. À chaque étage, on ajoute du
**contexte** : ce qu'on essayait de faire. Avec le verbe `%w` (*wrap*, « emballer »),
`fmt.Errorf` met l'erreur d'origine dans une nouvelle, comme un colis dans un carton
plus grand.

```go
package main

import (
	"fmt"
	"strconv"
)

func lirePrix(texte string) (int, error) {
	prix, err := strconv.Atoi(texte)
	if err != nil {
		return 0, fmt.Errorf("lecture du prix %q : %w", texte, err)
	}
	return prix, nil
}

func main() {
	_, err := lirePrix("dix")
	fmt.Println(err)
}
```

```sortie
lecture du prix "dix" : strconv.Atoi: parsing "dix": invalid syntax
```

Le message raconte toute l'histoire, du plus général au plus précis.

## Les erreurs sentinelles et errors.Is

Une **erreur sentinelle** est une erreur déclarée une fois, dans une variable, pour
qu'on puisse la reconnaître. Son nom commence par `Err`. La bibliothèque standard en a
plusieurs, comme `io.EOF` (« fin du fichier »).

`errors.Is(err, ErrX)` répond : « est-ce que `ErrX` se trouve dans `err`, même au fond
du carton ? ». Il ouvre tous les emballages faits avec `%w`.

```go
package main

import (
	"errors"
	"fmt"
)

var ErrIntrouvable = errors.New("introuvable")

var stock = map[string]int{"pomme": 3}

func quantite(fruit string) (int, error) {
	n, ok := stock[fruit]
	if !ok {
		return 0, fmt.Errorf("fruit %q : %w", fruit, ErrIntrouvable)
	}
	return n, nil
}

func main() {
	for _, f := range []string{"pomme", "kiwi"} {
		n, err := quantite(f)
		if errors.Is(err, ErrIntrouvable) {
			fmt.Println(err, "-> on commande des", f+"s")
			continue
		}
		fmt.Println(f, ":", n)
	}
}
```

```sortie
pomme : 3
fruit "kiwi" : introuvable -> on commande des kiwis
```

> **Attention :** ne comparez pas les messages (`err.Error() == "introuvable"`). Un
> message peut changer ; la variable sentinelle, non.

## Vos propres types d'erreur

`error` est une simple [interface](interfaces.md) :

```go norun
type error interface {
	Error() string
}
```

Tout type qui a une méthode `Error() string` est donc une erreur. Un type d'erreur
peut transporter des informations en plus du message, comme le champ fautif.

`errors.As(err, &cible)` cherche dans `err` une erreur du type de `cible`. S'il en
trouve une, il la range dans `cible` et renvoie `true`.

```go
package main

import (
	"errors"
	"fmt"
)

type ErreurValidation struct {
	Champ  string
	Raison string
}

func (e *ErreurValidation) Error() string {
	return e.Champ + " : " + e.Raison
}

func inscrire(nom string, age int) error {
	if nom == "" {
		return &ErreurValidation{Champ: "nom", Raison: "obligatoire"}
	}
	if age < 13 {
		return &ErreurValidation{Champ: "age", Raison: "13 ans minimum"}
	}
	return nil
}

func main() {
	err := inscrire("Lou", 9)
	err = fmt.Errorf("inscription refusée : %w", err)
	fmt.Println(err)

	var ev *ErreurValidation
	if errors.As(err, &ev) {
		fmt.Println("Champ à corriger :", ev.Champ)
	}
}
```

```sortie
inscription refusée : age : 13 ans minimum
Champ à corriger : age
```

> **À retenir :** `errors.Is` cherche **une valeur** précise (une sentinelle).
> `errors.As` cherche **un type** d'erreur et vous donne accès à ses champs.

## panic et recover

`panic` arrête le programme immédiatement, avec un message et la trace des appels.
C'est la [panique](../debuter/glossaire.md#panique). Go panique tout seul dans certains
cas : division entière par zéro, indice hors d'une slice, pointeur `nil`…

`recover`, appelé dans une fonction `defer`, rattrape une panique. `defer` programme un
appel pour la fin de la fonction, quoi qu'il arrive.

```go
package main

import "fmt"

func diviserProtege(a, b int) (resultat int, err error) {
	defer func() {
		if r := recover(); r != nil {
			err = fmt.Errorf("rattrapé : %v", r)
		}
	}()
	return a / b, nil
}

func main() {
	fmt.Println(diviserProtege(10, 2))
	fmt.Println(diviserProtege(1, 0))
	fmt.Println("Le programme continue.")
}
```

```sortie
5 <nil>
0 rattrapé : runtime error: integer divide by zero
Le programme continue.
```

### Quand paniquer ?

Presque jamais. Une panique signale un **bug** : une situation qui ne devrait jamais
arriver si le code était juste. Pour tout ce qui peut normalement échouer (un fichier
absent, une saisie incorrecte, un réseau coupé), renvoyez une `error`.

- Saisie invalide, fichier manquant : `return ..., err`.
- Bug du programmeur, état impossible : `panic`.
- `recover` : seulement pour qu'un serveur, par exemple, survive à un bug dans une
  requête. Pas pour remplacer `if err != nil`.

## Erreurs fréquentes

### Oublier la valeur d'erreur

```go norun
package main

import (
	"fmt"
	"strconv"
)

func main() {
	n := strconv.Atoi("12")
	fmt.Println(n)
}
```

```text
# command-line-arguments
./main.go:9:7: assignment mismatch: 1 variable but strconv.Atoi returns 2 values
```

`strconv.Atoi` renvoie deux valeurs : le nombre et l'erreur. Écrivez
`n, err := strconv.Atoi("12")`, puis vérifiez `err`.

### Récupérer l'erreur sans la vérifier

```go norun
package main

import (
	"fmt"
	"strconv"
)

func main() {
	n, err := strconv.Atoi("12")
	fmt.Println(n)
}
```

```text
# command-line-arguments
./main.go:9:5: declared and not used: err
```

Go refuse une variable inutilisée. Ajoutez le test `if err != nil { ... }`.

### Ignorer l'erreur avec _

```go norun
package main

import (
	"fmt"
	"strconv"
)

func main() {
	n, _ := strconv.Atoi("douze")
	fmt.Println(100 / n)
}
```

```text
panic: runtime error: integer divide by zero

goroutine 1 [running]:
main.main()
	/chemin/vers/main.go:10 +0x78
exit status 2
```

Le `_` jette l'erreur : `n` vaut `0` sans prévenir, et la division panique deux lignes
plus loin. Le vrai problème est loin du message. Gardez l'erreur et testez-la.

### Emballer avec %v au lieu de %w

```go norun
err := fmt.Errorf("fruit %q : %v", fruit, ErrIntrouvable)
fmt.Println(errors.Is(err, ErrIntrouvable))
```

```text
false
```

Avec `%v`, seul le **texte** de l'erreur est copié : l'erreur d'origine est perdue et
`errors.Is` ne la retrouve pas. Utilisez `%w`.

## Testez-vous

**Question 1 :** que renvoie, par habitude, une fonction `func f() (int, error)` quand
tout se passe bien ?

> **Réponse**
>
> Le résultat et `nil` : `return resultat, nil`. Une erreur `nil` veut dire « pas
> d'erreur ».

**Question 2 :** quelle différence entre `errors.Is` et `errors.As` ?

> **Réponse**
>
> `errors.Is` vérifie si une erreur précise (une sentinelle comme `io.EOF`) se trouve
> dans la chaîne. `errors.As` cherche une erreur d'un certain **type** et la range dans
> une variable pour lire ses champs.

**Question 3 :** écrivez une fonction `retirer(solde, montant int) (int, error)` qui
renvoie l'erreur sentinelle `ErrSoldeInsuffisant` si le montant dépasse le solde.

> **Réponse**
>
> ```go
> package main
>
> import (
> 	"errors"
> 	"fmt"
> )
>
> var ErrSoldeInsuffisant = errors.New("solde insuffisant")
>
> func retirer(solde, montant int) (int, error) {
> 	if montant > solde {
> 		return solde, ErrSoldeInsuffisant
> 	}
> 	return solde - montant, nil
> }
>
> func main() {
> 	fmt.Println(retirer(100, 30))
> 	_, err := retirer(10, 50)
> 	fmt.Println(errors.Is(err, ErrSoldeInsuffisant))
> }
> ```
>
> ```sortie
> 70 <nil>
> true
> ```

## En résumé

- Une erreur est une valeur de type `error`, renvoyée en dernier ; `nil` veut dire
  « pas d'erreur ».
- Vérifiez chaque erreur tout de suite avec `if err != nil`.
- `errors.New` et `fmt.Errorf` créent des erreurs ; `%w` en emballe une autre.
- `errors.Is` reconnaît une sentinelle ; `errors.As` retrouve un type d'erreur.
- `panic` est réservé aux bugs ; `recover` dans un `defer` la rattrape.

Suite : [packages et modules](modules.md). Comparez avec
[les erreurs en Rust](../rust/erreurs.md).
