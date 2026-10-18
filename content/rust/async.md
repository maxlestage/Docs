# La programmation asynchrone

Avec `async` et `.await`, un seul programme s'occupe de nombreuses attentes à la fois (réseau, disque, minuteries) sans lancer un thread pour chacune.

> **Objectifs :**
>
> - comprendre la différence entre attendre et calculer ;
> - écrire une fonction `async` et l'attendre avec `.await` ;
> - savoir qu'une future ne fait rien tant qu'on ne l'attend pas ;
> - utiliser le runtime tokio : `#[tokio::main]`, `spawn`, `join!`, `sleep`, `select!` ;
> - choisir entre threads et async.

## Attendre ou calculer ?

Un programme passe son temps à deux sortes de choses :

- **calculer** : additionner, trier, compresser une image… Le processeur travaille ;
- **attendre** : une réponse d'un site web, un fichier sur le disque, un message
  réseau… Le processeur ne fait rien, il patiente.

Imaginez un cuisinier avec plusieurs casseroles. Il met l'eau du riz à chauffer. Il ne
reste pas planté devant la casserole : pendant que l'eau chauffe, il lance la sauce, puis
la soupe. Quand une casserole bout, il s'en occupe. **Un seul** cuisinier gère trois
plats, parce que cuisiner, c'est surtout attendre.

C'est l'idée de la programmation **asynchrone** : pendant qu'une tâche attend, une autre
avance. C'est très utile pour un serveur web qui attend des milliers de clients à la fois.

## Le même problème avec des threads

Vous savez déjà faire plusieurs choses à la fois avec des
[threads](concurrence.md). Voici trois « cuissons » qui attendent chacune 300 ms, d'abord
l'une après l'autre, puis dans trois threads :

```rust
use std::thread;
use std::time::{Duration, Instant};

fn faire_cuire(plat: &str) -> String {
    thread::sleep(Duration::from_millis(300)); // on attend sans rien calculer
    format!("{plat} cuit")
}

fn main() {
    // l'un après l'autre
    let debut = Instant::now();
    for plat in ["riz", "sauce", "soupe"] {
        faire_cuire(plat);
    }
    println!("À la suite : plus de 0,8 s ? {}", debut.elapsed() > Duration::from_millis(800));

    // trois threads en même temps
    let debut = Instant::now();
    let poignees: Vec<_> = ["riz", "sauce", "soupe"]
        .into_iter()
        .map(|plat| thread::spawn(move || faire_cuire(plat)))
        .collect();
    for p in poignees {
        println!("{}", p.join().unwrap());
    }
    println!("En parallèle : moins de 0,8 s ? {}", debut.elapsed() < Duration::from_millis(800));
}
```

```sortie
À la suite : plus de 0,8 s ? true
riz cuit
sauce cuit
soupe cuit
En parallèle : moins de 0,8 s ? true
```

Ça marche. Mais chaque thread a un coût (de la mémoire, du travail pour le système). Trois
threads, aucun souci ; dix mille threads qui ne font qu'attendre, c'est du gaspillage.
L'asynchrone fait la même chose avec **quelques** threads seulement.

## async fn et .await

Une fonction marquée `async` ne renvoie pas directement son résultat. Elle renvoie une
**future** : une promesse de résultat, une recette prête à être cuisinée.

`.await` dit : « exécute cette future, et attends son résultat ; pendant l'attente,
laisse les autres tâches avancer ».

```rust norun
async fn faire_cuire(plat: &str) -> String {
    // … une attente …
    format!("{plat} cuit")
}

async fn preparer_repas() {
    let riz = faire_cuire("riz").await; // on récupère la String
    println!("{riz}");
}
```

> **À retenir :** `.await` ne s'écrit **que** dans une fonction ou un bloc `async`.

## Une future ne fait rien toute seule

