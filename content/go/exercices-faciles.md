# Exercices pour bien démarrer

Cette page propose 15 petits exercices très guidés, tous faciles, pour pratiquer les bases de Go : variables, calculs, conditions, boucles, fonctions, textes, slices et maps.

## Comment faire ces exercices

Prenez-les dans l'ordre. Ils vont du plus simple au moins simple. Pour chacun :

1. **Lisez** l'énoncé et la sortie attendue.
2. **Essayez sur papier** : écrivez les étapes en français, comme une recette.
3. **Écrivez** le code dans un fichier `main.go`.
4. **Lancez-le** avec `go run main.go`.
5. **Comparez** ce qui s'affiche avec la sortie attendue, caractère par caractère.
6. Bloqué ? Lisez **le premier indice**. Puis le second. La solution vient en dernier.

```text
 lire → papier → écrire → lancer → comparer
                   ↑                   │
                   └──── pas pareil ───┘
```

> **Astuce :** chaque programme commence par les mêmes lignes. Copiez ce modèle et
> remplissez le `main` :
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	fmt.Println("à vous !")
> }
> ```
>
> ```sortie
> à vous !
> ```

Un message d'erreur ? C'est normal. Lisez [les pièges du débutant](pieges.md) : il y est
sûrement.

## Afficher et calculer

### Exercice 1 : Bonjour

Niveau : ★☆☆

Affichez deux lignes : `Bonjour !` puis `J'apprends Go.`

Sortie attendue :

```text
Bonjour !
J'apprends Go.
```

> **Indice :** un `fmt.Println` par ligne.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	fmt.Println("Bonjour !")
> 	fmt.Println("J'apprends Go.")
> }
> ```
>
> ```sortie
> Bonjour !
> J'apprends Go.
> ```
>
> Pourquoi ça marche : chaque `fmt.Println` affiche son texte puis passe à la ligne.

### Exercice 2 : Ma fiche

Niveau : ★☆☆

Créez une variable `prenom` qui vaut `"Léa"` et une variable `age` qui vaut `11`.
Affichez `Léa a 11 ans.`

Sortie attendue :

```text
Léa a 11 ans.
```

> **Indice :** créez les variables avec `:=`.

> **Indice :** `fmt.Println(prenom, "a", age, "ans.")` met un espace entre chaque morceau…
> mais aussi avant le point ! Essayez plutôt `fmt.Printf` avec `%s` (un texte) et `%d`
> (un entier), sans oublier `\n`.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	prenom := "Léa"
> 	age := 11
> 	fmt.Printf("%s a %d ans.\n", prenom, age)
> }
> ```
>
> ```sortie
> Léa a 11 ans.
> ```
>
> Pourquoi ça marche : `Printf` remplace `%s` par `prenom` et `%d` par `age`, dans
> l'ordre. `\n` passe à la ligne.

### Exercice 3 : Le prix total

Niveau : ★☆☆

Un cahier coûte 3 euros. Vous en achetez 4. Calculez le total dans une variable et
affichez `Total : 12 euros`.

Sortie attendue :

```text
Total : 12 euros
```

> **Indice :** `total := prix * quantite`.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	prix := 3
> 	quantite := 4
> 	total := prix * quantite
> 	fmt.Println("Total :", total, "euros")
> }
> ```
>
> ```sortie
> Total : 12 euros
> ```
>
> Pourquoi ça marche : `*` multiplie. `Println` ajoute un espace entre les morceaux.

### Exercice 4 : Partager des bonbons

Niveau : ★☆☆

17 bonbons pour 5 enfants. Combien chaque enfant en reçoit-il, et combien en reste-t-il ?

Sortie attendue :

```text
Chacun : 3
Reste : 2
```

> **Indice :** entre deux entiers, `/` donne la division entière.

> **Indice :** `%` (modulo) donne le reste de la division.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	bonbons := 17
> 	enfants := 5
> 	fmt.Println("Chacun :", bonbons/enfants)
> 	fmt.Println("Reste :", bonbons%enfants)
> }
> ```
>
> ```sortie
> Chacun : 3
> Reste : 2
> ```
>
> Pourquoi ça marche : 5 × 3 = 15, et 17 − 15 = 2. `/` donne 3, `%` donne 2.

### Exercice 5 : La moyenne

Niveau : ★☆☆

Calculez la moyenne de trois notes : 12, 15 et 14. Affichez-la avec une décimale.

Sortie attendue :

```text
Moyenne : 13.7
```

> **Indice :** 12 + 15 + 14 = 41. Divisé par 3, cela fait 13,66… : il faut un
> `float64`, sinon Go coupe la virgule.

