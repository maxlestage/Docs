# Projets pas à pas

Cette page vous guide dans trois petits projets Go complets, construits étape par étape : un jeu, une liste de tâches enregistrée dans un fichier JSON et un bulletin de notes.

> **Objectifs :**
>
> - découper un programme en petites étapes qui marchent chacune ;
> - réutiliser ce que vous avez appris : conditions, boucles, fonctions, tranches, maps, structures ;
> - enregistrer des données dans un fichier JSON et les relire ;
> - lire des données CSV, calculer et afficher un rapport bien aligné.

## Comment suivre ces projets

Chaque projet avance par petites étapes. À chaque étape :

1. lisez le code et l'explication ;
2. tapez le code dans un fichier `main.go` et lancez-le avec `go run main.go` ;
3. comparez avec la sortie affichée ;
4. modifiez une valeur pour voir ce qui change.

> **Astuce :** ne copiez pas tout d'un coup. Un programme se construit morceau par
> morceau, en le lançant souvent. Le compilateur Go vous aide : il signale chaque
> variable oubliée ou chaque type faux dès que vous lancez le programme.

## Projet 1 : Le juste nombre

Le programme choisit un nombre secret entre 1 et 100. Le joueur propose des nombres ;
le programme répond « trop petit » ou « trop grand », jusqu'à ce que le joueur trouve.

### Étape 1 : comparer un essai

Commencez par le cœur du jeu : comparer **un** essai au nombre secret.

```go
package main

import "fmt"

func main() {
	secret := 42
	essai := 30

	if essai < secret {
		fmt.Println("Trop petit !")
	} else if essai > secret {
		fmt.Println("Trop grand !")
	} else {
		fmt.Println("Gagné !")
	}
}
```

```sortie
Trop petit !
```

Changez `essai` en `50`, puis en `42`, et relancez.

### Étape 2 : une fonction pour l'indice

Rangez cette comparaison dans une [fonction](fonctions.md). Elle **renvoie** le texte au
lieu de l'afficher : elle sera plus facile à réutiliser et à tester. Un `switch` sans
valeur remplace avantageusement la suite de `if`.

```go
package main

import "fmt"

func indice(secret, essai int) string {
	switch {
	case essai < secret:
		return "Trop petit !"
	case essai > secret:
		return "Trop grand !"
	default:
		return "Gagné !"
	}
}

func main() {
	fmt.Println(indice(42, 30))
	fmt.Println(indice(42, 50))
	fmt.Println(indice(42, 42))
}
```

```sortie
Trop petit !
Trop grand !
Gagné !
```

### Étape 3 : plusieurs essais et un compteur

Le joueur fait plusieurs essais. Pour l'instant, ses essais sont dans une
[tranche](collections.md) : le programme donne ainsi toujours le même résultat, et vous
pouvez le vérifier facilement. `range` donne le numéro de l'essai ; `break` arrête la
boucle quand c'est gagné.

```go
package main

import "fmt"

func indice(secret, essai int) string {
	switch {
	case essai < secret:
		return "Trop petit !"
	case essai > secret:
		return "Trop grand !"
	default:
		return "Gagné !"
	}
}

func main() {
	secret := 42
	essais := []int{50, 25, 40, 42, 10}

	for i, essai := range essais {
		fmt.Printf("Essai %d : %d -> %s\n", i+1, essai, indice(secret, essai))
		if essai == secret {
			fmt.Printf("Bravo, trouvé en %d essais !\n", i+1)
			break
		}
	}
}
```

```sortie
Essai 1 : 50 -> Trop grand !
Essai 2 : 25 -> Trop petit !
Essai 3 : 40 -> Trop petit !
Essai 4 : 42 -> Gagné !
Bravo, trouvé en 4 essais !
```

Le dernier essai (`10`) n'est jamais lu : `break` a arrêté la boucle avant.

### Étape 4 : limiter le nombre d'essais

Pour corser le jeu, le joueur n'a droit qu'à 5 essais. Rangez la partie dans une
fonction `jouer` qui renvoie `true` si le joueur gagne. `essais[:maximum]` ne garde que
les premiers essais, à condition qu'il y en ait assez.

