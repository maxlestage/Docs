# Gérer les erreurs

Cette page explique comment lire un message d'erreur de Node, comment attraper une erreur avec `try` et `catch`, et comment lever vos propres erreurs avec `throw`.

> **Objectifs :**
>
> - vous saurez lire un message d'erreur et sa trace d'appels ;
> - vous saurez attraper une erreur avec `try`, `catch` et `finally` ;
> - vous saurez lever une erreur avec `throw new Error(...)` ;
> - vous saurez créer vos propres classes d'erreur et chaîner une cause ;
> - vous saurez vérifier les données reçues avant de les utiliser.

## Une erreur arrête le programme

Quand JavaScript rencontre un problème qu'il ne sait pas régler, il **lève** une
[exception](../debuter/glossaire.md#exception) : un objet qui décrit
l'[erreur](../debuter/glossaire.md#erreur). « Lever » veut dire : déclencher l'alarme.
Si personne ne s'en occupe, le programme s'arrête net.

Voici un programme qui calcule la moyenne d'un élève… dont on a oublié les notes.

```javascript norun
function moyenne(notes) {
  const total = notes.reduce((a, b) => a + b, 0);
  return total / notes.length;
}

function bulletin(eleve) {
  return `${eleve.nom} : ${moyenne(eleve.notes)}`;
}

console.log(bulletin({ nom: "Léa" }));
```

```text
file:///home/vous/projet/main.mjs:2
  const total = notes.reduce((a, b) => a + b, 0);
                      ^

TypeError: Cannot read properties of undefined (reading 'reduce')
    at moyenne (file:///home/vous/projet/main.mjs:2:23)
    at bulletin (file:///home/vous/projet/main.mjs:7:28)
    at file:///home/vous/projet/main.mjs:10:13
    at ModuleJob.run (node:internal/modules/esm/module_job:343:25)
    at async onImport.tracePromise.__proto__ (node:internal/modules/esm/loader:665:26)
    at async asyncRunEntryPointWithESMLoader (node:internal/modules/run_main:117:5)

Node.js v22.22.0
```

### Lire le message

Ce bloc de texte fait peur, mais il se lit en quatre morceaux :

1. **Où ?** La première ligne donne le fichier et la ligne : `main.mjs:2`. Node recopie
   la ligne fautive et place un `^` sous l'endroit exact.
2. **Quoi ?** `TypeError` est le **type** de l'erreur, son nom de famille.
3. **Pourquoi ?** Le message : « impossible de lire les propriétés de `undefined` (en
   lisant `reduce`) ». Autrement dit : `notes` vaut `undefined`, et `undefined` n'a pas
   de méthode `reduce`.
4. **Comment en est-on arrivé là ?** Les lignes `at ...` forment la **trace d'appels**
   (en anglais *stack trace*) : le chemin suivi par le programme, de la fonction la plus
   profonde jusqu'au programme principal.

La trace se lit de haut en bas : l'erreur est dans `moyenne` (ligne 2), appelée par
`bulletin` (ligne 7), appelée par le programme (ligne 10). Les lignes `node:internal`
viennent de Node lui-même : ignorez-les et cherchez **vos** fichiers.

Ici, le vrai coupable est la ligne 10 : l'élève n'a pas de propriété `notes`.

## try et catch

`try` (« essayer ») et `catch` (« attraper ») forment un filet de sécurité, comme sous
un trapéziste : si une erreur tombe, elle est rattrapée, et le spectacle continue.

- Le code risqué va dans le bloc `try`.
- Le code de secours va dans le bloc `catch`. Il reçoit l'erreur en paramètre.

```javascript
const textes = ['{"nom": "Léa"}', "{nom: Léa}"];

for (const texte of textes) {
  try {
    const donnees = JSON.parse(texte);
    console.log("Lu :", donnees.nom);
  } catch (erreur) {
    console.log("Illisible :", erreur.message);
  }
}
console.log("Le programme continue.");
```

```sortie
Lu : Léa
Illisible : Expected property name or '}' in JSON at position 1 (line 1 column 2)
Le programme continue.
```

Dès qu'une erreur se produit dans `try`, JavaScript saute **immédiatement** au `catch` :
la ligne `console.log("Lu :", ...)` n'est pas exécutée pour le second texte.

Un objet erreur a trois propriétés utiles :

| Propriété | Contenu |
|-----------|---------|
| `name` | le type de l'erreur : `"SyntaxError"` |
| `message` | la description |
| `stack` | la trace d'appels complète, en texte |

> **Astuce :** si vous n'avez pas besoin de l'erreur, vous pouvez écrire `catch { ... }`
> sans parenthèses.

## finally

Le bloc `finally` (« pour finir ») s'exécute **toujours** : après `try` si tout va bien,
après `catch` s'il y a eu une erreur. Il sert à ranger, à fermer, à remettre en état.

```javascript
function charger(texte) {
  console.log("Ouverture");
  try {
    return JSON.parse(texte);
  } catch (erreur) {
    console.log("Problème :", erreur.name);
    return null;
  } finally {
    console.log("Fermeture");
  }
}

console.log(charger("[1, 2]"));
console.log(charger("[1, 2"));
```

```sortie
Ouverture
Fermeture
[ 1, 2 ]
Ouverture
Problème : SyntaxError
Fermeture
null
```

Même un `return` dans `try` n'empêche pas `finally` de s'exécuter.

## Lever une erreur avec throw

`throw` (« lancer ») lève une erreur vous-même, par exemple pour refuser une valeur
impossible. Lancez toujours un objet `Error`, avec un message clair.

```javascript
function retirer(solde, montant) {
  if (montant > solde) {
    throw new Error("solde insuffisant");
  }
  return solde - montant;
}

console.log(retirer(100, 30));

try {
  retirer(100, 500);
} catch (erreur) {
  console.log(`Refusé : ${erreur.message}`);
}
```

```sortie
70
Refusé : solde insuffisant
```

`throw` arrête la fonction sur-le-champ, comme `return`. L'erreur remonte ensuite la pile
d'appels jusqu'au premier `catch` qu'elle rencontre. S'il n'y en a aucun, le programme
s'arrête avec le message, comme au début de cette page.

## Les types d'erreurs

JavaScript a plusieurs classes d'erreurs prédéfinies. Le nom vous met sur la piste.

| Type | Quand ? | Exemple de message |
|------|---------|--------------------|
| `TypeError` | mauvaise sorte de valeur : propriété de `undefined`, appel d'une non-fonction | `f is not a function` |
| `ReferenceError` | variable inconnue (souvent une faute de frappe) | `inconnue is not defined` |
| `RangeError` | nombre hors des limites permises | `Invalid array length` |
| `SyntaxError` | code ou JSON mal écrit | `missing ) after argument list` |

Vous pouvez lever ces types vous-même : `throw new RangeError("note entre 0 et 20")`.
Vérifiez le type d'une erreur attrapée avec `instanceof`.

```javascript
const essais = [
  () => null.longueur,
  () => variableInconnue,
  () => "ab".repeat(-1),
  () => JSON.parse("{"),
];

for (const essai of essais) {
  try {
    essai();
  } catch (erreur) {
    const type = erreur instanceof TypeError ? "type" : "autre";
    console.log(`${erreur.name} (${type}) : ${erreur.message}`);
  }
}
```

```sortie
TypeError (type) : Cannot read properties of null (reading 'longueur')
ReferenceError (autre) : variableInconnue is not defined
RangeError (autre) : Invalid count value: -1
SyntaxError (autre) : Expected property name or '}' in JSON at position 1 (line 1 column 2)
```

> **Attention :** une `SyntaxError` dans **votre code** (une parenthèse oubliée) ne peut
> pas être attrapée : Node refuse le fichier entier avant de l'exécuter. Seul
> `JSON.parse` lève une `SyntaxError` qu'on peut attraper.

## Vos propres classes d'erreur

Une erreur est un objet comme un autre. Pour distinguer vos erreurs, créez une classe qui
hérite de `Error` (voir [l'héritage](heritage.md)). Donnez-lui un `name` : il apparaîtra
dans les messages.

```javascript
class ValidationError extends Error {
  constructor(message, champ) {
    super(message);
    this.name = "ValidationError";
    this.champ = champ;
  }
}

function inscrire(utilisateur) {
  if (!utilisateur.email.includes("@")) {
    throw new ValidationError("adresse invalide", "email");
  }
  return "Inscrit !";
}

try {
  inscrire({ email: "ada.exemple.fr" });
} catch (erreur) {
  if (erreur instanceof ValidationError) {
    console.log(`Champ ${erreur.champ} : ${erreur.message}`);
    console.log(String(erreur));
  } else {
    throw erreur;
  }
}
```

```sortie
Champ email : adresse invalide
ValidationError: adresse invalide
```

Remarquez le `else { throw erreur; }` : on ne traite que les erreurs qu'on connaît, et on
**relance** les autres. Ainsi, un vrai bug n'est pas caché en silence.

## Chaîner une cause avec error.cause

Souvent, une erreur technique (« JSON mal formé ») cache un problème plus parlant pour
l'utilisateur (« la configuration est illisible »). On lève alors une nouvelle erreur,
en gardant l'ancienne dans l'option `cause`.

```javascript
function lireConfiguration(texte) {
  try {
    return JSON.parse(texte);
  } catch (erreur) {
    throw new Error("Configuration illisible", { cause: erreur });
  }
}

try {
  lireConfiguration("{oups");
} catch (erreur) {
  console.log(erreur.message);
  console.log("Cause :", erreur.cause.name);
}
```

```sortie
Configuration illisible
Cause : SyntaxError
```

Si l'erreur n'est pas attrapée, Node affiche les deux, la cause sous `[cause]` : on
garde toute l'histoire.

## Vérifier les données reçues

La meilleure erreur est celle qu'on détecte tôt. Quand une fonction reçoit des données
de l'extérieur (un formulaire, un fichier), vérifiez-les **au début** et refusez
clairement ce qui ne va pas. C'est le contrôle des billets à l'entrée du train, plutôt
qu'en plein voyage.

```javascript
function lireNote(texte) {
  const note = Number(texte);
  if (texte.trim() === "" || Number.isNaN(note)) {
    throw new TypeError(`"${texte}" n'est pas un nombre`);
  }
  if (note < 0 || note > 20) {
    throw new RangeError(`${note} n'est pas entre 0 et 20`);
  }
  return note;
}

for (const saisie of ["15", "douze", "25", ""]) {
  try {
    console.log("Note :", lireNote(saisie));
  } catch (erreur) {
    console.log(`${erreur.name} : ${erreur.message}`);
  }
}
```

```sortie
Note : 15
TypeError : "douze" n'est pas un nombre
RangeError : 25 n'est pas entre 0 et 20
TypeError : "" n'est pas un nombre
```

> **À retenir :** `Number("douze")` ne lève pas d'erreur : il renvoie `NaN`, sans rien
> dire. C'est à vous de le vérifier avec `Number.isNaN`. De même, `Number("")` vaut `0` :
> d'où le test sur le texte vide.

## Erreurs fréquentes

### Lancer un texte au lieu d'une Error

```javascript norun
try {
  throw "fichier introuvable";
} catch (erreur) {
  console.log(erreur.message);
}
```

```text
undefined
```

Pas de message d'erreur, mais un résultat faux : un texte n'a pas de propriété
`message`, ni de trace d'appels. Lancez toujours un objet :
`throw new Error("fichier introuvable")`.

### Utiliser une variable déclarée dans try

```javascript norun
try {
  const age = Number.parseInt("42");
} catch (e) {
  console.log("Erreur");
}
console.log(age + 1);
```

```text
file:///home/vous/projet/main.mjs:6
console.log(age + 1);
            ^

ReferenceError: age is not defined
    at file:///home/vous/projet/main.mjs:6:13
```

Le bloc `try` a sa propre [portée](closures.md#la-portee-d-une-variable) : `age` disparaît
à l'accolade fermante. Déclarez `let age;` avant le `try`, ou utilisez `age` à
l'intérieur du bloc.

### Croire que try attrape une faute de syntaxe

```javascript norun
console.log("début");
console.log("bonjour";
```

```text
file:///home/vous/projet/main.mjs:2
console.log("bonjour";
            ^^^^^^^^^

SyntaxError: missing ) after argument list
```

Même `début` ne s'affiche pas : Node lit tout le fichier avant de l'exécuter, et refuse
un fichier mal écrit. Aucun `try` ne peut l'attraper. Corrigez le code : ici, la
parenthèse fermante manque.

## Testez-vous

**Question 1 :** qu'affiche ce code ?

```javascript norun
try {
  console.log("A");
  throw new Error("panne");
  console.log("B");
} catch {
  console.log("C");
} finally {
  console.log("D");
}
```

> **Réponse**
>
> `A`, `C`, puis `D`. `B` n'est jamais affiché : après `throw`, JavaScript saute
> directement au `catch`. `finally` s'exécute toujours.

**Question 2 :** dans une trace d'appels, quelle ligne regarder en premier ?

> **Réponse**
>
> La première ligne `at ...` qui parle de **votre** fichier. Les lignes `node:internal`
> appartiennent à Node.

**Question 3 :** écrivez une fonction `diviser(a, b)` qui lève une `RangeError` si `b`
vaut `0`, et appelez-la dans un `try`.

> **Réponse**
>
> ```javascript
> function diviser(a, b) {
>   if (b === 0) {
>     throw new RangeError("division par zéro");
>   }
>   return a / b;
> }
>
> try {
>   console.log(diviser(10, 2));
>   console.log(diviser(1, 0));
> } catch (erreur) {
>   console.log(`${erreur.name} : ${erreur.message}`);
> }
> ```
>
> ```sortie
> 5
> RangeError : division par zéro
> ```

## En résumé

- Un message d'erreur donne le fichier, la ligne, le type et la trace d'appels : cherchez
  la première ligne de votre code.
- `try` / `catch` attrapent une erreur ; `finally` s'exécute toujours.
- `throw new Error("...")` lève une erreur ; elle remonte jusqu'au premier `catch`.
- Vos erreurs héritent de `Error` ; `{ cause }` garde l'erreur d'origine.
- Vérifiez les données au début d'une fonction, et refusez-les avec un message clair.

Suite : [modules et npm](modules.md). Comparez avec
[les erreurs en Ruby](../ruby/erreurs.md).
