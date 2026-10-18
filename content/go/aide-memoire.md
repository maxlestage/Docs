# Aide-mémoire

Cette page résume en un coup d'œil la syntaxe Go vue dans tout le parcours, avec des tableaux compacts et de courts programmes à lancer.

## Bases

| Élément | Exemple |
| --- | --- |
| Programme | `package main` + `func main() { … }` |
| Importer | `import "fmt"` ou `import ( "fmt"; "os" )` |
| Variable (déclaration courte) | `age := 30` |
| Variable (type explicite) | `var prix float64 = 9.5` |
| Constante | `const TauxTVA = 0.2` |
| Plusieurs à la fois | `a, b := 1, 2` ; échange : `a, b = b, a` |
| Ignorer une valeur | `_ = valeur` |
| Commentaire | `// ligne` ou `/* bloc */` |
| Conversion | `float64(n)`, `int(x)`, `string(r)` |
| Texte ↔ nombre | `strconv.Itoa(42)`, `strconv.Atoi("42")` |

| Type | Valeur zéro | Exemple |
| --- | --- | --- |
| `int`, `int64` | `0` | `42` |
| `float64` | `0` | `3.14` |
| `string` | `""` | `"Bonjour"` |
| `bool` | `false` | `true` |
| `rune` (un caractère) | `0` | `'é'` |
| pointeur, tranche, map, canal, interface, fonction | `nil` | |

```go
package main

import "fmt"

func main() {
	nom := "Alice"
	var age int
	fmt.Println("Bonjour", nom, age)
	fmt.Println(7/2, 7.0/2, 7%2)
	fmt.Println(float64(7) / 2)
}
```

```sortie
Bonjour Alice 0
3 3.5 1
3.5
```

Détails : [premiers pas](premiers-pas.md), [les nombres](nombres.md).

## Afficher avec fmt

| Fonction | Rôle |
| --- | --- |
| `fmt.Println(a, b)` | affiche, sépare par des espaces, va à la ligne |
| `fmt.Printf(format, …)` | affiche selon un format (ajoutez `\n`) |
| `fmt.Sprintf(format, …)` | renvoie le texte au lieu de l'afficher |
| `fmt.Fprintf(w, format, …)` | écrit dans un `io.Writer` |
| `fmt.Errorf(format, …)` | crée une erreur (`%w` pour en envelopper une) |

| Verbe | Affiche | Exemple → résultat |
| --- | --- | --- |
| `%v` | la valeur | `{Alice 30}` |
| `%+v` | avec les noms des champs | `{Nom:Alice Age:30}` |
| `%#v` | comme du code Go | `main.P{Nom:"Alice", Age:30}` |
| `%T` | le type | `main.P` |
| `%d` | un entier | `42` |
| `%5d`, `%-5d`, `%05d` | largeur 5, à gauche, avec des zéros | `   42`, `42   `, `00042` |
| `%f`, `%.2f` | un flottant, 2 décimales | `3.141593`, `3.14` |
| `%g` | un flottant sans zéros inutiles | `2.5` |
| `%s`, `%q` | un texte, entre guillemets | `Go`, `"Go"` |
| `%-10s` | un texte sur 10 caractères, à gauche | `Go        ` |
| `%t` | un booléen | `true` |
| `%c`, `%U` | un caractère, son code Unicode | `é`, `U+00E9` |
| `%x`, `%b` | hexadécimal, binaire | `ff`, `101` |
| `%p` | une adresse | `0xc000012345` |
| `%%` | le signe `%` | `%` |

```go
package main

import "fmt"

type P struct {
	Nom string
	Age int
}

func main() {
	p := P{"Alice", 30}
	fmt.Printf("%v | %+v | %#v | %T\n", p, p, p, p)
	fmt.Printf("[%5d] [%-5d] [%05d] [%.2f] [%q] [%t]\n", 42, 42, 42, 3.14159, "Go", true)
	s := fmt.Sprintf("%-6s|%6s|", "ab", "cd")
	fmt.Println(s)
}
```

