# Dates et heures

Cette page montre comment créer, lire, comparer et afficher des dates en JavaScript, sans tomber dans les pièges des mois et des fuseaux horaires.

> **Objectifs :**
>
> - vous saurez créer une date précise avec `Date.UTC` et `new Date` ;
> - vous saurez éviter le piège des mois numérotés à partir de 0 ;
> - vous saurez lire une date avec les méthodes `getUTC…` et `toISOString` ;
> - vous saurez calculer un écart en jours et ajouter des jours ;
> - vous saurez afficher une date en français avec `Intl.DateTimeFormat`.

## Ce qu'est une date pour JavaScript

Pour JavaScript, une date est **un seul nombre** : le nombre de millisecondes écoulées
depuis le 1er janvier 1970 à minuit, en temps universel (UTC). C'est comme un
chronomètre lancé ce jour-là et jamais arrêté. Tout le reste (jour, mois, heure) est
calculé à partir de ce nombre.

```javascript
const debut = new Date(0);
console.log(debut.toISOString());

const date = new Date(Date.UTC(2024, 2, 5, 14, 30));
console.log(date.getTime());
```

```sortie
1970-01-01T00:00:00.000Z
1709649000000
```

- `getTime()` renvoie ce nombre de millisecondes ;
- `toISOString()` écrit la date au format **ISO 8601** : année-mois-jour, `T`, heure,
  et `Z` pour « UTC ».

### UTC et heure locale

**UTC** (temps universel coordonné) est l'heure de référence mondiale, sans heure d'été.
L'**heure locale** dépend du fuseau horaire de l'ordinateur : Paris a une ou deux
heures d'avance sur UTC selon la saison.

Le même instant s'affiche donc différemment selon la machine. Dans ce chapitre, tous
les exemples utilisent UTC : le résultat est le même partout.

> **Note :** `new Date()` sans argument donne l'instant présent. C'est utile dans un
> vrai programme, mais son résultat change à chaque exécution : les exemples de cette
> page utilisent des dates fixes.

```javascript norun
const maintenant = new Date();
console.log(maintenant.toISOString());
```

```text
2026-10-18T09:41:07.123Z
```

## Créer une date précise

### Avec Date.UTC

`Date.UTC(annee, mois, jour, heures, minutes, secondes)` calcule le nombre de
millisecondes d'une date en UTC. `new Date(…)` en fait un objet date. Les arguments
après le mois sont facultatifs.

```javascript
const noel = new Date(Date.UTC(2024, 11, 25));
console.log(noel.toISOString());

const rendezVous = new Date(Date.UTC(2024, 2, 5, 14, 30));
console.log(rendezVous.toISOString());
```

```sortie
2024-12-25T00:00:00.000Z
2024-03-05T14:30:00.000Z
```

### Le piège des mois

Avez-vous remarqué ? Pour Noël, on a écrit `11`, pas `12`. En JavaScript, **les mois
sont numérotés à partir de 0** : janvier vaut `0`, décembre vaut `11`. Les jours, eux,
commencent bien à 1.

| Mois | Janvier | Février | Mars | … | Novembre | Décembre |
|------|---------|---------|------|---|----------|----------|
| Numéro JavaScript | `0` | `1` | `2` | … | `10` | `11` |

Si vous donnez un mois ou un jour trop grand, JavaScript ne se plaint pas : il **déborde**
sur la suite du calendrier.

```javascript
console.log(new Date(Date.UTC(2024, 12, 25)).toISOString());
console.log(new Date(Date.UTC(2024, 1, 30)).toISOString());
```

```sortie
2025-01-25T00:00:00.000Z
2024-03-01T00:00:00.000Z
```

Le mois `12` est le treizième mois : janvier de l'année suivante. Le 30 février 2024
devient le 1er mars (2024 est bissextile, février a 29 jours).

> **Attention :** ce débordement ne lève aucune erreur. Une faute de mois passe
> inaperçue. Relisez toujours vos numéros de mois.

### À partir d'un texte ISO

`new Date(texte)` lit un texte au format ISO. Terminez par `Z` pour indiquer UTC.

```javascript
const d = new Date("2024-03-05T14:30:00Z");
console.log(d.toISOString());
console.log(new Date("2024-03-05").toISOString());
```

```sortie
2024-03-05T14:30:00.000Z
2024-03-05T00:00:00.000Z
```

Une date seule (`"2024-03-05"`) est lue en UTC. Mais une date **avec heure et sans
`Z`** (`"2024-03-05T14:30"`) est lue en heure **locale** : le résultat dépend de la
machine. Ajoutez toujours le `Z`.

## Lire les morceaux d'une date

Les méthodes `getUTC…` donnent chaque morceau en UTC.

```javascript
const d = new Date(Date.UTC(2024, 2, 5, 14, 30, 15));

console.log(d.getUTCFullYear());
console.log(d.getUTCMonth());
console.log(d.getUTCDate());
console.log(d.getUTCDay());
console.log(d.getUTCHours(), d.getUTCMinutes(), d.getUTCSeconds());
```

