# Packages et modules

Cette page explique comment Go range le code en packages, comment créer un module avec `go mod init` et comment utiliser du code écrit par d'autres.

> **Objectifs :**
>
> - vous saurez importer un package et utiliser ses noms exportés ;
> - vous saurez créer un module avec `go mod init` et lire le fichier `go.mod` ;
> - vous saurez répartir un projet sur plusieurs fichiers et dossiers ;
> - vous saurez protéger du code avec un dossier `internal` ;
> - vous saurez ajouter un module externe et lire la documentation avec `go doc`.

## Les packages

Un **package** (« paquet ») est un dossier de fichiers Go qui travaillent ensemble.
Chaque fichier commence par `package nom`. Le package `main` est spécial : c'est celui
d'un [programme](../debuter/glossaire.md#programme), et il contient la fonction `main`.

Pensez à une bibliothèque municipale : chaque rayon (package) regroupe des livres sur un
même sujet. Vous n'apportez pas tout le bâtiment chez vous, vous empruntez le rayon
dont vous avez besoin.

`import` rend un package disponible. Ensuite, on écrit le nom du package, un point, puis
la fonction : `strings.ToUpper`.

```go
package main

import (
	"fmt"
	"math"
	"strings"
)

func main() {
	fmt.Println(strings.ToUpper("bonjour"))
	fmt.Println(math.Sqrt(16))
}
```

```sortie
BONJOUR
4
```

### Le chemin et le nom

Le texte entre guillemets est le **chemin** du package. Le nom à utiliser dans le code
est le dernier morceau du chemin : avec `"math/rand"`, on écrit `rand.Intn`. Si deux
packages ont le même nom, donnez un **alias** à l'un d'eux.

```go
package main

import (
	"fmt"
	str "strings"
	"unicode/utf8"
)

func main() {
	mot := "élève"
	fmt.Println(str.Repeat("-", 5))
	fmt.Println(utf8.RuneCountInString(mot), "lettres")
}
```

```sortie
-----
5 lettres
```

## Les noms exportés

En Go, la **majuscule** décide de ce qui est visible hors du package :

- un nom qui commence par une majuscule (`ToUpper`, `Pi`, `Joueur`) est **exporté** :
  les autres packages peuvent l'utiliser ;
- un nom en minuscule (`majuscule`, `compteur`) reste **privé** au package.

Pas de mot-clé `public` ou `private` : la première lettre suffit. Cela vaut pour les
fonctions, les types, les variables, les constantes et même les champs des
[structures](types.md).

```go
package main

import (
	"fmt"
	"math"
)

func main() {
	fmt.Printf("%.4f\n", math.Pi)
	fmt.Println(math.MaxInt8)
}
```

```sortie
3.1416
127
```

## Créer un module

Un **module** est un projet entier : un ou plusieurs packages, avec un nom et une liste
de dépendances. `go mod init` le crée. On lui donne un **chemin de module**, souvent
l'adresse du dépôt où il sera publié (`github.com/vous/projet`). Pour un projet
personnel, n'importe quel nom fait l'affaire.

```sh
mkdir bonjour
cd bonjour
go mod init exemple.com/bonjour
```

```text
go: creating new go.mod: module exemple.com/bonjour
```

La commande crée un fichier `go.mod` :

```text
module exemple.com/bonjour

go 1.24.7
```

- `module` donne le chemin du module : les imports de vos packages commenceront par lui.
- `go` indique la version de Go utilisée.

Écrivez un `main.go` à côté, puis lancez `go run .` : le point veut dire « le package du
dossier courant ».

## Un projet en plusieurs fichiers

Dans un même dossier, tous les fichiers appartiennent au même package : ils partagent
leurs fonctions et leurs types sans rien importer. Pour un nouveau package, créez un
**sous-dossier**.

Voici un projet avec un package `salutation` :

```text
bonjour/
├── go.mod
├── main.go
└── salutation/
    └── salutation.go
```

Le fichier `salutation/salutation.go` :

```go norun
// Package salutation fabrique des messages de bienvenue.
package salutation

// Bonjour renvoie un message de bienvenue pour nom.
func Bonjour(nom string) string {
	return "Bonjour, " + majuscule(nom) + " !"
}

func majuscule(s string) string {
	if s == "" {
		return s
	}
	return string(s[0]-'a'+'A') + s[1:]
}
```

Le fichier `main.go` importe le package avec le chemin du module suivi du dossier :

```go norun
package main

import (
	"fmt"

	"exemple.com/bonjour/salutation"
)

func main() {
	fmt.Println(salutation.Bonjour("alice"))
}
```

```sh
go run .
```

```text
Bonjour, Alice !
```

`main.go` peut appeler `salutation.Bonjour` (majuscule), mais pas
`salutation.majuscule` (minuscule) : c'est un détail interne du package.

> **Astuce :** un commentaire juste au-dessus de `package` ou d'un nom exporté devient
> sa documentation. Commencez-le par le nom décrit : « Bonjour renvoie… ».

