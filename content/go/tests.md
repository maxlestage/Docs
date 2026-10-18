# Tester son code

Cette page montre comment écrire des tests automatiques avec le paquet `testing` et les lancer avec `go test`, des tests les plus simples aux tests en tableau.

> **Objectifs :**
>
> - vous saurez écrire un fichier `_test.go` et une fonction `TestX(t *testing.T)` ;
> - vous saurez signaler un échec avec `t.Errorf` et `t.Fatalf` ;
> - vous saurez écrire des tests en tableau, avec des sous-tests `t.Run` ;
> - vous saurez lancer les tests avec `go test` et lire leur résultat ;
> - vous saurez ce que sont les benchmarks et les exemples testés.

## Pourquoi tester ?

Un [test](../debuter/glossaire.md#test) est un petit programme qui vérifie qu'un autre
programme fait bien ce qu'il doit. Vous l'écrivez une fois, puis vous le relancez à
chaque modification, en une seconde.

C'est comme la liste de contrôle d'un pilote avant le décollage : on vérifie
toujours les mêmes points, dans le même ordre, sans se fier à sa mémoire.

Sans outil, on vérifierait à la main, avec des `fmt.Println` :

```go
package main

import "fmt"

func Somme(nombres []int) int {
	total := 0
	for _, n := range nombres {
		total += n
	}
	return total
}

func main() {
	if Somme([]int{1, 2, 3}) != 6 {
		fmt.Println("ÉCHEC : la somme de 1, 2, 3 devrait faire 6")
	} else {
		fmt.Println("OK")
	}
}
```

```sortie
OK
```

Cela marche, mais il faut lire la sortie et mélanger les vérifications avec le
programme. Go fournit un outil fait pour cela : le paquet `testing` et la commande
`go test`.

## Le projet à tester

Les tests vivent dans un **module** (voir [les modules](modules.md)). Créons un dossier
`calcul` :

```sh
mkdir calcul
cd calcul
go mod init calcul
```

Puis le fichier `calcul.go`, avec deux fonctions :

```go norun
// Package calcul regroupe de petites fonctions de calcul.
package calcul

import "errors"

// Somme renvoie la somme des nombres.
func Somme(nombres []int) int {
	total := 0
	for _, n := range nombres {
		total += n
	}
	return total
}

// Maximum renvoie le plus grand nombre de la tranche.
func Maximum(nombres []int) (int, error) {
	if len(nombres) == 0 {
		return 0, errors.New("tranche vide")
	}
	plusGrand := nombres[0]
	for _, n := range nombres[1:] {
		if n > plusGrand {
			plusGrand = n
		}
	}
	return plusGrand, nil
}
```

Ce paquet n'a pas de `main` : il ne s'exécute pas seul. Ce sont les tests qui vont
l'appeler. C'est pourquoi les blocs de cette page sont marqués `norun` : ils forment
un projet de plusieurs fichiers.

## Un premier test

Les règles de Go sont simples :

- les tests sont dans un fichier dont le nom finit par **`_test.go`**, à côté du code :
  ici `calcul_test.go` ;
- ce fichier est dans le **même paquet** (`package calcul`) ;
- chaque test est une fonction dont le nom commence par **`Test`** suivi d'une
  majuscule, avec un seul paramètre `t *testing.T`.

```go norun
package calcul

import "testing"

func TestSomme(t *testing.T) {
	resultat := Somme([]int{1, 2, 3})
	if resultat != 6 {
		t.Errorf("Somme([1 2 3]) = %d, attendu 6", resultat)
	}
}
```

Si le résultat n'est pas le bon, `t.Errorf` signale l'échec avec un message, comme
`fmt.Printf`. Sinon, le test ne dit rien : il réussit.

Lancez les tests depuis le dossier du module :

```sh
go test
```

```text
PASS
ok  	calcul	0.003s
```

`PASS` : tous les tests sont passés. Le temps à la fin varie d'une fois à l'autre.

> **À retenir :** un bon message d'échec dit ce qui a été appelé, ce qui a été obtenu
> et ce qui était attendu. Vous gagnerez du temps le jour où il s'affichera.

## Errorf ou Fatalf ?

`t` propose deux familles pour signaler un problème :

| Méthode | Effet |
|---------|-------|
| `t.Errorf(…)` / `t.Error(…)` | note l'échec et **continue** le test |
| `t.Fatalf(…)` / `t.Fatal(…)` | note l'échec et **arrête** ce test tout de suite |

Utilisez `t.Fatalf` quand la suite n'a plus de sens, par exemple si une fonction
renvoie une erreur inattendue : inutile de vérifier une valeur qui n'existe pas.

```go norun
func TestMaximumVide(t *testing.T) {
	_, err := Maximum([]int{})
	if err == nil {
		t.Error("une tranche vide devrait renvoyer une erreur")
	}
}
```

Ce test vérifie que l'**erreur** est bien renvoyée : tester les cas d'erreur est aussi
important que tester les cas normaux (voir [les erreurs](erreurs.md)).

## Les tests en tableau

Pour essayer plusieurs entrées, on ne copie pas le test dix fois. On écrit un
**tableau de cas** : une tranche de structures anonymes, chacune avec un nom, une
entrée et le résultat attendu. Puis une boucle les vérifie un par un. C'est la façon
la plus courante d'écrire des tests en Go.

`t.Run(nom, fonction)` crée un **sous-test** pour chaque cas : en cas d'échec, Go dit
lequel a raté.

```go norun
func TestMaximum(t *testing.T) {
	cas := []struct {
		nom     string
		entree  []int
		attendu int
	}{
		{"positifs", []int{3, 8, 1}, 8},
		{"négatifs", []int{-5, -2, -9}, -2},
		{"un seul", []int{4}, 4},
	}
	for _, c := range cas {
		t.Run(c.nom, func(t *testing.T) {
			obtenu, err := Maximum(c.entree)
			if err != nil {
				t.Fatalf("erreur inattendue : %v", err)
			}
			if obtenu != c.attendu {
				t.Errorf("Maximum(%v) = %d, attendu %d", c.entree, obtenu, c.attendu)
			}
		})
	}
}
```

Ajouter un cas, c'est ajouter une ligne au tableau.

## Lire le résultat de go test -v

L'option `-v` (*verbose*, « bavard ») affiche chaque test et chaque sous-test :

```sh
go test -v
```

```text
=== RUN   TestSomme
--- PASS: TestSomme (0.00s)
=== RUN   TestMaximum
=== RUN   TestMaximum/positifs
=== RUN   TestMaximum/négatifs
=== RUN   TestMaximum/un_seul
--- PASS: TestMaximum (0.00s)
    --- PASS: TestMaximum/positifs (0.00s)
    --- PASS: TestMaximum/négatifs (0.00s)
    --- PASS: TestMaximum/un_seul (0.00s)
=== RUN   TestMaximumVide
--- PASS: TestMaximumVide (0.00s)
PASS
ok  	calcul	0.006s
```

Les espaces des noms de sous-tests deviennent des `_` : `un seul` → `un_seul`.

### Quand un test échoue

Introduisons un bug : dans `Maximum`, on part de `0` au lieu du premier nombre
(`plusGrand := 0`). Pour des nombres négatifs, le résultat devient faux :

```text
--- FAIL: TestMaximum (0.00s)
    --- FAIL: TestMaximum/négatifs (0.00s)
        calcul_test.go:29: Maximum([-5 -2 -9]) = 0, attendu -2
FAIL
exit status 1
FAIL	calcul	0.003s
```

Le rapport donne tout ce qu'il faut : le sous-test (`négatifs`), le fichier et la ligne
(`calcul_test.go:29`), puis votre message. Les autres cas sont passés : sans `-v`, Go
n'affiche que les échecs.

### Choisir les tests à lancer

| Commande | Effet |
|----------|-------|
| `go test` | lance les tests du dossier courant |
| `go test ./...` | lance les tests de tous les paquets du module |
| `go test -v` | affiche chaque test |
| `go test -run Maximum` | lance seulement les tests dont le nom contient `Maximum` |
| `go test -cover` | affiche la part du code exécutée par les tests |

```sh
go test -cover
```

```text
PASS
coverage: 100.0% of statements
ok  	calcul	0.005s
```

## Les exemples testés

Une fonction dont le nom commence par `Example` sert à la fois de **documentation** et
de test. Le commentaire `// Output:` donne la sortie attendue : `go test` exécute
l'exemple et compare. On les range souvent dans `exemple_test.go`.

```go norun
package calcul

import "fmt"

func ExampleSomme() {
	fmt.Println(Somme([]int{10, 20, 12}))
	// Output: 42
}

func ExampleMaximum() {
	plusGrand, err := Maximum([]int{3, 8, 1})
	fmt.Println(plusGrand, err)
	// Output: 8 <nil>
}
```

```text
=== RUN   ExampleSomme
--- PASS: ExampleSomme (0.00s)
=== RUN   ExampleMaximum
--- PASS: ExampleMaximum (0.00s)
PASS
ok  	calcul	0.003s
```

Ces exemples apparaissent dans la documentation du paquet, et ils ne peuvent pas
devenir faux sans que les tests le signalent. C'est d'ailleurs le principe des
blocs `sortie` de ce site.

## Les benchmarks

Un **benchmark** mesure la vitesse d'une fonction. Son nom commence par `Benchmark` et
il reçoit `b *testing.B`. La boucle `for b.Loop()` répète l'appel autant de fois que
nécessaire pour obtenir une mesure fiable.

```go norun
func BenchmarkSomme(b *testing.B) {
	nombres := make([]int, 1000)
	for i := range nombres {
		nombres[i] = i
	}
	for b.Loop() {
		Somme(nombres)
	}
}
```

```sh
go test -bench .
```

```text
goos: linux
goarch: amd64
pkg: calcul
cpu: Intel(R) Xeon(R) Processor @ 2.10GHz
BenchmarkSomme-4   	 1617744	       725.0 ns/op
PASS
ok  	calcul	1.179s
```

La fonction a été appelée 1 617 744 fois, à environ 725 nanosecondes par appel. Ces
nombres dépendent de votre machine : comparez-les seulement entre deux versions de
votre code, sur le même ordinateur.

> **Note :** `b.Loop()` existe depuis Go 1.24. Dans du code plus ancien, vous verrez
> `for i := 0; i < b.N; i++`.

## Erreurs fréquentes

### Un fichier mal nommé

```go norun
// fichier calcul_tests.go (avec un « s »)
package calcul

import "testing"

func TestSomme(t *testing.T) {
	if Somme([]int{1, 2, 3}) != 6 {
		t.Error("mauvaise somme")
	}
}
```

```text
?   	calcul	[no test files]
```

Go ne cherche les tests que dans les fichiers qui finissent **exactement** par
`_test.go`. Renommez le fichier en `calcul_test.go`.

### Un nom de test en minuscule

```go norun
func testSomme(t *testing.T) {
	if Somme([]int{1, 2, 3}) != 6 {
		t.Error("mauvaise somme")
	}
}
```

```text
testing: warning: no tests to run
PASS
ok  	calcul	0.003s
```

Piège : `PASS` s'affiche, mais aucun test n'a tourné. Le nom doit commencer par `Test`
avec une majuscule : `TestSomme`.

### Oublier l'étoile de *testing.T

```go norun
func TestSomme(t testing.T) {
	if Somme([]int{1, 2, 3}) != 6 {
		t.Error("mauvaise somme")
	}
}
```

```text
# calcul
…/calcul/calcul_test.go:5:1: wrong signature for TestSomme, must be: func TestSomme(t *testing.T)
FAIL	calcul [setup failed]
```

Le message donne la bonne forme : le paramètre est un **pointeur**, `t *testing.T`.

### Une sortie d'exemple fausse

```go norun
func ExampleSomme() {
	fmt.Println(Somme([]int{10, 20, 12}))
	// Output: 43
}
```

```text
--- FAIL: ExampleSomme (0.00s)
got:
42
want:
43
FAIL
exit status 1
FAIL	calcul	0.002s
```

`got` est ce que l'exemple a affiché, `want` ce qu'annonce le commentaire. Corrigez
celui qui a tort (ici, le commentaire).

## Testez-vous

**Question 1 :** quelles conditions doit remplir une fonction pour que `go test` la
lance comme un test ?

> **Réponse**
>
> Elle doit être dans un fichier qui finit par `_test.go`, s'appeler `TestQuelqueChose`
> (avec une majuscule après `Test`) et avoir un seul paramètre `t *testing.T`.

**Question 2 :** quelle différence entre `t.Errorf` et `t.Fatalf` ?

> **Réponse**
>
> `t.Errorf` signale l'échec et laisse le test continuer. `t.Fatalf` signale l'échec
> et arrête ce test immédiatement.

**Question 3 :** écrivez un test en tableau `TestSomme` avec trois cas : une tranche
vide (0), `[5]` (5) et `[1, 2, 3]` (6).

> **Réponse**
>
> ```go norun
> func TestSomme(t *testing.T) {
> 	cas := []struct {
> 		nom     string
> 		entree  []int
> 		attendu int
> 	}{
> 		{"vide", []int{}, 0},
> 		{"un nombre", []int{5}, 5},
> 		{"trois nombres", []int{1, 2, 3}, 6},
> 	}
> 	for _, c := range cas {
> 		t.Run(c.nom, func(t *testing.T) {
> 			if obtenu := Somme(c.entree); obtenu != c.attendu {
> 				t.Errorf("Somme(%v) = %d, attendu %d", c.entree, obtenu, c.attendu)
> 			}
> 		})
> 	}
> }
> ```

## En résumé

- Les tests vont dans des fichiers `_test.go`, dans le même paquet que le code.
- Un test s'écrit `func TestNom(t *testing.T)` ; `t.Errorf` continue, `t.Fatalf`
  arrête.
- Les tests en tableau avec `t.Run` vérifient de nombreux cas en peu de lignes.
- `go test` lance les tests, `-v` les détaille, `-run` les filtre, `-cover` mesure la
  couverture.
- `ExampleX` documente et teste à la fois ; `BenchmarkX` mesure la vitesse.

Suite : [déboguer un programme](debogage.md). Comparez avec
[les tests en Ruby](../ruby/tests.md).