```sortie
{Alice 30} | {Nom:Alice Age:30} | main.P{Nom:"Alice", Age:30} | main.P
[   42] [42   ] [00042] [3.14] ["Go"] [true]
ab    |    cd|
```

## Conditions et boucles

| Besoin | Syntaxe |
| --- | --- |
| Condition | `if x > 0 { … } else if … { … } else { … }` |
| Avec initialisation | `if err := f(); err != nil { … }` |
| Plusieurs cas | `switch x { case 1, 2: … default: … }` |
| Conditions en cascade | `switch { case x < 0: … }` |
| Boucle classique | `for i := 0; i < 5; i++ { … }` |
| « Tant que » | `for x < 10 { … }` |
| Infinie | `for { … }` |
| Parcourir | `for i, v := range tranche { … }` |
| De 0 à n-1 | `for i := range 5 { … }` |
| Contrôle | `break`, `continue` |
| Logique | `&&`, `\|\|`, `!` |

```go
package main

import "fmt"

func main() {
	for i := range 5 {
		if i == 1 {
			continue
		}
		switch {
		case i%2 == 0:
			fmt.Println(i, "pair")
		default:
			fmt.Println(i, "impair")
		}
	}
}
```

```sortie
0 pair
2 pair
3 impair
4 pair
```

Détails : [conditions et boucles](conditions-boucles.md).

## Fonctions

| Forme | Exemple |
| --- | --- |
| Simple | `func double(x int) int { return x * 2 }` |
| Même type partagé | `func aire(l, h float64) float64` |
| Plusieurs résultats | `func div(a, b int) (int, int)` |
| Variadique | `func somme(n ...int) int` ; appel `somme(t...)` |
| Fonction anonyme | `f := func(x int) int { return x + 1 }` |
| Différer | `defer fichier.Close()` (à la sortie de la fonction) |

```go
package main

import "fmt"

func divmod(a, b int) (int, int) {
	return a / b, a % b
}

func somme(nombres ...int) int {
	total := 0
	for _, n := range nombres {
		total += n
	}
	return total
}

func compteur() func() int {
	n := 0
	return func() int {
		n++
		return n
	}
}

func main() {
	q, r := divmod(17, 5)
	fmt.Println(q, r)
	fmt.Println(somme(1, 2, 3), somme([]int{4, 5}...))
	suivant := compteur()
	suivant()
	fmt.Println(suivant())
}
```

```sortie
3 2
6 9
2
```

Détails : [les fonctions](fonctions.md).

## Texte

| Besoin | Exemple |
| --- | --- |
| Longueur en octets / en caractères | `len(s)`, `utf8.RuneCountInString(s)` |
| Majuscules, minuscules | `strings.ToUpper(s)`, `strings.ToLower(s)` |
| Enlever les espaces autour | `strings.TrimSpace(s)` |
| Chercher | `strings.Contains`, `strings.HasPrefix`, `strings.Index` |
| Découper, recoller | `strings.Split(s, ",")`, `strings.Fields(s)`, `strings.Join(t, " ")` |
| Remplacer | `strings.ReplaceAll(s, "a", "b")` |
| Répéter | `strings.Repeat("-", 10)` |
| Construire efficacement | `var sb strings.Builder` ; `sb.WriteString(…)` |
| Parcourir les caractères | `for i, r := range s` (r est une `rune`) |
| Texte brut sur plusieurs lignes | `` `ligne 1 \n reste tel quel` `` |

```go
package main

import (
	"fmt"
	"strings"
	"unicode/utf8"
)

func main() {
	s := "  Noël à Paris  "
	t := strings.TrimSpace(s)
	fmt.Println(len(t), utf8.RuneCountInString(t))
	fmt.Println(strings.ToUpper(t), strings.Contains(t, "Paris"))
	fmt.Printf("%q\n", strings.Fields(t))
	fmt.Println(strings.Join([]string{"a", "b", "c"}, "-"))
}
```

