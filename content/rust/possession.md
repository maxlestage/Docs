# Possession et emprunt

La possession est l'idée centrale de Rust : chaque valeur a un seul propriétaire, et les autres parties du code peuvent seulement l'emprunter.

> **Objectifs :**
>
> - expliquer les trois règles de la possession ;
> - reconnaître un déplacement, et l'éviter avec `clone` ou un emprunt ;
> - emprunter une valeur en lecture avec `&` et en écriture avec `&mut` ;
> - comprendre les messages d'erreur du vérificateur d'emprunts ;
> - lire une durée de vie comme `'a`.

## Pourquoi la possession ?

Un programme range ses données en mémoire. Il faut un jour libérer cette mémoire.

- Certains langages (C) vous demandent de la libérer à la main : on oublie, ou on la
  libère deux fois.
- D'autres (Ruby, Python) utilisent un ramasse-miettes qui tourne pendant l'exécution.

Rust choisit une troisième voie : des **règles simples**, vérifiées par le compilateur.
La mémoire est libérée automatiquement, au bon moment, sans coût caché.

Une image aide à tout comprendre : une valeur est un **objet**, par exemple un livre.
Une variable qui la **possède** en est le propriétaire. Vous pouvez donner le livre
(le déplacer), en faire une photocopie (le cloner) ou le prêter (l'emprunter).

## Les trois règles

> **À retenir :**
>
> 1. Chaque valeur a **un propriétaire** (une variable).
> 2. Il n'y a **qu'un seul** propriétaire à la fois.
> 3. Quand le propriétaire disparaît (fin de son bloc `{ }`), la valeur est libérée.

Une variable n'existe que dans le bloc où elle est créée : c'est sa
[portée](../debuter/glossaire.md#portee).

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
*move*). C'est comme donner votre livre : vous ne l'avez plus. L'ancienne variable
n'est plus utilisable.

```text
let a = String::from("chat");
let b = a;

   a  ──╳                     (a n'est plus valide)
   b  ──────►  [ "chat" ]     (b est le nouveau propriétaire)
```

Utiliser `a` après ce déplacement est refusé par le compilateur. Le même déplacement
a lieu quand on passe une valeur à une fonction : la fonction devient propriétaire.
Les deux cas sont montrés dans [Erreurs fréquentes](#erreurs-frequentes).

### Les types copiés

Les petits types simples (`i32`, `f64`, `bool`, `char`…) sont **copiés** au lieu
d'être déplacés : les copier ne coûte presque rien. On dit qu'ils ont le trait `Copy`.
Les deux variables restent utilisables :

```rust
let x = 5;
let y = x;
println!("{x} {y}");
```

```sortie
5 5
```

## Cloner une valeur

Pour avoir **deux** valeurs indépendantes, utilisez `clone()`. C'est la photocopie :
chacun a son exemplaire.

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
l'**emprunte**. C'est le prêt du livre : l'autre le lit, puis vous le rend. Le
propriétaire reste le même.

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

Une référence `&mut` permet de **modifier** la valeur empruntée : vous prêtez le livre
avec un crayon. La variable doit elle-même être `mut` :

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

Jamais les deux en même temps. Pensez à un tableau blanc : plusieurs personnes peuvent
le lire ensemble, mais quand quelqu'un écrit dessus, personne d'autre ne doit lire ou
écrire en même temps.

Le compilateur qui vérifie ces règles s'appelle le **vérificateur d'emprunts** (en
anglais : *borrow checker*).

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

Un emprunt dure jusqu'à sa dernière utilisation. Ici, `a` et `b` ne servent plus après
le premier `println!` : l'emprunt `&mut` qui suit est donc permis.

## Les durées de vie

Une référence ne doit jamais vivre plus longtemps que la valeur qu'elle emprunte. On ne
garde pas un livre prêté après la fermeture définitive de la bibliothèque. Le
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

Un texte écrit entre guillemets dans le code a la durée de vie `'static` : il existe
pendant toute l'exécution du programme.

## Erreurs fréquentes

### Utiliser une valeur déplacée

```rust norun
fn main() {
    let a = String::from("chat");
    let b = a;
    println!("{a} {b}");
}
```

```text
error[E0382]: borrow of moved value: `a`
 --> src/main.rs:4:16
  |
2 |     let a = String::from("chat");
  |         - move occurs because `a` has type `String`, which does not implement the `Copy` trait
3 |     let b = a;
  |             - value moved here
4 |     println!("{a} {b}");
  |                ^ value borrowed here after move
```

Après `let b = a;`, seul `b` possède le texte. Pour garder les deux, écrivez
`let b = a.clone();`, ou empruntez : `let b = &a;`.

### Donner une valeur à une fonction, deux fois

```rust norun
fn afficher(s: String) {
    println!("{s}");
}

fn main() {
    let nom = String::from("Zoé");
    afficher(nom);
    afficher(nom);
}
```

```text
error[E0382]: use of moved value: `nom`
 --> src/main.rs:8:14
  |
6 |     let nom = String::from("Zoé");
  |         --- move occurs because `nom` has type `String`, which does not implement the `Copy` trait
7 |     afficher(nom);
  |              --- value moved here
8 |     afficher(nom);
  |              ^^^ value used here after move
```

La fonction n'a besoin que de lire : faites-lui emprunter le texte.
`fn afficher(s: &str)`, puis `afficher(&nom);`.

### Modifier pendant une lecture

```rust norun
fn main() {
    let mut v = vec![1, 2, 3];
    let premier = &v[0];
    v.push(4);
    println!("{premier}");
}
```

```text
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 --> src/main.rs:4:5
  |
3 |     let premier = &v[0];
  |                    - immutable borrow occurs here
4 |     v.push(4);
  |     ^^^^^^^^^ mutable borrow occurs here
5 |     println!("{premier}");
  |                ------- immutable borrow later used here
```

Cette erreur vous protège : `push` peut déplacer le vecteur en mémoire, et `premier`
désignerait alors une zone libérée. Utilisez `premier` avant le `push`, ou copiez la
valeur : `let premier = v[0];`.

### Renvoyer une référence sans durée de vie

```rust norun
fn plus_long(x: &str, y: &str) -> &str {
    if x.len() >= y.len() { x } else { y }
}

fn main() {
    println!("{}", plus_long("pomme", "kiwi"));
}
```

```text
error[E0106]: missing lifetime specifier
 --> src/main.rs:1:35
  |
1 | fn plus_long(x: &str, y: &str) -> &str {
  |                 ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
```

Le compilateur ne sait pas si le résultat emprunte `x` ou `y`. Ajoutez une durée de
vie, comme dans [Les durées de vie](#les-durees-de-vie).

## Testez-vous

**1.** Après `let a = 5; let b = a;`, peut-on encore utiliser `a` ? Et avec
`String::from("5")` ?

> **Réponse**
>
> Avec `5`, oui : un `i32` est copié. Avec une `String`, non : elle est déplacée dans
> `b`.

**2.** Quelle signature choisir pour une fonction qui compte les voyelles d'un texte
sans le modifier ?

> **Réponse**
>
> Un emprunt en lecture : `fn voyelles(texte: &str) -> usize`.

**3.** Corrigez ce code pour qu'il ajoute `!` au texte :

```rust norun
fn crier(s: String) {
    s.push('!');
}
```

> **Réponse**
>
> Empruntez en écriture :
>
> ```rust
> fn crier(s: &mut String) {
>     s.push('!');
> }
>
> fn main() {
>     let mut mot = String::from("Stop");
>     crier(&mut mot);
>     println!("{mot}");
> }
> ```
>
> ```sortie
> Stop!
> ```

## En résumé

- Chaque valeur a un seul propriétaire, libérée à la fin de son bloc.
- `let b = a;` déplace `a` dans `b` (les types simples sont copiés).
- `a.clone()` fait une copie indépendante.
- `&a` emprunte en lecture ; plusieurs lecteurs sont permis.
- `&mut a` emprunte en écriture ; un seul à la fois.
- Une référence ne vit jamais plus longtemps que sa valeur : c'est sa durée de vie.

Suite : [Structures, énumérations et traits](types.md).
