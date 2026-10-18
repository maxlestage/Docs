# Dates et heures

Cette page montre comment représenter des dates et des heures, calculer des durées, et passer d'une date à du texte (et inversement) avec le module `datetime`.

> **Objectifs :**
>
> - vous saurez créer une date, une date avec heure et une durée ;
> - vous saurez calculer l'écart entre deux dates et ajouter des jours ;
> - vous saurez afficher une date avec `strftime` et lire un texte avec `strptime` ;
> - vous saurez utiliser le format ISO pour enregistrer des dates ;
> - vous saurez trouver le jour de la semaine et afficher un calendrier.

## Trois objets pour le temps

Le module `datetime` fournit trois objets principaux :

| Objet | Représente | Exemple |
|-------|------------|---------|
| `date` | un jour du calendrier | le 14 juillet 2024 |
| `datetime` | un jour **et** une heure | le 14 juillet 2024 à 21 h 30 |
| `timedelta` | une durée | 3 jours et 2 heures |

Une `date` est un point sur une frise chronologique ; un `timedelta` est la distance
entre deux points.

### Créer une date

On donne l'année, le mois, puis le jour, dans cet ordre.

```python
from datetime import date

fete = date(2024, 7, 14)
print(fete)
print(fete.year, fete.month, fete.day)
```

```sortie
2024-07-14
2024 7 14
```

Python affiche la date au format international : année, mois, jour.

### Créer une date avec une heure

```python
from datetime import datetime

feu = datetime(2024, 7, 14, 23, 0)
print(feu)
print(feu.hour, "h", feu.minute, "min")
print(feu.date())
```

```sortie
2024-07-14 23:00:00
23 h 0 min
2024-07-14
```

`.date()` garde seulement la partie « jour ».

### La date d'aujourd'hui

`date.today()` et `datetime.now()` donnent le moment présent.

```python norun
from datetime import date, datetime

print(date.today())
print(datetime.now())
```

```text
2024-03-15
2024-03-15 10:42:07.183920
```

Le résultat change à chaque exécution : c'est pourquoi les exemples de cette page
utilisent des dates fixes.

## Calculer avec les dates

### L'écart entre deux dates

Soustraire deux dates donne un `timedelta`. Son attribut `.days` donne le nombre de
jours.

```python
from datetime import date

rentree = date(2024, 9, 2)
vacances = date(2024, 12, 21)
ecart = vacances - rentree

print(ecart)
print(ecart.days, "jours")
print(ecart.days // 7, "semaines complètes")
```

```sortie
110 days, 0:00:00
110 jours
15 semaines complètes
```

### Ajouter une durée

`timedelta` accepte `days`, `weeks`, `hours`, `minutes` et `seconds`. On l'ajoute ou on
le retire à une date.

```python
from datetime import date, datetime, timedelta

commande = date(2024, 2, 26)
print("Livraison :", commande + timedelta(days=5))
print("Il y a 2 semaines :", commande - timedelta(weeks=2))

depart = datetime(2024, 3, 31, 22, 45)
print("Arrivée :", depart + timedelta(hours=3, minutes=20))
```

```sortie
Livraison : 2024-03-02
Il y a 2 semaines : 2024-02-12
Arrivée : 2024-04-01 02:05:00
```

Python gère tout seul les fins de mois et les années bissextiles : 2024 a un
29 février, donc 26 février + 5 jours tombe le 2 mars.

### Une durée en secondes

`total_seconds()` convertit une durée en secondes, pratique pour des calculs.

```python
from datetime import datetime

debut = datetime(2024, 5, 10, 8, 30)
fin = datetime(2024, 5, 10, 17, 15)
duree = fin - debut

print(duree)
print(duree.total_seconds() / 3600, "heures")
```

```sortie
8:45:00
8.75 heures
```

### Comparer des dates

Les dates se comparent avec `<`, `>`, `==`, comme des nombres. `sorted`, `min` et
`max` marchent donc aussi.

```python
from datetime import date

anniversaires = {"Léa": date(2015, 3, 7), "Hugo": date(2014, 11, 23), "Inès": date(2015, 1, 2)}
plus_age = min(anniversaires, key=anniversaires.get)
print("La plus âgée :", plus_age)
print(date(2024, 1, 1) < date(2024, 12, 31))
```

```sortie
La plus âgée : Hugo
True
```

## Le jour de la semaine

`weekday()` renvoie un numéro : 0 pour lundi, 6 pour dimanche. Une liste de noms
suffit pour l'afficher en français.

```python
from datetime import date

JOURS = ["lundi", "mardi", "mercredi", "jeudi", "vendredi", "samedi", "dimanche"]

for jour in [date(2024, 7, 14), date(2024, 12, 25), date(2025, 1, 1)]:
    print(jour, "est un", JOURS[jour.weekday()])
```

