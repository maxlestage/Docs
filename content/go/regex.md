# Les expressions régulières

Cette page explique comment décrire un motif de texte avec le paquet `regexp`, pour vérifier, chercher, extraire ou remplacer des morceaux de texte.

> **Objectifs :**
>
> - vous saurez compiler une expression régulière avec `regexp.MustCompile` ;
> - vous saurez tester un texte avec `MatchString` et chercher avec `FindString` et `FindAllString` ;
> - vous saurez extraire des morceaux avec les groupes, numérotés ou nommés ;
> - vous saurez remplacer du texte avec `ReplaceAllString` ;
> - vous saurez lire les pièces les plus courantes : `\d`, `\w`, `+`, `*`, `^`, `$`…

## Qu'est-ce qu'une expression régulière ?

Une [expression régulière](../debuter/glossaire.md#expression-reguliere) (en anglais
*regular expression*, souvent abrégé *regex*) décrit une **forme** de texte plutôt
qu'un texte précis. « Cinq chiffres » est une forme : `69001` la respecte, `Lyon` non.

C'est comme un avis de recherche : il ne donne pas le nom de la personne, mais sa
description. Go parcourt le texte et cherche ce qui correspond.

En Go, une regex est d'abord **compilée** : le texte du motif est transformé en un
objet `*regexp.Regexp`, prêt à chercher. `regexp.MustCompile` fait ce travail.

```go
package main

import (
	"fmt"
	"regexp"
)

func main() {
	chat := regexp.MustCompile(`chat`)
	fmt.Println(chat.MatchString("Le chat dort"))
	fmt.Println(chat.MatchString("Le chien dort"))
}
```

```sortie
true
false
```

`MatchString` répond seulement `true` ou `false` : le motif apparaît-il quelque part
dans le texte ?

## Écrire le motif entre accents graves

Les regex utilisent beaucoup la barre oblique inverse `\` : `\d` veut dire « un
chiffre ». Or, entre guillemets doubles, Go interprète déjà `\` (comme dans `"\n"`).

Écrivez donc toujours vos motifs entre **accents graves** : c'est une chaîne brute
(*raw string*), où `\` reste un simple caractère.

```go
package main

import "fmt"

func main() {
	fmt.Println(`\d+`)
	fmt.Println("\\d+")
}
```

```sortie
\d+
\d+
```

Les deux écrivent le même motif, mais la version brute se lit bien mieux.

## Les pièces de base

Une regex se construit avec des pièces. Une lettre ordinaire se représente elle-même.
Certains caractères ont un sens spécial.

| Pièce | Signification | Exemple qui correspond |
|-------|---------------|------------------------|
| `\d` | un chiffre | `7` |
| `\w` | une lettre ASCII, un chiffre ou `_` | `a`, `Z`, `4` |
| `\s` | un espace (ou tabulation, retour à la ligne) | ` ` |
| `.` | n'importe quel caractère (sauf retour à la ligne) | `x`, `!` |
| `[aeiou]` | un caractère de la liste | `e` |
| `[a-z]` | un caractère de l'intervalle | `m` |
| `[^0-9]` | un caractère **hors** de la liste | `k` |
| `\.` | un vrai point (le `\` annule le sens spécial) | `.` |

Après une pièce, un **quantificateur** dit combien de fois elle se répète.

| Quantificateur | Combien de fois |
|----------------|-----------------|
| `+` | une fois ou plus |
| `*` | zéro fois ou plus |
| `?` | zéro ou une fois |
| `{3}` | exactement 3 fois |
| `{2,4}` | de 2 à 4 fois |

Enfin, `^` marque le début du texte et `$` sa fin. Avec les deux, tout le texte doit
correspondre, pas seulement un morceau.

```go
package main

import (
	"fmt"
	"regexp"
)

func main() {
	codePostal := regexp.MustCompile(`^\d{5}$`)
	for _, texte := range []string{"69001", "6900", "690012", "69 001"} {
		fmt.Println(texte, "->", codePostal.MatchString(texte))
	}
}
```

```sortie
69001 -> true
6900 -> false
690012 -> false
69 001 -> false
```

> **Attention :** `\w` ne reconnaît que les lettres **ASCII** : `é` n'en fait pas
> partie. Pour toutes les lettres, accents compris, écrivez `\p{L}`.

## Chercher : FindString et FindAllString

`FindString` renvoie le **premier** morceau qui correspond, ou `""` s'il n'y en a pas.
`FindAllString` les renvoie **tous** dans une tranche ; son second argument est le
nombre maximum de résultats, `-1` voulant dire « sans limite ».

```go
package main

import (
	"fmt"
	"regexp"
)