```sortie
2024
2
5
2
14 30 15
```

- `getUTCMonth()` renvoie `2` pour mars : toujours le décalage de 0 ;
- `getUTCDate()` est le jour du mois (5) ;
- `getUTCDay()` est le jour de la **semaine** : `0` pour dimanche, `1` pour lundi…
  Le 5 mars 2024 est un mardi, donc `2`.

> **Note :** il existe aussi `getFullYear`, `getMonth`, `getHours`… sans `UTC`. Elles
> répondent en heure locale : le résultat change selon l'ordinateur.

Pour afficher un jour de la semaine en toutes lettres, un tableau suffit :

```javascript
const jours = ["dimanche", "lundi", "mardi", "mercredi", "jeudi", "vendredi", "samedi"];
const d = new Date(Date.UTC(2024, 6, 14));
console.log(`Le 14 juillet 2024 est un ${jours[d.getUTCDay()]}.`);
```

```sortie
Le 14 juillet 2024 est un dimanche.
```

## Comparer et calculer

### Comparer deux dates

Les opérateurs `<` et `>` comparent directement deux dates. Pour l'égalité, comparez
les nombres avec `getTime()` : `===` compare deux **objets**, et deux objets différents
ne sont jamais égaux.

```javascript
const a = new Date(Date.UTC(2024, 0, 1));
const b = new Date(Date.UTC(2024, 0, 1));
const c = new Date(Date.UTC(2024, 5, 1));

console.log(a < c);
console.log(a === b);
console.log(a.getTime() === b.getTime());
```

```sortie
true
false
true
```

### L'écart en jours

Soustraire deux dates donne un écart en millisecondes. Divisez par le nombre de
millisecondes d'un jour : `1000 × 60 × 60 × 24`.

```javascript
const MS_PAR_JOUR = 1000 * 60 * 60 * 24;

function joursEntre(debut, fin) {
  return Math.round((fin - debut) / MS_PAR_JOUR);
}

const rentree = new Date(Date.UTC(2024, 8, 2));
const vacances = new Date(Date.UTC(2024, 9, 19));
console.log(`${joursEntre(rentree, vacances)} jours d'école avant les vacances`);

const an2024 = joursEntre(new Date(Date.UTC(2024, 0, 1)), new Date(Date.UTC(2025, 0, 1)));
console.log(`2024 compte ${an2024} jours`);
```

```sortie
47 jours d'école avant les vacances
2024 compte 366 jours
```

> **Astuce :** `Math.round` évite les surprises quand l'écart n'est pas un nombre exact
> de jours (heure d'été en heure locale, par exemple).

### Ajouter des jours

Un objet date est **modifiable**. Pour ne pas changer l'original, copiez-le d'abord avec
`new Date(date)`, puis changez le jour avec `setUTCDate`. Le débordement, si piégeux
plus haut, devient ici utile : le 31 janvier + 1 jour donne bien le 1er février.

```javascript
function ajouterJours(date, n) {
  const copie = new Date(date);
  copie.setUTCDate(copie.getUTCDate() + n);
  return copie;
}

const depart = new Date(Date.UTC(2024, 0, 31));
console.log(ajouterJours(depart, 1).toISOString().slice(0, 10));
console.log(ajouterJours(depart, 30).toISOString().slice(0, 10));
console.log(ajouterJours(depart, -31).toISOString().slice(0, 10));
console.log(depart.toISOString().slice(0, 10));
```

```sortie
2024-02-01
2024-03-01
2023-12-31
2024-01-31
```

`toISOString().slice(0, 10)` garde seulement les 10 premiers caractères : la date sans
l'heure.

## Afficher une date en français

`Intl.DateTimeFormat` met en forme une date selon une langue. Donnez `"fr-FR"` pour le
français de France, et `timeZone: "UTC"` pour un résultat identique sur toutes les
machines.

```javascript
const d = new Date(Date.UTC(2024, 2, 5, 14, 30));

const court = new Intl.DateTimeFormat("fr-FR", { timeZone: "UTC" });
console.log(court.format(d));

const long = new Intl.DateTimeFormat("fr-FR", { timeZone: "UTC", dateStyle: "long" });
console.log(long.format(d));

