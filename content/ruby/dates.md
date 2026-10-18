# Dates et heures

Cette page montre comment créer des dates et des heures, les afficher dans le format voulu, et calculer avec elles : ajouter des jours, compter un écart, parcourir un calendrier.

> **Objectifs :**
>
> - vous saurez créer un moment précis avec `Time.new` et lire ses parties ;
> - vous saurez afficher une date à votre façon avec `strftime` ;
> - vous saurez calculer avec des secondes (`Time`) et des jours (`Date`) ;
> - vous saurez lire une date écrite en texte avec `Date.parse` ;
> - vous saurez parcourir un intervalle de dates.

## Deux classes : Time et Date

Ruby propose deux outils principaux :

- `Time` représente un **instant** précis : une date **et** une heure, à la seconde
  près. Il est toujours disponible.
- `Date` représente un **jour** du calendrier, sans heure. Il faut d'abord écrire
  `require "date"`.

Pensez à une montre (`Time`) et à un calendrier mural (`Date`). Pour un rendez-vous à
14 h 30, prenez la montre ; pour un anniversaire, le calendrier.

## Créer un instant avec Time.new

`Time.new(annee, mois, jour, heure, minute, seconde)` crée un instant fixe. Les
arguments de la fin sont facultatifs : sans heure, il est minuit.

```ruby
rdv = Time.new(2024, 3, 1, 14, 30)

puts rdv.year
puts rdv.month
puts rdv.day
puts rdv.hour
puts rdv.min
puts rdv.wday
puts rdv.yday
```

```sortie
2024
3
1
14
30
5
61
```

`wday` donne le jour de la semaine : `0` pour dimanche, `1` pour lundi… jusqu'à `6`
pour samedi. Le 1er mars 2024 était un vendredi (`5`). `yday` donne le numéro du jour
dans l'année.

> **Note :** `Time.now` donne l'instant présent. Son résultat change à chaque
> exécution : les exemples de cette page utilisent donc des dates fixes.

```ruby norun
maintenant = Time.now
puts maintenant.year
```

## Afficher avec strftime

`strftime` (« *string format time* ») transforme un instant en texte selon un modèle.
Chaque code qui commence par `%` est remplacé par une partie de la date.

```ruby
rdv = Time.new(2024, 3, 1, 9, 5, 7)

puts rdv.strftime("%d/%m/%Y")
puts rdv.strftime("%Hh%M")
puts rdv.strftime("%Y-%m-%d %H:%M:%S")
puts rdv.strftime("%-d/%-m/%y")
puts rdv.strftime("%A %d %B")
```

```sortie
01/03/2024
09h05
2024-03-01 09:05:07
1/3/24
Friday 01 March
```

| Code | Signification | Exemple |
|------|---------------|---------|
| `%Y` | année sur 4 chiffres | `2024` |
| `%y` | année sur 2 chiffres | `24` |
| `%m` | mois sur 2 chiffres | `03` |
| `%d` | jour sur 2 chiffres | `01` |
| `%H` | heure (00 à 23) | `09` |
| `%M` | minutes | `05` |
| `%S` | secondes | `07` |
| `%A` / `%a` | jour de la semaine (anglais), long / court | `Friday` / `Fri` |
| `%B` / `%b` | mois (anglais), long / court | `March` / `Mar` |
| `%j` | numéro du jour dans l'année | `061` |
| `%-d`, `%-m` | sans le zéro devant | `1`, `3` |
| `%%` | un vrai signe `%` | `%` |

### Les noms en français

`%A` et `%B` écrivent en anglais. Pour le français, gardez vos propres listes de noms
et choisissez avec `wday` et `month`.

```ruby
JOURS = %w[dimanche lundi mardi mercredi jeudi vendredi samedi]
MOIS = %w[janvier février mars avril mai juin juillet août septembre octobre novembre décembre]

def en_francais(t)
  "#{JOURS[t.wday]} #{t.day} #{MOIS[t.month - 1]} #{t.year}"
end

puts en_francais(Time.new(2024, 3, 1))
puts en_francais(Time.new(2024, 7, 14))
```

```sortie
vendredi 1 mars 2024
dimanche 14 juillet 2024
```

