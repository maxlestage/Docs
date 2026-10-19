# Suivre un programme pas à pas

Cette page vous apprend à exécuter un petit programme Rust dans votre tête, ligne par ligne, avec un tableau qui suit la valeur de chaque variable.

> **Objectifs :**
>
> - lire un programme ligne par ligne, dans l'ordre où l'ordinateur l'exécute ;
> - remplir un tableau de trace : une colonne par variable, une ligne par étape ;
> - suivre une condition, une boucle et un appel de fonction ;
> - deviner la sortie d'un programme **avant** de le lancer ;
> - faire afficher ses étapes au programme avec `println!` et `dbg!`.

## Pourquoi suivre un programme à la main

Un ordinateur ne devine rien. Il lit vos lignes **une par une**, de haut en bas, et
fait exactement ce qui est écrit. Pour comprendre un programme, le mieux est de faire
pareil : jouer l'ordinateur, avec un crayon.

Imaginez une recette de cuisine. Vous ne lisez pas toute la recette d'un coup : vous
faites l'étape 1, puis l'étape 2, et vous regardez ce qu'il y a dans le saladier après
chaque étape. Un programme, c'est pareil. Le saladier, ce sont les
[variables](../debuter/glossaire.md#variable).

Savoir faire cela vous servira tout le temps :

- pour **comprendre** un exemple que vous lisez ;
- pour **trouver un bug** : l'endroit où votre tableau et la réalité ne sont plus
  d'accord ;
- pour **réussir les exercices** sans tout essayer au hasard.

> **À retenir :** suivre un programme, c'est faire comme l'ordinateur : une ligne à la
> fois, en notant ce que vaut chaque variable après chaque ligne.

## La méthode du tableau de trace

Un **tableau de trace** a une colonne par variable et une ligne par étape. À chaque
ligne de code exécutée, vous notez les nouvelles valeurs. Une case vide veut dire :
« pas encore créée ».

```text
   le code                    le tableau
 ┌──────────────────┐     ┌───────┬────┬────┐
 │ let a = 2;       │ ──▶ │ Étape │ a  │ b  │
 │ let b = a + 1;   │     ├───────┼────┼────┤
 │ ...              │     │   1   │ 2  │    │
 └──────────────────┘     │   2   │ 2  │ 3  │
                          └───────┴────┴────┘
```

Trois règles simples :

1. **Une ligne de code à la fois.** Jamais deux.
2. **Lisez toujours la valeur actuelle** dans le tableau : la ligne du dessus.
3. **Une variable ne change que si une ligne la modifie** (avec `=` ou `+=`). Sinon,
   recopiez sa valeur.

Pour chaque programme de cette page, faites ceci :

1. lisez le code ;
2. **devinez** ce qui s'affiche, et écrivez-le ;
3. remplissez le tableau sur papier ;
4. comparez avec notre tableau, puis avec la sortie réelle.

> **À retenir :** une colonne par variable, une ligne par étape. La valeur à utiliser
> est toujours la plus récente.

## Programme 1 : des variables qui changent

Voici un petit tour de magie avec deux variables.

```rust
fn main() {
    let mut a = 3;
    let mut b = 5;
    a = a + b;
    b = a - b;
    a = a - b;
    println!("a = {a}, b = {b}");
}
```

Avant de lancer, devinez ce qui s'affiche. Écrivez votre réponse.

Le mot `mut` veut dire que la variable a le droit de changer
([mutable](../debuter/glossaire.md#mutable)). Suivons chaque ligne :

| Étape | Ligne | Calcul | a | b |
|-------|-------|--------|---|---|
| 1 | `let mut a = 3;` | | 3 | |
| 2 | `let mut b = 5;` | | 3 | 5 |
| 3 | `a = a + b;` | 3 + 5 = 8 | **8** | 5 |
| 4 | `b = a - b;` | 8 − 5 = 3 | 8 | **3** |
| 5 | `a = a - b;` | 8 − 3 = 5 | **5** | 3 |

```sortie
a = 5, b = 3
```

Les deux valeurs ont été **échangées**. Regardez l'étape 4 : `a` vaut déjà 8, pas 3.
On utilise toujours la valeur **la plus récente**, celle de la ligne du dessus.

C'est comme deux verres : pour échanger leur contenu sans troisième verre, on verse
tout dans le premier, puis on reprend la bonne quantité.

> **Essayez :** remplacez `let mut b = 5;` par `let mut b = 10;`. Refaites le tableau.
> Vous devez obtenir `a = 10, b = 3`.

> **À retenir :** à chaque étape, on lit les valeurs **actuelles** des variables, pas
> leurs valeurs de départ.

## Programme 2 : une condition

Une [condition](../debuter/glossaire.md#condition) choisit **un seul chemin**. Les
autres blocs sont sautés, comme un aiguillage de train.

```rust
fn main() {
    let temperature = 14;
    let mut tenue = "t-shirt";
    if temperature < 10 {
        tenue = "manteau";
    } else if temperature < 20 {
        tenue = "pull";
    }
    println!("{temperature} degrés : {tenue}");
}
```

Avant de lancer, devinez ce qui s'affiche.

Pour une condition, ajoutez une colonne « Test » : la question posée et sa réponse
(`true` ou `false`).

| Étape | Ligne | Test | temperature | tenue |
|-------|-------|------|-------------|-------|
| 1 | `let temperature = 14;` | | 14 | |
| 2 | `let mut tenue = "t-shirt";` | | 14 | t-shirt |
| 3 | `if temperature < 10` | 14 < 10 ? **false** | 14 | t-shirt |
| 4 | `else if temperature < 20` | 14 < 20 ? **true** | 14 | t-shirt |
| 5 | `tenue = "pull";` | | 14 | **pull** |
| 6 | `println!` | | 14 | pull |

```sortie
14 degrés : pull
```

La ligne `tenue = "manteau";` n'apparaît **pas** dans le tableau : son test était faux,
Rust saute ce bloc.

```text
          temperature < 10 ?
           /            \
        oui              non
         |                |
     "manteau"     temperature < 20 ?
                     /          \
                  oui            non
                   |              |
                 "pull"      (rien : reste "t-shirt")
```

> **Essayez :** mettez `let temperature = 25;`. Les deux tests sont faux : aucun bloc
> ne s'exécute, et le programme affiche `25 degrés : t-shirt`.

> **À retenir :** dans une condition, notez le test et sa réponse. Seul le bloc dont le
> test est vrai entre dans le tableau.

## Programme 3 : une boucle qui accumule

Une [boucle](../debuter/glossaire.md#boucle) répète un bloc. Dans le tableau, chaque
**tour** de boucle devient une ou plusieurs lignes.

```rust
fn main() {
    let mut total = 0;
    for i in 1..5 {
        total += i * 2;
    }
    println!("total = {total}");
}
```

Avant de lancer, devinez ce qui s'affiche. Attention à `1..5` !

L'intervalle `1..5` donne 1, 2, 3 et 4. Le 5 n'est **pas** inclus (pour l'inclure, il
faut `1..=5`).

| Tour | i | Calcul | total après le tour |
|------|---|--------|---------------------|
| avant | | | 0 |
| 1 | 1 | 0 + 1 × 2 | 2 |
| 2 | 2 | 2 + 2 × 2 | 6 |
| 3 | 3 | 6 + 3 × 2 | 12 |
| 4 | 4 | 12 + 4 × 2 | 20 |
| fin | | `i` vaudrait 5 : la boucle s'arrête | 20 |

```sortie
total = 20
```

`total` est comme une tirelire : elle est créée **avant** la boucle, une seule fois, et
chaque tour y glisse une pièce. Si on la créait dans la boucle, elle serait vidée à
chaque tour.

```text
 tirelire : [0] ──+2──▶ [2] ──+4──▶ [6] ──+6──▶ [12] ──+8──▶ [20]
 tour :            1           2           3             4
```

> **Essayez :** remplacez `1..5` par `1..=5`. Il y a un tour de plus (i = 5, +10) : le
> programme affiche `total = 30`.

> **À retenir :** pour une boucle, une ligne par tour. Notez la valeur de la variable
> de boucle **et** celle de l'accumulateur à la fin du tour.

## Programme 4 : une fonction appelée deux fois

Une [fonction](../debuter/glossaire.md#fonction) est une petite machine : on lui donne
une valeur, elle travaille dans **son propre coin**, puis elle rend un résultat.

```rust
fn prix_ttc(ht: i32) -> i32 {
    let taxe = ht / 5;
    ht + taxe
}

fn main() {
    let a = prix_ttc(100);
    let b = prix_ttc(30);
    println!("{a} + {b} = {}", a + b);
}
```

Avant de lancer, devinez ce qui s'affiche.

Le programme commence **toujours** par `main`. Quand `main` appelle `prix_ttc`, il se
met en pause et attend la réponse. Voici le dessin des appels :

```text
 main                          prix_ttc
 ────                          ────────
 a = prix_ttc(100) ──100──▶   ht = 100, taxe = 20
         ◀──────────120──────  renvoie 100 + 20
 a = 120

 b = prix_ttc(30)  ──30───▶   ht = 30, taxe = 6
         ◀──────────36───────  renvoie 30 + 6
 b = 36

 println! affiche 120 + 36 = 156
```

Dans le tableau, une colonne « Où ? » dit dans quelle fonction on se trouve :

| Étape | Où ? | Ligne | ht | taxe | a | b |
|-------|------|-------|----|------|---|---|
| 1 | main | `prix_ttc(100)` : appel | | | | |
| 2 | prix_ttc | `ht` reçoit 100 | 100 | | | |
| 3 | prix_ttc | `let taxe = ht / 5;` | 100 | 20 | | |
| 4 | prix_ttc | `ht + taxe` : renvoie 120 | 100 | 20 | | |
| 5 | main | `let a = …` | | | 120 | |
| 6 | main | `prix_ttc(30)` : appel | | | 120 | |
| 7 | prix_ttc | `ht` reçoit 30 | 30 | | 120 | |
| 8 | prix_ttc | `let taxe = ht / 5;` | 30 | 6 | 120 | |
| 9 | prix_ttc | `ht + taxe` : renvoie 36 | 30 | 6 | 120 | |
| 10 | main | `let b = …` | | | 120 | 36 |

```sortie
120 + 36 = 156
```

Le `println!` affiche `{a}` et `{b}`, donc les prix **avec** la taxe (120 et 36), pas
100 et 30. Si vous aviez deviné `100 + 30 = …`, vous avez lu les arguments au lieu des
résultats.

Deux choses à remarquer :

- `ht` et `taxe` **disparaissent** quand la fonction a fini (cases vides aux étapes 5
  et 10). Elles n'existent que pendant l'appel ;
- au deuxième appel, `ht` et `taxe` repartent de zéro : la fonction ne se souvient pas
  du premier appel.

> **Essayez :** ajoutez `let c = prix_ttc(a);` avant le `println!`, et affichez `c`.
> La fonction reçoit 120, la taxe vaut 24 : `c` vaut 144.

> **À retenir :** à chaque appel, on « entre » dans la fonction avec la valeur donnée,
> on la suit jusqu'au résultat, puis on revient à l'endroit exact de l'appel.

## Programme 5 : une liste et une condition dans la boucle

Dernier programme : une boucle sur une [liste](../debuter/glossaire.md#liste) (un
`Vec`), avec une condition **dans** la boucle. On compte les grosses dépenses (plus de
10 €) et on les additionne.

```rust
fn main() {
    let depenses = vec![4, 25, 12, 7, 30];
    let mut grosses = 0;
    let mut somme = 0;
    for d in &depenses {
        if *d > 10 {
            grosses += 1;
            somme += d;
        }
    }
    println!("{grosses} grosses dépenses, total {somme}");
}
```

Avant de lancer, devinez ce qui s'affiche.

`for d in &depenses` prend les cases **une par une**, de gauche à droite. Le `&` veut
dire « regarder sans prendre » : la liste reste utilisable après. L'étoile de `*d` lit
le nombre à travers ce `&`.

```text
 depenses : [ 4 | 25 | 12 |  7 | 30 ]
              ▲    ▲    ▲    ▲    ▲
 tour :       1    2    3    4    5
```

À chaque tour, il y a un test. Ajoutez une colonne pour lui :

| Tour | d | d > 10 ? | grosses | somme |
|------|---|----------|---------|-------|
| avant | | | 0 | 0 |
| 1 | 4 | false | 0 | 0 |
| 2 | 25 | **true** | 1 | 25 |
| 3 | 12 | **true** | 2 | 37 |
| 4 | 7 | false | 2 | 37 |
| 5 | 30 | **true** | 3 | 67 |

```sortie
3 grosses dépenses, total 67
```

Aux tours 1 et 4, le test est faux : `grosses` et `somme` ne bougent pas. Vous
**recopiez** simplement leurs valeurs dans le tableau.

> **Essayez :** remplacez `*d > 10` par `*d >= 12`. Le 12 compte toujours, mais pas
> le 4 ni le 7 : la sortie ne change pas. Avec `*d > 12`, en revanche, vous obtenez
> `2 grosses dépenses, total 55`.

> **À retenir :** quand le test d'un tour est faux, les variables gardent leur valeur.
> Recopiez-la : ne sautez pas la ligne du tableau.

## Faire afficher ses étapes au programme

Votre tableau est-il juste ? Le programme peut vous le dire lui-même : il suffit de lui
faire afficher ses variables pendant qu'il travaille. C'est comme demander à un
cuisinier de dire à voix haute chaque geste qu'il fait.

### Avec println! et une étiquette

Ajoutez un `println!` dans la boucle, avec une **étiquette** pour savoir d'où vient
chaque ligne :

```rust
fn main() {
    let mut total = 0;
    for i in 1..5 {
        total += i * 2;
        println!("[tour {i}] total = {total}");
    }
    println!("total = {total}");
}
```

```sortie
[tour 1] total = 2
[tour 2] total = 6
[tour 3] total = 12
[tour 4] total = 20
total = 20
```

Ce sont exactement les lignes de notre tableau du programme 3. Sans l'étiquette
`[tour …]`, vous verriez juste des nombres, sans savoir lequel est lequel.

### Avec dbg!

Rust a une [macro](../debuter/glossaire.md#macro) faite pour cela : `dbg!`. Elle
affiche le nom du fichier, le numéro de ligne, le nom de la variable **et** sa valeur.
Vous n'avez pas à écrire d'étiquette.

```rust
fn main() {
    let mut total = 0;
    for i in 1..5 {
        total += i * 2;
        dbg!(i, total);
    }
    println!("total = {total}");
}
```

```sortie
total = 20
```

La sortie ci-dessus ne contient qu'une ligne ! C'est normal : `dbg!` n'écrit **pas**
sur la sortie normale (*stdout*), mais sur la **sortie d'erreur** (*stderr*). Dans
votre terminal, avec `cargo run` (le fichier est alors `src/main.rs`), vous voyez les
deux mélangées :

```text
[src/main.rs:5:9] i = 1
[src/main.rs:5:9] total = 2
[src/main.rs:5:9] i = 2
[src/main.rs:5:9] total = 6
[src/main.rs:5:9] i = 3
[src/main.rs:5:9] total = 12
[src/main.rs:5:9] i = 4
[src/main.rs:5:9] total = 20
total = 20
```

`[src/main.rs:5:9]` veut dire : fichier `src/main.rs`, ligne 5, colonne 9.

Pourquoi deux sorties ? Imaginez deux tuyaux qui sortent du programme. Le premier
porte les **résultats**, le second les **messages pour vous**. Si vous envoyez les
résultats dans un fichier (`cargo run > resultat.txt`), les messages de `dbg!`
restent à l'écran et ne salissent pas le fichier.

```text
              ┌─────────────┐  stdout (println!) ──▶ résultats
 programme ──▶│             │
              └─────────────┘  stderr (dbg!)     ──▶ messages de débogage
```

Bonus : `dbg!` **renvoie** la valeur qu'on lui donne. Vous pouvez l'entourer autour
d'un calcul sans rien casser :

```rust
fn prix_ttc(ht: i32) -> i32 {
    let taxe = ht / 5;
    dbg!(ht + taxe)
}

fn main() {
    let a = prix_ttc(100);
    println!("a = {a}");
}
```

```sortie
a = 120
```

Et sur la sortie d'erreur, en plus :

```text
[src/main.rs:3:5] ht + taxe = 120
```

> **Essayez :** dans le programme 5, ajoutez `dbg!(d, grosses, somme);` à la fin du
> bloc de la boucle. Lancez avec `cargo run` : vous obtenez votre tableau de trace,
> écrit par le programme lui-même.

> **À retenir :** `println!` avec une étiquette, ou `dbg!(variable)`, transforment le
> programme en tableau de trace. `dbg!` écrit sur *stderr* : pensez à l'enlever quand
> vous avez trouvé.

## Et le débogueur ?

Un [débogueur](../debuter/glossaire.md#debogage) met le programme en pause sur une
ligne et vous montre toutes les variables, sans rien ajouter au code. Il fait votre
tableau de trace à votre place, une ligne à la fois : voyez
[Un débogueur dans VS Code](debogage.md#un-debogueur-dans-vs-code), dans le chapitre
[Déboguer en Rust](debogage.md).

## Erreurs fréquentes

### Oublier une étape

Dans le programme 5, on saute souvent les tours où « il ne se passe rien ». Le tableau
devient alors faux, car on ne sait plus où on en est dans la liste.

```text
Tableau incomplet (faux) :     Tableau complet (juste) :
| d  | grosses | somme |       | d  | grosses | somme |
| 25 | 1       | 25    |       | 4  | 0       | 0     |
| 12 | 2       | 37    |       | 25 | 1       | 25    |
| 30 | 3       | 67    |       | 12 | 2       | 37    |
                               | 7  | 2       | 37    |
                               | 30 | 3       | 67    |
```

Correction : **une ligne par tour**, même si rien ne change. Recopiez les valeurs.

### Réutiliser une ancienne valeur

Dans le programme 1, à la ligne `b = a - b;`, beaucoup calculent 3 − 5 avec le `a` du
départ. Mais `a` vaut déjà 8 :

```text
Faux : b = 3 - 5 = -2     (le "a" de l'étape 1)
Juste : b = 8 - 5 = 3     (le "a" de l'étape 3, la ligne juste au-dessus)
```

Correction : lisez toujours la valeur dans la **dernière** ligne du tableau.

### Un tour de trop (ou de moins)

`for i in 1..5` ne fait que **4** tours. Compter jusqu'à 5 donne un mauvais total
(30 au lieu de 20) :

```text
1..5   → 1, 2, 3, 4        (le 5 est exclu)
1..=5  → 1, 2, 3, 4, 5     (le 5 est inclus)
```

Correction : avant de remplir le tableau, écrivez la liste des valeurs que prendra la
variable de boucle.

### Oublier mut sur une variable qui change

En recopiant un programme pour le suivre, on oublie parfois `mut`. Rust refuse alors
de le compiler :

```rust norun
fn main() {
    let total = 0;
    for i in 1..5 {
        total += i * 2;
    }
    println!("total = {total}");
}
```

```text
error[E0384]: cannot assign twice to immutable variable `total`
 --> src/main.rs:4:9
  |
2 |     let total = 0;
  |         ----- first assignment to `total`
3 |     for i in 1..5 {
4 |         total += i * 2;
  |         ^^^^^^^^^^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
2 |     let mut total = 0;
  |         +++
```

Correction : `let mut total = 0;`. Dans votre tableau aussi, c'est un bon repère :
seules les variables `mut` peuvent changer de colonne en colonne.

### Donner la liste à dbg! au lieu de la prêter

`dbg!(depenses)` **prend** la liste (on dit qu'elle est
[déplacée](possession.md)). Ensuite, elle n'est plus utilisable :

```rust norun
fn main() {
    let depenses = vec![4, 25, 12];
    dbg!(depenses);
    println!("{}", depenses.len());
}
```

```text
error[E0382]: borrow of moved value: `depenses`
 --> src/main.rs:4:20
  |
2 |     let depenses = vec![4, 25, 12];
  |         -------- move occurs because `depenses` has type `Vec<i32>`, which does not implement the `Copy` trait
3 |     dbg!(depenses);
  |     -------------- value moved here
4 |     println!("{}", depenses.len());
  |                    ^^^^^^^^ value borrowed here after move
  |
help: consider borrowing instead of transferring ownership
  |
3 |     dbg!(&depenses);
  |          +
```

Correction : prêtez-la avec `&` : `dbg!(&depenses);`.

## Testez-vous

**1.** Que vaut `x` à la fin de ces trois lignes ?
`let mut x = 2; x = x * 3; x = x - 1;`

> **Réponse**
>
> | Étape | x |
> |-------|---|
> | 1 | 2 |
> | 2 | 2 × 3 = 6 |
> | 3 | 6 − 1 = 5 |
>
> `x` vaut **5** : chaque ligne part de la valeur de la ligne d'avant.

**2.** Combien de tours fait `for i in 0..3`, et quelles valeurs prend `i` ?

> **Réponse**
>
> Trois tours : `i` vaut 0, puis 1, puis 2. Le 3 est exclu.

**3.** Qu'affiche ce programme ? Faites le tableau avant de regarder la réponse.

```rust norun
fn main() {
    let mut compte = 0;
    for n in [3, 8, 5, 10] {
        if n % 2 == 0 {
            compte += n;
        }
    }
    println!("{compte}");
}
```

> **Réponse**
>
> | Tour | n | n pair ? | compte |
> |------|---|----------|--------|
> | 1 | 3 | false | 0 |
> | 2 | 8 | true | 8 |
> | 3 | 5 | false | 8 |
> | 4 | 10 | true | 18 |
>
> Le programme additionne les nombres pairs. Le voici avec sa sortie :
>
> ```rust
> fn main() {
>     let mut compte = 0;
>     for n in [3, 8, 5, 10] {
>         if n % 2 == 0 {
>             compte += n;
>         }
>     }
>     println!("{compte}");
> }
> ```
>
> ```sortie
> 18
> ```

## En résumé

- Suivre un programme, c'est jouer l'ordinateur : une ligne à la fois, de haut en bas.
- Le tableau de trace a une colonne par variable et une ligne par étape (ou par tour).
- Utilisez toujours la valeur **la plus récente** ; recopiez les valeurs qui ne
  changent pas.
- Pour une fonction, entrez dedans avec la valeur reçue, suivez-la, puis revenez au
  point d'appel avec le résultat.
- `println!` avec une étiquette ou `dbg!` font afficher ses étapes au programme ;
  `dbg!` écrit sur la sortie d'erreur (*stderr*).

Suite : [Les pièges du débutant en Rust](pieges.md)
