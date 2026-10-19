# Rust dans le navigateur avec WebAssembly

WebAssembly permet de compiler du Rust en un fichier `.wasm` que le navigateur exécute, et d'appeler vos fonctions Rust depuis JavaScript comme des fonctions ordinaires.

> **Objectifs :**
>
> - comprendre ce qu'est WebAssembly et le chemin Rust → `.wasm` → navigateur ;
> - préparer un projet avec `wasm-bindgen` et le compiler avec `wasm-pack` ;
> - appeler une fonction Rust depuis JavaScript ;
> - savoir ce qui traverse la frontière : nombres, textes, listes ;
> - écrire la logique en Rust pur et la tester avec `cargo run` avant de l'exporter.

## Qu'est-ce que WebAssembly ?

Un navigateur comprend normalement un seul langage de programmation : JavaScript.
**WebAssembly** (*Wasm*) est un deuxième langage qu'il comprend. Personne ne l'écrit à
la main : c'est un code machine compact, fabriqué par un
[compilateur](../debuter/glossaire.md#compilateur), comme celui de Rust.

Pensez à une prise électrique de voyage. Votre appareil (le code Rust) a une prise
française ; le pays (le navigateur) a ses propres prises. L'adaptateur (`.wasm` et un
peu de JavaScript) permet de brancher l'un dans l'autre.

```text
  Votre code             Compilation               Le navigateur
 ┌───────────┐   cargo /     ┌───────────────┐   charge  ┌──────────────────┐
 │  lib.rs   │ ────────────► │ calcul_bg.wasm│ ────────► │ Chrome, Firefox, │
 │  (Rust)   │   wasm-pack   │ calcul.js     │           │ Safari...        │
 └───────────┘               └───────────────┘           │  + votre page JS │
                                                         └──────────────────┘
```

- Le fichier `.wasm` contient votre code Rust compilé.
- Le petit fichier `.js` est la **colle** : il charge le `.wasm` et traduit les valeurs
  entre JavaScript et Rust.

À quoi ça sert ? Aux parties **lourdes** d'une page : un jeu, un éditeur de photos, la
compression d'un fichier, des calculs sur beaucoup de données. Le reste de la page
(les boutons, le texte) reste en JavaScript.

> **À retenir :** WebAssembly est un second langage du navigateur ; Rust peut s'y
> compiler. Le `.wasm` fait le calcul, la colle JavaScript fait le lien.

## Commencer par la logique en Rust pur

Avant de penser au navigateur, écrivez vos fonctions comme d'habitude, et vérifiez-les
avec un `main`. C'est la partie la plus importante : si elle est juste ici, elle sera
juste dans le navigateur.

Notre exemple : une fonction `force` qui donne une note de 0 à 4 à un mot de passe. Un
point par règle respectée.

```rust
fn force(mot_de_passe: &str) -> u32 {
    let mut points = 0;
    if mot_de_passe.chars().count() >= 8 {
        points += 1;
    }
    if mot_de_passe.chars().any(|c| c.is_uppercase()) {
        points += 1;
    }
    if mot_de_passe.chars().any(|c| c.is_ascii_digit()) {
        points += 1;
    }
    if mot_de_passe.chars().any(|c| !c.is_alphanumeric()) {
        points += 1;
    }
    points
}

fn main() {
    println!("chat : {}", force("chat"));
    println!("chaton2024 : {}", force("chaton2024"));
    println!("Chaton2024! : {}", force("Chaton2024!"));
}
```

```sortie
chat : 0
chaton2024 : 2
Chaton2024! : 4
```

### Pas à pas avec « Chaton2024! »

| Étape | Règle | Vraie ? | `points` après |
|-------|-------|---------|---------------:|
| début | | | 0 |
| 1 | au moins 8 caractères (il y en a 11) | oui | 1 |
| 2 | une majuscule (`C`) | oui | 2 |
| 3 | un chiffre (`2`) | oui | 3 |
| 4 | un caractère qui n'est ni lettre ni chiffre (`!`) | oui | 4 |

Avec `"chaton2024"` : 10 caractères (+1), pas de majuscule, des chiffres (+1), pas de
symbole. Total : 2.

