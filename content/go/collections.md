# Tableaux et slices

Cette page présente les deux façons de ranger plusieurs valeurs à la suite en Go : le tableau, de taille fixe, et la slice, qui peut grandir.

> **Objectifs :**
>
> - vous saurez créer un tableau et une slice, et lire leurs éléments ;
> - vous saurez ajouter des éléments avec `append` ;
> - vous saurez découper une slice avec `s[début:fin]` et la copier avec `copy` ;
> - vous saurez éviter le piège du tableau partagé ;
> - vous saurez trier et chercher avec les paquets `slices` et `sort`.

## Les tableaux

Imaginez que vous voulez garder les notes d'un élève. Créer une variable par note
(`note1`, `note2`, `note3`…) devient vite pénible. Un tableau range toutes ces valeurs
sous un seul nom, comme une boîte à compartiments numérotés.

Un tableau (*array*) range un nombre **fixe** de valeurs du même type. La taille fait
partie du type : `[3]int` est « un tableau de 3 entiers ».

```go
package main

import "fmt"

func main() {
	var notes [3]int
	notes[0] = 12
	notes[2] = 18
	fmt.Println(notes)

	jours := [3]string{"lun", "mar", "mer"}
	fmt.Println(jours[1], len(jours))
}
```

```sortie
[12 0 18]
mar 3
```

Ligne par ligne :

- `var notes [3]int` crée un tableau de 3 cases, toutes à 0 : `[0 0 0]` ;
- `notes[0] = 12` range 12 dans la première case, `notes[2] = 18` dans la troisième.
  La case du milieu garde son 0 ;
- `jours := [3]string{...}` crée le tableau et le remplit en une seule ligne ;
- `jours[1]` est la deuxième case, « mar », et `len(jours)` vaut 3.

Trois règles à connaître :

