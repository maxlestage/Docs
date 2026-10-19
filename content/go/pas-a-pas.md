# Suivre un programme pas à pas en Go

Cette page vous apprend à exécuter un programme Go dans votre tête, ligne par ligne, avec un tableau qui suit la valeur de chaque variable.

> **Objectifs :**
>
> - remplir un tableau de suivi (on dit aussi « tableau de trace ») pour un petit
>   programme ;
> - prévoir ce qu'un programme va afficher **avant** de le lancer ;
> - suivre une condition, une boucle et un appel de fonction sans vous perdre ;
> - faire afficher au programme ses propres étapes avec `fmt.Printf`.

## Jouer à l'ordinateur

Un ordinateur ne devine rien. Il lit une ligne, la fait, puis passe à la suivante.
Toujours dans le même ordre. Vous pouvez faire pareil, avec un crayon.

Pensez à un **carnet de scores** pendant un jeu de société. Chaque fois qu'un score
change, vous barrez l'ancien et vous écrivez le nouveau. Une
[variable](../debuter/glossaire.md#variable), c'est une ligne de ce carnet : une
étiquette (le nom) et un nombre (la valeur), qui peut changer.

Le **tableau de suivi** est ce carnet, mis en colonnes :

- une colonne par variable ;
- une ligne par étape (une ligne de code exécutée) ;
- dans chaque case, la valeur **après** l'étape.

```text
 code                    carnet
 ─────────────           ┌───────┬─────┬─────┐
 a := 5        ───────►  │ Étape │  a  │  b  │
 b := 3        ───────►  ├───────┼─────┼─────┤
 a = a + b     ───────►  │   1   │  5  │  —  │
                         │   2   │  5  │  3  │
                         │   3   │  8  │  3  │
                         └───────┴─────┴─────┘
```

Le tiret `—` veut dire « la variable n'existe pas encore ».

La méthode, toujours la même :

1. Lisez le programme en entier une fois.
2. **Devinez** ce qu'il va afficher. Écrivez-le.
3. Remplissez le tableau, une ligne à la fois. Ne sautez rien.
4. Lancez le programme avec `go run main.go`, puis comparez avec votre devinette.

> **À retenir :** une ligne de code = une ligne du tableau. On lit la valeur
> **actuelle** dans le tableau, jamais celle qu'on « croit » avoir.

## Programme 1 : des variables qui changent

```go
package main

import "fmt"

func main() {
	a := 5
	b := 3
	a = a + b
	b = a * 2
	a = a - 1
	fmt.Println(a, b)
}
```

Avant de lancer, devinez ce qui s'affiche. Écrivez votre réponse.

Voici le suivi, ligne par ligne :

| Étape | Ligne | Calcul | a | b |
|---|---|---|---|---|
| 1 | `a := 5` | — | 5 | — |
| 2 | `b := 3` | — | 5 | 3 |
| 3 | `a = a + b` | 5 + 3 | **8** | 3 |
| 4 | `b = a * 2` | 8 × 2 | 8 | **16** |
| 5 | `a = a - 1` | 8 − 1 | **7** | 16 |
| 6 | `fmt.Println(a, b)` | affiche | 7 | 16 |

```sortie
7 16
```

Le point important est l'étape 4. `b = a * 2` utilise la valeur de `a` **à ce
moment-là** : 8, pas 5. Le tableau vous le montre : il suffit de lire la case juste
au-dessus.

Lisez `a = a + b` de **droite à gauche** :

```text
   a  =  a + b
   ▲     └─┬─┘
   │       └── 1. on calcule avec les valeurs actuelles : 5 + 3 = 8
   └────────── 2. on range le résultat dans a (l'ancien 5 est effacé)
```

Essayez : remplacez `b := 3` par `b := 10`. Refaites le tableau avant de lancer.
Le programme affiche `14 30`.

> **À retenir :** le signe `=` ne veut pas dire « égal » comme en maths. Il veut dire
> « calcule à droite, puis range à gauche ».

## Programme 2 : une condition

Une [condition](../debuter/glossaire.md#condition) est un aiguillage : le programme
prend un seul chemin. Dans le tableau, on note la réponse à la question (vrai ou
faux), puis on saute directement au bon bloc.

```go
package main

import "fmt"

func main() {
	temperature := 12
	conseil := "Restez au chaud"
	if temperature > 20 {
		conseil = "Sortez en t-shirt"
	} else if temperature > 10 {
		conseil = "Prenez un pull"
	}
	fmt.Println(temperature, "degrés :", conseil)
}
```

Avant de lancer, devinez ce qui s'affiche.

| Étape | Ligne | Question | Réponse | temperature | conseil |
|---|---|---|---|---|---|
| 1 | `temperature := 12` | — | — | 12 | — |
| 2 | `conseil := ...` | — | — | 12 | Restez au chaud |
| 3 | `if temperature > 20` | 12 > 20 ? | faux | 12 | Restez au chaud |
| 4 | `else if temperature > 10` | 12 > 10 ? | **vrai** | 12 | Restez au chaud |
| 5 | `conseil = "Prenez un pull"` | — | — | 12 | **Prenez un pull** |
| 6 | `fmt.Println(...)` | — | — | 12 | Prenez un pull |

```sortie
12 degrés : Prenez un pull
```

À l'étape 3, la réponse est « faux » : le premier bloc est **sauté**. La ligne
`conseil = "Sortez en t-shirt"` n'apparaît jamais dans le tableau. C'est normal : une
ligne sautée n'a pas de ligne dans le tableau.

```text
             temperature > 20 ?
              /            \
           vrai            faux
            │                │
        « t-shirt »   temperature > 10 ?
                        /          \
                     vrai          faux
                      │              │
                  « pull »    (rien : on garde
                               « Restez au chaud »)
```

Essayez : mettez `temperature := 20`. Le programme affiche `20 degrés : Prenez un pull`,
car 20 n'est pas **plus grand** que 20. Avec `temperature := 5`, il affiche
`5 degrés : Restez au chaud` : aucun bloc n'est pris.

> **À retenir :** pour une condition, notez la question **et** sa réponse dans le
> tableau. Ensuite, sautez les blocs qui ne sont pas pris.

## Programme 3 : une boucle qui accumule

Une [boucle](../debuter/glossaire.md#boucle) répète des lignes. Pensez à une tirelire :
à chaque tour, vous glissez une pièce, et le total grandit. La variable `total` est la
tirelire.

```go
package main

import "fmt"

func main() {
	total := 0
	for i := 1; i <= 5; i++ {
		total = total + i
	}
	fmt.Println("Total :", total)
}
```

Avant de lancer, devinez ce qui s'affiche.

Dans le tableau, une ligne par **tour**. On note aussi la question `i <= 5 ?`, posée
**avant** chaque tour.

| Tour | i | i <= 5 ? | Calcul | total |
|---|---|---|---|---|
| départ | — | — | — | 0 |
| 1 | 1 | vrai | 0 + 1 | 1 |
| 2 | 2 | vrai | 1 + 2 | 3 |
| 3 | 3 | vrai | 3 + 3 | 6 |
| 4 | 4 | vrai | 6 + 4 | 10 |
| 5 | 5 | vrai | 10 + 5 | 15 |
| fin | 6 | **faux** | on sort | 15 |

```sortie
Total : 15
```

Regardez la dernière ligne : `i` vaut 6, la question répond « faux », et la boucle
s'arrête **sans** faire de calcul. Beaucoup de débutants oublient cette ligne. Elle
explique pourquoi la boucle s'arrête.

```text
        ┌────────────────────────────┐
        ▼                            │
  i <= 5 ? ── vrai ──► total = total + i ──► i++
        │
      faux
        ▼
  fmt.Println(...)
```

Essayez : remplacez `i <= 5` par `i < 5`. Le tour 5 disparaît. Le programme affiche
`Total : 10`.

> **À retenir :** pour une boucle, une ligne par tour, plus une dernière ligne où la
> question devient fausse. C'est cette ligne qui arrête tout.

## Programme 4 : une fonction appelée deux fois

Une [fonction](../debuter/glossaire.md#fonction) est comme une machine à café. Vous
mettez quelque chose dedans (le [paramètre](../debuter/glossaire.md#parametre)), elle
travaille, et elle vous rend un résultat (`return`). Pendant qu'elle travaille, vous
attendez.

```go
package main

import "fmt"

func carre(n int) int {
	fmt.Println("  carre reçoit", n)
	resultat := n * n
	return resultat
}

func main() {
	fmt.Println("début")
	x := carre(3)
	y := carre(x - 5)
	fmt.Println("x =", x, "y =", y)
}
```

Avant de lancer, devinez ce qui s'affiche. Attention : il y a quatre lignes.

Pour une fonction, ajoutez une colonne « Où ? ». Elle dit dans quelle fonction on se
trouve. Les variables de `carre` (`n`, `resultat`) n'existent que pendant l'appel.

| Étape | Où ? | Ligne | n | resultat | x | y | Affiche |
|---|---|---|---|---|---|---|---|
| 1 | main | `fmt.Println("début")` | — | — | — | — | début |
| 2 | main | `x := carre(3)` : on appelle | — | — | — | — | |
| 3 | carre | on reçoit `n` | 3 | — | — | — | |
| 4 | carre | `fmt.Println(...)` | 3 | — | — | — | carre reçoit 3 |
| 5 | carre | `resultat := n * n` | 3 | 9 | — | — | |
| 6 | carre | `return resultat` | 3 | 9 | — | — | |
| 7 | main | on range 9 dans `x` | — | — | **9** | — | |
| 8 | main | `y := carre(x - 5)` : 9 − 5 = 4 | — | — | 9 | — | |
| 9 | carre | on reçoit `n` | 4 | — | 9 | — | |
| 10 | carre | `fmt.Println(...)` | 4 | — | 9 | — | carre reçoit 4 |
| 11 | carre | `resultat := n * n` | 4 | 16 | 9 | — | |
| 12 | carre | `return resultat` | 4 | 16 | 9 | — | |
| 13 | main | on range 16 dans `y` | — | — | 9 | **16** | |
| 14 | main | `fmt.Println(...)` | — | — | 9 | 16 | x = 9 y = 16 |

```sortie
début
  carre reçoit 3
  carre reçoit 4
x = 9 y = 16
```

Aux étapes 7 et 13, `n` et `resultat` redeviennent `—` : l'appel est fini, ses
variables disparaissent. Au deuxième appel, `carre` repart de zéro, avec un nouveau
`n`.

Voici le même chemin, dessiné. Les flèches vers la droite sont les appels, celles vers
la gauche les valeurs rendues :

```text
 main                          carre
 ────                          ─────
 Println("début")
 x := carre(3)    ── n = 3 ──►  Println(...)
                                resultat = 9
                  ◄── 9 ─────   return
 y := carre(4)    ── n = 4 ──►  Println(...)
                                resultat = 16
                  ◄── 16 ────   return
 Println(x, y)
```

Essayez : remplacez `carre(x - 5)` par `carre(x)`. Le deuxième appel reçoit 9. Le
programme affiche `carre reçoit 9`, puis `x = 9 y = 81`.

> **À retenir :** un appel de fonction est un aller-retour. Le programme part dans la
> fonction, la fait en entier, puis revient **exactement** à la ligne d'où il est
> parti, avec la valeur rendue.

## Programme 5 : une slice et une condition dans la boucle

Dernier programme : on combine tout. Une [slice](../debuter/glossaire.md#slice) de
notes, une boucle `range` qui les lit une à une, et un `if` à l'intérieur. Imaginez un
professeur qui feuillette des copies : il regarde chaque note, et il ne compte que
celles qui valent au moins 10.

```go
package main

import "fmt"

func main() {
	notes := []int{12, 7, 15, 9, 18}
	admis := 0
	somme := 0
	for _, note := range notes {
		if note >= 10 {
			admis++
			somme += note
		}
	}
	fmt.Println("Notes >= 10 :", admis)
	fmt.Println("Leur somme :", somme)
}
```

Avant de lancer, devinez les deux lignes affichées.

| Tour | note | note >= 10 ? | admis | somme |
|---|---|---|---|---|
| départ | — | — | 0 | 0 |
| 1 | 12 | vrai | 1 | 12 |
| 2 | 7 | faux | 1 | 12 |
| 3 | 15 | vrai | 2 | 27 |
| 4 | 9 | faux | 2 | 27 |
| 5 | 18 | vrai | 3 | 45 |
| fin | — | plus de notes | 3 | 45 |

```sortie
Notes >= 10 : 3
Leur somme : 45
```

Aux tours 2 et 4, la réponse est « faux » : `admis` et `somme` **recopient** la ligne
du dessus. Rien ne change. C'est souvent là qu'on se trompe : on ajoute 7 « par
habitude ».

La boucle `range` avance toute seule dans la slice, comme un doigt qui suit les cases :

```text
 notes :  [ 12 |  7 | 15 |  9 | 18 ]
 tour 1 :    ▲
 tour 2 :         ▲
 tour 3 :              ▲
 tour 4 :                   ▲
 tour 5 :                        ▲
```

Essayez : changez `note >= 10` en `note > 12`. Seuls 15 et 18 passent. Le programme
affiche `Notes >= 10 : 2` et `Leur somme : 33` (le texte affiché, lui, ne change pas :
pensez à le corriger aussi).

> **À retenir :** quand la condition est fausse, recopiez simplement la ligne du
> dessus. Une case qui ne change pas est une information, pas un oubli.

## Faire afficher ses étapes au programme

Le tableau, vous pouvez aussi le faire écrire **par le programme**. Ajoutez un
`fmt.Printf` dans la boucle, avec une étiquette devant chaque valeur :

```go
package main

import "fmt"

func main() {
	notes := []int{12, 7, 15, 9, 18}
	admis := 0
	somme := 0
	for i, note := range notes {
		if note >= 10 {
			admis++
			somme += note
		}
		fmt.Printf("tour %d : note=%d admis=%d somme=%d\n", i+1, note, admis, somme)
	}
	fmt.Println("Leur somme :", somme)
}
```

```sortie
tour 1 : note=12 admis=1 somme=12
tour 2 : note=7 admis=1 somme=12
tour 3 : note=15 admis=2 somme=27
tour 4 : note=9 admis=2 somme=27
tour 5 : note=18 admis=3 somme=45
Leur somme : 45
```

C'est exactement le tableau de la section précédente ! Trois conseils :

- **Mettez des étiquettes** (`note=`) : des nombres seuls (`12 1 12`) sont illisibles.
- **Placez le `Printf` à la fin du tour**, pour voir les valeurs après les calculs.
- `range` compte à partir de 0 : on affiche `i+1` pour dire « tour 1 ».

Essayez : déplacez le `fmt.Printf` **avant** le `if`. Chaque ligne montre alors les
valeurs du tour précédent : `tour 1 : note=12 admis=0 somme=0`.

> **À retenir :** un `fmt.Printf` avec des étiquettes transforme le programme en
> tableau de suivi automatique. Enlevez-le quand le bug est trouvé.

## Le débogueur en deux phrases

Un débogueur, comme Delve pour Go, met le programme en pause sur une ligne choisie et
vous montre toutes les variables, puis avance d'une ligne à chaque commande. C'est le
tableau de suivi, rempli par l'ordinateur : vous le découvrirez dans
[le chapitre sur le débogage](debogage.md#le-debogueur-delve).

## Erreurs fréquentes

### Réutiliser une ancienne valeur

On veut échanger `a` et `b`. On pense que `b` reçoit l'ancien `a`.

```go
package main

import "fmt"

func main() {
	a := 1
	b := 2
	a = b
	b = a
	fmt.Println(a, b)
}
```

```sortie
2 2
```

Le tableau montre l'erreur : après `a = b`, `a` vaut 2. Le 1 est **effacé**.
Ensuite, `b = a` recopie ce 2.

| Étape | Ligne | a | b |
|---|---|---|---|
| 1 | `a := 1` | 1 | — |
| 2 | `b := 2` | 1 | 2 |
| 3 | `a = b` | **2** | 2 |
| 4 | `b = a` | 2 | **2** |

La correction : en Go, l'échange se fait en une seule ligne, `a, b = b, a`. Le
programme affiche alors `2 1`.

### Se tromper d'un tour

On veut additionner 1, 2, 3, 4 et 5. On écrit `<` au lieu de `<=`.

```go
package main

import "fmt"

func main() {
	total := 0
	for i := 1; i < 5; i++ {
		total += i
	}
	fmt.Println("Total :", total)
}
```

```sortie
Total : 10
```

Dans le tableau, la ligne où `i` vaut 5 répond « faux » à `5 < 5 ?` : le 5 n'est
jamais ajouté. La correction : `i <= 5`. Vérifiez toujours le **premier** et le
**dernier** tour.

### Croire qu'une variable existe encore après la boucle

`i` est créé dans la ligne du `for`. Il disparaît quand la boucle se termine.

```go norun
package main

import "fmt"

func main() {
	total := 0
	for i := 1; i <= 3; i++ {
		total += i
	}
	fmt.Println("Dernier i :", i)
	fmt.Println("Total :", total)
}
```

```text
# command-line-arguments
./main.go:10:29: undefined: i
```

Dans le tableau, mettez un `—` dans la colonne `i` dès la sortie de la boucle. La
correction : supprimez la ligne, ou créez `i` **avant** la boucle si vous en avez
besoin après.

### Remettre la tirelire à zéro à chaque tour

On veut afficher les totaux 1, 3, 6. Mais `total := 0` est placé **dans** la boucle.

```go
package main

import "fmt"

func main() {
	for i := 1; i <= 3; i++ {
		total := 0
		total += i
		fmt.Println(total)
	}
}
```

```sortie
1
2
3
```

À chaque tour, la ligne `total := 0` est exécutée de nouveau : le tableau repasse
`total` à 0 à chaque tour. La correction : placez `total := 0` **avant** le `for`.

## Testez-vous

**1.** Que vaut `x` après ces quatre lignes : `x := 4`, `y := x + 1`, `x = y * 2`,
`y = 0` ?

> **Réponse**
>
> `x` vaut 10. Étape par étape : `x` = 4, puis `y` = 5, puis `x` = 5 × 2 = 10. Mettre
> `y` à 0 ensuite ne change pas `x` : la valeur 10 a déjà été rangée.

**2.** Combien de fois `Bonjour` s'affiche-t-il avec `for i := 0; i < 3; i++` ?
Quelle est la valeur de `i` quand la boucle s'arrête ?

> **Réponse**
>
> Trois fois : pour `i` = 0, 1 et 2. La boucle s'arrête quand `i` vaut 3, car
> `3 < 3` est faux.

**3.** Qu'affiche ce programme ?

```go
package main

import "fmt"

func ajouter(a int, b int) int {
	return a + b
}

func main() {
	r := ajouter(2, 3)
	r = ajouter(r, r)
	fmt.Println(r)
}
```

> **Réponse**
>
> Il affiche 10. Le premier appel rend 2 + 3 = 5, rangé dans `r`. Le deuxième appel
> reçoit `a` = 5 et `b` = 5, et rend 10, qui remplace l'ancien 5.

## En résumé

- Un tableau de suivi a une colonne par variable et une ligne par étape.
- `=` calcule à droite avec les valeurs **actuelles**, puis range à gauche.
- Pour une condition, notez la question et sa réponse, puis sautez les blocs non pris.
- Pour une boucle, une ligne par tour, plus la ligne où la question devient fausse.
- Un appel de fonction est un aller-retour : ses variables naissent et disparaissent
  avec lui.
- `fmt.Printf` avec des étiquettes fait écrire le tableau par le programme.

Suite : [Les pièges du débutant en Go](pieges.md)
