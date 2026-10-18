# La programmation asynchrone

Cette page explique comment JavaScript fait plusieurs choses « en même temps » sans attendre bêtement : la boucle d'événements, les callbacks, les promesses et `async`/`await`.

> **Objectifs :**
>
> - vous comprendrez le principe de la boucle d'événements ;
> - vous saurez prévoir l'ordre d'affichage avec `setTimeout` ;
> - vous saurez créer et utiliser une promesse avec `then` et `catch` ;
> - vous saurez écrire du code asynchrone lisible avec `async` et `await` ;
> - vous saurez lancer plusieurs tâches avec `Promise.all`, `allSettled` et `race`.

## Attendre sans bloquer

Certaines opérations sont **lentes** : télécharger une page web, lire un gros fichier,
interroger une base de données. Si le programme restait figé pendant ce temps, rien
d'autre ne pourrait se passer : une page web ne répondrait plus aux clics.

JavaScript ne fait qu'**une seule chose à la fois**. Mais il ne reste jamais planté à
attendre : il lance l'opération lente, passe à la suite, et revient quand le résultat
est prêt. C'est la programmation [asynchrone](../debuter/glossaire.md#async).

## La boucle d'événements

Imaginez un serveur seul dans un restaurant. Il prend la commande de la table 1, la
transmet à la cuisine, et **n'attend pas** devant les fourneaux : il va prendre la
commande de la table 2. Quand la cuisine sonne, il apporte le plat.

JavaScript fonctionne ainsi :

1. il exécute le code du programme, ligne après ligne, jusqu'au bout ;
2. pendant ce temps, les opérations lentes (minuteries, réseau, fichiers) se font « en
   cuisine » ;
3. quand une opération est finie, sa suite est placée dans une **file d'attente** ;
4. dès qu'il est libre, JavaScript prend la tâche suivante de la file, et recommence.

Ce tour sans fin s'appelle la **boucle d'événements** (*event loop*). Le programme se
termine quand il n'y a plus rien à exécuter ni à attendre.

## setTimeout : programmer pour plus tard

`setTimeout(fonction, delai)` demande d'exécuter une fonction **plus tard**, après un
délai en millisecondes (1 000 ms = 1 seconde). La fonction donnée s'appelle un
**callback** (« rappel ») : on vous rappellera.

```javascript
console.log("1. Je commande");

setTimeout(() => {
  console.log("3. Le plat arrive");
}, 100);

console.log("2. Je discute en attendant");
```

```sortie
1. Je commande
2. Je discute en attendant
3. Le plat arrive
```

`setTimeout` ne bloque pas : il note la demande, et le programme continue tout de suite.

### Même avec un délai de 0

Un callback attend toujours que le code en cours soit **entièrement** fini, même avec un
délai de 0 : il passe par la file d'attente.

```javascript
setTimeout(() => console.log("C (délai 200)"), 200);
setTimeout(() => console.log("B (délai 0)"), 0);
setTimeout(() => console.log("D (délai 300)"), 300);
console.log("A (tout de suite)");
```

```sortie
A (tout de suite)
B (délai 0)
C (délai 200)
D (délai 300)
```

L'ordre dans le code ne compte pas : c'est le délai qui décide. Et le code « normal »
passe toujours avant.

## Les callbacks et leurs limites

Pendant longtemps, toutes les opérations lentes prenaient un callback : « quand tu as
fini, appelle cette fonction ». Simulons une base de données lente.

```javascript
function chargerUtilisateur(id, callback) {
  setTimeout(() => callback({ id, nom: "Ada" }), 50);
}

function chargerCommandes(utilisateur, callback) {
  setTimeout(() => callback(["livre", "stylo"]), 50);
}

chargerUtilisateur(1, (utilisateur) => {
  console.log("Utilisateur :", utilisateur.nom);
  chargerCommandes(utilisateur, (commandes) => {
    console.log("Commandes :", commandes.join(", "));
  });
});
```

```sortie
Utilisateur : Ada
Commandes : livre, stylo
```

Cela marche. Mais chaque étape ajoute un niveau d'indentation. Avec cinq étapes et la
gestion des erreurs à chaque niveau, le code devient une « pyramide » illisible, qu'on
appelle l'**enfer des callbacks**. Les promesses règlent ce problème.

