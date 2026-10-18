# Dates et heures

Cette page montre comment créer, afficher, lire et calculer des dates avec le paquet `time`, sans oublier les durées et les fuseaux horaires.

> **Objectifs :**
>
> - vous saurez créer une date précise avec `time.Date` ;
> - vous saurez afficher et lire une date grâce à la disposition de référence `2006-01-02 15:04:05` ;
> - vous saurez ajouter du temps et calculer un écart avec `Add`, `AddDate` et `Sub` ;
> - vous saurez manipuler des durées (`time.Duration`) ;
> - vous saurez passer d'un fuseau horaire à un autre.

## Le type time.Time

En Go, un instant précis (une date **et** une heure) est une valeur de type
`time.Time`. `time.Now()` donne l'instant présent :

```go norun
maintenant := time.Now()
fmt.Println(maintenant)
```

```text
2026-10-18 14:07:52.318406 +0200 CEST m=+0.000041201
```

Ce résultat change à chaque exécution. Dans la suite de la page, nous utilisons donc
des dates **fixes**, pour que les exemples affichent toujours la même chose.

## Créer une date avec time.Date

`time.Date` prend, dans l'ordre : l'année, le mois, le jour, l'heure, les minutes, les
secondes, les nanosecondes et le **fuseau horaire**. `time.UTC` est le temps universel,
l'heure de référence mondiale.

```go
package main

import (
	"fmt"
	"time"
)

func main() {
	rdv := time.Date(2026, time.October, 18, 9, 30, 0, 0, time.UTC)

	fmt.Println(rdv)
	fmt.Println("Année :", rdv.Year())
	fmt.Println("Mois :", rdv.Month(), "=", int(rdv.Month()))
	fmt.Println("Jour :", rdv.Day())
	fmt.Println("Heure :", rdv.Hour(), "h", rdv.Minute())
	fmt.Println("Jour de la semaine :", rdv.Weekday())
}
```

```sortie
2026-10-18 09:30:00 +0000 UTC
Année : 2026
Mois : October = 10
Jour : 18
Heure : 9 h 30
Jour de la semaine : Sunday
```

Les mois sont des constantes : `time.January` à `time.December`. `Weekday()` renvoie
le jour de la semaine, de `time.Sunday` (0) à `time.Saturday` (6).

### Les jours en français

Go ne connaît que les noms anglais. Pour du français, utilisez le numéro du jour comme
indice dans une tranche :

```go
package main

import (
	"fmt"
	"time"
)

var joursFr = []string{"dimanche", "lundi", "mardi", "mercredi", "jeudi", "vendredi", "samedi"}

func main() {
	noel := time.Date(2026, time.December, 25, 0, 0, 0, 0, time.UTC)
	fmt.Println("Noël 2026 tombe un", joursFr[noel.Weekday()])
}
```

```sortie
Noël 2026 tombe un vendredi
```

## Afficher une date : la disposition de référence

La plupart des langages décrivent un format avec des codes comme `YYYY` ou `%Y`. Go a
choisi une idée originale : on écrit **une date d'exemple**, toujours la même, telle
qu'on veut la voir. C'est la **date de référence** :

```text
Mon Jan 2 15:04:05 MST 2006
```

Chaque morceau a un nombre différent, facile à retenir dans l'ordre américain :
mois 1, jour 2, heure 3 (15 h), minute 4, seconde 5, année 6 (2006), fuseau 7 (MST
vaut UTC−7).

| Morceau | Signifie | Exemple |
|---------|----------|---------|
| `2006` | l'année sur 4 chiffres | `2026` |
| `01` | le mois sur 2 chiffres | `10` |
| `January` / `Jan` | le nom du mois | `October` / `Oct` |
| `02` | le jour sur 2 chiffres | `18` |
| `Monday` / `Mon` | le jour de la semaine | `Sunday` / `Sun` |
| `15` | l'heure sur 24 h | `09` |
| `03` | l'heure sur 12 h | `09` |
| `04` | les minutes | `30` |
| `05` | les secondes | `00` |
| `PM` | AM ou PM | `AM` |
| `-07:00` | le décalage horaire | `+02:00` |

`Format` reçoit cette disposition et remplace chaque morceau par la vraie valeur.
Tout le reste (`/`, `à`, `h`) est recopié tel quel.

```go
package main

import (
	"fmt"
	"time"
)

func main() {
	t := time.Date(2026, time.October, 18, 9, 30, 0, 0, time.UTC)

	fmt.Println(t.Format("2006-01-02 15:04:05"))
	fmt.Println(t.Format("02/01/2006"))
	fmt.Println(t.Format("02/01/2006 à 15h04"))
	fmt.Println(t.Format("Monday 2 January 2006"))
	fmt.Println(t.Format("03:04 PM"))
	fmt.Println(t.Format(time.RFC3339))
	fmt.Println(t.Format(time.DateOnly))
}
```