> **Indice :** `float64(somme) / 3`, puis `fmt.Printf("%.1f\n", …)` pour une seule
> décimale (arrondie).

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	somme := 12 + 15 + 14
> 	moyenne := float64(somme) / 3
> 	fmt.Printf("Moyenne : %.1f\n", moyenne)
> }
> ```
>
> ```sortie
> Moyenne : 13.7
> ```
>
> Pourquoi ça marche : la conversion a lieu **avant** la division, donc le calcul garde
> les décimales. `%.1f` arrondit 13,666… à 13,7.

## Conditions et boucles

### Exercice 6 : Majeur ou mineur

Niveau : ★☆☆

Avec `age := 15`, affichez `Mineur`. Avec 18 ou plus, le programme doit afficher
`Majeur`.

Sortie attendue :

```text
Mineur
```

> **Indice :** `if age >= 18 { … } else { … }`.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	age := 15
> 	if age >= 18 {
> 		fmt.Println("Majeur")
> 	} else {
> 		fmt.Println("Mineur")
> 	}
> }
> ```
>
> ```sortie
> Mineur
> ```
>
> Pourquoi ça marche : 15 >= 18 est faux, donc Go exécute le bloc `else`. Essayez avec
> `age := 18`.

### Exercice 7 : Compter jusqu'à 5

Niveau : ★☆☆

Affichez les nombres de 1 à 5, un par ligne.

Sortie attendue :

```text
1
2
3
4
5
```

> **Indice :** `for i := 1; i <= 5; i++ { … }`.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	for i := 1; i <= 5; i++ {
> 		fmt.Println(i)
> 	}
> }
> ```
>
> ```sortie
> 1
> 2
> 3
> 4
> 5
> ```
>
> Pourquoi ça marche : `i` part de 1, la boucle tourne tant que `i <= 5`, et `i++`
> ajoute 1 à chaque tour.

### Exercice 8 : La table de 7

Niveau : ★☆☆

Affichez la table de multiplication de 7, de 1 à 5.

Sortie attendue :

```text
7 x 1 = 7
7 x 2 = 14
7 x 3 = 21
7 x 4 = 28
7 x 5 = 35
```

> **Indice :** reprenez la boucle de l'exercice 7.

> **Indice :** dans la boucle : `fmt.Printf("7 x %d = %d\n", i, 7*i)`.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	for i := 1; i <= 5; i++ {
> 		fmt.Printf("7 x %d = %d\n", i, 7*i)
> 	}
> }
> ```
>
> ```sortie
> 7 x 1 = 7
> 7 x 2 = 14
> 7 x 3 = 21
> 7 x 4 = 28
> 7 x 5 = 35
> ```
>
> Pourquoi ça marche : le premier `%d` reçoit `i`, le second reçoit `7*i`.

### Exercice 9 : Additionner de 1 à 10

Niveau : ★☆☆

Calculez 1 + 2 + 3 + … + 10 avec une boucle.

Sortie attendue :

```text
Somme : 55
```

> **Indice :** créez `total := 0` **avant** la boucle. Dans la boucle, ajoutez `i` à
> `total`.

> **Indice :** suivez les valeurs sur papier :
>
> | Tour | i | total |
> |---|---|---|
> | 1 | 1 | 1 |
> | 2 | 2 | 3 |
> | 3 | 3 | 6 |
> | … | … | … |
> | 10 | 10 | 55 |

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	total := 0
> 	for i := 1; i <= 10; i++ {
> 		total += i
> 	}
> 	fmt.Println("Somme :", total)
> }
> ```
>
> ```sortie
> Somme : 55
> ```
>
> Pourquoi ça marche : `total += i` veut dire `total = total + i`. Attention à ne pas
> écrire `total := …` dans la boucle : ce serait une autre variable.

### Exercice 10 : FizzBuzz jusqu'à 15

Niveau : ★☆☆

Pour chaque nombre de 1 à 15 : affichez `Fizz` s'il est divisible par 3, `Buzz` s'il
est divisible par 5, `FizzBuzz` s'il est divisible par les deux, sinon le nombre.

Sortie attendue :

```text
1
2
Fizz
4
Buzz
Fizz
7
8
Fizz
Buzz
11
Fizz
13
14
FizzBuzz
```

> **Indice :** « divisible par 3 » s'écrit `i%3 == 0`.

> **Indice :** testez d'abord le cas « les deux » (`i%15 == 0`). Sinon, 15 afficherait
> seulement `Fizz`. Un `switch` sans valeur se lit bien ici.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	for i := 1; i <= 15; i++ {
> 		switch {
> 		case i%15 == 0:
> 			fmt.Println("FizzBuzz")
> 		case i%3 == 0:
> 			fmt.Println("Fizz")
> 		case i%5 == 0:
> 			fmt.Println("Buzz")
> 		default:
> 			fmt.Println(i)
> 		}
> 	}
> }
> ```
>
> ```sortie
> 1
> 2
> Fizz
> 4
> Buzz
> Fizz
> 7
> 8
> Fizz
> Buzz
> 11
> Fizz
> 13
> 14
> FizzBuzz
> ```
>
> Pourquoi ça marche : le `switch` prend le **premier** cas vrai. Le cas 15 est placé en
> premier, il passe donc avant `Fizz` et `Buzz`.

## Fonctions et textes

### Exercice 11 : Une fonction carre

Niveau : ★☆☆

Écrivez une fonction `carre` qui reçoit un entier et renvoie son carré. Affichez
`carre(4)` et `carre(9)`.

Sortie attendue :

```text
16
81
```

