# Une base de données en Go

Cette page montre comment un programme Go parle à une base de données avec le paquet `database/sql`, puis comment cacher la base derrière une interface `Store` que l'on peut tester sans base du tout.

> **Objectifs :**
>
> - comprendre le rôle de `database/sql` et d'un pilote ;
> - ouvrir une base, écrire avec `Exec`, lire avec `QueryRow` et `Query` ;
> - utiliser des paramètres `?` au lieu de coller du texte dans le SQL ;
> - écrire une interface `Store` et une version en mémoire (map + verrou) ;
> - voir comment une version SQL respecte la même interface.

Si vous n'avez jamais vu de SQL, lisez d'abord
[les bases de données](../debuter/bases-de-donnees.md) dans le parcours Débuter.

## Pourquoi une base de données ?

Une variable disparaît quand le programme s'arrête. Un [fichier](fichiers.md) reste,
mais chercher dedans devient lent et compliqué quand il grossit.

Une **base de données** est une bibliothèque bien rangée, avec un bibliothécaire. Vous
ne fouillez pas les rayons vous-même : vous demandez « les livres de Hugo, triés par
titre », et il vous les apporte. La langue pour lui parler s'appelle **SQL**.

```text
 votre programme ── « SELECT titre FROM livres » ──►  base de données
                 ◄──────── les lignes trouvées ─────
```

> **À retenir :** la base garde les données entre deux lancements et sait chercher vite.
> Votre programme lui envoie des ordres SQL.

## database/sql et les pilotes

