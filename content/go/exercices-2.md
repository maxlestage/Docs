# Exercices corrigés : deuxième série

Cette page propose 12 nouveaux exercices corrigés en Go, du plus facile au plus difficile, avec des tableaux qui suivent les variables pas à pas.

## Comment utiliser cette page

Ces exercices sont **nouveaux** : ils ne reprennent ni
[les exercices pour bien démarrer](exercices-faciles.md), ni
[la première série](exercices.md). Chacun a un niveau :

- ★☆☆ : facile (4 exercices) ;
- ★★☆ : moyen (5 exercices) ;
- ★★★ : plus dur (3 exercices).

Pour chaque exercice :

1. Lisez l'énoncé et la sortie attendue.
2. Écrivez les étapes en français, sur papier, comme une recette.
3. Codez dans `main.go`, puis lancez `go run main.go`.
4. Bloqué ? Lisez l'indice. La solution vient en dernier.

```text
 énoncé → papier → code → go run → bon résultat ?
                    ▲                  │
                    └────── non ───────┘
```

> **Astuce :** quand un résultat vous surprend, faites un tableau de suivi, comme dans
> [Suivre un programme pas à pas](pas-a-pas.md). Les solutions ci-dessous en montrent
> plusieurs.

## Échauffement

### Exercice 1 : Des secondes en heures

Niveau : ★☆☆

Un film dure 3725 secondes. Affichez sa durée en heures, minutes et secondes :
`1 h 2 min 5 s`.

> **Indice :** une heure fait 3600 secondes, une minute 60. Entre deux entiers, `/`
> donne la division entière et `%` le reste.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	total := 3725
> 	heures := total / 3600
> 	reste := total % 3600
> 	minutes := reste / 60
> 	secondes := reste % 60
> 	fmt.Printf("%d h %d min %d s\n", heures, minutes, secondes)
> }
> ```
>
> ```sortie
> 1 h 2 min 5 s
> ```
>
> Pourquoi ça marche : c'est comme ranger des pièces. On remplit d'abord les « boîtes
> d'une heure » (1 boîte, il reste 125 s), puis les « boîtes d'une minute » (2 boîtes,
> il reste 5 s).
>
> | Étape | Calcul | Résultat |
> |---|---|---|
> | heures | 3725 / 3600 | 1 |
> | reste | 3725 % 3600 | 125 |
> | minutes | 125 / 60 | 2 |
> | secondes | 125 % 60 | 5 |
>
> Essayez : avec `total := 59`, le programme affiche `0 h 0 min 59 s`.

### Exercice 2 : Compter les voyelles

Niveau : ★☆☆

Comptez les voyelles (a, e, i, o, u, y) du mot `"programmation"`.

Sortie attendue :

```text
programmation : 5 voyelles
```

> **Indice :** parcourez le mot avec `range`. `strings.ContainsRune("aeiouy", lettre)`
> répond `true` si la lettre fait partie des voyelles.

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
> func main() {
> 	mot := "programmation"
> 	voyelles := 0
> 	for _, lettre := range mot {
> 		if strings.ContainsRune("aeiouy", lettre) {
> 			voyelles++
> 		}
> 	}
> 	fmt.Println(mot, ":", voyelles, "voyelles")
> }
> ```
>
> ```sortie
> programmation : 5 voyelles
> ```
>
> Pourquoi ça marche : la chaîne `"aeiouy"` sert de liste de référence, comme une liste
> de courses qu'on vérifie. Chaque lettre du mot est cherchée dedans : o, a, a, i, o
> sont trouvées, soit 5.

### Exercice 3 : La somme des chiffres

Niveau : ★☆☆

Calculez la somme des chiffres de `4721`, sans le transformer en texte.

Sortie attendue :

```text
Somme des chiffres de 4721 : 14
```

