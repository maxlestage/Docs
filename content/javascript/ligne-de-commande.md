# Écrire un outil en ligne de commande

Cette page montre comment écrire un petit programme Node que l'on lance dans le terminal, avec des arguments, des options et un code de sortie.

> **Objectifs :**
>
> - vous saurez lire les arguments du programme avec `process.argv` ;
> - vous saurez lire des options comme `--lignes` avec `util.parseArgs` ;
> - vous saurez signaler une erreur avec `console.error` et `process.exitCode` ;
> - vous saurez rendre un script exécutable avec un shebang et `chmod` ;
> - vous saurez écrire un outil complet qui compte les mots d'un fichier.

## Qu'est-ce qu'un outil en ligne de commande ?

Un outil en ligne de commande est un programme que l'on lance dans le
[terminal](../debuter/glossaire.md#terminal), en tapant son nom suivi de quelques mots :

```sh
node compter.mjs --lignes poeme.txt
```

Ce qui suit le nom du programme s'appelle les **arguments**. Ceux qui commencent par
`-` ou `--` sont des **options** : des réglages. Les autres sont souvent des noms de
fichiers. `ls -l`, `git commit -m "…"` ou `node --version` fonctionnent tous ainsi
(voir [le terminal](../debuter/terminal.md)).

## Les arguments avec process.argv

`process.argv` (*argument vector*, « liste des arguments ») est un tableau de textes :

- la case `0` est le chemin de `node` ;
- la case `1` est le chemin du script ;
- **vos** arguments commencent à la case `2`.

```javascript norun
// saluer.mjs
console.log(process.argv);
```

```sh
node saluer.mjs Alice
```

```text
[
  '/usr/local/bin/node',
  '/home/alice/saluer.mjs',
  'Alice'
]
```

On enlève presque toujours les deux premières cases avec `slice(2)`. Sans argument, il
reste un tableau vide : prévoyez une valeur par défaut.

```javascript
const args = process.argv.slice(2);
const nom = args[0] ?? "inconnu";

console.log(args);
console.log(`Bonjour, ${nom} !`);
```

```sortie
[]
Bonjour, inconnu !
```

> **Note :** comme une saisie au clavier, un argument est toujours du **texte**.
> Convertissez avec `Number(args[0])` pour obtenir un nombre.

## Les options avec util.parseArgs

Trier les options à la main devient vite pénible : `--lignes`, `-l`, `--max 10`,
`--max=10`… La fonction `parseArgs` du module `node:util` le fait pour vous. Vous lui
décrivez les options attendues :

- `type: "boolean"` : un interrupteur, présent ou absent (`--lignes`) ;
- `type: "string"` : une option suivie d'une valeur (`--max 10`) ;
- `short: "l"` : un raccourci d'une lettre (`-l`) ;
- `default` : la valeur si l'option manque.

Normalement, `parseArgs` lit `process.argv` tout seul. Ici, on lui donne un tableau
`args` écrit à la main : on simule la commande `node outil.mjs -l --max 3 a.txt b.txt`.

```javascript
import { parseArgs } from "node:util";

const { values, positionals } = parseArgs({
  args: ["-l", "--max", "3", "a.txt", "b.txt"],
  options: {
    lignes: { type: "boolean", short: "l", default: false },
    max: { type: "string", default: "10" },
    aide: { type: "boolean", short: "h", default: false },
  },
  allowPositionals: true,
});

console.log(values);
console.log(positionals);
console.log(Number(values.max) + 1);
```

```sortie
[Object: null prototype] { lignes: true, max: '3', aide: false }
[ 'a.txt', 'b.txt' ]
4
```

- `values` contient les options ;
- `positionals` contient les autres arguments, dans l'ordre ;
- `allowPositionals: true` autorise ces arguments sans `-`.

> **Note :** `[Object: null prototype]` signale un objet « nu », sans les méthodes
> habituelles. Il s'utilise comme un objet normal : `values.lignes`.

Dans votre vrai programme, retirez simplement la ligne `args: […]`.

## Afficher une aide

Un bon outil explique comment s'en servir avec `--aide` (ou `-h`). Le texte d'aide
s'écrit facilement avec un gabarit sur plusieurs lignes.

```javascript
const AIDE = `Usage : compter [options] <fichier>

Options :
  -l, --lignes   compte aussi les lignes
  -h, --aide     affiche cette aide`;

console.log(AIDE);
```

```sortie
Usage : compter [options] <fichier>

Options :
  -l, --lignes   compte aussi les lignes
  -h, --aide     affiche cette aide
```

## La sortie d'erreur avec console.error

Un programme a deux sorties :

- la **sortie standard** (*stdout*), pour les résultats : `console.log` ;
- la **sortie d'erreur** (*stderr*), pour les problèmes : `console.error`.

Les deux s'affichent dans le terminal. Mais si l'on redirige les résultats vers un
fichier (`node outil.mjs > resultat.txt`), seuls les résultats y vont : les messages
d'erreur restent à l'écran. C'est comme séparer le courrier et les alertes.

```javascript
console.log("résultat : 42");
console.error("attention : fichier vide");
console.log("fin");
```

```sortie
résultat : 42
fin
```

La sortie ci-dessus ne montre que la sortie standard : le message d'erreur est parti
sur l'autre sortie.

## Le code de sortie avec process.exitCode

En s'arrêtant, un programme renvoie un **code de sortie** au terminal : `0` veut dire
« tout va bien », un autre nombre (souvent `1`) veut dire « échec ». Les scripts et les
outils d'intégration continue s'en servent pour savoir si une étape a réussi.

`process.exitCode = 1` choisit le code, et laisse le programme finir normalement.

```javascript norun
// verifier.mjs
const fichier = process.argv[2];

if (fichier === undefined) {
  console.error("Erreur : donnez un nom de fichier.");
  process.exitCode = 1;
} else {
  console.log(`Vérification de ${fichier}…`);
}
```

```sh
node verifier.mjs
echo $?
```

```text
Erreur : donnez un nom de fichier.
1
```

`echo $?` affiche le code de sortie de la dernière commande.

> **Astuce :** préférez `process.exitCode = 1` à `process.exit(1)`. `process.exit`
> coupe le programme tout de suite, même si un fichier est en cours d'écriture.

## Lancer le script directement

### Le shebang

Pour taper `./compter.mjs` au lieu de `node compter.mjs`, ajoutez en **toute première
ligne** un *shebang* : il indique au système quel programme doit lire le fichier.

```javascript norun
#!/usr/bin/env node
console.log("Je suis lancé sans taper node !");
```

`/usr/bin/env node` veut dire : « cherche `node` là où il est installé ».

### chmod

Il faut aussi donner au fichier le droit d'être **exécuté**, avec `chmod +x` (*change
mode*, « changer les droits ») :

