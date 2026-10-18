# Écrire un outil en ligne de commande

Cette page montre comment écrire un vrai programme pour le terminal : lire ses arguments et ses options, signaler les erreurs, renvoyer un code de sortie, puis le compiler et l'installer.

> **Objectifs :**
>
> - vous saurez lire les arguments du programme avec `os.Args` ;
> - vous saurez déclarer des options comme `-top 3` avec le paquet `flag` ;
> - vous saurez écrire les erreurs sur `os.Stderr` et quitter avec un code de sortie ;
> - vous saurez construire un outil complet qui compte les mots d'un fichier ;
> - vous saurez le compiler avec `go build` et l'installer avec `go install`.

## Pourquoi Go pour la ligne de commande ?

Beaucoup d'outils célèbres du [terminal](../debuter/glossaire.md#terminal) sont écrits
en Go (Docker, Hugo, `gh`…). La raison : `go build` produit **un seul fichier
exécutable**, sans rien d'autre à installer. Vous pouvez le copier sur une autre
machine du même système, et il fonctionne.

Un outil en ligne de commande reçoit des informations de trois façons :

- des **arguments** : `compte-mots texte.txt` ;
- des **options** (en anglais *flags*) : `compte-mots -top 3 texte.txt` ;
- l'entrée standard, vue dans [les fichiers](fichiers.md#lire-le-clavier-avec-bufio-scanner).

## Les arguments avec os.Args

`os.Args` est une tranche de `string`. La case 0 contient le chemin du programme ; les
vrais arguments commencent à la case 1. On prend donc presque toujours `os.Args[1:]`.

```go
package main

import (
	"fmt"
	"os"
)

func main() {
	arguments := os.Args[1:]
	fmt.Println("Nombre d'arguments :", len(arguments))
	for i, a := range arguments {
		fmt.Printf("%d : %s\n", i+1, a)
	}
}
```

```sortie
Nombre d'arguments : 0
```

Lancé sans rien, le programme n'a aucun argument. Dans le terminal, ajoutez-en après
le nom du fichier :

```sh
go run main.go Alice "Jean Dupont" 42
```

```text
Nombre d'arguments : 3
1 : Alice
2 : Jean Dupont
3 : 42
```

Les guillemets regroupent `Jean Dupont` en **un seul** argument. Et `42` arrive comme
du texte : convertissez-le avec `strconv.Atoi` si vous voulez un nombre.

## Les options avec le paquet flag

Analyser `-top 3` à la main serait fastidieux. Le paquet `flag` s'en charge. On
**déclare** chaque option avec son nom, sa valeur par défaut et une phrase d'aide.
On obtient un **pointeur** (voir [les pointeurs](pointeurs.md)), rempli au moment de
l'analyse.

Dans un vrai programme, on écrit `flag.Int(...)` puis `flag.Parse()`, qui lit
`os.Args`. Ici, pour que l'exemple tourne tout seul, on crée un **jeu d'options**
avec `flag.NewFlagSet` et on lui donne une liste d'arguments fixe.

```go
package main

import (
	"flag"
	"fmt"
)

func main() {
	fs := flag.NewFlagSet("salut", flag.ContinueOnError)
	nom := fs.String("nom", "inconnu", "le nom à saluer")
	fois := fs.Int("fois", 1, "nombre de répétitions")
	crier := fs.Bool("crier", false, "écrire en majuscules")

	args := []string{"-nom", "Léa", "-fois", "2", "-crier", "fichier1.txt", "fichier2.txt"}
	if err := fs.Parse(args); err != nil {
		fmt.Println("Options invalides :", err)
		return
	}

	for i := 0; i < *fois; i++ {
		message := "Bonjour, " + *nom
		if *crier {
			message += " !!!"
		}
		fmt.Println(message)
	}
	fmt.Println("Arguments restants :", fs.Args())
	fmt.Println("Nombre :", fs.NArg())
}
```

```sortie
Bonjour, Léa !!!
Bonjour, Léa !!!
Arguments restants : [fichier1.txt fichier2.txt]
Nombre : 2
```

