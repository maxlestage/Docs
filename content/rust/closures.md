# Les closures en détail

Une closure est une petite fonction sans nom qui emporte avec elle les variables qui l'entourent : ce chapitre montre comment elle les capture, comment la passer, la renvoyer et la ranger.

> **Objectifs :**
>
> - écrire une closure sous ses différentes formes ;
> - comprendre les trois façons de capturer une variable, dont `move` ;
> - distinguer `Fn`, `FnMut` et `FnOnce` ;
> - passer une closure à une fonction, et en renvoyer une ;
> - ranger des closures dans une structure ou un `Vec`.

## Rappel : une fonction sans nom

Vous avez déjà croisé les [closures](../debuter/glossaire.md#closure) dans le chapitre
sur [les fonctions](fonctions.md#les-closures). Une closure s'écrit avec des barres
`|…|` pour les paramètres, puis le corps :

```rust
fn double_fonction(x: i32) -> i32 {
    x * 2
}

fn main() {
    let double_closure = |x: i32| x * 2;
    println!("{}", double_fonction(4));
    println!("{}", double_closure(4));
}
```

```sortie
8
8
```

Les deux font la même chose. La différence : la closure peut utiliser les variables
qui l'entourent. Une fonction écrite avec `fn` ne le peut pas.

## Les différentes écritures

Une closure s'écrit de plusieurs façons, de la plus complète à la plus courte :

```rust
let complete = |x: i32| -> i32 { x + 1 }; // types écrits
let courte = |x| x + 1;                   // types devinés
let sans_parametre = || println!("Coucou !");
let plusieurs_lignes = |x: i32| {
    let carre = x * x;
    carre + 1
};

println!("{}", complete(1));
println!("{}", courte(1));
sans_parametre();
println!("{}", plusieurs_lignes(3));
```

```sortie
2
2
Coucou !
10
```

- Les accolades `{ }` ne sont obligatoires que pour plusieurs lignes, ou si vous
  écrivez le type de retour avec `->`.
- La dernière expression du bloc est la valeur renvoyée, comme dans une fonction.
- Quand les types ne sont pas écrits, Rust les devine au **premier appel**.

## Capturer une variable : trois façons

Une closure **capture** les variables qu'elle utilise. Pensez à un livre de
bibliothèque : on peut le **lire** sur place, l'**annoter** (personne d'autre ne le
touche pendant ce temps), ou l'**emporter** chez soi. Rust choisit tout seul la façon la
plus légère qui suffit, en suivant les règles de l'[emprunt](possession.md).

### Lire : emprunt partagé

```rust
let prenom = String::from("Léa");
let saluer = || println!("Bonjour {prenom}");
saluer();
saluer();
println!("{prenom} est toujours là");
```

```sortie
Bonjour Léa
Bonjour Léa
Léa est toujours là
```

### Modifier : emprunt exclusif

Si la closure modifie la variable, elle l'emprunte en écriture. La closure elle-même doit
alors être `mut` :

```rust
let mut compteur = 0;
let mut cliquer = || compteur += 1;
cliquer();
cliquer();
cliquer();
println!("{compteur} clics"); // la closure n'est plus utilisée : on peut relire
```

```sortie
3 clics
```

### Emporter : move

Le mot `move` force la closure à **prendre possession** des variables. C'est utile
quand la closure doit vivre plus longtemps que la fonction qui l'a créée (un thread,
une closure renvoyée…) :

```rust
let message = String::from("bon voyage");
let carte_postale = move || println!("Message : {message}");
carte_postale();
// `message` a été déplacé dans la closure : il n'est plus utilisable ici
```

```sortie
Message : bon voyage
```

> **Note :** avec un type copiable comme `i32`, `move` fait une **copie**. La variable
> d'origine reste utilisable.

## Fn, FnMut et FnOnce

