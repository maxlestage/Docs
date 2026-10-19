# Le navigateur et le DOM

Cette page montre comment JavaScript, dans le navigateur, lit et modifie une page web, et comment il réagit aux clics et aux saisies de l'utilisateur.

> **Objectifs :**
>
> - vous saurez relier un script à une page HTML ;
> - vous saurez trouver un élément avec `document.querySelector` et changer son texte ;
> - vous saurez créer des éléments et réagir aux événements avec `addEventListener` ;
> - vous saurez garder des données dans le navigateur avec `localStorage` ;
> - vous saurez séparer la logique (testable) du code qui touche la page.

## Une page HTML et son script

Une page web est écrite en [HTML](../debuter/glossaire.md#html) : un texte avec des
**balises** comme `<h1>` (un titre) ou `<button>` (un bouton). Le navigateur lit ce
texte et dessine la page. JavaScript arrive par la balise `<script>`.

```html norun
<!doctype html>
<html lang="fr">
  <head>
    <meta charset="utf-8">
    <title>Ma page</title>
  </head>
  <body>
    <h1>Bonjour</h1>
    <p id="message">Un paragraphe.</p>
    <button id="bouton">Cliquez-moi</button>

    <script type="module" src="main.js"></script>
  </body>
</html>
```

- `src="main.js"` : le code est dans un fichier à côté de la page.
- `type="module"` : le fichier est un module, comme ceux de [ce chapitre](modules.md).
  Un module attend que la page soit lue avant de s'exécuter.

Pour essayer, enregistrez la page sous `index.html` et ouvrez-la dans votre navigateur
(double-clic sur le fichier).

> **Note :** dans le navigateur, il n'y a pas de terminal. Ce que vous affichez avec
> `console.log` apparaît dans la **console** du navigateur (voir plus bas).

## La console du navigateur

Chaque navigateur contient des **outils de développement**. Ouvrez-les avec `F12` (ou
`Cmd`+`Option`+`I` sur Mac), puis choisissez l'onglet **Console**.

Vous y voyez :

- les messages de `console.log` ;
- les erreurs, en rouge, avec le fichier et la ligne ;
- une ligne de saisie : tapez du JavaScript, il s'exécute tout de suite sur la page.

```text
> document.title
'Ma page'
> 2 + 2
4
```

> **Astuce :** gardez la console ouverte quand vous travaillez sur une page. Une erreur
> silencieuse pour l'utilisateur y est toujours visible.

## L'arbre du DOM

Le navigateur transforme le HTML en un **arbre d'[objets](../debuter/glossaire.md#objet)** :
le **DOM** (*Document Object Model*, « modèle objet du document »). Chaque balise
devient un **nœud**, rangé dans la balise qui le contient, comme des boîtes dans des
boîtes.

```text
document
└── html
    ├── head
    │   └── title ─ "Ma page"
    └── body
        ├── h1 ─ "Bonjour"
        ├── p#message ─ "Un paragraphe."
        ├── button#bouton ─ "Cliquez-moi"
        └── script
```

L'objet `document` représente toute la page. C'est la racine de l'arbre. Modifier un
objet du DOM modifie la page à l'écran, immédiatement.

## Trouver un élément avec querySelector

`document.querySelector` prend un **sélecteur CSS** et renvoie le premier élément qui
correspond, ou `null` s'il n'y en a aucun.

| Sélecteur | Trouve |
|-----------|--------|
| `"h1"` | la première balise `<h1>` |
| `"#message"` | l'élément dont l'`id` vaut `message` |
| `".important"` | le premier élément de classe `important` |
| `"ul li"` | un `<li>` placé dans un `<ul>` |

```javascript norun
const titre = document.querySelector("h1");
const message = document.querySelector("#message");
console.log(titre.textContent);
```

```text
Bonjour
```

`document.querySelectorAll` renvoie **tous** les éléments qui correspondent. On les
parcourt avec `for...of`.

```javascript norun
for (const li of document.querySelectorAll("li")) {
  console.log(li.textContent);
}
```

## Changer le texte : textContent et innerHTML

`textContent` lit ou remplace le texte d'un élément.

```javascript norun
const message = document.querySelector("#message");
message.textContent = "Le texte a changé !";
```

`innerHTML` fait la même chose, mais le texte est lu **comme du HTML** : les balises
deviennent de vrais éléments.

```javascript norun
message.innerHTML = "Un mot en <strong>gras</strong>.";
```

> **Attention :** n'utilisez jamais `innerHTML` avec un texte tapé par un utilisateur.
> S'il écrit `<img src=x onerror="alert('piraté')">`, ce code s'exécute dans la page :
> c'est une attaque **XSS** (*cross-site scripting*). Avec `textContent`, le même texte
> s'affiche tel quel, sans danger.

Si vous devez vraiment construire du HTML à partir d'un texte, remplacez d'abord les
caractères spéciaux. Cette [fonction](../debuter/glossaire.md#fonction) ne touche pas
au DOM : elle tourne aussi dans Node.

```javascript
function echapper(texte) {
  return texte
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;");
}

console.log(echapper('<img src=x onerror="alert(1)">'));
```

```sortie
&lt;img src=x onerror=&quot;alert(1)&quot;&gt;
```

## Changer le style avec classList

Le style se décrit en CSS, dans des **classes**. `classList` ajoute ou enlève une
classe à un élément :

- `add("x")` ajoute la classe `x` ;
- `remove("x")` l'enlève ;
- `toggle("x")` l'ajoute si elle manque, l'enlève sinon (un interrupteur) ;
- `contains("x")` répond `true` ou `false`.

```javascript norun
const message = document.querySelector("#message");
message.classList.add("important");
message.classList.toggle("cache");
console.log(message.classList.contains("important"));
```

```text
true
```

## Créer des éléments

`document.createElement` fabrique un élément vide, qui n'est pas encore dans la page.
`append` l'accroche à un parent : il apparaît alors à l'écran.

```javascript norun
const liste = document.querySelector("#courses");

for (const article of ["pain", "lait", "œufs"]) {
  const li = document.createElement("li");
  li.textContent = article;
  liste.append(li);
}
```

La page contient maintenant :

```text
• pain
• lait
• œufs
```

Pour supprimer un élément, appelez `element.remove()`. Pour vider un parent,
`liste.replaceChildren()`.

## Réagir aux événements

Un **événement** est quelque chose qui arrive sur la page : un clic, une touche, un
formulaire envoyé. `addEventListener` (« ajouter un écouteur ») donne une fonction à
appeler quand l'événement arrive. C'est comme une sonnette : vous ne guettez pas la
porte, la sonnette vous prévient.

```javascript norun
const bouton = document.querySelector("#bouton");

bouton.addEventListener("click", () => {
  console.log("Clic !");
});
```

La fonction reçoit un objet `event` qui décrit l'événement. `event.target` est
l'élément concerné.

| Événement | Quand ? |
|-----------|---------|
| `"click"` | un clic sur l'élément |
| `"input"` | le contenu d'un champ change, à chaque touche |
| `"submit"` | un formulaire est envoyé (Entrée ou bouton) |
| `"keydown"` | une touche est enfoncée |

### Suivre la saisie avec input

```javascript norun
const champ = document.querySelector("#nom");
const apercu = document.querySelector("#apercu");

champ.addEventListener("input", (event) => {
  apercu.textContent = `Bonjour, ${event.target.value} !`;
});
```

`value` contient le texte tapé dans le champ.

### Un formulaire et preventDefault

Par défaut, envoyer un formulaire **recharge la page**. Tout ce que votre script a
construit disparaît. `event.preventDefault()` annule ce comportement par défaut.

```javascript norun
const formulaire = document.querySelector("form");

formulaire.addEventListener("submit", (event) => {
  event.preventDefault();
  console.log("Envoyé, sans recharger la page.");
});
```

## Séparer la logique du DOM

Le code qui touche la page ne tourne que dans un navigateur. Pour le tester
facilement, rangez les calculs dans des **fonctions pures** : elles reçoivent des
valeurs et renvoient un résultat, sans toucher au DOM. Le code du DOM se contente
ensuite d'appeler ces fonctions et d'afficher le résultat.

### La logique d'un compteur

```javascript
function suivant(valeur, pas, max) {
  return Math.min(valeur + pas, max);
}

function texteCompteur(valeur) {
  return valeur === 1 ? "1 clic" : `${valeur} clics`;
}

let valeur = 0;
for (let i = 0; i < 3; i++) {
  valeur = suivant(valeur, 1, 10);
  console.log(texteCompteur(valeur));
}
console.log(texteCompteur(suivant(9, 5, 10)));
```

```sortie
1 clic
2 clics
3 clics
10 clics
```

### La page du compteur

La page ne contient qu'un bouton et un paragraphe :

```html norun
<p id="compteur">0 clics</p>
<button id="plus">+1</button>
<script type="module" src="compteur.js"></script>
```

Le script relie le bouton à la logique :

```javascript norun
// compteur.js
function suivant(valeur, pas, max) {
  return Math.min(valeur + pas, max);
}
function texteCompteur(valeur) {
  return valeur === 1 ? "1 clic" : `${valeur} clics`;
}

let valeur = 0;
const affichage = document.querySelector("#compteur");

document.querySelector("#plus").addEventListener("click", () => {
  valeur = suivant(valeur, 1, 10);
  affichage.textContent = texteCompteur(valeur);
});
```

### La logique d'une liste de tâches

Une tâche est un objet `{ texte, faite }`. Chaque fonction renvoie une **nouvelle**
liste au lieu de modifier l'ancienne : c'est plus facile à suivre.

```javascript
function ajouter(taches, texte) {
  const propre = texte.trim();
  if (propre === "") return taches;
  return [...taches, { texte: propre, faite: false }];
}

function basculer(taches, index) {
  return taches.map((t, i) => (i === index ? { ...t, faite: !t.faite } : t));
}

function resume(taches) {
  const restantes = taches.filter((t) => !t.faite).length;
  return `${restantes} tâche(s) à faire sur ${taches.length}`;
}

let taches = [];
taches = ajouter(taches, "  Acheter du pain ");
taches = ajouter(taches, "");
taches = ajouter(taches, "Appeler Léa");
taches = basculer(taches, 0);
console.log(taches);
console.log(resume(taches));
```

```sortie
[
  { texte: 'Acheter du pain', faite: true },
  { texte: 'Appeler Léa', faite: false }
]
1 tâche(s) à faire sur 2
```

### La page de la liste de tâches

```html norun
<form id="formulaire">
  <input id="champ" placeholder="Nouvelle tâche">
  <button>Ajouter</button>
</form>
<ul id="liste"></ul>
<p id="resume"></p>
<script type="module" src="taches.js"></script>
```

Le script redessine toute la liste après chaque changement, avec la fonction
`afficher`. Les fonctions `ajouter`, `basculer` et `resume` sont celles du dessus.

```javascript norun
// taches.js (ajouter, basculer et resume sont définies au-dessus)
let taches = [];

const formulaire = document.querySelector("#formulaire");
const champ = document.querySelector("#champ");
const liste = document.querySelector("#liste");

function afficher() {
  liste.replaceChildren();
  taches.forEach((tache, index) => {
    const li = document.createElement("li");
    li.textContent = tache.texte;
    li.classList.toggle("faite", tache.faite);
    li.addEventListener("click", () => {
      taches = basculer(taches, index);
      afficher();
    });
    liste.append(li);
  });
  document.querySelector("#resume").textContent = resume(taches);
}

formulaire.addEventListener("submit", (event) => {
  event.preventDefault();
  taches = ajouter(taches, champ.value);
  champ.value = "";
  afficher();
});

afficher();
```

> **À retenir :** `toggle("faite", tache.faite)` avec un deuxième argument ajoute la
> classe si la valeur est `true`, l'enlève sinon.

## Garder des données avec localStorage

Quand on recharge la page, les variables repartent de zéro. `localStorage` est un petit
carnet que le navigateur garde pour votre site, même après sa fermeture.

- `localStorage.setItem("cle", texte)` enregistre un texte ;
- `localStorage.getItem("cle")` le relit (ou `null` s'il n'existe pas) ;
- `localStorage.removeItem("cle")` l'efface.

`localStorage` ne range que des [chaînes de caractères](../debuter/glossaire.md#chaine-de-caracteres).
Pour une liste ou un objet, passez par [JSON](../debuter/glossaire.md#json) (voir
[JSON et CSV](formats.md)) :

```javascript norun
function sauver(taches) {
  localStorage.setItem("taches", JSON.stringify(taches));
}

function charger() {
  return JSON.parse(localStorage.getItem("taches") ?? "[]");
}

let taches = charger();
```

La conversion elle-même ne dépend pas du navigateur. La voici en Node :

```javascript
const taches = [{ texte: "Acheter du pain", faite: true }];

const texte = JSON.stringify(taches);
console.log(texte);

const relu = JSON.parse(texte);
console.log(relu[0].texte, relu[0].faite);
console.log(JSON.parse(null ?? "[]"));
```

```sortie
[{"texte":"Acheter du pain","faite":true}]
Acheter du pain true
[]
```

> **Attention :** `localStorage` est visible par tout script de la page et par
> quiconque utilise l'ordinateur. N'y rangez jamais de mot de passe.

## Erreurs fréquentes

### Lancer un script de page avec Node

```javascript norun
document.querySelector("h1");
```

```text
document.querySelector("h1");
^

ReferenceError: document is not defined
```

`document` n'existe que dans un navigateur. Node n'a pas de page. Testez la logique
pure dans Node, et le code du DOM dans le navigateur.

### Un élément introuvable

Ici, la page n'a pas d'élément `id="bouton"` (ou il est mal orthographié) :

```javascript norun
const bouton = document.querySelector("#boutton");
bouton.addEventListener("click", () => {});
```

```text
Uncaught TypeError: Cannot read properties of null (reading 'addEventListener')
```

`querySelector` a renvoyé `null`. Vérifiez le sélecteur (le `#` pour un `id`) et
l'orthographe de l'`id` dans le HTML. Vérifiez aussi que le script est chargé avec
`type="module"` ou placé en bas de la page, après l'élément.

### Ranger un objet sans JSON dans localStorage

```javascript norun
localStorage.setItem("taches", [{ texte: "pain" }]);
const taches = JSON.parse(localStorage.getItem("taches"));
```

```text
Uncaught SyntaxError: "[object Object]" is not valid JSON
```

`setItem` a converti le tableau en texte à sa façon : `"[object Object]"`. Écrivez
`JSON.stringify(taches)` en enregistrant, `JSON.parse` en relisant.

### Oublier preventDefault

```javascript norun
formulaire.addEventListener("submit", () => {
  taches = ajouter(taches, champ.value);
  afficher();
});
```

La tâche apparaît une fraction de seconde, puis la page se recharge et tout
disparaît. Il n'y a pas de message d'erreur. Ajoutez `event.preventDefault()` au début
de la fonction.

## Testez-vous

**Question 1 :** quelle est la différence entre `textContent` et `innerHTML` ?

> **Réponse**
>
> `textContent` affiche le texte tel quel. `innerHTML` interprète le texte comme du
> HTML : les balises deviennent des éléments. Avec un texte venant de l'utilisateur,
> `innerHTML` ouvre la porte aux attaques XSS.

**Question 2 :** que renvoie `document.querySelector("#absent")` si aucun élément n'a
cet `id` ?

> **Réponse**
>
> `null`. Appeler une méthode dessus provoque une `TypeError`.

**Question 3 :** écrivez une fonction pure `retirer(taches, index)` qui renvoie la
liste sans la tâche numéro `index`.

> **Réponse**
>
> ```javascript
> function retirer(taches, index) {
>   return taches.filter((_, i) => i !== index);
> }
>
> console.log(retirer(["a", "b", "c"], 1));
> ```
>
> ```sortie
> [ 'a', 'c' ]
> ```

## En résumé

- La balise `<script type="module" src="…">` relie un fichier JavaScript à la page.
- Le DOM est l'arbre des éléments ; `querySelector` en trouve un, ou renvoie `null`.
- `textContent` change le texte sans danger ; `innerHTML` interprète du HTML (risque XSS).
- `createElement` et `append` créent des éléments ; `classList` change leurs classes.
- `addEventListener` réagit aux clics, saisies et envois ; `preventDefault` évite le rechargement.
- `localStorage` garde des textes : passez par `JSON.stringify` et `JSON.parse`.

Suite : [Dessiner dans le navigateur avec canvas](canvas.md)
