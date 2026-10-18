# La programmation asynchrone

Cette page explique comment TypeScript décrit les opérations qui prennent du temps avec le type `Promise<T>`, et comment les attendre proprement avec `async` et `await`.

> **Objectifs :**
>
> - comprendre ce qu'est une promesse et lire le type `Promise<T>` ;
> - écrire une fonction `async` et attendre son résultat avec `await` ;
> - lancer plusieurs tâches en même temps avec `Promise.all` ;
> - attraper les erreurs d'une promesse avec `try` et `catch` ;
> - transformer `setTimeout` en promesse pour faire une pause.

## Pourquoi attendre ?

Certaines opérations ne répondent pas tout de suite : lire un fichier, interroger un
[serveur](../debuter/glossaire.md#serveur), attendre une seconde. Si le programme
restait bloqué pendant ce temps, il ne pourrait rien faire d'autre.

JavaScript, et donc TypeScript, fonctionne autrement : il **lance** l'opération, continue
son travail, et revient au résultat quand il est prêt. On dit que le code est
**asynchrone** (voir [le glossaire](../debuter/glossaire.md#async)).

C'est comme au restaurant : vous commandez, le serveur vous donne un ticket, et vous
discutez en attendant. Le ticket, c'est la **promesse** d'un plat à venir.

## Le type Promise<T>

Une **promesse** est un objet qui contiendra plus tard une valeur. Son type s'écrit
`Promise<T>`, où `T` est le type de la valeur promise :

| Type | Se lit |
|------|--------|
| `Promise<number>` | « une promesse de nombre » |
| `Promise<string>` | « une promesse de texte » |
| `Promise<void>` | « une promesse qui ne donne aucune valeur, seulement un signal de fin » |

`Promise.resolve(valeur)` crée une promesse déjà tenue. Pour lire sa valeur, on écrit
`await` devant :

```typescript
const promesse: Promise<number> = Promise.resolve(42);
const valeur: number = await promesse;
console.log(valeur);
```

```sortie
42
```

`await` veut dire « attends que la promesse soit tenue, puis donne-moi sa valeur ». Le
type change : `promesse` est un `Promise<number>`, `valeur` est un simple `number`.

> **À retenir :** `await` « déballe » une promesse : `await` sur un `Promise<T>` donne un `T`.

## Les fonctions async

Le mot `async` devant une fonction la rend asynchrone. Une fonction `async` renvoie
**toujours** une promesse, même si vous écrivez `return 42`. TypeScript le sait : le
type de retour s'écrit `Promise<number>`, pas `number`.

```typescript
async function prixTTC(prixHT: number): Promise<number> {
  return prixHT * 1.2;
}

const prix = await prixTTC(50);
console.log(`Prix TTC : ${prix} €`);
```

```sortie
Prix TTC : 60 €
```

Dans une fonction `async`, vous pouvez utiliser `await` pour attendre d'autres
promesses. Le code se lit alors de haut en bas, comme du code ordinaire :

```typescript
async function chargerNom(id: number): Promise<string> {
  return id === 1 ? "Ada" : "Inconnu";
}

async function saluer(id: number): Promise<string> {
  const nom = await chargerNom(id);
  return `Bonjour ${nom} !`;
}

console.log(await saluer(1));
console.log(await saluer(7));
```

```sortie
Bonjour Ada !
Bonjour Inconnu !
```

> **Note :** dans ce site, les exemples sont des modules : `await` fonctionne directement
> au niveau du fichier (« top-level await »). Ailleurs, `await` n'est permis que dans une
> fonction `async`.

### Une fonction fléchée async

Le mot `async` se place aussi devant une fonction fléchée :

```typescript
const carre = async (n: number): Promise<number> => n * n;

console.log(await carre(9));
```

```sortie
81
```

## Faire une pause avec setTimeout

`setTimeout(fonction, ms)` appelle une fonction après un délai en millisecondes. Il
n'utilise pas de promesse, mais on peut l'**envelopper** dans une : `new Promise` reçoit
une fonction qui appelle `resolve` quand le travail est fini.

```typescript
function attendre(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

console.log("Je commande.");
await attendre(100);
console.log("Le plat arrive.");
```

```sortie
Je commande.
Le plat arrive.
```

Le programme s'arrête 100 millisecondes entre les deux lignes. Le type `Promise<void>`
dit que cette promesse ne donne pas de valeur : elle signale juste « c'est fini ».

> **Astuce :** Node fournit aussi une version toute prête :
> `import { setTimeout as attendre } from "node:timers/promises";`.

### L'ordre d'exécution

Sans `await`, le programme n'attend pas : il continue à la ligne suivante. Regardez
l'ordre des messages :

```typescript
function attendre(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function tache(nom: string, ms: number): Promise<void> {
  await attendre(ms);
  console.log(`${nom} terminée`);
}

const lente = tache("Tâche lente", 60);
const rapide = tache("Tâche rapide", 20);
console.log("Les deux tâches sont lancées");
await lente;
await rapide;
console.log("Fin");
```

```sortie
Les deux tâches sont lancées
Tâche rapide terminée
Tâche lente terminée
Fin
```

Les deux tâches tournent en même temps. La rapide finit la première, même si elle a
été lancée en second.

## Promise.all et les tuples

`Promise.all` attend **plusieurs** promesses à la fois et renvoie un tableau de leurs
résultats, dans l'ordre où vous les avez données (pas dans l'ordre où elles finissent).

Si les promesses ont des types différents, TypeScript donne un
[tuple](../debuter/glossaire.md#tuple) : un tableau dont chaque case a son propre type.
Ici, le résultat est de type `[string, number]`.

```typescript
function attendre(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function chargerNom(): Promise<string> {
  await attendre(50);
  return "Ada";
}

async function chargerAge(): Promise<number> {
  await attendre(10);
  return 36;
}

const [nom, age] = await Promise.all([chargerNom(), chargerAge()]);
console.log(`${nom} a ${age} ans`);
console.log(nom.toUpperCase(), age + 1);
```

```sortie
Ada a 36 ans
ADA 37
```

Grâce au tuple, TypeScript sait que `nom` est un `string` et `age` un `number` : vous
pouvez appeler `toUpperCase()` sur l'un et faire un calcul avec l'autre.

### Une liste de promesses

Avec `map`, vous créez une promesse par élément, puis `Promise.all` les attend toutes.
Le résultat est un `number[]` :

```typescript
async function longueur(mot: string): Promise<number> {
  return mot.length;
}

const mots = ["chat", "éléphant", "ours"];
const longueurs: number[] = await Promise.all(mots.map(longueur));
console.log(longueurs);
```

```sortie
[ 4, 8, 4 ]
```

> **À retenir :** lancez d'abord toutes les promesses, puis attendez-les ensemble avec
> `Promise.all`. C'est plus rapide que de les attendre une par une.

## Gérer les erreurs

Une promesse peut **échouer** : on dit qu'elle est **rejetée**. Avec `await`, un rejet
devient une [exception](../debuter/glossaire.md#exception) ordinaire, que vous attrapez
avec `try` et `catch`.

En mode strict, la variable du `catch` est de type `unknown` : TypeScript ne sait pas
ce qui a été lancé. Vérifiez avec `instanceof Error` avant de lire `.message`.

```typescript
async function diviser(a: number, b: number): Promise<number> {
  if (b === 0) {
    throw new Error("division par zéro");
  }
  return a / b;
}

try {
  console.log(await diviser(10, 2));
  console.log(await diviser(1, 0));
  console.log("Jamais affiché");
} catch (e) {
  if (e instanceof Error) {
    console.log(`Erreur : ${e.message}`);
  }
}
```

```sortie
5
Erreur : division par zéro
```

Si une seule promesse de `Promise.all` échoue, tout le `Promise.all` échoue.

### Promise.allSettled : garder tous les résultats

`Promise.allSettled` attend toutes les promesses, réussies ou non. Chaque résultat a un
champ `status` qui vaut `"fulfilled"` (tenue) ou `"rejected"` (rejetée). TypeScript
utilise ce champ pour savoir si vous pouvez lire `value` ou `reason` :

```typescript
async function verifier(age: number): Promise<number> {
  if (age < 0) {
    throw new Error(`âge invalide : ${age}`);
  }
  return age;
}

const resultats = await Promise.allSettled([verifier(12), verifier(-3), verifier(40)]);
for (const r of resultats) {
  if (r.status === "fulfilled") {
    console.log("OK", r.value);
  } else {
    console.log("Échec", String(r.reason));
  }
}
```

```sortie
OK 12
Échec Error: âge invalide : -3
OK 40
```

## Erreurs fréquentes

### Oublier await

```typescript norun
async function lireNombre(): Promise<number> {
  return 42;
}

const n: number = lireNombre();
console.log(n + 1);
```

```text
main.ts(5,7): error TS2322: Type 'Promise<number>' is not assignable to type 'number'.
```

`lireNombre()` renvoie une promesse, pas un nombre. Ajoutez `await` :
`const n: number = await lireNombre();`.

### Écrire le mauvais type de retour

```typescript norun
async function lireNombre(): number {
  return 42;
}
```

```text
main.ts(1,30): error TS1064: The return type of an async function or method must be the global Promise<T> type. Did you mean to write 'Promise<number>'?
```

Une fonction `async` renvoie toujours une promesse. Écrivez `Promise<number>`.

### Utiliser await hors d'une fonction async

```typescript norun
function attendre(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

function demarrer() {
  await attendre(100);
  console.log("Prêt");
}
```

```text
main.ts(6,3): error TS1308: 'await' expressions are only allowed within async functions and at the top levels of modules.
```

Ajoutez `async` devant la fonction : `async function demarrer() { … }`.

### Lire e.message sans vérifier

```typescript norun
try {
  await Promise.reject(new Error("panne"));
} catch (e) {
  console.log(e.message);
}
```

```text
main.ts(4,15): error TS18046: 'e' is of type 'unknown'.
```

N'importe quoi peut être lancé, pas seulement une `Error`. Vérifiez d'abord :
`if (e instanceof Error) { console.log(e.message); }`.

## Testez-vous

**Question 1 :** quel est le type de retour de `async function f() { return "ok"; }` ?

> **Réponse**
>
> `Promise<string>`. Une fonction `async` enveloppe toujours sa valeur de retour dans
> une promesse.

**Question 2 :** avec `const [a, b] = await Promise.all([Promise.resolve(1), Promise.resolve("x")])`,
quels sont les types de `a` et `b` ?

> **Réponse**
>
> `a` est un `number` et `b` un `string`. `Promise.all` donne un tuple
> `[number, string]`, dans l'ordre des promesses.

**Question 3 :** écrivez une fonction `attendre(ms)` qui renvoie une promesse tenue après
`ms` millisecondes, puis utilisez-la pour afficher « Un », attendre, et afficher « Deux ».

> **Réponse**
>
> ```typescript
> function attendre(ms: number): Promise<void> {
>   return new Promise((resolve) => setTimeout(resolve, ms));
> }
>
> console.log("Un");
> await attendre(50);
> console.log("Deux");
> ```
>
> ```sortie
> Un
> Deux
> ```

## En résumé

- Une promesse `Promise<T>` contiendra plus tard une valeur de type `T`.
- Une fonction `async` renvoie toujours une promesse ; `await` en extrait la valeur.
- `Promise.all` attend plusieurs promesses et donne un tuple bien typé.
- Une promesse rejetée devient une exception : attrapez-la avec `try` et `catch`, en
  vérifiant `e instanceof Error`.
- `new Promise((resolve) => setTimeout(resolve, ms))` crée une pause.

Suite : [configurer TypeScript](configuration.md). Comparez avec
[la concurrence en Ruby](../ruby/concurrence.md).