```sh
chmod +x compter.mjs
./compter.mjs poeme.txt
```

> **Note :** sous Windows, le shebang et `chmod` ne servent pas : lancez
> `node compter.mjs`. Le champ `bin` (plus bas) règle ce problème pour tout le monde.

## Un outil complet : compter les mots

Voici un outil qui compte les mots (et, en option, les lignes et les caractères) d'un
fichier. Pour que l'exemple tourne tout seul, il crée d'abord son fichier, puis
simule les arguments avec `args`.

```javascript
#!/usr/bin/env node
import { parseArgs } from "node:util";
import { readFile, writeFile } from "node:fs/promises";

const AIDE = `Usage : compter [options] <fichier>
  -l, --lignes      compte aussi les lignes
  -c, --caracteres  compte aussi les caractères
  -h, --aide        affiche cette aide`;

// Logique pure : facile à tester.
function compter(texte) {
  const mots = texte.split(/\s+/).filter((m) => m !== "");
  const lignes = texte.split("\n").filter((l) => l !== "");
  return { mots: mots.length, lignes: lignes.length, caracteres: [...texte].length };
}

async function main(args) {
  const { values, positionals } = parseArgs({
    args,
    options: {
      lignes: { type: "boolean", short: "l", default: false },
      caracteres: { type: "boolean", short: "c", default: false },
      aide: { type: "boolean", short: "h", default: false },
    },
    allowPositionals: true,
  });

  if (values.aide) {
    console.log(AIDE);
    return 0;
  }
  if (positionals.length !== 1) {
    console.error("Erreur : donnez exactement un fichier.");
    console.error(AIDE);
    return 1;
  }

  const fichier = positionals[0];
  let texte;
  try {
    texte = await readFile(fichier, "utf8");
  } catch (erreur) {
    if (erreur.code !== "ENOENT") throw erreur;
    console.error(`Erreur : le fichier ${fichier} n'existe pas.`);
    return 1;
  }

  const total = compter(texte);
  console.log(`${total.mots} mots`);
  if (values.lignes) console.log(`${total.lignes} lignes`);
  if (values.caracteres) console.log(`${total.caracteres} caractères`);
  return 0;
}

// Pour l'exemple : on crée le fichier et on simule deux commandes.
await writeFile("poeme.txt", "Demain, dès l'aube,\nà l'heure où blanchit la campagne,\nJe partirai.\n");

console.log("$ compter -l poeme.txt");
await main(["-l", "poeme.txt"]);
console.log("$ compter --caracteres absent.txt");
const code = await main(["--caracteres", "absent.txt"]);
console.log(`(code de sortie : ${code})`);

