# JSON et CSV

Cette page montre comment transformer vos structures Go en JSON et inversement, puis comment lire et écrire des tableaux au format CSV.

> **Objectifs :**
>
> - vous saurez convertir une structure en JSON avec `json.Marshal` et `json.MarshalIndent` ;
> - vous saurez relire du JSON dans une structure avec `json.Unmarshal` ;
> - vous saurez choisir les noms des champs avec les étiquettes `json:"..."` et `omitempty` ;
> - vous saurez lire et écrire du CSV avec `encoding/csv` ;
> - vous saurez choisir le bon format selon la situation.

## Pourquoi des formats ?

Une structure Go n'existe que dans la mémoire du programme. Pour l'enregistrer dans un
fichier ou l'envoyer sur le réseau, il faut la transformer en texte : c'est la
[sérialisation](../debuter/glossaire.md#serialisation). Relire ce texte pour retrouver
la structure, c'est la désérialisation.

C'est comme ranger un meuble en kit dans un carton avec sa notice : on peut le
transporter, puis le remonter à l'identique à l'arrivée.

Deux formats texte sont partout :

- le [JSON](../debuter/glossaire.md#json), pour des données imbriquées (objets, listes) ;
- le CSV (*comma-separated values*, « valeurs séparées par des virgules »), pour des
  tableaux, comme une feuille de calcul.

## Structure vers JSON : json.Marshal

Le paquet `encoding/json` convertit une structure avec `json.Marshal`. Il renvoie des
octets (`[]byte`) et une erreur.

```go
package main

import (
	"encoding/json"
	"fmt"
)

type Livre struct {
	Titre  string
	Auteur string
	Pages  int
	Lu     bool
}

func main() {
	l := Livre{Titre: "Dune", Auteur: "Frank Herbert", Pages: 712, Lu: true}

	data, err := json.Marshal(l)
	if err != nil {
		fmt.Println("Erreur :", err)
		return
	}
	fmt.Println(string(data))
}
```

```sortie
{"Titre":"Dune","Auteur":"Frank Herbert","Pages":712,"Lu":true}
```

> **À retenir :** seuls les champs qui commencent par une **majuscule** (les champs
> exportés) sont convertis. Un champ `titre` en minuscule est invisible pour
> `encoding/json`.

## Les étiquettes json

En JSON, on écrit souvent les clés en minuscules. Une **étiquette** (en anglais *tag*)
placée après le type, entre accents graves, donne le nom à utiliser :

- `json:"titre"` : la clé s'appelle `titre` ;
- `json:"note,omitempty"` : la clé est omise si la valeur est vide (`0`, `""`, `false`,
  `nil`) ;
- `json:"-"` : le champ n'est jamais converti.

`json.MarshalIndent` produit un JSON indenté, plus lisible. Ses deux derniers arguments
sont le préfixe de chaque ligne (ici rien) et l'indentation (ici deux espaces).

```go
package main

import (
	"encoding/json"
	"fmt"
)

type Livre struct {
	Titre      string   `json:"titre"`
	Auteur     string   `json:"auteur"`
	Note       int      `json:"note,omitempty"`
	Genres     []string `json:"genres"`
	MotDePasse string   `json:"-"`
}

func main() {
	livres := []Livre{
		{Titre: "Dune", Auteur: "Frank Herbert", Note: 5, Genres: []string{"SF"}},
		{Titre: "Le Petit Prince", Auteur: "Saint-Exupéry", Genres: []string{"conte", "jeunesse"}},
	}

	data, err := json.MarshalIndent(livres, "", "  ")
	if err != nil {
		fmt.Println("Erreur :", err)
		return
	}
	fmt.Println(string(data))
}
```

```sortie
[
  {
    "titre": "Dune",
    "auteur": "Frank Herbert",
    "note": 5,
    "genres": [
      "SF"
    ]
  },
  {
    "titre": "Le Petit Prince",
    "auteur": "Saint-Exupéry",
    "genres": [
      "conte",
      "jeunesse"
    ]
  }
]
```

Le second livre n'a pas de `note` : grâce à `omitempty`, la clé n'apparaît pas. Le
champ `MotDePasse` n'apparaît jamais.

## JSON vers structure : json.Unmarshal

`json.Unmarshal` fait le chemin inverse. Il remplit une variable existante : donnez-lui
son **adresse** avec `&` (voir [les pointeurs](pointeurs.md)), sinon il ne pourrait pas
la modifier.

```go
package main

import (
	"encoding/json"
	"fmt"
)

type Livre struct {
	Titre  string   `json:"titre"`
	Pages  int      `json:"pages"`
	Genres []string `json:"genres"`
}

func main() {
	texte := `{"titre": "Dune", "pages": 712, "genres": ["SF", "aventure"], "isbn": "978-2"}`

	var l Livre
	err := json.Unmarshal([]byte(texte), &l)
	if err != nil {
		fmt.Println("JSON invalide :", err)
		return
	}
	fmt.Println(l.Titre, "-", l.Pages, "pages")
	fmt.Println("Genres :", l.Genres)
}
```

```sortie
Dune - 712 pages
Genres : [SF aventure]
```

La clé `isbn` n'existe pas dans la structure : elle est simplement ignorée.

### Lire et écrire un fichier JSON

On combine avec `os.WriteFile` et `os.ReadFile` (voir [les fichiers](fichiers.md)).

```go
package main

import (
	"encoding/json"
	"fmt"
	"os"
)

type Tache struct {
	Titre string `json:"titre"`
	Faite bool   `json:"faite"`
}

func main() {
	taches := []Tache{{"Courses", true}, {"Répondre à Léa", false}}
	data, _ := json.MarshalIndent(taches, "", "  ")
	os.WriteFile("taches.json", data, 0644)

	contenu, err := os.ReadFile("taches.json")
	if err != nil {
		fmt.Println(err)
		return
	}
	var relues []Tache
	if err := json.Unmarshal(contenu, &relues); err != nil {
		fmt.Println(err)
		return
	}
	for _, t := range relues {
		fmt.Printf("[%t] %s\n", t.Faite, t.Titre)
	}
}
```

```sortie
[true] Courses
[false] Répondre à Léa
```

## JSON et maps

Quand vous ne connaissez pas la forme des données à l'avance, utilisez une
[map](maps.md). Une `map[string]int` devient un objet JSON ; `json.Marshal` trie ses
clés par ordre alphabétique, le résultat est donc toujours le même.

```go
package main

import (
	"encoding/json"
	"fmt"
)

func main() {
	scores := map[string]int{"zoé": 12, "alice": 18, "bob": 9}
	data, _ := json.Marshal(scores)
	fmt.Println(string(data))

	var inconnu map[string]any
	json.Unmarshal([]byte(`{"nom": "Alice", "age": 30, "admin": false}`), &inconnu)
	fmt.Println(inconnu["nom"], inconnu["age"], inconnu["admin"])
	fmt.Printf("age est de type %T\n", inconnu["age"])
}
```

```sortie
{"alice":18,"bob":9,"zoé":12}
Alice 30 false
age est de type float64
```

> **Attention :** dans une `map[string]any`, **tous** les nombres JSON deviennent des
> `float64`. Quand la forme des données est connue, préférez une structure : c'est plus
> sûr et plus clair.

## Lire du CSV

Un fichier CSV ressemble à ceci : une ligne d'en-tête, puis une ligne par
enregistrement, les valeurs séparées par des virgules.

`encoding/csv` gère les cas délicats, comme une virgule à l'intérieur de guillemets.
`csv.NewReader` lit depuis n'importe quelle source : un fichier ouvert, ou un texte
avec `strings.NewReader`. `ReadAll` renvoie toutes les lignes, chacune étant une
tranche de `string`.

```go
package main

import (
	"encoding/csv"
	"fmt"
	"strconv"
	"strings"
)

func main() {
	texte := `nom,ville,note
Alice,Lyon,15
Bob,"Paris, 11e",12
Chloé,Nantes,17
`
	r := csv.NewReader(strings.NewReader(texte))
	lignes, err := r.ReadAll()
	if err != nil {
		fmt.Println("CSV invalide :", err)
		return
	}

	total := 0
	for _, ligne := range lignes[1:] {
		note, _ := strconv.Atoi(ligne[2])
		total += note
		fmt.Printf("%-6s %-11s %d\n", ligne[0], ligne[1], note)
	}
	fmt.Println("Moyenne :", float64(total)/float64(len(lignes)-1))
}
```

```sortie
Alice  Lyon        15
Bob    Paris, 11e  12
Chloé  Nantes      17
Moyenne : 14.666666666666666
```

`lignes[1:]` saute la ligne d'en-tête. Toutes les valeurs sont du texte : convertissez
les nombres avec `strconv.Atoi`.

## Écrire du CSV

`csv.NewWriter` écrit dans un fichier (ou à l'écran avec `os.Stdout`). Il garde les
lignes en réserve : appelez `Flush()` à la fin pour tout écrire, puis vérifiez
`Error()`.

```go
package main

import (
	"encoding/csv"
	"fmt"
	"os"
)

func main() {
	f, err := os.Create("ventes.csv")
	if err != nil {
		fmt.Println(err)
		return
	}
	defer f.Close()

	w := csv.NewWriter(f)
	w.Write([]string{"produit", "prix"})
	w.Write([]string{"pain", "1.20"})
	w.Write([]string{"café, moulu", "4.50"})
	w.Flush()
	if err := w.Error(); err != nil {
		fmt.Println(err)
		return
	}

	contenu, _ := os.ReadFile("ventes.csv")
	fmt.Print(string(contenu))
}
```

```sortie
produit,prix
pain,1.20
"café, moulu",4.50
```

Remarquez les guillemets autour de `café, moulu` : le paquet les ajoute tout seul,
pour que la virgule ne soit pas prise pour un séparateur.

## Quel format choisir ?

| Situation | Format conseillé |
|-----------|------------------|
| Un tableau simple (lignes et colonnes) à ouvrir dans un tableur | CSV |
| Des données imbriquées (listes dans des objets) | JSON |
| Échanger avec une API web | JSON |
| Sauvegarder l'état d'un programme (réglages, tâches) | JSON |
| Exporter des milliers de lignes de mesures | CSV |

## Erreurs fréquentes

### Des champs en minuscules

```go norun
type Livre struct {
	titre  string
	auteur string
}

func main() {
	l := Livre{titre: "Dune", auteur: "Herbert"}
	data, err := json.Marshal(l)
	fmt.Println(string(data), err)
}
```

```text
{} <nil>
```

Aucune erreur, mais un objet vide : les champs en minuscules ne sont pas exportés.
Mettez une majuscule (`Titre`) et, si vous voulez une clé en minuscules, ajoutez une
étiquette : `json:"titre"`.

### Oublier le & avec json.Unmarshal

```go norun
var l Livre
err := json.Unmarshal([]byte(`{"titre":"Dune"}`), l)
fmt.Println("Erreur :", err)
```

```text
Erreur : json: Unmarshal(non-pointer main.Livre)
```

`json.Unmarshal` doit modifier la variable : passez son adresse, `&l`.

### Un type qui ne correspond pas

```go norun
type Livre struct {
	Pages int `json:"pages"`
}

func main() {
	var l Livre
	err := json.Unmarshal([]byte(`{"pages":"300"}`), &l)
	fmt.Println("Erreur :", err)
}
```

```text
Erreur : json: cannot unmarshal string into Go struct field Livre.pages of type int
```

Dans le JSON, `"300"` est un texte (entre guillemets), pas un nombre. Corrigez les
données (`"pages": 300`) ou le type du champ.

### Une ligne CSV incomplète

```go norun
texte := "nom,age\nAlice,30\nBob\n"
r := csv.NewReader(strings.NewReader(texte))
lignes, err := r.ReadAll()
fmt.Println(lignes)
fmt.Println("Erreur :", err)
```

```text
[]
Erreur : record on line 3: wrong number of fields
```

Toutes les lignes doivent avoir le même nombre de colonnes que la première. Le message
donne le numéro de la ligne fautive : complétez-la (`Bob,25`).

## Testez-vous

**Question 1 :** à quoi sert `omitempty` dans `json:"note,omitempty"` ?

> **Réponse**
>
> À ne pas écrire la clé `note` dans le JSON quand le champ a sa valeur vide (`0` pour
> un entier).

**Question 2 :** pourquoi faut-il appeler `Flush()` sur un `csv.Writer` ?

> **Réponse**
>
> Le writer garde les lignes en réserve en mémoire. `Flush()` les écrit vraiment dans
> le fichier. Sans lui, le fichier peut rester vide ou incomplet.

**Question 3 :** décodez `{"nom": "Pixel", "age": 3}` dans une structure `Chat` et
affichez `Pixel a 3 ans`.

> **Réponse**
>
> ```go
> package main
>
> import (
> 	"encoding/json"
> 	"fmt"
> )
>
> type Chat struct {
> 	Nom string `json:"nom"`
> 	Age int    `json:"age"`
> }
>
> func main() {
> 	var c Chat
> 	err := json.Unmarshal([]byte(`{"nom": "Pixel", "age": 3}`), &c)
> 	if err != nil {
> 		fmt.Println(err)
> 		return
> 	}
> 	fmt.Println(c.Nom, "a", c.Age, "ans")
> }
> ```
>
> ```sortie
> Pixel a 3 ans
> ```

## En résumé

- `json.Marshal` et `json.MarshalIndent` transforment une valeur Go en JSON.
- `json.Unmarshal(data, &v)` remplit `v` : n'oubliez pas le `&`.
- Seuls les champs en majuscule sont convertis ; les étiquettes `json:"nom"` choisissent
  les clés, `omitempty` omet les valeurs vides.
- `csv.NewReader` et `csv.NewWriter` lisent et écrivent des tableaux ; appelez `Flush()`.
- JSON pour les données imbriquées et les API, CSV pour les tableaux simples.

Suite : [les expressions régulières](regex.md). Comparez avec
[les formats en Ruby](../ruby/formats.md).