```go
package main

import "fmt"

func indice(secret, essai int) string {
	switch {
	case essai < secret:
		return "Trop petit !"
	case essai > secret:
		return "Trop grand !"
	default:
		return "Gagné !"
	}
}

func jouer(secret int, essais []int, maximum int) bool {
	if len(essais) > maximum {
		essais = essais[:maximum]
	}
	for i, essai := range essais {
		fmt.Printf("Essai %d : %d -> %s\n", i+1, essai, indice(secret, essai))
		if essai == secret {
			return true
		}
	}
	return false
}

func main() {
	if jouer(42, []int{50, 25, 40, 42}, 5) {
		fmt.Println("Bravo !")
	}

	fmt.Println("---")

	if !jouer(7, []int{50, 25, 12, 10, 8, 7}, 5) {
		fmt.Println("Perdu ! Le nombre était 7.")
	}
}
```

```sortie
Essai 1 : 50 -> Trop grand !
Essai 2 : 25 -> Trop petit !
Essai 3 : 40 -> Trop petit !
Essai 4 : 42 -> Gagné !
Bravo !
---
Essai 1 : 50 -> Trop grand !
Essai 2 : 25 -> Trop grand !
Essai 3 : 12 -> Trop grand !
Essai 4 : 10 -> Trop grand !
Essai 5 : 8 -> Trop grand !
Perdu ! Le nombre était 7.
```

> **Note :** `return true` quitte la fonction tout de suite, en pleine boucle. Si la
> boucle se termine sans victoire, la fonction arrive à la dernière ligne et renvoie
> `false`.

### Étape 5 : la vraie version, au clavier

Il reste deux changements pour un vrai jeu :

- `rand.IntN(100) + 1` (paquet `math/rand/v2`) tire un nombre au hasard entre 1 et 100 ;
- un `bufio.Scanner` lit les lignes tapées par le joueur, et `strconv.Atoi` les
  transforme en nombres (voir [la ligne de commande](ligne-de-commande.md)).

Ce code attend le clavier : il n'est pas exécuté ici. Lancez-le chez vous.

```go norun
package main

import (
	"bufio"
	"fmt"
	"math/rand/v2"
	"os"
	"strconv"
	"strings"
)

func indice(secret, essai int) string {
	switch {
	case essai < secret:
		return "Trop petit !"
	case essai > secret:
		return "Trop grand !"
	default:
		return "Gagné !"
	}
}

func main() {
	secret := rand.IntN(100) + 1
	clavier := bufio.NewScanner(os.Stdin)
	fmt.Println("J'ai choisi un nombre entre 1 et 100. Vous avez 5 essais.")

	for i := 1; i <= 5; i++ {
		fmt.Printf("Essai %d : ", i)
		if !clavier.Scan() {
			return // fin de la saisie (Ctrl+D)
		}
		essai, err := strconv.Atoi(strings.TrimSpace(clavier.Text()))
		if err != nil {
			fmt.Println("Ce n'est pas un nombre.")
			continue
		}
		fmt.Println(indice(secret, essai))
		if essai == secret {
			fmt.Println("Bravo !")
			return
		}
	}
	fmt.Printf("Perdu ! Le nombre était %d.\n", secret)
}
```

Une partie peut ressembler à ceci :

```text
J'ai choisi un nombre entre 1 et 100. Vous avez 5 essais.
Essai 1 : 50
Trop grand !
Essai 2 : 25
Trop petit !
Essai 3 : 37
Gagné !
Bravo !
```

### Pour aller plus loin

- Ne comptez pas un essai qui n'est pas un nombre (`i--` avant `continue`).
- Refusez un essai hors de 1..100.
- Demandez au joueur s'il veut rejouer, et entourez la partie d'une boucle `for`.
- Écrivez un test pour `indice` (voir [les tests](tests.md)).

## Projet 2 : Une liste de tâches

