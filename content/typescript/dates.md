# Dates et heures

Cette page montre comment créer, lire, calculer et afficher des dates en TypeScript, en évitant le piège des mois et celui des fuseaux horaires.

> **Objectifs :**
>
> - vous saurez ce qu'est un objet `Date` et créer une date précise ;
> - vous saurez éviter le piège des mois numérotés à partir de 0 ;
> - vous saurez travailler en UTC pour obtenir le même résultat partout ;
> - vous saurez afficher une date en français avec `Intl.DateTimeFormat` ;
> - vous saurez calculer un écart en jours et ajouter des jours ;
> - vous saurez ranger ces calculs dans des fonctions bien typées.

TypeScript utilise l'objet `Date` de JavaScript : tout ce que dit
[le chapitre JavaScript sur les dates](../javascript/dates.md) reste vrai ici. Cette
page reprend l'essentiel, pas à pas, et montre ce que les **types** ajoutent.

## Une date, c'est un nombre

Pour l'ordinateur, une date est **un seul nombre** : les millisecondes écoulées depuis
le 1er janvier 1970 à minuit (temps universel). Imaginez un chronomètre lancé ce
jour-là et jamais arrêté. Une date, c'est ce que le chronomètre affichait à un moment
donné.

```text
   1er janv. 1970                                  5 mars 2026, 14:30
   00:00 UTC
       │─────────────── 1 772 721 000 000 ms ─────────────────│
       0                                                  getTime()
```

```typescript
const zero: Date = new Date(0);
console.log(zero.toISOString());

const rendezVous: Date = new Date(Date.UTC(2026, 2, 5, 14, 30));
const millisecondes: number = rendezVous.getTime();
console.log(millisecondes);
console.log(rendezVous.toISOString());
```

```sortie
1970-01-01T00:00:00.000Z
1772721000000
2026-03-05T14:30:00.000Z
```

- `Date` est un **type** : une variable de type `Date` contient un objet date ;
- `getTime()` renvoie le nombre du chronomètre, de type `number` ;
- `toISOString()` écrit la date au format **ISO 8601** : année-mois-jour, un `T`,
  l'heure, et `Z` qui veut dire « temps universel ».

> **Note :** `new Date()` **sans argument** donne l'instant présent. C'est utile dans un
> vrai programme, mais le résultat change à chaque exécution. Tous les exemples de cette
> page utilisent donc des dates fixes.

> **À retenir :** un objet `Date` enveloppe un nombre de millisecondes. Tout le reste
> (jour, mois, heure) est calculé à partir de ce nombre.

## Créer une date précise

### Avec Date.UTC

`Date.UTC(année, mois, jour, heure, minute)` calcule le nombre du chronomètre pour une
date donnée en temps universel. On le donne ensuite à `new Date`.

```typescript
const noel = new Date(Date.UTC(2026, 11, 25));
const midi = new Date(Date.UTC(2026, 6, 14, 12, 0));
console.log(noel.toISOString());
console.log(midi.toISOString());
```

```sortie
2026-12-25T00:00:00.000Z
2026-07-14T12:00:00.000Z
```

Regardez bien : Noël s'écrit avec `11`, et le 14 juillet avec `6`. Ce n'est pas une
faute de frappe.

### Le piège des mois

Dans `Date.UTC` et `new Date(année, mois, …)`, **les mois commencent à 0**. Janvier
vaut 0, décembre vaut 11. Les jours, eux, commencent bien à 1.

C'est comme les cases d'une boîte à œufs numérotées depuis 0 : la première case est la
case 0.

```text
   mois réel :  janv  févr  mars  avr  mai  juin  juil  août  sept  oct  nov  déc
   numéro JS :    0     1     2    3    4     5     6     7     8    9   10   11
```

```typescript
const mois = [0, 1, 11, 12];
for (const m of mois) {
  const d = new Date(Date.UTC(2026, m, 1));
  console.log(`numéro ${m} -> ${d.toISOString().slice(0, 10)}`);
}
```

```sortie
numéro 0 -> 2026-01-01
numéro 1 -> 2026-02-01
numéro 11 -> 2026-12-01
numéro 12 -> 2027-01-01
```

