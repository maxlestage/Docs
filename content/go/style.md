# Bien écrire du Go

Cette page présente les habitudes d'écriture des programmeurs Go : la mise en forme automatique avec `gofmt`, les noms, la gestion des erreurs, les retours anticipés, les petites interfaces et les commentaires de documentation.

> **Objectifs :**
>
> - mettre en forme son code avec `gofmt` ;
> - nommer variables, fonctions, types et récepteurs comme le veut l'usage ;
> - gérer les erreurs et simplifier une fonction avec des retours anticipés ;
> - profiter des valeurs zéro et des petites interfaces ;
> - documenter son code et le vérifier avec `golangci-lint`.

## Pourquoi soigner son style ?

Un programme est lu bien plus souvent qu'il n'est écrit. En Go, le style est
particulièrement uniforme : presque tout le monde suit les mêmes règles, tirées du texte
*Effective Go* et des revues de code de l'équipe Go. Résultat : un programme écrit par
un inconnu se lit comme le vôtre.

Ces règles ne changent pas ce que fait le programme, seulement la facilité à le lire.

## gofmt : la mise en forme automatique

En Go, on ne discute pas de la mise en page : l'outil `gofmt` la fait pour vous. Il
règle l'indentation (avec des tabulations), les espaces et l'alignement.

Voici un fichier mal présenté :

```go norun
package main
import "fmt"
func   Afficher(x int){
fmt.Println( x )
}
```

L'option `-d` montre ce que `gofmt` changerait, sans toucher au fichier :

```sh
gofmt -d bad.go
```

```text
diff bad.go.orig bad.go
--- bad.go.orig
+++ bad.go
@@ -1,5 +1,7 @@
 package main
+
 import "fmt"
-func   Afficher(x int){
-fmt.Println( x )
+
+func Afficher(x int) {
+	fmt.Println(x)
 }
```

Les commandes utiles :

| Commande | Effet |
|----------|-------|
| `gofmt -w main.go` | réécrit le fichier bien formaté |
| `gofmt -l .` | liste les fichiers mal formatés |
| `go fmt ./...` | formate tous les fichiers du module |

> **Astuce :** réglez votre [éditeur](../debuter/editeur.md) pour lancer `gofmt` à
> chaque enregistrement. L'extension Go de VS Code le fait par défaut. Vous n'aurez
> plus jamais à y penser.

## Les noms

Go a des conventions de noms courtes et précises :

| Quoi | Style | Exemple |
|------|-------|---------|
| variable, fonction privée | `mixedCaps` (minuscule au début) | `prixTotal`, `calculerTVA` |
| nom exporté (visible des autres paquets) | `MixedCaps` (majuscule au début) | `PrixTotal`, `Compte` |
| sigle | tout en majuscules ou tout en minuscules | `URL`, `userID`, `serveurHTTP` |
| paquet | un mot, en minuscules | `strings`, `temperature` |
| interface à une méthode | le verbe + `-er` | `Reader`, `Stringer` |
| récepteur | une ou deux lettres | `func (c *Compte) Deposer(...)` |

Jamais de `_` dans les noms (`prix_total`) : c'est du style Ruby ou Python, pas Go.

