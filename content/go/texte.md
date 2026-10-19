# Travailler avec du texte

Cette page explique comment Go range le texte (des octets et des runes), et comment le découper, le transformer et l'assembler avec les paquets `strings`, `fmt` et `strconv`.

> **Objectifs :**
>
> - vous saurez ce que sont les octets et les runes d'une chaîne ;
> - vous saurez compter les caractères et parcourir un texte avec `range` ;
> - vous saurez utiliser les fonctions courantes du paquet `strings` ;
> - vous saurez assembler du texte avec `strings.Builder` et `fmt.Sprintf` ;
> - vous saurez écrire un texte brut entre accents graves.

## Les chaînes de caractères

Presque tous les programmes manipulent du texte : un prénom, un message, une ligne lue
dans un fichier. En Go, ce texte a son propre type, comme les nombres ont le leur.

Un texte s'appelle une [chaîne de caractères](../debuter/glossaire.md#chaine-de-caracteres),
de type `string`. On l'écrit entre guillemets doubles.

```go
package main

import "fmt"

func main() {
	prenom := "Alice"
	salut := "Bonjour, " + prenom + " !"
	fmt.Println(salut)
	fmt.Println(prenom == "Alice")
}
```

```sortie
Bonjour, Alice !
true
```

Ligne par ligne :

- `prenom := "Alice"` range le texte « Alice » dans la variable `prenom` ;
- la ligne suivante colle trois morceaux : `"Bonjour, "`, le contenu de `prenom`, puis
  `" !"`. `salut` vaut donc `"Bonjour, Alice !"` ;
- `fmt.Println(salut)` affiche ce texte ;
- `prenom == "Alice"` pose une question : « les deux textes sont-ils identiques ? ».
  La réponse est oui, d'où `true`.

- `+` colle deux chaînes bout à bout ;
- `==` compare deux chaînes.

Quelques caractères spéciaux s'écrivent avec une barre oblique inversée `\` :

| Écriture | Sens |
|----------|------|
| `\n` | passer à la ligne |
| `\t` | une tabulation |
| `\"` | un guillemet dans le texte |
| `\\` | la barre `\` elle-même |

```go
package main

import "fmt"

func main() {
	fmt.Println("Ligne 1\nLigne 2")
	fmt.Println("Il a dit : \"Salut\"")
}
```

```sortie
Ligne 1
Ligne 2
Il a dit : "Salut"
```

> **À retenir :** une chaîne Go est **immuable** : on ne peut pas changer une de ses
> lettres. Toutes les fonctions qui « modifient » un texte en créent en fait un nouveau.

**À vous :** écrivez un programme qui range `"Lyon"` dans une variable `ville`, puis
affiche « J'habite à Lyon. » en collant les morceaux avec `+`.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	ville := "Lyon"
> 	fmt.Println("J'habite à " + ville + ".")
> }
> ```
>
> ```sortie
> J'habite à Lyon.
> ```

## Octets et runes

Pourquoi s'en soucier ? Dès qu'un mot contient un accent, compter ses lettres ou
prendre sa première lettre ne marche plus comme vous l'attendez. Savoir comment Go range
le texte vous évite ces surprises.

Une chaîne Go est une suite d'[octets](../debuter/glossaire.md#octet) (des nombres de 0
à 255), encodés en UTF-8. Les lettres simples (`a`, `Z`, `5`) prennent 1 octet. Les
lettres accentuées en prennent 2, et un emoji en prend 4.

Un caractère s'appelle une **rune** en Go. Une rune s'écrit entre apostrophes : `'é'`.

`len` compte les **octets**, pas les caractères. Pour compter les caractères, utilisez
`utf8.RuneCountInString`.

```go
package main

import (
	"fmt"
	"unicode/utf8"
)

func main() {
	mot := "été"
	fmt.Println(len(mot))
	fmt.Println(utf8.RuneCountInString(mot))
	fmt.Println(len("abc"))
}
```

```sortie
5
3
3
```

Ligne par ligne :

- `mot := "été"` range un mot de 3 lettres, dont deux accentuées ;
- `len(mot)` compte les octets : 2 pour chaque `é` et 1 pour le `t`, soit 5 ;
- `utf8.RuneCountInString(mot)` compte les caractères : 3 ;
- `len("abc")` donne 3 : sans accent, un caractère occupe un seul octet.

`"été"` a 3 caractères, mais 5 octets : chaque `é` en occupe 2.

### Lire un octet ou une rune

`mot[i]` donne l'**octet** numéro `i`. Pour obtenir les caractères, convertissez la
chaîne en slice de runes avec `[]rune(mot)`.

