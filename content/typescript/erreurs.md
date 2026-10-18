# Gérer les erreurs

Cette page explique comment lever et attraper des erreurs en TypeScript, comment manipuler une erreur de type `unknown`, et comment renvoyer un résultat au lieu de lever une erreur.

> **Objectifs :**
>
> - vous saurez lever une erreur avec `throw new Error(...)` ;
> - vous saurez l'attraper avec `try`, `catch` et `finally` ;
> - vous saurez rétrécir une erreur `unknown` avec `instanceof Error` ;
> - vous saurez créer vos propres classes d'erreur ;
> - vous saurez renvoyer un résultat `{ ok: true, … } | { ok: false, … }` et valider
>   une donnée inconnue.

## Une erreur arrête le programme

Quand un problème empêche de continuer, le code **lève** une
[exception](../debuter/glossaire.md#exception) avec `throw`. « Lever » veut dire :
déclencher l'alarme. Sans traitement, le programme s'arrête.

```typescript norun
function diviser(a: number, b: number): number {
  if (b === 0) {
    throw new Error("division par zéro");
  }
  return a / b;
}

console.log("Début");
console.log(diviser(10, 0));
console.log("Fin");
```

```text
Début
file:///home/alice/projet/main.js:3
        throw new Error("division par zéro");
              ^

Error: division par zéro
    at diviser (file:///home/alice/projet/main.js:3:15)
    at file:///home/alice/projet/main.js:8:13
    at ModuleJob.run (node:internal/modules/esm/module_job:343:25)
    at async onImport.tracePromise.__proto__ (node:internal/modules/esm/loader:665:26)
    at async asyncRunEntryPointWithESMLoader (node:internal/modules/run_main:117:5)

Node.js v22.22.0
```

`Fin` ne s'affiche jamais. Le message se lit ainsi :

- `Error: division par zéro` : le type de l'erreur et sa description ;
- les lignes `at …` forment la **trace d'appels** : le chemin suivi jusqu'à l'erreur.
  Cherchez la première ligne qui parle de **votre** fichier.

> **Note :** Node exécute le fichier JavaScript compilé (`main.js`). Les numéros de
> ligne peuvent donc différer un peu de votre fichier `.ts`. L'option `sourceMap`
> corrige cela (voir [la configuration](configuration.md)).

`new Error("…")` crée un objet erreur. Il a deux propriétés utiles : `message` (votre
texte) et `name` (`"Error"`). JavaScript a aussi `TypeError`, `RangeError`,
`SyntaxError`… qui fonctionnent pareil.

## try et catch

`try` (« essayer ») entoure le code risqué. Si une erreur est levée, l'exécution saute
dans `catch` (« attraper »). C'est un filet sous un trapéziste : la chute est
rattrapée, le spectacle continue.

```typescript
function diviser(a: number, b: number): number {
  if (b === 0) {
    throw new Error("division par zéro");
  }
  return a / b;
}

try {
  console.log(diviser(10, 2));
  console.log(diviser(10, 0));
  console.log("jamais affiché");
} catch (e) {
  console.log("Une erreur est arrivée.");
}
console.log("Le programme continue.");
```

```sortie
5
Une erreur est arrivée.
Le programme continue.
```

Dès que `diviser(10, 0)` lève l'erreur, le reste du `try` est sauté.

## L'erreur attrapée est unknown

En JavaScript, on peut lever **n'importe quoi** : `throw "oups"`, `throw 42`… Une
bibliothèque peut le faire. TypeScript ne peut donc pas savoir ce que contient `e`.
Il lui donne le type `unknown` (« inconnu »).

Avec `unknown`, TypeScript interdit tout tant que vous n'avez pas vérifié le type.
C'est le [rétrécissement](unions.md#le-retrecissement-avec-instanceof) vu dans le
chapitre sur les unions : testez avec `instanceof Error`.

```typescript
function lireNombre(texte: string): number {
  const n = Number(texte);
  if (Number.isNaN(n)) {
    throw new Error(`« ${texte} » n'est pas un nombre`);
  }
  return n;
}

for (const saisie of ["12", "douze"]) {
  try {
    console.log(lireNombre(saisie) * 2);
  } catch (e) {
    if (e instanceof Error) {
      console.log(`Erreur : ${e.message}`);
    } else {
      console.log("Erreur inconnue :", e);
    }
  }
}
```

```sortie
24
Erreur : « douze » n'est pas un nombre
```

Dans le `if`, `e` est un `Error` : `e.message` est permis. Le `else` traite le cas
rare où quelqu'un a levé autre chose qu'une erreur.

> **Astuce :** une petite fonction évite de répéter ce test partout :
>
> ```typescript norun
> function messageDe(e: unknown): string {
>   return e instanceof Error ? e.message : String(e);
> }
> ```

## finally

Le bloc `finally` (« finalement ») s'exécute **toujours**, qu'il y ait une erreur ou
non. Il sert à ranger : fermer un fichier, arrêter un minuteur, remettre un état.

```typescript
function traiter(valeur: number) {
  console.log(`Ouverture pour ${valeur}`);
  try {
    if (valeur < 0) {
      throw new Error("valeur négative");
    }
    console.log("Traitement réussi");
  } catch (e) {
    if (e instanceof Error) console.log(`Problème : ${e.message}`);
  } finally {
    console.log("Fermeture");
  }
}

