# Embarquer des fichiers avec embed

Cette page montre comment ranger des fichiers (textes, modèles, pages web, configuration) directement dans le programme compilé, avec la directive `//go:embed` et le paquet `io/fs`.

> **Objectifs :**
>
> - vous saurez pourquoi embarquer des fichiers dans un programme Go ;
> - vous saurez embarquer un fichier dans un `string` ou un `[]byte` avec `//go:embed` ;
> - vous saurez embarquer plusieurs fichiers ou un dossier dans un `embed.FS` ;
> - vous saurez lire ces fichiers avec `fs.ReadFile`, `fs.ReadDir`, `fs.WalkDir` et `fs.Sub` ;
> - vous saurez servir des fichiers embarqués sur le web et charger des modèles avec `ParseFS` ;
> - vous connaîtrez les règles de `//go:embed` et les cas où il vaut mieux ne pas embarquer.

## Pourquoi embarquer des fichiers ?

Un programme Go se compile en un seul [binaire](../debuter/glossaire.md#binaire) : un
fichier exécutable que l'on copie sur une autre machine, et qui marche. Mais beaucoup de
programmes ont besoin de fichiers à côté : des modèles de pages, une configuration par
défaut, une feuille de style. Si on oublie de les copier, le programme plante.

**Embarquer** un fichier, c'est le mettre *dans* le binaire, au moment de la
compilation. C'est comme une valise qui contient déjà tout : un seul objet à
transporter. Avec un `main.go` qui embarque `message.txt` (le code est expliqué
juste après) :

```sh
go build -o salut .
rm message.txt
./salut
```

```text
Bonjour depuis un fichier embarqué !
```

Le fichier a été supprimé, mais son texte est toujours là : il a été copié dans le
binaire pendant `go build`.

## La directive //go:embed sur un string

Trois choses suffisent pour embarquer un fichier :

1. importer le paquet `embed`. Quand on ne l'utilise pas par son nom, on l'importe
   avec `_` : `import _ "embed"` ;
2. écrire le commentaire spécial `//go:embed nom-du-fichier`, **sans espace** entre
   `//` et `go:embed` ;
3. déclarer, juste en dessous, une variable de type `string`.

Ici, le programme embarque son propre code, `main.go` : ce fichier existe forcément.

```go
package main

import (
	_ "embed"
	"fmt"
	"strings"
)

//go:embed main.go
var source string

func main() {
	lignes := strings.Split(source, "\n")
	fmt.Println("Première ligne :", lignes[0])
}
```

```sortie
Première ligne : package main
```

Le [compilateur](../debuter/glossaire.md#compilateur) lit `main.go` et place tout son
contenu dans `source`. Aucun `os.ReadFile`, aucune erreur à vérifier : si le fichier
manque, c'est la compilation qui échoue.

> **Attention :** `// go:embed` (avec une espace) n'est qu'un commentaire ordinaire.
> Go l'ignore, et la variable reste vide.

## Embarquer dans un []byte

Pour une image ou tout fichier qui n'est pas du texte, utilisez une tranche
d'[octets](../debuter/glossaire.md#octet), `[]byte`. C'est aussi le type attendu par
`json.Unmarshal`, par exemple.

```go
package main

import (
	_ "embed"
	"fmt"
)

//go:embed main.go
var contenu []byte

func main() {
	fmt.Printf("Type : %T\n", contenu)
	fmt.Println("Début :", string(contenu[:12]))
	fmt.Println("Premier octet :", contenu[0])
}
```

```sortie
Type : []uint8
Début : package main
Premier octet : 112
```

`[]uint8` est l'autre nom de `[]byte`, et `112` est le code de la lettre `p`.

> **À retenir :** `string` pour un texte, `[]byte` pour des octets. Les deux ne
> reçoivent qu'**un seul** fichier.

## Plusieurs fichiers avec embed.FS

Pour plusieurs fichiers, ou un dossier entier, le type est `embed.FS` : un petit
**système de fichiers** en lecture seule, rangé dans le binaire. On utilise cette fois
le paquet par son nom : l'import s'écrit `import "embed"`, sans `_`.

```go
package main

import (
	"embed"
	"fmt"
)

//go:embed main.go
var fichiers embed.FS

func main() {
	contenu, err := fichiers.ReadFile("main.go")
	fmt.Println("main.go :", len(contenu) > 0, err)

	_, err = fichiers.ReadFile("absent.txt")
	fmt.Println("absent.txt :", err)
}
```

```sortie
main.go : true <nil>
absent.txt : open absent.txt: file does not exist
```

`ReadFile` renvoie une erreur, car le nom est choisi pendant l'exécution : le
compilateur ne peut pas savoir s'il existe.

### Motifs et dossiers

Après `//go:embed`, vous pouvez écrire plusieurs noms, des **motifs** comme `*.txt`,
ou des noms de dossiers. Prenons ce projet :

```text
site/
├── go.mod
├── main.go
├── version.txt          (contient « 1.4.2 »)
├── templates/
│   ├── aurevoir.txt
│   └── bonjour.txt
└── static/
    ├── .brouillon.txt
    ├── _notes.txt
    ├── index.html
    ├── style.css
    └── images/
        └── logo.png
```

```go norun
package main

import (
	"embed"
	"fmt"
	"io/fs"
)

//go:embed version.txt templates/*.txt
var fichiers embed.FS

//go:embed static
var statique embed.FS

func lister(nom string, fsys fs.FS) {
	fmt.Println(nom + " :")
	fs.WalkDir(fsys, ".", func(chemin string, d fs.DirEntry, err error) error {
		if !d.IsDir() {
			fmt.Println("  " + chemin)
		}
		return nil
	})
}

func main() {
	version, _ := fichiers.ReadFile("version.txt")
	fmt.Print("Version : ", string(version))
	lister("fichiers", fichiers)
	lister("statique", statique)
}
```

```text
Version : 1.4.2
fichiers :
  templates/aurevoir.txt
  templates/bonjour.txt
  version.txt
statique :
  static/images/logo.png
  static/index.html
  static/style.css
```

- `templates/*.txt` prend tous les fichiers `.txt` du dossier `templates` ;
- `static` prend le dossier **et ses sous-dossiers** ;
- les chemins gardent leur dossier : on lit `static/style.css`, pas `style.css`.

`.brouillon.txt` et `_notes.txt` manquent : c'est une des
[règles à connaître](#les-regles-a-connaitre).

## Lire un système de fichiers avec io/fs

Un `embed.FS` respecte l'interface `fs.FS` du paquet `io/fs` : toutes les fonctions de
ce paquet marchent dessus. Pour les essayer sans créer de fichiers, `testing/fstest`
offre `fstest.MapFS`, un faux système de fichiers écrit comme une map. Le code est le
même qu'avec un vrai `embed.FS`.

### fs.ReadFile et fs.ReadDir

`fs.ReadFile` lit un fichier entier. `fs.ReadDir` liste un dossier, trié par nom.

```go
package main

import (
	"fmt"
	"io/fs"
	"testing/fstest"
)

func main() {
	var fsys fs.FS = fstest.MapFS{
		"config.txt":     {Data: []byte("couleur=bleu\n")},
		"docs/aide.txt":  {Data: []byte("Tapez q pour quitter.\n")},
		"docs/intro.txt": {Data: []byte("Bienvenue !\n")},
	}

	contenu, _ := fs.ReadFile(fsys, "config.txt")
	fmt.Print("config.txt : ", string(contenu))

	entrees, _ := fs.ReadDir(fsys, "docs")
	for _, e := range entrees {
		fmt.Println("dans docs :", e.Name())
	}
}
```

```sortie
config.txt : couleur=bleu
dans docs : aide.txt
dans docs : intro.txt
```

Les chemins de `io/fs` utilisent toujours `/`, même sous Windows, et la racine
s'appelle `.`.

### fs.WalkDir : parcourir tout l'arbre

`fs.WalkDir` visite chaque fichier et chaque dossier, sous-dossiers compris, et
appelle votre fonction pour chacun.

```go
package main

import (
	"fmt"
	"io/fs"
	"testing/fstest"
)

func main() {
	fsys := fstest.MapFS{
		"index.html":      {Data: []byte("<h1>Accueil</h1>")},
		"css/style.css":   {Data: []byte("body {}")},
		"images/logo.png": {Data: []byte("PNG")},
	}

	fs.WalkDir(fsys, ".", func(chemin string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if d.IsDir() {
			fmt.Println("[dossier]", chemin)
		} else {
			fmt.Println("         ", chemin)
		}
		return nil
	})
}
```

```sortie
[dossier] .
[dossier] css
          css/style.css
[dossier] images
          images/logo.png
          index.html
```

### fs.Sub : entrer dans un sous-dossier

`fs.Sub` renvoie un nouveau système de fichiers dont la racine est un sous-dossier.
C'est très utile avec `//go:embed static` : on veut lire `style.css`, pas
`static/style.css`.

```go
package main

import (
	"fmt"
	"io/fs"
	"testing/fstest"
)

func main() {
	fsys := fstest.MapFS{
		"static/style.css": {Data: []byte("body { color: navy; }\n")},
	}

	racine, _ := fs.Sub(fsys, "static")
	css, _ := fs.ReadFile(racine, "style.css")
	fmt.Print(string(css))
}
```

```sortie
body { color: navy; }
```

## Servir des fichiers sur le web

Avec un `embed.FS`, un petit site web tient dans un seul binaire. Le serveur
[HTTP](../debuter/glossaire.md#http) de Go sert un `fs.FS` grâce à
`http.FileServerFS` (Go 1.22 et plus). Le dossier `static/` contient `index.html`
(`<h1>Bienvenue</h1>`) et `style.css` :

```go norun
package main

import (
	"embed"
	"io/fs"
	"log"
	"net/http"
)

//go:embed static
var statique embed.FS

func main() {
	racine, err := fs.Sub(statique, "static")
	if err != nil {
		log.Fatal(err)
	}

	http.Handle("/", http.FileServerFS(racine))

	log.Println("Serveur sur http://localhost:8080")
	log.Fatal(http.ListenAndServe(":8080", nil))
}
```

Pendant que le serveur tourne, dans un autre terminal :

```sh
curl http://localhost:8080/
curl http://localhost:8080/style.css
```

```text
<h1>Bienvenue</h1>
body { color: navy; }
```

Sans `fs.Sub`, les adresses seraient `/static/index.html` et `/static/style.css`.

> **Note :** avant Go 1.22, on écrivait `http.FileServer(http.FS(racine))` :
> `http.FS` transforme un `fs.FS` en `http.FileSystem`, l'ancien type attendu. Vous le
> verrez souvent dans du code existant. Le chapitre [le web](web.md) détaille le
> serveur.

## Charger des modèles avec ParseFS

Les paquets `text/template` et `html/template` lisent leurs modèles dans un `fs.FS`
avec `ParseFS`. On lui donne le système de fichiers et un ou plusieurs motifs. Chaque
modèle prend le nom de son fichier.

```go
package main

import (
	"fmt"
	"os"
	"testing/fstest"
	"text/template"
)

func main() {
	fsys := fstest.MapFS{
		"modeles/bonjour.txt":  {Data: []byte("Bonjour {{.}} !\n")},
		"modeles/aurevoir.txt": {Data: []byte("Au revoir {{.}}.\n")},
	}

	modeles, err := template.ParseFS(fsys, "modeles/*.txt")
	if err != nil {
		fmt.Println(err)
		return
	}
	modeles.ExecuteTemplate(os.Stdout, "bonjour.txt", "Léa")
	modeles.ExecuteTemplate(os.Stdout, "aurevoir.txt", "Léa")
}
```

```sortie
Bonjour Léa !
Au revoir Léa.
```

Dans un vrai projet, on embarque le dossier `templates`, puis on charge les modèles
une seule fois, au démarrage, dans une variable du paquet. `template.Must` arrête le
programme si un modèle est faux : vous le voyez tout de suite.

```go norun
//go:embed templates/*.html
var dossierTemplates embed.FS

var pages = template.Must(template.ParseFS(dossierTemplates, "templates/*.html"))
```

Avec `html/template`, un nom comme `Léa <admin>` s'affiche `Léa &lt;admin&gt;` : les
caractères spéciaux du [HTML](../debuter/glossaire.md#html) ne cassent pas la page.

## Les règles à connaître

`//go:embed` obéit à quelques règles strictes :

| Règle | Exemple |
|-------|---------|
| seulement sur une variable **du paquet**, hors des fonctions | `var page string` en haut du fichier |
| type `string`, `[]byte` ou `embed.FS` | pas de `int`, pas de `[]string` |
| chemins **relatifs au dossier du paquet** (celui du fichier `.go`) | `templates/a.html` |
| séparateur `/`, jamais `\` | `static/css/style.css` |
| pas de `..`, pas de chemin absolu | ni `../config.txt`, ni `/etc/config` |
| le fichier doit exister à la compilation | sinon : `no matching files found` |

### Les fichiers cachés et le préfixe all:

Quand vous embarquez un **dossier**, Go ignore les fichiers dont le nom commence par
`.` ou `_`. Cela évite d'embarquer par erreur un `.git` ou un fichier de travail. Pour
les inclure, ajoutez `all:` devant le motif :

```go norun
//go:embed all:static
var statiqueComplet embed.FS
```

Avec le projet `site/` vu plus haut, `lister("statiqueComplet", statiqueComplet)`
affiche :

```text
statiqueComplet :
  static/.brouillon.txt
  static/_notes.txt
  static/images/logo.png
  static/index.html
  static/style.css
```

Un fichier caché nommé **explicitement**, comme `//go:embed static/.brouillon.txt`,
est toujours embarqué.

## Quand ne pas embarquer

- **Les gros fichiers** (vidéos, grosses bases de données) : ils gonflent le binaire,
  qui devient long à compiler, à copier et à charger.
- **Les fichiers qui changent souvent**, ou que l'utilisateur doit modifier (sa
  configuration, ses données) : chaque changement demanderait de recompiler.
- **Les secrets** (mots de passe, clés) : n'importe qui peut les relire dans le binaire.

Une bonne habitude : embarquer une configuration **par défaut**, et la remplacer par
un fichier sur le disque s'il existe, lu avec `os.ReadFile` comme dans
[le chapitre sur les fichiers](fichiers.md).

## Erreurs fréquentes

### Un fichier qui n'existe pas

```go norun
//go:embed message.txt
var message string
```

```text
main.go:8:12: pattern message.txt: no matching files found
```

`message.txt` n'est pas dans le dossier de `main.go`. Vérifiez son nom et son
emplacement : le chemin part du dossier du paquet, pas du dossier où vous lancez la
commande.

### Oublier l'import du paquet embed

```go norun
package main

import "fmt"

//go:embed main.go
var source string

func main() {
	fmt.Print(source)
}
```

```text
# command-line-arguments
./main.go:5:3: go:embed only allowed in Go files that import "embed"
```

Même pour un simple `string`, il faut importer le paquet : ajoutez `_ "embed"`.

### //go:embed dans une fonction

```go norun
package main

import (
	_ "embed"
	"fmt"
)

func main() {
	//go:embed main.go
	var source string
	fmt.Print(source)
}
```

```text
# command-line-arguments
./main.go:9:4: go:embed cannot apply to var inside func
```

Le fichier est copié à la compilation : la variable doit vivre au niveau du paquet.
Sortez la déclaration de `main`.

### Remonter avec ..

```go norun
//go:embed ../config.txt
var config string
```

```text
main.go:8:12: pattern ../config.txt: invalid pattern syntax
```

Un paquet n'embarque que des fichiers de son dossier ou de ses sous-dossiers. Déplacez
le fichier, ou embarquez-le depuis un paquet placé plus haut qui l'exporte.

## Testez-vous

**Question 1 :** quelle différence entre `import _ "embed"` et `import "embed"` ?

> **Réponse**
>
> Les deux chargent le paquet. Avec `_`, on ne l'utilise pas par son nom : c'est le
> cas pour un `string` ou un `[]byte`. Pour un `embed.FS`, on écrit le nom `embed`,
> donc l'import normal suffit.

**Question 2 :** vous avez écrit `//go:embed static`. Pourquoi
`fs.ReadFile(statique, "style.css")` échoue-t-il ?

> **Réponse**
>
> Les chemins gardent le nom du dossier : le fichier s'appelle `static/style.css`.
> Lisez ce chemin complet, ou créez une racine avec `fs.Sub(statique, "static")`.

**Question 3 :** pourquoi ne pas embarquer le fichier de configuration que
l'utilisateur modifie chaque jour ?

> **Réponse**
>
> Le fichier embarqué est figé au moment de `go build` : chaque modification
> demanderait de recompiler le programme. Lisez plutôt ce fichier sur le disque avec
> `os.ReadFile`, et n'embarquez qu'une configuration par défaut.

## En résumé

- `//go:embed` copie des fichiers dans le binaire à la compilation : un seul fichier
  à distribuer.
- Un fichier va dans un `string` ou un `[]byte` (avec `import _ "embed"`) ; plusieurs
  fichiers ou des dossiers vont dans un `embed.FS`.
- `fs.ReadFile`, `fs.ReadDir`, `fs.WalkDir` et `fs.Sub` lisent un `embed.FS`, comme
  tout `fs.FS` ; `fstest.MapFS` permet de les essayer.
- `http.FileServerFS` sert des fichiers embarqués, et `ParseFS` charge des modèles.
- La variable est au niveau du paquet, les chemins partent du dossier du paquet, sans
  `..` ; `all:` inclut les fichiers qui commencent par `.` ou `_`.
- N'embarquez ni les gros fichiers, ni ceux qui changent souvent, ni les secrets.

Suite : [JSON et CSV](formats.md).