```go
package main

import "fmt"

func main() {
	mot := "café"
	fmt.Println(mot[0])
	fmt.Printf("%c\n", mot[0])

	runes := []rune(mot)
	fmt.Println(len(runes))
	fmt.Printf("%c\n", runes[3])
}
```

```sortie
99
c
4
é
```

> **Attention :** `mot[3]` ne donne pas `é`, mais le premier des deux octets de `é`.
> Dès qu'un texte peut contenir des accents, passez par `[]rune` ou par `range`.

### Parcourir un texte avec range

`range` sur une chaîne avance **caractère par caractère**. Il donne la position (en
octets) et la rune.

```go
package main

import "fmt"

func main() {
	for i, c := range "Noël" {
		fmt.Printf("%d %c\n", i, c)
	}
}
```

```sortie
0 N
1 o
2 ë
4 l
```

La position saute de 2 à 4 : `ë` occupe les octets 2 et 3.

**À vous :** écrivez un programme qui parcourt le mot `"vélo"` avec `range` et affiche
chaque lettre sur sa propre ligne.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	for _, c := range "vélo" {
> 		fmt.Printf("%c\n", c)
> 	}
> }
> ```
>
> ```sortie
> v
> é
> l
> o
> ```

## Le paquet strings

Couper une phrase en mots, mettre un texte en majuscules, vérifier qu'une adresse
contient un `@` : ces besoins reviennent sans cesse. Plutôt que de tout réécrire, vous
prenez l'outil tout prêt dans cette boîte à outils.

Le paquet `strings` contient les outils de base pour le texte.

### Chercher

```go
package main

import (
	"fmt"
	"strings"
)

func main() {
	phrase := "Le chat dort sur le canapé"
	fmt.Println(strings.Contains(phrase, "chat"))
	fmt.Println(strings.HasPrefix(phrase, "Le"))
	fmt.Println(strings.HasSuffix(phrase, "lit"))
	fmt.Println(strings.Index(phrase, "dort"))
	fmt.Println(strings.Count(phrase, "a"))
}
```

```sortie
true
true
false
8
3
```

Ligne par ligne :

- `phrase` contient le texte dans lequel on cherche ;
- chaque ligne suivante pose une question sur `phrase` et affiche la réponse ;
- la phrase ne finit pas par « lit », d'où `false` ;
- « dort » commence à la position 8, car on compte à partir de 0 : `L` est en 0, `e`
  en 1, l'espace en 2, et ainsi de suite ;
- la lettre `a` apparaît 3 fois : une dans « chat », deux dans « canapé ».

- `Contains` : le texte contient-il ce morceau ?
- `HasPrefix`, `HasSuffix` : commence-t-il, finit-il par ce morceau ?
- `Index` : la position du morceau (`-1` s'il est absent) ;
- `Count` : combien de fois le morceau apparaît.

### Transformer

```go
package main

import (
	"fmt"
	"strings"
)

func main() {
	fmt.Println(strings.ToUpper("bonjour"))
	fmt.Println(strings.ToLower("ÉCOLE"))
	fmt.Println(strings.Replace("a-b-c", "-", "+", 1))
	fmt.Println(strings.ReplaceAll("a-b-c", "-", "+"))
	fmt.Println(strings.Repeat("ha", 3))
	fmt.Printf("[%s]\n", strings.TrimSpace("   bonjour  \n"))
}
```

```sortie
BONJOUR
école
a+b-c
a+b+c
hahaha
[bonjour]
```

- `Replace(texte, ancien, nouveau, n)` remplace les `n` premières fois ; `ReplaceAll`
  les remplace toutes ;
- `TrimSpace` enlève les espaces et retours à la ligne au début et à la fin. Très utile
  pour nettoyer ce qu'un utilisateur a tapé.

### Découper et recoller

`strings.Split` coupe un texte à chaque séparateur et renvoie une slice de morceaux.
`strings.Join` fait l'inverse.

```go
package main

import (
	"fmt"
	"strings"
)

func main() {
	ligne := "pomme;kiwi;poire"
	fruits := strings.Split(ligne, ";")
	fmt.Println(len(fruits), fruits[1])

	fmt.Println(strings.Join(fruits, ", "))

	mots := strings.Fields("  un   deux\ttrois  ")
	fmt.Println(len(mots), mots)
}
```

```sortie
3 kiwi
pomme, kiwi, poire
3 [un deux trois]
```

`strings.Fields` coupe aux espaces, quel que soit leur nombre. C'est le meilleur choix
pour séparer les mots d'une phrase.

**À vous :** écrivez un programme qui découpe le texte `"lundi,mardi,mercredi"` à
chaque virgule, affiche le nombre de jours, puis les jours en majuscules, séparés par
` - `.

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
> 	jours := strings.Split("lundi,mardi,mercredi", ",")
> 	fmt.Println(len(jours))
> 	fmt.Println(strings.ToUpper(strings.Join(jours, " - ")))
> }
> ```
>
> ```sortie
> 3
> LUNDI - MARDI - MERCREDI
> ```

