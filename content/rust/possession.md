# Possession et emprunt

La possession est l'idée centrale de Rust : chaque valeur a un seul propriétaire, et les autres parties du code peuvent seulement l'emprunter.

## Pourquoi la possession ?

Un programme range ses données en mémoire. Il faut un jour libérer cette mémoire.

- Certains langages (C) vous demandent de la libérer à la main : on oublie, ou on la
  libère deux fois.
- D'autres (Ruby, Python) utilisent un ramasse-miettes qui tourne pendant l'exécution.

Rust choisit une troisième voie : des **règles simples**, vérifiées par le compilateur.
La mémoire est libérée automatiquement, au bon moment, sans coût caché.

## Les trois règles

> **À retenir :**
> 1. Chaque valeur a **un propriétaire** (une variable).
> 2. Il n'y a **qu'un seul** propriétaire à la fois.
> 3. Quand le propriétaire disparaît (fin de son bloc `{ }`), la valeur est libérée.

```rust
{
    let s = String::from("bonjour");
    println!("{s}");
} // ici, s disparaît : la mémoire est libérée
println!("fin du bloc");
```

```sortie
bonjour
fin du bloc
```

## Le déplacement

Quand vous donnez une `String` à une autre variable, elle est **déplacée** (en anglais :
*move*). L'ancienne variable n'est plus utilisable.

```text
let a = String::from("chat");
let b = a;

   a  ──╳                     (a n'est plus valide)
   b  ──────►  [ "chat" ]     (b est le nouveau propriétaire)
```

```rust norun
let a = String::from("chat");
let b = a;
println!("{a}"); // erreur : borrow of moved value: `a`
```

Le même déplacement a lieu quand on passe une valeur à une fonction :

```rust norun
fn afficher(s: String) {
    println!("{s}");
}

fn main() {
    let nom = String::from("Zoé");
    afficher(nom);
    afficher(nom); // erreur : use of moved value: `nom`
}
```

### Les types copiés

Les petits types simples (`i32`, `f64`, `bool`, `char`…) sont **copiés** au lieu
d'être déplacés. Les deux variables restent utilisables :

```rust
let x = 5;
let y = x;
println!("{x} {y}");
```

```sortie
5 5
```

## Cloner une valeur

Pour avoir **deux** valeurs indépendantes, utilisez `clone()` :

```rust
let a = String::from("chat");
let mut b = a.clone();
b.push_str("on");
println!("{a} {b}");
```

```sortie
chat chaton
```

```text
   a  ──────►  [ "chat" ]
   b  ──────►  [ "chaton" ]   (une copie complète, modifiée)
```

> **Astuce :** `clone()` est pratique pour débuter. Mais copier coûte du temps :
> l'emprunt, ci-dessous, est souvent meilleur.

## Emprunter avec &

Une **référence** `&` permet d'utiliser une valeur sans la prendre. On dit qu'on
l'**emprunte**. Le propriétaire reste le même.

```text
   nom  ──────►  [ "Zoé" ]
                    ▲
   &nom ────────────┘   (on regarde, on ne prend pas)
```

```rust
fn longueur(s: &String) -> usize {
    s.len()
}

fn main() {
    let nom = String::from("Zoé");
    let n = longueur(&nom);
    println!("{nom} a {n} octets");
}
```

```sortie
Zoé a 4 octets
```

> **Note :** `é` occupe deux octets en mémoire, d'où le `4`.

En pratique, pour un paramètre texte, on écrit `&str` plutôt que `&String` : il accepte
les deux sortes de texte.

```rust
fn crier(s: &str) -> String {
    s.to_uppercase()
}

fn main() {
    let a = String::from("salut");
    println!("{}", crier(&a));
    println!("{}", crier("coucou"));
}
```

```sortie
SALUT
COUCOU
```

## Emprunter pour modifier avec &mut

Une référence `&mut` permet de **modifier** la valeur empruntée. La variable doit
elle-même être `mut` :

```rust
fn ajouter_point(s: &mut String) {
    s.push('.');
}

fn main() {
    let mut phrase = String::from("Fin");
    ajouter_point(&mut phrase);
    ajouter_point(&mut phrase);
    println!("{phrase}");
}
```

```sortie
Fin..
```

## Les règles de l'emprunt

À un moment donné, vous pouvez avoir **soit** :

- autant de références `&` que vous voulez (plusieurs lecteurs),
- **soit** une seule référence `&mut` (un seul écrivain).

Jamais les deux en même temps.

```rust
let mut v = vec![1, 2, 3];
let a = &v;
let b = &v;
println!("{:?} {:?}", a, b); // plusieurs lecteurs : permis
let c = &mut v;
c.push(4); // a et b ne servent plus : un écrivain est permis
println!("{:?}", v);
```

```sortie
[1, 2, 3] [1, 2, 3]
[1, 2, 3, 4]
```

```rust norun
let mut v = vec![1, 2, 3];
let premier = &v[0];
v.push(4); // erreur : cannot borrow `v` as mutable because it is also borrowed as immutable
println!("{premier}");
```

> **Attention :** cette erreur vous protège. `push` peut déplacer le vecteur en
> mémoire : `premier` pointerait alors vers une zone libérée.

## Les durées de vie

Une référence ne doit jamais vivre plus longtemps que la valeur qu'elle emprunte. Le
compilateur le vérifie avec des **durées de vie** (en anglais : *lifetimes*).

Le plus souvent, il les devine seul. Parfois, il faut les écrire avec une apostrophe,
comme `'a` :

```rust
fn plus_long<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

fn main() {
    println!("{}", plus_long("pomme", "kiwi"));
}
```

```sortie
pomme
```

Lisez `'a` ainsi : « le résultat vit aussi longtemps que `x` et `y` ». Pour débuter, il
suffit de savoir que cela existe.

## En résumé

- Chaque valeur a un seul propriétaire, libérée à la fin de son bloc.
- `let b = a;` déplace `a` dans `b` (les types simples sont copiés).
- `a.clone()` fait une copie indépendante.
- `&a` emprunte en lecture ; plusieurs lecteurs sont permis.
- `&mut a` emprunte en écriture ; un seul à la fois.

Suite : [Structures, énumérations et traits](types.md).
