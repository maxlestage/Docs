# Faire plusieurs choses à la fois

Avec les threads, votre programme fait plusieurs tâches en même temps, et Rust vérifie dès la compilation qu'elles ne se marchent pas sur les pieds.

> **Objectifs :**
>
> - lancer un thread avec `thread::spawn` et attendre sa fin avec `join` ;
> - donner des données à un thread avec une closure `move` ;
> - envoyer des messages entre threads avec un canal `mpsc` ;
> - partager un compteur avec `Arc<Mutex<T>>` ;
> - emprunter des données locales avec `thread::scope`.

## Qu'est-ce qu'un thread ?

Imaginez une cuisine. Avec un seul cuisinier, on épluche, puis on coupe, puis on fait
cuire : une chose après l'autre. Avec trois cuisiniers, les tâches avancent **en même
temps**.

Un **thread** (« fil d'exécution ») est un de ces cuisiniers. Votre programme démarre
avec un thread, celui de `main`, et peut en lancer d'autres.

> **Attention :** les threads avancent chacun à leur rythme. L'ordre de leurs
> affichages peut changer d'une exécution à l'autre. Dans cette page, on attend
> toujours la fin des threads avant d'afficher, pour obtenir un résultat stable.

## Lancer un thread : spawn et join

`thread::spawn` lance un nouveau thread, qui exécute une [closure](fonctions.md).
Il renvoie une **poignée** (*handle*). `join()` sur cette poignée **attend** que le
thread ait fini, et récupère la valeur qu'il renvoie :

```rust
use std::thread;

fn main() {
    let poignee = thread::spawn(|| {
        let mut total = 0;
        for n in 1..=100 {
            total += n;
        }
        total // la valeur renvoyée par le thread
    });

    // ici, main pourrait faire autre chose en même temps…

    let resultat = poignee.join().unwrap();
    println!("Le thread a calculé : {resultat}");
}
```

```sortie
Le thread a calculé : 5050
```

`join()` renvoie un `Result` : `Err` si le thread s'est arrêté sur un `panic!`. Ici,
`unwrap()` suffit.

Pour lancer plusieurs threads, gardez leurs poignées dans un `Vec`, puis attendez-les
toutes :

```rust
use std::thread;

fn main() {
    let mut poignees = Vec::new();
    for numero in 1..=3 {
        poignees.push(thread::spawn(move || numero * 10));
    }

    let resultats: Vec<i32> = poignees.into_iter().map(|p| p.join().unwrap()).collect();
    println!("{:?}", resultats);
}
```

```sortie
[10, 20, 30]
```

Les résultats sont dans l'ordre des poignées, quel que soit le thread qui a fini en
premier.

> **À retenir :** si `main` se termine, le programme s'arrête, même si d'autres
> threads n'ont pas fini. Pensez à appeler `join()`.

## Donner des données : move

Un thread peut vivre plus longtemps que la fonction qui l'a lancé. Il ne peut donc pas
simplement **emprunter** une variable locale : elle risquerait de disparaître avant lui.

Le mot `move` devant la closure **déplace** les variables utilisées dans le thread. Le
thread en devient propriétaire :

```rust
use std::thread;

fn main() {
    let courses = vec!["pain", "lait", "œufs"];

    let poignee = thread::spawn(move || {
        // le thread possède maintenant `courses`
        format!("{} articles à acheter", courses.len())
    });

    println!("{}", poignee.join().unwrap());
    // `courses` n'est plus utilisable ici : elle a été déplacée
}
```

```sortie
3 articles à acheter
```

> **Astuce :** si `main` a encore besoin de la valeur, donnez au thread une copie :
> `let copie = courses.clone();` puis utilisez `copie` dans la closure `move`.

## Envoyer des messages : les canaux

Un **canal** est comme un tuyau : un thread y glisse des messages d'un côté, un autre
les récupère de l'autre côté. On le crée avec `mpsc::channel()` (*multiple producer,
single consumer* : plusieurs envoyeurs, un seul receveur).

- `tx` (l'émetteur) : `tx.send(message)` ;
- `rx` (le récepteur) : `rx.recv()` attend un message, ou `for m in rx` les lit tous.

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    let poignee = thread::spawn(move || {
        for etape in ["épluchage", "découpe", "cuisson"] {
            tx.send(etape).unwrap();
        }
        // tx disparaît ici : le canal se ferme
    });

    for message in rx {
        println!("Reçu : {message}");
    }
    poignee.join().unwrap();
    println!("Le repas est prêt !");
}
```

```sortie
Reçu : épluchage
Reçu : découpe
Reçu : cuisson
Le repas est prêt !
```

La boucle `for message in rx` s'arrête quand **tous** les émetteurs ont disparu.

### Plusieurs envoyeurs

Clonez l'émetteur pour chaque thread. Comme l'ordre d'arrivée peut varier, on range les
messages puis on les **trie** avant de les afficher :

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    for cuisinier in ["Léa", "Tom", "Inès"] {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(format!("{cuisinier} a fini")).unwrap();
        });
    }
    drop(tx); // on lâche l'émetteur d'origine, sinon le canal ne se ferme jamais

    let mut messages: Vec<String> = rx.iter().collect();
    messages.sort();
    for m in &messages {
        println!("{m}");
    }
}
```

```sortie
Inès a fini
Léa a fini
Tom a fini
```

## Partager une valeur : Arc et Mutex

Parfois, plusieurs threads doivent modifier **la même** valeur, par exemple un
compteur. Il faut deux outils :