Go a un seul paquet pour toutes les bases : `database/sql`. Mais il ne sait parler à
**aucune** base tout seul. Il lui faut un **pilote** (*driver*) : un petit
[paquet](../debuter/glossaire.md#paquet) qui connaît une base précise.

C'est comme une prise électrique de voyage : `database/sql` est votre appareil, le
pilote est l'adaptateur du pays.

```text
           votre code
               │   (toujours les mêmes fonctions : Open, Exec, Query…)
         database/sql
          ┌────┴─────────────┐
   pilote SQLite      pilote PostgreSQL
          │                  │
    fichier .db        serveur PostgreSQL
```

Les pilotes ne sont pas dans la bibliothèque standard. On les ajoute avec `go get`
(voir [les modules](modules.md)) :

| Base | Pilote courant | Nom à donner à `sql.Open` | Paramètres |
|---|---|---|---|
| SQLite (un simple fichier) | `modernc.org/sqlite` | `"sqlite"` | `?` |
| PostgreSQL (un serveur) | `github.com/jackc/pgx/v5/stdlib` | `"pgx"` | `$1`, `$2`… |

On importe le pilote avec un **tiret bas** : `_ "modernc.org/sqlite"`. Le `_` veut dire
« je n'utilise aucun nom de ce paquet, mais charge-le ». En se chargeant, le pilote
s'inscrit auprès de `database/sql`.

### Sans pilote, que se passe-t-il ?

Ce programme utilise seulement la bibliothèque standard. Il demande un pilote qui n'a
pas été importé :

```go
package main

import (
	"database/sql"
	"fmt"
)

func main() {
	db, err := sql.Open("sqlite", "livres.db")
	fmt.Println(db == nil)
	fmt.Println(err)
}
```

```sortie
true
sql: unknown driver "sqlite" (forgotten import?)
```

Le message le dit lui-même : « pilote inconnu (import oublié ?) ».

> **À retenir :** `database/sql` + un pilote importé avec `_`. Le nom donné à
> `sql.Open` dépend du pilote.

## Ouvrir la base

Les exemples de cette section ont besoin d'un pilote : ils sont montrés sans être
exécutés, avec ce qu'ils affichent vraiment.

```go norun
package main

import (
	"database/sql"
	"fmt"
	"log"

	_ "modernc.org/sqlite"
)

func main() {
	db, err := sql.Open("sqlite", "livres.db")
	if err != nil {
		log.Fatal(err)
	}
	defer db.Close()

	if err := db.Ping(); err != nil {
		log.Fatal(err)
	}
	fmt.Println("base ouverte")
}
```

```text
base ouverte
```

- `sql.Open` **prépare** l'accès, mais ne se connecte pas encore.
- `db.Ping()` vérifie que la base répond vraiment.
- `db` n'est pas une seule connexion : c'est un **réservoir** de connexions, sûr avec
  plusieurs [goroutines](concurrence.md). On l'ouvre une fois, au démarrage, et on le
  garde.

Pour PostgreSQL, seules deux lignes changent :

```go norun
import _ "github.com/jackc/pgx/v5/stdlib"

db, err := sql.Open("pgx", "postgres://lea:secret@localhost:5432/biblio")
```

## Écrire : Exec

`db.Exec` envoie un ordre qui ne renvoie pas de lignes : créer une table, insérer,
modifier, supprimer.

```go norun
_, err = db.Exec(`CREATE TABLE IF NOT EXISTS livres (
	id     INTEGER PRIMARY KEY,
	titre  TEXT NOT NULL,
	auteur TEXT NOT NULL
)`)
if err != nil {
	log.Fatal(err)
}

res, err := db.Exec("INSERT INTO livres (titre, auteur) VALUES (?, ?)",
	"Les Misérables", "Victor Hugo")
if err != nil {
	log.Fatal(err)
}
id, _ := res.LastInsertId()
n, _ := res.RowsAffected()
fmt.Println("nouvel id :", id, "- lignes ajoutées :", n)
```

```text
nouvel id : 1 - lignes ajoutées : 1
```

Les `?` sont des **trous** remplis par les valeurs qui suivent, dans l'ordre :

| Trou | Valeur |
|---|---|
| premier `?` | `"Les Misérables"` |
| deuxième `?` | `"Victor Hugo"` |

Avec PostgreSQL, les trous s'écrivent `$1`, `$2`, et l'id se récupère avec
`… RETURNING id` et `QueryRow`.

### Ne collez jamais du texte dans le SQL

Pourquoi des trous, et pas `"… VALUES ('" + titre + "')"` ? Parce qu'un visiteur
pourrait taper un titre comme `x'); DROP TABLE livres; --`. Collé dans l'ordre SQL, ce
titre devient un **ordre** qui efface la table. C'est l'**injection SQL** (voir
[la sécurité](../debuter/securite.md)).

Avec `?`, le pilote envoie la valeur **à part** : elle reste toujours une simple
donnée, même si elle contient des apostrophes.

> **À retenir :** toujours des paramètres `?` (ou `$1`). Jamais de `+` ni de
> `fmt.Sprintf` pour fabriquer un ordre SQL avec des données.

## Lire une ligne : QueryRow

`db.QueryRow` renvoie **une** ligne. `Scan` copie chaque colonne dans une variable,
dans l'ordre du `SELECT`. On donne l'**adresse** des variables (`&titre`) pour que
`Scan` puisse les remplir (voir [les pointeurs](pointeurs.md)).

```go norun
var titre, auteur string
err = db.QueryRow("SELECT titre, auteur FROM livres WHERE id = ?", 1).
	Scan(&titre, &auteur)
if errors.Is(err, sql.ErrNoRows) {
	fmt.Println("aucun livre avec cet id")
} else if err != nil {
	log.Fatal(err)
} else {
	fmt.Println(titre, "-", auteur)
}
```

```text
Les Misérables - Victor Hugo
```

Avec l'id `99`, qui n'existe pas, `Scan` renvoie l'erreur `sql.ErrNoRows`, et le
programme affiche `aucun livre avec cet id`. Ce n'est pas une panne : c'est la réponse
« rien trouvé ».

```go
package main

import (
	"database/sql"
	"fmt"
)

func main() {
	fmt.Println(sql.ErrNoRows)
}
```

```sortie
sql: no rows in result set
```

## Lire plusieurs lignes : Query

`db.Query` renvoie un curseur `rows`. On avance ligne par ligne avec `rows.Next()`,
comme un doigt qui descend une liste.

```go norun
rows, err := db.Query("SELECT id, titre FROM livres ORDER BY id")
if err != nil {
	log.Fatal(err)
}
defer rows.Close()

for rows.Next() {
	var id int
	var titre string
	if err := rows.Scan(&id, &titre); err != nil {
		log.Fatal(err)
	}
	fmt.Println(id, titre)
}
if err := rows.Err(); err != nil {
	log.Fatal(err)
}
```

```text
1 Les Misérables
2 Germinal
3 Le Petit Prince
```

| Tour | `rows.Next()` | id | titre |
|---|---|---|---|
| 1 | `true` | 1 | Les Misérables |
| 2 | `true` | 2 | Germinal |
| 3 | `true` | 3 | Le Petit Prince |
| 4 | `false` : plus de ligne | | |

Les trois règles à ne pas oublier :

1. `defer rows.Close()` juste après `Query` : sinon la connexion reste occupée ;
2. `rows.Scan` avec autant de variables que de colonnes ;
3. `rows.Err()` après la boucle : une erreur a pu arrêter la lecture en route.

> **À retenir :** `QueryRow(…).Scan(…)` pour une ligne ; `Query` + `for rows.Next()` +
> `defer rows.Close()` pour plusieurs.

## Cacher la base derrière une interface

Mettre des `db.Query` partout dans le programme pose deux problèmes :

- pour tester, il faut une vraie base ;
- changer de base (SQLite → PostgreSQL) oblige à tout relire.

La solution : une [interface](interfaces.md) qui dit **ce qu'on veut faire** avec les
livres, sans dire **comment**. On l'appelle souvent `Store` (« magasin »).

C'est comme un guichet de consigne : vous donnez un sac, on vous rend un ticket ; avec
le ticket, on vous rend le sac. Vous ne savez pas si les sacs sont rangés dans une
armoire ou à la cave. Et vous n'avez pas besoin de le savoir.

```text
             main, le serveur web, les tests…
                          │  n'utilisent que :
          ┌───────────────┴────────────────┐
          │  Store : Add  Get  List  Delete │
          └───────┬────────────────┬───────┘
                  │                │
         MemoireStore          SQLStore
     (map + verrou, tests)   (database/sql, production)
```

### L'interface et la version en mémoire

Voici un programme complet. `MemoireStore` range les livres dans une [map](maps.md),
protégée par un verrou `sync.RWMutex` (voir [sync](sync.md)) : plusieurs goroutines
pourront l'utiliser en même temps.

```go
package main

import (
	"errors"
	"fmt"
	"sort"
	"sync"
)

type Livre struct {
	ID     int
	Titre  string
	Auteur string
}

var ErrIntrouvable = errors.New("livre introuvable")

// Store dit ce qu'on peut faire avec les livres, pas comment.
type Store interface {
	Add(l Livre) (int, error)
	Get(id int) (Livre, error)
	List() ([]Livre, error)
	Delete(id int) error
}

// MemoireStore garde les livres dans une map.
type MemoireStore struct {
	mu      sync.RWMutex
	livres  map[int]Livre
	suivant int
}

func NewMemoireStore() *MemoireStore {
	return &MemoireStore{livres: map[int]Livre{}, suivant: 1}
}

func (s *MemoireStore) Add(l Livre) (int, error) {
	s.mu.Lock()
	defer s.mu.Unlock()
	l.ID = s.suivant
	s.livres[l.ID] = l
	s.suivant++
	return l.ID, nil
}

func (s *MemoireStore) Get(id int) (Livre, error) {
	s.mu.RLock()
	defer s.mu.RUnlock()
	l, ok := s.livres[id]
	if !ok {
		return Livre{}, fmt.Errorf("get %d : %w", id, ErrIntrouvable)
	}
	return l, nil
}

func (s *MemoireStore) List() ([]Livre, error) {
	s.mu.RLock()
	defer s.mu.RUnlock()
	liste := make([]Livre, 0, len(s.livres))
	for _, l := range s.livres {
		liste = append(liste, l)
	}
	// une map n'a pas d'ordre : on trie par id
	sort.Slice(liste, func(i, j int) bool { return liste[i].ID < liste[j].ID })
	return liste, nil
}

func (s *MemoireStore) Delete(id int) error {
	s.mu.Lock()
	defer s.mu.Unlock()
	if _, ok := s.livres[id]; !ok {
		return fmt.Errorf("delete %d : %w", id, ErrIntrouvable)
	}
	delete(s.livres, id)
	return nil
}

// afficher ne connaît que l'interface Store.
func afficher(s Store) {
	liste, _ := s.List()
	for _, l := range liste {
		fmt.Printf("  %d. %s (%s)\n", l.ID, l.Titre, l.Auteur)
	}
}

func main() {
	var s Store = NewMemoireStore()

	id1, _ := s.Add(Livre{Titre: "Les Misérables", Auteur: "Victor Hugo"})
	id2, _ := s.Add(Livre{Titre: "Germinal", Auteur: "Émile Zola"})
	s.Add(Livre{Titre: "Le Petit Prince", Auteur: "Saint-Exupéry"})
	fmt.Println("ajoutés :", id1, id2)

	l, err := s.Get(2)
	fmt.Println("get 2 :", l.Titre, err)

	fmt.Println("liste :")
	afficher(s)

	fmt.Println("delete 1 :", s.Delete(1))
	fmt.Println("delete 1 :", s.Delete(1))

	_, err = s.Get(1)
	fmt.Println("get 1 :", err)
	fmt.Println("introuvable ?", errors.Is(err, ErrIntrouvable))

	fmt.Println("liste :")
	afficher(s)
}
```

```sortie
ajoutés : 1 2
get 2 : Germinal <nil>
liste :
  1. Les Misérables (Victor Hugo)
  2. Germinal (Émile Zola)
  3. Le Petit Prince (Saint-Exupéry)
delete 1 : <nil>
delete 1 : delete 1 : livre introuvable
get 1 : get 1 : livre introuvable
introuvable ? true
liste :
  2. Germinal (Émile Zola)
  3. Le Petit Prince (Saint-Exupéry)
```

### Pas à pas

Suivons le contenu du magasin à chaque appel :

| Étape | Appel | suivant après | Livres dans la map | Renvoie |
|---|---|---|---|---|
| 0 | `NewMemoireStore()` | 1 | (vide) | |
| 1 | `Add(Misérables)` | 2 | 1 | `1, nil` |
| 2 | `Add(Germinal)` | 3 | 1, 2 | `2, nil` |
| 3 | `Add(Petit Prince)` | 4 | 1, 2, 3 | `3, nil` |
| 4 | `Get(2)` | 4 | 1, 2, 3 | Germinal, `nil` |
| 5 | `Delete(1)` | 4 | 2, 3 | `nil` |
| 6 | `Delete(1)` | 4 | 2, 3 | erreur : introuvable |
| 7 | `Get(1)` | 4 | 2, 3 | erreur : introuvable |

Trois détails à remarquer :

- `suivant` ne redescend jamais : un id supprimé n'est **pas** réutilisé. Une base SQL
  fait pareil.
- `List` **trie** la liste. Une map Go rend ses clés dans un ordre qui change à chaque
  fois ; sans tri, l'affichage changerait d'un lancement à l'autre.
- Les erreurs enveloppent `ErrIntrouvable` avec `%w`. L'appelant teste
  `errors.Is(err, ErrIntrouvable)` sans regarder le texte (voir [les erreurs](erreurs.md)).

Essayez : ajoutez un quatrième livre à la fin de `main`, puis un `afficher(s)`. Il reçoit
l'id `4`, pas `1` : l'id 1 n'est pas réutilisé.

> **À retenir :** le reste du programme ne connaît que `Store`. La version en mémoire
> suffit pour écrire et tester toute la logique, sans aucune base.

### Vérifier qu'un type respecte l'interface

Une ligne placée au niveau du paquet demande au compilateur de vérifier que
`*MemoireStore` a bien les quatre méthodes :

```go norun
var _ Store = (*MemoireStore)(nil)
```

S'il en manque une, la [compilation](../debuter/glossaire.md#compilation) échoue tout de
suite, avec le nom de la méthode manquante.

## La même interface, version SQL

Une version SQL est un autre type, `SQLStore`, qui contient un `*sql.DB`. Ses méthodes
ont **exactement** les mêmes signatures. Elles traduisent chaque appel en ordre SQL.

```go norun
type SQLStore struct {
	db *sql.DB
}

var _ Store = (*SQLStore)(nil)

func (s *SQLStore) Add(l Livre) (int, error) {
	res, err := s.db.Exec("INSERT INTO livres (titre, auteur) VALUES (?, ?)",
		l.Titre, l.Auteur)
	if err != nil {
		return 0, err
	}
	id, err := res.LastInsertId()
	return int(id), err
}

func (s *SQLStore) Get(id int) (Livre, error) {
	l := Livre{ID: id}
	err := s.db.QueryRow("SELECT titre, auteur FROM livres WHERE id = ?", id).
		Scan(&l.Titre, &l.Auteur)
	if errors.Is(err, sql.ErrNoRows) {
		return Livre{}, fmt.Errorf("get %d : %w", id, ErrIntrouvable)
	}
	return l, err
}

func (s *SQLStore) List() ([]Livre, error) {
	rows, err := s.db.Query("SELECT id, titre, auteur FROM livres ORDER BY id")
	if err != nil {
		return nil, err
	}
	defer rows.Close()
	var liste []Livre
	for rows.Next() {
		var l Livre
		if err := rows.Scan(&l.ID, &l.Titre, &l.Auteur); err != nil {
			return nil, err
		}
		liste = append(liste, l)
	}
	return liste, rows.Err()
}

func (s *SQLStore) Delete(id int) error {
	res, err := s.db.Exec("DELETE FROM livres WHERE id = ?", id)
	if err != nil {
		return err
	}
	if n, _ := res.RowsAffected(); n == 0 {
		return fmt.Errorf("delete %d : %w", id, ErrIntrouvable)
	}
	return nil
}
```

Comparez méthode par méthode :

| Méthode | MemoireStore | SQLStore |
|---|---|---|
| `Add` | `s.livres[id] = l`, `suivant++` | `INSERT`, puis `LastInsertId` |
| `Get` | `s.livres[id]` et `ok` | `SELECT … WHERE id = ?`, `ErrNoRows` |
| `List` | parcourir la map, puis trier | `SELECT … ORDER BY id` |
| `Delete` | `delete(s.livres, id)` | `DELETE`, puis `RowsAffected` |
| verrou | `sync.RWMutex` | inutile : `*sql.DB` est déjà sûr |

Dans `main`, une seule ligne change :

```go norun
var s Store = NewMemoireStore()        // pour essayer, pour les tests
var s Store = &SQLStore{db: db}        // pour de vrai, avec la base
```

Avec SQLite, le même `main` que plus haut affiche **la même chose** : c'est tout
l'intérêt de l'interface. Et les deux versions renvoient la même erreur
`ErrIntrouvable`, donc le code qui l'utilise ne voit aucune différence.

> **À retenir :** une interface `Store`, deux implémentations. La mémoire pour les
> [tests](tests.md), SQL pour la production. Le reste du programme ne change pas.

## Erreurs fréquentes

### Oublier d'importer le pilote

```go norun
import "database/sql"

db, err := sql.Open("sqlite", "livres.db")
if err != nil {
	fmt.Println(err)
	return
}
```

```text
sql: unknown driver "sqlite" (forgotten import?)
```

Ajoutez la ligne `_ "modernc.org/sqlite"` dans les imports, et `go get` le module.

### Un Scan avec le mauvais nombre de variables

```go norun
rows, _ := db.Query("SELECT id, titre, auteur FROM livres")
for rows.Next() {
	var id int
	var titre string
	err := rows.Scan(&id, &titre) // 3 colonnes, 2 variables
	fmt.Println(err)
}
```

```text
sql: expected 3 destination arguments in Scan, not 2
```

Il faut une variable par colonne du `SELECT`, dans le même ordre.

### Une map partagée sans verrou

Si on retire le `sync.RWMutex` de `MemoireStore` et que plusieurs goroutines appellent
`Add` en même temps, Go arrête le programme :

```text
fatal error: concurrent map writes

goroutine 7 [running]:
main.(*MemoireStore).Add(…)
…
exit status 2
```

Une map Go n'est pas faite pour être modifiée par plusieurs goroutines à la fois.
Gardez le verrou (`Lock` pour écrire, `RLock` pour lire).

## Testez-vous

**Question 1 :** à quoi sert le `_` dans `import _ "modernc.org/sqlite"` ?

> **Réponse**
>
> À charger le paquet sans utiliser ses noms. En se chargeant, le pilote s'inscrit
> auprès de `database/sql` sous le nom `"sqlite"`.

**Question 2 :** pourquoi écrire `WHERE id = ?` avec un paramètre plutôt que coller
l'id dans le texte de la requête ?

> **Réponse**
>
> Pour éviter l'injection SQL : la valeur est envoyée à part et reste une donnée, elle
> ne peut jamais devenir un ordre SQL.

**Question 3 :** dans l'exemple `MemoireStore`, pourquoi `List` trie-t-elle les livres ?

> **Réponse**
>
> Parce qu'une map Go rend ses éléments dans un ordre imprévisible. Le tri par id donne
> toujours la même liste, comme le ferait `ORDER BY id` en SQL.

## En résumé

- `database/sql` donne les mêmes fonctions pour toutes les bases ; un pilote, importé
  avec `_`, fait le lien avec une base précise.
- `Exec` pour écrire, `QueryRow(…).Scan(…)` pour une ligne, `Query` + `rows.Next()` +
  `defer rows.Close()` pour plusieurs.
- Toujours des paramètres `?` ou `$1`, jamais de texte collé.
- Une interface `Store` cache la base : une version en mémoire (map + `RWMutex`) pour
  essayer et tester, une version SQL pour la production.
- `var _ Store = (*SQLStore)(nil)` vérifie à la compilation qu'un type respecte
  l'interface.

Suite : [les expressions régulières](regex.md).
