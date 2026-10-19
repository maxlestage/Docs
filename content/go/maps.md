# Les maps

Cette page présente la map, la structure de Go qui associe des clés à des valeurs, comme un dictionnaire associe des mots à leurs définitions.

> **Objectifs :**
>
> - vous saurez créer une map avec `make` ou avec des valeurs de départ ;
> - vous saurez lire, ajouter, modifier et supprimer une entrée ;
> - vous saurez tester si une clé existe avec la forme « virgule ok » ;
> - vous saurez parcourir une map dans un ordre fixe, en triant les clés ;
> - vous saurez ranger des slices ou des structures dans une map.

## Qu'est-ce qu'une map ?

Une slice range des valeurs par **position** : 0, 1, 2… Une map les range par **clé** :
un nom que vous choisissez. C'est un [dictionnaire](../debuter/glossaire.md#dictionnaire) :
on cherche un mot (la clé) pour trouver sa définition (la valeur).

Le type d'une map s'écrit `map[TypeDeClé]TypeDeValeur`. Par exemple, `map[string]int`
associe des textes à des entiers.

```go
package main

import "fmt"

func main() {
	ages := map[string]int{
		"Alice": 12,
		"Bob":   9,
	}
	fmt.Println(ages["Alice"])
	fmt.Println(len(ages))
}
```

```sortie
12
2
```

- Chaque entrée s'écrit `clé: valeur`, suivie d'une virgule (même la dernière).
- `ages["Alice"]` lit la valeur associée à la clé `"Alice"`.
- `len` donne le nombre d'entrées.

## Créer une map vide avec make

Souvent, vous ne savez pas à l'avance ce que la map contiendra : vous la remplissez
au fil du programme. C'est comme ouvrir un tiroir vide pour y ranger des choses plus
tard.

`make(map[string]int)` crée une map vide, prête à recevoir des entrées.

```go
package main

import "fmt"

func main() {
	stock := make(map[string]int)
	stock["pommes"] = 5
	stock["poires"] = 3
	fmt.Println(stock["pommes"], len(stock))
}
```

```sortie
5 2
```

Ligne par ligne :

- `make(map[string]int)` crée une map vide : elle n'a encore aucune entrée ;
- `stock["pommes"] = 5` ajoute une entrée : la clé `"pommes"`, avec la valeur 5 ;
- la ligne suivante ajoute la clé `"poires"`, avec la valeur 3 ;
- `Println` affiche la valeur de `"pommes"` (5) puis le nombre d'entrées (2).

> **Attention :** une map déclarée avec `var stock map[string]int` vaut `nil`. On peut
> la lire, mais y écrire fait planter le programme. Créez-la toujours avec `make` ou
> avec `{}`.

## Ajouter, modifier, supprimer

Une map change pendant que le programme tourne, comme la liste des prix d'une
boulangerie : un produit arrive, un prix augmente, un autre produit disparaît.

La même écriture `m[clé] = valeur` ajoute une entrée si la clé est nouvelle, et la
**remplace** si elle existe déjà. `delete(m, clé)` supprime une entrée.

```go
package main

import "fmt"

func main() {
	prix := map[string]float64{"pain": 1.2}
	prix["lait"] = 0.95
	prix["pain"] = 1.3
	fmt.Println(prix["pain"], len(prix))

	delete(prix, "lait")
	fmt.Println(len(prix))
	delete(prix, "chocolat")
	fmt.Println(len(prix))
}
```

```sortie
1.3 2
1
1
```

Ligne par ligne :

- au départ, `prix` contient une seule entrée : `"pain"` vaut 1.2 ;
- `prix["lait"] = 0.95` utilise une clé nouvelle : la map a maintenant 2 entrées ;
- `prix["pain"] = 1.3` ne crée rien : la clé existe déjà, sa valeur passe de 1.2
  à 1.3 ;
- `delete(prix, "lait")` retire le lait : il reste 1 entrée.

Supprimer une clé absente (`"chocolat"`) n'est pas une erreur : il ne se passe rien.