Selon ce qu'elle fait de ses captures, une closure appartient à une ou plusieurs de ces
trois familles (ce sont des [traits](../debuter/glossaire.md#trait)) :

| Trait | Ce que fait la closure | Combien d'appels | Image |
|-------|------------------------|------------------|-------|
| `Fn` | lit ses captures | autant qu'on veut | regarder une photo |
| `FnMut` | modifie ses captures | autant qu'on veut, mais la closure est `mut` | écrire dans un carnet |
| `FnOnce` | consomme (donne) une capture | une seule fois | utiliser un ticket de cinéma |

Ces familles s'emboîtent : toute closure `Fn` est aussi `FnMut`, et toute closure
`FnMut` est aussi `FnOnce`. Une fonction qui accepte un `FnOnce` accepte donc **toutes**
les closures.

```rust
fn main() {
    let nom = String::from("Tom");

    // Fn : elle ne fait que lire `nom`
    let lire = || println!("lecture : {nom}");
    lire();
    lire();

    // FnMut : elle modifie `liste`
    let mut liste = Vec::new();
    let mut ajouter = |x| liste.push(x);
    ajouter(1);
    ajouter(2);
    println!("{:?}", liste);

    // FnOnce : elle renvoie `nom`, donc s'en sépare
    let donner = move || nom;
    let recupere = donner();
    println!("récupéré : {recupere}");
}
```

```sortie
lecture : Tom
lecture : Tom
[1, 2]
récupéré : Tom
```

Appeler `donner()` une deuxième fois serait impossible : le ticket est déjà utilisé.

## Passer une closure à une fonction

### Avec impl Fn

C'est l'écriture la plus simple. Choisissez le trait le moins exigeant pour **vous**
qui écrivez la fonction :

```rust
// on appelle `f` plusieurs fois sans rien modifier : Fn
fn repeter(fois: u32, f: impl Fn(u32)) {
    for i in 1..=fois {
        f(i);
    }
}

// on appelle `f` plusieurs fois et elle peut modifier : FnMut
fn trois_fois(mut f: impl FnMut()) {
    f();
    f();
    f();
}

// on appelle `f` une seule fois : FnOnce (accepte tout)
fn plus_tard(f: impl FnOnce() -> String) -> String {
    f()
}

fn main() {
    repeter(2, |i| println!("tour {i}"));

    let mut total = 0;
    trois_fois(|| total += 10);
    println!("total : {total}");

    let mot = String::from("fini");
    println!("{}", plus_tard(move || mot));
}
```

```sortie
tour 1
tour 2
total : 30
fini
```

### Avec un générique

`impl Fn(…)` est un raccourci pour un [générique](generiques.md). L'écriture longue,
avec `where`, est utile quand la signature devient chargée :

```rust
fn appliquer<F>(valeurs: &[i32], f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    valeurs.iter().map(|&v| f(v)).collect()
}

fn main() {
    let decalage = 100;
    println!("{:?}", appliquer(&[1, 2, 3], |x| x + decalage));
}
```

```sortie
[101, 102, 103]
```

> **Astuce :** une fonction ordinaire peut aussi être passée là où on attend une
> closure : `appliquer(&[1, 2], i32::abs)` ou `appliquer(&[1, 2], ma_fonction)`.

## Renvoyer une closure

### Avec impl Fn

Une fonction peut fabriquer une closure et la renvoyer. Il faut presque toujours `move`,
car les variables locales de la fonction vont disparaître :

```rust
fn fabrique_multiplicateur(facteur: i32) -> impl Fn(i32) -> i32 {
    move |x| x * facteur
}

fn main() {
    let triple = fabrique_multiplicateur(3);
    let decuple = fabrique_multiplicateur(10);
    println!("{} {}", triple(7), decuple(7));
}
```

```sortie
21 70
```

### Avec Box<dyn Fn>

Chaque closure a son **propre type**, unique, même si deux closures se ressemblent.
Pour renvoyer des closures différentes selon un `if`, rangez-les dans une
[`Box`](pointeurs.md) : `Box<dyn Fn(…)>` veut dire « n'importe quelle closure de cette
forme, rangée sur le tas ».

```rust
fn operation(nom: &str) -> Box<dyn Fn(i32, i32) -> i32> {
    match nom {
        "plus" => Box::new(|a, b| a + b),
        "fois" => Box::new(|a, b| a * b),
        _ => Box::new(|_, _| 0),
    }
}

fn main() {
    let f = operation("plus");
    let g = operation("fois");
    println!("{} {}", f(6, 7), g(6, 7));
}
```

```sortie
13 42
```

## Ranger des closures

### Dans une structure

Une structure peut garder une closure dans un champ générique. Pour l'appeler, mettez
le champ entre parenthèses : `(self.regle)(x)`.

```rust
struct Filtre<F: Fn(i32) -> bool> {
    nom: String,
    regle: F,
}

impl<F: Fn(i32) -> bool> Filtre<F> {
    fn tester(&self, x: i32) -> bool {
        (self.regle)(x)
    }
}

fn main() {
    let majeur = Filtre { nom: String::from("majeur"), regle: |age| age >= 18 };
    for age in [12, 18, 40] {
        println!("{} ({age}) : {}", majeur.nom, majeur.tester(age));
    }
}
```

```sortie
majeur (12) : false
majeur (18) : true
majeur (40) : true
```

### Dans un Vec

Pour une liste de closures différentes, utilisez `Box<dyn Fn>` :

```rust
fn main() {
    let etapes: Vec<(&str, Box<dyn Fn(i32) -> i32>)> = vec![
        ("ajouter 1", Box::new(|x| x + 1)),
        ("doubler", Box::new(|x| x * 2)),
        ("au carré", Box::new(|x| x * x)),
    ];

    let mut valeur = 3;
    for (nom, etape) in &etapes {
        valeur = etape(valeur);
        println!("{nom} → {valeur}");
    }
}
```

```sortie
ajouter 1 → 4
doubler → 8
au carré → 64
```

## Les closures dans la bibliothèque standard

Beaucoup de méthodes prennent une closure pour savoir **quoi** faire. En voici
quelques-unes, très courantes :

```rust
fn main() {
    let mut eleves = vec![("Léa", 15), ("Tom", 9), ("Inès", 18), ("Max", 11)];

    // trier selon une clé : ici la note
    eleves.sort_by_key(|&(_, note)| note);
    println!("{:?}", eleves);

    // trier avec une comparaison : note décroissante
    eleves.sort_by(|a, b| b.1.cmp(&a.1));
    println!("{:?}", eleves);

    // garder seulement certains éléments, sur place
    eleves.retain(|&(_, note)| note >= 10);
    println!("{:?}", eleves);

    // une valeur par défaut calculée seulement si besoin
    let premier_nul = eleves.iter().find(|e| e.1 < 5).map(|e| e.0);
    println!("{}", premier_nul.unwrap_or_else(|| "personne"));
}
```

```sortie
[("Tom", 9), ("Max", 11), ("Léa", 15), ("Inès", 18)]
[("Inès", 18), ("Léa", 15), ("Max", 11), ("Tom", 9)]
[("Inès", 18), ("Léa", 15), ("Max", 11)]
personne
```

Et bien sûr, `map`, `filter`, `fold`… de [la page sur les itérateurs](iterateurs.md).

## Erreurs fréquentes

### Oublier mut sur une closure qui modifie

```rust norun
let mut compteur = 0;
let cliquer = || compteur += 1;
cliquer();
```

```text
error[E0596]: cannot borrow `cliquer` as mutable, as it is not declared as mutable
help: consider changing this to be mutable
  |
4 | let mut cliquer = || compteur += 1;
  |     +++
```

Une closure qui modifie une capture change d'état à chaque appel : déclarez-la avec
`let mut cliquer`.

### Renvoyer une closure sans move

```rust norun
fn ajouteur(n: i32) -> impl Fn(i32) -> i32 {
    |x| x + n
}
```

```text
error[E0373]: closure may outlive the current function, but it borrows `n`, which is owned by the current function
help: to force the closure to take ownership of `n` (and any other referenced variables), use the `move` keyword
  |
4 |     move |x| x + n
  |     ++++
```

`n` disparaît à la fin de `ajouteur`, mais la closure lui survit. Ajoutez `move`.

### Appeler deux fois une closure FnOnce

```rust norun
let nom = String::from("Léa");
let donner = move || nom;
let a = donner();
let b = donner();
```

```text
error[E0382]: use of moved value: `donner`
note: closure cannot be invoked more than once because it moves the variable `nom` out of its environment
```

La closure donne `nom` au premier appel : elle n'a plus rien à donner ensuite. Renvoyez
une copie (`move || nom.clone()`) si vous voulez l'appeler plusieurs fois.

### Mettre deux closures dans un Vec sans Box

```rust norun
let n = 2;
let etapes = vec![|x: i32| x + n, |x: i32| x * n];
```

```text
error[E0308]: mismatched types
  = note: expected closure `{closure@main.rs:4:19: 4:27}`
             found closure `{closure@main.rs:4:35: 4:43}`
  = note: no two closures, even if identical, have the same type
  = help: consider boxing your closure and/or using it as a trait object
```

Chaque closure a son propre type. (Sans capture, comme `|x: i32| x + 1`, Rust les
convertit en simples pointeurs de fonction et accepte le `Vec`.) Rangez-les dans des `Box<dyn Fn(i32) -> i32>`, comme
dans la section [Dans un Vec](#dans-un-vec).

## Testez-vous

1. Une closure ajoute des éléments à un `Vec` capturé. Est-elle `Fn`, `FnMut` ou
   `FnOnce` ?

> **Réponse**
>
> `FnMut` : elle modifie sa capture. On peut l'appeler plusieurs fois, mais elle doit
> être déclarée `mut`.

2. Pourquoi écrit-on `move` dans `fn f(n: i32) -> impl Fn() -> i32 { move || n }` ?

> **Réponse**
>
> Parce que `n` disparaît à la fin de `f`. Avec `move`, la closure garde sa propre copie
> de `n` et peut vivre plus longtemps que la fonction.

3. Quand faut-il `Box<dyn Fn>` plutôt que `impl Fn` ?

> **Réponse**
>
> Quand plusieurs closures différentes doivent avoir le même type : les renvoyer selon un
> `if` ou un `match`, ou les ranger ensemble dans un `Vec`.

## En résumé

- Une closure s'écrit `|params| corps` et peut utiliser les variables qui l'entourent.
- Elle capture en lecture, en écriture, ou en possession avec `move`.
- `Fn` lit, `FnMut` modifie, `FnOnce` consomme et ne s'appelle qu'une fois.
- En paramètre : `impl Fn(…)` ou un générique `F: Fn(…)`.
- Pour renvoyer : `impl Fn` avec `move` ; pour mélanger des closures : `Box<dyn Fn>`.
- `sort_by_key`, `retain`, `unwrap_or_else`… prennent une closure pour savoir quoi faire.

Suite : [Les itérateurs](iterateurs.md)
