# Go : introduction et installation

Cette page présente Go, explique comment l'installer et vous fait écrire, lancer et compiler votre premier programme.

> **Objectifs :**
>
> - vous saurez dire ce qu'est Go et à quoi il sert ;
> - vous saurez installer Go et vérifier qu'il marche avec `go version` ;
> - vous saurez écrire un programme avec `package main`, `import` et `func main` ;
> - vous saurez lancer un programme avec `go run` et le compiler avec `go build` ;
> - vous saurez mettre votre code en forme avec `gofmt`.

## Qu'est-ce que Go ?

Go est un langage de programmation créé chez Google en 2009, par Robert Griesemer, Rob
Pike et Ken Thompson. On l'appelle aussi « Golang », pour le retrouver plus facilement
dans un moteur de recherche.

Un **langage de programmation** est une langue précise, que l'ordinateur sait suivre.
Un **programme** est un texte écrit dans cette langue : une suite d'ordres, comme une
recette de cuisine.

```go
package main

import "fmt"

func main() {
	fmt.Println("Bonjour depuis Go !")
}
```

```sortie
Bonjour depuis Go !
```

Ces quelques lignes sont un programme complet. Nous les expliquerons une par une un peu
plus bas.

## Pourquoi apprendre Go ?

Go a été pensé pour être **simple**. Le langage tient en peu de mots-clés : vous en
faites vite le tour.

