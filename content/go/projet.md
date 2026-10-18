# Organiser un projet

Cette page montre comment organiser un vrai projet Go : créer un module avec `go mod init`, ranger le code dans `cmd/` et `internal/`, compiler pour d'autres systèmes et gérer les dépendances avec `go get` et `go mod tidy`.

> **Objectifs :**
>
> - créer un module et comprendre le fichier `go.mod` ;
> - ranger un projet en paquets, avec les dossiers `cmd/` et `internal/` ;
> - lancer, compiler et installer un programme avec `go run`, `go build` et `go install` ;
> - compiler pour Windows ou macOS avec `GOOS` et `GOARCH` ;
> - ajouter une dépendance et choisir sa version.

## Le module : la carte d'identité du projet

Jusqu'ici, vos programmes tenaient dans un seul fichier, lancé avec `go run main.go`.
Un vrai projet est un **module** : un dossier qui contient un fichier `go.mod`, et tout
le code en dessous. Le module a un **nom**, qui sert de début à tous ses chemins
d'import (voir [les modules](modules.md)).

Créez un dossier, entrez dedans, puis lancez `go mod init` avec le nom du module :

```sh
mkdir salut
cd salut
go mod init example.com/salut
```

```text
go: creating new go.mod: module example.com/salut
```

Le fichier `go.mod` créé est très court :

```text
module example.com/salut

go 1.24.7
```

- `module` donne le nom du module. Par habitude, c'est l'adresse où le code sera
  publié, comme `github.com/alice/salut`. Pour un projet personnel, `example.com/salut`
  ou même `salut` conviennent.
- `go` indique la version de Go utilisée.

> **À retenir :** une commande `go` lancée dans un dossier cherche le `go.mod` dans ce
> dossier, puis dans ses parents. Lancez donc vos commandes depuis le dossier du projet.

## Un projet bien rangé

Un petit programme peut garder un seul `main.go` à la racine du module. Quand le projet
grandit, on suit une organisation courante :

```text
salut/
├── go.mod
├── cmd/
│   └── salut/
│       └── main.go          le programme (package main)
└── internal/
    └── salutation/
        ├── salutation.go    le code réutilisable
        └── salutation_test.go
```

- **`cmd/`** contient un dossier par programme. Chacun a son `package main` et sa
  fonction `main`. Un projet peut ainsi fournir plusieurs commandes.
