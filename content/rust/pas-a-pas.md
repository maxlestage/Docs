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