- **Il est rapide.** Go est un langage [compilé](../debuter/glossaire.md#compilation) :
  avant de tourner, votre code est traduit en langage machine. Le programme obtenu
  s'exécute très vite.
- **Il est simple à lire.** Il y a souvent une seule bonne façon d'écrire une chose.
  Le code d'une autre personne ressemble au vôtre.
- **Il fabrique un seul fichier.** Le résultat de la compilation est un
  [binaire](../debuter/glossaire.md#binaire) : un fichier que vous copiez sur une autre
  machine, et qui marche sans rien installer.
- **Il sait faire plusieurs choses à la fois.** Go a été conçu pour la
  [concurrence](../debuter/glossaire.md#concurrence) : lancer des milliers de petites
  tâches en même temps est facile.

Go est très utilisé pour les **serveurs** (les programmes qui répondent aux sites web
et aux applications) et pour les **outils** en ligne de commande. Docker, Kubernetes ou
Hugo sont écrits en Go.

> **Note :** ce site présente aussi d'autres langages : comparez avec
> [l'introduction à Ruby](../ruby/introduction.md) ou
> [l'introduction à Rust](../rust/introduction.md).

### Compilé ou interprété ?

Avec un langage **interprété** (comme Ruby ou Python), un programme lit votre fichier et
exécute les ordres un par un, à chaque lancement.

Avec un langage **compilé** comme Go, un
[compilateur](../debuter/glossaire.md#compilateur) lit d'abord tout votre fichier. Il
vérifie qu'il n'y a pas d'erreur, puis il le traduit en langage machine. C'est comme
traduire un livre entier avant de le donner à lire, au lieu de le traduire à voix haute
page par page.

> **À retenir :** le compilateur de Go est très strict. Il refuse une variable ou un
> import inutilisé. C'est parfois agaçant au début, mais votre code reste propre.

## Installer Go

Ce site utilise **Go 1.24**. Une version plus récente marche aussi.

Toutes les versions se téléchargent sur le site officiel : **go.dev/dl**.

### Sur Windows

Téléchargez le fichier `.msi` pour Windows sur go.dev/dl, puis lancez-le. Gardez les
options proposées par défaut. L'installeur ajoute Go au terminal tout seul.

### Sur macOS

Téléchargez le fichier `.pkg` sur go.dev/dl (choisissez « Apple » ou « ARM64 » pour un
Mac récent), puis lancez-le. Vous pouvez aussi utiliser Homebrew :

```sh
brew install go
```

### Sur Linux

Le gestionnaire de paquets de votre système propose souvent Go, mais sa version peut
être ancienne. Le plus sûr est l'archive officielle de go.dev/dl :

```sh
sudo rm -rf /usr/local/go
sudo tar -C /usr/local -xzf go1.24.7.linux-amd64.tar.gz
echo 'export PATH=$PATH:/usr/local/go/bin' >> ~/.profile
```

- La première ligne efface une éventuelle ancienne version.
- La deuxième décompresse Go dans `/usr/local/go`.
- La troisième ajoute le dossier `bin` de Go au `PATH` : la liste des dossiers où le
  terminal cherche les commandes.

Fermez et rouvrez votre session (ou tapez `source ~/.profile`) pour que le changement
compte.

> **Astuce :** sur Ubuntu, `sudo snap install go --classic` installe aussi une version
> récente.

## Vérifier l'installation

Ouvrez un terminal et tapez la commande suivante. Si le terminal est nouveau pour vous,
lisez d'abord [le terminal](../debuter/terminal.md).

```sh
go version
```

Vous devez voir un numéro de version, par exemple :

```text
go version go1.24.7 linux/amd64
```

La fin indique votre système (`linux`, `darwin` pour macOS, `windows`) et le type de
processeur (`amd64`, `arm64`).

Si le terminal répond « command not found » (commande introuvable), Go n'est pas
installé, ou le terminal a été ouvert avant l'installation : fermez-le et rouvrez-le.

## Votre premier programme

Ouvrez votre [éditeur de code](../debuter/editeur.md). Créez un dossier `bonjour`, et
dedans un fichier `bonjour.go` avec ce contenu :

```go
package main

import "fmt"

func main() {
	fmt.Println("Bonjour, le monde !")
}
```

```sortie
Bonjour, le monde !
```

Lisons-le ligne par ligne.

- `package main` : tout fichier Go appartient à un **paquet** (*package*), un groupe de
  fichiers qui vont ensemble. Le paquet `main` est spécial : c'est celui d'un programme
  qu'on peut lancer.
- `import "fmt"` : on importe le paquet `fmt` (pour *format*), fourni avec Go. Il sait
  afficher du texte.
- `func main() { ... }` : on définit la [fonction](../debuter/glossaire.md#fonction)
  `main`. C'est la porte d'entrée : Go commence toujours par elle.
- `fmt.Println("...")` : on appelle la fonction `Println` du paquet `fmt`. Elle affiche
  le texte, puis passe à la ligne (*ln* pour *line*).

Le texte est entre guillemets doubles `"..."`. C'est une
[chaîne de caractères](../debuter/glossaire.md#chaine-de-caracteres).

### Plusieurs lignes

Les instructions dans `main` s'exécutent dans l'ordre, de haut en bas.

```go
package main

import "fmt"

func main() {
	fmt.Println("Ligne 1")
	fmt.Println("Ligne 2")
	fmt.Println(2 + 3)
	fmt.Println("Total :", 2+3, "euros")
}
```

```sortie
Ligne 1
Ligne 2
5
Total : 5 euros
```

- `fmt.Println(2 + 3)` affiche `5` : sans guillemets, Go calcule.
- Avec plusieurs valeurs séparées par des virgules, `Println` les affiche avec un espace
  entre chacune.

> **Note :** il n'y a pas de point-virgule à la fin des lignes. Go les ajoute tout seul
> en lisant le fichier.

## Lancer un programme avec go run

Dans le terminal, placez-vous dans le dossier du fichier (avec la commande `cd`), puis
tapez :

```sh
go run bonjour.go
```

```text
Bonjour, le monde !
```

`go run` fait deux choses d'un coup : il compile le fichier, puis il lance le résultat.
Rien n'est gardé sur le disque. C'est la commande que vous utiliserez le plus pour
essayer votre code.

## Compiler avec go build

`go build` compile le fichier et **garde** le programme obtenu.

```sh
go build bonjour.go
./bonjour
```

```text
Bonjour, le monde !
```

- Sur Linux et macOS, `go build` crée un fichier `bonjour`. Vous le lancez avec
  `./bonjour`.
- Sur Windows, il crée `bonjour.exe`. Vous le lancez avec `.\bonjour.exe`.

Ce fichier est autonome : vous pouvez le copier sur une autre machine du même système,
même sans Go installé dessus, et il marchera.

> **À retenir :** `go run` pour essayer, `go build` pour fabriquer le programme final.

## Mettre en forme avec gofmt

En Go, tout le monde présente son code de la même façon. L'outil `gofmt` (et la
commande `go fmt`) range votre fichier pour vous : il corrige les espaces, aligne les
lignes et met des tabulations pour l'indentation.

```sh
gofmt -w bonjour.go
```

L'option `-w` (*write*) réécrit le fichier. Sans elle, `gofmt` affiche seulement le
résultat.

> **Astuce :** la plupart des éditeurs (avec l'extension Go) lancent `gofmt` à chaque
> sauvegarde. Activez-le : vous ne penserez plus jamais à la présentation.

## Lire un message d'erreur

Quand le compilateur ne comprend pas votre code, il s'arrête et affiche une
[erreur](../debuter/glossaire.md#erreur). Ce n'est pas grave : le message vous aide.

```text
# command-line-arguments
./main.go:6:6: undefined: fmt.println
```

Lisez-le de gauche à droite :

- `# command-line-arguments` : le nom que Go donne au paquet lancé directement ;
- `./main.go:6:6` : le fichier, la ligne (6) et la colonne (6) du problème ;
- `undefined: fmt.println` : la description (« `fmt.println` n'est pas défini »).

Les messages sont en anglais. Quelques mots reviennent souvent : *undefined* (non
défini), *unexpected* (inattendu), *expected* (j'attendais), *missing* (il manque),
*not used* (pas utilisé).

## Erreurs fréquentes

### Oublier la majuscule de Println

```go norun
package main

import "fmt"

func main() {
	fmt.println("Bonjour")
}
```

```text
# command-line-arguments
./main.go:6:6: undefined: fmt.println
```

Go fait la différence entre majuscules et minuscules. La fonction s'appelle `Println`,
avec un `P` majuscule. Écrivez `fmt.Println("Bonjour")`.

### Importer un paquet sans s'en servir

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

Le paquet `os` est importé, mais jamais utilisé. Go refuse de compiler. Supprimez la
ligne `"os"`.

### Mettre l'accolade sur la ligne suivante

```go norun
package main

import "fmt"

func main()
{
	fmt.Println("Bonjour")
}
```

```text
# command-line-arguments
./main.go:6:1: syntax error: unexpected semicolon or newline before {
```

Go ajoute un point-virgule invisible à la fin de `func main()`. L'accolade `{` doit donc
être sur la **même** ligne : `func main() {`.

### Se tromper de nom de paquet

```go norun
package bonjour

import "fmt"

func main() {
	fmt.Println("Bonjour")
}
```

```text
package command-line-arguments is not a main package
```

Un programme qu'on lance doit être dans le paquet `main`. Écrivez `package main` sur
la première ligne.

## Testez-vous

**Question 1 :** quelle est la différence entre `go run` et `go build` ?

> **Réponse**
>
> `go run` compile puis lance le programme, sans rien garder. `go build` compile et
> crée un fichier exécutable, que vous lancez ensuite vous-même.

**Question 2 :** écrivez un programme qui affiche `Je débute en Go` puis, sur la ligne
suivante, le résultat de `7 * 6`.

> **Réponse**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	fmt.Println("Je débute en Go")
> 	fmt.Println(7 * 6)
> }
> ```
>
> ```sortie
> Je débute en Go
> 42
> ```

**Question 3 :** par quelle fonction un programme Go commence-t-il toujours ?

> **Réponse**
>
> Par la fonction `main` du paquet `main`.

## En résumé

- Go est un langage compilé, simple et rapide, créé chez Google ; il sert surtout aux
  serveurs et aux outils.
- Installez-le depuis go.dev/dl, puis vérifiez avec `go version`.
- Un programme commence par `package main`, importe ses paquets avec `import`, et
  démarre dans `func main()`.
- `go run fichier.go` lance le code ; `go build` fabrique un exécutable.
- `gofmt` met le code en forme ; le compilateur refuse les imports inutilisés.

Suite : [premiers pas](premiers-pas.md).