`MOIS[t.month - 1]` : les mois vont de 1 à 12, mais les cases du tableau commencent à 0.

## Calculer avec Time : des secondes

Avec `Time`, on additionne et on soustrait des **secondes**. Une heure fait
`60 * 60` secondes, un jour `24 * 60 * 60`.

```ruby
HEURE = 60 * 60
debut = Time.new(2024, 3, 1, 14, 30)

fin = debut + 90 * 60
puts fin.strftime("%H:%M")
puts (debut + 2 * HEURE).strftime("%H:%M")
puts (debut - 24 * HEURE).strftime("%d/%m")

ecart = fin - debut
p ecart
puts "#{(ecart / 60).to_i} minutes"
```

```sortie
16:00
16:30
29/02
5400.0
90 minutes
```

La différence entre deux `Time` est un nombre de secondes, à
[virgule](../debuter/glossaire.md#flottant). Divisez par 60 pour des minutes, par 3600
pour des heures. Les instants se comparent aussi avec `<`, `>` et `==`.

## Date : les jours du calendrier

Après `require "date"`, `Date.new(annee, mois, jour)` crée un jour. Avec `Date`, on
compte en **jours** : `+ 1` passe au lendemain.

```ruby
require "date"

noel = Date.new(2024, 12, 25)

puts noel
puts noel + 7
puts noel - 30
puts noel >> 1
puts noel.strftime("%d/%m/%Y")
```

```sortie
2024-12-25
2025-01-01
2024-11-25
2025-01-25
25/12/2024
```

`>> 1` avance d'un mois (et `<< 1` recule d'un mois). `puts` affiche une date au format
international `année-mois-jour`.

### L'écart en jours

La soustraction de deux dates donne un nombre de jours sous forme de fraction
(`Rational`). `to_i` le transforme en entier.

```ruby
require "date"

rentree = Date.new(2024, 9, 2)
vacances = Date.new(2024, 10, 19)

ecart = vacances - rentree
p ecart
puts "Encore #{ecart.to_i} jours avant les vacances."
puts vacances > rentree
```

```sortie
(47/1)
Encore 47 jours avant les vacances.
true
```

### Jour de la semaine et année bissextile

`wday` fonctionne comme pour `Time`. `cwday` numérote à la française : lundi `1`,
dimanche `7`. `leap?` dit si l'année est bissextile (un 29 février).

```ruby
require "date"

d = Date.new(2024, 3, 3)
puts d.wday
puts d.cwday
puts d.sunday?
puts d.leap?
puts Date.new(2023, 1, 1).leap?
puts Date.valid_date?(2023, 2, 29)
```

```sortie
0
7
true
true
false
false
```

`Date.valid_date?` vérifie qu'une date existe **avant** de la créer.

## Lire une date écrite en texte

`Date.parse` devine le format d'un texte. `Date.strptime` est plus sûr : vous lui
donnez le modèle exact, avec les mêmes codes que `strftime`.

```ruby
require "date"

puts Date.parse("2024-03-01")
puts Date.parse("1 March 2024")
puts Date.strptime("14/07/2024", "%d/%m/%Y")
puts Date.iso8601("2024-12-31").yday
```

```sortie
2024-03-01
2024-03-01
2024-07-14
366
```

> **Astuce :** pour des données qui viennent de l'extérieur (un fichier, un
> formulaire), préférez `strptime` : `"03/04/2024"` peut être le 3 avril ou le 4 mars,
> et `Date.parse` ne peut pas le savoir.

## Parcourir des dates