const complet = new Intl.DateTimeFormat("fr-FR", {
  timeZone: "UTC",
  dateStyle: "full",
  timeStyle: "short",
});
console.log(complet.format(d));
```

```sortie
05/03/2024
5 mars 2024
mardi 5 mars 2024 à 14:30
```

Vous pouvez aussi choisir chaque morceau :

```javascript
const d = new Date(Date.UTC(2024, 2, 5, 14, 30));
const format = new Intl.DateTimeFormat("fr-FR", {
  timeZone: "UTC",
  weekday: "long",
  day: "numeric",
  month: "long",
});
console.log(format.format(d));
```

```sortie
mardi 5 mars
```

### Changer de fuseau horaire

`timeZone` accepte le nom d'un fuseau : `"Europe/Paris"`, `"America/New_York"`,
`"Asia/Tokyo"`… Le même instant s'affiche alors à l'heure de cette ville.

```javascript
const d = new Date(Date.UTC(2024, 2, 5, 14, 30));
for (const zone of ["UTC", "Europe/Paris", "America/New_York", "Asia/Tokyo"]) {
  const f = new Intl.DateTimeFormat("fr-FR", { timeZone: zone, timeStyle: "short" });
  console.log(`${zone.padEnd(17)} ${f.format(d)}`);
}
```

```sortie
UTC               14:30
Europe/Paris      15:30
America/New_York  09:30
Asia/Tokyo        23:30
```

> **À retenir :** stockez et calculez en UTC, et ne convertissez dans un fuseau qu'au
> moment d'afficher.

## Demain : Temporal

L'objet `Date` a des défauts connus : mois à partir de 0, objets modifiables, fuseaux
difficiles. Une nouvelle API, **Temporal**, corrige tout cela. Elle commence à arriver
dans les navigateurs, mais Node 22 ne l'a pas encore.

```javascript norun
const date = Temporal.PlainDate.from("2024-03-05");
const plus = date.add({ days: 30 });
console.log(plus.toString());
console.log(plus.month);
```

```text
2024-04-04
4
```

Ici, avril vaut bien `4`. En attendant Temporal partout, des
[bibliothèques](../debuter/glossaire.md#bibliotheque) comme **date-fns** ou **Luxon**
simplifient les calculs de dates (voir [les modules](modules.md) pour les installer).

## Erreurs fréquentes

### Compter les mois à partir de 1

```javascript norun
const noel = new Date(Date.UTC(2024, 12, 25));
console.log(noel.toISOString());
```

```text
2025-01-25T00:00:00.000Z
```

Pas d'erreur, mais une mauvaise date : le mois `12` déborde en janvier 2025. Décembre
s'écrit `11`. Écrivez `Date.UTC(2024, 11, 25)`.

### Une date invalide

```javascript norun
const d = new Date("05/03/2024 à 14h");
console.log(d);
console.log(d.toISOString());
```

```text
Invalid Date
file:///…/main.mjs:3
console.log(d.toISOString());
              ^

RangeError: Invalid time value
```

JavaScript n'a pas su lire ce texte : la date est **invalide**. `toISOString` lève alors
une `RangeError`. Donnez un texte ISO (`"2024-03-05T14:00:00Z"`), et testez une date
lue avec `Number.isNaN(d.getTime())`.

### Utiliser l'heure locale sans le savoir

Sur un ordinateur réglé à l'heure de Paris :

```javascript norun
const jourDeLAn = new Date(2024, 0, 1);
console.log(jourDeLAn.toISOString());
```

```text
2023-12-31T23:00:00.000Z
```

`new Date(2024, 0, 1)` crée minuit **à Paris**, soit 23 h la veille en UTC. Le même code
affiche autre chose à New York. Pour un résultat stable, utilisez
`new Date(Date.UTC(2024, 0, 1))` et les méthodes `getUTC…`.

## Testez-vous

**Question 1 :** quel nombre représente le mois de mai dans `Date.UTC` ?

> **Réponse**
>
> `4`. Les mois commencent à 0 : janvier `0`, février `1`, mars `2`, avril `3`, mai `4`.

**Question 2 :** pourquoi `a === b` vaut-il `false` pour deux dates identiques ?

> **Réponse**
>
> `===` compare deux objets : ce sont deux objets différents en mémoire. Comparez
> `a.getTime() === b.getTime()`.

**Question 3 :** combien de jours séparent le 1er mars 2024 et le 1er avril 2024 ?
Calculez-le.

> **Réponse**
>
> ```javascript
> const debut = new Date(Date.UTC(2024, 2, 1));
> const fin = new Date(Date.UTC(2024, 3, 1));
> console.log((fin - debut) / (1000 * 60 * 60 * 24));
> ```
>
> ```sortie
> 31
> ```

## En résumé

- Une date est un nombre de millisecondes depuis le 1er janvier 1970 en UTC.
- Les mois vont de `0` (janvier) à `11` (décembre) ; un débordement ne lève pas d'erreur.
- Créez des dates fixes avec `Date.UTC`, lisez-les avec `getUTC…` et `toISOString`.
- La différence de deux dates donne des millisecondes ; `setUTCDate` ajoute des jours.
- `Intl.DateTimeFormat("fr-FR", { timeZone: "UTC" })` affiche une date en français.
- Temporal remplacera `Date` ; en attendant, restez en UTC.

Suite : [écrire un outil en ligne de commande](ligne-de-commande.md)