> **Indice :** `n % 10` donne le dernier chiffre (`4721 % 10` vaut 1). `n / 10` l'enlève
> (`4721 / 10` vaut 472). Répétez tant que `n` n'est pas 0.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	nombre := 4721
> 	n := nombre
> 	somme := 0
> 	for n > 0 {
> 		somme += n % 10
> 		n = n / 10
> 	}
> 	fmt.Println("Somme des chiffres de", nombre, ":", somme)
> }
> ```
>
> ```sortie
> Somme des chiffres de 4721 : 14
> ```
>
> Pourquoi ça marche : à chaque tour, on « détache » le dernier chiffre, comme on
> arrache la dernière page d'un carnet.
>
> | Tour | n au début | n % 10 | somme | n à la fin |
> |---|---|---|---|---|
> | 1 | 4721 | 1 | 1 | 472 |
> | 2 | 472 | 2 | 3 | 47 |
> | 3 | 47 | 7 | 10 | 4 |
> | 4 | 4 | 4 | 14 | 0 |
> | fin | 0 | `0 > 0` est faux | 14 | — |
>
> On copie `nombre` dans `n` pour pouvoir afficher le nombre de départ à la fin.

### Exercice 4 : Un triangle d'étoiles

Niveau : ★☆☆

Affichez un triangle de 4 lignes : une étoile sur la première, deux sur la deuxième,
et ainsi de suite.

Sortie attendue :

```text
*
**
***
****
```

> **Indice :** deux boucles l'une dans l'autre. La boucle du dehors compte les lignes
> (`ligne` de 1 à 4). Celle du dedans affiche `ligne` étoiles avec `fmt.Print("*")`,
> qui ne passe pas à la ligne.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	for ligne := 1; ligne <= 4; ligne++ {
> 		for etoile := 1; etoile <= ligne; etoile++ {
> 			fmt.Print("*")
> 		}
> 		fmt.Println()
> 	}
> }
> ```
>
> ```sortie
> *
> **
> ***
> ****
> ```
>
> Pourquoi ça marche : la boucle du dedans fait un tour **complet** pour chaque tour de
> celle du dehors, comme les aiguilles d'une horloge : la grande fait le tour pendant
> que la petite avance d'un cran. `fmt.Println()` sans rien termine la ligne.
>
> Essayez : remplacez `etoile <= ligne` par `etoile <= 5-ligne`. Le triangle est à
> l'envers : 4 étoiles, puis 3, 2, 1.

> **À retenir :** `/` et `%` découpent un nombre ; une boucle dans une boucle dessine
> des lignes et des colonnes.

## Un peu plus de réflexion

### Exercice 5 : Les nombres premiers

Niveau : ★★☆

Un nombre premier n'est divisible que par 1 et par lui-même (7 l'est, 8 ne l'est pas).
Écrivez `estPremier(n int) bool`, puis affichez les nombres premiers de 2 à 30 sur une
ligne.

Sortie attendue :

```text
2 3 5 7 11 13 17 19 23 29
```