```sortie
2026-10-18 09:30:00
18/10/2026
18/10/2026 à 09h30
Sunday 18 October 2026
09:30 AM
2026-10-18T09:30:00Z
2026-10-18
```

`time.RFC3339` et `time.DateOnly` sont des dispositions toutes prêtes. RFC 3339 est le
format standard pour échanger des dates, par exemple en JSON.

> **À retenir :** pour un format, demandez-vous « comment s'écrirait le 2 janvier 2006
> à 15 h 04 min 05 s ? ». Écrivez-le exactement ainsi.

## Lire une date : time.Parse

`time.Parse` fait l'inverse : il lit un texte selon une disposition. Il renvoie la
date et une erreur, à vérifier comme toujours.

```go
package main

import (
	"fmt"
	"time"
)

func main() {
	d, err := time.Parse("02/01/2006", "14/07/1789")
	if err != nil {
		fmt.Println("Date invalide :", err)
		return
	}
	fmt.Println(d.Format("2006-01-02"), d.Weekday())

	_, err = time.Parse("02/01/2006", "31/02/2026")
	fmt.Println("Erreur :", err)
}
```

```sortie
1789-07-14 Tuesday
Erreur : parsing time "31/02/2026": day out of range
```

Le 31 février n'existe pas : `time.Parse` le refuse. Sans indication de fuseau dans le
texte, la date lue est en UTC.

## Calculer avec les dates

### Ajouter des jours, des mois, des années

`AddDate(années, mois, jours)` avance (ou recule, avec des nombres négatifs) dans le
calendrier.

```go
package main

import (
	"fmt"
	"time"
)

func main() {
	debut := time.Date(2026, time.January, 31, 0, 0, 0, 0, time.UTC)

	fmt.Println(debut.AddDate(0, 0, 10).Format("02/01/2006"))
	fmt.Println(debut.AddDate(1, 0, 0).Format("02/01/2006"))
	fmt.Println(debut.AddDate(0, 0, -31).Format("02/01/2006"))
	fmt.Println(debut.AddDate(0, 1, 0).Format("02/01/2006"))
}
```

```sortie
10/02/2026
31/01/2027
31/12/2025
03/03/2026
```

> **Attention :** le 31 janvier plus un mois donne le « 31 février », que Go
> **normalise** en 3 mars. Pensez-y pour les échéances mensuelles.

### L'écart entre deux dates

`Sub` soustrait deux dates et renvoie une **durée**. `Before`, `After` et `Equal`
comparent deux dates.

```go
package main

import (
	"fmt"
	"time"
)

func main() {
	aujourdhui := time.Date(2026, time.October, 18, 0, 0, 0, 0, time.UTC)
	vacances := time.Date(2026, time.December, 19, 0, 0, 0, 0, time.UTC)

	ecart := vacances.Sub(aujourdhui)
	fmt.Println("Écart :", ecart)
	fmt.Println("Jours restants :", int(ecart.Hours()/24))
	fmt.Println("Vacances après aujourd'hui ?", vacances.After(aujourdhui))
}
```

```sortie
Écart : 1488h0m0s
Jours restants : 62
Vacances après aujourd'hui ? true
```

## Les durées

Une durée est une valeur de type `time.Duration`. Le paquet fournit des unités :
`time.Second`, `time.Minute`, `time.Hour`, `time.Millisecond`… On les multiplie par un
nombre. `Add` ajoute une durée à une date.

```go
package main

import (
	"fmt"
	"time"
)

func main() {
	film := 2*time.Hour + 15*time.Minute
	fmt.Println("Durée du film :", film)
	fmt.Println("En minutes :", film.Minutes())

	seance := time.Date(2026, time.October, 18, 20, 45, 0, 0, time.UTC)
	fin := seance.Add(film)
	fmt.Println("Fin de la séance :", fin.Format("15h04"))

	d, err := time.ParseDuration("1h30m")
	if err != nil {
		fmt.Println(err)
		return
	}
	fmt.Println("Pause :", d, "=", d.Seconds(), "secondes")
}
```

```sortie
Durée du film : 2h15m0s
En minutes : 135
Fin de la séance : 23h00
Pause : 1h30m0s = 5400 secondes
```

`time.ParseDuration` lit une durée écrite comme `"90s"`, `"1h30m"` ou `"250ms"`.

> **Note :** pour mesurer le temps d'exécution d'un programme, on écrit
> `debut := time.Now()` puis `time.Since(debut)`. Le résultat change à chaque fois :
> c'est pourquoi on ne l'affiche pas dans ces exemples.

## Les fuseaux horaires

Une même date s'affiche différemment selon le fuseau : midi à Paris, c'est 10 h à
Londres en été… `time.FixedZone(nom, secondes)` crée un fuseau avec un décalage fixe
par rapport à UTC. `In` affiche le même instant dans un autre fuseau.