**À vous :** écrivez un programme qui crée une map vide `couleurs` avec `make`, y
ajoute `"rouge"` (3) et `"bleu"` (5), remplace la valeur de `"rouge"` par 4, supprime
`"bleu"`, puis affiche la valeur de `"rouge"` et le nombre d'entrées.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	couleurs := make(map[string]int)
> 	couleurs["rouge"] = 3
> 	couleurs["bleu"] = 5
> 	couleurs["rouge"] = 4
> 	delete(couleurs, "bleu")
> 	fmt.Println(couleurs["rouge"], len(couleurs))
> }
> ```
>
> ```sortie
> 4 1
> ```

## Une clé absente

Un programme demande souvent une clé qui n'est peut-être pas là : un prénom tapé par
quelqu'un, un produit qui n'est pas en stock. Il faut savoir ce que Go répond alors.

Lire une clé qui n'existe pas ne provoque pas d'erreur : Go renvoie la **valeur zéro**
du type des valeurs.

```go
package main

import "fmt"

func main() {
	notes := map[string]int{"Alice": 15, "Bob": 0}
	fmt.Println(notes["Chloé"])
	fmt.Println(notes["Bob"])
}
```

```sortie
0
0
```

Ligne par ligne :

- `notes` contient deux entrées : Alice a 15, Bob a 0 ;
- `notes["Chloé"]` cherche une clé qui n'existe pas : Go renvoie 0, la valeur zéro
  d'un `int` ;
- `notes["Bob"]` trouve la clé : sa valeur est vraiment 0.

Problème : `0` pour Chloé (absente) et `0` pour Bob (qui a vraiment eu 0) se
ressemblent. Comment les distinguer ?

### La forme « virgule ok »

Lisez la map avec **deux** variables. La seconde, un booléen, dit si la clé existe.

```go
package main

import "fmt"

func main() {
	notes := map[string]int{"Alice": 15, "Bob": 0}

	note, ok := notes["Bob"]
	fmt.Println(note, ok)

	note, ok = notes["Chloé"]
	fmt.Println(note, ok)

	if n, trouve := notes["Alice"]; trouve {
		fmt.Println("Alice a", n)
	}
}
```

```sortie
0 true
0 false
Alice a 15
```

Par habitude, on appelle cette variable `ok`. La dernière forme, avec un `if` qui crée
la variable, est très courante.

> **À retenir :** `v := m[k]` donne la valeur (ou zéro). `v, ok := m[k]` dit en plus si
> la clé existe.

**À vous :** écrivez un programme qui crée un annuaire
`map[string]string{"Alice": "06 12 34 56 78"}`, cherche `"Bob"` avec la forme
« virgule ok », puis affiche son numéro s'il existe, ou « Bob n'est pas dans
l'annuaire » sinon.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	annuaire := map[string]string{"Alice": "06 12 34 56 78"}
> 	numero, ok := annuaire["Bob"]
> 	if ok {
> 		fmt.Println("Numéro de Bob :", numero)
> 	} else {
> 		fmt.Println("Bob n'est pas dans l'annuaire")
> 	}
> }
> ```
>
> ```sortie
> Bob n'est pas dans l'annuaire
> ```

## Parcourir une map

Il faut souvent tout passer en revue : afficher tout le stock, additionner toutes les
notes. Vous ne voulez pas écrire chaque clé à la main : c'est le rôle de `range`.

`range` parcourt toutes les entrées d'une map, en donnant la clé et la valeur.

```go norun
for prenom, age := range ages {
	fmt.Println(prenom, age)
}
```

Mais attention : **l'ordre est aléatoire**. Go le change exprès d'un lancement à
l'autre, pour que personne ne compte dessus. Le même programme peut afficher Alice puis
Bob, puis Bob puis Alice au lancement suivant.

### Trier les clés avant d'afficher

Pour un affichage stable, récupérez les clés, triez-les, puis parcourez-les dans
l'ordre.

```go
package main

import (
	"fmt"
	"slices"
)

func main() {
	ages := map[string]int{"Zoé": 8, "Alice": 12, "Marc": 10}

	cles := make([]string, 0, len(ages))
	for prenom := range ages {
		cles = append(cles, prenom)
	}
	slices.Sort(cles)

	for _, prenom := range cles {
		fmt.Println(prenom, ages[prenom])
	}
}
```

```sortie
Alice 12
Marc 10
Zoé 8
```

Ligne par ligne :

- `cles` est une slice de textes vide, avec assez de place pour toutes les clés ;
- la première boucle passe sur chaque prénom de la map et l'ajoute à `cles`, dans un
  ordre quelconque ;
