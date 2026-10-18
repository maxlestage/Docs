# Exercices corrigés

Cette page propose 12 exercices corrigés, du plus facile au plus difficile, pour pratiquer tout le parcours Go.

## Comment utiliser cette page

Chaque exercice a un niveau :

- ★☆☆ : facile ;
- ★★☆ : moyen ;
- ★★★ : plus dur.

Lisez l'énoncé, puis écrivez votre solution dans un fichier `main.go` et lancez-la avec
`go run main.go`. Ouvrez la solution seulement après avoir essayé.

> **Astuce :** il existe souvent plusieurs bonnes solutions. Si votre programme affiche
> le bon résultat, c'est gagné ! Pensez aussi à lancer `go vet main.go` : il repère les
> petites fautes.

## Les bases

### Exercice 1 : Se présenter

Niveau : ★☆☆

Créez deux variables, `prenom` et `age`. Affichez ensuite la phrase
`Je m'appelle Alice et j'ai 30 ans.`, avec vos variables.

> **Indice :** `fmt.Printf` avec `%s` pour un texte et `%d` pour un entier (voir
> [les premiers pas](premiers-pas.md)). N'oubliez pas `\n` à la fin.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	prenom := "Alice"
> 	age := 30
> 	fmt.Printf("Je m'appelle %s et j'ai %d ans.\n", prenom, age)
> }
> ```
>
> ```sortie
> Je m'appelle Alice et j'ai 30 ans.
> ```

### Exercice 2 : Pair ou impair

Niveau : ★☆☆

Pour chaque nombre de la liste `7, 10, 0`, affichez s'il est pair ou impair :
`7 est impair`, `10 est pair`, `0 est pair`.

> **Indice :** `n%2 == 0` est vrai quand `n` est pair (voir
> [les nombres](nombres.md)).

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	for _, n := range []int{7, 10, 0} {
> 		if n%2 == 0 {
> 			fmt.Println(n, "est pair")
> 		} else {
> 			fmt.Println(n, "est impair")
> 		}
> 	}
> }
> ```
>
> ```sortie
> 7 est impair
> 10 est pair
> 0 est pair
> ```

### Exercice 3 : FizzBuzz

Niveau : ★☆☆

Affichez les nombres de 1 à 15, mais remplacez les multiples de 3 par `Fizz`, ceux de 5
par `Buzz`, et ceux de 3 et 5 à la fois par `FizzBuzz`. Affichez tout sur une ligne,
séparé par des espaces.

> **Indice :** testez d'abord le cas « multiple de 15 », sinon il ne sera jamais
> atteint. Un `switch` sans valeur se lit bien (voir
> [conditions et boucles](conditions-boucles.md)).

> **Solution**
>
> ```go
> package main
>
> import (
> 	"fmt"
> 	"strconv"
> 	"strings"
> )
>
> func main() {
> 	var mots []string
> 	for i := 1; i <= 15; i++ {
> 		switch {
> 		case i%15 == 0:
> 			mots = append(mots, "FizzBuzz")
> 		case i%3 == 0:
> 			mots = append(mots, "Fizz")
> 		case i%5 == 0:
> 			mots = append(mots, "Buzz")
> 		default:
> 			mots = append(mots, strconv.Itoa(i))
> 		}
> 	}
> 	fmt.Println(strings.Join(mots, " "))
> }
> ```
>
> ```sortie
> 1 2 Fizz 4 Buzz Fizz 7 8 Fizz Buzz 11 Fizz 13 14 FizzBuzz
> ```

### Exercice 4 : Aire et périmètre

Niveau : ★☆☆

Écrivez une fonction `rectangle(largeur, hauteur float64)` qui renvoie **deux**
valeurs : l'aire et le périmètre. Affichez-les pour un rectangle de 4 sur 2,5.