- Les cases sont numérotées à partir de **0** : c'est l'[indice](../debuter/glossaire.md#indice).
- Une case non remplie a la valeur zéro de son type (`0` ici).
- `len` donne le nombre de cases.

Un tableau ne peut ni grandir ni rétrécir. C'est pourquoi on l'utilise peu : on
préfère presque toujours la slice.

## Les slices

Souvent, vous ne savez pas d'avance combien de valeurs vous aurez : une liste de
courses s'allonge au fil de la semaine. La slice est faite pour cela.

Une slice est une [liste](../debuter/glossaire.md#liste) de valeurs du même type, dont
la taille peut changer. Son type s'écrit sans taille : `[]int`.

```go
package main

import "fmt"

func main() {
	fruits := []string{"pomme", "kiwi", "poire"}
	fmt.Println(fruits)
	fmt.Println(fruits[0], len(fruits))

	fruits[1] = "mangue"
	fmt.Println(fruits)
	fmt.Println(fruits[len(fruits)-1])
}
```

```sortie
[pomme kiwi poire]
pomme 3
[pomme mangue poire]
poire
```

Ligne par ligne :

- `fruits := []string{...}` crée une slice qui contient trois textes ;
- `fruits[0]` est le premier élément, « pomme », et `len(fruits)` vaut 3 ;
- `fruits[1] = "mangue"` remplace « kiwi » par « mangue ». Les autres cases ne
  bougent pas.

`fruits[len(fruits)-1]` est le dernier élément : avec 3 éléments, les indices vont de 0
à 2.

**À vous :** écrivez un programme qui crée une slice `couleurs` avec « rouge », « vert »
et « bleu », remplace « vert » par « jaune », puis affiche la slice et sa longueur.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	couleurs := []string{"rouge", "vert", "bleu"}
> 	couleurs[1] = "jaune"
> 	fmt.Println(couleurs, len(couleurs))
> }
> ```
>
> ```sortie
> [rouge jaune bleu] 3
> ```

### Créer une slice avec make

`make([]type, longueur)` crée une slice remplie de valeurs zéro.

```go
package main

import "fmt"

func main() {
	scores := make([]int, 4)
	scores[2] = 7
	fmt.Println(scores, len(scores))

	var vide []string
	fmt.Println(vide, len(vide), vide == nil)
}
```

```sortie
[0 0 7 0] 4
[] 0 true
```

Une slice déclarée avec `var` sans valeur vaut `nil` : elle est vide, et on peut
quand même lui ajouter des éléments.

## Ajouter avec append

Une liste de courses se remplit petit à petit : vous ajoutez un article dès que vous
y pensez. `append` fait la même chose pour une slice.

`append` ajoute des éléments à la fin d'une slice et **renvoie** la nouvelle slice. Il
faut toujours récupérer ce résultat.

```go
package main

import "fmt"

func main() {
	var courses []string
	courses = append(courses, "pain")
	courses = append(courses, "lait", "œufs")
	fmt.Println(courses, len(courses))

	autres := []string{"riz", "thé"}
	courses = append(courses, autres...)
	fmt.Println(courses)
}
```

```sortie
[pain lait œufs] 3
[pain lait œufs riz thé]
```

Ligne par ligne :

- `var courses []string` crée une slice vide ;
- le premier `append` renvoie une slice qui contient « pain ». On la range de nouveau
  dans `courses` ;
- le deuxième `append` ajoute deux éléments d'un coup : `courses` en a maintenant 3 ;
- le dernier `append` ajoute à la fin tous les éléments de `autres`.

`autres...` déplie la slice `autres` en plusieurs arguments.

**À vous :** écrivez un programme qui part d'une slice vide `invites`, y ajoute « Léa »,
puis « Tom » et « Sami » en un seul `append`, et affiche la slice et sa longueur.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	var invites []string
> 	invites = append(invites, "Léa")
> 	invites = append(invites, "Tom", "Sami")
> 	fmt.Println(invites, len(invites))
> }
> ```
>
> ```sortie
> [Léa Tom Sami] 3
> ```

### La capacité

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

Derrière chaque slice se cache un tableau, qui a une certaine place : la **capacité**
(`cap`). Quand la place manque, `append` crée un tableau plus grand et y recopie tout.
C'est pour cela qu'il faut récupérer le résultat de `append` : ce n'est peut-être plus
le même tableau.

```go
package main

import "fmt"

func main() {
	s := make([]int, 0, 3)
	for i := range 5 {
		s = append(s, i)
		fmt.Println(len(s), cap(s))
	}
}
```

```sortie
1 3
2 3
3 3
4 6
5 6
```

- `len` : le nombre d'éléments présents ;
- `cap` : le nombre de places réservées.

Imaginez une boîte à œufs : `len` compte les œufs, `cap` compte les alvéoles. Quand la
boîte est pleine, Go en prend une plus grande.

> **Astuce :** si vous savez d'avance combien d'éléments vous ajouterez, réservez la
> place avec `make([]int, 0, n)`. Go n'aura pas à recopier en cours de route.

## Découper une slice

Parfois, vous n'avez besoin que d'une partie d'une liste : les trois premiers d'un
classement, ou tout sauf le premier élément. Le découpage sert à cela.

`s[début:fin]` prend les éléments de l'indice `début` **inclus** à l'indice `fin`
**exclu**.

```go
package main

import "fmt"

func main() {
	lettres := []string{"a", "b", "c", "d", "e"}
	fmt.Println(lettres[1:3])
	fmt.Println(lettres[:2])
	fmt.Println(lettres[3:])
	fmt.Println(lettres[:])
}
```

```sortie
[b c]
[a b]
[d e]
[a b c d e]
```

- `[:2]` : depuis le début jusqu'à 2 (exclu) ;
- `[3:]` : depuis 3 jusqu'à la fin ;
- `[:]` : tout.

> **À retenir :** `s[1:3]` contient `3 - 1 = 2` éléments. La fin est toujours exclue.

**À vous :** écrivez un programme qui range les mois `"jan"` à `"juin"` dans une slice
`mois`, puis affiche les trois premiers, et enfin les trois derniers.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	mois := []string{"jan", "fév", "mar", "avr", "mai", "juin"}
> 	fmt.Println(mois[:3])
> 	fmt.Println(mois[3:])
> }
> ```
>
> ```sortie
> [jan fév mar]
> [avr mai juin]
> ```

## Le piège du tableau partagé

Ce point surprend presque tous les débutants, et il cause des bugs difficiles à
trouver. Mieux vaut le comprendre dès maintenant.

Un découpage ne copie **rien** : il regarde le même tableau caché, par une autre
fenêtre. Modifier l'un modifie l'autre.

```go
package main

import "fmt"