> **Indice :** `func carre(n int) int { … }`. Le `int` après la parenthèse est le type
> de ce qui est renvoyé.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func carre(n int) int {
> 	return n * n
> }
>
> func main() {
> 	fmt.Println(carre(4))
> 	fmt.Println(carre(9))
> }
> ```
>
> ```sortie
> 16
> 81
> ```
>
> Pourquoi ça marche : `return` renvoie `n * n` à l'endroit de l'appel. `carre(4)` est
> remplacé par 16.

### Exercice 12 : Crier un prénom

Niveau : ★☆☆

Écrivez une fonction `crier` qui reçoit un texte et renvoie ce texte en majuscules, suivi
de `!!!`. Testez-la avec `"bonjour chloé"`.

Sortie attendue :

```text
BONJOUR CHLOÉ!!!
```

> **Indice :** importez `"strings"` en plus de `"fmt"`. `strings.ToUpper` met en
> majuscules.

> **Indice :** collez deux textes avec `+`.

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
> func crier(texte string) string {
> 	return strings.ToUpper(texte) + "!!!"
> }
>
> func main() {
> 	fmt.Println(crier("bonjour chloé"))
> }
> ```
>
> ```sortie
> BONJOUR CHLOÉ!!!
> ```
>
> Pourquoi ça marche : `ToUpper` fabrique un nouveau texte en majuscules (même le `é`),
> puis `+` lui colle `!!!`.

### Exercice 13 : Compter les lettres

Niveau : ★☆☆

Comptez les lettres du mot `"élève"` et affichez `élève : 5 lettres`.

Sortie attendue :

```text
élève : 5 lettres
```

> **Indice :** `len("élève")` donne 7, pas 5 : il compte les octets, et chaque `è` ou `é`
> en prend deux.

> **Indice :** transformez le texte en runes (une rune = une lettre) : `[]rune(mot)`.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	mot := "élève"
> 	fmt.Println(mot, ":", len([]rune(mot)), "lettres")
> }
> ```
>
> ```sortie
> élève : 5 lettres
> ```
>
> Pourquoi ça marche : `[]rune(mot)` découpe le texte en 5 lettres, et `len` les compte.

## Slices et maps

### Exercice 14 : Le plus grand

Niveau : ★☆☆

Trouvez le plus grand nombre de la slice `[]int{4, 17, 9, 23, 8}`, avec une boucle.

Sortie attendue :

```text
Le plus grand : 23
```

> **Indice :** gardez le « champion » dans une variable. Au départ, c'est la première
> case : `max := nombres[0]`.

> **Indice :** parcourez avec `range`. Si un nombre est plus grand que le champion, il
> devient le nouveau champion.
>
> | Nombre lu | Plus grand ? | max |
> |---|---|---|
> | 4 | non | 4 |
> | 17 | oui | 17 |
> | 9 | non | 17 |
> | 23 | oui | 23 |
> | 8 | non | 23 |

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	nombres := []int{4, 17, 9, 23, 8}
> 	plusGrand := nombres[0]
> 	for _, n := range nombres {
> 		if n > plusGrand {
> 			plusGrand = n
> 		}
> 	}
> 	fmt.Println("Le plus grand :", plusGrand)
> }
> ```
>
> ```sortie
> Le plus grand : 23
> ```
>
> Pourquoi ça marche : à chaque tour, le champion est comparé au nombre lu. À la fin,
> il ne reste que le plus grand. (Le nom `plusGrand` évite de cacher la fonction `max`
> de Go.)

### Exercice 15 : Compter les fruits

Niveau : ★☆☆

Voici un panier : `[]string{"pomme", "poire", "pomme", "kiwi", "pomme", "kiwi"}`.
Comptez chaque fruit avec une map, puis affichez les fruits dans l'ordre alphabétique.

Sortie attendue :

```text
kiwi : 2
poire : 1
pomme : 3
```

> **Indice :** `compte := make(map[string]int)`, puis `compte[fruit]++` pour chaque
> fruit. Une clé absente vaut 0 : le premier `++` donne 1.

> **Indice :** l'ordre d'une map est aléatoire. Pour un ordre fixe, parcourez
> `slices.Sorted(maps.Keys(compte))` (importez `"maps"` et `"slices"`).

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
> 	panier := []string{"pomme", "poire", "pomme", "kiwi", "pomme", "kiwi"}
> 	compte := make(map[string]int)
> 	for _, fruit := range panier {
> 		compte[fruit]++
> 	}
> 	for _, fruit := range slices.Sorted(maps.Keys(compte)) {
> 		fmt.Println(fruit, ":", compte[fruit])
> 	}
> }
> ```
>
> ```sortie
> kiwi : 2
> poire : 1
> pomme : 3
> ```
>
> Pourquoi ça marche : la map garde un compteur par fruit. Trier les clés donne
> toujours le même affichage. Revoir [les maps](maps.md#trier-les-cles-avant-d-afficher)
> si besoin.

## Et après ?

Bravo ! Vous avez pratiqué toutes les bases. Pour des exercices un peu plus durs, voyez
[les exercices corrigés](exercices.md).

Suite : [les types](types.md).
