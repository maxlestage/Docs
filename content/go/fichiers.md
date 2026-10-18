# Lire et écrire : clavier et fichiers

Cette page montre comment lire ce que tape l'utilisateur, puis comment écrire, relire et compléter des fichiers avec les paquets `os`, `bufio` et `path/filepath`.

> **Objectifs :**
>
> - vous saurez lire des lignes tapées au clavier avec `bufio.Scanner` ;
> - vous saurez écrire et relire un fichier entier avec `os.WriteFile` et `os.ReadFile` ;
> - vous saurez ouvrir un fichier, le fermer avec `defer` et le lire ligne par ligne ;
> - vous saurez ajouter du texte à la fin d'un fichier avec `os.OpenFile` ;
> - vous saurez vérifier qu'un fichier existe et construire un chemin avec `filepath.Join`.

## Lire le clavier avec bufio.Scanner

L'entrée du clavier s'appelle en Go `os.Stdin` (*standard input*, « entrée standard »).
Pour la lire ligne par ligne, on pose dessus un **scanner** du paquet `bufio`. Le
scanner est comme un lecteur qui avance d'une ligne à chaque appel de `Scan()`.

```go norun
package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)

	fmt.Print("Votre prénom : ")
	scanner.Scan()
	prenom := scanner.Text()

	fmt.Println("Bonjour,", prenom, "!")
}
```

Dans le [terminal](../debuter/glossaire.md#terminal), cela donne :

```text
Votre prénom : Alice
Bonjour, Alice !
```

- `scanner.Scan()` attend une ligne et renvoie `true` s'il en a lu une ;
- `scanner.Text()` renvoie cette ligne, **sans** le retour à la ligne final.

> **Note :** ce bloc est marqué `norun` : il attend une saisie au clavier, ce qui
> n'est pas possible pendant la vérification automatique du site.

### Lire un nombre

`Text()` renvoie toujours du texte. Pour obtenir un nombre, convertissez-le avec
`strconv.Atoi` et vérifiez l'erreur, comme vu dans [les erreurs](erreurs.md).

```go norun
scanner.Scan()
age, err := strconv.Atoi(scanner.Text())
if err != nil {
	fmt.Println("Ce n'est pas un nombre.")
	return
}
fmt.Println("L'an prochain, vous aurez", age+1, "ans.")
```

### Lire jusqu'à la fin

Dans une boucle `for scanner.Scan()`, le programme lit toutes les lignes, jusqu'à ce que
l'utilisateur tape Ctrl+D (fin de l'entrée).

```go norun
scanner := bufio.NewScanner(os.Stdin)
for scanner.Scan() {
	fmt.Println("Vous avez tapé :", scanner.Text())
}
```

## Écrire un fichier entier