func main() {
	nombres := []int{1, 2, 3, 4}
	debut := nombres[:2]
	debut[0] = 99
	fmt.Println(nombres)
	fmt.Println(debut)
}
```

```sortie
[99 2 3 4]
[99 2]
```

Ligne par ligne :

- `nombres` contient 1, 2, 3 et 4 ;
- `debut := nombres[:2]` ne crée aucune case : `debut` montre les deux premières cases
  de `nombres` ;
- `debut[0] = 99` écrit donc 99 dans la première case, commune aux deux ;
- les deux `Println` lisent les mêmes cases : 99 apparaît deux fois.

`debut` et `nombres` partagent les mêmes cases : changer `debut[0]` a changé
`nombres[0]`.

C'est la même chose quand vous **affectez** une slice à une autre variable, ou que vous
la passez à une fonction : les deux noms désignent les mêmes cases.

```go
package main

import "fmt"

func remettreAZero(s []int) {
	for i := range s {
		s[i] = 0
	}
}

func main() {
	a := []int{5, 6, 7}
	b := a
	b[0] = 100
	fmt.Println(a)

	remettreAZero(a)
	fmt.Println(a)
}
```

```sortie
[100 6 7]
[0 0 0]
```

### Faire une vraie copie avec copy

Pour obtenir une slice indépendante, créez-en une nouvelle et copiez-y les éléments
avec `copy(destination, source)`.

```go
package main

import "fmt"

func main() {
	original := []int{1, 2, 3}
	copie := make([]int, len(original))
	copy(copie, original)
	copie[0] = 99
	fmt.Println(original)
	fmt.Println(copie)
}
```

```sortie
[1 2 3]
[99 2 3]
```

> **Astuce :** depuis Go 1.21, `slices.Clone(original)` fait la même chose en une ligne.

## Les slices à deux dimensions

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

Une slice peut contenir des slices : c'est une grille, comme un tableur ou un plateau
de jeu.

```go
package main

import "fmt"

func main() {
	grille := [][]int{
		{1, 2, 3},
		{4, 5, 6},
	}
	fmt.Println(grille[1][2])

	for i, ligne := range grille {
		fmt.Println("ligne", i, ":", ligne)
	}

	plateau := make([][]string, 2)
	for i := range plateau {
		plateau[i] = make([]string, 3)
	}
	plateau[0][1] = "X"
	fmt.Printf("%q\n", plateau)
}
```

```sortie
6
ligne 0 : [1 2 3]
ligne 1 : [4 5 6]
[["" "X" ""] ["" "" ""]]
```

`grille[1][2]` se lit : ligne 1, colonne 2. Avec `make`, il faut créer **chaque**
ligne, une par une. Le verbe `%q` affiche les textes entre guillemets : on voit ainsi
les cases vides `""`.

## Trier et chercher

Chercher un prénom dans une liste, ranger des notes de la plus petite à la plus grande :
ce sont des tâches très courantes. Go les fournit toutes faites, vous n'avez pas à les
écrire vous-même.

### Le paquet slices

Depuis Go 1.21, le paquet `slices` regroupe les opérations courantes.

```go
package main

import (
	"fmt"
	"slices"
)

func main() {
	notes := []int{14, 9, 17, 12}
	fmt.Println(slices.Contains(notes, 9))
	fmt.Println(slices.Index(notes, 17))
	fmt.Println(slices.Max(notes), slices.Min(notes))

	slices.Sort(notes)
	fmt.Println(notes)

	prenoms := []string{"Zoé", "Alice", "Marc"}
	slices.Sort(prenoms)
	fmt.Println(prenoms)
}
```

```sortie
true
2
17 9
[9 12 14 17]
[Alice Marc Zoé]
```

| Fonction | Rôle |
|----------|------|
| `slices.Contains(s, v)` | `s` contient-elle `v` ? |
| `slices.Index(s, v)` | la position de `v` (`-1` si absent) |
| `slices.Max(s)`, `slices.Min(s)` | le plus grand, le plus petit |
| `slices.Sort(s)` | trie `s` sur place, dans l'ordre croissant |
| `slices.Reverse(s)` | inverse l'ordre de `s` |

> **Note :** `slices.Sort` modifie la slice elle-même, et ne renvoie rien.

**À vous :** écrivez un programme qui range les températures 18, 25, 12 et 21 dans une
slice, affiche la plus haute, puis affiche la slice triée.

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
> 	temperatures := []int{18, 25, 12, 21}
> 	fmt.Println(slices.Max(temperatures))
> 	slices.Sort(temperatures)
> 	fmt.Println(temperatures)
> }
> ```
>
> ```sortie
> 25
> [12 18 21 25]
> ```

### Trier selon une règle : sort.Slice

Pour trier selon votre propre règle, `sort.Slice` prend une fonction qui répond à la
question : « l'élément `i` doit-il passer avant l'élément `j` ? ».