- `*nom` lit la valeur pointée (l'étoile « ouvre » le pointeur) ;
- une option booléenne n'a pas besoin de valeur : `-crier` suffit ;
- `fs.Args()` renvoie ce qui reste après les options, `fs.NArg()` leur nombre.

> **À retenir :** les options viennent **avant** les arguments. Dans
> `outil fichier.txt -top 3`, le paquet `flag` s'arrête au premier argument et ne voit
> pas `-top`.

### L'aide gratuite

Le paquet `flag` fabrique tout seul le message d'aide, affiché avec `-h` :

```text
Usage of salut:
  -crier
    	écrire en majuscules
  -fois int
    	nombre de répétitions (default 1)
  -nom string
    	le nom à saluer (default "inconnu")
```

## Sortie normale et sortie d'erreur

Un programme a deux sorties distinctes :

- `os.Stdout`, la sortie standard, pour les **résultats** (`fmt.Println` y écrit) ;
- `os.Stderr`, la sortie d'erreur, pour les **messages d'erreur**.

On les sépare pour qu'une redirection comme `outil > resultats.txt` n'enregistre que
les résultats : les erreurs restent visibles à l'écran. `fmt.Fprintln(os.Stderr, …)`
écrit sur la sortie d'erreur.

```go norun
fmt.Fprintln(os.Stderr, "erreur : fichier introuvable")
```

## Les codes de sortie avec os.Exit

Quand un programme se termine, il renvoie un **code de sortie** au terminal. C'est sa
façon de dire comment ça s'est passé :

| Code | Signification habituelle |
|------|---------------------------|
| `0` | tout s'est bien passé |
| `1` | une erreur pendant le travail (fichier absent…) |
| `2` | mauvaise utilisation (option inconnue, argument manquant) |

`os.Exit(code)` arrête le programme **immédiatement** avec ce code. Attention : les
`defer` en attente ne sont **pas** exécutés.

```go
package main

import (
	"fmt"
	"os"
)

func main() {
	defer fmt.Println("Ceci ne s'affichera jamais.")
	fmt.Println("Travail terminé.")
	os.Exit(0)
}
```

```sortie
Travail terminé.
```

Dans le terminal, `echo $?` affiche le code de sortie du dernier programme lancé.
Les scripts s'en servent pour savoir si une étape a réussi.

> **Astuce :** pour ne pas perdre les `defer`, mettez tout le travail dans une fonction
> qui **renvoie** le code, et appelez `os.Exit` une seule fois, dans `main` :
> `os.Exit(executer(os.Args[1:]))`.

## Un outil complet : compter les mots

Assemblons tout. L'outil `compte-mots` lit un fichier et affiche les mots les plus
fréquents. Il accepte deux options : `-top` (combien de mots afficher) et `-min` (la
longueur minimale d'un mot).

Toute la logique est dans `executer`, qui reçoit les arguments et renvoie le code de
sortie. Pour que l'exemple tourne ici, `main` crée d'abord son fichier d'entrée et
appelle `executer` avec des arguments fixes.

```go
package main

import (
	"bufio"
	"errors"
	"flag"
	"fmt"
	"os"
	"sort"
	"strings"
	"unicode"
)

func executer(args []string) int {
	fs := flag.NewFlagSet("compte-mots", flag.ContinueOnError)
	top := fs.Int("top", 5, "nombre de mots à afficher")
	minimum := fs.Int("min", 1, "longueur minimale d'un mot")
	if err := fs.Parse(args); err != nil {
		if errors.Is(err, flag.ErrHelp) {
			return 0
		}
		return 2
	}
	if fs.NArg() != 1 {
		fmt.Fprintln(os.Stderr, "usage : compte-mots [-top N] [-min N] FICHIER")
		return 2
	}

	f, err := os.Open(fs.Arg(0))
	if err != nil {
		fmt.Fprintln(os.Stderr, "erreur :", err)
		return 1
	}
	defer f.Close()

	compte := map[string]int{}
	total := 0
	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		mots := strings.FieldsFunc(scanner.Text(), func(r rune) bool {
			return !unicode.IsLetter(r)
		})
		for _, mot := range mots {
			mot = strings.ToLower(mot)
			if len([]rune(mot)) < *minimum {
				continue
			}
			compte[mot]++
			total++
		}
	}

	mots := make([]string, 0, len(compte))
	for mot := range compte {
		mots = append(mots, mot)
	}
	sort.Slice(mots, func(i, j int) bool {
		if compte[mots[i]] != compte[mots[j]] {
			return compte[mots[i]] > compte[mots[j]]
		}
		return mots[i] < mots[j]
	})

	fmt.Println("Mots :", total, "- différents :", len(compte))
	for i, mot := range mots {
		if i == *top {
			break
		}
		fmt.Printf("%3d  %s\n", compte[mot], mot)
	}
	return 0
}

func main() {
	texte := "Le chat dort. Le chien dort aussi.\n" +
		"Le chat rêve du chien, et le chien rêve du chat.\n" +
		"Quel chat !\n"
	os.WriteFile("texte.txt", []byte(texte), 0644)

	code := executer([]string{"-top", "3", "-min", "4", "texte.txt"})
	fmt.Println("Code de sortie :", code)

	code = executer([]string{"texte.txt"})
	fmt.Println("Code de sortie :", code)
}
```

```sortie
Mots : 13 - différents : 6
  4  chat
  3  chien
  2  dort
Code de sortie : 0
Mots : 20 - différents : 9
  4  chat
  4  le
  3  chien
  2  dort
  2  du
Code de sortie : 0
```

Quelques détails :

- `strings.FieldsFunc` découpe la ligne partout où la fonction renvoie `true`, ici sur
  tout ce qui n'est pas une lettre (espaces, points, virgules) ;
- `len([]rune(mot))` compte les lettres, même accentuées (voir [le texte](texte.md)) ;
- on trie les mots par nombre d'apparitions décroissant, puis par ordre alphabétique,
  car l'ordre d'une [map](maps.md) change d'une exécution à l'autre.

Dans le vrai programme, `main` devient simplement :

```go norun
func main() {
	os.Exit(executer(os.Args[1:]))
}
```

## Compiler avec go build

Placez `main.go` dans un dossier `compte-mots`, créez le module (voir
[les modules](modules.md)), puis compilez :

```sh
cd compte-mots
go mod init compte-mots
go build
```

`go build` crée un exécutable nommé comme le module : `compte-mots` (ou
`compte-mots.exe` sous Windows). Lancez-le avec `./` :

```sh
./compte-mots -top 3 -min 4 texte.txt
./compte-mots absent.txt
echo $?
```

```text
Mots : 13 - différents : 6
  4  chat
  3  chien
  2  dort
erreur : open absent.txt: no such file or directory
1
```

Le fichier absent donne un message sur la sortie d'erreur et le code `1`.

## Installer avec go install

`go install` compile le programme et range l'exécutable dans le dossier `bin` de
Go (par défaut `~/go/bin`). Si ce dossier est dans votre `PATH`, l'outil devient
utilisable de partout, comme `ls` ou `git` :

```sh
go install
compte-mots -top 3 ~/Documents/roman.txt
```

> **Note :** `go env GOPATH` affiche le dossier de Go ; les exécutables vont dans son
> sous-dossier `bin`. Pour installer l'outil de quelqu'un d'autre, on écrit
> `go install github.com/nom/outil@latest`.

## Erreurs fréquentes

### Lire os.Args[1] sans vérifier

```go norun
package main

import (
	"fmt"
	"os"
)

func main() {
	nom := os.Args[1]
	fmt.Println("Bonjour,", nom)
}
```

```text
panic: runtime error: index out of range [1] with length 1

goroutine 1 [running]:
main.main()
	…/salut/main.go:9 +0x98
exit status 2
```

Lancé sans argument, `os.Args` ne contient que le nom du programme (longueur 1). Testez
d'abord `if len(os.Args) < 2`, affichez l'usage sur `os.Stderr` et quittez avec
`os.Exit(2)`.

### Se tromper de nom d'option

```go norun
fs := flag.NewFlagSet("salut", flag.ExitOnError)
nom := fs.String("nom", "inconnu", "le nom à saluer")
fs.Parse([]string{"-name", "Alice"})
fmt.Println("Bonjour,", *nom)
```

```text
flag provided but not defined: -name
Usage of salut:
  -nom string
    	le nom à saluer (default "inconnu")
exit status 2
```

L'option déclarée s'appelle `-nom`, pas `-name`. Le paquet `flag` affiche l'aide et
quitte avec le code 2. Lisez la liste des options proposées.

### Mettre les options après les arguments

```sh
./compte-mots texte.txt -top 3
```

```text
usage : compte-mots [-top N] [-min N] FICHIER
```

`flag` s'arrête au premier argument qui n'est pas une option : `-top` et `3` deviennent
des arguments, il y en a trois au lieu d'un. Écrivez les options d'abord :
`./compte-mots -top 3 texte.txt`.

## Testez-vous

**Question 1 :** que contient `os.Args[0]` ?

> **Réponse**
>
> Le chemin du programme lui-même. Les arguments donnés par l'utilisateur commencent à
> `os.Args[1]`.

**Question 2 :** pourquoi écrire les messages d'erreur sur `os.Stderr` ?

> **Réponse**
>
> Pour les séparer des résultats. Si l'utilisateur redirige les résultats dans un
> fichier (`> sortie.txt`), les erreurs restent visibles et ne polluent pas le fichier.

**Question 3 :** déclarez une option `-langue` (texte, défaut `fr`), analysez
`[]string{"-langue", "en"}` et affichez sa valeur.

> **Réponse**
>
> ```go
> package main
>
> import (
> 	"flag"
> 	"fmt"
> )
>
> func main() {
> 	fs := flag.NewFlagSet("outil", flag.ContinueOnError)
> 	langue := fs.String("langue", "fr", "langue des messages")
> 	if err := fs.Parse([]string{"-langue", "en"}); err != nil {
> 		fmt.Println(err)
> 		return
> 	}
> 	fmt.Println("Langue :", *langue)
> }
> ```
>
> ```sortie
> Langue : en
> ```

## En résumé

- `os.Args[1:]` contient les arguments donnés au programme.
- Le paquet `flag` déclare des options (`fs.Int`, `fs.String`, `fs.Bool`) et génère
  l'aide ; les options viennent avant les arguments.
- Les erreurs vont sur `os.Stderr` avec `fmt.Fprintln(os.Stderr, …)`.
- `os.Exit(code)` quitte avec un code : 0 succès, 1 erreur, 2 mauvaise utilisation.
- `go build` produit un exécutable, `go install` l'installe dans `~/go/bin`.

Suite : [Go pour le web](web.md). Comparez avec
[la ligne de commande en Ruby](../ruby/ligne-de-commande.md).