```sortie
2024-07-14 est un dimanche
2024-12-25 est un mercredi
2025-01-01 est un mercredi
```

> **Note :** `isoweekday()` compte de 1 (lundi) à 7 (dimanche).

## De la date au texte : strftime

`strftime` (*string format time*, « mettre le temps en forme de texte ») transforme une
date en texte selon un modèle. Chaque code commence par `%`.

```python
from datetime import datetime

moment = datetime(2024, 3, 5, 9, 7)
print(moment.strftime("%d/%m/%Y"))
print(moment.strftime("%d/%m/%y à %Hh%M"))
print(moment.strftime("Le %d, semaine %W"))
```

```sortie
05/03/2024
05/03/24 à 09h07
Le 05, semaine 10
```

### Les codes les plus utiles

| Code | Signification | Exemple |
|------|---------------|---------|
| `%d` | jour sur 2 chiffres | `05` |
| `%m` | mois sur 2 chiffres | `03` |
| `%Y` | année sur 4 chiffres | `2024` |
| `%y` | année sur 2 chiffres | `24` |
| `%H` | heure (00 à 23) | `09` |
| `%M` | minutes | `07` |
| `%S` | secondes | `00` |
| `%j` | numéro du jour dans l'année | `065` |
| `%W` | numéro de semaine (lundi en premier) | `10` |
| `%A` | nom du jour (dans la langue du système) | `Tuesday` |
| `%B` | nom du mois (dans la langue du système) | `March` |
| `%%` | un vrai `%` | `%` |

> **Attention :** `%A` et `%B` dépendent de la langue réglée sur l'ordinateur ; ils
> sortent souvent en anglais. Pour du français fiable, utilisez vos propres listes,
> comme `JOURS` plus haut.

Une f-string accepte aussi ces codes après `:` :

```python
from datetime import date

d = date(2024, 3, 5)
print(f"Rendez-vous le {d:%d/%m/%Y}")
```

```sortie
Rendez-vous le 05/03/2024
```

## Du texte à la date : strptime

`strptime` (*string parse time*, « analyser un texte de temps ») fait l'inverse : il lit
un texte avec un modèle et renvoie un `datetime`. Le modèle doit correspondre
**exactement** au texte.

```python
from datetime import datetime

saisie = "25/12/2024 18:30"
moment = datetime.strptime(saisie, "%d/%m/%Y %H:%M")
print(moment)
print(moment.date())
```

```sortie
2024-12-25 18:30:00
2024-12-25
```

`strptime` vérifie aussi que la date existe : c'est un bon moyen de contrôler une
saisie.

```python
from datetime import datetime

def date_valide(texte):
    try:
        datetime.strptime(texte, "%d/%m/%Y")
        return True
    except ValueError:
        return False

for texte in ["29/02/2024", "29/02/2023", "31/04/2024", "2024-01-01"]:
    print(texte, date_valide(texte))
```

```sortie
29/02/2024 True
29/02/2023 False
31/04/2024 False
2024-01-01 False
```

## Le format ISO