traiter(5);
traiter(-1);
```

```sortie
Ouverture pour 5
Traitement réussi
Fermeture
Ouverture pour -1
Problème : valeur négative
Fermeture
```

## Vos propres classes d'erreur

Une erreur est un objet comme un autre : vous pouvez créer votre propre
[classe](types.md) qui **hérite** de `Error` avec `extends`. Cela permet de
distinguer les erreurs avec `instanceof`, et d'y ajouter des informations.

```typescript
class SoldeInsuffisantError extends Error {
  constructor(public manque: number) {
    super(`solde insuffisant (il manque ${manque} €)`);
    this.name = "SoldeInsuffisantError";
  }
}

function retirer(solde: number, montant: number): number {
  if (montant > solde) {
    throw new SoldeInsuffisantError(montant - solde);
  }
  return solde - montant;
}

try {
  retirer(50, 80);
} catch (e) {
  if (e instanceof SoldeInsuffisantError) {
    console.log(`Refusé : ${e.message}`);
    console.log(`Montant manquant : ${e.manque}`);
  } else {
    throw e;
  }
}
```

```sortie
Refusé : solde insuffisant (il manque 30 €)
Montant manquant : 30
```

Trois détails :

- `super(message)` appelle le constructeur de `Error`, qui range le message ;
- `this.name` change le nom affiché dans la trace d'appels ;
- `throw e` dans le `else` **relance** les erreurs qu'on ne sait pas traiter. Ne les
  avalez pas en silence : vous cacheriez de vrais bugs.

## Renvoyer un résultat au lieu de lever

Une erreur levée est invisible dans la signature : rien dans
`function lireAge(t: string): number` ne prévient qu'elle peut échouer. On oublie
facilement le `try`.

Une autre approche : la fonction **renvoie** soit un succès, soit un échec, dans une
[union discriminée](unions.md#les-unions-discriminees). Le champ `ok` sert d'étiquette.

```typescript
type Resultat<T> = { ok: true; value: T } | { ok: false; error: string };

function lireAge(texte: string): Resultat<number> {
  const n = Number(texte);
  if (!Number.isInteger(n)) {
    return { ok: false, error: `« ${texte} » n'est pas un entier` };
  }
  if (n < 0 || n > 150) {
    return { ok: false, error: `${n} n'est pas un âge réaliste` };
  }
  return { ok: true, value: n };
}

for (const saisie of ["34", "trente", "200"]) {
  const r = lireAge(saisie);
  if (r.ok) {
    console.log(`Âge accepté : ${r.value}`);
  } else {
    console.log(`Refusé : ${r.error}`);
  }
}
```

```sortie
Âge accepté : 34
Refusé : « trente » n'est pas un entier
Refusé : 200 n'est pas un âge réaliste
```

Le type de retour annonce clairement l'échec possible. Et TypeScript vous **oblige** à
tester `r.ok` avant de lire `r.value` (voir les
[erreurs fréquentes](#erreurs-frequentes)). C'est l'idée du type `Result` de Rust
(voir [les erreurs en Rust](../rust/erreurs.md)).

> **À retenir :** levez une erreur pour ce qui est **anormal** (un bug, un fichier de
> configuration manquant). Renvoyez un résultat pour ce qui est **prévisible** (une
> saisie invalide, une recherche sans réponse).

## Valider une donnée inconnue

Les données venues de l'extérieur (un fichier [JSON](../debuter/glossaire.md#json),
une saisie, une réponse web) n'ont aucune garantie de forme. `JSON.parse` renvoie
`any`, ce qui désactive toute vérification. Rangez plutôt le résultat dans une
variable `unknown`, puis vérifiez sa forme pas à pas.

```typescript
interface Joueur {
  nom: string;
  score: number;
}

function validerJoueur(donnee: unknown): Resultat<Joueur> {
  if (typeof donnee !== "object" || donnee === null) {
    return { ok: false, error: "ce n'est pas un objet" };
  }
  if (!("nom" in donnee) || typeof donnee.nom !== "string") {
    return { ok: false, error: "nom manquant ou invalide" };
  }
  if (!("score" in donnee) || typeof donnee.score !== "number") {
    return { ok: false, error: "score manquant ou invalide" };
  }
  return { ok: true, value: { nom: donnee.nom, score: donnee.score } };
}

type Resultat<T> = { ok: true; value: T } | { ok: false; error: string };

const textes = ['{"nom": "Lina", "score": 42}', '{"nom": "Tom"}', "[1, 2]"];

