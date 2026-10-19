# Générer du texte et du HTML avec les templates

Cette page montre comment fabriquer du texte ou des pages web à partir d'un modèle à trous, avec les paquets `text/template` et `html/template` de la bibliothèque standard.

> **Objectifs :**
>
> - écrire un modèle et le remplir avec `Execute` ;
> - afficher des champs avec `{{.Champ}}` ;
> - répéter avec `{{range}}` et choisir avec `{{if}}` ;
> - utiliser les fonctions des modèles et ajouter les vôtres avec `FuncMap` ;
> - comprendre pourquoi `html/template` échappe le HTML.

## Un modèle, c'est une lettre à trous

Pensez à une lettre type de la mairie : « Bonjour ____, votre rendez-vous est le ____ ».
Le texte est écrit une fois. On remplit les trous pour chaque personne.

Un **modèle** (en anglais *template*) fonctionne pareil. Le texte fixe est écrit tel
quel. Les trous sont entre doubles accolades : `{{ … }}`. On les appelle des
**actions**.

```text
  modèle :   "Bonjour {{.}} !"          données : "Léa"
                     │                              │
                     └──────────── Execute ─────────┘
                                     │
  résultat :          "Bonjour Léa !"
```

> **À retenir :** un modèle = du texte fixe + des trous `{{ … }}`. `Execute` remplit les
> trous avec vos données.

## Un premier modèle

Trois étapes :

1. `template.New("nom")` crée un modèle vide, avec un nom ;
2. `.Parse(texte)` lit le texte du modèle ;
3. `.Execute(destination, données)` remplit les trous et écrit le résultat.

```go
package main

import (
	"os"
	"text/template"
)

func main() {
	t := template.Must(template.New("salut").Parse("Bonjour {{.}} !\n"))
	t.Execute(os.Stdout, "Léa")
	t.Execute(os.Stdout, "Tom")
}
```

```sortie
Bonjour Léa !
Bonjour Tom !
```

- `{{.}}` (un point seul) veut dire « la donnée entière ». Ici, le texte `"Léa"`.
- `os.Stdout` est l'écran. On pourrait écrire dans un fichier ou une page web à la
  place.
- `template.Must` arrête le programme si le modèle est mal écrit. Vous le voyez tout de
  suite, au démarrage.

Le même modèle sert deux fois : on l'écrit une fois, on le remplit autant qu'on veut.

Essayez : remplacez `"Léa"` par `42`. Le modèle affiche `Bonjour 42 !` : `{{.}}` accepte
n'importe quelle valeur.

> **À retenir :** `{{.}}` = la donnée qu'on a donnée à `Execute`.

## Les champs : {{.Champ}}