// Dans le vrai outil, on écrirait seulement :
// process.exitCode = await main(process.argv.slice(2));
```

```sortie
$ compter -l poeme.txt
11 mots
3 lignes
$ compter --caracteres absent.txt
(code de sortie : 1)
```

Remarquez la structure :

- `compter` ne fait que des calculs : on peut la tester sans fichier ;
- `main` reçoit les arguments en paramètre, au lieu de lire `process.argv` : on peut
  l'essayer avec n'importe quelle liste ;
- `main` **renvoie** le code de sortie ; seule la dernière ligne touche à `process`.

Le message « le fichier absent.txt n'existe pas » est bien affiché, mais sur la sortie
d'erreur : il n'apparaît donc pas dans la sortie ci-dessus.

## Installer l'outil avec package.json

Pour taper `compter` depuis n'importe quel dossier, déclarez l'outil dans le
`package.json` du projet (voir [les modules](modules.md)), avec le champ `bin` :

```text
{
  "name": "compter",
  "version": "1.0.0",
  "type": "module",
  "bin": {
    "compter": "./compter.mjs"
  }
}
```

Puis, dans le dossier du projet :

```sh
npm link
compter -l poeme.txt
```

`npm link` installe la commande sur votre ordinateur. Si vous publiez le
[paquet](../debuter/glossaire.md#paquet) sur npm, d'autres pourront l'installer avec
`npm install -g compter`, ou le lancer sans l'installer avec `npx compter`.

## Erreurs fréquentes

### Oublier allowPositionals

```javascript norun
import { parseArgs } from "node:util";

parseArgs({
  args: ["--lignes", "a.txt"],
  options: { lignes: { type: "boolean" } },
});
```

```text
TypeError [ERR_PARSE_ARGS_UNEXPECTED_POSITIONAL]: Unexpected argument 'a.txt'. This command does not take positional arguments
```

Par défaut, `parseArgs` refuse les arguments sans `-`. Ajoutez
`allowPositionals: true`.

### Une option inconnue

```javascript norun
import { parseArgs } from "node:util";

parseArgs({
  args: ["--verbeux"],
  options: { lignes: { type: "boolean" } },
});
```

```text
TypeError [ERR_PARSE_ARGS_UNKNOWN_OPTION]: Unknown option '--verbeux'
```

L'utilisateur a tapé une option que vous n'avez pas déclarée (ou il y a une faute de
frappe). C'est voulu : `parseArgs` protège des fautes. Pour un message plus aimable,
entourez l'appel d'un `try...catch` et affichez l'aide avec `console.error`.

### Oublier chmod

```sh
./compter.mjs poeme.txt
```

```text
bash: ./compter.mjs: Permission denied
```

Le fichier n'a pas le droit d'être exécuté. Lancez `chmod +x compter.mjs` une fois, ou
tapez `node compter.mjs`.

### Lire le mauvais indice de process.argv

```javascript norun
// saluer.mjs
console.log(`Bonjour, ${process.argv[0]} !`);
```

```text
Bonjour, /usr/local/bin/node !
```

La case `0` est le chemin de `node`, pas votre premier argument. Utilisez
`process.argv[2]`, ou mieux `process.argv.slice(2)`.

## Testez-vous

**Question 1 :** pour `node outil.mjs a b`, que vaut `process.argv.slice(2)` ?

> **Réponse**
>
> `[ 'a', 'b' ]`.

**Question 2 :** quelle différence entre `console.log` et `console.error` ?

> **Réponse**
>
> `console.log` écrit sur la sortie standard (les résultats), `console.error` sur la
> sortie d'erreur (les problèmes). Une redirection `>` n'emporte que la sortie
> standard.

**Question 3 :** avec `parseArgs`, déclarez une option `--nom` (raccourci `-n`) qui prend
une valeur, puis lisez `["-n", "Alice"]`.

> **Réponse**
>
> ```javascript
> import { parseArgs } from "node:util";
>
> const { values } = parseArgs({
>   args: ["-n", "Alice"],
>   options: { nom: { type: "string", short: "n" } },
> });
> console.log(`Bonjour, ${values.nom} !`);
> ```
>
> ```sortie
> Bonjour, Alice !
> ```

## En résumé

- `process.argv.slice(2)` donne les arguments du programme, toujours sous forme de texte.
- `parseArgs` de `node:util` lit les options ; `allowPositionals: true` garde les autres arguments.
- `console.error` écrit les erreurs à part ; `process.exitCode = 1` signale un échec.
- Un shebang `#!/usr/bin/env node` et `chmod +x` rendent le script exécutable.
- Le champ `bin` de `package.json` transforme le script en vraie commande.

Suite : [JavaScript pour le web](web.md)