## Les promesses

Une **promesse** (`Promise`) est un objet qui représente un résultat **pas encore
disponible**. C'est le ticket qu'on vous donne au comptoir : pour l'instant, pas de
sandwich, mais le ticket promet qu'il arrivera… ou qu'on vous dira pourquoi il n'arrivera
pas.

Une promesse est dans l'un de trois états :

| État | Sens |
|------|------|
| en attente (*pending*) | le résultat n'est pas encore là |
| tenue (*fulfilled*) | le résultat est arrivé |
| rompue (*rejected*) | une erreur s'est produite |

### Créer une promesse

On crée une promesse avec `new Promise`. On lui donne une fonction qui reçoit deux
outils : `resolve` (tenir la promesse, avec un résultat) et `reject` (la rompre, avec une
erreur).

```javascript
function lancerDe(valeur) {
  return new Promise((resolve, reject) => {
    setTimeout(() => {
      if (valeur >= 1 && valeur <= 6) {
        resolve(valeur);
      } else {
        reject(new Error(`face impossible : ${valeur}`));
      }
    }, 50);
  });
}

console.log(lancerDe(4));
```

```sortie
Promise { <pending> }
```

Juste après sa création, la promesse est en attente : le dé roule encore.

### then et catch

`.then(fonction)` dit quoi faire **quand** la promesse est tenue : la fonction reçoit le
résultat. `.catch(fonction)` dit quoi faire si elle est rompue : la fonction reçoit
l'erreur. `.finally(fonction)` s'exécute dans les deux cas. Avec la même fonction
`lancerDe` :

```javascript norun
lancerDe(4)
  .then((face) => console.log("Résultat :", face))
  .catch((erreur) => console.log("Erreur :", erreur.message));

lancerDe(9)
  .then((face) => console.log("Résultat :", face))
  .catch((erreur) => console.log("Erreur :", erreur.message))
  .finally(() => console.log("Lancer terminé"));
```

```text
Résultat : 4
Erreur : face impossible : 9
Lancer terminé
```

Chaque `then` renvoie une nouvelle promesse : on peut les **enchaîner** au lieu de les
emboîter. Le `catch` final attrape une erreur survenue à n'importe quelle étape.

```javascript
const doubler = (n) => new Promise((resolve) => setTimeout(() => resolve(n * 2), 20));

doubler(5)
  .then((n) => doubler(n))
  .then((n) => doubler(n))
  .then((n) => console.log("Résultat final :", n))
  .catch((erreur) => console.log(erreur.message));
```

```sortie
Résultat final : 40
```

## async et await

`async` et `await` permettent d'écrire du code asynchrone **comme du code normal**, de
haut en bas. C'est la façon recommandée aujourd'hui.

- `async` devant une fonction la rend asynchrone : elle renvoie toujours une promesse.
- `await` devant une promesse **met la fonction en pause** jusqu'au résultat, sans
  bloquer le reste du programme.

Voici l'exemple des callbacks, réécrit :

```javascript
const attendre = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

async function chargerUtilisateur(id) {
  await attendre(50);
  return { id, nom: "Ada" };
}

async function chargerCommandes(utilisateur) {
  await attendre(50);
  return ["livre", "stylo"];
}

async function afficherCommandes() {
  const utilisateur = await chargerUtilisateur(1);
  console.log("Utilisateur :", utilisateur.nom);
  const commandes = await chargerCommandes(utilisateur);
  console.log("Commandes :", commandes.join(", "));
}

await afficherCommandes();
console.log("Terminé");
```

```sortie
Utilisateur : Ada
Commandes : livre, stylo
Terminé
```

Plus de pyramide : chaque étape est une ligne. Dans un module (fichier `.mjs`, ou
`"type": "module"`), on peut aussi écrire `await` **directement** au niveau principal du
fichier, comme à l'avant-dernière ligne.

> **À retenir :** `await` ne s'utilise que dans une fonction `async`, ou au niveau
> principal d'un module.

## Gérer les erreurs avec try et catch

Avec `await`, une promesse rompue **lève une erreur**, comme `throw`. On l'attrape donc
avec `try` et `catch`, exactement comme dans [gérer les erreurs](erreurs.md).