> **Indice :** essayez de diviser `n` par 2, 3, 4… Si un reste vaut 0 (`n%d == 0`),
> `n` n'est pas premier : renvoyez `false` tout de suite. Il suffit de tester les `d`
> tels que `d*d <= n`.

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
> func estPremier(n int) bool {
> 	if n < 2 {
> 		return false
> 	}
> 	for d := 2; d*d <= n; d++ {
> 		if n%d == 0 {
> 			return false
> 		}
> 	}
> 	return true
> }
>
> func main() {
> 	var premiers []string
> 	for n := 2; n <= 30; n++ {
> 		if estPremier(n) {
> 			premiers = append(premiers, strconv.Itoa(n))
> 		}
> 	}
> 	fmt.Println(strings.Join(premiers, " "))
> }
> ```
>
> ```sortie
> 2 3 5 7 11 13 17 19 23 29
> ```
>
> Pourquoi ça marche : la fonction est un videur à l'entrée d'une salle. Dès qu'un
> diviseur est trouvé, elle dit « non » et s'arrête. Pour 25 :
>
> | d | d*d <= 25 ? | 25 % d | Décision |
> |---|---|---|---|
> | 2 | 4 : oui | 1 | on continue |
> | 3 | 9 : oui | 1 | on continue |
> | 4 | 16 : oui | 1 | on continue |
> | 5 | 25 : oui | **0** | `return false` |
>
> Si `n` avait un diviseur plus grand que sa racine, il en aurait aussi un plus petit,
> déjà testé : d'où `d*d <= n`.

### Exercice 6 : La suite de Fibonacci

Niveau : ★★☆

Dans la suite de Fibonacci, chaque nombre est la somme des deux précédents. Elle
commence par 0 et 1. Affichez ses 10 premiers nombres.

Sortie attendue :

```text
0 1 1 2 3 5 8 13 21 34
```

> **Indice :** gardez deux variables, `a` (le nombre actuel) et `b` (le suivant). À
> chaque tour, affichez `a`, puis avancez d'un cran avec `a, b = b, a+b`.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	a, b := 0, 1
> 	for i := 1; i <= 10; i++ {
> 		fmt.Print(a)
> 		if i < 10 {
> 			fmt.Print(" ")
> 		}
> 		a, b = b, a+b
> 	}
> 	fmt.Println()
> }
> ```
>
> ```sortie
> 0 1 1 2 3 5 8 13 21 34
> ```
>
> Pourquoi ça marche : `a` et `b` avancent ensemble, comme deux pieds qui marchent.
> `a, b = b, a+b` calcule **toute** la partie droite avec les anciennes valeurs, puis
> range les deux résultats.
>
> | Tour | affiche a | nouveau a | nouveau b |
> |---|---|---|---|
> | 1 | 0 | 1 | 1 |
> | 2 | 1 | 1 | 2 |
> | 3 | 1 | 2 | 3 |
> | 4 | 2 | 3 | 5 |
> | 5 | 3 | 5 | 8 |
> | … | … | … | … |
> | 10 | 34 | 55 | 89 |
>
> Le `if i < 10` évite un espace en trop à la fin de la ligne.

### Exercice 7 : Les loisirs en commun

Niveau : ★★☆

Léa aime `lecture`, `foot`, `piano` et `échecs`. Tom aime `piano`, `natation` et
`lecture`. Affichez les loisirs qu'ils ont en commun, dans l'ordre de la liste de Léa.

Sortie attendue :

```text
En commun : [lecture piano]
```