func main() {
	nombre := regexp.MustCompile(`\d+`)
	texte := "3 pommes, 12 poires et 7 kiwis"

	fmt.Printf("%q\n", nombre.FindString(texte))
	fmt.Println(nombre.FindAllString(texte, -1))
	fmt.Println(nombre.FindAllString(texte, 2))
	fmt.Printf("%q\n", nombre.FindString("aucun chiffre"))
}
```

```sortie
"3"
[3 12 7]
[3 12]
""
```

`%q` affiche le texte entre guillemets : on voit bien le texte vide `""`.

## Extraire avec les groupes

Des parenthèses forment un **groupe** : elles capturent une partie du résultat.
`FindStringSubmatch` renvoie une tranche : la case 0 contient tout le morceau trouvé,
puis une case par groupe, dans l'ordre. Sans résultat, la tranche vaut `nil`.

```go
package main

import (
	"fmt"
	"regexp"
)

func main() {
	date := regexp.MustCompile(`(\d{2})/(\d{2})/(\d{4})`)

	m := date.FindStringSubmatch("Rendez-vous le 18/10/2026 à midi")
	if m == nil {
		fmt.Println("Pas de date.")
		return
	}
	fmt.Println("Tout :", m[0])
	fmt.Println("Jour :", m[1])
	fmt.Println("Mois :", m[2])
	fmt.Println("Année :", m[3])
}
```

```sortie
Tout : 18/10/2026
Jour : 18
Mois : 10
Année : 2026
```

### Les groupes nommés

Compter les parenthèses devient vite pénible. Un **groupe nommé** s'écrit
`(?P<nom>...)`. `SubexpIndex("nom")` donne ensuite le numéro de sa case.

```go
package main

import (
	"fmt"
	"regexp"
)

func main() {
	email := regexp.MustCompile(`(?P<utilisateur>[\w.]+)@(?P<domaine>[\w.]+)`)

	m := email.FindStringSubmatch("Écrivez à lea.martin@exemple.fr demain")
	fmt.Println("Utilisateur :", m[email.SubexpIndex("utilisateur")])
	fmt.Println("Domaine :", m[email.SubexpIndex("domaine")])
}
```

```sortie
Utilisateur : lea.martin
Domaine : exemple.fr
```

### Tous les résultats avec leurs groupes

`FindAllStringSubmatch` combine les deux : une tranche de résultats, chacun avec ses
groupes.

```go
package main

import (
	"fmt"
	"regexp"
)

func main() {
	reglage := regexp.MustCompile(`(\w+)=(\w+)`)
	texte := "couleur=bleu taille=42 mode=sombre"

	for _, m := range reglage.FindAllStringSubmatch(texte, -1) {
		fmt.Printf("%s vaut %s\n", m[1], m[2])
	}
}
```

```sortie
couleur vaut bleu
taille vaut 42
mode vaut sombre
```

## Remplacer avec ReplaceAllString

`ReplaceAllString` remplace chaque morceau trouvé. Dans le texte de remplacement,
`${1}` désigne le groupe 1 et `${nom}` un groupe nommé.

```go
package main

import (
	"fmt"
	"regexp"
)

func main() {
	espaces := regexp.MustCompile(`\s+`)
	fmt.Println(espaces.ReplaceAllString("trop    d'espaces   ici", " "))

	date := regexp.MustCompile(`(?P<j>\d{2})/(?P<m>\d{2})/(?P<a>\d{4})`)
	fmt.Println(date.ReplaceAllString("Né le 05/03/1990", "${a}-${m}-${j}"))

	telephone := regexp.MustCompile(`\d{2}(\d{2})(\d{2})(\d{2})(\d{2})`)
	fmt.Println(telephone.ReplaceAllString("Tél : 0612345678", "06 ${1} ${2} ${3} ${4}"))
}
```

```sortie
trop d'espaces ici
Né le 1990-03-05
Tél : 06 12 34 56 78
```

> **Astuce :** écrivez toujours `${1}` plutôt que `$1`. Dans `$1x`, Go chercherait un
> groupe nommé `1x`, qui n'existe pas, et mettrait du vide à la place.

Pour un texte de remplacement qui ne doit pas interpréter les `$`, utilisez
`ReplaceAllLiteralString`.

## Valider une saisie

Un usage très courant : vérifier qu'une saisie a la bonne forme avant de l'utiliser.
Compilez la regex **une seule fois**, en dehors de la fonction, dans une variable du
paquet : la compilation a un coût.

```go
package main

import (
	"fmt"
	"regexp"
)

var motifPseudo = regexp.MustCompile(`^[a-z][a-z0-9_]{2,11}$`)

func pseudoValide(p string) bool {
	return motifPseudo.MatchString(p)
}

func main() {
	for _, p := range []string{"lea_92", "92lea", "lo", "super_pseudo_trop_long"} {
		fmt.Printf("%-24s %t\n", p, pseudoValide(p))
	}
}
```

```sortie
lea_92                   true
92lea                    false
lo                       false
super_pseudo_trop_long   false
```

Le motif dit : une lettre minuscule, puis 2 à 11 lettres, chiffres ou `_`.

## Ce que Go ne sait pas faire

Le paquet `regexp` utilise la syntaxe **RE2**. Elle garantit une recherche toujours
rapide, quel que soit le texte. En échange, quelques pièces d'autres langages manquent,
notamment les **références arrière** comme `\1` (« le même texte que le groupe 1 »).

Pour trouver une lettre doublée, faites-le sans regex, avec une boucle :

```go
package main