- `chars().count()` compte les caractères (voir [Le texte](texte.md)).
- `any(|c| …)` répond `true` dès qu'**un** caractère vérifie la
  [closure](../debuter/glossaire.md#closure) (voir [Les itérateurs](iterateurs.md)).

Essayez : appelez `force("Été2024!")`. Le résultat est 4 : `É` est bien une majuscule,
car `is_uppercase` connaît les lettres accentuées.

> **À retenir :** écrivez et testez la logique en Rust normal, avec `cargo run`. Le
> navigateur viendra après.

## Tester avant d'exporter

Ajoutons deux autres fonctions que la page web utilisera, et vérifions tout avec
`assert_eq!` (voir [Tester son code](tests.md)) :

```rust
fn additionner(a: i32, b: i32) -> i32 {
    a + b
}

fn saluer(nom: &str) -> String {
    format!("Bonjour, {nom} !")
}

fn moyenne(notes: &[f64]) -> f64 {
    if notes.is_empty() {
        return 0.0;
    }
    notes.iter().sum::<f64>() / notes.len() as f64
}

fn main() {
    assert_eq!(additionner(2, 3), 5);
    assert_eq!(saluer("Léa"), "Bonjour, Léa !");
    assert_eq!(moyenne(&[]), 0.0);
    println!("{}", moyenne(&[12.0, 15.5, 9.0]));
    println!("Tous les tests passent");
}
```

```sortie
12.166666666666666
Tous les tests passent
```

La boucle cachée dans `moyenne(&[12.0, 15.5, 9.0])`, pas à pas :

| Étape | Note lue | Somme |
|-------|---------:|------:|
| début | | 0.0 |
| 1 | 12.0 | 12.0 |
| 2 | 15.5 | 27.5 |
| 3 | 9.0 | 36.5 |
| fin | 36.5 / 3 | 12.166666666666666 |

Le test `moyenne(&[]) == 0.0` protège d'un piège : sans le `if`, on diviserait 0.0
par 0, et le résultat serait `NaN` (« pas un nombre »).

Essayez : supprimez le `if notes.is_empty()`. Le programme s'arrête sur
`assertion `left == right` failed` avec `left: NaN` et `right: 0.0`.

> **À retenir :** les tests tournent sur votre ordinateur, sans navigateur. Une erreur
> trouvée ici est dix fois plus simple à corriger qu'une erreur dans la page.

## Préparer le projet avec wasm-bindgen

Le Rust est prêt. Il faut maintenant le **montrer** à JavaScript. C'est le travail de
la [crate](../debuter/glossaire.md#crate) `wasm-bindgen`.

Créez une bibliothèque (pas un programme : il n'y a pas de `main`) :

```sh
cargo new --lib calcul
cd calcul
```

Puis complétez `Cargo.toml` :

```toml
[package]
name = "calcul"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
```

- `crate-type = ["cdylib"]` demande une bibliothèque qu'**un autre langage** peut
  charger, ici JavaScript.
- `wasm-bindgen` écrit la colle entre les deux langages.

Dans `src/lib.rs`, on reprend **les mêmes fonctions**. Seuls changent `pub` (visible
de l'extérieur) et l'étiquette `#[wasm_bindgen]` (« à exporter vers JavaScript ») :

```rust norun
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn additionner(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn saluer(nom: &str) -> String {
    format!("Bonjour, {nom} !")
}

#[wasm_bindgen]
pub fn force(mot_de_passe: &str) -> u32 {
    let mut points = 0;
    if mot_de_passe.chars().count() >= 8 {
        points += 1;
    }
    if mot_de_passe.chars().any(|c| c.is_uppercase()) {
        points += 1;
    }
    if mot_de_passe.chars().any(|c| c.is_ascii_digit()) {
        points += 1;
    }
    if mot_de_passe.chars().any(|c| !c.is_alphanumeric()) {
        points += 1;
    }
    points
}

#[wasm_bindgen]
pub fn moyenne(notes: &[f64]) -> f64 {
    if notes.is_empty() {
        return 0.0;
    }
    notes.iter().sum::<f64>() / notes.len() as f64
}
```

> **Note :** `#[wasm_bindgen]` est une [macro](../debuter/glossaire.md#macro) : elle
> écrit pour vous le code qui traduit les arguments à l'entrée et le résultat à la
> sortie.

> **À retenir :** même code qu'avant, plus `pub` et `#[wasm_bindgen]` devant chaque
> fonction à exporter.

## Compiler avec wasm-pack

`wasm-pack` fait tout en une commande : il compile vers la cible Wasm, appelle
`wasm-bindgen` et range le résultat dans un dossier `pkg/`. On l'installe une fois :

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
```

Puis, dans le dossier du projet :

```sh
wasm-pack build --target web
```

```text
[INFO]: Checking for the Wasm target...
[INFO]: Compiling to Wasm...
   Compiling wasm-bindgen v0.2.129
   Compiling calcul v0.1.0 (/home/lea/calcul)
    Finished `release` profile [optimized] target(s) in 11.33s
[INFO]: Installing wasm-bindgen...
[INFO]: Optional fields missing from Cargo.toml: 'description', 'repository', and 'license'. These are not necessary, but recommended
[INFO]: :-) Done in 11.72s
[INFO]: :-) Your wasm pkg is ready to publish at /home/lea/calcul/pkg.
```

Le dossier contient alors :

```text
calcul/
├── Cargo.toml
├── src/
│   └── lib.rs
└── pkg/                    ← fabriqué par wasm-pack
    ├── calcul_bg.wasm      ← votre Rust compilé (environ 28 Ko)
    ├── calcul.js           ← la colle JavaScript
    ├── calcul.d.ts         ← les types, pour TypeScript
    └── package.json
```

`--target web` produit un module que le navigateur charge directement. Avec
`--target nodejs`, le même code se charge dans [Node.js](../debuter/glossaire.md#node-js).

> **Note :** la ligne `Optional fields missing` n'est qu'un conseil, pas une erreur.

> **À retenir :** `wasm-pack build --target web` → un dossier `pkg/` avec le `.wasm`
> et sa colle `.js`.

## Appeler Rust depuis JavaScript

Dans la page, un fichier `main.js` importe les fonctions de `pkg/calcul.js`. Il faut
d'abord appeler `init()` : elle télécharge le `.wasm` et le prépare.

```javascript norun
import init, { additionner, saluer, force, moyenne } from "./pkg/calcul.js";

await init();

console.log(additionner(2, 3));
console.log(saluer("Léa"));
console.log(force("Chaton2024!"));
console.log(moyenne(new Float64Array([12, 15.5, 9])));
```

La page HTML le charge avec `<script type="module" src="main.js"></script>`. Ouvrez la
console du navigateur (touche F12) :

```text
5
Bonjour, Léa !
4
12.166666666666666
```

Les mêmes résultats qu'avec `cargo run` : c'est le même code.

Pour voir la page, servez le dossier avec un petit serveur web, par exemple
`python3 -m http.server`, puis ouvrez `http://localhost:8000`. Un double-clic sur le
fichier HTML ne suffit pas : le navigateur refuse de charger un module depuis le disque.

Pour brancher `force` sur un vrai champ de saisie, on lit le texte tapé à chaque touche
(voir [le DOM en JavaScript](../javascript/dom.md)) :

```javascript norun
const champ = document.querySelector("#mot-de-passe");
const note = document.querySelector("#note");

champ.addEventListener("input", () => {
  note.textContent = `Force : ${force(champ.value)} / 4`;
});
```

Ce qui se passe quand l'utilisateur tape `Chat1` :

| Touche | `champ.value` | `force(…)` | Affiché |
|--------|---------------|-----------:|---------|
| `C` | `C` | 1 | Force : 1 / 4 |
| `h` | `Ch` | 1 | Force : 1 / 4 |
| `a` | `Cha` | 1 | Force : 1 / 4 |
| `t` | `Chat` | 1 | Force : 1 / 4 |
| `1` | `Chat1` | 2 | Force : 2 / 4 |

> **À retenir :** `import init, { … }`, puis `await init()`, puis vos fonctions Rust
> s'appellent comme des fonctions JavaScript.

## Ce qui traverse la frontière

JavaScript et Wasm ne partagent pas les mêmes valeurs. Imaginez une douane : chaque
valeur qui passe est contrôlée et parfois **transformée**.

### Les nombres : ils passent tels quels

Wasm ne connaît que des nombres. Un `i32`, un `u32`, un `f64` passent directement. Le
fichier `calcul.d.ts` le montre : tous deviennent `number` en JavaScript.

```text
export function additionner(a: number, b: number): number;
export function force(mot_de_passe: string): number;
export function saluer(nom: string): string;
export function moyenne(notes: Float64Array): number;
```

Mais un `number` JavaScript est toujours un `f64`. Quand Rust attend un `i32`, la
colle le **convertit** :

| Appel JavaScript | Ce que Rust reçoit | Résultat |
|------------------|--------------------|---------:|
| `additionner(2, 3)` | `a = 2`, `b = 3` | 5 |
| `additionner(2.7, 1)` | `a = 2` (virgule coupée) | 3 |
| `additionner(2147483647, 1)` | le plus grand `i32`, puis + 1 | -2147483648 |

Le dernier résultat est surprenant. Un `i32` ne dépasse pas 2 147 483 647 ; dans le
`.wasm` compilé en mode `release`, le calcul fait le tour, comme un compteur kilométrique
qui repasse à zéro. Le Rust ordinaire montre le même tour avec `wrapping_add` (voir
[Les nombres](nombres.md)) :

```rust
fn main() {
    let max = i32::MAX;
    println!("{max}");
    println!("{}", max.wrapping_add(1));
    println!("{:?}", max.checked_add(1));
}
```

```sortie
2147483647
-2147483648
None
```

Si de très grands nombres sont possibles, prenez `i64` ou `f64`, ou utilisez
`checked_add`, qui répond `None` au lieu de faire le tour.

### Les textes : copiés dans la mémoire du Wasm

Le `.wasm` a sa propre mémoire, comme une boîte fermée. Pour lui donner un texte, la
colle JavaScript le **copie** dans cette boîte, en UTF-8, puis donne à Rust deux
nombres : **où** il commence et **combien** d'[octets](../debuter/glossaire.md#octet)
il fait. C'est pour ça que `force` reçoit `(a: number, b: number)` dans les
coulisses de `calcul.d.ts`.

```text
  JavaScript : "Léa"
        │  copie en UTF-8
        ▼
  mémoire du Wasm :  ... │ L │ é │ é │ a │ ...
                           ▲   └─2 octets─┘
                        position 1024, longueur 4
        │
        ▼
  Rust reçoit nom: &str = "Léa"
```

Rust compte ces octets avec `len()`, et les caractères avec `chars().count()` :

```rust
fn main() {
    let nom = "Léa";
    println!("octets : {}", nom.len());
    println!("caractères : {}", nom.chars().count());
    for (i, octet) in nom.bytes().enumerate() {
        println!("{i} : {octet}");
    }
}
```

```sortie
octets : 4
caractères : 3
0 : 76
1 : 195
2 : 169
3 : 97
```

Le `é` occupe deux octets (195 et 169). Pour le retour (`saluer` renvoie un `String`),
c'est l'inverse : la colle relit les octets dans la boîte et fabrique un texte
JavaScript.

> **Note :** chaque traversée copie le texte. Pour une fonction appelée un million de
> fois, mieux vaut envoyer tout le travail en une seule fois.

### Les listes de nombres : des tableaux typés

`&[f64]` devient un `Float64Array` (un tableau JavaScript de `f64`), `&[i32]` un
`Int32Array`, `&[u8]` un `Uint8Array`. Comme pour les textes, le tableau est copié dans
la mémoire du Wasm.

### Le reste : pas directement

Une `HashMap`, un `Vec<String>` en retour, vos propres structures… ne traversent pas
toutes seules. Il faut les annoter avec `#[wasm_bindgen]`, ou les transformer en JSON
avec `serde` (voir [JSON, CSV et TOML](formats.md)).

| Type Rust | Type JavaScript | Comment ça passe |
|-----------|-----------------|------------------|
| `i32`, `u32`, `f64` | `number` | directement |
| `bool` | `boolean` | directement |
| `&str`, `String` | `string` | copie en UTF-8 |
| `&[f64]`, `Vec<f64>` | `Float64Array` | copie |
| `HashMap`, structure | — | `#[wasm_bindgen]` ou JSON |

> **À retenir :** les nombres passent tels quels ; les textes et les listes sont copiés
> à chaque traversée ; le reste demande un effort.

## Erreurs fréquentes

### Oublier #[wasm_bindgen]

```rust norun
pub fn force(mot_de_passe: &str) -> u32 {
    // ...
}
```

Le projet compile, mais la page plante au chargement :

```text
SyntaxError: The requested module './pkg/calcul.js' does not provide an export named 'force'
```

Sans l'étiquette, la fonction reste dans le Rust : la colle ne l'exporte pas. Ajoutez
`#[wasm_bindgen]` au-dessus, puis relancez `wasm-pack build --target web`.

### Renvoyer un type qui ne traverse pas

```rust norun
use std::collections::HashMap;

#[wasm_bindgen]
pub fn compter(mots: Vec<String>) -> HashMap<String, u32> {
    // ...
}
```

```text
error[E0277]: the trait bound `HashMap<String, u32>: IntoWasmAbi` is not satisfied
  --> src/lib.rs:38:1
   |
38 | #[wasm_bindgen]
   | ^^^^^^^^^^^^^^^ the trait `IntoWasmAbi` is not implemented for `HashMap<String, u32>`
```

`IntoWasmAbi` veut dire « sait traverser la frontière ». Une `HashMap` ne sait pas.
Renvoyez un texte JSON (`serde_json::to_string`), ou un type plus simple.

### Une panique dans le Wasm

```rust norun
#[wasm_bindgen]
pub fn diviser(a: i32, b: i32) -> i32 {
    a / b
}
```

```javascript norun
console.log(diviser(10, 2));
console.log(diviser(10, 0));
```

```text
5
RuntimeError: unreachable
```

La division par zéro fait [paniquer](../debuter/glossaire.md#panique) Rust. Dans le
navigateur, la panique devient ce `RuntimeError: unreachable`, peu parlant. Vérifiez les
cas dangereux en Rust et renvoyez une valeur claire, par exemple `Option<i32>`, qui
devient `number | undefined` en JavaScript.

### Envoyer un nombre là où Rust attend un texte

```javascript norun
console.log(saluer(42));
```

```text
RuntimeError: memory access out of bounds
```

La colle croit recevoir un texte et se trompe en le copiant. JavaScript ne vérifie pas
les types : TypeScript, avec le fichier `calcul.d.ts`, aurait signalé l'erreur avant de
lancer (voir [le parcours TypeScript](../typescript/introduction.md)).

## Testez-vous

1. À quoi sert le fichier `calcul.js` produit par `wasm-pack` ?

> **Réponse**
>
> C'est la colle : il charge `calcul_bg.wasm` et traduit les valeurs entre JavaScript
> et Rust (par exemple, il copie les textes dans la mémoire du Wasm).

2. Pourquoi tester `force` avec un `main` avant de compiler en Wasm ?

> **Réponse**
>
> Parce que c'est plus simple et plus rapide : `cargo run` affiche le résultat tout de
> suite, avec des messages d'erreur clairs. Dans le navigateur, une panique devient un
> `RuntimeError: unreachable` difficile à comprendre.

3. Que reçoit vraiment la fonction Rust quand JavaScript lui passe `"Léa"` ?

> **Réponse**
>
> Une copie du texte en UTF-8 dans la mémoire du Wasm : une position et une longueur
> (4 octets, car `é` en prend deux). Rust la voit comme un `&str`.

## En résumé

- WebAssembly est un second langage du navigateur ; Rust se compile en `.wasm`.
- Écrivez et testez la logique en Rust pur, avec `main` et `assert_eq!`.
- Pour exporter : `crate-type = ["cdylib"]`, la crate `wasm-bindgen`, `pub` et
  `#[wasm_bindgen]` devant chaque fonction.
- `wasm-pack build --target web` fabrique `pkg/` ; en JavaScript : `await init()`, puis
  on appelle les fonctions.
- Les nombres passent tels quels, les textes et les tableaux sont copiés ; une
  `HashMap` ne traverse pas sans aide.

Suite : [Écrire du code rapide](performance.md)