- **`internal/`** contient les [paquets](../debuter/glossaire.md#paquet) privés du
  module. Go **interdit** aux autres modules de les importer : vous pouvez les modifier
  sans casser le code de personne.

> **Note :** chaque dossier est **un** paquet. Tous les fichiers `.go` d'un même dossier
> doivent avoir la même ligne `package`.

Voici le paquet `salutation` :

```go norun
// internal/salutation/salutation.go

// Package salutation fabrique des messages de bienvenue.
package salutation

import "fmt"

// Bonjour renvoie un message de bienvenue pour nom.
func Bonjour(nom string) string {
	return fmt.Sprintf("Bonjour, %s !", nom)
}
```

Et le programme, qui l'importe avec le nom du module suivi du chemin du dossier :

```go norun
// cmd/salut/main.go
package main

import (
	"fmt"
	"os"

	"example.com/salut/internal/salutation"
)

var version = "dev"

func main() {
	if len(os.Args) > 1 && os.Args[1] == "-version" {
		fmt.Println("salut", version)
		return
	}
	nom := "le monde"
	if len(os.Args) > 1 {
		nom = os.Args[1]
	}
	fmt.Println(salutation.Bonjour(nom))
}
```

`Bonjour` commence par une majuscule : il est **exporté**, donc utilisable depuis
`main` (voir [bien écrire du Go](style.md)).

## Lancer avec go run

`go run` compile le programme dans un dossier temporaire, puis le lance. On lui donne un
**dossier** de paquet plutôt qu'un fichier :

```sh
go run ./cmd/salut
go run ./cmd/salut Alice
```

```text
Bonjour, le monde !
Bonjour, Alice !
```

Les mots après le chemin sont passés au programme, dans `os.Args` (voir
[la ligne de commande](ligne-de-commande.md)). Pour un projet avec un seul `main.go` à
la racine, écrivez simplement `go run .` : le point désigne le dossier courant.

Le motif `./...` veut dire « ce dossier et tous ses sous-dossiers ». Il marche avec la
plupart des commandes :

```sh
go vet ./...
go test ./...
```

```text
?   	example.com/salut/cmd/salut	[no test files]
ok  	example.com/salut/internal/salutation	0.002s
```

## Compiler avec go build

`go build` crée un **exécutable** : un fichier qui se lance sans Go. Il contient tout
le nécessaire, bibliothèque standard comprise.

```sh
go build ./cmd/salut
ls
./salut Bob
```

```text
cmd  go.mod  internal  salut
Bonjour, Bob !
```

L'exécutable prend le nom du dossier (`salut`). L'option `-o` choisit un autre nom ou
un autre dossier : `go build -o bin/salut ./cmd/salut`.

### Graver une version dans l'exécutable

L'option `-ldflags "-X ..."` remplace la valeur d'une variable de type `string` au
moment de la compilation. Pratique pour afficher la version du programme :

```sh
go build -o salut ./cmd/salut
./salut -version
go build -ldflags "-X main.version=1.2.0" -o salut ./cmd/salut
./salut -version
```

```text
salut dev
salut 1.2.0
```

## Installer avec go install

`go install` compile et range l'exécutable dans le dossier des programmes Go, en général
`~/go/bin` (le dossier `bin` de `go env GOPATH`) :

```sh
go install ./cmd/salut
go env GOPATH
```

```text
/home/alice/go
```

Si `~/go/bin` est dans votre `PATH` (voir [le terminal](../debuter/terminal.md)), la
commande `salut` marche alors depuis n'importe quel dossier. `go install` sert aussi à
installer des outils publiés, comme le débogueur Delve (voir [déboguer](debogage.md)) :

```sh
go install github.com/go-delve/delve/cmd/dlv@latest
```

| Commande | Ce qu'elle fait |
|----------|-----------------|
| `go run ./cmd/salut` | compile dans un dossier temporaire et lance |
| `go build ./cmd/salut` | crée l'exécutable dans le dossier courant |
| `go install ./cmd/salut` | crée l'exécutable dans `~/go/bin` |

## La compilation croisée

Go sait compiler pour un autre [système d'exploitation](../debuter/glossaire.md#systeme-d-exploitation)
ou un autre [processeur](../debuter/glossaire.md#processeur) que le vôtre. C'est la
**compilation croisée**. Deux variables d'environnement la commandent :

- `GOOS` : le système (`linux`, `windows`, `darwin` pour macOS...) ;
- `GOARCH` : le processeur (`amd64` pour la plupart des PC, `arm64` pour les Mac
  récents et le Raspberry Pi...).

```sh
go env GOOS GOARCH
GOOS=windows GOARCH=amd64 go build -o salut.exe ./cmd/salut
GOOS=darwin GOARCH=arm64 go build -o salut-mac ./cmd/salut
file salut.exe salut-mac
```

```text
linux
amd64
salut.exe: PE32+ executable (console) x86-64, for MS Windows, 15 sections
salut-mac: Mach-O 64-bit arm64 executable, flags:<|DYLDLINK|PIE>
```

Depuis Linux, on a fabriqué un programme Windows et un programme pour Mac, sans rien
installer de plus. `go tool dist list` affiche toutes les combinaisons possibles.

> **Note :** sous Windows, dans PowerShell, la syntaxe change :
> `$env:GOOS="linux"; go build ./cmd/salut`.

## Les dépendances

Une **dépendance** est un module écrit par quelqu'un d'autre, que votre projet utilise.
Les modules publics se trouvent sur pkg.go.dev. Prenons `github.com/google/uuid`, qui
fabrique des identifiants uniques.

```go norun
// cmd/id/main.go
package main

import (
	"fmt"

	"github.com/google/uuid"
)

func main() {
	id := uuid.New()
	fmt.Println(len(id.String()))
}
```

Si vous lancez ce programme sans rien faire de plus, Go refuse :

```text
cmd/id/main.go:6:2: no required module provides package github.com/google/uuid; to add it:
	go get github.com/google/uuid
```

### Ajouter une dépendance avec go get

Suivez le conseil du message :

```sh
go get github.com/google/uuid@v1.6.0
```

```text
go: added github.com/google/uuid v1.6.0
```

`go get` télécharge le module et l'inscrit dans `go.mod`, avec sa version :

```text
module example.com/salut

go 1.24.7

require github.com/google/uuid v1.6.0
```

Il crée aussi un fichier `go.sum`, qui contient une **empreinte** de chaque module
téléchargé. Si quelqu'un modifiait le code d'une version déjà publiée, l'empreinte ne
correspondrait plus et Go refuserait de compiler. Gardez `go.mod` et `go.sum` dans
[Git](../debuter/git.md).

### Choisir une version

Les versions suivent le modèle `vMAJEUR.MINEUR.CORRECTIF`, par exemple `v1.6.0` :

- **correctif** (`v1.6.1`) : corrige des bugs ;
- **mineur** (`v1.7.0`) : ajoute des fonctions, sans rien casser ;
- **majeur** (`v2.0.0`) : peut casser votre code. En Go, une version majeure 2 ou plus
  change le chemin d'import : `example.com/lib/v2`.

| Commande | Effet |
|----------|-------|
| `go get module@v1.6.0` | une version précise |
| `go get module@latest` | la dernière version |
| `go get -u ./...` | met à jour toutes les dépendances (sans changer de majeure) |
| `go list -m all` | liste les modules utilisés |
| `go list -m -versions module` | liste les versions publiées |

```sh
go list -m -versions github.com/google/uuid
```

```text
github.com/google/uuid v1.0.0 v1.1.0 v1.1.1 v1.1.2 v1.1.3 v1.1.4 v1.1.5 v1.2.0 v1.3.0 v1.3.1 v1.4.0 v1.5.0 v1.6.0
```

### Faire le ménage avec go mod tidy

`go mod tidy` (« ranger ») compare `go.mod` avec les `import` de votre code : il ajoute
ce qui manque et retire ce qui ne sert plus. Si vous supprimez le programme `cmd/id`,
puis lancez :

```sh
go mod tidy
cat go.mod
```

```text
module example.com/salut

go 1.24.7
```

La ligne `require` a disparu, car plus aucun fichier n'importe `uuid`.

> **Astuce :** lancez `go mod tidy` avant chaque commit. Votre `go.mod` reste ainsi
> exact.

## Un Makefile (facultatif)

Les commandes deviennent longues. Beaucoup de projets les rangent dans un fichier
`Makefile`, lu par l'outil `make` (souvent présent sous Linux et macOS). Ce n'est pas
obligatoire : un simple fichier texte avec les commandes rend le même service.

```text
VERSION := 1.2.0

.PHONY: build test clean

build:
	go build -ldflags "-X main.version=$(VERSION)" -o bin/salut ./cmd/salut

test:
	go vet ./...
	go test ./...

clean:
	rm -rf bin
```

Les lignes de commandes commencent par une **tabulation**, pas par des espaces.

```sh
make build
bin/salut -version
```

```text
go build -ldflags "-X main.version=1.2.0" -o bin/salut ./cmd/salut
salut 1.2.0
```

## Erreurs fréquentes

### Lancer une commande hors du module

```sh
cd ~
go build ./cmd/salut
```

```text
go: go.mod file not found in current directory or any parent directory; see 'go help modules'
```

Go ne trouve aucun `go.mod`. Placez-vous dans le dossier du projet (`cd salut`), ou
créez le module avec `go mod init`.

### Deux paquets dans le même dossier

```text
deux/
├── main.go      package main
└── outils.go    package outils
```

```text
found packages main (main.go) and outils (outils.go) in /home/alice/deux
```

Un dossier, un paquet. Mettez `outils.go` dans son propre dossier `outils/`, puis
importez-le avec `"example.com/deux/outils"`.

### Importer un paquet internal depuis un autre module

```go norun
import "example.com/salut/internal/salutation"
```

```text
package example.com/autre
	main.go:6:2: use of internal package example.com/salut/internal/salutation not allowed
```

C'est le rôle de `internal/` : seul le module `example.com/salut` peut l'importer. Si
le paquet doit servir ailleurs, sortez-le de `internal/`.

### Lancer un paquet qui n'est pas un programme

```sh
go run .
```

```text
package example.com/lib is not a main package
```

Seul un `package main` avec une fonction `main` se lance. Un paquet comme `lib` est une
[bibliothèque](../debuter/glossaire.md#bibliotheque) : on l'importe depuis un
programme, et on le vérifie avec `go test`.

## Testez-vous

**Question 1 :** à quoi sert le fichier `go.mod` ?

> **Réponse**
>
> Il marque la racine du module, donne son nom (le début des chemins d'import), la
> version de Go et la liste des dépendances avec leur version.

**Question 2 :** quelle est la différence entre `cmd/` et `internal/` ?

> **Réponse**
>
> `cmd/` contient les programmes (un dossier par `package main`). `internal/` contient
> les paquets privés du module, que les autres modules n'ont pas le droit d'importer.

**Question 3 :** comment compiler `./cmd/salut` pour Windows depuis Linux ?

> **Réponse**
>
> `GOOS=windows GOARCH=amd64 go build -o salut.exe ./cmd/salut`.

## En résumé

- `go mod init nom` crée le module ; `go.mod` donne son nom, sa version de Go et ses
  dépendances.
- Les programmes vont dans `cmd/<nom>/`, le code privé dans `internal/`.
- `go run` lance, `go build` crée un exécutable, `go install` le range dans `~/go/bin`.
- `GOOS` et `GOARCH` permettent de compiler pour un autre système.
- `go get module@version` ajoute une dépendance ; `go mod tidy` fait le ménage.

Suite : [projets pas à pas](projets.md).