> **Indice :** mettez les loisirs de Tom dans une `map[string]bool`. Ensuite,
> parcourez ceux de Léa : `tom[loisir]` vaut `true` seulement si Tom l'aime aussi.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	lea := []string{"lecture", "foot", "piano", "échecs"}
> 	tom := []string{"piano", "natation", "lecture"}
>
> 	aimeTom := make(map[string]bool)
> 	for _, loisir := range tom {
> 		aimeTom[loisir] = true
> 	}
>
> 	var communs []string
> 	for _, loisir := range lea {
> 		if aimeTom[loisir] {
> 			communs = append(communs, loisir)
> 		}
> 	}
> 	fmt.Println("En commun :", communs)
> }
> ```
>
> ```sortie
> En commun : [lecture piano]
> ```
>
> Pourquoi ça marche : la map est une liste d'invités à l'entrée. Une clé absente
> renvoie `false` (la valeur zéro d'un `bool`), donc `aimeTom["foot"]` est faux sans
> erreur. Comme on parcourt la **slice** de Léa, l'ordre est toujours le même.

### Exercice 8 : Compresser un texte

Niveau : ★★☆

Écrivez `compresser(s string) string` qui remplace chaque suite de lettres identiques
par la lettre suivie du nombre de répétitions : `"aaabccdddd"` devient `"a3b1c2d4"`.

> **Indice :** gardez la lettre en cours et un compteur. Quand la lettre change, écrivez
> la lettre et son compteur, puis repartez à 1. N'oubliez pas la dernière suite, après
> la boucle.

> **Solution**
>
> ```go
> package main
>
> import (
> 	"fmt"
> 	"strconv"
> )
>
> func compresser(s string) string {
> 	lettres := []rune(s)
> 	if len(lettres) == 0 {
> 		return ""
> 	}
> 	resultat := ""
> 	enCours := lettres[0]
> 	compte := 1
> 	for _, l := range lettres[1:] {
> 		if l == enCours {
> 			compte++
> 		} else {
> 			resultat += string(enCours) + strconv.Itoa(compte)
> 			enCours = l
> 			compte = 1
> 		}
> 	}
> 	resultat += string(enCours) + strconv.Itoa(compte)
> 	return resultat
> }
>
> func main() {
> 	fmt.Println(compresser("aaabccdddd"))
> 	fmt.Println(compresser("éé"))
> }
> ```
>
> ```sortie
> a3b1c2d4
> é2
> ```
>
> Pourquoi ça marche : c'est comme compter des voitures de la même couleur dans un
> embouteillage. Quand la couleur change, on note le paquet et on recommence.
>
> | Lettre lue | Pareille ? | enCours | compte | resultat |
> |---|---|---|---|---|
> | (départ) | — | a | 1 | "" |
> | a | oui | a | 2 | "" |
> | a | oui | a | 3 | "" |
> | b | non | b | 1 | "a3" |
> | c | non | c | 1 | "a3b1" |
> | c | oui | c | 2 | "a3b1" |
> | d | non | d | 1 | "a3b1c2" |
> | d, d, d | oui | d | 4 | "a3b1c2" |
> | (après la boucle) | — | d | 4 | "a3b1c2d4" |
>
> Les `[]rune` font marcher les lettres accentuées, comme `é`.

### Exercice 9 : Ranger par première lettre

Niveau : ★★☆

Rangez les fruits `banane`, `avocat`, `bleuet`, `abricot`, `cerise` selon leur
première lettre. Affichez les lettres dans l'ordre alphabétique.

Sortie attendue :

```text
a : [avocat abricot]
b : [banane bleuet]
c : [cerise]
```

> **Indice :** une `map[string][]string` : la clé est la lettre, la valeur une slice
> de fruits. `fruit[:1]` donne la première lettre (ici, sans accent). Pour l'ordre,
> triez les clés avec `slices.Sorted(maps.Keys(...))`.

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
> 	fruits := []string{"banane", "avocat", "bleuet", "abricot", "cerise"}
> 	parLettre := make(map[string][]string)
> 	for _, fruit := range fruits {
> 		lettre := fruit[:1]
> 		parLettre[lettre] = append(parLettre[lettre], fruit)
> 	}
> 	for _, lettre := range slices.Sorted(maps.Keys(parLettre)) {
> 		fmt.Println(lettre, ":", parLettre[lettre])
> 	}
> }
> ```
>
> ```sortie
> a : [avocat abricot]
> b : [banane bleuet]
> c : [cerise]
> ```
>
> Pourquoi ça marche : la map est un meuble à tiroirs, un tiroir par lettre. Pour une
> lettre nouvelle, `parLettre[lettre]` vaut une slice vide (`nil`), et `append` sur
> `nil` marche : le tiroir est créé au premier fruit.
>
> ```text
>  ┌─── a ───┐ ┌─── b ───┐ ┌─── c ───┐
>  │ avocat  │ │ banane  │ │ cerise  │
>  │ abricot │ │ bleuet  │ │         │
>  └─────────┘ └─────────┘ └─────────┘
> ```

> **À retenir :** une map sert à **retrouver vite** (exercice 7) et à **regrouper**
> (exercice 9) ; triez ses clés pour un affichage stable.

