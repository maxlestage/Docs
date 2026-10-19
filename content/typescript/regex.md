# Les expressions régulières en TypeScript

Cette page montre ce que TypeScript ajoute aux expressions régulières : des résultats qui peuvent valoir `null`, des groupes typés et des fonctions d'analyse sûres.

> **Objectifs :**
>
> - vous saurez écrire une regex et reconnaître le type `RegExp` ;
> - vous saurez traiter le `null` renvoyé par `match` grâce au rétrécissement ;
> - vous saurez lire les groupes numérotés et nommés sans erreur de type ;
> - vous saurez remplacer avec une fonction typée et parcourir `matchAll` ;
> - vous saurez écrire un analyseur qui renvoie un objet typé ou `null`, et un garde de
>   type qui valide un format ;
> - vous saurez échapper un texte saisi par l'utilisateur.

## Écrire une regex en TypeScript

Une [expression régulière](../debuter/glossaire.md#expression-reguliere) (ou **regex**)
décrit un motif de texte. La syntaxe des motifs (`\d`, `+`, `[a-z]`, `^`, `$`…) est
exactement celle de JavaScript : elle est expliquée dans
[les expressions régulières en JavaScript](../javascript/regex.md). Cette page ne la
répète pas : elle montre ce que les **types** changent.

Une regex s'écrit entre deux barres obliques, ou se construit avec `new RegExp` à partir
d'un texte. Dans les deux cas, son type est `RegExp`.

```typescript
const codePostal: RegExp = /^\d{5}$/;
const mot = "chat";
const motif = new RegExp(mot, "i");

console.log(codePostal.test("75011"));
console.log(codePostal.test("750"));
console.log(motif.test("Le CHAT dort."));
console.log(motif.source, motif.flags);
```

```sortie
true
false
true
chat i
```

`test` renvoie un `boolean` : c'est la méthode la plus simple, et elle ne pose aucun
problème de type.

> **Note :** TypeScript vérifie aussi la **syntaxe** d'une regex écrite entre barres.
> Une parenthèse oubliée dans `/(abc/` est une erreur de compilation. Avec
> `new RegExp("(abc")`, le motif est un simple texte : l'erreur n'arrive qu'à
> l'exécution.

## match peut renvoyer null

`match` cherche le motif dans un texte. S'il le trouve, il renvoie un tableau ; sinon,
il renvoie `null`. Son type de retour le dit : `RegExpMatchArray | null`.

En mode strict, TypeScript refuse que vous utilisiez le résultat sans avoir écarté
`null` (voir les [erreurs fréquentes](#erreurs-frequentes)). Un simple `if` suffit : c'est
le [rétrécissement](../debuter/glossaire.md#retrecissement).

```typescript
function premierNombre(texte: string): number | null {
  const m = texte.match(/\d+/);
  if (m === null) {
    return null;
  }
  // Ici, m est un RegExpMatchArray.
  return Number(m[0]);
}

console.log(premierNombre("Il reste 12 places."));
console.log(premierNombre("Complet."));
```

```sortie
12
null
```

Pour une valeur par défaut, `?.` et `??` sont plus courts :
`texte.match(/\d+/)?.[0] ?? "inconnu"`. `?.[0]` ne lit la case 0 que si le résultat
n'est pas `null`.

### Avec le drapeau g

Avec le drapeau `g`, `match` renvoie **toutes** les correspondances, sous forme de
tableau de textes. Mais quand il n'y en a aucune, c'est encore `null`, pas un tableau
vide :

```typescript
function nombres(texte: string): number[] {
  const trouves = texte.match(/\d+/g) ?? [];
  return trouves.map(Number);
}

console.log(nombres("3 pommes, 12 poires, 7 kiwis"));
console.log(nombres("aucun fruit"));
```

```sortie
[ 3, 12, 7 ]
[]
```

`?? []` remplace `null` par un tableau vide : la suite du code n'a plus à s'en soucier.

## Les drapeaux

Les drapeaux se placent après la dernière barre : `/motif/gi`. Le chapitre JavaScript
les détaille ; voici le rappel.

| Drapeau | Effet |
|---------|-------|
| `g` | toutes les correspondances |
| `i` | ignore majuscules et minuscules |
| `m` | `^` et `$` marchent sur chaque ligne |
| `s` | le `.` accepte aussi le retour à la ligne |
| `u` | lit correctement les accents et les émojis, active `\p{…}` |
| `v` | une version plus récente de `u`, avec des ensembles de caractères plus riches |

```typescript
const prenoms = "Élise, zoé, Ana";
console.log(prenoms.match(/\p{Lu}\p{Ll}+/gu));
console.log(/début.fin/s.test("début\nfin"));
```

```sortie
[ 'Élise', 'Ana' ]
true
```

`\p{Lu}` est une lettre majuscule de n'importe quelle langue, `\p{Ll}` une minuscule :
`zoé` est écarté car il commence par une minuscule.

> **Attention :** TypeScript refuse le drapeau `v` si la cible de compilation
> (`target` dans `tsconfig.json`) est plus ancienne que `es2024`. Le drapeau `u` suffit
> dans presque tous les cas.

## Les groupes numérotés

Des parenthèses dans le motif forment des **groupes**. Le texte trouvé par le premier
groupe est dans `m[1]`, le deuxième dans `m[2]`, et ainsi de suite. `m[0]` contient
toute la correspondance.

```typescript
const m = "Rendez-vous à 14h30.".match(/(\d{1,2})h(\d{2})/);
if (m) {
  const heures = Number(m[1]);
  const minutes = Number(m[2]);
  console.log(`${m[0]} = ${heures * 60 + minutes} minutes`);
}
```

```sortie
14h30 = 870 minutes
```

TypeScript donne le type `string` à `m[1]` et `m[2]`. Il ne compte pas les groupes : il
ne sait pas combien votre motif en contient.

> **Attention :** un groupe **facultatif** qui n'a rien trouvé vaut `undefined`, alors
> que TypeScript annonce `string`. Pour un groupe suivi de `?`, annotez
> la variable avec `string | undefined` (`const centimes: string | undefined = m[2];`)
> et traitez le cas manquant.

## Les groupes nommés

Un groupe nommé s'écrit `(?<nom>…)`. On le lit dans `m.groups`, plus lisible que des
numéros. Mais attention au type de `groups` :

```text
{ [key: string]: string } | undefined
```

Deux conséquences :

- `groups` peut valoir `undefined` (quand le motif n'a aucun groupe nommé) : il faut
  donc le vérifier, comme `m` ;
- TypeScript ne connaît pas les noms de vos groupes : `groups.annee` et `groups.anee`
  (faute de frappe) ont tous les deux le type `string`.

```typescript
const motif = /(?<annee>\d{4})-(?<mois>\d{2})-(?<jour>\d{2})/;
const m = "Livraison le 2026-10-19.".match(motif);

if (m?.groups) {
  const { annee, mois, jour } = m.groups;
  console.log(`${jour}/${mois}/${annee}`);
}
```

```sortie
19/10/2026
```

`m?.groups` vérifie d'un coup que `m` n'est pas `null` et que `groups` existe. Dans le
`if`, TypeScript sait que `m.groups` est un objet.

## Parcourir avec matchAll

`matchAll` renvoie un [itérateur](../debuter/glossaire.md#iterateur) : chaque élément
est une correspondance complète, avec ses groupes. Le motif doit avoir le drapeau `g`.
On le parcourt avec `for…of`, et chaque élément a le type `RegExpExecArray`.

```typescript
const panier = "pommes=3; poires=12; kiwis=7";
let total = 0;

for (const m of panier.matchAll(/(?<fruit>\w+)=(?<quantite>\d+)/g)) {
  const fruit = m.groups?.fruit ?? "?";
  const quantite = Number(m.groups?.quantite ?? 0);
  console.log(`${fruit.padEnd(7)} ${quantite} (position ${m.index})`);
  total += quantite;
}
console.log(`Total : ${total}`);
```

```sortie
pommes  3 (position 0)
poires  12 (position 10)
kiwis   7 (position 21)
Total : 22
```

Contrairement à `match`, `matchAll` ne renvoie jamais `null` : sans correspondance,
l'itérateur est simplement vide. `Array.from(…)` le transforme en vrai tableau.

## Remplacer avec une fonction typée

`replace` et `replaceAll` acceptent une fonction à la place du texte de remplacement.
Elle reçoit la correspondance, puis un argument par groupe. Passée directement,
TypeScript devine les types (`string` pour chaque groupe). Si vous écrivez la fonction
à part, typez ses paramètres vous-même. Ici, on passe des dates au format français :

```typescript
function versFrancais(_tout: string, annee: string, mois: string, jour: string): string {
  return `${jour}/${mois}/${annee}`;
}

const journal = "Début : 2026-10-19, fin : 2026-12-31.";
console.log(journal.replaceAll(/(\d{4})-(\d{2})-(\d{2})/g, versFrancais));
```

```sortie
Début : 19/10/2026, fin : 31/12/2026.
```

Le premier paramètre est toute la correspondance ; le `_` du nom signale qu'on ne s'en
sert pas.

> **Note :** `replaceAll` avec une regex exige le drapeau `g`, sinon il lève une
> `TypeError` à l'exécution. TypeScript ne le détecte pas.

## Découper avec split

`split` accepte une regex comme séparateur. Le résultat est toujours un `string[]` :
pas de `null` à gérer.

```typescript
const saisie = "rouge, vert ;bleu  ;  jaune";
const couleurs: string[] = saisie.split(/\s*[,;]\s*/);
console.log(couleurs);
```

```sortie
[ 'rouge', 'vert', 'bleu', 'jaune' ]
```

Le séparateur « une virgule ou un point-virgule, avec des espaces autour » nettoie la
saisie en une seule ligne.

## Écrire un analyseur typé

Le grand intérêt de TypeScript : enfermer la regex dans une fonction qui transforme un
texte en **objet typé**. Le reste du programme ne manipule plus de textes, mais des
objets dont chaque champ est vérifié.

### Renvoyer T ou null

Voici une fonction qui lit une date `2026-10-19`. Elle renvoie un objet `DateSimple`, ou
`null` si le texte n'a pas la bonne forme :

```typescript
interface DateSimple {
  annee: number;
  mois: number;
  jour: number;
}

function lireDate(texte: string): DateSimple | null {
  const m = texte.match(/^(?<annee>\d{4})-(?<mois>\d{2})-(?<jour>\d{2})$/);
  if (!m?.groups) {
    return null;
  }
  const annee = Number(m.groups.annee);
  const mois = Number(m.groups.mois);
  const jour = Number(m.groups.jour);
  if (mois < 1 || mois > 12 || jour < 1 || jour > 31) {
    return null;
  }
  return { annee, mois, jour };
}

for (const texte of ["2026-10-19", "2026-13-01", "19/10/2026"]) {
  const d = lireDate(texte);
  console.log(texte, "->", d ?? "date invalide");
}
```

```sortie
2026-10-19 -> { annee: 2026, mois: 10, jour: 19 }
2026-13-01 -> date invalide
19/10/2026 -> date invalide
```

La regex vérifie la **forme** (quatre chiffres, un tiret…) ; le code vérifie le **sens**
(un mois entre 1 et 12). Les deux sont utiles.

### Renvoyer une union discriminée

Quand plusieurs formes sont possibles, renvoyez une
[union discriminée](unions.md#les-unions-discriminees). Ici, une ligne de journal
(*log*) est une requête, une erreur, ou une ligne inconnue :

```typescript
type Ligne =
  | { kind: "requete"; methode: "GET" | "POST"; chemin: string; duree: number }
  | { kind: "erreur"; message: string }
  | { kind: "inconnue"; texte: string };

function lireLigne(texte: string): Ligne {
  const req = texte.match(/^(?<methode>GET|POST) (?<chemin>\S+) (?<duree>\d+)ms$/);
  if (req?.groups) {
    const methode = req.groups.methode === "GET" ? "GET" : "POST";
    return { kind: "requete", methode, chemin: req.groups.chemin, duree: Number(req.groups.duree) };
  }
  const err = texte.match(/^ERREUR: (?<message>.+)$/);
  if (err?.groups) {
    return { kind: "erreur", message: err.groups.message };
  }
  return { kind: "inconnue", texte };
}

const journal = ["GET /accueil 12ms", "ERREUR: base indisponible", "POST /panier 85ms", "???"];

for (const texte of journal) {
  const ligne = lireLigne(texte);
  switch (ligne.kind) {
    case "requete":
      console.log(`${ligne.methode} ${ligne.chemin} en ${ligne.duree} ms`);
      break;
    case "erreur":
      console.log(`Problème : ${ligne.message}`);
      break;
    case "inconnue":
      console.log(`Ignorée : ${ligne.texte}`);
      break;
  }
}
```

```sortie
GET /accueil en 12 ms
Problème : base indisponible
POST /panier en 85 ms
Ignorée : ???
```

Dans chaque `case`, TypeScript sait quels champs existent : `ligne.duree` n'est permis
que pour une requête. Le texte brut n'est analysé qu'à un seul endroit.

## Valider avec un garde de type

Une regex peut aussi servir de **garde de type** : une fonction qui renvoie
`v is T` (voir [vos propres gardes de type](unions.md#vos-propres-gardes-de-type)).
Ici, le type `Email` décrit « un texte qui contient un `@` », et la regex fait la vraie
vérification :

```typescript
type Email = `${string}@${string}`;

function estEmail(texte: string): texte is Email {
  return /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(texte);
}

function envoyer(destinataire: Email) {
  console.log(`Envoi à ${destinataire}`);
}

for (const saisie of ["ada@exemple.fr", "ada@", "pas un e-mail"]) {
  if (estEmail(saisie)) {
    envoyer(saisie);
  } else {
    console.log(`Adresse refusée : ${saisie}`);
  }
}
```

```sortie
Envoi à ada@exemple.fr
Adresse refusée : ada@
Adresse refusée : pas un e-mail
```

`envoyer` n'accepte qu'un `Email` : impossible de lui passer un texte non vérifié par
mégarde. Le même modèle marche pour un code postal :

```typescript
type CodePostal = `${number}`;

function estCodePostal(texte: string): texte is CodePostal {
  return /^\d{5}$/.test(texte);
}

const saisies = ["75011", "7501", "75O11"];
console.log(saisies.filter(estCodePostal));
```

```sortie
[ '75011' ]
```

Avec `filter`, le garde de type rétrécit tout le tableau : le résultat a le type
`CodePostal[]`.

> **Astuce :** une regex d'e-mail parfaite n'existe pas. Restez simple (« quelque chose,
> un `@`, quelque chose, un point, quelque chose ») et envoyez un e-mail de confirmation
> pour être sûr.

## Types de gabarit ou regex ?

Les [template literal types](types-avances.md#les-template-literal-types) comme
`` `${string}@${string}` `` ressemblent à des motifs. Mais ils n'ont pas le même rôle :

| | Template literal type | Regex |
|---|---|---|
| Quand ? | à la compilation | à l'exécution |
| Sur quoi ? | les textes écrits dans le code | n'importe quel texte, même saisi |
| Précision | grossière : `${number}` accepte `"1.5"` | fine : `\d{5}` exige 5 chiffres |

Par exemple, `"99h99"` a bien le type `` `${number}h${number}` ``, mais la regex
`/^([01]\d|2[0-3])h[0-5]\d$/` le refuse. Le type sert à documenter et à guider ; la
regex vérifie vraiment. Pour un texte venu de l'extérieur, seule la regex (dans un garde
de type) protège votre programme.

## Échapper une saisie

Pour chercher un texte saisi par l'utilisateur, on construit la regex avec
`new RegExp`. Mais si le texte contient `.`, `(`, `+` ou `?`, ces caractères gardent
leur sens spécial : la recherche devient fausse, ou lève une erreur.

Une méthode `RegExp.escape` a été ajoutée récemment à JavaScript. Elle n'existe pas
encore dans Node 22, et TypeScript 5.8 ne la connaît pas (voir les
[erreurs fréquentes](#erreurs-frequentes)). Écrivez donc votre propre fonction :

```typescript
function echapper(texte: string): string {
  return texte.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

function compter(texte: string, recherche: string): number {
  const motif = new RegExp(echapper(recherche), "gi");
  return texte.match(motif)?.length ?? 0;
}

console.log(echapper("1+1=2 (vrai ?)"));
console.log(compter("Prix : 3.5 €, ou 3.5 € en promo, pas 345 €", "3.5"));
```

```sortie
1\+1=2 \(vrai \?\)
2
```

Dans le remplacement, `$&` représente le caractère trouvé : chaque caractère spécial
est précédé d'une barre oblique inverse. Sans `echapper`, le `.` de `3.5` accepterait
aussi le `345`.

## Erreurs fréquentes

### Utiliser le résultat de match sans vérifier null

```typescript norun
const m = "2026-10-19".match(/(\d{4})-(\d{2})/);
console.log(m[1]);
```

```text
main.ts(2,13): error TS18047: 'm' is possibly 'null'.
```

`match` renvoie `null` quand il ne trouve rien. Vérifiez d'abord : `if (m) { … }`, ou
lisez avec `m?.[1]`.

### Lire groups sans vérifier undefined

```typescript norun
const m = "2026-10-19".match(/(?<annee>\d{4})/);
if (m) {
  console.log(m.groups.annee);
}
```

```text
main.ts(3,15): error TS18048: 'm.groups' is possibly 'undefined'.
```

Vérifier `m` ne suffit pas : `groups` peut aussi manquer. Écrivez
`if (m?.groups) { … }`.

### Utiliser le drapeau v avec une cible ancienne

```typescript norun
const emoji = /\p{Emoji}/v;
```

```text
main.ts(1,26): error TS1501: This regular expression flag is only available when targeting 'es2024' or later.
```

Passez `target` à `es2024` (ou plus) dans `tsconfig.json`, ou utilisez le drapeau `u` :
`/\p{Emoji}/u`.

### Appeler RegExp.escape

```typescript norun
const motif = new RegExp(RegExp.escape("3.5"));
```

```text
main.ts(1,33): error TS2339: Property 'escape' does not exist on type 'RegExpConstructor'.
```

`RegExp.escape` est trop récente pour TypeScript 5.8 et pour Node 22 (où elle vaut
`undefined`). Utilisez votre propre fonction `echapper`, vue plus haut.

## Testez-vous

**Question 1 :** quel est le type de retour de `texte.match(/\d+/)`, et que faut-il
faire avant de lire `[0]` ?

> **Réponse**
>
> `RegExpMatchArray | null`. Il faut écarter `null` : avec un `if (m)`, ou avec
> `m?.[0]` suivi d'une valeur de secours `?? …`.

**Question 2 :** pourquoi `if (m) { m.groups.nom }` est-il refusé ?

> **Réponse**
>
> `groups` a le type `{ [key: string]: string } | undefined` : il peut manquer même si
> `m` existe. Il faut vérifier `m.groups`, par exemple avec `if (m?.groups)`.

**Question 3 :** écrivez un garde de type `estHeure(texte: string): texte is Heure` pour
``type Heure = `${number}:${number}` ``, qui accepte `08:30` mais refuse `25:00`.

> **Réponse**
>
> ```typescript
> type Heure = `${number}:${number}`;
>
> function estHeure(texte: string): texte is Heure {
>   return /^([01]\d|2[0-3]):[0-5]\d$/.test(texte);
> }
>
> console.log(estHeure("08:30"), estHeure("25:00"));
> ```
>
> ```sortie
> true false
> ```

## En résumé

- La syntaxe des regex est celle de JavaScript ; TypeScript leur donne le type `RegExp`.
- `match` renvoie `RegExpMatchArray | null` : écartez `null` avec `if`, `?.` ou `??`.
- `m.groups` est `{ [key: string]: string } | undefined` : vérifiez-le, et sachez que
  TypeScript ne connaît pas les noms de vos groupes.
- Enfermez chaque regex dans une fonction qui renvoie un objet typé, `T | null` ou une
  union discriminée.
- Une regex dans un garde de type `texte is T` valide à l'exécution ce que les types ne
  vérifient qu'à la compilation.
- Échappez toute saisie avant `new RegExp` avec votre propre fonction `echapper`.

Suite : [TypeScript pour le web](web.md).