import "fmt"

func main() {
	mot := "aller"
	lettres := []rune(mot)
	for i := 1; i < len(lettres); i++ {
		if lettres[i] == lettres[i-1] {
			fmt.Printf("Lettre doublée : %c\n", lettres[i])
		}
	}
}
```

```sortie
Lettre doublée : l
```

> **À retenir :** une regex est parfaite pour reconnaître une forme simple. Pour une
> logique compliquée, une boucle ou les fonctions du paquet `strings` (voir
> [le texte](texte.md)) restent souvent plus claires.

## Antisèche

| Méthode | Renvoie |
|---------|---------|
| `regexp.MustCompile(motif)` | la regex compilée (panique si le motif est faux) |
| `regexp.Compile(motif)` | la regex et une erreur |
| `re.MatchString(s)` | `true` ou `false` |
| `re.FindString(s)` | le premier morceau, ou `""` |
| `re.FindAllString(s, -1)` | tous les morceaux |
| `re.FindStringSubmatch(s)` | `[tout, groupe1, groupe2…]`, ou `nil` |
| `re.FindAllStringSubmatch(s, -1)` | tous les résultats avec leurs groupes |
| `re.SubexpIndex("nom")` | le numéro du groupe nommé |
| `re.ReplaceAllString(s, remplacement)` | le texte remplacé (`${1}`, `${nom}`) |
| `re.Split(s, -1)` | le texte découpé à chaque correspondance |

## Erreurs fréquentes

### Écrire le motif entre guillemets doubles

```go norun
package main

import (
	"fmt"
	"regexp"
)

func main() {
	re := regexp.MustCompile("\d+")
	fmt.Println(re.FindString("rue 42"))
}
```

```text
# command-line-arguments
./main.go:9:29: unknown escape
```

Entre guillemets doubles, `\d` n'est pas une séquence connue de Go. Écrivez le motif
entre accents graves : `` regexp.MustCompile(`\d+`) ``.

### Utiliser une référence arrière

```go norun
doublon := regexp.MustCompile(`(\w)\1`)
fmt.Println(doublon.MatchString("aller"))
```

```text
panic: regexp: Compile(`(\w)\1`): error parsing regexp: invalid escape sequence: `\1`

goroutine 1 [running]:
regexp.MustCompile({0x4d7e92, 0x6})
	/usr/local/go1.24.7/src/regexp/regexp.go:313 +0xb4
main.main()
	…/main.go:9 +0x25
exit status 2
```

`MustCompile` **panique** quand le motif est invalide : le programme s'arrête. RE2 ne
connaît pas `\1`. Faites la vérification avec une boucle, comme plus haut.

> **Note :** si le motif vient de l'utilisateur, utilisez `regexp.Compile`, qui
> renvoie une erreur au lieu de paniquer.

### Oublier ^ et $

```go norun
codePostal := regexp.MustCompile(`\d{5}`)
fmt.Println(codePostal.MatchString("123456"))
fmt.Println(codePostal.MatchString("tel 0612345678"))
```

```text
true
true
```

Sans ancres, `MatchString` accepte un texte qui **contient** cinq chiffres quelque
part. Pour exiger que tout le texte ait la forme, écrivez `` `^\d{5}$` ``.

## Testez-vous

**Question 1 :** que renvoie `FindAllString(texte, -1)` ? Et que veut dire `-1` ?

> **Réponse**
>
> Une tranche avec tous les morceaux qui correspondent. `-1` veut dire « sans limite
> de nombre ».

**Question 2 :** pourquoi écrire les motifs entre accents graves ?

> **Réponse**
>
> Ce sont des chaînes brutes : la barre oblique inverse `\` n'y est pas interprétée par
> Go. `\d` arrive donc tel quel au paquet `regexp`.

**Question 3 :** extrayez toutes les heures (forme `14h30`) du texte
`"Départ 08h15, arrivée 11h40"`.

> **Réponse**
>
> ```go
> package main
>
> import (
> 	"fmt"
> 	"regexp"
> )
>
> func main() {
> 	heure := regexp.MustCompile(`\d{2}h\d{2}`)
> 	fmt.Println(heure.FindAllString("Départ 08h15, arrivée 11h40", -1))
> }
> ```
>
> ```sortie
> [08h15 11h40]
> ```

## En résumé

- `regexp.MustCompile` compile un motif ; écrivez-le entre accents graves.
- `MatchString` teste, `FindString` et `FindAllString` cherchent.
- Les parenthèses capturent : `FindStringSubmatch` renvoie les groupes, `(?P<nom>…)`
  les nomme.
- `ReplaceAllString` remplace, avec `${1}` ou `${nom}` dans le remplacement.
- `^` et `$` imposent que tout le texte corresponde ; RE2 n'a pas de références arrière.

Suite : [dates et heures](dates.md). Comparez avec
[les expressions régulières en Ruby](../ruby/regex.md).
