# Structures et méthodes

Cette page montre comment créer vos propres types en Go avec `struct`, leur donner des méthodes et les combiner par l'embarquement.

> **Objectifs :**
>
> - vous saurez définir une structure et créer des valeurs de ce type ;
> - vous saurez lire et modifier les champs d'une structure ;
> - vous saurez écrire des méthodes, avec un récepteur valeur ou pointeur ;
> - vous saurez écrire un constructeur `NewX` ;
> - vous saurez réutiliser une structure dans une autre par l'embarquement.

## Définir une structure

Une [structure](../debuter/glossaire.md#structure) regroupe plusieurs valeurs sous un
seul nom. Pensez à une fiche d'inscription : elle a des cases (nom, âge, ville), et
chaque personne remplit sa propre fiche. La fiche vierge est le **type**, chaque fiche
remplie est une **valeur** de ce type.

En Go, on écrit `type`, le nom, puis `struct` et la liste des **champs** entre
accolades. Chaque champ a un nom et un [type](../debuter/glossaire.md#type).

```go
package main

import "fmt"

type Personne struct {
	Nom   string
	Age   int
	Ville string
}

func main() {
	alice := Personne{Nom: "Alice", Age: 30, Ville: "Lyon"}
	fmt.Println(alice)
}
```

```sortie
{Alice 30 Lyon}
```

La structure se déclare **en dehors** de `main`, au niveau du fichier. Le nom du type
commence souvent par une majuscule : vous verrez pourquoi dans
[packages et modules](modules.md).

## Créer des valeurs

Il existe plusieurs façons d'écrire une valeur de structure, appelée un **littéral**.

- Avec les noms des champs : `Personne{Nom: "Alice", Age: 30}`. C'est la forme
  conseillée : l'ordre ne compte pas et le code se lit tout seul.
- Sans les noms : `Personne{"Bob", 25, "Paris"}`. Il faut alors donner **tous** les
  champs, dans l'ordre.
- Vide : `Personne{}`. Chaque champ reçoit sa **valeur zéro** : `""` pour un texte,
  `0` pour un nombre, `false` pour un booléen.

```go
package main

import "fmt"

type Personne struct {
	Nom   string
	Age   int
	Ville string
}

func main() {
	a := Personne{Nom: "Alice", Age: 30}
	b := Personne{"Bob", 25, "Paris"}
	var c Personne

	fmt.Println(a)
	fmt.Println(b)
	fmt.Println(c)
	fmt.Printf("%+v\n", a)
	fmt.Printf("%#v\n", b)
}
```

```sortie
{Alice 30 }
{Bob 25 Paris}
{ 0 }
{Nom:Alice Age:30 Ville:}
main.Personne{Nom:"Bob", Age:25, Ville:"Paris"}
```

> **Astuce :** pour afficher une structure pendant que vous cherchez un bug, utilisez
> `%+v` : il montre le nom de chaque champ. `%#v` montre en plus le nom du type.

## Lire et modifier les champs

Un point `.` donne accès à un champ : `alice.Age`. Vous pouvez le lire et lui donner une
nouvelle valeur.

```go
package main

import "fmt"

type Compte struct {
	Titulaire string
	Solde     float64
}

func main() {
	c := Compte{Titulaire: "Alice", Solde: 100}
	c.Solde = c.Solde + 50
	fmt.Println(c.Titulaire, "a", c.Solde, "euros")
}
```

```sortie
Alice a 150 euros
```

### Une copie est indépendante

Quand vous affectez une structure à une autre variable, Go **copie** toutes les cases.
Modifier la copie ne touche pas l'original : c'est comme photocopier la fiche.

```go
package main

import "fmt"

type Point struct {
	X, Y int
}

func main() {
	a := Point{X: 1, Y: 2}
	b := a
	b.X = 100
	fmt.Println(a, b)
}
```

```sortie
{1 2} {100 2}
```

`X, Y int` déclare deux champs du même type sur une seule ligne.

## Comparer deux structures

Deux structures du même type se comparent avec `==` : elles sont égales si **tous**
leurs champs sont égaux. Cela marche tant que les champs sont eux-mêmes comparables
(nombres, textes, booléens… mais pas les slices ni les maps).

```go
package main

import "fmt"

type Point struct {
	X, Y int
}

func main() {
	a := Point{1, 2}
	b := Point{1, 2}
	c := Point{2, 1}
	fmt.Println(a == b)
	fmt.Println(a == c)
}
```

```sortie
true
false
```

## Les méthodes

Une [méthode](../debuter/glossaire.md#methode) est une fonction attachée à un type. On
l'écrit comme une fonction, avec en plus un **récepteur** entre parenthèses avant le
nom : `func (r Rectangle) Aire()`. Le récepteur, c'est la valeur sur laquelle on appelle
la méthode. Par habitude, on lui donne un nom court : la première lettre du type.

```go
package main

import "fmt"

type Rectangle struct {
	Largeur, Hauteur float64
}

func (r Rectangle) Aire() float64 {
	return r.Largeur * r.Hauteur
}

func (r Rectangle) Perimetre() float64 {
	return 2 * (r.Largeur + r.Hauteur)
}

func main() {
	r := Rectangle{Largeur: 3, Hauteur: 4}
	fmt.Println("Aire :", r.Aire())
	fmt.Println("Périmètre :", r.Perimetre())
}
```

```sortie
Aire : 12
Périmètre : 14
```

Go n'a pas de classes : une structure et ses méthodes jouent le même rôle. Comparez
avec [les classes en Ruby](../ruby/types.md).

## Récepteur valeur ou pointeur

Avec `func (r Rectangle)`, la méthode reçoit une **copie** de la valeur. Elle peut lire
les champs, mais une modification se perd à la fin de la méthode.

Pour modifier la valeur d'origine, écrivez une étoile : `func (c *Compteur)`. La méthode
reçoit alors l'**adresse** de la valeur, un [pointeur](../debuter/glossaire.md#pointeur),
et travaille sur l'original. Le chapitre [les pointeurs](pointeurs.md) explique cela en
détail.

```go
package main

import "fmt"

type Compteur struct {
	Valeur int
}

func (c Compteur) IncrementerCopie() {
	c.Valeur++
}

func (c *Compteur) Incrementer() {
	c.Valeur++
}

func main() {
	c := Compteur{}
	c.IncrementerCopie()
	fmt.Println("Après la copie :", c.Valeur)
	c.Incrementer()
	c.Incrementer()
	fmt.Println("Après le pointeur :", c.Valeur)
}
```

```sortie
Après la copie : 0
Après le pointeur : 2
```

Vous n'avez pas besoin d'écrire `(&c).Incrementer()` : Go prend l'adresse tout seul
quand vous écrivez `c.Incrementer()`.

> **À retenir :** une méthode qui **modifie** la structure prend un récepteur pointeur
> `*T`. Si une seule méthode du type a un récepteur pointeur, donnez-en un à toutes :
> c'est plus cohérent.

## Les constructeurs NewX

Go n'a pas de mot-clé pour construire un objet. L'habitude est d'écrire une fonction
`NewNomDuType` qui prépare une valeur valide et la renvoie. On l'appelle un
**constructeur**. Elle renvoie souvent un pointeur.

```go
package main

import (
	"fmt"
	"strings"
)

type Joueur struct {
	Nom    string
	Vies   int
	Points int
}

func NewJoueur(nom string) *Joueur {
	return &Joueur{
		Nom:  strings.TrimSpace(nom),
		Vies: 3,
	}
}

func main() {
	j := NewJoueur("  Zoé ")
	j.Points += 10
	fmt.Printf("%+v\n", *j)
}
```

```sortie
{Nom:Zoé Vies:3 Points:10}
```

Le constructeur garantit qu'un joueur commence toujours avec 3 vies et un nom propre.
Remarquez la virgule après le dernier champ : quand un littéral s'étale sur plusieurs
lignes, Go l'exige.

## L'embarquement

Go n'a pas d'[héritage](../debuter/glossaire.md#heritage). À la place, une structure
peut en **embarquer** une autre : on écrit le type seul, sans nom de champ. Les champs
et les méthodes du type embarqué deviennent accessibles directement.

C'est comme une voiture qui contient un moteur : on dit « la voiture démarre », même si
c'est le moteur qui fait le travail.

```go
package main

import "fmt"

type Animal struct {
	Nom string
}

func (a Animal) SePresenter() string {
	return "Je m'appelle " + a.Nom
}

type Chien struct {
	Animal
	Race string
}

func main() {
	rex := Chien{Animal: Animal{Nom: "Rex"}, Race: "labrador"}
	fmt.Println(rex.Nom)
	fmt.Println(rex.SePresenter())
	fmt.Println(rex.Animal.Nom)
}
```

```sortie
Rex
Je m'appelle Rex
Rex
```

`rex.Nom` est un raccourci pour `rex.Animal.Nom`. Le champ embarqué porte le nom de son
type : `Animal`.

## Les étiquettes de champ

Après le type d'un champ, vous pouvez ajouter une **étiquette** (*tag*) entre accents
graves. Go l'ignore, mais certains packages la lisent. Le plus courant est
`encoding/json`, qui s'en sert pour choisir le nom du champ dans le
[JSON](../debuter/glossaire.md#json) (voir [les formats](formats.md)).

```go
package main

import (
	"encoding/json"
	"fmt"
)

type Livre struct {
	Titre  string `json:"titre"`
	Pages  int    `json:"pages"`
	Secret string `json:"-"`
}

func main() {
	l := Livre{Titre: "Dune", Pages: 600, Secret: "caché"}
	donnees, _ := json.Marshal(l)
	fmt.Println(string(donnees))
}
```

```sortie
{"titre":"Dune","pages":600}
```

`json:"-"` veut dire : ne pas écrire ce champ.

## Erreurs fréquentes

### Mélanger champs nommés et non nommés

```go norun
package main

import "fmt"

type Point struct {
	X, Y int
}

func main() {
	p := Point{X: 1, 2}
	fmt.Println(p)
}
```

```text
# command-line-arguments
./main.go:10:19: mixture of field:value and value elements in struct literal
```

Un littéral utilise soit les noms pour tous les champs, soit pour aucun. Écrivez
`Point{X: 1, Y: 2}`.

### Oublier des champs sans les noms

```go norun
package main

import "fmt"

type Point struct {
	X, Y int
}

func main() {
	p := Point{1}
	fmt.Println(p)
}
```

```text
# command-line-arguments
./main.go:10:14: too few values in struct literal of type Point
```

Sans les noms, il faut donner **tous** les champs. Écrivez `Point{X: 1}` : le champ
oublié vaut alors `0`.

### Se tromper de majuscule

```go norun
package main

import "fmt"

type Personne struct {
	nom string
}

func main() {
	p := Personne{nom: "Alice"}
	fmt.Println(p.Nom)
}
```

```text
# command-line-arguments
./main.go:11:16: p.Nom undefined (type Personne has no field or method Nom, but does have field nom)
```

Go distingue majuscules et minuscules : `Nom` et `nom` sont deux noms différents. Le
message vous donne même la bonne orthographe : `p.nom`.

### Modifier une copie dans une méthode

```go norun
func (c Compteur) Incrementer() {
	c.Valeur++
}
```

```text
0
```

Aucun message d'erreur ici, mais le compteur reste à `0` : la méthode a modifié une
copie. Écrivez `func (c *Compteur) Incrementer()`.

## Testez-vous

**Question 1 :** qu'affiche `fmt.Printf("%+v\n", Point{X: 3})` si `Point` a deux champs
`X` et `Y` de type `int` ?

> **Réponse**
>
> `{X:3 Y:0}`. Le champ `Y` n'est pas donné : il prend la valeur zéro des entiers.

**Question 2 :** quand faut-il écrire `func (c *Compte)` plutôt que `func (c Compte)` ?

> **Réponse**
>
> Quand la méthode doit modifier le compte d'origine. Avec `(c Compte)`, la méthode
> travaille sur une copie et les changements sont perdus.

**Question 3 :** écrivez une structure `Cercle` avec un champ `Rayon` et une méthode
`Diametre()` qui renvoie le double du rayon.

> **Réponse**
>
> ```go
> package main
>
> import "fmt"
>
> type Cercle struct {
> 	Rayon float64
> }
>
> func (c Cercle) Diametre() float64 {
> 	return 2 * c.Rayon
> }
>
> func main() {
> 	c := Cercle{Rayon: 2.5}
> 	fmt.Println(c.Diametre())
> }
> ```
>
> ```sortie
> 5
> ```

## En résumé

- `type Nom struct { ... }` définit une structure ; ses champs ont chacun un type.
- Un littéral `T{Champ: valeur}` crée une valeur ; les champs oubliés valent zéro.
- `%+v` affiche une structure avec le nom des champs ; `==` compare tous les champs.
- Une méthode a un récepteur ; prenez `*T` pour modifier la valeur d'origine.
- Une fonction `NewT` sert de constructeur ; l'embarquement remplace l'héritage.

Suite : [les interfaces](interfaces.md).