```go
package main

import (
	"fmt"
	"time"
)

func main() {
	parisEte := time.FixedZone("CEST", 2*60*60)
	tokyo := time.FixedZone("JST", 9*60*60)

	reunion := time.Date(2026, time.July, 1, 14, 0, 0, 0, parisEte)

	fmt.Println("Paris :", reunion.Format("15:04 -07:00"))
	fmt.Println("UTC   :", reunion.In(time.UTC).Format("15:04 -07:00"))
	fmt.Println("Tokyo :", reunion.In(tokyo).Format("15:04 -07:00"))
	fmt.Println("Même instant ?", reunion.Equal(reunion.In(tokyo)))
}
```

```sortie
Paris : 14:00 +02:00
UTC   : 12:00 +00:00
Tokyo : 21:00 +09:00
Même instant ? true
```

C'est le même instant, vu depuis trois horloges différentes. Comparez les dates avec
`Equal`, pas avec `==` : `==` compare aussi le fuseau.

Pour les vrais fuseaux, avec les changements d'heure été/hiver, utilisez
`time.LoadLocation` :

```go norun
paris, err := time.LoadLocation("Europe/Paris")
if err != nil {
	fmt.Println(err)
	return
}
fmt.Println(time.Now().In(paris))
```

> **Astuce :** enregistrez les dates en UTC (dans un fichier, une base de données) et
> convertissez-les dans le fuseau de l'utilisateur seulement au moment de l'affichage.

## Erreurs fréquentes

### Écrire un format avec YYYY

```go norun
t := time.Date(2026, time.October, 18, 9, 30, 0, 0, time.UTC)
fmt.Println(t.Format("DD/MM/YYYY"))
```

```text
DD/MM/YYYY
```

Pas d'erreur, mais le texte est recopié tel quel : Go ne connaît pas `YYYY`. Utilisez
la date de référence : `t.Format("02/01/2006")`.

### Lire avec une disposition qui ne correspond pas

```go norun
_, err := time.Parse("2006-01-02", "18/10/2026")
fmt.Println("Erreur :", err)
```

```text
Erreur : parsing time "18/10/2026" as "2006-01-02": cannot parse "18/10/2026" as "2006"
```

La disposition attend `année-mois-jour`, le texte est `jour/mois/année`. La disposition
doit avoir **exactement** la forme du texte : `time.Parse("02/01/2006", "18/10/2026")`.

### Multiplier une durée par une variable int

```go norun
package main

import (
	"fmt"
	"time"
)

func main() {
	n := 3
	attente := time.Second * n
	fmt.Println(attente)
}
```

```text
# command-line-arguments
./main.go:10:13: invalid operation: time.Second * n (mismatched types time.Duration and int)
```

`3 * time.Second` fonctionne, car `3` est une constante. Mais `n` est un `int`, un type
différent de `time.Duration`. Convertissez : `time.Duration(n) * time.Second`.

## Testez-vous

**Question 1 :** quelle disposition donne `18/10/2026 09:30` ?

> **Réponse**
>
> `"02/01/2006 15:04"` : le jour `02`, le mois `01`, l'année `2006`, l'heure `15` et
> les minutes `04`.

**Question 2 :** quel est le type de `b.Sub(a)` quand `a` et `b` sont des `time.Time` ?

> **Réponse**
>
> `time.Duration`, une durée. On peut ensuite utiliser `.Hours()`, `.Minutes()` ou
> `.Seconds()`.

**Question 3 :** une abonnée commence le 15/03/2026 pour 90 jours. Affichez la date de
fin au format `02/01/2006` et son jour de la semaine en anglais.

> **Réponse**
>
> ```go
> package main
>
> import (
> 	"fmt"
> 	"time"
> )
>
> func main() {
> 	debut := time.Date(2026, time.March, 15, 0, 0, 0, 0, time.UTC)
> 	fin := debut.AddDate(0, 0, 90)
> 	fmt.Println(fin.Format("02/01/2006"), fin.Weekday())
> }
> ```
>
> ```sortie
> 13/06/2026 Saturday
> ```

## En résumé

- Une date est un `time.Time` ; `time.Date(…, time.UTC)` en crée une précise.
- `Format` et `Parse` utilisent la date de référence `2006-01-02 15:04:05`.
- `AddDate` avance dans le calendrier, `Add` ajoute une durée, `Sub` donne un écart.
- Une `time.Duration` s'écrit `2*time.Hour + 15*time.Minute`.
- `In` affiche un instant dans un autre fuseau ; comparez avec `Equal`.

Suite : [écrire un outil en ligne de commande](ligne-de-commande.md). Comparez avec
[les dates en Ruby](../ruby/dates.md).
