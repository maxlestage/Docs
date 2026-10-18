# Go pour le web

Cette page montre comment écrire un petit serveur web avec le paquet `net/http`, le tester sans réseau grâce à `net/http/httptest`, renvoyer du JSON et appeler un autre site.

> **Objectifs :**
>
> - vous saurez écrire un gestionnaire (*handler*) qui répond à une requête HTTP ;
> - vous saurez lancer un serveur avec `http.HandleFunc` et `http.ListenAndServe` ;
> - vous saurez tester un gestionnaire avec `httptest`, sans ouvrir de navigateur ;
> - vous saurez écrire une petite API JSON avec des routes comme `GET /livres/{id}` ;
> - vous saurez appeler un site avec `http.Get`.

## Le web en deux phrases

Quand vous ouvrez une page, votre navigateur (le [client](../debuter/glossaire.md#client))
envoie une **requête** [HTTP](../debuter/glossaire.md#http) à un
[serveur](../debuter/glossaire.md#serveur). Le serveur renvoie une **réponse** : un
code (`200` pour « OK », `404` pour « introuvable ») et un contenu (du HTML, du JSON…).

C'est comme au guichet d'une bibliothèque : vous demandez un livre précis, le
bibliothécaire vous le tend, ou vous répond qu'il n'existe pas. Go est très utilisé
côté guichet : sa bibliothèque standard contient un serveur web complet, prêt pour la
production.

## Un gestionnaire

Un **gestionnaire** (en anglais *handler*) est une fonction qui reçoit deux paramètres :

- `w http.ResponseWriter` : là où l'on **écrit** la réponse ;
- `r *http.Request` : la **requête** reçue (son chemin, ses paramètres…).

```go norun
func accueil(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintln(w, "Bienvenue sur mon site !")
}
```

`fmt.Fprintln(w, …)` écrit dans la réponse, exactement comme on écrit dans un fichier
(voir [les fichiers](fichiers.md#ouvrir-puis-fermer-avec-defer)).

## Lancer un serveur

`http.HandleFunc` associe un chemin à un gestionnaire. `http.ListenAndServe` démarre
le serveur sur un **port** (ici 8080) et attend les requêtes, sans jamais s'arrêter.

```go norun
package main

import (
	"fmt"
	"log"
	"net/http"
)

func accueil(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintln(w, "Bienvenue sur mon site !")
}

func saluer(w http.ResponseWriter, r *http.Request) {
	nom := r.URL.Query().Get("nom")
	fmt.Fprintf(w, "Bonjour, %s !\n", nom)
}

func main() {
	http.HandleFunc("/", accueil)
	http.HandleFunc("/saluer", saluer)

	fmt.Println("Serveur sur http://localhost:8080")
	log.Fatal(http.ListenAndServe(":8080", nil))
}
```

Lancez `go run main.go`, puis ouvrez `http://localhost:8080/saluer?nom=Léa` dans votre
navigateur, ou utilisez `curl` dans un autre terminal :

```sh
curl "http://localhost:8080/saluer?nom=Lea"
```

```text
Bonjour, Lea !
```

- `r.URL.Query().Get("nom")` lit le paramètre `nom` de l'[URL](../debuter/glossaire.md#url)
  (après le `?`) ;
- `ListenAndServe` ne rend la main qu'en cas d'erreur ; `log.Fatal` affiche alors
  l'erreur et arrête le programme ;
- Ctrl+C arrête le serveur.

> **Note :** ce bloc est `norun` : un serveur tourne sans fin. La suite montre comment
> le tester quand même.

## Tester un gestionnaire avec httptest

Un gestionnaire n'est qu'une fonction. Pour l'essayer, pas besoin de vrai serveur : le
paquet `net/http/httptest` fabrique une fausse requête et un **enregistreur** de
réponse. C'est comme répéter une pièce de théâtre sans public : tout se passe pareil,
mais en coulisses.

```go
package main

import (
	"fmt"
	"net/http"
	"net/http/httptest"
)

func saluer(w http.ResponseWriter, r *http.Request) {
	nom := r.URL.Query().Get("nom")
	if nom == "" {
		http.Error(w, "paramètre nom manquant", http.StatusBadRequest)
		return
	}
	fmt.Fprintf(w, "Bonjour, %s !\n", nom)
}

func main() {
	for _, url := range []string{"/saluer?nom=L%C3%A9a", "/saluer"} {
		requete := httptest.NewRequest("GET", url, nil)
		enregistreur := httptest.NewRecorder()

		saluer(enregistreur, requete)

		fmt.Println("Code :", enregistreur.Code)
		fmt.Print("Corps : ", enregistreur.Body.String())
	}
}
```

```sortie
Code : 200
Corps : Bonjour, Léa !
Code : 400
Corps : paramètre nom manquant
```

- `httptest.NewRequest(méthode, url, corps)` crée la requête ;
- `httptest.NewRecorder()` garde le code et le corps de la réponse ;
- `http.Error` écrit un message d'erreur avec un code, ici `400` (« mauvaise
  requête »). Les constantes `http.StatusOK`, `http.StatusNotFound`… évitent de
  retenir les numéros.

Dans `L%C3%A9a`, `%C3%A9` est la façon d'écrire `é` dans une URL.

## Une API JSON

Une [API](../debuter/glossaire.md#api) web répond en général en JSON (voir
[JSON et CSV](formats.md)). Le gestionnaire indique le type du contenu dans l'en-tête
`Content-Type`, puis encode la réponse avec `json.NewEncoder(w).Encode(...)`, qui
écrit directement dans `w`.

```go
package main

import (
	"encoding/json"
	"fmt"
	"net/http"
	"net/http/httptest"
)

type Livre struct {
	ID    int    `json:"id"`
	Titre string `json:"titre"`
}

var livres = []Livre{
	{1, "Dune"},
	{2, "Le Petit Prince"},
}

func listerLivres(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(livres)
}

func main() {
	rec := httptest.NewRecorder()
	listerLivres(rec, httptest.NewRequest("GET", "/livres", nil))

	fmt.Println(rec.Code, rec.Header().Get("Content-Type"))
	fmt.Print(rec.Body.String())

	var recus []Livre
	json.NewDecoder(rec.Body).Decode(&recus)
	fmt.Println("Premier titre :", recus[0].Titre)
}
```

```sortie
200 application/json
[{"id":1,"titre":"Dune"},{"id":2,"titre":"Le Petit Prince"}]
Premier titre : Dune
```

Le test relit le JSON reçu avec `json.NewDecoder`, comme le ferait un vrai client.

## Les routes avec méthode et paramètres

Depuis Go 1.22, un **motif de route** peut préciser la méthode HTTP et contenir des
parties variables entre accolades : `"GET /livres/{id}"`. Dans le gestionnaire,
`r.PathValue("id")` donne la valeur trouvée dans le chemin.

Les routes s'enregistrent sur un **routeur**, `http.NewServeMux()`. Le routeur est
lui-même un gestionnaire : on peut donc le tester avec `httptest`, routes comprises.

```go
//go:debug httpmuxgo121=0
package main

import (
	"encoding/json"
	"fmt"
	"net/http"
	"net/http/httptest"
	"strconv"
)

type Livre struct {
	ID    int    `json:"id"`
	Titre string `json:"titre"`
}

var livres = map[int]Livre{
	1: {1, "Dune"},
	2: {2, "Le Petit Prince"},
}

func trouverLivre(w http.ResponseWriter, r *http.Request) {
	id, err := strconv.Atoi(r.PathValue("id"))
	if err != nil {
		http.Error(w, "identifiant invalide", http.StatusBadRequest)
		return
	}
	livre, ok := livres[id]
	if !ok {
		http.Error(w, "livre introuvable", http.StatusNotFound)
		return
	}
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(livre)
}

func main() {
	routeur := http.NewServeMux()
	routeur.HandleFunc("GET /livres/{id}", trouverLivre)

	essais := []struct{ methode, url string }{
		{"GET", "/livres/1"},
		{"GET", "/livres/9"},
		{"GET", "/livres/abc"},
		{"DELETE", "/livres/1"},
	}
	for _, e := range essais {
		rec := httptest.NewRecorder()
		routeur.ServeHTTP(rec, httptest.NewRequest(e.methode, e.url, nil))
		fmt.Printf("%-6s %-11s -> %d %s", e.methode, e.url, rec.Code, rec.Body.String())
	}
}
```

```sortie
GET    /livres/1   -> 200 {"id":1,"titre":"Dune"}
GET    /livres/9   -> 404 livre introuvable
GET    /livres/abc -> 400 identifiant invalide
DELETE /livres/1   -> 405 Method Not Allowed
```

Le routeur refuse tout seul la méthode `DELETE`, avec le code `405` (« méthode non
autorisée »).

> **Note :** la première ligne, `//go:debug httpmuxgo121=0`, n'est utile que parce que
> les exemples du site sont exécutés hors module. Dans un projet créé avec
> `go mod init` (voir [les modules](modules.md)), les motifs de Go 1.22 sont actifs
> d'office : ne l'écrivez pas.

Pour lancer ce routeur pour de vrai, donnez-le à `ListenAndServe` à la place de `nil` :

```go norun
log.Fatal(http.ListenAndServe(":8080", routeur))
```

## Appeler un site avec http.Get

Go sait aussi être le client. `http.Get` envoie une requête et renvoie la réponse.
Fermez toujours son corps avec `defer`, et lisez-le avec `io.ReadAll`.

```go norun
package main

import (
	"fmt"
	"io"
	"log"
	"net/http"
)

func main() {
	reponse, err := http.Get("https://example.com")
	if err != nil {
		log.Fatal(err)
	}
	defer reponse.Body.Close()

	corps, err := io.ReadAll(reponse.Body)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println("Code :", reponse.StatusCode)
	fmt.Println("Début :", string(corps[:15]))
}
```

```text
Code : 200
Début : <!doctype html>
```

Ce bloc a besoin d'Internet. Pour essayer un client sans réseau, `httptest.NewServer`
lance un vrai petit serveur local, le temps du programme :

```go
package main

import (
	"fmt"
	"io"
	"net/http"
	"net/http/httptest"
)

func main() {
	serveur := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprintln(w, "pong")
	}))
	defer serveur.Close()

	reponse, err := http.Get(serveur.URL + "/ping")
	if err != nil {
		fmt.Println(err)
		return
	}
	defer reponse.Body.Close()

	corps, _ := io.ReadAll(reponse.Body)
	fmt.Println(reponse.StatusCode, string(corps))
}
```

```sortie
200 pong
```

## Et les frameworks ?

La bibliothèque standard suffit pour beaucoup de projets. Des **frameworks** comme Gin
ou Echo ajoutent des facilités (validation, groupes de routes, intergiciels). Ce sont
des paquets externes, à installer avec `go get` :

```go norun
package main

import "github.com/gin-gonic/gin"

func main() {
	r := gin.Default()
	r.GET("/livres/:id", func(c *gin.Context) {
		c.JSON(200, gin.H{"id": c.Param("id")})
	})
	r.Run(":8080")
}
```

> **Astuce :** apprenez d'abord `net/http`. Les frameworks Go reposent tous dessus :
> ce que vous savez restera utile.

## Erreurs fréquentes

### Oublier return après http.Error

```go norun
func saluer(w http.ResponseWriter, r *http.Request) {
	nom := r.URL.Query().Get("nom")
	if nom == "" {
		http.Error(w, "paramètre nom manquant", http.StatusBadRequest)
	}
	fmt.Fprintf(w, "Bonjour, %s !\n", nom)
}
```

```text
400
paramètre nom manquant
Bonjour,  !
```

`http.Error` écrit la réponse, mais n'arrête **pas** la fonction. La suite s'exécute
et ajoute `Bonjour,  !`. Écrivez `return` juste après `http.Error`.

### Lancer deux serveurs sur le même port

```go norun
err := http.ListenAndServe(":8080", nil)
fmt.Println(err)
```

```text
listen tcp :8080: bind: address already in use
```

Un autre programme (souvent votre serveur précédent, toujours lancé) occupe déjà le
port 8080. Arrêtez-le avec Ctrl+C, ou choisissez un autre port (`":8081"`).

### Deux routes identiques

```go norun
mux := http.NewServeMux()
mux.HandleFunc("GET /livres/{id}", livre)
mux.HandleFunc("GET /livres/{numero}", livre)
```

```text
panic: pattern "GET /livres/{numero}" (registered at …/main.go:16) conflicts with pattern "GET /livres/{id}" (registered at …/main.go:15):
	GET /livres/{numero} matches the same requests as GET /livres/{id}
```

Le nom entre accolades ne compte pas : les deux motifs reconnaissent les mêmes chemins.
Le routeur refuse l'ambiguïté dès le démarrage. Gardez une seule route.

## Testez-vous

**Question 1 :** quels sont les deux paramètres d'un gestionnaire ?

> **Réponse**
>
> `w http.ResponseWriter`, où l'on écrit la réponse, et `r *http.Request`, la requête
> reçue.

**Question 2 :** pourquoi `httptest.NewRecorder` est-il pratique ?

> **Réponse**
>
> Il permet d'appeler un gestionnaire comme une fonction ordinaire et de vérifier son
> code et son corps, sans lancer de serveur ni ouvrir de navigateur.

**Question 3 :** écrivez un gestionnaire `heure` qui répond `Il est 12h00` avec le code
200, et testez-le avec `httptest`.

> **Réponse**
>
> ```go
> package main
>
> import (
> 	"fmt"
> 	"net/http"
> 	"net/http/httptest"
> )
>
> func heure(w http.ResponseWriter, r *http.Request) {
> 	fmt.Fprintln(w, "Il est 12h00")
> }
>
> func main() {
> 	rec := httptest.NewRecorder()
> 	heure(rec, httptest.NewRequest("GET", "/heure", nil))
> 	fmt.Print(rec.Code, " ", rec.Body.String())
> }
> ```
>
> ```sortie
> 200 Il est 12h00
> ```

## En résumé

- Un gestionnaire est une fonction `func(w http.ResponseWriter, r *http.Request)`.
- `http.HandleFunc` associe un chemin à un gestionnaire, `http.ListenAndServe` lance le
  serveur.
- `httptest.NewRequest` et `httptest.NewRecorder` testent un gestionnaire sans réseau.
- Depuis Go 1.22, les routes s'écrivent `"GET /livres/{id}"` et `r.PathValue("id")` lit
  la valeur.
- `http.Get` appelle un site ; fermez toujours `reponse.Body`.

Suite : [tester son code](tests.md). Comparez avec [le web en Ruby](../ruby/web.md).