```sortie
14 12
NOËL À PARIS true
["Noël" "à" "Paris"]
a-b-c
```

Détails : [le texte](texte.md), [les expressions régulières](regex.md).

## Tranches

| Besoin | Exemple |
| --- | --- |
| Créer | `t := []int{1, 2, 3}`, `make([]int, 0, 10)` |
| Longueur, capacité | `len(t)`, `cap(t)` |
| Lire, modifier | `t[0]`, `t[len(t)-1] = 9` |
| Ajouter | `t = append(t, 4, 5)` |
| Sous-tranche | `t[1:3]` (indices 1 et 2), `t[:2]`, `t[2:]` |
| Copier | `c := slices.Clone(t)` |
| Trier | `slices.Sort(t)`, `slices.SortFunc(t, cmp)` |
| Chercher | `slices.Contains(t, 3)`, `slices.Index(t, 3)` |
| Min, max | `slices.Min(t)`, `slices.Max(t)` |
| Tableau (taille fixe) | `var a [3]int` |

```go
package main

import (
	"fmt"
	"slices"
)

func main() {
	t := []int{5, 2, 8}
	t = append(t, 1)
	slices.Sort(t)
	fmt.Println(t, len(t), t[1:3])
	fmt.Println(slices.Contains(t, 8), slices.Index(t, 8), slices.Max(t))
}
```

```sortie
[1 2 5 8] 4 [2 5]
true 3 8
```

Détails : [les collections](collections.md).

## Maps

| Besoin | Exemple |
| --- | --- |
| Créer | `m := map[string]int{"a": 1}`, `make(map[string]int)` |
| Lire, écrire | `m["a"]`, `m["b"] = 2` |
| Tester une clé | `v, ok := m["c"]` |
| Supprimer | `delete(m, "a")` |
| Taille | `len(m)` |
| Clés triées | `slices.Sorted(maps.Keys(m))` |
| Attention | l'ordre de `range` sur une map est aléatoire ; écrire dans une map `nil` panique |

```go
package main

import (
	"fmt"
	"maps"
	"slices"
)

func main() {
	stock := map[string]int{"pommes": 3, "poires": 0}
	stock["kiwis"] = 7
	v, ok := stock["bananes"]
	fmt.Println(v, ok)
	delete(stock, "poires")
	for _, k := range slices.Sorted(maps.Keys(stock)) {
		fmt.Println(k, stock[k])
	}
}
```

```sortie
0 false
kiwis 7
pommes 3
```

Détails : [les maps](maps.md).

## Structures et méthodes

| Besoin | Exemple |
| --- | --- |
| Définir | `type Point struct { X, Y int }` |
| Créer | `p := Point{X: 1, Y: 2}`, `Point{1, 2}` |
| Champ | `p.X` |
| Méthode (lecture) | `func (p Point) Norme() float64` |
| Méthode (modifie) | `func (p *Point) Deplacer(dx int)` |
| Pointeur | `pp := &p` ; `*pp` ; `pp.X` marche directement |
| Constructeur (par habitude) | `func NewPoint(x, y int) *Point` |
| Exporté / privé | `Majuscule` / `minuscule` |
| Intégrer un type | `type Chat struct { Animal }` |

```go
package main

import "fmt"

type Compte struct {
	Titulaire string
	Solde     int
}

func (c Compte) String() string {
	return fmt.Sprintf("%s : %d €", c.Titulaire, c.Solde)
}

func (c *Compte) Deposer(montant int) {
	c.Solde += montant
}

func main() {
	c := Compte{Titulaire: "Alice"}
	c.Deposer(50)
	c.Deposer(20)
	fmt.Println(c)
}
```

```sortie
Alice : 70 €
```

Détails : [les types](types.md), [les pointeurs](pointeurs.md).

## Interfaces