Un **fichier** est une suite d'octets rangée sur le disque, avec un nom et un
[chemin](../debuter/glossaire.md#chemin). `os.WriteFile` écrit un fichier en une seule
fois. Il prend trois arguments :

1. le nom du fichier ;
2. le contenu, sous forme de `[]byte` (une tranche d'octets) ;
3. les **permissions** du fichier, presque toujours `0644` (lisible par tous,
   modifiable par vous).

```go
package main

import (
	"fmt"
	"os"
)

func main() {
	texte := "Acheter du pain\nAppeler Léa\n"
	err := os.WriteFile("notes.txt", []byte(texte), 0644)
	if err != nil {
		fmt.Println("Impossible d'écrire :", err)
		return
	}
	fmt.Println("Fichier écrit.")
}
```

```sortie
Fichier écrit.
```

Si le fichier existe déjà, il est **remplacé**. `[]byte(texte)` convertit le texte en
octets.

## Lire un fichier entier

`os.ReadFile` fait l'inverse : il renvoie le contenu (des octets) et une erreur.
`string(...)` retransforme les octets en texte.

```go
package main

import (
	"fmt"
	"os"
)

func main() {
	os.WriteFile("notes.txt", []byte("Acheter du pain\nAppeler Léa\n"), 0644)

	contenu, err := os.ReadFile("notes.txt")
	if err != nil {
		fmt.Println("Impossible de lire :", err)
		return
	}
	fmt.Print(string(contenu))
	fmt.Println("Taille :", len(contenu), "octets")
}
```

```sortie
Acheter du pain
Appeler Léa
Taille : 29 octets
```

> **À retenir :** en Go, presque toutes les opérations sur les fichiers renvoient une
> erreur. Un disque plein, un fichier absent, des droits manquants : tout peut arriver.
> Vérifiez toujours `err`.

## Ouvrir, puis fermer avec defer

`os.ReadFile` charge tout le fichier en mémoire. Pour un gros fichier, ou pour écrire
petit à petit, on **ouvre** le fichier : on obtient un `*os.File`, une poignée sur le
fichier. Une poignée ouverte doit être **fermée** avec `Close()`, sinon le système la
garde réservée.

`defer` (« différer ») programme un appel pour la fin de la fonction. Écrivez
`defer f.Close()` juste après l'ouverture réussie : vous ne pourrez plus l'oublier.
C'est comme refermer la porte du frigo : mieux vaut y penser dès qu'on l'ouvre.

`os.Create` crée un fichier vide (ou vide un fichier existant) et l'ouvre en écriture.

```go
package main

import (
	"fmt"
	"os"
)

func main() {
	f, err := os.Create("courses.txt")
	if err != nil {
		fmt.Println(err)
		return
	}
	defer f.Close()

	fmt.Fprintln(f, "pommes")
	fmt.Fprintln(f, "farine")
	fmt.Fprintf(f, "œufs x%d\n", 6)
	fmt.Println("Liste enregistrée.")
}
```

```sortie
Liste enregistrée.
```

`fmt.Fprintln` est le cousin de `fmt.Println` : il écrit dans le fichier donné en
premier argument au lieu de l'écran.

## Lire ligne par ligne

Le même `bufio.Scanner` que pour le clavier fonctionne sur un fichier ouvert avec
`os.Open` (ouverture en lecture seule). C'est la bonne manière de traiter un fichier
ligne après ligne.

```go
package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	os.WriteFile("courses.txt", []byte("pommes\nfarine\nœufs\n"), 0644)

	f, err := os.Open("courses.txt")
	if err != nil {
		fmt.Println(err)
		return
	}
	defer f.Close()

	scanner := bufio.NewScanner(f)
	numero := 1
	for scanner.Scan() {
		fmt.Printf("%d. %s\n", numero, scanner.Text())
		numero++
	}
	if err := scanner.Err(); err != nil {
		fmt.Println("Erreur de lecture :", err)
	}
}
```

```sortie
1. pommes
2. farine
3. œufs
```

`scanner.Err()` signale une éventuelle erreur rencontrée pendant la lecture.

## Ajouter à la fin d'un fichier

`os.WriteFile` et `os.Create` effacent l'ancien contenu. Pour **ajouter** des lignes à
la fin (un journal, par exemple), utilisez `os.OpenFile` avec des **options** combinées
par `|` :

| Option | Effet |
|--------|-------|
| `os.O_APPEND` | écrire à la fin du fichier |
| `os.O_CREATE` | créer le fichier s'il n'existe pas |
| `os.O_WRONLY` | ouvrir en écriture seule |

```go
package main

import (
	"fmt"
	"os"
)

func ajouter(ligne string) error {
	f, err := os.OpenFile("journal.txt", os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	if err != nil {
		return err
	}
	defer f.Close()
	_, err = fmt.Fprintln(f, ligne)
	return err
}

func main() {
	ajouter("lundi : footing")
	ajouter("mardi : piscine")
	ajouter("mercredi : repos")

	contenu, _ := os.ReadFile("journal.txt")
	fmt.Print(string(contenu))
}
```

```sortie
lundi : footing
mardi : piscine
mercredi : repos
```

Chaque appel à `ajouter` rouvre le fichier et écrit après les lignes précédentes.

## Le fichier existe-t-il ?

`os.Stat` renvoie des informations sur un fichier : taille, date, dossier ou non. S'il
n'existe pas, l'erreur renvoyée « est » l'erreur `fs.ErrNotExist`. `errors.Is` vérifie
cela, même si l'erreur a été enveloppée.

```go
package main

import (
	"errors"
	"fmt"
	"io/fs"
	"os"
)

func decrire(nom string) {
	info, err := os.Stat(nom)
	if errors.Is(err, fs.ErrNotExist) {
		fmt.Println(nom, ": n'existe pas")
		return
	}
	if err != nil {
		fmt.Println(nom, ": erreur", err)
		return
	}
	if info.IsDir() {
		fmt.Println(nom, ": un dossier")
		return
	}
	fmt.Println(nom, ":", info.Size(), "octets")
}

func main() {
	os.WriteFile("config.txt", []byte("couleur=bleu\n"), 0644)
	os.Mkdir("photos", 0755)

	decrire("config.txt")
	decrire("photos")
	decrire("absent.txt")
}
```

```sortie
config.txt : 13 octets
photos : un dossier
absent.txt : n'existe pas
```

`info.IsDir()` indique si le chemin désigne un dossier, et `info.Size()` donne la
taille d'un fichier en octets.

## Construire un chemin avec filepath.Join

Un chemin colle des noms de dossiers avec un séparateur : `/` sous Linux et macOS, `\`
sous Windows. `filepath.Join` choisit le bon séparateur et nettoie les doublons.
N'assemblez pas les chemins à la main avec `+`.

```go
package main

import (
	"fmt"
	"os"
	"path/filepath"
)

func main() {
	dossier := filepath.Join("donnees", "2026")
	os.MkdirAll(dossier, 0755)

	chemin := filepath.Join(dossier, "ventes.txt")
	os.WriteFile(chemin, []byte("42\n"), 0644)

	fmt.Println(chemin)
	fmt.Println(filepath.Base(chemin))
	fmt.Println(filepath.Ext(chemin))
	fmt.Println(filepath.Dir(chemin))
}
```

```sortie
donnees/2026/ventes.txt
ventes.txt
.txt
donnees/2026
```

- `os.MkdirAll` crée le dossier et tous ses parents si besoin ;
- `filepath.Base` donne le dernier élément, `filepath.Ext` l'extension,
  `filepath.Dir` le dossier parent.

> **Note :** sous Windows, le même programme afficherait `donnees\2026\ventes.txt`.

## Erreurs fréquentes

### Oublier la valeur d'erreur

```go norun
package main

import (
	"fmt"
	"os"
)

func main() {
	contenu := os.ReadFile("notes.txt")
	fmt.Println(string(contenu))
}
```

```text
# command-line-arguments
./main.go:9:13: assignment mismatch: 1 variable but os.ReadFile returns 2 values
```

`os.ReadFile` renvoie **deux** valeurs : le contenu et l'erreur. Récupérez les deux :
`contenu, err := os.ReadFile("notes.txt")`, puis testez `err`.

### Donner du texte à os.WriteFile

```go norun
package main

import "os"

func main() {
	os.WriteFile("notes.txt", "Bonjour")
}
```

```text
# command-line-arguments
./main.go:6:28: not enough arguments in call to os.WriteFile
	have (string, string)
	want (string, []byte, os.FileMode)
```

Le message dit ce qui est attendu (`want`) : des octets et des permissions. Écrivez
`os.WriteFile("notes.txt", []byte("Bonjour"), 0644)`.

### Écrire dans un fichier ouvert avec os.Open

```go norun
f, err := os.Open("journal.txt")
if err != nil {
	fmt.Println(err)
	return
}
defer f.Close()

_, err = f.WriteString("mardi\n")
fmt.Println("Erreur :", err)
```

```text
Erreur : write journal.txt: bad file descriptor
```

`os.Open` ouvre en **lecture seule**. Pour écrire, utilisez `os.Create` (remplacer) ou
`os.OpenFile` avec `os.O_APPEND|os.O_WRONLY` (ajouter).

## Testez-vous

**Question 1 :** quelle différence entre `os.Create("a.txt")` et
`os.OpenFile("a.txt", os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)` ?

> **Réponse**
>
> `os.Create` vide le fichier s'il existe déjà. `os.OpenFile` avec `os.O_APPEND`
> garde le contenu et écrit à la fin.

**Question 2 :** pourquoi écrire `defer f.Close()` juste après l'ouverture ?

> **Réponse**
>
> Pour être sûr que le fichier sera fermé à la fin de la fonction, quel que soit le
> chemin pris (un `return` anticipé, par exemple). On ne peut plus l'oublier.

**Question 3 :** écrivez un programme qui crée `nombres.txt` contenant `3`, `5` et `8`
(un par ligne), puis le relit ligne par ligne et affiche la somme.

> **Réponse**
>
> ```go
> package main
>
> import (
> 	"bufio"
> 	"fmt"
> 	"os"
> 	"strconv"
> )
>
> func main() {
> 	os.WriteFile("nombres.txt", []byte("3\n5\n8\n"), 0644)
>
> 	f, err := os.Open("nombres.txt")
> 	if err != nil {
> 		fmt.Println(err)
> 		return
> 	}
> 	defer f.Close()
>
> 	somme := 0
> 	scanner := bufio.NewScanner(f)
> 	for scanner.Scan() {
> 		n, err := strconv.Atoi(scanner.Text())
> 		if err != nil {
> 			continue
> 		}
> 		somme += n
> 	}
> 	fmt.Println("Somme :", somme)
> }
> ```
>
> ```sortie
> Somme : 16
> ```

## En résumé

- `bufio.NewScanner(os.Stdin)` lit le clavier ligne par ligne avec `Scan()` et `Text()`.
- `os.WriteFile` et `os.ReadFile` écrivent et lisent un fichier entier, en `[]byte`.
- `os.Create`, `os.Open` et `os.OpenFile` ouvrent un fichier ; `defer f.Close()` le ferme.
- `bufio.Scanner` sur un fichier ouvert le lit ligne par ligne.
- `os.Stat` et `errors.Is(err, fs.ErrNotExist)` testent l'existence d'un fichier.
- `filepath.Join` construit des chemins valables sur tous les systèmes.

Suite : [JSON et CSV](formats.md). Comparez avec
[les fichiers en Ruby](../ruby/fichiers.md).