## Les packages internal

Un dossier nommé `internal` est protégé : seul le code **du dossier parent et de ses
sous-dossiers** peut importer ce qu'il contient. C'est une pièce « réservée au
personnel ».

```text
bonjour/
├── go.mod
├── main.go
└── salutation/
    ├── salutation.go
    └── internal/
        └── outil/
            └── outil.go
```

Ici, `salutation` peut importer `exemple.com/bonjour/salutation/internal/outil`. Mais
`main.go`, qui est **hors** de `salutation/`, ne le peut pas :

```text
package exemple.com/bonjour
	main.go:6:2: use of internal package exemple.com/bonjour/salutation/internal/outil not allowed
```

Vous pouvez ainsi réorganiser ce code plus tard sans casser les programmes des autres.

## Ajouter un module externe

Des milliers de modules sont publiés en ligne. `go get` en télécharge un et l'ajoute à
`go.mod`. Par exemple, le module `uuid` génère des identifiants uniques :

```sh
go get github.com/google/uuid
```

```text
go: downloading github.com/google/uuid v1.6.0
go: added github.com/google/uuid v1.6.0
```

Le fichier `go.mod` gagne une ligne `require`, et un fichier `go.sum` apparaît : il
contient les empreintes des fichiers téléchargés, pour vérifier que personne ne les a
modifiés.

```go norun
package main

import (
	"fmt"

	"github.com/google/uuid"
)

func main() {
	fmt.Println(uuid.NewString())
}
```

Après avoir ajouté ou retiré des imports, `go mod tidy` range `go.mod` : il ajoute ce
qui manque et retire ce qui ne sert plus.

> **Note :** préférez la bibliothèque standard quand elle suffit. Elle est très riche
> (texte, fichiers, JSON, web, tests…) et n'ajoute aucune dépendance.

## Lire la documentation avec go doc

`go doc` affiche la documentation d'un package ou d'une fonction, directement dans le
[terminal](../debuter/terminal.md).

```sh
go doc strings.ToUpper
```

```text
package strings // import "strings"

func ToUpper(s string) string
    ToUpper returns s with all Unicode letters mapped to their upper case.
```

Cela marche aussi pour vos propres packages, grâce aux commentaires :

```sh
go doc ./salutation
```

```text
package salutation // import "exemple.com/bonjour/salutation"

Package salutation fabrique des messages de bienvenue.

func Bonjour(nom string) string
```

Seuls les noms exportés apparaissent. La même documentation est en ligne sur
`pkg.go.dev`.

## Erreurs fréquentes

### Un import inutilisé

```go norun
package main

import (
	"fmt"
	"os"
)

func main() {
	fmt.Println("Bonjour")
}
```

```text
# command-line-arguments
./main.go:5:2: "os" imported and not used
```

Go refuse un import qui ne sert pas. Supprimez la ligne `"os"`. Beaucoup d'éditeurs le
font tout seuls à l'enregistrement (avec l'outil `goimports`).

### Appeler un nom non exporté

```go norun
fmt.Println(salutation.majuscule("alice"))
```

```text
# exemple.com/bonjour
./main.go:10:25: name majuscule not exported by package salutation
```

`majuscule` commence par une minuscule : il est invisible hors de son package.
Utilisez une fonction exportée, ou renommez-la `Majuscule` si elle doit vraiment être
publique.

### Lancer go run . sans module

```sh
go run .
```

```text
go: go.mod file not found in current directory or any parent directory; see 'go help modules'
```

Le dossier n'a pas de `go.mod`. Lancez `go mod init exemple.com/projet` d'abord, ou
placez-vous dans le dossier du module.

## Testez-vous

**Question 1 :** dans un package, quelle est la différence entre `Calculer` et
`calculer` ?

> **Réponse**
>
> `Calculer` commence par une majuscule : il est exporté, les autres packages peuvent
> l'appeler. `calculer` est privé à son package.

**Question 2 :** votre module s'appelle `exemple.com/jeu` et contient un dossier
`score/`. Quelle ligne d'import écrire dans `main.go` ?

> **Réponse**
>
> `import "exemple.com/jeu/score"`, puis on utilise `score.QuelqueChose`.

**Question 3 :** à quoi sert `go mod tidy` ?

> **Réponse**
>
> Il met `go.mod` et `go.sum` en ordre : il ajoute les modules importés qui manquent et
> retire ceux qui ne servent plus.

## En résumé

- Un package est un dossier ; `import` le rend disponible, et on écrit `pkg.Nom`.
- Une majuscule au début d'un nom l'exporte ; une minuscule le garde privé.
- `go mod init chemin` crée un module et son fichier `go.mod`.
- Un sous-dossier est un nouveau package ; `internal/` limite qui peut l'importer.
- `go get` ajoute un module externe ; `go doc` affiche la documentation.

Suite : [les goroutines](concurrence.md). Comparez avec
[les modules en Rust](../rust/modules.md).