| Besoin | Exemple |
| --- | --- |
| Définir | `type Forme interface { Aire() float64 }` |
| Satisfaire | avoir les méthodes, sans rien déclarer |
| N'importe quel type | `any` (alias de `interface{}`) |
| Assertion de type | `c, ok := f.(Cercle)` |
| Switch de type | `switch v := x.(type) { case int: … }` |
| Courantes | `fmt.Stringer`, `error`, `io.Reader`, `io.Writer` |

```go
package main

import "fmt"

func decrire(x any) string {
	switch v := x.(type) {
	case int:
		return fmt.Sprintf("entier %d", v)
	case string:
		return fmt.Sprintf("texte %q", v)
	default:
		return fmt.Sprintf("autre (%T)", v)
	}
}

func main() {
	for _, x := range []any{42, "Go", 3.5} {
		fmt.Println(decrire(x))
	}
}
```

```sortie
entier 42
texte "Go"
autre (float64)
```

Détails : [les interfaces](interfaces.md).

## Erreurs

| Besoin | Exemple |
| --- | --- |
| Renvoyer une erreur | `return 0, errors.New("message")` |
| Avec des valeurs | `fmt.Errorf("note %d invalide", n)` |
| Envelopper | `fmt.Errorf("lecture: %w", err)` |
| Tester | `if err != nil { return err }` |
| Erreur précise | `errors.Is(err, os.ErrNotExist)` |
| Type d'erreur | `var e *MonErreur; errors.As(err, &e)` |
| Bug grave | `panic("…")` ; `recover()` dans un `defer` |

```go
package main

import (
	"errors"
	"fmt"
	"strconv"
)

var ErrNegatif = errors.New("nombre négatif")

func lire(s string) (int, error) {
	n, err := strconv.Atoi(s)
	if err != nil {
		return 0, fmt.Errorf("lire %q: %w", s, err)
	}
	if n < 0 {
		return 0, ErrNegatif
	}
	return n, nil
}

func main() {
	for _, s := range []string{"12", "x", "-3"} {
		n, err := lire(s)
		switch {
		case errors.Is(err, ErrNegatif):
			fmt.Println(s, "->", "refusé :", err)
		case err != nil:
			fmt.Println(s, "->", err)
		default:
			fmt.Println(s, "->", n)
		}
	}
}
```

```sortie
12 -> 12
x -> lire "x": strconv.Atoi: parsing "x": invalid syntax
-3 -> refusé : nombre négatif
```

Détails : [les erreurs](erreurs.md).

## Génériques

| Besoin | Exemple |
| --- | --- |
| Fonction générique | `func Premier[T any](t []T) T` |
| Contrainte « comparable » | `func Contient[T comparable](t []T, v T) bool` |
| Contrainte « ordonné » | `func Max[T cmp.Ordered](a, b T) T` |
| Union de types | `type Nombre interface { ~int \| ~float64 }` |
| Type générique | `type Pile[T any] struct { elements []T }` |

```go
package main

import "fmt"

type Nombre interface {
	~int | ~float64
}

func Somme[T Nombre](valeurs []T) T {
	var total T
	for _, v := range valeurs {
		total += v
	}
	return total
}

type Pile[T any] struct {
	elements []T
}

func (p *Pile[T]) Empiler(v T) { p.elements = append(p.elements, v) }

func main() {
	fmt.Println(Somme([]int{1, 2, 3}), Somme([]float64{0.5, 1.25}))
	var p Pile[string]
	p.Empiler("a")
	p.Empiler("b")
	fmt.Println(p.elements)
}
```

```sortie
6 1.75
[a b]
```

Détails : [les génériques](generiques.md).

## Goroutines et canaux

| Besoin | Exemple |
| --- | --- |
| Lancer une goroutine | `go travail()` |
| Attendre un groupe | `var wg sync.WaitGroup` ; `wg.Add(1)` ; `defer wg.Done()` ; `wg.Wait()` |
| Protéger une variable | `var mu sync.Mutex` ; `mu.Lock()` ; `mu.Unlock()` |
| Créer un canal | `ch := make(chan int)`, avec tampon : `make(chan int, 10)` |
| Envoyer, recevoir | `ch <- v`, `v := <-ch`, `v, ok := <-ch` |
| Fermer, parcourir | `close(ch)` ; `for v := range ch` |
| Attendre plusieurs canaux | `select { case v := <-a: … case <-time.After(d): … }` |
| Détecter les courses | `go run -race`, `go test -race` |