> **Indice :** une fonction Go peut renvoyer plusieurs résultats :
> `func f() (float64, float64)` (voir [les fonctions](fonctions.md)).

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func rectangle(largeur, hauteur float64) (float64, float64) {
> 	aire := largeur * hauteur
> 	perimetre := 2 * (largeur + hauteur)
> 	return aire, perimetre
> }
>
> func main() {
> 	aire, perimetre := rectangle(4, 2.5)
> 	fmt.Println("Aire :", aire)
> 	fmt.Println("Périmètre :", perimetre)
> }
> ```
>
> ```sortie
> Aire : 10
> Périmètre : 13
> ```

## Texte et collections

### Exercice 5 : Moyenne, minimum, maximum

Niveau : ★★☆

Avec la tranche `notes := []int{12, 15, 9, 14}`, affichez la moyenne (avec une
décimale), la plus petite et la plus grande note.

> **Indice :** convertissez en `float64` avant de diviser. Le paquet `slices` fournit
> `slices.Min` et `slices.Max` (voir [les collections](collections.md)).

> **Solution**
>
> ```go
> package main
>
> import (
> 	"fmt"
> 	"slices"
> )
>
> func main() {
> 	notes := []int{12, 15, 9, 14}
> 	total := 0
> 	for _, n := range notes {
> 		total += n
> 	}
> 	moyenne := float64(total) / float64(len(notes))
> 	fmt.Printf("Moyenne : %.1f\n", moyenne)
> 	fmt.Println("Minimum :", slices.Min(notes))
> 	fmt.Println("Maximum :", slices.Max(notes))
> }
> ```
>
> ```sortie
> Moyenne : 12.5
> Minimum : 9
> Maximum : 15
> ```

### Exercice 6 : Compter les mots

Niveau : ★★☆

Comptez combien de fois chaque mot apparaît dans la phrase
`"le chat et le chien et le lapin"`. Affichez les mots **par ordre alphabétique**,
avec leur nombre.

> **Indice :** `strings.Fields` découpe aux espaces ; une `map[string]int` compte. L'ordre
> de parcours d'une map change d'une exécution à l'autre : triez les clés (voir
> [les maps](maps.md)).

> **Solution**
>
> ```go
> package main
>
> import (
> 	"fmt"
> 	"maps"
> 	"slices"
> 	"strings"
> )
>
> func main() {
> 	phrase := "le chat et le chien et le lapin"
> 	compte := map[string]int{}
> 	for _, mot := range strings.Fields(phrase) {
> 		compte[mot]++
> 	}
> 	for _, mot := range slices.Sorted(maps.Keys(compte)) {
> 		fmt.Printf("%s : %d\n", mot, compte[mot])
> 	}
> }
> ```
>
> ```sortie
> chat : 1
> chien : 1
> et : 2
> lapin : 1
> le : 3
> ```

### Exercice 7 : Inverser un texte

Niveau : ★★☆

Écrivez une fonction `inverser(s string) string` qui renvoie le texte à l'envers.
Elle doit fonctionner avec les accents : `"Noël"` donne `"lëoN"`.

> **Indice :** une chaîne Go est une suite d'**octets**, et `ë` en occupe deux.
> Convertissez en `[]rune` pour travailler caractère par caractère (voir
> [le texte](texte.md)).

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func inverser(s string) string {
> 	r := []rune(s)
> 	for i, j := 0, len(r)-1; i < j; i, j = i+1, j-1 {
> 		r[i], r[j] = r[j], r[i]
> 	}
> 	return string(r)
> }
>
> func main() {
> 	fmt.Println(inverser("Bonjour"))
> 	fmt.Println(inverser("Noël"))
> }
> ```
>
> ```sortie
> ruojnoB
> lëoN
> ```

### Exercice 8 : Palindrome

Niveau : ★★☆

Un palindrome se lit pareil dans les deux sens, en ignorant les espaces et les
majuscules. Écrivez `estPalindrome(s string) bool` et testez-la sur `"Kayak"`,
`"Esope reste ici et se repose"` et `"Bonjour"`.

> **Indice :** mettez le texte en minuscules avec `strings.ToLower`, enlevez les espaces
> avec `strings.ReplaceAll`, puis comparez les runes deux à deux.

> **Solution**
>
> ```go
> package main
>
> import (
> 	"fmt"
> 	"strings"
> )
>
> func estPalindrome(s string) bool {
> 	r := []rune(strings.ReplaceAll(strings.ToLower(s), " ", ""))
> 	for i := 0; i < len(r)/2; i++ {
> 		if r[i] != r[len(r)-1-i] {
> 			return false
> 		}
> 	}
> 	return true
> }
>
> func main() {
> 	for _, s := range []string{"Kayak", "Esope reste ici et se repose", "Bonjour"} {
> 		fmt.Printf("%q : %t\n", s, estPalindrome(s))
> 	}
> }
> ```
>
> ```sortie
> "Kayak" : true
> "Esope reste ici et se repose" : true
> "Bonjour" : false
> ```

## Types et erreurs

### Exercice 9 : Des formes et une interface

Niveau : ★★☆

Créez deux types, `Rectangle` (largeur, hauteur) et `Cercle` (rayon), qui ont chacun
une méthode `Aire() float64`. Définissez une interface `Forme`, puis affichez l'aire de
chaque forme d'une tranche `[]Forme`, avec deux décimales, et l'aire totale.

> **Indice :** un type satisfait une interface dès qu'il a ses méthodes, sans rien
> déclarer (voir [les interfaces](interfaces.md)). `math.Pi` donne π.

> **Solution**
>
> ```go
> package main
>
> import (
> 	"fmt"
> 	"math"
> )
>
> type Forme interface {
> 	Aire() float64
> }
>
> type Rectangle struct {
> 	Largeur, Hauteur float64
> }
>
> func (r Rectangle) Aire() float64 { return r.Largeur * r.Hauteur }
>
> type Cercle struct {
> 	Rayon float64
> }
>
> func (c Cercle) Aire() float64 { return math.Pi * c.Rayon * c.Rayon }
>
> func main() {
> 	formes := []Forme{Rectangle{4, 3}, Cercle{1}, Rectangle{2, 2}}
> 	total := 0.0
> 	for _, f := range formes {
> 		fmt.Printf("%T : %.2f\n", f, f.Aire())
> 		total += f.Aire()
> 	}
> 	fmt.Printf("Total : %.2f\n", total)
> }
> ```
>
> ```sortie
> main.Rectangle : 12.00
> main.Cercle : 3.14
> main.Rectangle : 4.00
> Total : 19.14
> ```

