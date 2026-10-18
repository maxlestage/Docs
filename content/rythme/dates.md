# Les dates

En Rythme, une date est un simple enregistrement : cette page montre comment la lire dans un texte, compter les jours, avancer dans le calendrier, trouver le jour de la semaine et dessiner la page d'un mois.

> **Objectifs :**
>
> - vous saurez écrire une date sous forme d'enregistrement `{year, month, day}` ;
> - vous saurez lire et écrire une date au format ISO 8601 avec `parse_date` et `format_date` ;
> - vous saurez compter les jours avec `days_between` et avancer avec `add_days` ;
> - vous saurez trouver le jour de la semaine et le nombre de jours d'un mois ;
> - vous saurez construire le calendrier d'un mois.

## Une date est un enregistrement

Pas de type spécial à apprendre : une date est un
[enregistrement](../debuter/glossaire.md#enregistrement) avec trois champs,
`year` (l'année), `month` (le mois, de 1 à 12) et `day` (le jour).

```rythme
fete = {year: 2024, month: 7, day: 14}

print(fete)
print(fete.year, fete.month, fete.day)
```

```sortie
{year: 2024, month: 7, day: 14}
2024 7 14
```

Les noms des champs sont en anglais, comme ceux des fonctions intégrées.
Vos propres noms, eux, peuvent être en français : `fete`, `rentree`,
`anniversaire`.

Comme toutes les valeurs de Rythme, une date est
[immuable](../debuter/glossaire.md#immuable) : « avancer » une date, c'est
en construire une nouvelle.

`is_date` dit si un enregistrement est une date qui existe vraiment :
`is_date({year: 2024, month: 2, day: 29})` vaut `true` (2024 est
bissextile), mais `is_date({year: 2024, month: 4, day: 31})` vaut `false` :
avril n'a que 30 jours.

## Lire une date dans un texte

Les dates arrivent souvent sous forme de texte : dans un fichier, dans un
formulaire. Le format le plus sûr est **ISO 8601** : l'année, le mois, puis
le jour, séparés par des tirets, comme `"2024-07-14"`. Il se trie bien et
ne laisse aucun doute (le 07/04, est-ce le 7 avril ou le 4 juillet ?).

`parse_date` lit ce format. Le texte peut ne pas être une date : la
fonction renvoie donc un [Result](../debuter/glossaire.md#result).

```rythme
print(parse_date("2024-07-14"))
print(parse_date("2024-02-30"))
print(parse_date("14/07/2024"))
```

```sortie
Ok({year: 2024, month: 7, day: 14})
Err("\"2024-02-30\" n'est pas une date valide : le jour doit aller de 1 à 29 en février 2024, pas 30")
Err("\"14/07/2024\" n'est pas une date écrite en ISO 8601 : écrivez-la comme \"2024-02-29\" ou \"2024-02-29T13:05:00\"")
```

Le message d'erreur explique ce qui ne va pas. Pour ouvrir le `Result`,
utilisez `match`, comme pour toute valeur qui peut échouer (voir
[gérer les erreurs](erreurs.md)) :

```rythme
fn montrer(texte) = match parse_date(texte)
  Ok(d) -> "le {d.day} du mois {d.month} de {d.year}"
  Err(_) -> "pas une date : {texte}"
end

print(montrer("1969-07-21"))
print(montrer("demain"))
```

```sortie
le 21 du mois 7 de 1969
pas une date : demain
```

## Écrire une date

`format_date` fait le chemin inverse : il écrit une date en ISO 8601.

```rythme
print(format_date({year: 2024, month: 7, day: 4}))
```

```sortie
2024-07-04
```

Pour un affichage « à la française », construisez le texte vous-même.
`pad_left(x, 2, "0")` ajoute un zéro devant les nombres à un chiffre :

```rythme
fn date_francaise(d) = "{pad_left(d.day, 2, "0")}/{pad_left(d.month, 2, "0")}/{d.year}"

print(date_francaise({year: 2024, month: 7, day: 4}))
```

```sortie
04/07/2024
```

## Des heures et des fuseaux

Une date peut aussi avoir une heure : les champs `hour`, `minute` et
`second` (un champ absent vaut 0). En ISO 8601, l'heure vient après un `T` :

```rythme
rdv = unwrap(parse_date("2024-07-14T09:30"))

print(rdv)
print(format_date({year: 2024, month: 7, day: 14, hour: 18}))
```

```sortie
{year: 2024, month: 7, day: 14, hour: 9, minute: 30, second: 0}
2024-07-14T18:00:00
```

Le champ `offset` dit de combien de **minutes** l'heure est en avance sur
le temps universel (UTC). Paris en été a 120 minutes d'avance, Londres 60 :

```rythme
paris = unwrap(parse_date("2024-07-14T12:00:00+02:00"))
londres = unwrap(parse_date("2024-07-14T11:00:00+01:00"))

print(paris.offset, londres.offset)
print(seconds_between(paris, londres))
```

```sortie
120 60
0
```

`seconds_between` compte les secondes entre deux moments. Midi à Paris et
11 h à Londres, c'est le même instant : 0 seconde d'écart.

## Compter les jours

`days_between(a, b)` compte les jours entiers de la date `a` à la date `b`.
Le résultat est négatif quand `b` est avant `a`.

```rythme
fete = {year: 2024, month: 7, day: 14}
noel = {year: 2024, month: 12, day: 25}

print(days_between(fete, noel))
print(days_between(noel, fete))
print(days_between({year: 2024, month: 1, day: 1}, {year: 2025, month: 1, day: 1}))
```

```sortie
164
-164
366
```

Les années bissextiles et la longueur des mois sont comptées pour vous :
2024 a bien 366 jours.

## Avancer et reculer dans le calendrier

`add_days(date, n)` donne la date `n` jours plus tard, ou plus tôt quand
`n` est négatif. Le passage d'un mois ou d'une année à l'autre est fait
pour vous :

```rythme
fete = {year: 2024, month: 7, day: 14}

print(add_days(fete, 30) |> format_date)
print(add_days(fete, -14) |> format_date)
print(add_days({year: 2024, month: 12, day: 31}, 1) |> format_date)
```

```sortie
2024-08-13
2024-06-30
2025-01-01
```

`add_seconds` fait de même avec des secondes. Pour 90 minutes, écrivez
`90 * 60` :

```rythme
rdv = {year: 2024, month: 3, day: 31, hour: 23, minute: 15}

print(add_seconds(rdv, 90 * 60) |> format_date)
```

```sortie
2024-04-01T00:45:00
```

Il n'y a pas de boucle en Rythme : pour une suite de dates, comme un cours
chaque semaine, utilisez `map` sur un intervalle.

```rythme
debut = {year: 2024, month: 9, day: 2}
seances = map(0..3, fn(i) = add_days(debut, 7 * i) |> format_date)

print(seances)
```

```sortie
["2024-09-02", "2024-09-09", "2024-09-16", "2024-09-23"]
```

## Le jour de la semaine

`day_of_week` donne un nombre de 1 (lundi) à 7 (dimanche). Une liste de
noms le transforme en mot ; attention, les positions d'une liste commencent
à 0, d'où le `- 1` :

```rythme
jours = ["lundi", "mardi", "mercredi", "jeudi", "vendredi", "samedi", "dimanche"]

fn nom_du_jour(date) = jours[day_of_week(date) - 1]
fn est_weekend(date) = day_of_week(date) >= 6

print(nom_du_jour({year: 1969, month: 7, day: 21}))
print(est_weekend({year: 2024, month: 7, day: 13}))
```

```sortie
lundi
true
```

## Mois et années bissextiles

Deux fonctions répondent aux questions de calendrier :

- `days_in_month(annee, mois)` : le nombre de jours d'un mois, de 28 à 31 ;
- `is_leap_year(annee)` : l'année a-t-elle un 29 février ?

```rythme
print(days_in_month(2024, 2), days_in_month(2023, 2), days_in_month(2024, 4))
print(is_leap_year(2024), is_leap_year(1900), is_leap_year(2000))
```

```sortie
29 28 30
true false true
```

La règle : une année bissextile tous les 4 ans, sauf tous les 100 ans, sauf
tous les 400 ans. 1900 ne l'est pas, 2000 l'est.

## Aujourd'hui

`today()` donne la date du jour, et `now()` la date et l'heure, avec le
décalage de votre fuseau horaire. Leur valeur change à chaque lancement :

```rythme norun
print(today())
print(now())
```

```text
{year: 2026, month: 10, day: 18}
{year: 2026, month: 10, day: 18, hour: 14, minute: 5, second: 9, offset: 120}
```

> **Astuce :** une fonction qui reçoit « aujourd'hui » en paramètre, au lieu
> d'appeler `today()` elle-même, donne toujours le même résultat pour les
> mêmes arguments. Elle est bien plus facile à
> [tester](tests.md) : vous choisissez le jour.

```rythme
fn age(naissance, jour)
  pas_encore = [jour.month, jour.day] < [naissance.month, naissance.day]
  jour.year - naissance.year - (if pas_encore then 1 else 0)
end

ada = {year: 1815, month: 12, day: 10}
print(age(ada, {year: 1852, month: 11, day: 27}))
```

```sortie
36
```

Dans le vrai programme, vous écrirez `age(ada, today())`. Ici, les listes
`[mois, jour]` se comparent élément par élément : l'anniversaire de
l'année n'est pas encore passé, on enlève 1.

## Construire le calendrier d'un mois

Rassemblons tout. La page d'un mois commence par des cases vides jusqu'au
jour de la semaine du 1er, puis les jours, coupés en semaines de 7 :

```rythme
noms_mois = ["janvier", "février", "mars", "avril", "mai", "juin", "juillet", "août", "septembre", "octobre", "novembre", "décembre"]

fn page(annee, mois)
  premier = {year: annee, month: mois, day: 1}
  vides = repeat(["  "], day_of_week(premier) - 1)
  jours = map(1..days_in_month(annee, mois), fn(j) = pad_left(j, 2))
  semaines = chunks(vides + jours, 7) |> map(fn(semaine) = join(semaine, " "))
  ["{noms_mois[mois - 1]} {annee}", "lu ma me je ve sa di"] + semaines
end

page(2024, 2) |> each(print)
```

```sortie
février 2024
lu ma me je ve sa di
          1  2  3  4
 5  6  7  8  9 10 11
12 13 14 15 16 17 18
19 20 21 22 23 24 25
26 27 28 29
```

Pas à pas :

1. le 1er février 2024 est un jeudi (`day_of_week` vaut 4) : il faut 3 cases
   vides, de deux espaces chacune ;
2. `pad_left(j, 2)` écrit chaque jour sur deux caractères, pour aligner les
   colonnes ;
3. `chunks(..., 7)` coupe la liste en semaines, et `join` fait une ligne de
   chaque semaine ;
4. `each(print)` affiche les lignes une à une.

Le dossier `examples/` de Rythme contient la version anglaise de ce
programme, `calendar.ry`.

## Erreurs fréquentes

### Donner un texte au lieu d'une date

```rythme norun
print(add_days("2024-07-14", 1))
```

```text
erreur[E005]: `add_days` attend une date (un enregistrement {year, month, day}) comme argument 1, mais a reçu Text "2024-07-14"
 --> main.ry:1:16
  |
1 | print(add_days("2024-07-14", 1))
  |                ^^^^^^^^^^^^ mauvaise sorte de valeur
  |
  = aide: utilisation : `add_days(date, n)`
  = aide: pour lire une date dans un texte comme "2024-02-29", utilisez `parse_date(texte)`
  = en savoir plus : `rythme explain E005`
```

Un texte qui ressemble à une date n'est pas une date. Lisez-le d'abord avec
`parse_date`, puis ouvrez le `Result`.

### Oublier d'ouvrir le Result de parse_date

```rythme norun
depart = parse_date("2024-07-14")
print(add_days(depart, 7))
```

```text
erreur[E005]: `add_days` attend une date (un enregistrement {year, month, day}) comme argument 1, mais a reçu une valeur de type Result
 --> main.ry:2:16
  |
2 | print(add_days(depart, 7))
  |                ^^^^^^ mauvaise sorte de valeur
  |
  = aide: utilisation : `add_days(date, n)`
  = en savoir plus : `rythme explain E005`
```

`depart` vaut `Ok({...})`, une boîte qui contient la date. Ouvrez-la avec
`match`, avec `?` dans une fonction, ou avec `unwrap` quand le texte est
écrit dans votre code et que vous savez qu'il est juste.

### Écrire une date qui n'existe pas

```rythme norun
rentree = {year: 2024, month: 9, day: 31}
print(format_date(rentree))
```

```text
erreur[E023]: `format_date` a besoin d'une date valide : le jour doit aller de 1 à 30 en septembre 2024, pas 31
 --> main.ry:2:7
  |
2 | print(format_date(rentree))
  |       ^^^^^^^^^^^^^^^^^^^^ pas une date valide
  |
  = aide: une date est un enregistrement comme `{year: 2024, month: 2, day: 29}`, avec `hour`, `minute`, `second` et `offset` (minutes d'avance sur UTC) si besoin
  = en savoir plus : `rythme explain E023`
```

Septembre a 30 jours. Pour une date qui vient de l'utilisateur, vérifiez-la
avec `is_date` ou lisez-la avec `parse_date`, qui renvoie un `Err` au lieu
d'arrêter le programme.

### Comparer deux dates avec <

```rythme norun
a = {year: 2024, month: 7, day: 14}
b = {year: 2024, month: 12, day: 25}
print(a < b)
```

```text
erreur[E005]: impossible d'utiliser `<` avec Record et Record
 --> main.ry:3:9
  |
3 | print(a < b)
  |         ^ Record < Record
  |
  = en savoir plus : `rythme explain E005`
```

Les enregistrements ne se rangent pas avec `<`. Demandez plutôt
`days_between(a, b) > 0` : « b est-il après a ? ».

## Testez-vous

**Question 1 :** que donne `add_days({year: 2023, month: 2, day: 28}, 1) |> format_date` ?

> **Réponse**
>
> `"2023-03-01"`. 2023 n'est pas bissextile : après le 28 février vient le
> 1er mars.

**Question 2 :** pourquoi `parse_date` renvoie-t-il un `Result`, alors que
`format_date` renvoie directement un texte ?

> **Réponse**
>
> Un texte peut ne pas être une date (`"demain"`, `"2024-02-30"`) : la
> lecture peut échouer, et le `Err` dit pourquoi. Écrire une date valide,
> elle, réussit toujours.

**Question 3 :** comment savoir si le 1er janvier 2030 est un dimanche ?

> **Réponse**
>
> `day_of_week({year: 2030, month: 1, day: 1}) == 7`. Le dimanche est le
> jour 7, le lundi le jour 1. Ici, la réponse est `false` : `day_of_week`
> donne 2, c'est un mardi.

## En résumé

- Une date est un enregistrement `{year, month, day}`, avec `hour`, `minute`,
  `second` et `offset` si besoin.
- `parse_date` lit une date ISO 8601 (`"2024-07-14"`) et renvoie un
  `Result` ; `format_date` l'écrit.
- `days_between` compte les jours, `add_days` et `add_seconds` avancent ou
  reculent.
- `day_of_week` va de 1 (lundi) à 7 (dimanche) ; `days_in_month` et
  `is_leap_year` répondent aux questions de calendrier.
- `today()` et `now()` changent à chaque lancement : passez la date du jour
  en paramètre pour garder des fonctions faciles à tester.

Suite : [écrire un outil en ligne de commande](ligne-de-commande.md).
Comparez avec les dates en [Ruby](../ruby/dates.md).