- `slices.Sort(cles)` range les prénoms dans l'ordre alphabétique : Alice, Marc, Zoé ;
- la seconde boucle parcourt `cles` dans cet ordre et lit l'âge de chacun dans la map.

### Plus court : maps.Keys et slices.Sorted

Depuis Go 1.23, le paquet `maps` fournit `maps.Keys`, qui donne les clés une par une.
`slices.Sorted` les ramasse dans une slice triée. Les trois étapes tiennent en une
ligne.

```go
package main

import (
	"fmt"
	"maps"
	"slices"
)

func main() {
	ages := map[string]int{"Zoé": 8, "Alice": 12, "Marc": 10}
	for _, prenom := range slices.Sorted(maps.Keys(ages)) {
		fmt.Println(prenom, ages[prenom])
	}
}
```

```sortie
Alice 12
Marc 10
Zoé 8
```

> **Astuce :** prenez l'habitude : dès que vous **affichez** une map, triez ses clés.
> Vos résultats et vos tests seront toujours les mêmes.

**À vous :** écrivez un programme qui range le stock d'un fruitier dans
`map[string]int{"poires": 3, "kiwis": 7, "pommes": 5}`, puis affiche chaque fruit et
sa quantité, dans l'ordre alphabétique.

> **Solution**
>
> ```go
> package main
>
> import (
> 	"fmt"
> 	"maps"
> 	"slices"
> )
>
> func main() {
> 	stock := map[string]int{"poires": 3, "kiwis": 7, "pommes": 5}
> 	for _, fruit := range slices.Sorted(maps.Keys(stock)) {
> 		fmt.Println(fruit, stock[fruit])
> 	}
> }
> ```
>
> ```sortie
> kiwis 7
> poires 3
> pommes 5
> ```

## Compter des mots

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

Compter est l'usage le plus classique d'une map. Comme une clé absente vaut 0,
`compte[mot]++` marche même la première fois.

```go
package main

import (
	"fmt"
	"maps"
	"slices"
	"strings"
)

func main() {
	texte := "le chat et le chien et le poisson"
	compte := make(map[string]int)
	for _, mot := range strings.Fields(texte) {
		compte[mot]++
	}

	for _, mot := range slices.Sorted(maps.Keys(compte)) {
		fmt.Printf("%-8s %d\n", mot, compte[mot])
	}
}
```

```sortie
chat     1
chien    1
et       2
le       3
poisson  1
```

Ligne par ligne :

- `strings.Fields(texte)` découpe le texte en mots, à chaque espace ;
- pour chaque mot, `compte[mot]++` ajoute 1. La première fois que « le » arrive, sa
  valeur passe de 0 à 1 ; à la fin, elle vaut 3 ;
- la seconde boucle affiche les mots triés. `%-8s` complète chaque mot avec des espaces,
  pour aligner les nombres.

## Des valeurs plus riches

Parfois, une seule valeur par clé ne suffit pas : une classe a plusieurs élèves, un
produit a un prix **et** un stock.

Une valeur de map peut être de n'importe quel type : une slice, une autre map, une
structure…

### Une map de slices

Pour ranger plusieurs valeurs sous une même clé, utilisez une slice comme valeur.
`append` sur une clé absente part de la slice vide (`nil`) : pas besoin de la créer.

```go
package main

import (
	"fmt"
	"maps"
	"slices"
)

func main() {
	eleves := []string{"Alice", "Bob", "Chloé", "David", "Emma"}
	classes := []string{"6A", "6B", "6A", "6B", "6A"}

	parClasse := make(map[string][]string)
	for i, nom := range eleves {
		parClasse[classes[i]] = append(parClasse[classes[i]], nom)
	}

	for _, classe := range slices.Sorted(maps.Keys(parClasse)) {
		fmt.Println(classe, parClasse[classe])
	}
}
```

```sortie
6A [Alice Chloé Emma]
6B [Bob David]
```

Ligne par ligne :

- `eleves` et `classes` vont ensemble : Alice est en 6A, Bob en 6B, Chloé en 6A… ;
- `parClasse` est une map vide : à chaque classe, elle associera une slice de prénoms ;
- la boucle prend chaque élève et l'ajoute (`append`) à la slice de sa classe ;
- à la fin, `"6A"` contient trois prénoms et `"6B"` en contient deux.