## Pour les plus courageux

### Exercice 10 : Des parenthèses bien fermées

Niveau : ★★★

Écrivez `equilibre(s string) bool`. Elle vérifie que chaque `(`, `[` ou `{` est fermé
par le bon signe, dans le bon ordre. Testez `"(a[b]{c})"`, `"(]"` et `"(("`.

Sortie attendue :

```text
"(a[b]{c})" : true
"(]" : false
"((" : false
```

> **Indice :** utilisez une **pile** : une slice où l'on ajoute et retire toujours à la
> fin. Un signe ouvrant est empilé. Un signe fermant doit correspondre au dernier
> signe empilé, qu'on retire alors. À la fin, la pile doit être vide.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func equilibre(s string) bool {
> 	paires := map[rune]rune{')': '(', ']': '[', '}': '{'}
> 	var pile []rune
> 	for _, c := range s {
> 		switch c {
> 		case '(', '[', '{':
> 			pile = append(pile, c)
> 		case ')', ']', '}':
> 			if len(pile) == 0 || pile[len(pile)-1] != paires[c] {
> 				return false
> 			}
> 			pile = pile[:len(pile)-1]
> 		}
> 	}
> 	return len(pile) == 0
> }
>
> func main() {
> 	for _, s := range []string{"(a[b]{c})", "(]", "(("} {
> 		fmt.Printf("%q : %t\n", s, equilibre(s))
> 	}
> }
> ```
>
> ```sortie
> "(a[b]{c})" : true
> "(]" : false
> "((" : false
> ```
>
> Pourquoi ça marche : la pile est une pile d'assiettes. On ne peut prendre que celle
> du dessus. Pour `"(a[b]{c})"` (les lettres sont ignorées) :
>
> | Signe lu | Action | Pile après |
> |---|---|---|
> | `(` | empiler | `(` |
> | `[` | empiler | `( [` |
> | `]` | le dessus est `[` : on retire | `(` |
> | `{` | empiler | `( {` |
> | `}` | le dessus est `{` : on retire | `(` |
> | `)` | le dessus est `(` : on retire | (vide) |
>
> Pile vide à la fin : `true`. Pour `"(("`, il reste deux `(` : `false`.

### Exercice 11 : La recherche par moitiés

Niveau : ★★★

La slice `[]int{3, 8, 15, 21, 34, 42, 57, 63}` est triée. Écrivez
`chercher(s []int, cible int) (int, int)` qui renvoie la position de `cible` (ou -1)
et le nombre d'essais. À chaque essai, regardez le milieu et jetez la moitié inutile.
Cherchez 42 puis 10.

Sortie attendue :

```text
42 : position 5 (2 essais)
10 : absent (3 essais)
```

> **Indice :** gardez deux bornes, `bas` et `haut`. Le milieu est `(bas + haut) / 2`.
> Si la valeur du milieu est trop petite, `bas = milieu + 1` ; trop grande,
> `haut = milieu - 1`. On s'arrête quand `bas > haut`.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func chercher(s []int, cible int) (int, int) {
> 	bas, haut := 0, len(s)-1
> 	essais := 0
> 	for bas <= haut {
> 		essais++
> 		milieu := (bas + haut) / 2
> 		if s[milieu] == cible {
> 			return milieu, essais
> 		} else if s[milieu] < cible {
> 			bas = milieu + 1
> 		} else {
> 			haut = milieu - 1
> 		}
> 	}
> 	return -1, essais
> }
>
> func main() {
> 	nombres := []int{3, 8, 15, 21, 34, 42, 57, 63}
> 	for _, cible := range []int{42, 10} {
> 		pos, essais := chercher(nombres, cible)
> 		if pos == -1 {
> 			fmt.Printf("%d : absent (%d essais)\n", cible, essais)
> 		} else {
> 			fmt.Printf("%d : position %d (%d essais)\n", cible, pos, essais)
> 		}
> 	}
> }
> ```
>
> ```sortie
> 42 : position 5 (2 essais)
> 10 : absent (3 essais)
> ```
>
> Pourquoi ça marche : c'est le jeu « plus grand, plus petit ». On ouvre un
> dictionnaire au milieu, et on garde seulement la moitié où le mot peut être. Pour 10 :
>
> | Essai | bas | haut | milieu | s[milieu] | Décision |
> |---|---|---|---|---|---|
> | 1 | 0 | 7 | 3 | 21 | trop grand : haut = 2 |
> | 2 | 0 | 2 | 1 | 8 | trop petit : bas = 2 |
> | 3 | 2 | 2 | 2 | 15 | trop grand : haut = 1 |
> | fin | 2 | 1 | — | — | bas > haut : absent |
>
> ```text
>  [ 3 | 8 | 15 | 21 | 34 | 42 | 57 | 63 ]   essai 1 : milieu = 21
>  [ 3 | 8 | 15 ]                             essai 2 : milieu = 8
>          [ 15 ]                             essai 3 : milieu = 15
> ```
>
> Avec 1000 nombres, il faut au plus 10 essais, au lieu de 1000.

### Exercice 12 : Les chiffres romains

Niveau : ★★★

Écrivez `romain(n int) string` qui écrit un nombre en chiffres romains. Testez 1994,
2026 et 49.

Sortie attendue :

```text
1994 = MCMXCIV
2026 = MMXXVI
49 = XLIX
```

> **Indice :** préparez deux slices côte à côte, du plus grand au plus petit, **avec**
> les cas spéciaux : `1000 M, 900 CM, 500 D, 400 CD, 100 C, 90 XC, 50 L, 40 XL, 10 X,
> 9 IX, 5 V, 4 IV, 1 I`. Tant que `n` est assez grand pour la valeur, écrivez son
> symbole et soustrayez-la.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func romain(n int) string {
> 	valeurs := []int{1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1}
> 	symboles := []string{"M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"}
> 	resultat := ""
> 	for i, v := range valeurs {
> 		for n >= v {
> 			resultat += symboles[i]
> 			n -= v
> 		}
> 	}
> 	return resultat
> }
>
> func main() {
> 	for _, n := range []int{1994, 2026, 49} {
> 		fmt.Println(n, "=", romain(n))
> 	}
> }
> ```
>
> ```sortie
> 1994 = MCMXCIV
> 2026 = MMXXVI
> 49 = XLIX
> ```
>
> Pourquoi ça marche : c'est rendre la monnaie avec les plus grosses pièces d'abord.
> Pour 1994 (les valeurs trop grandes sont sautées) :
>
> | Valeur | n avant | Symbole ajouté | n après | resultat |
> |---|---|---|---|---|
> | 1000 | 1994 | M | 994 | M |
> | 900 | 994 | CM | 94 | MCM |
> | 90 | 94 | XC | 4 | MCMXC |
> | 4 | 4 | IV | 0 | MCMXCIV |
>
> Mettre 900, 400, 90… dans la liste évite d'écrire `DCCCC` au lieu de `CM`.

> **À retenir :** pour un problème plus dur, trouvez la bonne image (une pile
> d'assiettes, un dictionnaire, une caisse de monnaie), puis vérifiez-la avec un
> tableau de suivi sur un petit exemple.

## En résumé

- Les exercices 1 à 4 travaillent `/`, `%`, `range` sur un texte et les boucles
  imbriquées.
- Les exercices 5 à 9 utilisent des fonctions, l'échange `a, b = b, a+b`, les maps et
  les slices.
- Les exercices 10 à 12 demandent une méthode : une pile, la recherche par moitiés, la
  monnaie gloutonne.
- Un tableau de suivi sur un petit exemple vaut mieux que dix relectures du code.

Suite : [L'aide-mémoire](aide-memoire.md)