```javascript
const attendre = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

async function chargerProfil(nom) {
  await attendre(30);
  if (nom === "") {
    throw new Error("nom vide");
  }
  return { nom, niveau: 3 };
}

async function afficher(nom) {
  try {
    const profil = await chargerProfil(nom);
    console.log(`${profil.nom} est au niveau ${profil.niveau}`);
  } catch (erreur) {
    console.log("Impossible :", erreur.message);
  } finally {
    console.log("--");
  }
}

await afficher("Ada");
await afficher("");
```

```sortie
Ada est au niveau 3
--
Impossible : nom vide
--
```

## Plusieurs tâches à la fois

Avec `await` l'une après l'autre, les tâches se suivent : 3 tâches de 100 ms prennent
300 ms. Si elles ne dépendent pas les unes des autres, on peut les lancer **toutes en
même temps**.

### Promise.all : tout ou rien

`Promise.all(liste)` attend que **toutes** les promesses soient tenues, et donne la liste
des résultats, **dans l'ordre de la liste** (pas dans l'ordre d'arrivée). Si une seule
est rompue, tout est rompu.

```javascript
const attendre = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

async function telecharger(nom, ms) {
  await attendre(ms);
  console.log(`  ${nom} reçu`);
  return `${nom} (${ms} ms)`;
}

const resultats = await Promise.all([
  telecharger("image", 150),
  telecharger("texte", 50),
  telecharger("son", 100),
]);
console.log(resultats);
```

```sortie
  texte reçu
  son reçu
  image reçu
[ 'image (150 ms)', 'texte (50 ms)', 'son (100 ms)' ]
```

Les trois téléchargements se font en parallèle : le total prend environ 150 ms, le temps
du plus lent.

### Promise.allSettled : le bilan de chacune

`Promise.allSettled(liste)` attend que toutes soient terminées, tenues **ou** rompues,
et donne un bilan pour chacune. Une erreur n'arrête pas les autres.

```javascript
const attendre = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

async function verifier(site) {
  await attendre(20);
  if (site.includes("panne")) {
    throw new Error(`${site} ne répond pas`);
  }
  return `${site} OK`;
}

const bilans = await Promise.allSettled([
  verifier("exemple.fr"),
  verifier("panne.fr"),
  verifier("demo.fr"),
]);

for (const bilan of bilans) {
  if (bilan.status === "fulfilled") {
    console.log("✓", bilan.value);
  } else {
    console.log("✗", bilan.reason.message);
  }
}
```

```sortie
✓ exemple.fr OK
✗ panne.fr ne répond pas
✓ demo.fr OK
```

### Promise.race : le premier arrivé

`Promise.race(liste)` donne le résultat de la **première** promesse terminée. On s'en
sert souvent pour fixer une limite de temps.

```javascript
const attendre = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

async function reponseServeur(ms) {
  await attendre(ms);
  return `réponse en ${ms} ms`;
}

async function delaiDepasse(ms) {
  await attendre(ms);
  throw new Error(`trop long (plus de ${ms} ms)`);
}

for (const duree of [50, 300]) {
  try {
    const reponse = await Promise.race([reponseServeur(duree), delaiDepasse(150)]);
    console.log(reponse);
  } catch (erreur) {
    console.log("Abandon :", erreur.message);
  }
}
```

```sortie
réponse en 50 ms
Abandon : trop long (plus de 150 ms)
```

| Fonction | Attend | Résultat | Si une erreur |
|----------|--------|----------|---------------|
| `Promise.all` | toutes | la liste des valeurs | tout est rompu |
| `Promise.allSettled` | toutes | un bilan par promesse | noté dans le bilan |
| `Promise.race` | la première | sa valeur | si la première échoue |

## Un mot sur fetch

`fetch(adresse)` télécharge une ressource sur le web. C'est l'exemple le plus courant
d'opération asynchrone : il renvoie une promesse. Il est disponible dans Node et dans
tous les navigateurs.

```javascript norun
try {
  const reponse = await fetch("https://registry.npmjs.org/slugify/1.6.6");
  if (!reponse.ok) {
    throw new Error(`HTTP ${reponse.status}`);
  }
  const paquet = await reponse.json();
  console.log(`${paquet.name} ${paquet.version} : ${paquet.description}`);
} catch (erreur) {
  console.log("Échec :", erreur.message);
}
```