Le format ISO 8601 (`2024-03-05T09:07:00`) est la façon standard d'écrire une date
dans un fichier ou un échange [JSON](../debuter/glossaire.md#json). Il se trie bien et
tous les langages le comprennent. `isoformat` écrit, `fromisoformat` relit.

```python
from datetime import date, datetime

moment = datetime(2024, 3, 5, 9, 7)
texte = moment.isoformat()
print(texte)

relu = datetime.fromisoformat(texte)
print(relu == moment)
print(date.fromisoformat("2024-12-25").weekday())
```

```sortie
2024-03-05T09:07:00
True
2
```

> **À retenir :** pour **enregistrer** une date, utilisez l'ISO. Pour **l'afficher** à
> une personne, utilisez `strftime` avec le format de son pays.

## Le module calendar

Le module `calendar` répond aux questions sur le calendrier lui-même.

```python
import calendar

print(calendar.isleap(2024), calendar.isleap(2023))
print(calendar.monthrange(2024, 2)[1], "jours en février 2024")
print(calendar.leapdays(2000, 2025), "années bissextiles de 2000 à 2024")
print(calendar.month(2024, 2))
```

```sortie
True False
29 jours en février 2024
7 années bissextiles de 2000 à 2024
   February 2024
Mo Tu We Th Fr Sa Su
          1  2  3  4
 5  6  7  8  9 10 11
12 13 14 15 16 17 18
19 20 21 22 23 24 25
26 27 28 29
```

`monthrange` renvoie deux valeurs : le jour de la semaine du 1er du mois, et le nombre
de jours du mois. `[1]` garde la deuxième.

## Les fuseaux horaires

Les objets vus jusqu'ici sont **naïfs** : ils ne savent pas dans quel pays on est.
21 h 30 à Paris n'est pas 21 h 30 à Tokyo. Le module `zoneinfo` ajoute un fuseau
horaire ; la date devient **consciente** de son fuseau.

```python norun
from datetime import datetime
from zoneinfo import ZoneInfo

concert = datetime(2024, 7, 14, 21, 30, tzinfo=ZoneInfo("Europe/Paris"))
print(concert)
print(concert.astimezone(ZoneInfo("America/New_York")))
print(concert.astimezone(ZoneInfo("Asia/Tokyo")).strftime("%d/%m %H:%M"))
```

```text
2024-07-14 21:30:00+02:00
2024-07-14 15:30:00-04:00
15/07 04:30
```

`+02:00` indique l'écart avec l'heure universelle (UTC). `zoneinfo` gère aussi les
changements d'heure d'été et d'hiver. Sous Windows, il faut parfois installer le
paquet `tzdata` (`pip install tzdata`).

> **Astuce :** pour une application utilisée dans plusieurs pays, enregistrez les
> moments en UTC et convertissez seulement à l'affichage.

## Erreurs fréquentes

### Un modèle qui ne correspond pas au texte

```python norun
from datetime import datetime
datetime.strptime("2024-03-15", "%d/%m/%Y")
```

```text
Traceback (most recent call last):
  File "main.py", line 2, in <module>
    datetime.strptime("2024-03-15", "%d/%m/%Y")
    ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^
  ...
ValueError: time data '2024-03-15' does not match format '%d/%m/%Y'
```

Le texte utilise des tirets et commence par l'année. Le modèle doit être
`"%Y-%m-%d"` (ou utilisez `date.fromisoformat`).

### Une date qui n'existe pas

```python norun
from datetime import date
date(2024, 2, 30)
```

```text
Traceback (most recent call last):
  File "main.py", line 2, in <module>
    date(2024, 2, 30)
    ~~~~^^^^^^^^^^^^^
ValueError: day is out of range for month
```

Il n'y a pas de 30 février. Pour « le dernier jour du mois », utilisez
`calendar.monthrange`.

### Ajouter un nombre à une date

```python norun
from datetime import date
print(date(2024, 3, 15) + 7)
```

```text
Traceback (most recent call last):
  File "main.py", line 2, in <module>
    print(date(2024, 3, 15) + 7)
          ~~~~~~~~~~~~~~~~~~^~~
TypeError: unsupported operand type(s) for +: 'datetime.date' and 'int'
```

Python ne sait pas si `7` veut dire 7 jours, 7 heures ou 7 mois. Soyez précis :
`date(2024, 3, 15) + timedelta(days=7)`.

### Mélanger dates naïves et conscientes

```python norun
from datetime import datetime
a = datetime(2024, 3, 15, 10, 0)
b = datetime.fromisoformat("2024-03-15T10:00:00+01:00")
print(a < b)
```

```text
Traceback (most recent call last):
  File "main.py", line 4, in <module>
    print(a < b)
          ^^^^^
TypeError: can't compare offset-naive and offset-aware datetimes
```

`b` a un fuseau, pas `a`. Donnez un fuseau aux deux, ou à aucun.

## Testez-vous

**Question 1 :** que vaut `(date(2024, 3, 1) - date(2024, 2, 1)).days` ?

> **Réponse**
>
> `29` : février 2024 compte 29 jours (année bissextile).

**Question 2 :** quel modèle `strftime` donne `14/07/2024 à 21:30` ?

> **Réponse**
>
> `"%d/%m/%Y à %H:%M"`.

**Question 3 :** une personne est née le 07/03/2015. Écrivez le code qui lit cette
date et affiche son âge en jours au 1er janvier 2025.

> **Réponse**
>
> ```python
> from datetime import date, datetime
>
> naissance = datetime.strptime("07/03/2015", "%d/%m/%Y").date()
> print((date(2025, 1, 1) - naissance).days, "jours")
> ```
>
> ```sortie
> 3588 jours
> ```

## En résumé

- `date` pour un jour, `datetime` pour un jour et une heure, `timedelta` pour une durée.
- Date moins date donne un `timedelta` ; `.days` donne le nombre de jours.
- `strftime` met une date en texte, `strptime` lit un texte ; les codes `%d %m %Y %H %M`
  sont les plus courants.
- L'ISO (`isoformat`, `fromisoformat`) sert à enregistrer et échanger des dates.
- `weekday()` donne le jour de la semaine (0 = lundi), `calendar` répond aux questions
  de calendrier.
- `zoneinfo` ajoute un fuseau horaire ; ne mélangez pas dates naïves et conscientes.

Suite : [écrire un outil en ligne de commande](ligne-de-commande.md). Comparez avec
[les dates en Ruby](../ruby/dates.md).