```go
package main

import (
	"fmt"
	"sync"
)

func main() {
	resultats := make(chan int)
	var wg sync.WaitGroup
	for i := 1; i <= 3; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			resultats <- i * i
		}()
	}
	go func() {
		wg.Wait()
		close(resultats)
	}()

	total := 0
	for r := range resultats {
		total += r
	}
	fmt.Println("Total :", total)
}
```

```sortie
Total : 14
```

Détails : [la concurrence](concurrence.md), [les canaux](canaux.md).

## Fichiers

| Besoin | Exemple |
| --- | --- |
| Lire tout | `donnees, err := os.ReadFile("a.txt")` |
| Écrire tout | `os.WriteFile("a.txt", donnees, 0o644)` |
| Ouvrir, créer | `os.Open(nom)`, `os.Create(nom)` ; puis `defer f.Close()` |
| Ajouter à la fin | `os.OpenFile(nom, os.O_APPEND\|os.O_CREATE\|os.O_WRONLY, 0o644)` |
| Lire ligne par ligne | `sc := bufio.NewScanner(f)` ; `for sc.Scan() { sc.Text() }` |
| Fichier absent ? | `errors.Is(err, os.ErrNotExist)` |
| Chemins | `filepath.Join("dossier", "a.txt")`, `filepath.Ext(nom)` |
| Arguments du programme | `os.Args`, ou le paquet `flag` |

```go
package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	err := os.WriteFile("notes.txt", []byte("Alice 15\nBob 9\n"), 0o644)
	if err != nil {
		fmt.Println(err)
		return
	}
	f, err := os.Open("notes.txt")
	if err != nil {
		fmt.Println(err)
		return
	}
	defer f.Close()
	sc := bufio.NewScanner(f)
	for n := 1; sc.Scan(); n++ {
		fmt.Printf("%d: %s\n", n, sc.Text())
	}
	_, err = os.ReadFile("absent.txt")
	fmt.Println(err)
}
```

```sortie
1: Alice 15
2: Bob 9
open absent.txt: no such file or directory
```

Détails : [les fichiers](fichiers.md), [la ligne de commande](ligne-de-commande.md).

## JSON

| Besoin | Exemple |
| --- | --- |
| Structure → JSON | `json.Marshal(v)`, `json.MarshalIndent(v, "", "  ")` |
| JSON → structure | `json.Unmarshal(donnees, &v)` |
| Nom du champ | `` Titre string `json:"titre"` `` |
| Omettre si vide | `` `json:"note,omitempty"` `` |
| Ignorer un champ | `` `json:"-"` `` |
| Règle | seuls les champs **exportés** (majuscule) sont lus et écrits |

```go
package main

import (
	"encoding/json"
	"fmt"
)

type Tache struct {
	Titre string   `json:"titre"`
	Faite bool     `json:"faite"`
	Tags  []string `json:"tags,omitempty"`
}

func main() {
	donnees, _ := json.Marshal(Tache{Titre: "Pain"})
	fmt.Println(string(donnees))

	var t Tache
	err := json.Unmarshal([]byte(`{"titre":"Go","faite":true,"tags":["étude"]}`), &t)
	fmt.Printf("%+v %v\n", t, err)
}
```

```sortie
{"titre":"Pain","faite":false}
{Titre:Go Faite:true Tags:[étude]} <nil>
```

Détails : [les formats](formats.md).

## Dates et durées

Go décrit les formats de date avec une **date modèle** : le lundi 2 janvier 2006, à
15:04:05. Écrivez cette date comme vous voulez voir la vôtre.