Comme les [itérateurs](iterateurs.md#les-iterateurs-sont-paresseux), les futures sont
**paresseuses**. Appeler une fonction `async` prépare le travail, mais ne l'exécute
pas :

```rust
async fn dire_bonjour() {
    println!("Bonjour !");
}

fn main() {
    let future = dire_bonjour(); // rien ne s'affiche
    println!("La future est créée, mais elle n'a rien fait.");
    drop(future); // on la jette sans l'avoir attendue : « Bonjour ! » n'apparaît jamais
}
```

```sortie
La future est créée, mais elle n'a rien fait.
```

Pour qu'une future avance, il faut quelqu'un qui l'exécute : un **runtime**.

## Un runtime : tokio

La bibliothèque standard de Rust définit les futures, mais ne fournit pas de quoi les
exécuter. On ajoute un **runtime**, une [crate](../debuter/glossaire.md#crate) qui joue
le rôle du cuisinier : il fait avancer les futures et sait laquelle réveiller quand une
attente se termine. Le plus utilisé s'appelle **tokio**.

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

`#[tokio::main]` transforme votre `async fn main` en un `main` normal qui démarre le
runtime :

```rust norun
use std::time::Duration;
use tokio::time::sleep;

async fn faire_cuire(plat: &str, secondes: u64) -> String {
    println!("{plat} : sur le feu");
    sleep(Duration::from_secs(secondes)).await;
    println!("{plat} : prêt");
    format!("{plat} cuit")
}

#[tokio::main]
async fn main() {
    let resultat = faire_cuire("le riz", 1).await;
    println!("Résultat : {resultat}");
}
```

```text
le riz : sur le feu
le riz : prêt
Résultat : le riz cuit
```

> **Attention :** dans du code `async`, utilisez `tokio::time::sleep(…).await`, jamais
> `std::thread::sleep`. Ce dernier bloque le thread entier : le cuisinier s'endort
> devant sa casserole, et plus aucune autre tâche n'avance.

## Plusieurs attentes à la fois : join!

Si on écrit trois `.await` à la suite, les cuissons se font l'une après l'autre.
`tokio::join!` lance plusieurs futures **ensemble** et attend qu'elles soient toutes
finies :

```rust norun
use std::time::{Duration, Instant};
use tokio::time::sleep;

async fn faire_cuire(plat: &str, secondes: u64) -> String {
    println!("{plat} : sur le feu");
    sleep(Duration::from_secs(secondes)).await;
    println!("{plat} : prêt");
    format!("{plat} cuit")
}

#[tokio::main]
async fn main() {
    let debut = Instant::now();
    let (a, b, c) = tokio::join!(
        faire_cuire("le riz", 3),
        faire_cuire("la sauce", 2),
        faire_cuire("la soupe", 1),
    );
    println!("{a}, {b}, {c}");
    println!("Durée : {} s", debut.elapsed().as_secs());
}
```

```text
le riz : sur le feu
la sauce : sur le feu
la soupe : sur le feu
la soupe : prêt
la sauce : prêt
le riz : prêt
le riz cuit, la sauce cuit, la soupe cuit
Durée : 3 s
```

Les trois plats sont sur le feu en même temps : le repas prend 3 secondes (la cuisson la
plus longue), et non 3 + 2 + 1 = 6.

## Lancer une tâche : tokio::spawn

`tokio::spawn` ressemble à `thread::spawn` : il lance une **tâche** qui avance toute
seule, et renvoie une poignée. On attend la poignée avec `.await`. Une tâche est bien
plus légère qu'un thread : on peut en lancer des milliers.

```rust norun
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let mut taches = Vec::new();
    for numero in 1..=3 {
        taches.push(tokio::spawn(async move {
            sleep(Duration::from_millis(100 * numero)).await;
            numero * 10
        }));
    }

    for tache in taches {
        let valeur = tache.await.unwrap();
        println!("Tâche terminée : {valeur}");
    }
}
```

```text
Tâche terminée : 10
Tâche terminée : 20
Tâche terminée : 30
```

`async move { … }` est un **bloc async** : une future écrite sur place, qui prend
possession de ses variables, comme une [closure `move`](closures.md#emporter-move).

## Le premier arrivé : select!

`tokio::select!` lance plusieurs futures et garde **la première** qui se termine. Les
autres sont abandonnées. C'est pratique pour un délai maximum :

```rust norun
use std::time::Duration;
use tokio::time::sleep;

async fn telecharger() -> String {
    sleep(Duration::from_secs(5)).await; // un téléchargement très lent
    String::from("fichier reçu")
}

#[tokio::main]
async fn main() {
    tokio::select! {
        resultat = telecharger() => println!("{resultat}"),
        _ = sleep(Duration::from_secs(2)) => println!("Trop long : abandon après 2 s"),
    }
}
```

```text
Trop long : abandon après 2 s
```

## Threads ou async ?

| Situation | Choix conseillé |
|-----------|-----------------|
| Gros calculs (images, chiffres, compression) | threads |
| Quelques tâches en parallèle | threads, plus simples |
| Beaucoup d'attentes réseau (serveur web, robot qui télécharge) | async |
| Des milliers de connexions en même temps | async |
| Petit script ou programme débutant | ni l'un ni l'autre : du code normal |

> **Astuce :** l'async ajoute de la complexité (un runtime, des `.await` partout).
> Commencez sans. Passez à l'async quand vos programmes passent leur temps à attendre
> le réseau, par exemple pour [le web](web.md).

## Erreurs fréquentes

### Écrire .await hors d'une fonction async

```rust norun
async fn dire_bonjour() {
    println!("Bonjour !");
}

fn main() {
    dire_bonjour().await;
}
```

```text
error[E0728]: `await` is only allowed inside `async` functions and blocks
 --> main.rs:6:20
  |
5 | fn main() {
  | --------- this is not `async`
6 |     dire_bonjour().await;
  |                    ^^^^^ only allowed inside `async` functions and blocks
```

`.await` n'a de sens que dans du code `async`. Rendez la fonction `async`, et pour
`main`, ajoutez un runtime (voir l'erreur suivante).

### Un main async sans runtime

```rust norun
async fn main() {
    println!("Bonjour !");
}
```

```text
error[E0752]: `main` function is not allowed to be `async`
 --> main.rs:1:1
  |
1 | async fn main() {
  | ^^^^^^^^^^^^^^^ `main` function is not allowed to be `async`
```

Rust ne sait pas exécuter les futures tout seul. Ajoutez tokio dans `Cargo.toml` et
`#[tokio::main]` au-dessus de `async fn main`.

### Oublier .await

```rust norun
async fn sauvegarder() {
    println!("Sauvegarde…");
}

async fn quitter() {
    sauvegarder();
    println!("Au revoir");
}
```

```text
warning: unused implementer of `Future` that must be used
 --> main.rs:6:5
  |
6 |     sauvegarder();
  |     ^^^^^^^^^^^^^
  |
  = note: futures do nothing unless you `.await` or poll them
```

Ce n'est qu'un avertissement, mais la sauvegarde n'a **jamais lieu** : la future est
créée puis jetée. Écrivez `sauvegarder().await;`.

## Testez-vous

1. Que renvoie l'appel `faire_cuire("riz")` si `faire_cuire` est une `async fn` ?

> **Réponse**
>
> Une future : le travail est préparé, mais rien n'est exécuté tant qu'on n'écrit pas
> `.await` (ou qu'un runtime ne la lance pas).

2. Trois futures attendent 1, 2 et 3 secondes. Combien de temps prend `tokio::join!` sur
   les trois ?

> **Réponse**
>
> Environ 3 secondes : elles attendent en même temps, et `join!` s'arrête quand la plus
> longue a fini.

3. Pour compresser 4 grosses vidéos, vaut-il mieux des threads ou de l'async ?

> **Réponse**
>
> Des threads : c'est du calcul, pas de l'attente. L'async brille quand le programme
> attend beaucoup (réseau, disque).

## En résumé

- L'asynchrone sert à gérer beaucoup d'**attentes** avec peu de threads.
- Une `async fn` renvoie une future ; `.await` l'exécute et donne son résultat.
- Une future ne fait rien tant qu'on ne l'attend pas.
- Il faut un runtime, comme tokio : `#[tokio::main]`, `tokio::spawn`, `join!`,
  `select!`, `tokio::time::sleep`.
- Calcul : threads. Beaucoup d'attentes réseau : async.

Suite : [Les macros](macros.md).
