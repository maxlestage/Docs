# Un outil en ligne de commande typé

Cette page montre comment écrire en TypeScript un petit programme qu'on lance dans le terminal, avec des arguments, des options vérifiées, un code de sortie et la lecture de l'entrée standard.

> **Objectifs :**
>
> - vous saurez lire les arguments avec `process.argv` et savoir qu'ils sont toujours du
>   texte ;
> - vous saurez décrire des options avec `util.parseArgs` et profiter des types qu'il
>   devine ;
> - vous saurez renvoyer un code de sortie (`0` pour réussi, autre chose pour échoué) ;
> - vous saurez lire l'entrée standard ;
> - vous saurez construire un compteur de mots dont chaque morceau se teste avec des
>   tableaux fixes.

Le [chapitre JavaScript](../javascript/ligne-de-commande.md) présente les mêmes outils
de Node. Ici, on insiste sur les **types** et sur une façon d'écrire le programme qui le
rend facile à tester.

## Qu'est-ce qu'un outil en ligne de commande ?

C'est un programme qu'on lance en tapant son nom dans le
[terminal](../debuter/glossaire.md#terminal), suivi de quelques mots :

```sh
node compter.ts --lignes poeme.txt
```

Pensez à un guichet. Vous tendez un **bon de commande** rempli (les arguments). Le
guichetier le lit, fait le travail, vous rend le résultat, et vous dit « c'est bon » ou
« il y a un problème » (le code de sortie).

```text
   vous ──►  node compter.ts --lignes poeme.txt
                       │
                       ▼
             ┌───────────────────┐
             │  lire les options │  --lignes = oui, fichier = poeme.txt
             │  faire le travail │  compter les mots
             └─────────┬─────────┘
                       ▼
             résultat à l'écran  +  code de sortie (0 = tout va bien)
```

> **Note :** depuis Node 22.18, `node fichier.ts` lance directement un fichier
> TypeScript, sans vérifier les types (voir l'[introduction](introduction.md)). Lancez
> `npx tsc --noEmit` pour vérifier les types avant.

> **À retenir :** un outil en ligne de commande reçoit des mots, travaille, affiche un
> résultat et renvoie un code de sortie.

## Les arguments avec process.argv

`process.argv` contient tout ce qui a été tapé. Son type est `string[]` : un tableau de
textes.

```text
   node  /home/ada/saluer.ts  Ada  3
   ───┬  ────────┬──────────  ─┬─  ┬
   case 0     case 1          case 2  case 3
   (node)     (le script)     vos arguments
```

Vos arguments commencent donc à la case `2`. On enlève les deux premières cases avec
`slice(2)`.

Pour que l'exemple donne toujours la même sortie, on écrit ici le tableau à la main,
comme si on avait tapé `node saluer.ts Ada 3` :

```typescript
const argv: string[] = ["/usr/bin/node", "/home/ada/saluer.ts", "Ada", "3"];

const args = argv.slice(2);
console.log(args);
console.log(`Bonjour, ${args[0]} !`);
console.log(typeof args[1]);
```

```sortie
[ 'Ada', '3' ]
Bonjour, Ada !
string
```

Remarquez la dernière ligne : `"3"` est un **texte**, pas un nombre. Un argument est
toujours du texte.

Dans un vrai programme, on écrit simplement :

```typescript norun
const args = process.argv.slice(2);
```

### Un argument peut manquer

TypeScript annonce `args[0]` comme un `string`. Mais si l'utilisateur ne tape rien, la
case n'existe pas et vaut `undefined`. TypeScript ne le signale pas (il ne compte pas
les cases d'un tableau) : à vous de prévoir une valeur par défaut avec `??`.

```typescript
function saluer(args: string[]): string {
  const nom = args[0] ?? "inconnu";
  return `Bonjour, ${nom} !`;
}

console.log(saluer(["Ada"]));
console.log(saluer([]));
```

```sortie
Bonjour, Ada !
Bonjour, inconnu !
```

Remarquez la forme : `saluer` reçoit le tableau **en paramètre**, au lieu de lire
`process.argv` elle-même. On peut donc l'essayer avec n'importe quel tableau. Cette
idée sert dans toute la page.

Essayez : appelez `saluer(["Ada", "Grace"])`. Seul `Ada` est salué : la case 1 est
ignorée.

### Convertir un argument en nombre

Pour obtenir un nombre, on convertit le texte avec `Number`. Si le texte n'est pas un
nombre, on obtient `NaN` (« pas un nombre »). Une petite fonction typée gère les deux
cas :

```typescript
function lireEntier(texte: string | undefined): number | null {
  if (texte === undefined) {
    return null;
  }
  const n = Number(texte);
  return Number.isInteger(n) ? n : null;
}

for (const essai of ["3", "trois", "2.5", undefined]) {
  console.log(essai, "->", lireEntier(essai));
}
```

```sortie
3 -> 3
trois -> null
2.5 -> null
undefined -> null
```

| Étape | `texte` | `Number(texte)` | Entier ? | Résultat |
|-------|---------|-----------------|----------|----------|
| 1 | `"3"` | 3 | oui | 3 |
| 2 | `"trois"` | `NaN` | non | `null` |
| 3 | `"2.5"` | 2.5 | non | `null` |
| 4 | `undefined` | (pas calculé) | | `null` |

> **À retenir :** `process.argv.slice(2)` donne vos arguments, de type `string[]`. Ce
> sont toujours des textes, et ils peuvent manquer.

## Les options avec util.parseArgs

Les vrais outils acceptent des **options** : `--lignes`, `-l`, `--max 10`… Les trier à
la main est pénible. La fonction `parseArgs` du module `node:util` le fait pour vous.
Vous lui décrivez les options attendues, comme un formulaire avec des cases à remplir :

- `type: "boolean"` : une case à cocher, présente ou absente (`--lignes`) ;
- `type: "string"` : une case à remplir, suivie d'une valeur (`--max 10`) ;
- `short: "l"` : un raccourci d'une lettre (`-l`) ;
- `default` : la valeur si l'option manque.

Normalement, `parseArgs` lit `process.argv` tout seul. Ici, on lui donne un tableau
`args` écrit à la main : on simule `node outil.ts -l --max 3 a.txt b.txt`.

```typescript
import { parseArgs } from "node:util";

const { values, positionals } = parseArgs({
  args: ["-l", "--max", "3", "a.txt", "b.txt"],
  options: {
    lignes: { type: "boolean", short: "l", default: false },
    max: { type: "string" },
  },
  allowPositionals: true,
});

console.log(values.lignes);
console.log(values.max);
console.log(positionals);
```

```sortie
true
3
[ 'a.txt', 'b.txt' ]
```

Voici comment `parseArgs` lit le tableau, case par case :

| Étape | Case lue | Ce que c'est | `values` après | `positionals` après |
|-------|----------|--------------|----------------|---------------------|
| 1 | `"-l"` | raccourci de `--lignes` | `{ lignes: true }` | `[]` |
| 2 | `"--max"` | option texte : on prend la case suivante | `{ lignes: true, max: "3" }` | `[]` |
| 3 | `"3"` | (déjà prise par `--max`) | | |
| 4 | `"a.txt"` | pas de tiret : argument | | `["a.txt"]` |
| 5 | `"b.txt"` | pas de tiret : argument | | `["a.txt", "b.txt"]` |

`allowPositionals: true` autorise les arguments sans tiret (les noms de fichiers).

### Les types devinés par TypeScript

TypeScript lit la description des options et en **déduit** le type de chaque valeur :

| Description | Type de la valeur |
|-------------|-------------------|
| `{ type: "boolean" }` | `boolean \| undefined` |
| `{ type: "boolean", default: false }` | `boolean` |
| `{ type: "string" }` | `string \| undefined` |
| `{ type: "string", default: "10" }` | `string` |

Sans `default`, l'option peut manquer : le type contient `undefined`, et TypeScript vous
oblige à traiter ce cas. Et une option `string` reste un **texte** : `values.max` n'est
jamais un `number` (voir les [erreurs fréquentes](#erreurs-frequentes)).

```typescript
import { parseArgs } from "node:util";

function lireMax(args: string[]): number {
  const { values } = parseArgs({
    args,
    options: { max: { type: "string", default: "10" } },
  });
  // values.max est un string (grâce à default) : on le convertit.
  return Number(values.max);
}

console.log(lireMax(["--max", "3"]));
console.log(lireMax(["--max=25"]));
console.log(lireMax([]));
```

```sortie
3
25
10
```

Essayez : enlevez `default: "10"`. Le programme compile encore (`Number(undefined)`
est permis), mais le dernier appel affiche `NaN`. Ajoutez `?? "10"` pour corriger.

### Une option inconnue

`parseArgs` est **strict** : une option qu'il ne connaît pas lève une erreur. C'est
une bonne chose, car une faute de frappe (`--lignse`) ne passe pas inaperçue. On
l'attrape avec `try` / `catch` (voir [gérer les erreurs](erreurs.md)) :

```typescript
import { parseArgs } from "node:util";

function essayer(args: string[]): void {
  try {
    const { values } = parseArgs({ args, options: { lignes: { type: "boolean" } } });
    console.log("OK :", values.lignes);
  } catch (e) {
    const message = e instanceof Error ? e.message : String(e);
    console.log("Refusé :", message);
  }
}

essayer(["--lignes"]);
essayer(["--lignse"]);
essayer(["poeme.txt"]);
```

```sortie
OK : true
Refusé : Unknown option '--lignse'
Refusé : Unexpected argument 'poeme.txt'. This command does not take positional arguments
```

Le troisième appel est refusé car `allowPositionals` manque : sans lui, un nom de
fichier est interdit.

> **À retenir :** `parseArgs` trie les options et TypeScript devine leurs types. Une
> option `string` reste un texte ; sans `default`, elle peut valoir `undefined`.

## Le code de sortie

En s'arrêtant, un programme renvoie au terminal un **code de sortie**, un petit
nombre :

| Code | Sens habituel |
|------|---------------|
| `0` | tout s'est bien passé |
| `1` | le travail a échoué (fichier introuvable…) |
| `2` | la commande était mal écrite (option inconnue…) |

C'est le « c'est bon » ou « il y a un problème » du guichetier. Les scripts et les
outils d'automatisation le lisent pour savoir si l'étape a réussi.

On le choisit avec `process.exitCode`. Le plus simple : une fonction `principal` qui
**renvoie** le code, et une seule ligne qui le transmet à Node.

```typescript
function principal(args: string[]): number {
  if (args.length === 0) {
    console.log("Erreur : donnez au moins un nom.");
    return 2;
  }
  for (const nom of args) {
    console.log(`Bonjour, ${nom} !`);
  }
  return 0;
}

console.log("code :", principal(["Ada", "Grace"]));
console.log("code :", principal([]));
```

```sortie
Bonjour, Ada !
Bonjour, Grace !
code : 0
Erreur : donnez au moins un nom.
code : 2
```

Dans le vrai programme, la dernière ligne devient :

```typescript norun
process.exitCode = principal(process.argv.slice(2));
```

Et dans le terminal, `echo $?` affiche le code de la dernière commande :

```sh
node saluer.ts
echo $?
```

```text
Erreur : donnez au moins un nom.
2
```

> **Note :** dans un vrai outil, écrivez les messages d'erreur avec `console.error` :
> ils partent sur la **sortie d'erreur**, séparée des résultats (voir le
> [chapitre JavaScript](../javascript/ligne-de-commande.md)). Ici, on utilise
> `console.log` pour que les exemples affichent tout.

> **Astuce :** préférez `process.exitCode = …` à `process.exit(…)`. `process.exit`
> coupe le programme net, même au milieu de l'écriture d'un fichier.

> **À retenir :** `0` veut dire réussi, tout autre code veut dire échoué. Une fonction
> `principal(args): number` rend cela facile à tester.

## Lire l'entrée standard

Un outil peut aussi recevoir du texte par un **tuyau** (`|`) : la sortie d'une commande
devient l'**entrée standard** (*stdin*) de la suivante.

```text
   cat poeme.txt  ──|──►  node compter.ts
   (affiche le texte)     (le lit sur son entrée standard)
```

La fonction `text` de `node:stream/consumers` lit toute l'entrée standard et renvoie
une [promesse](../debuter/glossaire.md#promesse) de `string`. Cet exemple ne peut pas
tourner ici, car il attend un texte venu du terminal :

```typescript norun
import { text } from "node:stream/consumers";

const entree: string = await text(process.stdin);
const mots = entree.split(/\s+/).filter((m) => m !== "");
console.log(`${mots.length} mots`);
```

```sh
printf 'un deux\ntrois\n' | node compter.ts
```

```text
3 mots
```

Comme d'habitude, gardez le calcul dans une fonction qui reçoit un `string` : vous
pourrez la tester sans tuyau, avec un texte écrit à la main. C'est ce que fait la
section suivante.

> **À retenir :** `await text(process.stdin)` lit tout ce qui arrive par le tuyau, sous
> forme d'un seul `string`.

## Un compteur de mots typé

Assemblons tout : un outil `compter` qui compte les mots d'un ou plusieurs fichiers,
et, sur demande, les lignes.

```text
   Usage : compter [-l] <fichier>...
     -l, --lignes   compte aussi les lignes
     -h, --aide     affiche cette aide
```

Le programme est découpé en petites fonctions, chacune avec un rôle :

```text
   args (string[]) ──► analyser ──► Commande ──► principal ──► code de sortie
                                                    │
                                     lire(fichier) ─┤
                                                    ▼
                                                 compter ──► Compte
```

La lecture des fichiers est **passée en paramètre** (`lire`). Dans l'exemple, on lui
donne un faux disque : un simple objet qui associe un nom de fichier à son contenu.
Dans le vrai programme, on lui donnera `readFileSync`.

```typescript
import { parseArgs } from "node:util";

type Commande =
  | { kind: "aide" }
  | { kind: "erreur"; message: string }
  | { kind: "compter"; lignes: boolean; fichiers: string[] };

interface Compte {
  mots: number;
  lignes: number;
}

const AIDE = "Usage : compter [-l] <fichier>...";

function analyser(args: string[]): Commande {
  try {
    const { values, positionals } = parseArgs({
      args,
      options: {
        lignes: { type: "boolean", short: "l", default: false },
        aide: { type: "boolean", short: "h", default: false },
      },
      allowPositionals: true,
    });
    if (values.aide) {
      return { kind: "aide" };
    }
    if (positionals.length === 0) {
      return { kind: "erreur", message: "donnez au moins un fichier" };
    }
    return { kind: "compter", lignes: values.lignes, fichiers: positionals };
  } catch (e) {
    return { kind: "erreur", message: e instanceof Error ? e.message : String(e) };
  }
}

function compter(texte: string): Compte {
  const mots = texte.split(/\s+/).filter((m) => m !== "").length;
  const lignes = texte === "" ? 0 : texte.split("\n").filter((l) => l !== "").length;
  return { mots, lignes };
}

function principal(args: string[], lire: (fichier: string) => string | null): number {
  const commande = analyser(args);
  switch (commande.kind) {
    case "aide":
      console.log(AIDE);
      return 0;
    case "erreur":
      console.log(`Erreur : ${commande.message}`);
      console.log(AIDE);
      return 2;
    case "compter": {
      let code = 0;
      for (const fichier of commande.fichiers) {
        const texte = lire(fichier);
        if (texte === null) {
          console.log(`${fichier} : introuvable`);
          code = 1;
          continue;
        }
        const { mots, lignes } = compter(texte);
        const detail = commande.lignes ? `, ${lignes} lignes` : "";
        console.log(`${fichier} : ${mots} mots${detail}`);
      }
      return code;
    }
  }
}

// Un faux disque, pour essayer sans vrais fichiers.
const disque: Record<string, string> = {
  "poeme.txt": "Le chat dort\nsur le tapis\n",
  "liste.txt": "pain lait\nœufs\n\nbeurre\n",
};
const lireFaux = (fichier: string): string | null => disque[fichier] ?? null;

const essais = [
  ["poeme.txt"],
  ["-l", "poeme.txt", "liste.txt"],
  ["absent.txt"],
  ["--mots", "poeme.txt"],
];

for (const args of essais) {
  console.log(`$ compter ${args.join(" ")}`);
  console.log(`(code ${principal(args, lireFaux)})`);
}
```

```sortie
$ compter poeme.txt
poeme.txt : 6 mots
(code 0)
$ compter -l poeme.txt liste.txt
poeme.txt : 6 mots, 2 lignes
liste.txt : 4 mots, 3 lignes
(code 0)
$ compter absent.txt
absent.txt : introuvable
(code 1)
$ compter --mots poeme.txt
Erreur : Unknown option '--mots'. To specify a positional argument starting with a '-', place it at the end of the command after '--', as in '-- "--mots"
Usage : compter [-l] <fichier>...
(code 2)
```

Avec `allowPositionals: true`, Node ajoute un conseil au message : si vous voulez
vraiment un fichier nommé `--mots`, tapez-le après `--`.

### Suivre compter pas à pas

Prenons le texte de `liste.txt` : `"pain lait\nœufs\n\nbeurre\n"` (`\n` est un retour à
la ligne).

| Étape | Calcul | Valeur |
|-------|--------|--------|
| 1 | `texte.split(/\s+/)` : on coupe à chaque groupe d'espaces ou de retours | `["pain", "lait", "œufs", "beurre", ""]` |
| 2 | `.filter((m) => m !== "")` : on enlève les morceaux vides | `["pain", "lait", "œufs", "beurre"]` |
| 3 | `.length` | `mots` = 4 |
| 4 | `texte.split("\n")` : on coupe à chaque retour à la ligne | `["pain lait", "œufs", "", "beurre", ""]` |
| 5 | `.filter((l) => l !== "")` : on enlève les lignes vides | `["pain lait", "œufs", "beurre"]` |
| 6 | `.length` | `lignes` = 3 |

### Ce que les types apportent

- `Commande` est une [union discriminée](unions.md#les-unions-discriminees) : dans le
  `case "compter"`, TypeScript sait que `commande.fichiers` existe ; dans le
  `case "erreur"`, que `commande.message` existe.
- `lire` renvoie `string | null` : TypeScript oblige `principal` à traiter le fichier
  introuvable avant d'appeler `compter`.
- `principal` renvoie `number` dans **tous** les cas : si vous oubliez un `return`,
  TypeScript le signale.

Essayez : ajoutez `"vide.txt": ""` au faux disque et l'essai `["-l", "vide.txt"]`. Vous
obtenez `vide.txt : 0 mots, 0 lignes` et le code 0.

### Le brancher sur le vrai monde

Il ne reste qu'à remplacer le faux disque par de vrais fichiers. Le fichier `compter.ts`
contient les types et les fonctions ci-dessus, puis ces lignes à la place des essais :

```typescript norun
import { readFileSync } from "node:fs";

function lireFichier(fichier: string): string | null {
  try {
    return readFileSync(fichier, "utf8");
  } catch {
    return null;
  }
}

process.exitCode = principal(process.argv.slice(2), lireFichier);
```

```sh
node compter.ts -l poeme.txt
echo $?
```

```text
poeme.txt : 6 mots, 2 lignes
0
```

Seules ces quelques lignes touchent au vrai monde (`process.argv`, le disque,
`process.exitCode`). Tout le reste se teste avec des tableaux fixes, comme plus haut.
C'est ce qui rend l'outil facile à [tester](tests.md).

> **À retenir :** séparez le « cœur » (des fonctions qui reçoivent des tableaux et des
> textes) des quelques lignes qui lisent `process.argv` et le disque.

## Erreurs fréquentes

### Utiliser une option texte comme un nombre

```typescript norun
import { parseArgs } from "node:util";

const { values } = parseArgs({ options: { max: { type: "string" } } });
const max: number = values.max;
```

```text
main.ts(4,7): error TS2322: Type 'string | undefined' is not assignable to type 'number'.
  Type 'undefined' is not assignable to type 'number'.
```

Une option `string` donne un texte, qui peut manquer. Donnez une valeur par défaut et
convertissez : `const max = Number(values.max ?? "10");`.

### Une option inconnue non attrapée

```typescript norun
import { parseArgs } from "node:util";

const { values } = parseArgs({ options: { mots: { type: "boolean" } } });
console.log(values);
```

```sh
node compter.ts --mts
```

```text
TypeError [ERR_PARSE_ARGS_UNKNOWN_OPTION]: Unknown option '--mts'
```

TypeScript ne peut rien voir : l'erreur dépend de ce que l'utilisateur tape. Entourez
`parseArgs` d'un `try` / `catch`, affichez l'aide et renvoyez le code 2, comme dans
`analyser`.

### Oublier allowPositionals

```typescript norun
import { parseArgs } from "node:util";

const { values } = parseArgs({ options: { mots: { type: "boolean" } } });
console.log(values);
```

```sh
node compter.ts texte.txt
```

```text
TypeError [ERR_PARSE_ARGS_UNEXPECTED_POSITIONAL]: Unexpected argument 'texte.txt'. This command does not take positional arguments
```

Par défaut, `parseArgs` refuse les arguments sans tiret. Ajoutez
`allowPositionals: true` et lisez-les dans `positionals`.

### Lire la mauvaise case de process.argv

```typescript norun
const nom = process.argv[0];
console.log(`Bonjour, ${nom} !`);
```

```sh
node saluer.ts Ada
```

```text
Bonjour, /usr/bin/node !
```

Aucune erreur, mais un résultat faux : la case 0 est le chemin de `node`. Utilisez
`process.argv.slice(2)`, puis la case 0 de ce nouveau tableau.

## Testez-vous

**Question 1 :** avec la commande `node outil.ts 5 chats`, que vaut
`process.argv.slice(2)`, et quel est le type de sa première case ?

> **Réponse**
>
> `["5", "chats"]`. La première case est le texte `"5"`, de type `string` : il faut
> la convertir avec `Number` pour calculer.

**Question 2 :** quel est le type de `values.verbeux` avec
`options: { verbeux: { type: "boolean" } }` ? Et avec `default: false` ?

> **Réponse**
>
> Sans `default` : `boolean | undefined`, car l'option peut manquer. Avec
> `default: false` : `boolean`.

**Question 3 :** écrivez `principal(args: string[]): number` qui affiche la somme des
nombres reçus et renvoie 0, ou affiche `Erreur : X n'est pas un nombre` et renvoie 1.
Essayez-la avec `["2", "3"]` puis `["2", "x"]`.

> **Réponse**
>
> ```typescript
> function principal(args: string[]): number {
>   let somme = 0;
>   for (const a of args) {
>     const n = Number(a);
>     if (Number.isNaN(n)) {
>       console.log(`Erreur : ${a} n'est pas un nombre`);
>       return 1;
>     }
>     somme += n;
>   }
>   console.log(somme);
>   return 0;
> }
>
> console.log("code :", principal(["2", "3"]));
> console.log("code :", principal(["2", "x"]));
> ```
>
> ```sortie
> 5
> code : 0
> Erreur : x n'est pas un nombre
> code : 1
> ```

## En résumé

- `process.argv.slice(2)` donne vos arguments : un `string[]`, toujours du texte, dont
  les cases peuvent manquer.
- `parseArgs` de `node:util` trie les options ; TypeScript devine leurs types
  (`boolean`, `string`, avec `| undefined` sans `default`).
- `parseArgs` lève une erreur pour une option inconnue : attrapez-la et renvoyez le
  code 2.
- Le code de sortie vaut `0` si tout va bien ; choisissez-le avec
  `process.exitCode = principal(…)`.
- `await text(process.stdin)` lit l'entrée standard.
- Écrivez le cœur de l'outil avec des fonctions qui reçoivent des tableaux : vous les
  essayez avec des valeurs fixes.

Suite : [TypeScript pour le web](web.md).
