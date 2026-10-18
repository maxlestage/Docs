# JavaScript : introduction et installation

Cette page présente JavaScript, explique comment installer Node.js et vous fait écrire votre premier programme.

> **Objectifs :**
>
> - vous saurez dire ce qu'est JavaScript et où il s'exécute ;
> - vous saurez installer Node.js et vérifier qu'il marche ;
> - vous saurez essayer du code dans la console du navigateur et dans Node ;
> - vous saurez écrire un fichier `.js` et le lancer avec `node` ;
> - vous saurez ce qu'est un module ES et pourquoi ce site les utilise.

## Qu'est-ce que JavaScript ?

JavaScript est **le langage du web**. Il a été créé en 1995 par Brendan Eich, en dix
jours, pour rendre les pages web vivantes : un bouton qui réagit, un menu qui s'ouvre,
un formulaire qui vérifie ce que vous tapez.

Un **langage de programmation** est une langue précise, que l'ordinateur sait suivre.
Un [programme](../debuter/glossaire.md#programme) est un texte écrit dans cette langue :
une suite d'ordres, comme une recette de cuisine.

```javascript
console.log("Bonjour !");
console.log(2 + 3);
```

```sortie
Bonjour !
5
```

`console.log(...)` affiche une valeur. Lisez-le comme « dans la console, note ceci ».
Le point-virgule `;` marque la fin d'une instruction, comme le point à la fin d'une
phrase.

> **Note :** malgré son nom, JavaScript n'a presque rien à voir avec le langage Java.
> Le nom a été choisi en 1995 pour profiter de la mode de Java.

## Où s'exécute JavaScript ?

JavaScript est un langage **interprété** : un programme appelé
[interpréteur](../debuter/glossaire.md#interpreteur) lit votre code et l'exécute
directement. Il existe deux grands endroits où cet interpréteur vit.

- **Dans le navigateur** (Firefox, Chrome, Safari…). Chaque navigateur contient un
  moteur JavaScript. C'est lui qui fait bouger les pages web.
- **Dans Node.js**, un programme qui fait tourner JavaScript **hors du navigateur** :
  dans le [terminal](../debuter/terminal.md), sur un serveur, pour des scripts.

C'est comme une même langue parlée dans deux pays : les mots sont les mêmes, mais
certaines choses n'existent que dans un pays. Le navigateur sait manipuler une page
web ; Node sait lire des fichiers sur votre disque.

Ce site utilise **Node.js** pour presque tous les exemples. La page sur
[le DOM](dom.md) montre ce qui est propre au navigateur.

## Installer Node.js

Ce site utilise **Node.js 22**. Une version plus récente marche aussi.

### Depuis le site officiel

Allez sur **nodejs.org** et téléchargez la version **LTS** (*Long Term Support*,
« support à long terme ») : c'est la version stable, conseillée à tout le monde.
Lancez l'installeur et gardez les options proposées. Cela marche sur Windows, macOS et
Linux.

### Avec nvm

`nvm` (*Node Version Manager*) installe plusieurs versions de Node et passe de l'une à
l'autre. Une fois nvm installé (voir sa page sur GitHub), tapez :

```sh
nvm install 22
nvm use 22
```

> **Astuce :** utilisez nvm si vous travaillez sur plusieurs projets qui demandent des
> versions différentes de Node. Sur Windows, l'équivalent s'appelle **nvm-windows**.

### Avec un gestionnaire de paquets

Sur Linux et macOS, vous pouvez aussi passer par le gestionnaire de paquets du système.

```sh
sudo apt install nodejs      # Debian, Ubuntu
brew install node            # macOS avec Homebrew
```

> **Attention :** les paquets de certaines distributions Linux sont anciens. Vérifiez
> la version ; si elle est inférieure à 22, préférez nodejs.org ou nvm.

## Vérifier l'installation

Ouvrez un terminal et tapez la commande suivante. Si le terminal est nouveau pour vous,
lisez d'abord [le terminal](../debuter/terminal.md).

```sh
node --version
```

Vous devez voir un numéro de version, par exemple `v22.22.0`. Si le terminal répond
« command not found » (commande introuvable), Node n'est pas installé, ou le terminal a
été ouvert avant l'installation : fermez-le et rouvrez-le.

Node arrive avec **npm** (*Node Package Manager*), l'outil qui installe des
[bibliothèques](../debuter/glossaire.md#bibliotheque). `npm --version` vérifie qu'il est
là aussi. Vous n'en aurez pas besoin avant la page sur [les modules](modules.md).

## Essayer JavaScript tout de suite

### Dans la console du navigateur

Vous avez déjà JavaScript : il est dans votre navigateur. Ouvrez n'importe quelle page,
puis appuyez sur **F12** (ou **Cmd + Option + I** sur Mac). Choisissez l'onglet
**Console**. Tapez une ligne, puis Entrée : le navigateur répond tout de suite.

```text
> 2 + 3
5
> "Bon" + "jour"
'Bonjour'
```

C'est une [console interactive](../debuter/glossaire.md#repl) : parfait pour tester une
petite idée sans créer de fichier.

### Dans Node

Tapez `node` seul dans le terminal. Vous obtenez la même console interactive, dans le
terminal cette fois.

```text
$ node
Welcome to Node.js v22.22.0.
Type ".help" for more information.
> 1 + 2
3
> "Bon" + "jour"
'Bonjour'
> .exit
```

- Le signe `>` attend votre ligne. La ligne suivante montre le résultat.
- `.exit` (ou deux fois **Ctrl + C**) quitte la console.

## Votre premier programme

Ouvrez votre [éditeur de code](../debuter/editeur.md). Créez un dossier `js`, puis un
fichier `bonjour.js` avec ce contenu :

```javascript
console.log("Bonjour, le monde !");
```

```sortie
Bonjour, le monde !
```

- `console.log` affiche une valeur, puis passe à la ligne.
- Le texte est entre guillemets `"..."`. Sans guillemets, JavaScript croirait que c'est
  un nom de variable.

Un programme peut avoir plusieurs lignes. JavaScript les exécute dans l'ordre, de haut
en bas.

```javascript
console.log("Ligne 1");
console.log("Ligne 2");
console.log(2 + 3);
console.log("2 + 3");
```

```sortie
Ligne 1
Ligne 2
5
2 + 3
```

Sans guillemets, JavaScript calcule. Entre guillemets, il affiche le texte tel quel.

## Lancer un programme

Dans le terminal, placez-vous dans le dossier du fichier (avec la commande `cd`), puis
tapez `node` suivi du nom du fichier :

```sh
cd js
node bonjour.js
```

```text
Bonjour, le monde !
```

> **À retenir :** un programme JavaScript est un fichier `.js`. Vous le lancez avec
> `node nom_du_fichier.js`.

## Les modules ES

Un **module** est un fichier de code qui peut partager des choses avec d'autres
fichiers. JavaScript a deux façons d'écrire des modules :

- les **modules ES** (*ECMAScript modules*), la façon moderne et standard, avec les
  mots `import` et `export` ;
- **CommonJS**, l'ancienne façon propre à Node, avec `require(...)`.

Ce site utilise **toujours les modules ES**. Pour que Node le sache, deux solutions :

1. nommer vos fichiers en **`.mjs`** au lieu de `.js` (`bonjour.mjs`) ;
2. ou ajouter un fichier `package.json` dans le dossier, avec ce contenu :

```json
{
  "type": "module"
}
```

Avec ce fichier, tous les `.js` du dossier sont des modules ES. Voici un programme qui
utilise `import` pour lire la version de Node :

```javascript
import { platform } from "node:process";

console.log(typeof platform);
```

```sortie
string
```

`import` va chercher `platform` dans le module `node:process`, livré avec Node.
Ne cherchez pas à tout comprendre : [les modules](modules.md) ont leur propre page.

> **Note :** Node 22 devine souvent seul qu'un `.js` est un module ES quand il voit
> `import`. Mais dire les choses clairement (`.mjs` ou `"type": "module"`) évite les
> surprises.

## Lire un message d'erreur

Quand JavaScript ne comprend pas votre code, il s'arrête et affiche une
[erreur](../debuter/glossaire.md#erreur). Ce n'est pas grave : le message vous aide.

```text
/home/alice/js/bonjour.js:1
console.log(Bonjour);
            ^

ReferenceError: Bonjour is not defined
    at Object.<anonymous> (/home/alice/js/bonjour.js:1:13)

Node.js v22.22.0
```

Lisez-le de haut en bas :

- `bonjour.js:1` : le fichier et le numéro de ligne du problème ;
- la ligne de code, avec un `^` sous l'endroit exact ;
- `ReferenceError` : le type d'erreur ; `Bonjour is not defined` : la cause
  (« `Bonjour` n'est pas défini ») ;
- les lignes `at ...` : le chemin suivi par le programme (ici `1:13` veut dire ligne 1,
  colonne 13).

Les messages sont en anglais. Quelques mots reviennent souvent : *not defined* (non
défini), *unexpected* (inattendu), *cannot* (impossible), *is not a function* (n'est
pas une fonction).

## Erreurs fréquentes

### Une majuscule à console

```javascript norun
Console.log("Bonjour");
```

```text
/home/alice/js/bonjour.js:1
Console.log("Bonjour");
^

ReferenceError: Console is not defined
    at Object.<anonymous> (/home/alice/js/bonjour.js:1:1)

Node.js v22.22.0
```

JavaScript fait la différence entre majuscules et minuscules. `Console` et `console`
sont deux noms différents, et seul le second existe. Écrivez `console.log("Bonjour");`.

### Oublier le guillemet de fin

```javascript norun
console.log("Bonjour);
```

```text
/home/alice/js/bonjour.js:1
console.log("Bonjour);
            ^^^^^^^^^^

SyntaxError: Invalid or unexpected token

Node.js v22.22.0
```

Le texte n'est jamais fermé : JavaScript ne sait plus où il s'arrête (*invalid or
unexpected token*, « morceau invalide ou inattendu »). Ajoutez le guillemet :
`console.log("Bonjour");`.

### Lancer le fichier depuis le mauvais dossier

```sh
node bonjour.js
```

```text
node:internal/modules/cjs/loader:1386
  throw err;
  ^

Error: Cannot find module '/home/alice/bonjour.js'
```

Node cherche `bonjour.js` dans le dossier où se trouve le terminal (ici `/home/alice`),
pas ailleurs. Allez dans le bon dossier avec `cd js`, ou vérifiez le nom du fichier
avec `ls`.

### Taper la commande dans la console de Node

```text
> node bonjour.js
     ^^^^^^^

Uncaught SyntaxError: Unexpected identifier 'bonjour'
```

L'invite `>` montre que vous êtes **dans** Node, qui attend du JavaScript, pas une
commande du terminal. Quittez avec `.exit`, puis tapez `node bonjour.js`.

## Testez-vous

**Question 1 :** quelle commande vérifie que Node.js est installé ?

> **Réponse**
>
> `node --version`. Elle affiche le numéro de version, par exemple `v22.22.0`.

**Question 2 :** qu'affiche ce programme ?

```javascript norun
console.log("4 * 2");
console.log(4 * 2);
```

> **Réponse**
>
> `4 * 2`, puis `8`. Entre guillemets, c'est un texte affiché tel quel. Sans
> guillemets, JavaScript calcule.

**Question 3 :** vous voulez utiliser `import` dans `jeu.js`. Que faites-vous ?

> **Réponse**
>
> Renommez le fichier en `jeu.mjs`, ou ajoutez un `package.json` qui contient
> `"type": "module"`. Lancez-le ensuite avec `node jeu.mjs` (ou `node jeu.js`).

## En résumé

- JavaScript est le langage du web : il tourne dans le navigateur et, avec Node.js,
  dans le terminal.
- Installez Node depuis nodejs.org (version LTS) ou avec nvm ; `node --version` vérifie.
- La console du navigateur (F12) et la commande `node` seule permettent d'essayer du
  code tout de suite.
- Un programme est un fichier `.js` lancé avec `node nom_du_fichier.js` ;
  `console.log` affiche une valeur.
- Ce site utilise les modules ES : fichiers `.mjs` ou `"type": "module"`.
- Un message d'erreur donne le fichier, la ligne, le type d'erreur et la cause.

Suite : [premiers pas](premiers-pas.md).