Deux dates forment un [intervalle](../debuter/glossaire.md#intervalle), comme deux
nombres. Vous pouvez le parcourir jour par jour, le filtrer, le compter.

```ruby
require "date"

semaine = Date.new(2024, 3, 4)..Date.new(2024, 3, 10)
puts semaine.map(&:day).join(" ")

mars = Date.new(2024, 3, 1)..Date.new(2024, 3, 31)
week_ends = mars.select { |j| j.saturday? || j.sunday? }
puts "#{week_ends.size} jours de week-end en mars 2024"
puts mars.cover?(Date.new(2024, 3, 15))

Date.new(2024, 1, 31).step(Date.new(2024, 3, 31), 14) { |j| puts j }
```

```sortie
4 5 6 7 8 9 10
10 jours de week-end en mars 2024
true
2024-01-31
2024-02-14
2024-02-28
2024-03-13
2024-03-27
```

`step(fin, 14)` avance de 14 jours à chaque tour.

## Les fuseaux horaires

Un même instant n'a pas la même heure à Paris et à Montréal. Chaque `Time` connaît son
**décalage** par rapport au temps universel (UTC). Vous pouvez le donner en dernier
argument de `Time.new`, et `utc` convertit vers le temps universel.

```ruby
paris = Time.new(2024, 3, 1, 12, 0, 0, "+01:00")
montreal = Time.new(2024, 3, 1, 6, 0, 0, "-05:00")

puts paris.strftime("%H:%M %z")
puts paris.utc.strftime("%H:%M UTC")
puts paris == montreal
```

```sortie
12:00 +0100
11:00 UTC
true
```

Midi à Paris et 6 h à Montréal sont le **même instant** : `==` répond `true`.

> **Note :** sans décalage, `Time.new` utilise le fuseau de l'ordinateur. Pour gérer
> les fuseaux par leur nom (« Europe/Paris ») et les changements d'heure, on utilise
> souvent la gem `tzinfo`.

## Erreurs fréquentes

### Oublier require "date"

```ruby norun
p Date.today
```

```text
main.rb:1:in `<main>': uninitialized constant Date (NameError)

p Date.today
  ^^^^
Did you mean?  Data
```

`Time` est toujours chargé, mais pas `Date`. Ajoutez `require "date"` en haut du
fichier.

### Une date qui n'existe pas

```ruby norun
require "date"
Date.new(2023, 2, 30)
```

```text
main.rb:2:in `initialize': invalid date (Date::Error)

Date.new(2023, 2, 30)
         ^^^^^^^^^^^
	from main.rb:2:in `new'
	from main.rb:2:in `<main>'
```

Le 30 février n'existe pas. Vérifiez avec `Date.valid_date?(2023, 2, 30)`, ou attrapez
l'erreur avec `rescue Date::Error`. `Date.parse("bonjour")` lève la même erreur.

### Ajouter des jours à un Time

```ruby norun
rdv = Time.new(2024, 3, 1)
puts (rdv + 7).strftime("%d/%m/%Y %H:%M:%S")
```

```text
01/03/2024 00:00:07
```

Pas d'erreur, mais un résultat surprenant : avec `Time`, `+ 7` ajoute 7 **secondes**.
Écrivez `rdv + 7 * 24 * 60 * 60`, ou utilisez une `Date` : `Date.new(2024, 3, 1) + 7`.

## Testez-vous

**Question 1 :** que vaut `Time.new(2024, 3, 1, 10, 0) - Time.new(2024, 3, 1, 9, 0)` ?

> **Réponse**
>
> `3600.0` : la différence entre deux `Time` est un nombre de secondes.

**Question 2 :** quel modèle `strftime` affiche `01/03/2024` ?

> **Réponse**
>
> `"%d/%m/%Y"`.

**Question 3 :** combien de jours séparent le 1er janvier 2024 du 1er janvier 2025 ?
Écrivez le calcul.

> **Réponse**
>
> ```ruby
> require "date"
> puts (Date.new(2025, 1, 1) - Date.new(2024, 1, 1)).to_i
> ```
>
> ```sortie
> 366
> ```
>
> 2024 est bissextile : elle compte 366 jours.

## En résumé

- `Time.new(2024, 3, 1, 14, 30)` crée un instant ; `Time` calcule en secondes.
- `require "date"` puis `Date.new(2024, 3, 1)` crée un jour ; `Date` calcule en jours.
- `strftime("%d/%m/%Y")` affiche une date ; `strptime` relit un texte au format connu.
- `wday`, `cwday`, `leap?` et `valid_date?` répondent aux questions de calendrier.
- Deux dates forment un intervalle que l'on peut parcourir avec `each`, `select` ou `step`.
- Un décalage comme `"+01:00"` fixe le fuseau horaire ; `utc` convertit en temps universel.

Suite : [écrire un outil en ligne de commande](ligne-de-commande.md)