| Étape | `m` | Mois réel | Date obtenue |
|-------|-----|-----------|--------------|
| 1 | 0 | janvier | 2026-01-01 |
| 2 | 1 | février | 2026-02-01 |
| 3 | 11 | décembre | 2026-12-01 |
| 4 | 12 | (il n'y a pas de 13e mois) | 2027-01-01 |

Le numéro 12 ne provoque **aucune erreur** : la date déborde sur janvier de l'année
suivante. TypeScript ne peut pas vous protéger ici, car `12` est un `number` valable.

Essayez : remplacez `12` par `13`. Vous obtenez `2027-02-01`.

> **Astuce :** quand vous avez un mois « humain » (1 à 12), écrivez toujours
> `mois - 1` au moment de créer la date, et `+ 1` au moment de le lire.

### À partir d'un texte ISO

Un texte au format ISO se lit directement, avec les mois **normaux** (03 = mars).
Terminez-le par `Z` pour dire « temps universel » :

```typescript
const d = new Date("2026-03-05T14:30:00Z");
console.log(d.toISOString());
console.log(d.getTime() === Date.UTC(2026, 2, 5, 14, 30));
```

```sortie
2026-03-05T14:30:00.000Z
true
```

> **Attention :** un texte dans un autre format, comme `"05/03/2026"`, donne une date
> **invalide** (ou pire, une date fausse). Utilisez le format ISO, ou découpez le texte
> vous-même (voir [les expressions régulières](regex.md#renvoyer-t-ou-null)).

> **À retenir :** avec des nombres, les mois vont de 0 à 11. Avec un texte ISO, ils
> vont de 01 à 12.

## UTC ou heure locale

**UTC** est l'heure de référence mondiale, la même pour tout le monde, sans heure
d'été. L'**heure locale** dépend de l'endroit où tourne le programme : à Paris, elle a
une ou deux heures d'avance sur UTC.

```text
   le même instant, vu depuis trois villes

   UTC         ──────●──────  14:30
   Paris       ──────●──────  15:30
   New York    ──────●──────  09:30
```

L'objet `Date` a donc **deux familles** de méthodes :

| Heure locale (à éviter ici) | UTC (utilisée dans cette page) |
|-----------------------------|--------------------------------|
| `getHours()` | `getUTCHours()` |
| `getDate()` | `getUTCDate()` |
| `setDate()` | `setUTCDate()` |
| `new Date(2026, 2, 5)` | `new Date(Date.UTC(2026, 2, 5))` |

Les méthodes « locales » donnent un résultat différent selon l'ordinateur. Un programme
qui marche à Paris peut se tromper d'un jour sur un serveur à Tokyo. Utilisez toujours
la famille **UTC** pour calculer, et ne passez à un fuseau précis qu'au moment
d'**afficher**.

> **À retenir :** calculez en UTC (`getUTC…`, `setUTC…`, `Date.UTC`). Le résultat est
> alors le même sur toutes les machines.

## Lire les morceaux d'une date

Chaque méthode `getUTC…` renvoie un `number` :

```typescript
const d = new Date(Date.UTC(2026, 2, 5, 14, 30));

console.log("année :", d.getUTCFullYear());
console.log("mois :", d.getUTCMonth(), "(donc mars)");
console.log("jour :", d.getUTCDate());
console.log("heure :", d.getUTCHours());
console.log("minute :", d.getUTCMinutes());
console.log("jour de la semaine :", d.getUTCDay());
```

```sortie
année : 2026
mois : 2 (donc mars)
jour : 5
heure : 14
minute : 30
jour de la semaine : 4
```

`getUTCDay()` donne le jour de la semaine : **0 pour dimanche**, 1 pour lundi… et 4
pour jeudi. Le 5 mars 2026 est un jeudi.

Pour obtenir le nom du jour, un tableau suffit :

```typescript
const JOURS = ["dimanche", "lundi", "mardi", "mercredi", "jeudi", "vendredi", "samedi"] as const;

const d = new Date(Date.UTC(2026, 2, 5));
const nom = JOURS[d.getUTCDay()];
console.log(nom);
```

```sortie
jeudi
```

Essayez : remplacez le `5` par `8`. Vous obtenez `dimanche`.

> **À retenir :** `getUTCMonth()` renvoie 0 à 11, `getUTCDay()` renvoie 0 (dimanche)
> à 6 (samedi).

## Afficher une date en français

`Intl.DateTimeFormat` met une date en forme selon une langue. Donnez `"fr-FR"` pour le
français de France, et **toujours** `timeZone: "UTC"` pour obtenir le même affichage
partout.

```typescript
const d = new Date(Date.UTC(2026, 2, 5, 14, 30));

const court = new Intl.DateTimeFormat("fr-FR", { timeZone: "UTC" });
const long = new Intl.DateTimeFormat("fr-FR", { timeZone: "UTC", dateStyle: "long" });
const complet = new Intl.DateTimeFormat("fr-FR", {
  timeZone: "UTC",
  dateStyle: "full",
  timeStyle: "short",
});

console.log(court.format(d));
console.log(long.format(d));
console.log(complet.format(d));
```

```sortie
05/03/2026
5 mars 2026
jeudi 5 mars 2026 à 14:30
```

### Des options vérifiées par TypeScript

Les options ont un type : `Intl.DateTimeFormatOptions`. TypeScript connaît donc les
valeurs permises. Une faute de frappe, comme `dateStyle: "longue"`, est refusée avant
même l'exécution (voir les [erreurs fréquentes](#erreurs-frequentes)).

Vous pouvez ranger vos réglages dans une variable typée et choisir chaque morceau :

```typescript
const options: Intl.DateTimeFormatOptions = {
  timeZone: "UTC",
  weekday: "long",
  day: "numeric",
  month: "long",
};

const format = new Intl.DateTimeFormat("fr-FR", options);
console.log(format.format(new Date(Date.UTC(2026, 6, 14))));
console.log(format.format(new Date(Date.UTC(2026, 11, 25))));
```

```sortie
mardi 14 juillet
vendredi 25 décembre
```

Essayez : ajoutez `year: "numeric"` aux options. Les deux lignes se terminent alors par
`2026`.

> **À retenir :** `Intl.DateTimeFormat("fr-FR", { timeZone: "UTC", … })` affiche en
> français ; le type `Intl.DateTimeFormatOptions` vérifie vos options.

## Calculer avec des dates

### Comparer deux dates

Pour comparer, comparez les nombres du chronomètre, avec `getTime()` :

```typescript
const depart = new Date(Date.UTC(2026, 0, 10));
const arrivee = new Date(Date.UTC(2026, 0, 24));

console.log(depart.getTime() < arrivee.getTime());
console.log(depart.getTime() === new Date("2026-01-10T00:00:00Z").getTime());
console.log(depart === new Date("2026-01-10T00:00:00Z"));
```

```sortie
true
true
false
```

La dernière ligne affiche `false` : `===` compare deux **objets**, pas leur contenu.
Ce sont deux boîtes différentes, même si elles contiennent le même nombre.

### L'écart en jours

Un jour dure 24 × 60 × 60 × 1000 = 86 400 000 millisecondes. Pour un écart en jours,
on soustrait les deux nombres, puis on divise.

```typescript
const MS_PAR_JOUR = 24 * 60 * 60 * 1000;

const depart = new Date(Date.UTC(2026, 0, 10));
const arrivee = new Date(Date.UTC(2026, 0, 24));

const ecartMs = arrivee.getTime() - depart.getTime();
const jours = Math.round(ecartMs / MS_PAR_JOUR);
console.log(ecartMs);
console.log(`${jours} jours`);
```

```sortie
1209600000
14 jours
```

| Étape | Calcul | Valeur |
|-------|--------|--------|
| 1 | `depart.getTime()` | 1 768 003 200 000 |
| 2 | `arrivee.getTime()` | 1 769 212 800 000 |
| 3 | `ecartMs` = 2 − 1 | 1 209 600 000 |
| 4 | `ecartMs / MS_PAR_JOUR` | 14 |

`Math.round` arrondit au jour le plus proche : utile si les heures ne sont pas
exactement les mêmes.

> **Note :** en JavaScript, on voit souvent `arrivee - depart`. TypeScript le
> **refuse** : on ne soustrait pas deux objets. Passez par `getTime()` (voir les
> [erreurs fréquentes](#erreurs-frequentes)).

### Ajouter des jours

Pour ajouter des jours, on **copie** la date, puis on change le jour de la copie avec
`setUTCDate`. Si le jour dépasse la fin du mois, la date passe toute seule au mois
suivant.

```typescript
const facture = new Date(Date.UTC(2026, 0, 20));

const echeance = new Date(facture.getTime());
echeance.setUTCDate(echeance.getUTCDate() + 30);

console.log(facture.toISOString().slice(0, 10));
console.log(echeance.toISOString().slice(0, 10));
```

```sortie
2026-01-20
2026-02-19
```

| Étape | Instruction | `echeance` |
|-------|-------------|------------|
| 1 | copie de `facture` | 2026-01-20 |
| 2 | `getUTCDate()` | 20 |
| 3 | `20 + 30` | 50 |
| 4 | `setUTCDate(50)` : janvier n'a que 31 jours, 50 − 31 = 19 | 2026-02-19 |

Pourquoi copier ? Parce que `setUTCDate` **modifie** l'objet. Sans copie, la date de la
facture changerait aussi. C'est comme écrire sur l'original d'un document au lieu
d'écrire sur une photocopie.

Essayez : remplacez `+ 30` par `- 25`. Vous obtenez `2025-12-26` : le calcul recule
même d'une année.

> **À retenir :** comparez et soustrayez avec `getTime()`. Pour ajouter des jours,
> copiez la date puis utilisez `setUTCDate`.

## Des fonctions de dates bien typées

Les calculs de dates sont faciles à rater. Le bon réflexe : les écrire **une fois**,
dans de petites fonctions typées, et ne plus y toucher. Le reste du programme appelle
ces fonctions.

```typescript
const MS_PAR_JOUR = 24 * 60 * 60 * 1000;

// Lit "2026-03-05" ; renvoie null si le texte n'est pas une vraie date.
function lireJour(texte: string): Date | null {
  if (!/^\d{4}-\d{2}-\d{2}$/.test(texte)) {
    return null;
  }
  const d = new Date(`${texte}T00:00:00Z`);
  if (Number.isNaN(d.getTime()) || d.toISOString().slice(0, 10) !== texte) {
    return null;
  }
  return d;
}

function ajouterJours(date: Date, jours: number): Date {
  const copie = new Date(date.getTime());
  copie.setUTCDate(copie.getUTCDate() + jours);
  return copie;
}

function ecartEnJours(debut: Date, fin: Date): number {
  return Math.round((fin.getTime() - debut.getTime()) / MS_PAR_JOUR);
}

const enFrancais = new Intl.DateTimeFormat("fr-FR", { timeZone: "UTC", dateStyle: "long" });

function afficher(date: Date): string {
  return enFrancais.format(date);
}

for (const texte of ["2026-01-20", "2026-02-30", "20/01/2026"]) {
  const jour = lireJour(texte);
  if (jour === null) {
    console.log(`${texte} : date invalide`);
    continue;
  }
  const echeance = ajouterJours(jour, 30);
  console.log(`${afficher(jour)} -> à payer avant le ${afficher(echeance)} (${ecartEnJours(jour, echeance)} jours)`);
}
```

```sortie
20 janvier 2026 -> à payer avant le 19 février 2026 (30 jours)
2026-02-30 : date invalide
20/01/2026 : date invalide
```

Ce que chaque fonction garantit :

| Fonction | Reçoit | Renvoie | Ce qu'elle évite |
|----------|--------|---------|------------------|
| `lireJour` | `string` | `Date \| null` | les textes mal formés et les dates impossibles |
| `ajouterJours` | `Date`, `number` | une **nouvelle** `Date` | de modifier l'original |
| `ecartEnJours` | `Date`, `Date` | `number` | la soustraction d'objets |
| `afficher` | `Date` | `string` | d'oublier `timeZone: "UTC"` |

Pourquoi `2026-02-30` est-il refusé ? `new Date` accepte ce texte et le transforme
discrètement en 2 mars. `lireJour` relit la date obtenue avec `toISOString()` : comme
ce n'est plus le même texte, elle renvoie `null`.

Grâce au type `Date | null`, TypeScript **oblige** l'appelant à traiter le cas
invalide avant d'utiliser la date (c'est le
[rétrécissement](../debuter/glossaire.md#retrecissement)).

Essayez : supprimez le bloc `if (jour === null) { … }`. TypeScript refuse le programme :
`jour` est peut-être `null`.

> **À retenir :** enfermez les calculs de dates dans de petites fonctions typées. Une
> fonction qui lit un texte renvoie `Date | null`.

## Et Temporal ?

Une nouvelle API de JavaScript, **Temporal**, corrige les défauts de `Date` : des mois
qui commencent à 1, des objets non modifiables et de vrais fuseaux horaires. Elle n'est
pas encore disponible dans Node 22 ni dans les types de TypeScript 5.8 : en attendant,
utilisez `Date` avec les précautions de cette page.

## Erreurs fréquentes

### Soustraire deux dates directement

```typescript norun
const depart = new Date(Date.UTC(2026, 0, 10));
const arrivee = new Date(Date.UTC(2026, 0, 24));
const ecart = arrivee - depart;
console.log(ecart);
```

```text
main.ts(3,15): error TS2362: The left-hand side of an arithmetic operation must be of type 'any', 'number', 'bigint' or an enum type.
main.ts(3,25): error TS2363: The right-hand side of an arithmetic operation must be of type 'any', 'number', 'bigint' or an enum type.
```

JavaScript accepte cette soustraction, TypeScript non : `arrivee` et `depart` sont des
objets, pas des nombres. Écrivez `arrivee.getTime() - depart.getTime()`.

### Se tromper dans une option d'affichage

```typescript norun
const format = new Intl.DateTimeFormat("fr-FR", { timeZone: "UTC", dateStyle: "longue" });
```

```text
main.ts(1,68): error TS2769: No overload matches this call.
  Overload 1 of 2, '(locales?: LocalesArgument, options?: DateTimeFormatOptions | undefined): DateTimeFormat', gave the following error.
    Type '"longue"' is not assignable to type '"long" | "short" | "medium" | "full" | undefined'. Did you mean '"long"'?
  Overload 2 of 2, '(locales?: string | string[] | undefined, options?: DateTimeFormatOptions | undefined): DateTimeFormat', gave the following error.
    Type '"longue"' is not assignable to type '"long" | "short" | "medium" | "full" | undefined'. Did you mean '"long"'?
```

Le message est long, mais la fin dit tout : `Did you mean '"long"'?`. Les valeurs des
options sont en anglais : `"short"`, `"medium"`, `"long"`, `"full"`.

### Passer un texte au lieu d'une date

```typescript norun
function ajouterJours(date: Date, jours: number): Date {
  const copie = new Date(date.getTime());
  copie.setUTCDate(copie.getUTCDate() + jours);
  return copie;
}

console.log(ajouterJours("2026-03-01", 7));
```

```text
main.ts(7,26): error TS2345: Argument of type 'string' is not assignable to parameter of type 'Date'.
```

Un texte n'est pas une `Date`. Transformez-le d'abord, et vérifiez le résultat :
`const d = lireJour("2026-03-01"); if (d) { ajouterJours(d, 7); }`.

### Utiliser une date invalide

```typescript norun
const d = new Date("31/12/2026");
console.log(d.toISOString());
```

```text
RangeError: Invalid time value
```

Ce n'est **pas** une erreur de TypeScript : `new Date` renvoie toujours une `Date`, même
invalide. L'erreur arrive à l'exécution. Utilisez le format ISO (`"2026-12-31"`) et
vérifiez avec `Number.isNaN(d.getTime())`, comme dans `lireJour`.

## Testez-vous

**Question 1 :** quelle date affiche ce programme ?

```typescript
const d = new Date(Date.UTC(2026, 9, 1));
console.log(d.toISOString().slice(0, 10));
```

> **Réponse**
>
> Le 1er **octobre**, car les mois commencent à 0 : 9 est le dixième mois.
>
> ```sortie
> 2026-10-01
> ```

**Question 2 :** pourquoi utiliser `getUTCDate()` plutôt que `getDate()` ?

> **Réponse**
>
> `getDate()` lit la date dans le fuseau de l'ordinateur : le résultat peut changer
> d'une machine à l'autre. `getUTCDate()` donne le même résultat partout.

**Question 3 :** écrivez une fonction `estAvant(a: Date, b: Date): boolean` et
testez-la avec le 1er et le 2 mars 2026.

> **Réponse**
>
> ```typescript
> function estAvant(a: Date, b: Date): boolean {
>   return a.getTime() < b.getTime();
> }
>
> const premier = new Date(Date.UTC(2026, 2, 1));
> const deux = new Date(Date.UTC(2026, 2, 2));
> console.log(estAvant(premier, deux), estAvant(deux, premier));
> ```
>
> ```sortie
> true false
> ```

## En résumé

- Un objet `Date` enveloppe un nombre de millisecondes depuis le 1er janvier 1970 (UTC).
- Avec `Date.UTC` et `new Date(année, mois, …)`, les mois vont de **0 à 11**.
- Calculez en UTC (`getUTC…`, `setUTC…`) et affichez avec
  `Intl.DateTimeFormat("fr-FR", { timeZone: "UTC" })`.
- TypeScript refuse `date1 - date2` : passez par `getTime()`.
- Pour ajouter des jours, copiez la date puis utilisez `setUTCDate`.
- Rangez ces calculs dans de petites fonctions typées, qui renvoient `Date | null`
  quand la lecture peut échouer.

Suite : [Un outil en ligne de commande typé](ligne-de-commande.md).