| Élément | Code du modèle | Exemple |
| --- | --- | --- |
| Année | `2006` | `2026` |
| Mois (nombre, nom) | `01`, `Jan`, `January` | `03`, `Mar` |
| Jour | `02` | `14` |
| Heure (24 h), minutes, secondes | `15`, `04`, `05` | `09`, `30`, `00` |
| Jour de la semaine | `Mon`, `Monday` | `Sat` |
| Format ISO tout prêt | `time.RFC3339` | `2026-03-14T09:30:00Z` |

| Besoin | Exemple |
| --- | --- |
| Maintenant | `time.Now()` |
| Date précise | `time.Date(2026, 3, 14, 9, 30, 0, 0, time.UTC)` |
| Formater, lire | `t.Format("02/01/2006")`, `time.Parse("02/01/2006", s)` |
| Durées | `90 * time.Minute`, `t.Add(d)`, `t2.Sub(t1)` |
| Ajouter des jours | `t.AddDate(0, 0, 7)` |
| Attendre | `time.Sleep(500 * time.Millisecond)` |

```go
package main

import (
	"fmt"
	"time"
)

func main() {
	t := time.Date(2026, 3, 14, 9, 30, 0, 0, time.UTC)
	fmt.Println(t.Format("02/01/2006 15:04"))
	fmt.Println(t.Format(time.RFC3339), t.Weekday())
	fin := t.Add(90 * time.Minute)
	fmt.Println(fin.Format("15h04"), fin.Sub(t))
	d, err := time.Parse("02/01/2006", "25/12/2026")
	fmt.Println(d.Sub(t).Hours()/24, err)
}
```

```sortie
14/03/2026 09:30
2026-03-14T09:30:00Z Saturday
11h00 1h30m0s
285.6041666666667 <nil>
```

Détails : [les dates](dates.md).

## Tests

| Élément | Règle |
| --- | --- |
| Fichier | `xxx_test.go`, dans le même dossier que le code |
| Fonction | `func TestNom(t *testing.T)` |
| Signaler un échec | `t.Errorf("got %v, want %v", got, want)` ; `t.Fatalf` arrête le test |
| Sous-tests | `t.Run("cas", func(t *testing.T) { … })` |
| Lancer | `go test ./...`, `go test -v`, `go test -run TestNom` |
| Couverture, courses | `go test -cover`, `go test -race` |
| Mesurer | `func BenchmarkNom(b *testing.B)` ; `go test -bench .` |

```go norun
// calcul_test.go
package calcul

import "testing"

func TestDouble(t *testing.T) {
	cas := []struct {
		entree, attendu int
	}{
		{2, 4},
		{0, 0},
		{-3, -6},
	}
	for _, c := range cas {
		if got := Double(c.entree); got != c.attendu {
			t.Errorf("Double(%d) = %d, attendu %d", c.entree, got, c.attendu)
		}
	}
}
```

Détails : [les tests](tests.md).

## Projet et outils

| Commande | Rôle |
| --- | --- |
| `go mod init example.com/app` | créer un module (`go.mod`) |
| `go run .` / `go run main.go` | compiler et lancer |
| `go build -o app ./cmd/app` | créer un exécutable |
| `go install ./cmd/app` | installer dans `~/go/bin` |
| `GOOS=windows GOARCH=amd64 go build` | compiler pour un autre système |
| `go get module@v1.2.3` | ajouter une dépendance |
| `go mod tidy` | ajouter ce qui manque, retirer l'inutile |
| `go fmt ./...` / `gofmt -w .` | mettre en forme |
| `go vet ./...` | repérer le code suspect |
| `go doc strings.Split` | lire la documentation |
| `dlv debug` | lancer le débogueur Delve |

| Dossier | Contenu |
| --- | --- |
| `cmd/<nom>/` | un programme (`package main`) |
| `internal/` | des paquets privés au module |
| `*_test.go` | les tests, à côté du code |

Détails : [déboguer](debogage.md), [bien écrire du Go](style.md),
[organiser un projet](projet.md), [les modules](modules.md).