for (const texte of textes) {
  const donnee: unknown = JSON.parse(texte);
  const r = validerJoueur(donnee);
  console.log(r.ok ? `${r.value.nom} a ${r.value.score} points` : `Invalide : ${r.error}`);
}
```

```sortie
Lina a 42 points
Invalide : score manquant ou invalide
Invalide : nom manquant ou invalide
```

Chaque test rétrécit `donnee` un peu plus : d'abord un objet, puis un objet avec un
`nom` texte, puis avec un `score` nombre. À la fin, TypeScript est convaincu.

> **Note :** un tableau est aussi un `object` : `[1, 2]` passe le premier test, puis
> échoue sur le `nom`. Pour de gros formats, des bibliothèques comme Zod écrivent ces
> vérifications à votre place (voir [les modules](modules.md)).

## Erreurs fréquentes

### Lire e.message sans vérifier

```typescript norun
try {
  JSON.parse("{oups");
} catch (e) {
  console.log(e.message);
}
```

```text
main.ts(4,15): error TS18046: 'e' is of type 'unknown'.
```

`e` peut être n'importe quoi. Vérifiez d'abord :
`if (e instanceof Error) console.log(e.message);`.

### Lire value sans tester ok

```typescript norun
type Resultat<T> = { ok: true; value: T } | { ok: false; error: string };
function diviser(a: number, b: number): Resultat<number> {
  if (b === 0) return { ok: false, error: "division par zéro" };
  return { ok: true, value: a / b };
}
const r = diviser(10, 2);
console.log(r.value);
```

```text
main.ts(7,15): error TS2339: Property 'value' does not exist on type 'Resultat<number>'.
  Property 'value' does not exist on type '{ ok: false; error: string; }'.
```

En cas d'échec, il n'y a pas de `value`. Testez l'étiquette :
`if (r.ok) console.log(r.value);`.

### Utiliser une donnée unknown directement

```typescript norun
const donnees: unknown = JSON.parse('{"nom": "Alice"}');
console.log(donnees.nom);
```

```text
main.ts(2,13): error TS18046: 'donnees' is of type 'unknown'.
```

C'est voulu : `unknown` vous force à vérifier. Validez la forme
(`typeof donnees === "object" && donnees !== null && "nom" in donnees`) ou utilisez
une fonction de validation comme `validerJoueur` ci-dessus.

### Lever un texte au lieu d'une Error

```typescript norun
function lireAge(texte: string): number {
  const n = Number(texte);
  if (Number.isNaN(n)) {
    throw "âge invalide";
  }
  return n;
}
try {
  lireAge("abc");
} catch (e) {
  if (e instanceof Error) console.log(e.message);
  else console.log("autre chose :", e);
}
```

```text
autre chose : âge invalide
```

TypeScript accepte `throw "…"`, mais ce texte n'est pas une `Error` : il n'a ni
`message`, ni trace d'appels, et `instanceof Error` échoue. Levez toujours un objet :
`throw new Error("âge invalide");`.

## Testez-vous

**Question 1 :** qu'affiche ce code ?

```typescript norun
try {
  console.log("A");
  throw new Error("panne");
  console.log("B");
} catch (e) {
  console.log("C");
} finally {
  console.log("D");
}
```

> **Réponse**
>
> `A`, `C`, puis `D`. `B` n'est jamais affiché : après `throw`, l'exécution saute au
> `catch`. `finally` s'exécute toujours.

**Question 2 :** pourquoi l'erreur attrapée dans `catch (e)` a-t-elle le type
`unknown` ?

> **Réponse**
>
> Parce qu'en JavaScript, on peut lever n'importe quelle valeur, pas seulement des
> `Error`. TypeScript ne peut pas savoir laquelle : il vous oblige à vérifier, par
> exemple avec `e instanceof Error`.

**Question 3 :** écrivez une fonction `racine(x: number)` qui renvoie un
`Resultat<number>` : un échec si `x` est négatif, sinon la racine carrée.

> **Réponse**
>
> ```typescript
> type Resultat<T> = { ok: true; value: T } | { ok: false; error: string };
>
> function racine(x: number): Resultat<number> {
>   if (x < 0) {
>     return { ok: false, error: "nombre négatif" };
>   }
>   return { ok: true, value: Math.sqrt(x) };
> }
>
> const r1 = racine(16);
> const r2 = racine(-4);
> console.log(r1.ok ? r1.value : r1.error);
> console.log(r2.ok ? r2.value : r2.error);
> ```
>
> ```sortie
> 4
> nombre négatif
> ```

## En résumé

- `throw new Error("…")` lève une erreur ; sans `catch`, le programme s'arrête.
- `try` / `catch` attrape l'erreur, `finally` s'exécute dans tous les cas.
- L'erreur attrapée est `unknown` : rétrécissez-la avec `instanceof Error`.
- Vos erreurs héritent de `Error` ; relancez celles que vous ne savez pas traiter.
- Pour les échecs prévisibles, renvoyez `{ ok: true, value } | { ok: false, error }`.
- Une donnée venue de l'extérieur est `unknown` : validez sa forme avant de l'utiliser.

Suite : [les modules et npm](modules.md)