Une petite application en ligne de commande : ajouter une tâche, afficher la liste,
marquer une tâche comme faite. Les tâches sont enregistrées dans un fichier
[JSON](../debuter/glossaire.md#json) : elles sont toujours là au prochain lancement.

### Étape 1 : représenter les tâches

Une tâche a un titre et un état (faite ou non) : c'est une
[structure](../debuter/glossaire.md#structure). La liste est une tranche de tâches.

Les textes entre accents graves, comme `` `json:"titre"` ``, sont des **étiquettes** :
ils indiquent le nom de chaque champ dans le fichier JSON (voir [les formats](formats.md)).

```go
package main

import "fmt"

type Tache struct {
	Titre string `json:"titre"`
	Faite bool   `json:"faite"`
}

func main() {
	taches := []Tache{
		{Titre: "Acheter du pain"},
		{Titre: "Appeler Zoé", Faite: true},
	}
	for i, t := range taches {
		fmt.Println(i+1, t.Titre, t.Faite)
	}
}
```

```sortie
1 Acheter du pain false
2 Appeler Zoé true
```

`Faite` n'est pas précisé pour la première tâche : il prend sa **valeur zéro**, `false`.

### Étape 2 : ajouter, afficher, terminer

Écrivez une fonction par action. `terminer` reçoit un **numéro** qui commence à 1,
comme pour un humain ; l'indice de la tranche commence à 0, d'où le `numero-1`. Elle
renvoie une [erreur](erreurs.md) si le numéro n'existe pas.

```go
package main

import "fmt"

type Tache struct {
	Titre string `json:"titre"`
	Faite bool   `json:"faite"`
}

func ajouter(taches []Tache, titre string) []Tache {
	return append(taches, Tache{Titre: titre})
}

func terminer(taches []Tache, numero int) error {
	if numero < 1 || numero > len(taches) {
		return fmt.Errorf("pas de tâche numéro %d", numero)
	}
	taches[numero-1].Faite = true
	return nil
}

func afficher(taches []Tache) {
	if len(taches) == 0 {
		fmt.Println("Aucune tâche.")
		return
	}
	for i, t := range taches {
		case_ := "[ ]"
		if t.Faite {
			case_ = "[x]"
		}
		fmt.Printf("%d. %s %s\n", i+1, case_, t.Titre)
	}
}

func main() {
	var taches []Tache
	afficher(taches)
	taches = ajouter(taches, "Acheter du pain")
	taches = ajouter(taches, "Réviser Go")
	if err := terminer(taches, 1); err != nil {
		fmt.Println("Erreur :", err)
	}
	if err := terminer(taches, 5); err != nil {
		fmt.Println("Erreur :", err)
	}
	afficher(taches)
}
```

```sortie
Aucune tâche.
Erreur : pas de tâche numéro 5
1. [x] Acheter du pain
2. [ ] Réviser Go
```

> **Note :** `append` peut créer une nouvelle tranche. Il faut donc récupérer son
> résultat : `taches = ajouter(taches, ...)`. `terminer`, lui, modifie une case
> existante : pas besoin de renvoyer la tranche.

### Étape 3 : enregistrer dans un fichier JSON

`json.MarshalIndent` transforme la tranche en texte JSON bien présenté ;
`os.WriteFile` l'écrit sur le disque. Pour relire : `os.ReadFile`, puis
`json.Unmarshal` (voir [les fichiers](fichiers.md)).

Au tout premier lancement, le fichier n'existe pas : ce n'est pas une erreur, on part
d'une liste vide. `errors.Is(err, os.ErrNotExist)` reconnaît ce cas.

```go
package main

import (
	"encoding/json"
	"errors"
	"fmt"
	"os"
)

type Tache struct {
	Titre string `json:"titre"`
	Faite bool   `json:"faite"`
}

const fichier = "taches.json"

func charger() ([]Tache, error) {
	donnees, err := os.ReadFile(fichier)
	if errors.Is(err, os.ErrNotExist) {
		return nil, nil
	}
	if err != nil {
		return nil, err
	}
	var taches []Tache
	err = json.Unmarshal(donnees, &taches)
	return taches, err
}

func enregistrer(taches []Tache) error {
	donnees, err := json.MarshalIndent(taches, "", "  ")
	if err != nil {
		return err
	}
	return os.WriteFile(fichier, donnees, 0o644)
}

func main() {
	// Premier lancement : le fichier n'existe pas encore.
	taches, err := charger()
	if err != nil {
		fmt.Println("Erreur :", err)
		return
	}
	fmt.Println("Au départ :", len(taches), "tâche(s)")

	taches = append(taches, Tache{Titre: "Acheter du pain"})
	if err := enregistrer(taches); err != nil {
		fmt.Println("Erreur :", err)
		return
	}

	// Lancement suivant : la tâche est relue depuis le fichier.
	relues, _ := charger()
	fmt.Printf("Relu : %+v\n", relues)

	contenu, _ := os.ReadFile(fichier)
	fmt.Println(string(contenu))
}
```

```sortie
Au départ : 0 tâche(s)
Relu : [{Titre:Acheter du pain Faite:false}]
[
  {
    "titre": "Acheter du pain",
    "faite": false
  }
]
```

`0o644` donne les droits du fichier : lisible par tous, modifiable par vous.

### Étape 4 : le programme complet

Assemblez le tout dans une fonction `executer(commande, arguments)`. Chaque appel
charge le fichier, fait l'action et enregistre : exactement comme si l'utilisateur
lançait le programme plusieurs fois de suite.

```go
package main

import (
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Tache struct {
	Titre string `json:"titre"`
	Faite bool   `json:"faite"`
}

const fichier = "taches.json"

func charger() ([]Tache, error) {
	donnees, err := os.ReadFile(fichier)
	if errors.Is(err, os.ErrNotExist) {
		return nil, nil
	}
	if err != nil {
		return nil, err
	}
	var taches []Tache
	err = json.Unmarshal(donnees, &taches)
	return taches, err
}

func enregistrer(taches []Tache) error {
	donnees, err := json.MarshalIndent(taches, "", "  ")
	if err != nil {
		return err
	}
	return os.WriteFile(fichier, donnees, 0o644)
}

func afficher(taches []Tache) {
	if len(taches) == 0 {
		fmt.Println("Aucune tâche.")
		return
	}
	for i, t := range taches {
		case_ := "[ ]"
		if t.Faite {
			case_ = "[x]"
		}
		fmt.Printf("%d. %s %s\n", i+1, case_, t.Titre)
	}
}

func executer(commande string, arguments ...string) error {
	taches, err := charger()
	if err != nil {
		return err
	}
	switch commande {
	case "ajouter":
		titre := strings.Join(arguments, " ")
		if titre == "" {
			return errors.New("titre manquant")
		}
		taches = append(taches, Tache{Titre: titre})
		fmt.Println("Ajouté :", titre)
	case "finir":
		if len(arguments) == 0 {
			return errors.New("numéro manquant")
		}
		numero, err := strconv.Atoi(arguments[0])
		if err != nil || numero < 1 || numero > len(taches) {
			return fmt.Errorf("numéro invalide : %s", arguments[0])
		}
		taches[numero-1].Faite = true
		fmt.Println("Terminé :", taches[numero-1].Titre)
	case "liste":
		afficher(taches)
		return nil
	default:
		return fmt.Errorf("commande inconnue : %s", commande)
	}
	return enregistrer(taches)
}

func main() {
	appels := [][]string{
		{"liste"},
		{"ajouter", "Acheter", "du", "pain"},
		{"ajouter", "Réviser", "Go"},
		{"finir", "1"},
		{"finir", "9"},
		{"liste"},
		{"effacer"},
	}
	for _, a := range appels {
		fmt.Println("$ taches", strings.Join(a, " "))
		if err := executer(a[0], a[1:]...); err != nil {
			fmt.Println("Erreur :", err)
		}
	}
}
```

```sortie
$ taches liste
Aucune tâche.
$ taches ajouter Acheter du pain
Ajouté : Acheter du pain
$ taches ajouter Réviser Go
Ajouté : Réviser Go
$ taches finir 1
Terminé : Acheter du pain
$ taches finir 9
Erreur : numéro invalide : 9
$ taches liste
1. [x] Acheter du pain
2. [ ] Réviser Go
$ taches effacer
Erreur : commande inconnue : effacer
```

`arguments ...string` rend la fonction **variadique** : elle accepte autant de textes
qu'on veut. `a[1:]...` passe tous les éléments d'une tranche sauf le premier.

### Étape 5 : depuis le terminal avec os.Args

Pour un vrai outil, les commandes viennent du terminal. `os.Args[0]` est le nom du
programme, `os.Args[1]` la commande, et la suite les arguments. Remplacez la fonction
`main` :

```go norun
func main() {
	if len(os.Args) < 2 {
		fmt.Println("usage : taches liste | ajouter TITRE | finir NUMÉRO")
		os.Exit(1)
	}
	if err := executer(os.Args[1], os.Args[2:]...); err != nil {
		fmt.Println("Erreur :", err)
		os.Exit(1)
	}
}
```

```sh
go build -o taches .
./taches ajouter Arroser les plantes
./taches liste
```

```text
Ajouté : Arroser les plantes
1. [ ] Arroser les plantes
```

### Pour aller plus loin

- Ajoutez une commande `supprimer NUMÉRO` (astuce : `slices.Delete`).
- Ajoutez une date limite à chaque tâche, avec le paquet `time` (voir
  [les dates](dates.md)).
- Utilisez le paquet `flag` pour une option `-fichier` qui choisit le fichier JSON.
- Testez `executer` dans un dossier temporaire (`t.TempDir()`, voir [les tests](tests.md)).

## Projet 3 : Le bulletin de notes

Le programme lit les notes d'une classe, écrites en texte au format CSV. Il calcule la
moyenne de chaque élève et de chaque matière, trouve le meilleur élève, puis affiche un
bulletin bien aligné.

### Étape 1 : lire les données

Les notes sont dans un texte CSV : une ligne de titres, puis une ligne par élève. Un
texte sur plusieurs lignes s'écrit entre accents graves. Le paquet `encoding/csv` le
découpe en lignes et en cases ; `strings.NewReader` lui présente le texte comme un
fichier.

```go
package main

import (
	"encoding/csv"
	"fmt"
	"strings"
)

const donnees = `nom,maths,francais,histoire
Alice,15,12,17
Bob,9,14,11
Chloé,18,16,13`

func main() {
	lignes, err := csv.NewReader(strings.NewReader(donnees)).ReadAll()
	if err != nil {
		fmt.Println("Erreur :", err)
		return
	}
	fmt.Printf("%q\n", lignes[0])
	fmt.Printf("%q\n", lignes[1])
	fmt.Println(len(lignes)-1, "élèves")
}
```

```sortie
["nom" "maths" "francais" "histoire"]
["Alice" "15" "12" "17"]
3 élèves
```

Chaque case est encore du **texte** : `"15"`, pas `15`. Il faut la convertir.

> **Note :** dans un vrai programme, le texte viendrait d'un fichier : `os.Open("notes.csv")`
> donne directement un lecteur à passer à `csv.NewReader`.

### Étape 2 : les élèves et leur moyenne

Rangez chaque élève dans une structure, en convertissant les notes avec `strconv.Atoi`.
Une note illisible produit une erreur claire, avec le nom de l'élève. Pour la moyenne,
convertissez en `float64` **avant** de diviser.

```go
package main

import (
	"encoding/csv"
	"fmt"
	"strconv"
	"strings"
)

const donnees = `nom,maths,francais,histoire
Alice,15,12,17
Bob,9,14,11
Chloé,18,16,13`

type Eleve struct {
	Nom   string
	Notes []int
}

func lireEleves(texte string) ([]string, []Eleve, error) {
	lignes, err := csv.NewReader(strings.NewReader(texte)).ReadAll()
	if err != nil {
		return nil, nil, err
	}
	matieres := lignes[0][1:]
	var eleves []Eleve
	for _, ligne := range lignes[1:] {
		e := Eleve{Nom: ligne[0]}
		for _, caseNote := range ligne[1:] {
			n, err := strconv.Atoi(caseNote)
			if err != nil {
				return nil, nil, fmt.Errorf("note de %s : %w", e.Nom, err)
			}
			e.Notes = append(e.Notes, n)
		}
		eleves = append(eleves, e)
	}
	return matieres, eleves, nil
}

func moyenne(notes []int) float64 {
	total := 0
	for _, n := range notes {
		total += n
	}
	return float64(total) / float64(len(notes))
}

func main() {
	_, eleves, err := lireEleves(donnees)
	if err != nil {
		fmt.Println("Erreur :", err)
		return
	}
	for _, e := range eleves {
		fmt.Printf("%s : %.1f\n", e.Nom, moyenne(e.Notes))
	}

	_, _, err = lireEleves("nom,maths\nDavid,quinze")
	fmt.Println("Erreur :", err)
}
```

```sortie
Alice : 14.7
Bob : 11.3
Chloé : 15.7
Erreur : note de David : strconv.Atoi: parsing "quinze": invalid syntax
```

### Étape 3 : le meilleur élève et les moyennes par matière

Pour trouver le meilleur, on parcourt les élèves en gardant le plus fort vu jusqu'ici.
Pour une matière, on prend la note numéro `i` de chaque élève.

```go
package main

import "fmt"

type Eleve struct {
	Nom   string
	Notes []int
}

func moyenne(notes []int) float64 {
	total := 0
	for _, n := range notes {
		total += n
	}
	return float64(total) / float64(len(notes))
}

func meilleur(eleves []Eleve) Eleve {
	premier := eleves[0]
	for _, e := range eleves[1:] {
		if moyenne(e.Notes) > moyenne(premier.Notes) {
			premier = e
		}
	}
	return premier
}

func main() {
	matieres := []string{"maths", "francais", "histoire"}
	eleves := []Eleve{
		{"Alice", []int{15, 12, 17}},
		{"Bob", []int{9, 14, 11}},
		{"Chloé", []int{18, 16, 13}},
	}

	fmt.Println("Meilleure moyenne :", meilleur(eleves).Nom)

	for i, matiere := range matieres {
		var notes []int
		for _, e := range eleves {
			notes = append(notes, e.Notes[i])
		}
		fmt.Printf("%s : %.1f\n", matiere, moyenne(notes))
	}
}
```

```sortie
Meilleure moyenne : Chloé
maths : 14.0
francais : 14.0
histoire : 13.7
```

### Étape 4 : un bulletin bien aligné

Il reste à afficher un tableau propre. Dans un verbe de `Printf`, un nombre fixe la
**largeur** de la colonne : `%-10s` aligne un texte à gauche sur 10 caractères, `%9d`
aligne un nombre à droite sur 9. Le paquet `text/tabwriter` sait aussi le faire, et
gère mieux les lettres accentuées : il compte les caractères, pas les octets.

```go
package main

import (
	"encoding/csv"
	"fmt"
	"os"
	"strconv"
	"strings"
	"text/tabwriter"
)

const donnees = `nom,maths,francais,histoire
Alice,15,12,17
Bob,9,14,11
Chloé,18,16,13`

type Eleve struct {
	Nom   string
	Notes []int
}

func lireEleves(texte string) ([]string, []Eleve, error) {
	lignes, err := csv.NewReader(strings.NewReader(texte)).ReadAll()
	if err != nil {
		return nil, nil, err
	}
	var eleves []Eleve
	for _, ligne := range lignes[1:] {
		e := Eleve{Nom: ligne[0]}
		for _, caseNote := range ligne[1:] {
			n, err := strconv.Atoi(caseNote)
			if err != nil {
				return nil, nil, fmt.Errorf("note de %s : %w", e.Nom, err)
			}
			e.Notes = append(e.Notes, n)
		}
		eleves = append(eleves, e)
	}
	return lignes[0][1:], eleves, nil
}

func moyenne(notes []int) float64 {
	total := 0
	for _, n := range notes {
		total += n
	}
	return float64(total) / float64(len(notes))
}

func main() {
	matieres, eleves, err := lireEleves(donnees)
	if err != nil {
		fmt.Println("Erreur :", err)
		return
	}

	w := tabwriter.NewWriter(os.Stdout, 0, 0, 2, ' ', tabwriter.AlignRight)
	fmt.Fprint(w, "Élève\t")
	for _, m := range matieres {
		fmt.Fprintf(w, "%s\t", m)
	}
	fmt.Fprintln(w, "moyenne\t")

	for _, e := range eleves {
		fmt.Fprintf(w, "%s\t", e.Nom)
		for _, n := range e.Notes {
			fmt.Fprintf(w, "%d\t", n)
		}
		fmt.Fprintf(w, "%.1f\t\n", moyenne(e.Notes))
	}

	fmt.Fprint(w, "Classe\t")
	var toutes []int
	for i := range matieres {
		var notes []int
		for _, e := range eleves {
			notes = append(notes, e.Notes[i])
		}
		toutes = append(toutes, notes...)
		fmt.Fprintf(w, "%.1f\t", moyenne(notes))
	}
	fmt.Fprintf(w, "%.1f\t\n", moyenne(toutes))
	w.Flush()
}
```

```sortie
   Élève  maths  francais  histoire  moyenne
   Alice     15        12        17     14.7
     Bob      9        14        11     11.3
   Chloé     18        16        13     15.7
  Classe   14.0      14.0      13.7     13.9
```

`tabwriter` reçoit les cases séparées par `\t`, puis calcule la largeur de chaque
colonne au moment de `Flush`. Les arguments de `NewWriter` règlent l'espace entre les
colonnes (`2`), le caractère de remplissage (`' '`) et l'alignement à droite. N'oubliez pas cet appel final : sans lui, rien ne
s'affiche.

### Pour aller plus loin

- Lisez les notes depuis un vrai fichier `notes.csv` avec `os.Open`.
- Enregistrez le bulletin dans `bulletin.txt` : donnez le fichier à `tabwriter` au lieu
  de `os.Stdout`.
- Ignorez les cases vides (`,,`) au lieu de produire une erreur.
- Triez les élèves par moyenne décroissante avec `slices.SortFunc`.

## Erreurs fréquentes

### Additionner du texte sans le convertir

Les cases CSV et les morceaux de `strings.Split` sont des **chaînes**. Go refuse de les
ajouter à un nombre.

```go norun
total := 0
for _, morceau := range strings.Split("15,12,17", ",") {
	total += morceau
}
```

```text
./main.go:11:3: invalid operation: total += morceau (mismatched types int and string)
```

Convertissez chaque morceau avec `n, err := strconv.Atoi(morceau)`, et testez `err`.

### Des champs en minuscules dans le JSON

```go norun
type Tache struct {
	titre string
	faite bool
}

donnees, _ := json.Marshal([]Tache{{"Acheter du pain", false}})
fmt.Println(string(donnees))
```

```text
[{}]
```

Aucune erreur, mais le JSON est vide ! `encoding/json` ne voit que les champs
**exportés**, ceux qui commencent par une majuscule. Écrivez `Titre` et `Faite`, avec
les étiquettes `` `json:"titre"` `` pour garder des noms en minuscules dans le fichier.

### Un numéro de tâche décalé

L'utilisateur tape `1` pour la première tâche, mais les indices commencent à 0.

```go norun
taches := []Tache{{Titre: "Pain"}}
numero := 1
taches[numero].Faite = true
```

```text
panic: runtime error: index out of range [1] with length 1
```

Écrivez `taches[numero-1]`, et vérifiez d'abord que `numero` est entre 1 et
`len(taches)`.

### La division entière

Entre deux entiers, `/` donne un entier : la moyenne perd ses décimales.

```go
package main

import "fmt"

func main() {
	fmt.Println(44 / 3)
	fmt.Println(float64(44) / 3)
}
```

```sortie
14
14.666666666666666
```

Convertissez en `float64` avant de diviser : `float64(total) / float64(len(notes))`.

## Testez-vous

**1.** Dans le juste nombre, pourquoi utiliser une tranche d'essais plutôt que le
clavier pendant la mise au point ?

> **Réponse**
>
> Le programme donne toujours le même résultat, sans rien taper : on peut le relancer
> et le vérifier vite. On passe au clavier une fois que tout marche.

**2.** Pourquoi `charger` teste-t-elle `errors.Is(err, os.ErrNotExist)` ?

> **Réponse**
>
> Au tout premier lancement, `taches.json` n'existe pas encore. Ce n'est pas une vraie
> erreur : on part d'une liste vide. Les autres erreurs (droits, disque) sont remontées.

**3.** Que se passe-t-il si on oublie `w.Flush()` avec `tabwriter` ?

> **Réponse**
>
> Rien ne s'affiche : `tabwriter` garde le texte en mémoire pour calculer la largeur des
> colonnes, et ne l'écrit qu'au moment de `Flush`.

## En résumé

- Un projet se construit par petites étapes, en lançant le programme à chaque fois.
- Remplacer le clavier par des valeurs fixes rend un programme facile à vérifier.
- Des fonctions courtes (`indice`, `terminer`, `moyenne`) qui renvoient une erreur
  rendent le code clair et testable.
- `json.MarshalIndent` et `json.Unmarshal` enregistrent et relisent des structures aux
  champs exportés.
- `encoding/csv` lit un tableau ; `Printf` avec une largeur ou `tabwriter` l'affichent
  bien aligné.

Suite : entraînez-vous avec [les exercices corrigés](exercices.md). Comparez avec
[les projets en Ruby](../ruby/projets.md) et [en Rust](../rust/projets.md).