### Exercice 10 : Une division sûre

Niveau : ★★☆

Écrivez `diviser(a, b float64) (float64, error)`. Si `b` vaut 0, elle renvoie l'erreur
`ErrDivisionParZero`, déclarée une fois pour toutes. Dans `main`, essayez `10 / 4` et
`1 / 0`, et reconnaissez cette erreur précise avec `errors.Is`.

> **Indice :** `var ErrDivisionParZero = errors.New("division par zéro")` crée une erreur
> sentinelle (voir [les erreurs](erreurs.md)).

> **Solution**
>
> ```go
> package main
>
> import (
> 	"errors"
> 	"fmt"
> )
>
> var ErrDivisionParZero = errors.New("division par zéro")
>
> func diviser(a, b float64) (float64, error) {
> 	if b == 0 {
> 		return 0, ErrDivisionParZero
> 	}
> 	return a / b, nil
> }
>
> func main() {
> 	paires := [][2]float64{{10, 4}, {1, 0}}
> 	for _, p := range paires {
> 		resultat, err := diviser(p[0], p[1])
> 		if errors.Is(err, ErrDivisionParZero) {
> 			fmt.Printf("%g / %g : impossible (%v)\n", p[0], p[1], err)
> 			continue
> 		}
> 		fmt.Printf("%g / %g = %g\n", p[0], p[1], resultat)
> 	}
> }
> ```
>
> ```sortie
> 10 / 4 = 2.5
> 1 / 0 : impossible (division par zéro)
> ```

## Pour aller plus loin

### Exercice 11 : Un filtre générique

Niveau : ★★★

Écrivez **une seule** fonction générique `Filtrer` qui garde les éléments d'une tranche
pour lesquels une fonction renvoie `true`. Utilisez-la pour garder les nombres pairs
d'une tranche d'entiers, puis les mots de plus de 4 lettres d'une tranche de chaînes.

> **Indice :** `func Filtrer[T any](s []T, garder func(T) bool) []T` (voir
> [les génériques](generiques.md)).

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func Filtrer[T any](s []T, garder func(T) bool) []T {
> 	var resultat []T
> 	for _, v := range s {
> 		if garder(v) {
> 			resultat = append(resultat, v)
> 		}
> 	}
> 	return resultat
> }
>
> func main() {
> 	nombres := []int{1, 2, 3, 4, 5, 6}
> 	pairs := Filtrer(nombres, func(n int) bool { return n%2 == 0 })
> 	fmt.Println(pairs)
>
> 	mots := []string{"go", "gopher", "code", "canal"}
> 	longs := Filtrer(mots, func(m string) bool { return len(m) > 4 })
> 	fmt.Println(longs)
> }
> ```
>
> ```sortie
> [2 4 6]
> [gopher canal]
> ```

### Exercice 12 : Une somme en parallèle

Niveau : ★★★

Découpez les nombres de 1 à 100 en 4 morceaux de 25. Lancez une goroutine par morceau :
chacune calcule la somme de son morceau et l'envoie dans un canal. `main` reçoit les 4
sommes partielles et affiche le total.

> **Indice :** un canal `make(chan int)` ; chaque goroutine fait `ch <- somme`, et `main`
> fait `<-ch` quatre fois (voir [les canaux](canaux.md)). L'ordre d'arrivée change d'une
> exécution à l'autre, mais pas le total.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func somme(nombres []int, ch chan<- int) {
> 	total := 0
> 	for _, n := range nombres {
> 		total += n
> 	}
> 	ch <- total
> }
>
> func main() {
> 	var nombres []int
> 	for i := 1; i <= 100; i++ {
> 		nombres = append(nombres, i)
> 	}
>
> 	ch := make(chan int)
> 	for debut := 0; debut < len(nombres); debut += 25 {
> 		go somme(nombres[debut:debut+25], ch)
> 	}
>
> 	total := 0
> 	for range 4 {
> 		total += <-ch
> 	}
> 	fmt.Println("Total :", total)
> }
> ```
>
> ```sortie
> Total : 5050
> ```
>
> `chan<- int` indique que la fonction peut seulement **envoyer** dans le canal. Le
> total ne dépend pas de l'ordre d'arrivée des sommes partielles : le résultat est
> toujours 5050.

## En résumé

- Commencez par les exercices ★☆☆, puis montez en difficulté.
- Essayez toujours seul avant d'ouvrir la solution.
- Les exercices 1 à 4 reprennent les bases : variables, conditions, boucles, fonctions.
- Les exercices 5 à 8 travaillent le texte, les tranches et les maps.
- Les exercices 9 et 10 utilisent les interfaces et les erreurs ; 11 et 12, les
  génériques et les goroutines.

Suite : gardez [l'aide-mémoire](aide-memoire.md) sous la main. Comparez avec
[les exercices en Ruby](../ruby/exercices.md) et [en Rust](../rust/exercices.md).