```text
slugify 1.6.6 : Slugifies a String
```

Ce code demande à npm la fiche du paquet `slugify` (voir [modules et npm](modules.md)).
Si le serveur répond une erreur, `reponse.ok` vaut `false` : avec une version qui
n'existe pas, le programme affiche `Échec : HTTP 404`.

Deux `await` : un pour la réponse du serveur, un pour lire son contenu en
[JSON](../debuter/glossaire.md#json). Le chapitre sur [le web](web.md) va plus loin.

## Erreurs fréquentes

### Oublier await

```javascript norun
async function chargerNom() {
  return "Ada";
}
const nom = chargerNom();
console.log(`Bonjour ${nom}`);
console.log(nom);
```

```text
Bonjour [object Promise]
Promise { 'Ada' }
```

Pas d'erreur, mais un résultat faux : une fonction `async` renvoie une **promesse**, pas
la valeur. Écrivez `const nom = await chargerNom();`.

### Utiliser await hors d'une fonction async

```javascript norun
function attendre(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}
function demarrer() {
  await attendre(100);
  console.log("Prêt");
}
demarrer();
```

```text
file:///home/vous/projet/main.mjs:5
  await attendre(100);
  ^^^^^

SyntaxError: Unexpected reserved word
```

« Mot réservé inattendu » : `await` n'est pas permis dans une fonction ordinaire.
Ajoutez `async` : `async function demarrer() { ... }`.

### Oublier de gérer une promesse rompue

```javascript norun
function charger() {
  return new Promise((resolve, reject) => {
    setTimeout(() => reject(new Error("serveur injoignable")), 10);
  });
}
charger().then((d) => console.log(d));
console.log("Suite du programme");
```

```text
Suite du programme
file:///home/vous/projet/main.mjs:3
    setTimeout(() => reject(new Error("serveur injoignable")), 10);
                            ^

Error: serveur injoignable
    at Timeout._onTimeout (file:///home/vous/projet/main.mjs:3:29)
```

Une promesse rompue que personne n'attrape arrête le programme. Ajoutez
`.catch((erreur) => ...)` à la chaîne, ou utilisez `await` dans un `try`/`catch`.

## Testez-vous

**Question 1 :** dans quel ordre s'affichent les lettres ?

```javascript norun
setTimeout(() => console.log("A"), 100);
console.log("B");
setTimeout(() => console.log("C"), 0);
console.log("D");
```

> **Réponse**
>
> `B`, `D`, `C`, `A`. Le code normal passe d'abord (`B`, `D`). Ensuite viennent les
> callbacks, du plus petit délai au plus grand : `C` (0 ms), puis `A` (100 ms).

**Question 2 :** que renvoie toujours une fonction `async` ?

> **Réponse**
>
> Une promesse. Si la fonction fait `return 5`, la promesse est tenue avec `5` ; si elle
> lève une erreur, la promesse est rompue.

**Question 3 :** écrivez une fonction `async` `sommeLente(a, b)` qui attend 50 ms puis
renvoie `a + b`, et lancez deux appels en parallèle avec `Promise.all`.

> **Réponse**
>
> ```javascript
> const attendre = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
>
> async function sommeLente(a, b) {
>   await attendre(50);
>   return a + b;
> }
>
> const resultats = await Promise.all([sommeLente(1, 2), sommeLente(10, 20)]);
> console.log(resultats);
> ```
>
> ```sortie
> [ 3, 30 ]
> ```

## En résumé

- JavaScript fait une chose à la fois, mais n'attend jamais en bloquant : la boucle
  d'événements reprend les tâches quand elles sont prêtes.
- `setTimeout` programme un callback ; le code en cours passe toujours avant.
- Une promesse représente un résultat futur : `then` pour le succès, `catch` pour
  l'erreur.
- `async`/`await` écrivent le code asynchrone de haut en bas ; les erreurs s'attrapent
  avec `try`/`catch`.
- `Promise.all`, `allSettled` et `race` lancent plusieurs tâches en parallèle.

Suite : [le DOM](dom.md). Comparez avec
[la concurrence en Ruby](../ruby/concurrence.md).