### Une map de structures

Une structure regroupe plusieurs champs (voir [les types](types.md)). Elle fait une
bonne valeur de map, comme une fiche dans un classeur.

```go
package main

import "fmt"

type Produit struct {
	Prix  float64
	Stock int
}

func main() {
	catalogue := map[string]Produit{
		"stylo":  {Prix: 1.5, Stock: 40},
		"cahier": {Prix: 3.2, Stock: 12},
	}
	fmt.Printf("%+v\n", catalogue["cahier"])

	p := catalogue["stylo"]
	p.Stock -= 5
	catalogue["stylo"] = p
	fmt.Println(catalogue["stylo"].Stock)
}
```

```sortie
{Prix:3.2 Stock:12}
35
```

> **Attention :** `catalogue["stylo"].Stock -= 5` est refusé par le compilateur. Il
> faut sortir la structure, la modifier, puis la remettre dans la map.

## Erreurs fréquentes

### Écrire dans une map nil

```go norun
package main

import "fmt"

func main() {
	var stock map[string]int
	stock["pommes"] = 5
	fmt.Println(stock)
}
```

```text
panic: assignment to entry in nil map

goroutine 1 [running]:
main.main()
	/tmp/tmp.TQHY4cxvk4/main.go:7 +0x28
exit status 2
```

`var stock map[string]int` ne crée pas de map, seulement une variable `nil`. Écrivez
`stock := make(map[string]int)`.

### Déclarer ok sans s'en servir

```go norun
package main

import "fmt"

func main() {
	ages := map[string]int{"Alice": 30}
	age, ok := ages["Bob"]
	fmt.Println(age)
}
```

```text
# command-line-arguments
./main.go:7:7: declared and not used: ok
```

Si vous demandez `ok`, testez-le : `if !ok { ... }`. Sinon, écrivez simplement
`age := ages["Bob"]`.

### Utiliser une slice comme clé

```go norun
package main

import "fmt"

func main() {
	m := map[[]int]string{}
	fmt.Println(m)
}
```

```text
# command-line-arguments
./main.go:6:11: invalid map key type []int
```

Une clé doit pouvoir se comparer avec `==` : texte, nombre, booléen, tableau ou
structure simple. Une slice ne le peut pas. Utilisez un tableau (`[2]int`) ou un texte.

## Testez-vous

**Question 1 :** qu'affiche ce programme ?

```go norun
package main

import "fmt"

func main() {
	m := map[string]int{"a": 1}
	m["b"] += 5
	m["a"] = 10
	fmt.Println(m["a"], m["b"], len(m))
}
```

> **Réponse**
>
> `10 5 2`. `m["b"]` valait 0 (clé absente), puis 5 ; `m["a"]` est remplacée par 10. La
> map a deux clés.

**Question 2 :** comptez les lettres du mot `"banane"` et affichez-les dans l'ordre
alphabétique.

> **Réponse**
>
> ```go
> package main
>
> import (
> 	"fmt"
> 	"maps"
> 	"slices"
> )
>
> func main() {
> 	compte := make(map[rune]int)
> 	for _, c := range "banane" {
> 		compte[c]++
> 	}
> 	for _, c := range slices.Sorted(maps.Keys(compte)) {
> 		fmt.Printf("%c : %d\n", c, compte[c])
> 	}
> }
> ```
>
> ```sortie
> a : 2
> b : 1
> e : 1
> n : 2
> ```

**Question 3 :** pourquoi faut-il trier les clés avant d'afficher une map ?

> **Réponse**
>
> Parce que Go parcourt une map dans un ordre aléatoire, différent à chaque lancement.
> Trier les clés donne un affichage toujours identique.

## En résumé

- Une map associe des clés à des valeurs : `map[string]int`.
- Créez-la avec `make(map[K]V)` ou `map[K]V{...}` ; une map `nil` ne reçoit rien.
- `m[k] = v` ajoute ou remplace, `delete(m, k)` supprime, `len(m)` compte.
- Une clé absente donne la valeur zéro ; `v, ok := m[k]` dit si elle existe.
- L'ordre de `range` est aléatoire : triez avec `slices.Sorted(maps.Keys(m))`.

Suite : [Suivre un programme pas à pas en Go](pas-a-pas.md)