```go
package main

import (
	"fmt"
	"sort"
)

func main() {
	mots := []string{"pamplemousse", "kiwi", "ananas", "figue"}
	sort.Slice(mots, func(i, j int) bool {
		return len(mots[i]) < len(mots[j])
	})
	fmt.Println(mots)

	notes := []int{12, 18, 9}
	sort.Slice(notes, func(i, j int) bool {
		return notes[i] > notes[j]
	})
	fmt.Println(notes)
}
```

```sortie
[kiwi figue ananas pamplemousse]
[18 12 9]
```

Le premier tri range les mots du plus court au plus long, le second les notes de la
plus grande à la plus petite.

> **Astuce :** `sort.SliceStable` fait le même tri, mais garde l'ordre de départ des
> éléments égaux. Utilisez-le si deux mots de même longueur doivent rester dans leur
> ordre d'origine.

## Erreurs fréquentes

### Lire après la fin

```go norun
package main

import "fmt"

func main() {
	notes := []int{12, 15, 9}
	fmt.Println(notes[3])
}
```

```text
panic: runtime error: index out of range [3] with length 3

goroutine 1 [running]:
main.main()
	/tmp/tmp.XlL8TvM6jN/main.go:7 +0x15
exit status 2
```

Avec 3 éléments, les indices vont de 0 à 2. L'indice 3 est hors limites (*out of
range*). Le dernier élément est `notes[len(notes)-1]`.

### Oublier de récupérer le résultat de append

```go norun
package main

import "fmt"

func main() {
	notes := []int{12, 15}
	append(notes, 9)
	fmt.Println(notes)
}
```

```text
# command-line-arguments
./main.go:7:2: append(notes, 9) (value of type []int) is not used
```

`append` ne modifie pas `notes` : il renvoie une nouvelle slice. Écrivez
`notes = append(notes, 9)`.

### Comparer deux slices avec ==

```go norun
package main

import "fmt"

func main() {
	a := []int{1, 2}
	b := []int{1, 2}
	fmt.Println(a == b)
}
```

```text
# command-line-arguments
./main.go:8:14: invalid operation: a == b (slice can only be compared to nil)
```

Une slice ne se compare qu'à `nil`. Pour comparer le contenu, utilisez
`slices.Equal(a, b)`.

### Trop de valeurs pour un tableau

```go norun
package main

import "fmt"

func main() {
	jours := [3]string{"lun", "mar", "mer", "jeu"}
	fmt.Println(jours)
}
```

```text
# command-line-arguments
./main.go:6:42: index 3 is out of bounds (>= 3)
```

Le tableau a 3 cases, pas 4. Agrandissez-le (`[4]string`), ou utilisez une slice
(`[]string{...}`), qui s'adapte toute seule.

## Testez-vous

**Question 1 :** qu'affiche ce programme ?

```go norun
package main

import "fmt"

func main() {
	s := []int{10, 20, 30, 40, 50}
	fmt.Println(s[1:4], len(s[2:]))
}
```

> **Réponse**
>
> `[20 30 40] 3`. `s[1:4]` va de l'indice 1 à 3 inclus ; `s[2:]` contient 30, 40 et 50.

**Question 2 :** à partir de `[]int{5, 3, 8, 1}`, créez une copie triée, sans changer
l'originale, puis affichez les deux.

> **Réponse**
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
> 	original := []int{5, 3, 8, 1}
> 	triee := slices.Clone(original)
> 	slices.Sort(triee)
> 	fmt.Println(original, triee)
> }
> ```
>
> ```sortie
> [5 3 8 1] [1 3 5 8]
> ```

**Question 3 :** après `b := a[:2]` puis `b[0] = 0`, `a` a-t-elle changé ?

> **Réponse**
>
> Oui. Un découpage ne copie pas les éléments : `a` et `b` partagent le même tableau
> caché. Pour une copie indépendante, utilisez `copy` ou `slices.Clone`.

## En résumé

- Un tableau `[3]int` a une taille fixe ; une slice `[]int` peut grandir.
- `append` ajoute à la fin et renvoie la slice : écrivez `s = append(s, v)`.
- `s[début:fin]` découpe, fin exclue, sans rien copier : les deux slices partagent
  leurs cases.
- `copy` ou `slices.Clone` font une vraie copie.
- Le paquet `slices` trie, cherche et compare ; `sort.Slice` trie selon votre règle.

Suite : [les maps](maps.md). Comparez avec
[les collections en Rust](../rust/collections.md).