## Assembler du texte

Un programme doit souvent fabriquer un texte à partir de plusieurs morceaux : un nom,
une note, une date. C'est comme remplir un formulaire : le modèle ne change pas, seules
les cases changent.

### fmt.Sprintf

`fmt.Sprintf` marche comme `Printf`, mais au lieu d'afficher, elle **renvoie** le texte.

```go
package main

import "fmt"

func main() {
	nom := "Alice"
	moyenne := 14.256
	ligne := fmt.Sprintf("%-8s|%6.1f", nom, moyenne)
	fmt.Println(ligne)
	fmt.Println(len(ligne))
}
```

```sortie
Alice   |  14.3
15
```

Ligne par ligne :

- `nom` vaut `"Alice"` et `moyenne` vaut `14.256` ;
- `%-8s` place le nom dans une case de 8 caractères, collé à gauche : `Alice` est suivi
  de 3 espaces. Le `|` est recopié tel quel ;
- `%6.1f` écrit le nombre avec un seul chiffre après la virgule (`14.3`), dans une case
  de 6 caractères, collé à droite ;
- rien n'est affiché à ce moment : le texte est rangé dans `ligne`, puis affiché par
  `Println`. Il fait 8 + 1 + 6 = 15 caractères.

### strings.Builder

Coller des textes avec `+` dans une boucle crée beaucoup de chaînes intermédiaires.
Pour construire un long texte morceau par morceau, utilisez un `strings.Builder` :
c'est comme un carnet où l'on ajoute des lignes.

```go
package main

import (
	"fmt"
	"strings"
)

func main() {
	var sb strings.Builder
	for i := 1; i <= 3; i++ {
		sb.WriteString("Ligne ")
		fmt.Fprintf(&sb, "%d\n", i)
	}
	sb.WriteString("Fin")
	fmt.Println(sb.String())
}
```

```sortie
Ligne 1
Ligne 2
Ligne 3
Fin
```

- `sb.WriteString(...)` ajoute un texte ;
- `fmt.Fprintf(&sb, ...)` ajoute un texte formaté ;
- `sb.String()` renvoie le texte complet.

Le `&` devant `sb` est expliqué dans [les pointeurs](pointeurs.md).

**À vous :** écrivez un programme qui range `"Alice"` dans `prenom` et `12` dans `age`,
fabrique avec `fmt.Sprintf` le texte « Alice a 12 ans. », le range dans une variable
`phrase`, puis l'affiche.

> **Solution**
>
> ```go
> package main
>
> import "fmt"
>
> func main() {
> 	prenom := "Alice"
> 	age := 12
> 	phrase := fmt.Sprintf("%s a %d ans.", prenom, age)
> 	fmt.Println(phrase)
> }
> ```
>
> ```sortie
> Alice a 12 ans.
> ```

## Texte et nombres : strconv

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

Pour coller un nombre à un texte, il faut d'abord le convertir. Le paquet `strconv`
s'en charge (voir aussi [nombres et calculs](nombres.md)).

```go
package main

import (
	"fmt"
	"strconv"
)

func main() {
	age := 12
	fmt.Println("J'ai " + strconv.Itoa(age) + " ans")

	n, err := strconv.Atoi("2026")
	fmt.Println(n+1, err)

	ok, _ := strconv.ParseBool("true")
	fmt.Println(ok)

	fmt.Println(strconv.Quote("dit \"oui\""))
}
```

```sortie
J'ai 12 ans
2027 <nil>
true
"dit \"oui\""
```

Ligne par ligne :

- `strconv.Itoa(age)` transforme le nombre 12 en texte `"12"`, que `+` peut coller ;
- `strconv.Atoi("2026")` fait l'inverse : il lit le texte et donne le nombre 2026. Il
  renvoie aussi `err`, qui vaut `nil` (« rien ») quand tout s'est bien passé ;
- `strconv.ParseBool("true")` lit le texte `"true"` et donne le booléen `true`. Le `_`
  ignore l'erreur ;