- `Mutex<T>` (*mutual exclusion*) : un verrou. `lock()` donne l'accès à la valeur, à
  **un seul thread à la fois**. Les autres attendent leur tour, comme pour une salle de
  bain avec un verrou.
- `Arc<T>` (*atomic reference counted*) : comme [Rc](pointeurs.md), il permet plusieurs
  propriétaires, mais il fonctionne entre threads.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let compteur = Arc::new(Mutex::new(0));
    let mut poignees = Vec::new();

    for _ in 0..10 {
        let compteur = Arc::clone(&compteur);
        poignees.push(thread::spawn(move || {
            let mut valeur = compteur.lock().unwrap();
            *valeur += 1;
            // le verrou est relâché quand `valeur` disparaît
        }));
    }

    for p in poignees {
        p.join().unwrap();
    }
    println!("Résultat : {}", *compteur.lock().unwrap());
}
```

```sortie
Résultat : 10
```

Chaque thread ajoute 1, l'un après l'autre grâce au verrou : le total est toujours 10.

> **À retenir :** `Arc<Mutex<T>>` est la version « plusieurs threads » de
> `Rc<RefCell<T>>`.

## Emprunter sans move : thread::scope

`thread::scope` crée une **zone** dans laquelle on lance des threads. Rust garantit que
tous ces threads sont terminés à la fin de la zone. Ils peuvent donc **emprunter** les
variables locales, sans `move` ni `Arc` :

```rust
use std::thread;

fn main() {
    let notes = vec![12, 15, 9, 18, 14, 10];
    let (debut, fin) = notes.split_at(3);

    let (somme1, somme2) = thread::scope(|s| {
        let a = s.spawn(|| debut.iter().sum::<i32>());
        let b = s.spawn(|| fin.iter().sum::<i32>());
        (a.join().unwrap(), b.join().unwrap())
    }); // tous les threads sont finis ici

    println!("{somme1} + {somme2} = {}", somme1 + somme2);
    println!("Les notes sont toujours là : {:?}", notes);
}
```

```sortie
36 + 42 = 78
Les notes sont toujours là : [12, 15, 9, 18, 14, 10]
```

> **Astuce :** quand le travail tient dans une seule fonction, `thread::scope` est
> souvent le plus simple : pas de `move`, pas de `Arc`.

## Erreurs fréquentes

### Oublier move

```rust norun
use std::thread;

fn main() {
    let nom = String::from("Léa");
    let tache = thread::spawn(|| {
        println!("Bonjour {nom}");
    });
    tache.join().unwrap();
}
```

```text
error[E0373]: closure may outlive the current function, but it borrows `nom`, which is owned by the current function
help: to force the closure to take ownership of `nom` (and any other referenced variables), use the `move` keyword
  |
5 |     let tache = thread::spawn(move || {
  |                               ++++
```

Le thread pourrait vivre plus longtemps que `nom`. Ajoutez `move` (ou utilisez
`thread::scope`).

### Utiliser Rc entre threads

```rust norun
use std::rc::Rc;
use std::thread;

fn main() {
    let valeur = Rc::new(5);
    let copie = Rc::clone(&valeur);
    let tache = thread::spawn(move || {
        println!("{copie}");
    });
    tache.join().unwrap();
}
```

```text
error[E0277]: `Rc<i32>` cannot be sent between threads safely
  = help: within `{closure@main.rs:7:31: 7:38}`, the trait `Send` is not implemented for `Rc<i32>`
```

`Rc` n'est pas fait pour plusieurs threads. Remplacez-le par `Arc` (et `RefCell` par
`Mutex`).

### Modifier l'Arc au lieu de la valeur verrouillée

```rust norun
let compteur = Arc::new(Mutex::new(0));
let c = Arc::clone(&compteur);
thread::spawn(move || {
    c += 1;
}).join().unwrap();
```

```text
error[E0368]: binary assignment operation `+=` cannot be applied to type `Arc<std::sync::Mutex<{integer}>>`
```

Il faut d'abord ouvrir le verrou avec `lock()`, puis modifier la valeur avec `*` :
`*c.lock().unwrap() += 1;`.

## Testez-vous

1. Que se passe-t-il si `main` se termine avant un thread qu'il a lancé ?

> **Réponse**
>
> Le programme s'arrête, et le thread est interrompu. Appelez `join()` pour attendre
> la fin du thread.

2. Pourquoi faut-il souvent `move` devant la closure de `thread::spawn` ?

> **Réponse**
>
> Le thread peut vivre plus longtemps que la fonction qui l'a lancé. Il doit donc
> posséder ses données, au lieu de les emprunter.

3. Quel type utiliser pour un compteur que dix threads augmentent ?

> **Réponse**
>
> `Arc<Mutex<i32>>` (par exemple) : `Arc` pour partager entre threads, `Mutex` pour
> qu'un seul thread à la fois le modifie.

## En résumé

- `thread::spawn(closure)` lance un thread ; `join()` attend sa fin et donne son
  résultat.
- `move` donne au thread la possession des variables qu'il utilise.
- Un canal `mpsc::channel()` envoie des messages d'un thread à l'autre.
- `Arc<Mutex<T>>` partage une valeur modifiable entre threads, avec un verrou.
- `thread::scope` permet aux threads d'emprunter des variables locales.
- Rust refuse à la compilation le code qui partagerait mal les données.

Suite : [Tester son code](tests.md).