En général, on passe une [structure](../debuter/glossaire.md#structure).
`{{.Nom}}` lit son champ `Nom`.

```go
package main

import (
	"os"
	"text/template"
)

type RendezVous struct {
	Nom   string
	Jour  string
	Heure int
}

func main() {
	modele := "Bonjour {{.Nom}}, votre rendez-vous est {{.Jour}} à {{.Heure}} h.\n"
	t := template.Must(template.New("rdv").Parse(modele))

	t.Execute(os.Stdout, RendezVous{Nom: "Léa", Jour: "lundi", Heure: 9})
	t.Execute(os.Stdout, RendezVous{Nom: "Tom", Jour: "mardi", Heure: 14})
}
```

```sortie
Bonjour Léa, votre rendez-vous est lundi à 9 h.
Bonjour Tom, votre rendez-vous est mardi à 14 h.
```

Pas à pas, pour Léa :

| Étape | Morceau du modèle | Ce qui est écrit |
|---|---|---|
| 1 | `Bonjour ` | `Bonjour ` |
| 2 | `{{.Nom}}` | `Léa` |
| 3 | `, votre rendez-vous est ` | `, votre rendez-vous est ` |
| 4 | `{{.Jour}}` | `lundi` |
| 5 | ` à ` | ` à ` |
| 6 | `{{.Heure}}` | `9` |
| 7 | ` h.` et le retour à la ligne | ` h.` |

Les champs doivent commencer par une **majuscule** (être exportés). Le paquet `template`
ne peut pas lire un champ en minuscule (voir [les erreurs](#erreurs-frequentes)).

Une [map](maps.md) marche aussi : `{{.ville}}` lit la valeur de la clé `"ville"`.

```go
package main

import (
	"os"
	"text/template"
)

func main() {
	t := template.Must(template.New("m").Parse("{{.ville}} : {{.temp}} °C\n"))
	t.Execute(os.Stdout, map[string]any{"ville": "Lyon", "temp": 21})
}
```

```sortie
Lyon : 21 °C
```

> **À retenir :** `{{.Champ}}` lit un champ de structure (avec une majuscule) ou une clé
> de map.

## Choisir avec if

`{{if …}} … {{else}} … {{end}}` choisit un morceau de texte. La condition est vraie si
la valeur n'est pas « vide » : `false`, `0`, `""`, une liste vide et `nil` sont faux.

```go
package main

import (
	"os"
	"text/template"
)

type Article struct {
	Nom   string
	Stock int
}

func main() {
	modele := "{{.Nom}} : {{if .Stock}}en stock ({{.Stock}}){{else}}épuisé{{end}}\n"
	t := template.Must(template.New("a").Parse(modele))

	t.Execute(os.Stdout, Article{"stylo", 12})
	t.Execute(os.Stdout, Article{"gomme", 0})
}
```

```sortie
stylo : en stock (12)
gomme : épuisé
```

| Article | `.Stock` | vrai ? | Morceau choisi |
|---|---|---|---|
| stylo | 12 | oui | `en stock (12)` |
| gomme | 0 | non (0 est vide) | `épuisé` |

### Comparer

Dans un modèle, on n'écrit pas `>` ou `==`. On utilise des **fonctions** placées
**avant** les valeurs : `eq` (égal), `ne` (différent), `lt` (plus petit), `le`, `gt`
(plus grand), `ge`.

```go
package main

import (
	"os"
	"text/template"
)

func main() {
	modele := "{{.}} : {{if ge . 10}}note correcte{{else}}à retravailler{{end}}\n"
	t := template.Must(template.New("n").Parse(modele))
	for _, note := range []int{15, 8, 10} {
		t.Execute(os.Stdout, note)
	}
}
```

```sortie
15 : note correcte
8 : à retravailler
10 : note correcte
```

`ge . 10` se lit « est-ce que `.` est plus grand ou égal à 10 ? ».

Essayez : remplacez `ge` par `gt`. La note 10 devient `à retravailler`, car 10 n'est
pas strictement plus grand que 10.

> **À retenir :** `{{if}}` … `{{else}}` … `{{end}}`, et les comparaisons s'écrivent
> `eq a b`, `gt a b`…

## Répéter avec range

`{{range .Liste}} … {{end}}` répète le morceau pour chaque élément. **Dans** la boucle,
le point `.` devient l'élément en cours.

```go
package main

import (
	"os"
	"text/template"
)

func main() {
	modele := "Courses :\n{{range .}}- {{.}}\n{{end}}"
	t := template.Must(template.New("c").Parse(modele))
	t.Execute(os.Stdout, []string{"pain", "lait", "pommes"})
}
```

```sortie
Courses :
- pain
- lait
- pommes
```

| Tour | Valeur de `.` | Ce qui est écrit |
|---|---|---|
| avant | la liste entière | `Courses :` |
| 1 | `"pain"` | `- pain` |
| 2 | `"lait"` | `- lait` |
| 3 | `"pommes"` | `- pommes` |

C'est comme un tampon encreur : le même dessin, imprimé une fois par élément.

### L'indice, et le cas d'une liste vide

`{{range $i, $x := .}}` donne aussi l'indice. Les noms qui commencent par `$` sont des
**variables du modèle**. `{{else}}` dans un `range` s'affiche quand la liste est vide.

```go
package main

import (
	"os"
	"text/template"
)

type Panier struct {
	Client   string
	Articles []string
}

func main() {
	modele := `Panier de {{.Client}} :
{{range $i, $a := .Articles}}  {{$i}}. {{$a}}
{{else}}  (vide)
{{end}}`
	t := template.Must(template.New("p").Parse(modele))
	t.Execute(os.Stdout, Panier{"Léa", []string{"stylo", "cahier"}})
	t.Execute(os.Stdout, Panier{"Tom", nil})
}
```

```sortie
Panier de Léa :
  0. stylo
  1. cahier
Panier de Tom :
  (vide)
```

### Enlever les espaces en trop avec {{- et -}}

Un modèle recopie **tout**, même les retours à la ligne. Un tiret collé aux accolades
efface les espaces et retours à la ligne de ce côté : `{{-` à gauche, `-}}` à droite.

```go
package main

import (
	"os"
	"text/template"
)

func main() {
	modele := `Nombres :
{{- range .}}
  {{.}}
{{- end}}
`
	t := template.Must(template.New("n").Parse(modele))
	t.Execute(os.Stdout, []int{1, 2, 3})
}
```

```sortie
Nombres :
  1
  2
  3
```

Essayez : enlevez les deux tirets. Des lignes vides apparaissent entre les nombres.

> **À retenir :** dans `{{range}}`, le point `.` est l'élément en cours. Après
> `{{end}}`, il redevient la donnée entière.

## Les fonctions des modèles

Le paquet fournit quelques fonctions. On les appelle en écrivant le nom, puis les
arguments, séparés par des espaces (sans parenthèses ni virgules).

| Fonction | Exemple | Résultat |
|---|---|---|
| `len` | `{{len .Articles}}` | le nombre d'éléments |
| `index` | `{{index .Notes 0}}` | le premier élément |
| `printf` | `{{printf "%.2f" .Prix}}` | comme `fmt.Sprintf` |
| `and`, `or`, `not` | `{{if and .A .B}}` | logique |

```go
package main

import (
	"os"
	"text/template"
)

type Facture struct {
	Lignes []string
	Total  float64
}

func main() {
	modele := "{{len .Lignes}} lignes, la première : {{index .Lignes 0}}, total {{printf \"%.2f\" .Total}} €\n"
	t := template.Must(template.New("f").Parse(modele))
	t.Execute(os.Stdout, Facture{[]string{"café", "croissant"}, 3.5})
}
```

```sortie
2 lignes, la première : café, total 3.50 €
```

### Le tuyau |

`{{.Total | printf "%.2f"}}` envoie la valeur de gauche comme **dernier** argument de la
fonction de droite. C'est un tuyau, comme dans le [terminal](../debuter/terminal.md).
`{{printf "%.2f" .Total}}` et `{{.Total | printf "%.2f"}}` donnent la même chose.

### Vos propres fonctions : FuncMap

Une `template.FuncMap` est une map « nom → fonction Go ». On la donne au modèle avec
`.Funcs(...)`, **avant** `Parse`.

```go
package main

import (
	"fmt"
	"os"
	"strings"
	"text/template"
)

func main() {
	fonctions := template.FuncMap{
		"majuscules": strings.ToUpper,
		"euros": func(centimes int) string {
			return fmt.Sprintf("%d,%02d €", centimes/100, centimes%100)
		},
	}

	modele := "{{majuscules .Nom}} coûte {{euros .Prix}}\n"
	t := template.Must(template.New("p").Funcs(fonctions).Parse(modele))

	type Produit struct {
		Nom  string
		Prix int
	}
	t.Execute(os.Stdout, Produit{"cahier", 249})
	t.Execute(os.Stdout, Produit{"stylo", 90})
}
```

```sortie
CAHIER coûte 2,49 €
STYLO coûte 0,90 €
```

| Produit | `.Prix` | `centimes/100` | `centimes%100` | `euros .Prix` |
|---|---|---|---|---|
| cahier | 249 | 2 | 49 | `2,49 €` |
| stylo | 90 | 0 | 90 | `0,90 €` |

Essayez : ajoutez `"double": func(n int) int { return n * 2 }` dans la `FuncMap`, puis
`{{euros (double .Prix)}}` dans le modèle. Les parenthèses calculent d'abord
`double .Prix` : le cahier coûte `4,98 €`.

> **À retenir :** les fonctions s'appellent sans parenthèses : `{{nom arg1 arg2}}`. Les
> vôtres s'ajoutent avec `.Funcs(template.FuncMap{…})` avant `Parse`.

## html/template : le HTML sans danger

Pour fabriquer une page web, utilisez `html/template` au lieu de `text/template`. Les
deux s'écrivent **exactement** pareil. La différence : `html/template` **échappe** les
données.

Échapper, c'est transformer les caractères spéciaux du [HTML](../debuter/glossaire.md#html)
(`<`, `>`, `"`, `&`) en codes inoffensifs. Imaginez un visiteur qui s'appelle
`<script>alert("coucou")</script>`. Recopié tel quel dans la page, ce texte deviendrait
un **programme** exécuté chez tous les lecteurs. C'est une attaque connue, la faille XSS
(voir [la sécurité](../debuter/securite.md)).

Comparons les deux paquets avec le même nom piégé :

```go
package main

import (
	htmlt "html/template"
	"os"
	textt "text/template"
)

func main() {
	nom := `<script>alert("coucou")</script>`
	modele := "<p>Bonjour {{.}}</p>\n"

	t1 := textt.Must(textt.New("t").Parse(modele))
	t1.Execute(os.Stdout, nom)

	t2 := htmlt.Must(htmlt.New("h").Parse(modele))
	t2.Execute(os.Stdout, nom)
}
```

```sortie
<p>Bonjour <script>alert("coucou")</script></p>
<p>Bonjour &lt;script&gt;alert(&#34;coucou&#34;)&lt;/script&gt;</p>
```

| Caractère | Devient | Le navigateur affiche |
|---|---|---|
| `<` | `&lt;` | `<` (du texte, pas une balise) |
| `>` | `&gt;` | `>` |
| `"` | `&#34;` | `"` |
| `&` | `&amp;` | `&` |

Avec `html/template`, le navigateur **affiche** le nom, au lieu de l'exécuter.

### Il connaît le contexte

`html/template` sait **où** se trouve chaque trou : dans du texte, dans un lien, dans
une adresse. Il échappe différemment selon l'endroit.

```go
package main

import (
	"html/template"
	"os"
)

func main() {
	modele := `<a href="{{.}}">lien</a>` + "\n"
	t := template.Must(template.New("lien").Parse(modele))
	t.Execute(os.Stdout, "https://go.dev/?a=1&b=2")
	t.Execute(os.Stdout, "javascript:alert(1)")
}
```

```sortie
<a href="https://go.dev/?a=1&amp;b=2">lien</a>
<a href="#ZgotmplZ">lien</a>
```

- La première [URL](../debuter/glossaire.md#url) est normale : seul le `&` est échappé.
- La seconde est dangereuse (`javascript:` lance un programme quand on clique). Elle est
  remplacée par `#ZgotmplZ`, un mot de code qui veut dire « valeur refusée ».

### Quand le HTML est sûr : template.HTML

Si **vous** avez écrit le HTML (pas un visiteur), dites-le avec le type `template.HTML`.
Il n'est alors pas échappé.

```go
package main

import (
	"html/template"
	"os"
)

func main() {
	t := template.Must(template.New("h").Parse("<p>{{.}}</p>\n"))
	t.Execute(os.Stdout, "<b>gras</b>")
	t.Execute(os.Stdout, template.HTML("<b>gras</b>"))
}
```

```sortie
<p>&lt;b&gt;gras&lt;/b&gt;</p>
<p><b>gras</b></p>
```

> **Attention :** ne mettez jamais un texte venu d'un visiteur dans `template.HTML`.
> Vous désactiveriez la protection.

> **À retenir :** pour du HTML, toujours `html/template`. Il échappe tout seul, selon
> l'endroit, et c'est ce qui protège votre site.

## D'où viennent les modèles ?

### D'une chaîne

C'est ce que fait cette page : `Parse("…")`. Pratique pour de petits modèles. Un modèle
peut en contenir d'autres, nommés avec `{{define "nom"}}` et appelés avec
`{{template "nom" .}}`.

```go
package main

import (
	"os"
	"text/template"
)

const modeles = `
{{define "ligne"}}* {{.}}{{end}}
{{define "liste"}}{{range .}}{{template "ligne" .}}
{{end}}{{end}}`

func main() {
	t := template.Must(template.New("tout").Parse(modeles))
	t.ExecuteTemplate(os.Stdout, "liste", []string{"Go", "Rust"})
}
```

```sortie
* Go
* Rust
```

`ExecuteTemplate` choisit le modèle à remplir par son nom. C'est comme un classeur de
lettres types : on sort celle dont on a besoin.

### De fichiers

Pour de vrais modèles, on les range dans des fichiers `.html` ou `.txt` :

```go norun
t := template.Must(template.ParseFiles("modeles/accueil.html", "modeles/pied.html"))
t.ExecuteTemplate(os.Stdout, "accueil.html", donnees)
```

Chaque modèle prend le nom de son fichier. `template.ParseGlob("modeles/*.html")`
charge tout un dossier.

### Embarqués dans le programme

Avec `//go:embed`, les fichiers de modèles sont rangés **dans** l'exécutable : plus
besoin de les livrer à côté. La page [embarquer des fichiers](embed.md) montre
`template.ParseFS` en détail.

```go norun
//go:embed modeles/*.html
var dossier embed.FS

var pages = template.Must(template.ParseFS(dossier, "modeles/*.html"))
```

Dans un serveur [web](web.md), on charge les modèles **une seule fois**, au démarrage,
puis on appelle `pages.ExecuteTemplate(w, "accueil.html", donnees)` dans chaque
gestionnaire : `w` remplace `os.Stdout`.

> **À retenir :** chaîne pour les petits modèles, fichiers ou `embed` pour les vrais.
> On analyse (`Parse`) une fois, on remplit (`Execute`) souvent.

## Erreurs fréquentes

### Un champ qui n'existe pas

```go norun
type Personne struct{ Nom string }

t := template.Must(template.New("d").Parse("{{.Prenom}}\n"))
err := t.Execute(os.Stdout, Personne{Nom: "Léa"})
fmt.Println(err)
```

```text
template: d:1:2: executing "d" at <.Prenom>: can't evaluate field Prenom in type main.Personne
```

L'erreur arrive à l'`Execute`, pas au `Parse`. `d:1:2` veut dire : modèle `d`, ligne 1,
colonne 2. Vérifiez le nom du champ, majuscules comprises. Et regardez toujours l'erreur
renvoyée par `Execute`.

### Un champ en minuscule

```go norun
type Personne struct {
	Nom string
	age int
}

t := template.Must(template.New("e").Parse("{{.age}}\n"))
fmt.Println(t.Execute(os.Stdout, Personne{Nom: "Léa"}))
```

```text
template: e:1:2: executing "e" at <.age>: age is an unexported field of struct type main.Personne
```

Le modèle vit dans un autre paquet : il ne voit que les champs exportés. Renommez le
champ `Age`.

### Oublier {{end}}

```go norun
_, err := template.New("b").Parse("{{if .Nom}}oui")
fmt.Println(err)
```

```text
template: b:1: unexpected EOF
```

`EOF` veut dire « fin du texte ». Le modèle s'arrête alors qu'un `{{if}}` attend encore
son `{{end}}`. Chaque `if`, `range` et `define` se ferme par `{{end}}`.

### Une fonction ajoutée trop tard

```go norun
t, err := template.New("c").Parse("{{majuscules .Nom}}")
fmt.Println(err)
```

```text
template: c:1: function "majuscules" not defined
```

Le modèle vérifie les noms de fonctions pendant `Parse`. Appelez `.Funcs(...)` **avant**
`.Parse(...)`.

## Testez-vous

**Question 1 :** qu'affiche ce modèle avec la donnée `[]int{4, 5}` ?

```text
{{range .}}[{{.}}]{{end}} total : {{len .}}
```

> **Réponse**
>
> `[4][5] total : 2`. Dans `range`, `.` vaut 4 puis 5. Après `{{end}}`, `.` redevient la
> liste entière, qui a 2 éléments.

**Question 2 :** pourquoi utiliser `html/template` plutôt que `text/template` pour une
page web ?

> **Réponse**
>
> Parce qu'il échappe les données : `<script>` devient `&lt;script&gt;` et s'affiche
> comme du texte, au lieu d'être exécuté par le navigateur.

**Question 3 :** comment écrire « si `.Age` est plus petit que 18 » dans un modèle ?

> **Réponse**
>
> `{{if lt .Age 18}} … {{end}}`. La fonction de comparaison vient en premier, puis les
> deux valeurs.

## En résumé

- Un modèle est un texte à trous ; `Parse` le lit, `Execute` le remplit.
- `{{.}}` est la donnée, `{{.Champ}}` un champ exporté ou une clé de map.
- `{{if}}`/`{{else}}`/`{{end}}` choisit, `{{range}}` répète ; `eq`, `lt`, `gt`
  comparent.
- Les fonctions s'appellent sans parenthèses ; `FuncMap` ajoute les vôtres avant
  `Parse`.
- `html/template` échappe le HTML selon le contexte : utilisez-le pour toute page web.

Suite : [une base de données en Go](base-de-donnees.md).