- `strconv.Quote` entoure le texte de guillemets et ajoute les `\` nécessaires.

> **Attention :** `string(65)` ne donne pas `"65"`, mais `"A"` (la lettre de code 65).
> Pour transformer un nombre en texte, utilisez toujours `strconv.Itoa` ou
> `fmt.Sprint`.

## Les chaînes brutes

Dans une chaîne normale, chaque `\` doit être doublé : un chemin Windows devient vite
illisible, comme `"C:\\Users\\alice"`. Les chaînes brutes évitent ce problème.

Entre accents graves `` ` ``, une chaîne est **brute** : les `\` ne sont pas
interprétés, et le texte peut tenir sur plusieurs lignes.

```go
package main

import "fmt"

func main() {
	chemin := `C:\Users\alice\notes.txt`
	fmt.Println(chemin)

	poeme := `Le ciel est bleu,
	la mer aussi.`
	fmt.Println(poeme)
}
```

```sortie
C:\Users\alice\notes.txt
Le ciel est bleu,
	la mer aussi.
```

Ligne par ligne :

- `chemin` contient des `\` recopiés tels quels : `\n` dans `\notes` ne passe pas à la
  ligne ;
- `poeme` s'étend sur deux lignes : le retour à la ligne fait partie du texte.

Tout est gardé tel quel, même la tabulation au début de la deuxième ligne. C'est
pratique pour un chemin Windows, un modèle de texte, du JSON ou une
[expression régulière](regex.md).

## Erreurs fréquentes

### Modifier une lettre d'une chaîne

```go norun
package main

import "fmt"

func main() {
	mot := "salut"
	mot[0] = 'S'
	fmt.Println(mot)
}
```

```text
# command-line-arguments
./main.go:7:2: cannot assign to mot[0] (neither addressable nor a map index expression)
```

Une chaîne est immuable. Créez-en une nouvelle :
`mot = "S" + mot[1:]`, ou passez par `[]rune(mot)` puis `string(runes)`.

### Utiliser des apostrophes pour un texte

```go norun
package main

import "fmt"

func main() {
	fmt.Println('Bonjour')
}
```

```text
# command-line-arguments
./main.go:6:14: more than one character in rune literal
```

Les apostrophes `'...'` sont réservées à **un seul** caractère (une rune). Pour un
texte, utilisez les guillemets : `"Bonjour"`.

### Coller un nombre à un texte avec +

```go norun
package main

import "fmt"

func main() {
	age := 12
	fmt.Println("J'ai " + age + " ans")
}
```

```text
# command-line-arguments
./main.go:7:14: invalid operation: "J'ai " + age (mismatched types untyped string and int)
```

On ne peut pas additionner un texte et un nombre. Convertissez avec
`strconv.Itoa(age)`, ou écrivez `fmt.Println("J'ai", age, "ans")`.

## Testez-vous

**Question 1 :** que valent `len("añ")` et `utf8.RuneCountInString("añ")` ?

> **Réponse**
>
> `3` et `2`. `a` prend 1 octet, `ñ` en prend 2 : 3 octets, mais 2 caractères.

**Question 2 :** à partir de `"  alice,BOB , Chloé "`, affichez les prénoms en
majuscules, séparés par ` / `.

> **Réponse**
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
> 	morceaux := strings.Split("  alice,BOB , Chloé ", ",")
> 	for i, m := range morceaux {
> 		morceaux[i] = strings.ToUpper(strings.TrimSpace(m))
> 	}
> 	fmt.Println(strings.Join(morceaux, " / "))
> }
> ```
>
> ```sortie
> ALICE / BOB / CHLOÉ
> ```

**Question 3 :** pourquoi préférer `strings.Builder` à `+` dans une grande boucle ?

> **Réponse**
>
> Chaque `+` crée une nouvelle chaîne complète, ce qui devient lent quand le texte
> grandit. `strings.Builder` ajoute les morceaux au même endroit, sans tout recopier.

## En résumé

- Une chaîne est une suite d'octets en UTF-8, et elle est immuable.
- Un caractère est une `rune` (`'é'`) ; `len` compte les octets,
  `utf8.RuneCountInString` les caractères.
- `range` sur une chaîne donne les runes une par une.
- `strings` cherche (`Contains`, `HasPrefix`), transforme (`ToUpper`, `Replace`,
  `TrimSpace`) et découpe (`Split`, `Fields`, `Join`).
- `fmt.Sprintf` et `strings.Builder` assemblent du texte ; `strconv` convertit les
  nombres.
- Les accents graves `` `...` `` font une chaîne brute, sur plusieurs lignes.

Suite : [tableaux et slices](collections.md). Comparez avec
[le texte en Ruby](../ruby/texte.md).