**La majuscule a un sens.** Un nom qui commence par une majuscule est **exporté** :
les autres [paquets](../debuter/glossaire.md#paquet) peuvent l'utiliser. En minuscule, il
reste privé au paquet (voir [les modules](modules.md)).

**Des noms courts quand la portée est courte.** Une variable qui vit trois lignes peut
s'appeler `i`, `n` ou `s`. Une variable qui traverse toute une fonction mérite un nom
plus long. Les noms `err` (une erreur), `ok` (un booléen de succès) et `ctx` sont
universels.

**Pas de `Get` devant un accesseur.** Une méthode qui renvoie le solde s'appelle
`Solde()`, pas `GetSolde()`.

```go
package main

import "fmt"

const tauxTVA = 0.2

type Article struct {
	Nom    string
	PrixHT float64
}

func (a Article) PrixTTC() float64 {
	return a.PrixHT * (1 + tauxTVA)
}

func main() {
	articles := []Article{{"stylo", 2}, {"cahier", 5}}
	for _, a := range articles {
		fmt.Printf("%s : %.2f\n", a.Nom, a.PrixTTC())
	}
}
```

```sortie
stylo : 2.40
cahier : 6.00
```

Le récepteur s'appelle `a` (pour **A**rticle), et garde ce nom dans toutes les méthodes
du type.

## Gérer les erreurs

En Go, une fonction qui peut échouer renvoie une [erreur](../debuter/glossaire.md#erreur)
en **dernier** résultat. L'appelant la teste tout de suite, avec `if err != nil`. Les
habitudes :

- traiter l'erreur **juste après** l'appel, jamais plus loin ;
- ajouter du contexte avec `fmt.Errorf("...: %w", err)` avant de la remonter ;
- écrire les messages en **minuscules**, sans point final : ils seront enchaînés ;
- ne pas utiliser `panic` pour une erreur ordinaire (un fichier absent, une saisie
  fausse).

```go
package main

import (
	"fmt"
	"strconv"
)

func lireAge(texte string) (int, error) {
	age, err := strconv.Atoi(texte)
	if err != nil {
		return 0, fmt.Errorf("âge invalide %q: %w", texte, err)
	}
	if age < 0 {
		return 0, fmt.Errorf("âge négatif: %d", age)
	}
	return age, nil
}

func main() {
	for _, t := range []string{"30", "trente", "-4"} {
		age, err := lireAge(t)
		if err != nil {
			fmt.Println("erreur :", err)
			continue
		}
		fmt.Println("âge :", age)
	}
}
```

```sortie
âge : 30
erreur : âge invalide "trente": strconv.Atoi: parsing "trente": invalid syntax
erreur : âge négatif: -4
```

Chaque niveau ajoute son morceau d'explication, séparé par `: `. On lit le message comme
une phrase qui va du général au précis. Les détails sont dans
[les erreurs](erreurs.md).

## Les retours anticipés

Le code Go garde le **chemin normal** collé à gauche. Les cas d'erreur sortent tôt de
la fonction, avec `return`. On évite ainsi les `if` imbriqués et les `else` inutiles.

```go
package main

import "fmt"

// Avant : trois niveaux d'imbrication.
func remiseAvant(total float64, fidele bool) float64 {
	if total > 0 {
		if fidele {
			if total >= 100 {
				return total * 0.9
			} else {
				return total * 0.95
			}
		} else {
			return total
		}
	} else {
		return 0
	}
}

// Après : les cas particuliers sortent d'abord.
func remise(total float64, fidele bool) float64 {
	if total <= 0 {
		return 0
	}
	if !fidele {
		return total
	}
	if total >= 100 {
		return total * 0.9
	}
	return total * 0.95
}

func main() {
	fmt.Println(remiseAvant(200, true), remise(200, true))
	fmt.Println(remiseAvant(50, true), remise(50, true))
	fmt.Println(remiseAvant(50, false), remise(50, false))
}
```

```sortie
180 180
47.5 47.5
50 50
```

Les deux versions donnent les mêmes résultats. Mais la seconde se lit de haut en bas,
comme une liste de règles.

> **À retenir :** après un `if` qui se termine par `return`, n'écrivez pas de `else`.
> La suite du code **est** le « sinon ».

## Des valeurs zéro utiles

En Go, toute variable déclarée sans valeur reçoit la **valeur zéro** de son type : `0`,
`""`, `false`, `nil`. Un bon type est **utilisable tout de suite**, sans fonction
d'initialisation. La bibliothèque standard en donne l'exemple :

```go
package main

import (
	"fmt"
	"strings"
	"sync"
)

type Journal struct {
	mu     sync.Mutex
	lignes []string
}

func (j *Journal) Ajouter(ligne string) {
	j.mu.Lock()
	defer j.mu.Unlock()
	j.lignes = append(j.lignes, ligne)
}

func main() {
	var sb strings.Builder // prêt à l'emploi
	sb.WriteString("Bonjour")
	sb.WriteString(" !")
	fmt.Println(sb.String())

	var j Journal // prêt aussi : verrou libre, liste vide
	j.Ajouter("démarrage")
	j.Ajouter("fin")
	fmt.Println(len(j.lignes), j.lignes)
}
```

```sortie
Bonjour !
2 [démarrage fin]
```

`append` fonctionne sur une tranche `nil`, et un `sync.Mutex` à zéro est un verrou
ouvert. `Journal` n'a donc besoin d'aucune préparation. Attention en revanche aux
**maps** : une map `nil` panique à l'écriture (voir [les maps](maps.md)).

## De petites interfaces

Les interfaces Go les plus utiles n'ont qu'**une** méthode : `io.Writer` (`Write`),
`io.Reader` (`Read`), `fmt.Stringer` (`String`). Plus une interface est petite, plus
de types la satisfont.

Deux habitudes en découlent :

- **acceptez des interfaces** : une fonction qui écrit du texte prend un `io.Writer`,
  pas un `*os.File` ;
- **renvoyez des types concrets** : l'appelant garde ainsi accès à tout.

```go
package main

import (
	"fmt"
	"io"
	"os"
	"strings"
)

func ecrireBulletin(w io.Writer, nom string, moyenne float64) {
	fmt.Fprintf(w, "%-8s %5.1f\n", nom, moyenne)
}

func main() {
	// Vers le terminal.
	ecrireBulletin(os.Stdout, "Alice", 14.5)

	// Vers une chaîne en mémoire : pratique pour les tests.
	var sb strings.Builder
	ecrireBulletin(&sb, "Bob", 9)
	fmt.Printf("%q\n", sb.String())
}
```

```sortie
Alice     14.5
"Bob        9.0\n"
```

La même fonction écrit dans le terminal, dans une chaîne, et demain dans un fichier ou
une réponse web. Voir [les interfaces](interfaces.md).

## Commentaires et documentation

Un commentaire placé juste **avant** une déclaration exportée devient sa
**documentation**. La règle : il commence par le nom de ce qu'il décrit, et forme une
phrase complète.

```go norun
// Package temperature convertit des températures.
package temperature

// ZeroAbsolu est la plus basse température possible, en degrés Celsius.
const ZeroAbsolu = -273.15

// VersFahrenheit convertit une température de degrés Celsius
// en degrés Fahrenheit.
func VersFahrenheit(c float64) float64 {
	return c*9/5 + 32
}
```

La commande `go doc` affiche cette documentation dans le terminal :

```sh
go doc ./temperature VersFahrenheit
```

```text
package temperature // import "example.com/meteo/temperature"

func VersFahrenheit(c float64) float64
    VersFahrenheit convertit une température de degrés Celsius en degrés
    Fahrenheit.
```

Elle marche aussi pour la bibliothèque standard : `go doc strings.Split`. Les mêmes
commentaires alimentent le site pkg.go.dev pour les paquets publiés.

Comme dans tous les langages, un commentaire dans le corps d'une fonction explique
**pourquoi**, pas ce que le code dit déjà :

```go norun
// Inutile : le code dit déjà la même chose.
// Ajoute 1 à i.
i++

// Utile : explique un choix.
// La banque arrondit au centime inférieur, pas au plus proche.
centimes := int(math.Floor(montant * 100))
```

## Vérifier avec golangci-lint

`go vet` trouve déjà des fautes probables (voir [déboguer](debogage.md)). L'outil
**golangci-lint** regroupe des dizaines de vérificateurs (*linters*) en une commande. Il
s'installe à part, en suivant les instructions de son site (golangci-lint.run).

```go norun
func main() {
	f, err := os.Create("notes.txt")
	if err != nil {
		fmt.Println(err)
		return
	}
	defer f.Close()
	f.WriteString("bonjour\n")
	fmt.Println("ok")
}
```

```sh
golangci-lint run
```

```text
main.go:14:15: Error return value of `f.Close` is not checked (errcheck)
	defer f.Close()
	             ^
main.go:15:15: Error return value of `f.WriteString` is not checked (errcheck)
	f.WriteString("bonjour\n")
	             ^
2 issues:
* errcheck: 2
```

Le vérificateur `errcheck` rappelle que `WriteString` peut échouer (disque plein, par
exemple) : l'erreur renvoyée est ignorée en silence.

## Une refactorisation complète

Refactoriser, c'est améliorer l'écriture d'un code **sans changer ce qu'il fait**. Voici
une fonction qui marche, mais qui accumule les maladresses : noms avec `_`, `else`
inutiles, erreur ignorée, tout dans une seule fonction.

```go
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func Calcul_Moyenne(Donnees string) string {
	var resultat string
	if Donnees != "" {
		morceaux := strings.Split(Donnees, ",")
		total := 0
		for i := 0; i < len(morceaux); i++ {
			n, _ := strconv.Atoi(strings.TrimSpace(morceaux[i]))
			total = total + n
		}
		resultat = fmt.Sprintf("%.2f", float64(total)/float64(len(morceaux)))
	} else {
		resultat = "aucune note"
	}
	return resultat
}

func main() {
	fmt.Println(Calcul_Moyenne("12, 15, 16"))
	fmt.Println(Calcul_Moyenne(""))
	fmt.Println(Calcul_Moyenne("12, abc"))
}
```

```sortie
14.33
aucune note
6.00
```

La dernière ligne est un bug caché : `"abc"` devient `0` sans prévenir. Après
refactorisation : deux fonctions au rôle clair, des noms Go, un retour anticipé, et une
vraie erreur.

```go
package main

import (
	"errors"
	"fmt"
	"strconv"
	"strings"
)

// lireNotes transforme "12, 15" en []int{12, 15}.
func lireNotes(texte string) ([]int, error) {
	var notes []int
	for _, morceau := range strings.Split(texte, ",") {
		n, err := strconv.Atoi(strings.TrimSpace(morceau))
		if err != nil {
			return nil, fmt.Errorf("note invalide %q", morceau)
		}
		notes = append(notes, n)
	}
	return notes, nil
}

// moyenne renvoie la moyenne des notes, ou une erreur si la liste est vide.
func moyenne(notes []int) (float64, error) {
	if len(notes) == 0 {
		return 0, errors.New("aucune note")
	}
	total := 0
	for _, n := range notes {
		total += n
	}
	return float64(total) / float64(len(notes)), nil
}

func main() {
	for _, texte := range []string{"12, 15, 16", "12, abc"} {
		notes, err := lireNotes(texte)
		if err != nil {
			fmt.Println("erreur :", err)
			continue
		}
		m, err := moyenne(notes)
		if err != nil {
			fmt.Println("erreur :", err)
			continue
		}
		fmt.Printf("%.2f\n", m)
	}
	_, err := moyenne(nil)
	fmt.Println("erreur :", err)
}
```

```sortie
14.33
erreur : note invalide " abc"
erreur : aucune note
```

Le code est un peu plus long, mais chaque fonction fait **une** chose, et plus aucune
erreur ne passe inaperçue. Un [test](tests.md) pour chaque fonction devient facile à
écrire.

## Erreurs fréquentes

### Un nom en minuscule qu'on croit exporté

```go norun
// outils/outils.go
package outils

func double(x int) int { return x * 2 }

// main.go
fmt.Println(outils.double(2))
```

```text
# example.com/exp
./main.go:10:21: undefined: outils.double
```

`double` commence par une minuscule : il est privé au paquet `outils`. Renommez-le
`Double` pour l'exporter.

### Un if / else if sans return final

```go norun
func signe(n int) string {
	if n >= 0 {
		return "positif"
	} else if n < 0 {
		return "négatif"
	}
}
```

```text
./main.go:11:1: missing return
```

Le compilateur ne devine pas que les deux conditions couvrent tous les cas. Écrivez la
fonction avec un retour anticipé : `if n >= 0 { return "positif" }` puis
`return "négatif"`.

### Ignorer l'erreur d'une écriture

```go norun
f.WriteString("bonjour\n")
```

```text
main.go:15:15: Error return value of `f.WriteString` is not checked (errcheck)
```

Le compilateur l'accepte, mais l'écriture peut échouer. Récupérez l'erreur :
`if _, err := f.WriteString("bonjour\n"); err != nil { ... }`.

### Des messages d'erreur avec majuscule et point

```go norun
err := errors.New("Fichier introuvable.")
fmt.Println(fmt.Errorf("lecture de la configuration: %w", err))
```

```text
lecture de la configuration: Fichier introuvable.
```

Une fois enchaîné, le message a une majuscule au milieu de la phrase et un point
en trop. Écrivez `errors.New("fichier introuvable")`.

## Testez-vous

**Question 1 :** comment s'appelle, en Go, la variable `prix_total` ?

> **Réponse**
>
> `prixTotal` : Go utilise les `mixedCaps`, jamais les `_`. Et `PrixTotal` si elle doit
> être visible depuis d'autres paquets.

**Question 2 :** pourquoi accepter un `io.Writer` plutôt qu'un `*os.File` ?

> **Réponse**
>
> Parce que beaucoup de types satisfont `io.Writer` : le terminal, un fichier, un
> `strings.Builder`, une réponse web. La fonction sert partout, et se teste facilement
> en mémoire.

**Question 3 :** quelle commande remet en forme tous les fichiers d'un module ?

> **Réponse**
>
> `go fmt ./...` (ou `gofmt -w .`).

## En résumé

- `gofmt` règle la mise en page : laissez votre éditeur le lancer à chaque
  enregistrement.
- Noms en `mixedCaps`, courts quand la portée est courte ; une majuscule exporte.
- Testez `err` juste après l'appel, ajoutez du contexte avec `%w`, sortez tôt avec
  `return`.
- Des types utilisables à leur valeur zéro, de petites interfaces en paramètre.
- Un commentaire de documentation commence par le nom qu'il décrit ; `go doc` l'affiche.
- `go vet` et `golangci-lint` repèrent le code suspect.

Suite : [organiser un projet](projet.md).
