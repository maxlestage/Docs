# Dessiner dans le navigateur avec canvas

Cette page montre comment dessiner des formes et animer une balle dans une page web avec l'élément `<canvas>`, en séparant le dessin (dans le navigateur) des calculs (testables avec Node).

> **Objectifs :**
>
> - vous saurez placer un `<canvas>` dans une page et obtenir son « pinceau » ;
> - vous comprendrez les coordonnées du canvas, où l'axe `y` descend ;
> - vous saurez dessiner des rectangles, des cercles, des traits et du texte ;
> - vous saurez calculer des positions avec des fonctions pures, testées dans Node ;
> - vous saurez animer une balle qui rebondit, une étape à la fois.

## Une toile de pixels

Un `<canvas>` (« toile » en anglais) est un rectangle vide dans la page. Vous y
dessinez avec JavaScript, pixel par pixel. C'est comme une feuille de papier
quadrillé : chaque petit carreau est un **pixel**, un point de couleur de l'écran.

Le canvas ne fait pas partie du [DOM](../debuter/glossaire.md#dom) comme un titre ou un
bouton : ce que vous y dessinez n'est pas un élément. C'est de la peinture. Une fois
posée, elle reste là, jusqu'à ce que vous peigniez par-dessus ou que vous effaciez.

```html norun
<!doctype html>
<html lang="fr">
  <head>
    <meta charset="utf-8">
    <title>Ma toile</title>
  </head>
  <body>
    <canvas id="toile" width="300" height="150"></canvas>
    <script type="module" src="dessin.js"></script>
  </body>
</html>
```

- `width="300"` : la toile fait 300 pixels de large ;
- `height="150"` : et 150 pixels de haut ;
- le script `dessin.js` fera le dessin (voir [le DOM](dom.md) pour relier un script).

Pour dessiner, on demande au canvas son **contexte 2D** : c'est le pinceau, l'objet
qui sait tracer des formes.

```javascript norun
// dessin.js
const toile = document.querySelector("#toile");
const ctx = toile.getContext("2d");   // le pinceau

ctx.fillStyle = "tomato";             // la couleur de remplissage
ctx.fillRect(10, 20, 100, 50);        // un rectangle plein
```

On note presque toujours ce pinceau `ctx` (pour *context*).

> **À retenir :** `<canvas>` est une toile vide ; `getContext("2d")` vous donne le
> pinceau `ctx`, et c'est lui qui dessine.

## Les coordonnées : y descend

Pour dire où dessiner, on donne deux nombres : `x` et `y`.

- `x` compte les pixels **vers la droite**, depuis le bord gauche ;
- `y` compte les pixels **vers le bas**, depuis le bord du haut.

Le point `(0, 0)` est donc en **haut à gauche**. C'est le contraire des maths de
l'école, où `y` monte. Pensez à une page de livre : on lit de haut en bas, la ligne 1
est en haut.

```text
 (0,0) ─────── x augmente ───────▶
   │ ┌──────────────────────────┐
   │ │                          │
   │ │    ■ (60,30)             │
   y │                          │
   │ │               ● (240,110)│
   │ │                          │
   ▼ └──────────────────────────┘
  y augmente            (300,150) = en bas à droite
```

Le point `(240, 110)` est plus **bas** que `(60, 30)`, car son `y` est plus grand.

| Point | Où dans la toile 300 × 150 |
|---|---|
| `(0, 0)` | coin en haut à gauche |
| `(300, 0)` | coin en haut à droite |
| `(0, 150)` | coin en bas à gauche |
| `(150, 75)` | le centre |

Si vous avez des coordonnées « à l'endroit », comme en maths, une petite fonction les
retourne. Elle ne touche pas au navigateur : Node peut la lancer.

```javascript
function versCanvas(x, y, hauteur) {
  return { x, y: hauteur - y };
}

console.log(versCanvas(0, 0, 150));    // en bas à gauche en maths
console.log(versCanvas(0, 150, 150));  // en haut à gauche en maths
console.log(versCanvas(100, 40, 150));
```

```sortie
{ x: 0, y: 150 }
{ x: 0, y: 0 }
{ x: 100, y: 110 }
```

| Appel | `hauteur - y` | `y` dans le canvas |
|---|---|---|
| `versCanvas(0, 0, 150)` | `150 - 0` | `150` (en bas) |
| `versCanvas(0, 150, 150)` | `150 - 150` | `0` (en haut) |
| `versCanvas(100, 40, 150)` | `150 - 40` | `110` |

Essayez : appelez `versCanvas(150, 75, 150)`. Le centre reste le centre :
`{ x: 150, y: 75 }`.

> **À retenir :** `(0, 0)` est en haut à gauche. Plus `y` est grand, plus le point est
> **bas**.

## Dessiner des formes

Voici les ordres principaux du pinceau. Ce code ne marche que dans un navigateur
(`document` n'existe pas dans Node).

### Rectangles

```javascript norun
ctx.fillStyle = "steelblue";
ctx.fillRect(10, 10, 80, 40);     // plein : x, y, largeur, hauteur

ctx.strokeStyle = "black";
ctx.lineWidth = 3;
ctx.strokeRect(110, 10, 80, 40);  // seulement le contour

ctx.clearRect(0, 0, 300, 150);    // efface toute la toile
```

`x` et `y` désignent le coin **en haut à gauche** du rectangle. La largeur va vers la
droite, la hauteur va **vers le bas**.

```text
 (10,10)
    ┌────────── 80 ──────────┐
    │                        │ 40
    └────────────────────────┘
                         (90,50)
```

### Traits et chemins

Un trait se dessine en trois temps, comme avec un crayon : on lève le crayon, on le
pose, on trace.

```javascript norun
ctx.beginPath();          // nouveau dessin : on oublie le précédent
ctx.moveTo(20, 130);      // poser le crayon (sans tracer)
ctx.lineTo(150, 20);      // tracer jusqu'ici
ctx.lineTo(280, 130);     // puis jusqu'ici
ctx.closePath();          // revenir au point de départ
ctx.stroke();             // dessiner le contour (un triangle)
```

### Cercles

Un cercle est un **arc** qui fait le tour complet : de l'angle `0` à l'angle
`2 * Math.PI` (les angles sont en radians ; `2 * Math.PI` est un tour).

```javascript norun
ctx.beginPath();
ctx.arc(150, 75, 30, 0, 2 * Math.PI);  // centre (150, 75), rayon 30
ctx.fillStyle = "gold";
ctx.fill();
```

Attention : pour un cercle, `(150, 75)` est le **centre**, pas le coin.

### Du texte

```javascript norun
ctx.font = "20px sans-serif";
ctx.fillStyle = "black";
ctx.fillText("Bonjour", 10, 140);  // x, y de la ligne de base du texte
```

| Ordre | Ce qu'il fait |
|---|---|
| `fillRect(x, y, l, h)` | un rectangle plein |
| `strokeRect(x, y, l, h)` | le contour d'un rectangle |
| `clearRect(x, y, l, h)` | efface une zone |
| `beginPath()` | commence un nouveau chemin |
| `moveTo(x, y)` / `lineTo(x, y)` | déplace le crayon / trace un trait |
| `arc(x, y, r, début, fin)` | un arc ou un cercle |
| `fill()` / `stroke()` | remplit / trace le chemin |
| `fillText(texte, x, y)` | écrit un texte |

> **À retenir :** on choisit d'abord la couleur (`fillStyle`, `strokeStyle`), puis on
> dessine. Pour les traits et les cercles : `beginPath`, le chemin, puis `fill` ou
> `stroke`.

## Voir un dessin sans navigateur

Pour comprendre ce que fait un dessin, inutile d'ouvrir un navigateur. On peut
fabriquer une **toile en caractères** : une grille de points `.`, où l'on écrit `#` à
la place d'un pixel peint. Chaque caractère joue le rôle d'un pixel.

```javascript
class ToileTexte {
  constructor(largeur, hauteur) {
    this.largeur = largeur;
    this.hauteur = hauteur;
    // une ligne par valeur de y, une case par valeur de x
    this.lignes = Array.from({ length: hauteur }, () => Array(largeur).fill("."));
  }

  point(x, y, car = "#") {
    if (x >= 0 && x < this.largeur && y >= 0 && y < this.hauteur) {
      this.lignes[y][x] = car;
    }
  }

  fillRect(x, y, l, h, car = "#") {
    for (let dy = 0; dy < h; dy++) {
      for (let dx = 0; dx < l; dx++) {
        this.point(x + dx, y + dy, car);
      }
    }
  }

  afficher() {
    console.log(this.lignes.map((ligne) => ligne.join("")).join("\n"));
  }
}

const toile = new ToileTexte(12, 5);
toile.point(0, 0, "O");       // l'origine
toile.fillRect(3, 1, 4, 2);   // un rectangle de 4 sur 2
toile.point(11, 4, "X");      // le dernier pixel
toile.afficher();
```

```sortie
O...........
...####.....
...####.....
............
...........X
```

Le `O` est bien en haut à gauche, et le `X` en bas à droite. Le rectangle commence en
`x = 3`, `y = 1`, et descend.

Suivons `fillRect(3, 1, 4, 2)` pas à pas. La boucle de `dy` (les lignes) contient la
boucle de `dx` (les colonnes) :

| Étape | dy | dx | Case peinte `(x + dx, y + dy)` |
|---|---|---|---|
| 1 | 0 | 0 | (3, 1) |
| 2 | 0 | 1 | (4, 1) |
| 3 | 0 | 2 | (5, 1) |
| 4 | 0 | 3 | (6, 1) |
| 5 | 1 | 0 | (3, 2) |
| 6 | 1 | 1 | (4, 2) |
| 7 | 1 | 2 | (5, 2) |
| 8 | 1 | 3 | (6, 2) |

Huit cases : 4 de large fois 2 de haut.

Essayez : remplacez `toile.fillRect(3, 1, 4, 2)` par `toile.fillRect(10, 3, 4, 4)`.
Le rectangle dépasse de la toile : seules les cases à l'intérieur sont peintes (c'est
le rôle du `if` de `point`). Un vrai canvas fait pareil, sans erreur.

> **À retenir :** une grille de caractères est un « canvas du pauvre ». Elle permet de
> vérifier la logique d'un dessin avec un simple `console.log`.

## Calculer des positions

Le plus dur, dans un dessin, ce sont les calculs : où placer chaque forme. Rangez-les
dans des **fonctions pures** (elles reçoivent des nombres, renvoient des nombres). Elles
marchent partout, et vous pouvez les vérifier dans Node.

### Centrer un rectangle

Pour centrer, on part du milieu de la toile et on recule de la moitié du rectangle.

```javascript
function centrer(largeurToile, hauteurToile, l, h) {
  return {
    x: (largeurToile - l) / 2,
    y: (hauteurToile - h) / 2,
  };
}

console.log(centrer(300, 150, 100, 50));
console.log(centrer(12, 5, 4, 1));
```

```sortie
{ x: 100, y: 50 }
{ x: 4, y: 2 }
```

| Calcul | Valeur |
|---|---|
| `(300 - 100) / 2` | `100` |
| `(150 - 50) / 2` | `50` |

Le rectangle va donc de `x = 100` à `x = 200` : il reste 100 pixels de chaque côté.

### Une rangée de carrés

On veut 4 carrés de 20 pixels, séparés de 10 pixels. Le carré numéro `i` commence à
`marge + i * (taille + ecart)`.

```javascript
function rangee(nombre, taille, ecart, marge) {
  const positions = [];
  for (let i = 0; i < nombre; i++) {
    positions.push(marge + i * (taille + ecart));
  }
  return positions;
}

console.log(rangee(4, 20, 10, 5));
```

```sortie
[ 5, 35, 65, 95 ]
```

| i | `i * (20 + 10)` | `+ 5` | x du carré |
|---|---|---|---|
| 0 | 0 | 5 | 5 |
| 1 | 30 | 35 | 35 |
| 2 | 60 | 65 | 65 |
| 3 | 90 | 95 | 95 |

Dans le navigateur, il ne reste qu'à dessiner :

```javascript norun
for (const x of rangee(4, 20, 10, 5)) {
  ctx.fillRect(x, 10, 20, 20);
}
```

Essayez : appelez `rangee(4, 20, 0, 5)`. L'écart est nul : `[ 5, 25, 45, 65 ]`,
les carrés se touchent et forment une seule bande.

> **À retenir :** séparez les calculs (fonctions pures, testables) des ordres de dessin
> (`ctx.fillRect`…). Le navigateur ne sert qu'à peindre.

## Animer : une étape à la fois

Un dessin animé, c'est une suite d'images fixes. Pour animer une balle, on répète
sans arrêt trois choses :

```text
 ┌──────────────────────────────────────────┐
 │ 1. effacer la toile         (clearRect)  │
 │ 2. calculer la position suivante (etape) │◀──┐
 │ 3. dessiner la balle        (arc, fill)  │   │
 └──────────────────────┬───────────────────┘   │
                        └── image suivante ─────┘
```

Dans le navigateur, `requestAnimationFrame` appelle votre fonction juste avant chaque
nouvelle image de l'écran (environ 60 fois par seconde) :

```javascript norun
let balle = { x: 20, y: 20, vx: 3, vy: 2 };

function image() {
  ctx.clearRect(0, 0, toile.width, toile.height);       // 1. effacer
  balle = etape(balle, toile.width, toile.height);      // 2. calculer
  ctx.beginPath();                                       // 3. dessiner
  ctx.arc(balle.x, balle.y, 5, 0, 2 * Math.PI);
  ctx.fill();
  requestAnimationFrame(image);                          // et on recommence
}

requestAnimationFrame(image);
```

### La fonction etape

La balle a une position (`x`, `y`) et une **vitesse** (`vx`, `vy`) : de combien elle
bouge à chaque image. Si le prochain pas la ferait sortir de la toile, on inverse la
vitesse : elle rebondit, comme une balle de ping-pong contre un mur.

```javascript
function etape(balle, largeur, hauteur) {
  let { x, y, vx, vy } = balle;
  if (x + vx < 0 || x + vx > largeur - 1) vx = -vx;   // mur gauche ou droit
  if (y + vy < 0 || y + vy > hauteur - 1) vy = -vy;   // plafond ou sol
  return { x: x + vx, y: y + vy, vx, vy };
}

let balle = { x: 0, y: 0, vx: 3, vy: 1 };
for (let i = 1; i <= 7; i++) {
  balle = etape(balle, 10, 3);
  console.log(i, balle.x, balle.y, balle.vx, balle.vy);
}
```

```sortie
1 3 1 3 1
2 6 2 3 1
3 9 1 3 -1
4 6 0 -3 -1
5 3 1 -3 1
6 0 2 -3 1
7 3 1 3 -1
```

La toile fait 10 cases de large (`x` de 0 à 9) et 3 de haut (`y` de 0 à 2). Suivons
chaque appel :

| Étape | x | y | vx | vy | Ce qui se passe |
|---|---|---|---|---|---|
| départ | 0 | 0 | 3 | 1 | |
| 1 | 3 | 1 | 3 | 1 | rien de spécial |
| 2 | 6 | 2 | 3 | 1 | la balle touche le sol (`y = 2`) |
| 3 | 9 | 1 | 3 | -1 | `2 + 1 = 3 > 2` : rebond sur le sol, `vy` devient `-1` |
| 4 | 6 | 0 | -3 | -1 | `9 + 3 = 12 > 9` : rebond sur le mur droit |
| 5 | 3 | 1 | -3 | 1 | `0 - 1 < 0` : rebond au plafond |
| 6 | 0 | 2 | -3 | 1 | la balle touche le mur gauche |
| 7 | 3 | 1 | 3 | -1 | deux rebonds d'un coup : mur gauche et sol |

À chaque étape, on vérifie **avant** de bouger. Ainsi la balle ne sort jamais.

### Voir la trajectoire

Sur une toile en caractères, on note le numéro de chaque étape à la place de la
balle :

```javascript
function etape(balle, largeur, hauteur) {
  let { x, y, vx, vy } = balle;
  if (x + vx < 0 || x + vx > largeur - 1) vx = -vx;
  if (y + vy < 0 || y + vy > hauteur - 1) vy = -vy;
  return { x: x + vx, y: y + vy, vx, vy };
}

const lignes = Array.from({ length: 3 }, () => Array(10).fill("."));
let balle = { x: 0, y: 0, vx: 3, vy: 1 };
lignes[balle.y][balle.x] = "0";
for (let i = 1; i <= 6; i++) {
  balle = etape(balle, 10, 3);
  lignes[balle.y][balle.x] = String(i);
}
console.log(lignes.map((l) => l.join("")).join("\n"));
```

```sortie
0.....4...
...5.....3
6.....2...
```

Lisez les chiffres dans l'ordre : `0`, `2`, `3`… Le `1` manque : la case de l'étape 1
a été recouverte par le `5` (même position `(3, 1)`), comme une peinture fraîche cache l'ancienne.

Essayez : démarrez avec `vx: 2`. La balle ne rebondit plus au même endroit, et le
chemin dessiné change.

> **À retenir :** une animation = effacer, calculer, dessiner, recommencer. La fonction
> `etape` est pure : vous pouvez la tester dans Node, étape par étape.

## Réagir à un clic

Un clic donne des coordonnées dans la **fenêtre**. Pour savoir où il tombe dans la
toile, on retire la position de la toile dans la page.

```javascript norun
toile.addEventListener("click", (event) => {
  const cadre = toile.getBoundingClientRect();  // où est la toile à l'écran
  const x = event.clientX - cadre.left;
  const y = event.clientY - cadre.top;
  console.log(caseCliquee(x, y, 20));
});
```

Trouver la case d'un damier est un calcul pur : on divise par la taille d'une case, et
on garde la partie entière avec `Math.floor`.

```javascript
function caseCliquee(x, y, taille) {
  return { colonne: Math.floor(x / taille), ligne: Math.floor(y / taille) };
}

console.log(caseCliquee(5, 5, 20));
console.log(caseCliquee(45, 12, 20));
console.log(caseCliquee(60, 99, 20));
```

```sortie
{ colonne: 0, ligne: 0 }
{ colonne: 2, ligne: 0 }
{ colonne: 3, ligne: 4 }
```

| Clic | `x / 20` | `y / 20` | Case |
|---|---|---|---|
| (5, 5) | 0.25 → 0 | 0.25 → 0 | colonne 0, ligne 0 |
| (45, 12) | 2.25 → 2 | 0.6 → 0 | colonne 2, ligne 0 |
| (60, 99) | 3 → 3 | 4.95 → 4 | colonne 3, ligne 4 |

> **À retenir :** convertissez d'abord le clic en coordonnées de la toile, puis
> laissez une fonction pure faire le reste.

## Erreurs fréquentes

### La toile est introuvable

```javascript norun
const toile = document.querySelector("#tolie");
const ctx = toile.getContext("2d");
```

```text
Uncaught TypeError: Cannot read properties of null (reading 'getContext')
```

`querySelector` n'a trouvé aucun élément `id="tolie"` (faute de frappe) et a renvoyé
`null`. Vérifiez l'`id` dans le HTML, et chargez le script après la toile
(`type="module"`).

### Un contexte mal nommé

```javascript norun
const ctx = toile.getContext("2D");
ctx.fillRect(0, 0, 10, 10);
```

```text
Uncaught TypeError: Cannot read properties of null (reading 'fillRect')
```

Le nom du contexte s'écrit en minuscules : `"2d"`. Avec un nom inconnu, `getContext`
renvoie `null`, sans erreur. L'erreur n'arrive qu'à la ligne suivante.

### Oublier beginPath

```javascript norun
ctx.arc(50, 50, 10, 0, 2 * Math.PI);
ctx.fill();
ctx.arc(150, 50, 10, 0, 2 * Math.PI);   // pas de beginPath avant
ctx.fill();
```

Il n'y a pas de message d'erreur. Mais un trait relie les deux cercles et la forme
est bizarre : le second `arc` s'est ajouté au **même** chemin. Commencez chaque forme
par `ctx.beginPath()`.

### Agrandir la toile avec le CSS

```html norun
<canvas id="toile" style="width: 600px; height: 300px"></canvas>
```

Pas d'erreur non plus, mais le dessin est flou et déformé. Sans `width` et `height`,
la toile fait 300 × 150 pixels ; le CSS l'étire comme une photo qu'on agrandit.
Donnez la taille avec les attributs `width="600" height="300"`.

## Testez-vous

**Question 1 :** dans une toile de 300 × 150, où se trouve le point `(0, 150)` ?

> **Réponse**
>
> En bas à gauche. `x = 0` est le bord gauche, et `y = 150` est tout en bas, car `y`
> descend.

**Question 2 :** une balle est en `x = 8` avec `vx = 3`, dans une toile de 10 de large
(`x` de 0 à 9). Que donne `etape` pour `x` et `vx` ?

> **Réponse**
>
> `8 + 3 = 11`, plus grand que `9` : la balle rebondit. `vx` devient `-3`, et `x`
> devient `8 - 3 = 5`.

**Question 3 :** écrivez une fonction `colonne(x, taille, ecart, marge)` qui donne
l'abscisse de la colonne numéro `x` d'une grille, puis affichez les 3 premières pour
une taille de 10, un écart de 2 et une marge de 4.

> **Réponse**
>
> ```javascript
> function colonne(x, taille, ecart, marge) {
>   return marge + x * (taille + ecart);
> }
>
> console.log([0, 1, 2].map((x) => colonne(x, 10, 2, 4)));
> ```
>
> ```sortie
> [ 4, 16, 28 ]
> ```

## En résumé

- `<canvas width height>` est une toile ; `getContext("2d")` donne le pinceau `ctx`.
- `(0, 0)` est en haut à gauche, et `y` augmente vers le bas.
- `fillRect`, `arc`, `lineTo`, `fillText` dessinent ; `beginPath` commence une forme.
- Rangez les calculs dans des fonctions pures et vérifiez-les dans Node, au besoin
  avec une toile en caractères.
- Une animation : effacer, calculer (`etape`), dessiner, avec `requestAnimationFrame`.

Suite : [garder des données dans le navigateur](stockage.md)
