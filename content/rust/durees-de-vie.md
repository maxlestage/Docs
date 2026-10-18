# Les durées de vie

Une durée de vie est la période pendant laquelle une référence reste valable : ce chapitre explique pourquoi Rust les vérifie, et comment les écrire quand le compilateur vous le demande.

> **Objectifs :**
>
> - comprendre ce qu'est une référence pendante, et pourquoi Rust la refuse ;
> - lire et écrire une annotation `'a` sur une fonction ;
> - savoir quand on peut se passer d'annotation ;
> - ranger une référence dans une structure ;
> - savoir quand utiliser simplement une `String` à la place.

Ce chapitre prolonge [Possession et emprunt](possession.md). Relisez-le si les mots
« emprunter » et « référence » ne vous sont pas encore familiers.

## Le problème : la référence pendante

Une [référence](../debuter/glossaire.md#emprunt) `&x` désigne une valeur qui appartient
à quelqu'un d'autre. Que se passe-t-il si cette valeur disparaît pendant que la
référence existe encore ?

Imaginez un ticket de vestiaire. Le ticket (la référence) dit où se trouve votre
manteau (la valeur). Si le vestiaire ferme et jette tous les manteaux, votre ticket
désigne un crochet vide, ou pire, le manteau de quelqu'un d'autre. C'est une
**référence pendante** (en anglais : *dangling reference*).

```rust norun
fn main() {
    let r;
    {
        let x = 5;
        r = &x;
    } // x disparaît ici
    println!("{r}"); // r désignerait une case libérée
}
```

```text
error[E0597]: `x` does not live long enough
 --> src/main.rs:5:13
  |
4 |         let x = 5;
  |             - binding `x` declared here
5 |         r = &x;
  |             ^^ borrowed value does not live long enough
6 |     }
  |     - `x` dropped here while still borrowed
7 |     println!("{r}");
  |                - borrow later used here
```

En C, ce programme compilerait et afficherait n'importe quoi. Rust le refuse **avant**
l'exécution. Le message se lit ainsi : « `x` ne vit pas assez longtemps ; il est
libéré à la ligne 6 alors qu'il est encore emprunté ».

La correction est simple : la valeur doit vivre au moins aussi longtemps que la
référence.

```rust
let x = 5;
let r = &x;
println!("{r}");
```

```sortie
5
```

## Le vérificateur d'emprunts

Le **vérificateur d'emprunts** (*borrow checker*) est la partie du
[compilateur](../debuter/glossaire.md#compilateur) qui fait cette vérification.
Pensez à un bibliothécaire très strict : il note la date de retour de chaque livre
prêté, et il refuse tout prêt qui durerait plus longtemps que le livre lui-même.

Pour chaque référence, il calcule deux zones du code :

- la zone où la **valeur** existe (de sa création à la fin de son bloc) ;
- la zone où la **référence** est utilisée (de sa création à sa dernière utilisation).

La seconde doit tenir entièrement dans la première. Cette zone s'appelle la **durée
de vie** de la référence (en anglais : *lifetime*).

```text
let x = 5;           ──┐ x existe
let r = &x;          ──┼─┐ r est utilisée
println!("{r}");     ──┼─┘
                       │
}                    ──┘ x disparaît : tout va bien
```

> **À retenir :** une durée de vie n'est pas une durée en secondes. C'est une portion
> du code source, vérifiée à la compilation. Elle ne coûte rien à l'exécution.

## Annoter une fonction avec 'a

La plupart du temps, le compilateur suit les durées de vie seul. Mais une fonction
qui **renvoie une référence** pose une question : de quel paramètre vient-elle ?

```rust
fn plus_long<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

fn main() {
    let a = String::from("abricot");
    let b = String::from("kiwi");
    println!("{}", plus_long(&a, &b));
}
```

```sortie
abricot
```

Décomposons la signature :

- `<'a>` déclare une durée de vie nommée `'a` (une apostrophe, puis un nom court) ;
- `x: &'a str` et `y: &'a str` : les deux textes vivent au moins pendant `'a` ;
- `-> &'a str` : le résultat est valable pendant `'a`, pas plus.

En clair : « le résultat vit aussi longtemps que **le plus court** des deux
paramètres ». L'annotation ne change pas la durée de vie des valeurs. Elle **décrit**
un lien, pour que le compilateur puisse vérifier chaque appel.

### Le lien est vérifié à l'appel

Grâce à ce lien, le compilateur sait que `resultat` dépend de `a` **et** de `b`. Ici,
tout est utilisé à l'intérieur du bloc de `b` : c'est permis.

```rust
fn plus_long<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

fn main() {
    let a = String::from("abricot");
    {
        let b = String::from("kiwi");
        let resultat = plus_long(&a, &b);
        println!("{resultat}");
    }
}
```

```sortie
abricot
```

Utiliser `resultat` après la fin du bloc de `b` serait refusé, même si le résultat
est en fait `a`. Le compilateur ne regarde que la signature (voir
[Erreurs fréquentes](#erreurs-frequentes)).

### Une durée de vie pour un seul paramètre

Si le résultat ne dépend que d'un paramètre, ne liez que celui-là :

```rust
fn premier_mot<'a>(texte: &'a str, _separateur: &str) -> &'a str {
    texte.split(' ').next().unwrap_or("")
}

fn main() {
    println!("{}", premier_mot("bonjour tout le monde", " "));
}
```

```sortie
bonjour
```

## Quand peut-on ne rien écrire ?

Écrire `'a` partout serait pénible. Le compilateur applique donc trois règles,
appelées **règles d'élision** (élider = omettre). En version simple :

1. Chaque paramètre référence reçoit sa propre durée de vie.
2. S'il y a **un seul** paramètre référence, le résultat prend sa durée de vie.
3. Dans une méthode avec `&self`, le résultat prend la durée de vie de `self`.

Si ces règles suffisent, vous n'écrivez rien. Sinon, le compilateur vous demande une
annotation.

```rust
// Règle 2 : un seul paramètre référence, rien à écrire.
fn sans_espaces(s: &str) -> &str {
    s.trim()
}

fn main() {
    println!("[{}]", sans_espaces("  chat  "));
}
```

```sortie
[chat]
```

Le compilateur lit cette fonction comme `fn sans_espaces<'a>(s: &'a str) -> &'a str`.
Avec deux paramètres références, comme `plus_long`, aucune règle ne permet de
choisir : il faut écrire `'a`.

## Une référence dans une structure

Une structure peut contenir une référence, par exemple un `&str` qui emprunte un texte
existant. Il faut alors une durée de vie sur la structure :

```rust
struct Extrait<'a> {
    texte: &'a str,
    ligne: usize,
}

impl<'a> Extrait<'a> {
    fn afficher(&self) {
        println!("ligne {} : {}", self.ligne, self.texte);
    }
}

fn main() {
    let livre = String::from("Il était une fois.\nUn petit chat.");
    let deuxieme = livre.lines().nth(1).unwrap();
    let e = Extrait { texte: deuxieme, ligne: 2 };
    e.afficher();
}
```

```sortie
ligne 2 : Un petit chat.
```

Lisez `Extrait<'a>` ainsi : « un `Extrait` ne peut pas vivre plus longtemps que le
texte qu'il emprunte ». Ici, `e` ne vit pas plus longtemps que `livre` : tout va bien.

> **Note :** `impl<'a> Extrait<'a>` répète la durée de vie, comme on répète un
> paramètre [générique](generiques.md).

## La durée de vie 'static

`'static` est une durée de vie spéciale : « valable pendant **toute** l'exécution du
programme ». Les textes écrits entre guillemets dans le code sont `&'static str` : ils
sont rangés dans le programme lui-même.

```rust
fn saison(mois: u32) -> &'static str {
    match mois {
        12 | 1 | 2 => "hiver",
        3..=5 => "printemps",
        6..=8 => "été",
        _ => "automne",
    }
}

fn main() {
    println!("{}", saison(7));
    println!("{}", saison(1));
}
```

```sortie
été
hiver
```

Ici, `'static` est indispensable : la fonction n'a aucun paramètre référence, donc
aucune règle d'élision ne s'applique.

> **Attention :** si le compilateur vous propose `'static` pour faire taire une
> erreur, méfiez-vous. Ce n'est presque jamais la bonne solution pour une valeur
> calculée pendant l'exécution.

## Ou alors : utilisez String

Les durées de vie servent à **emprunter** sans copier. Mais vous n'êtes pas obligé
d'emprunter. Une structure qui **possède** son texte, avec une `String`, n'a besoin
d'aucune annotation :

```rust
struct Livre {
    titre: String,
    auteur: String,
}

fn creer_livre(titre: &str) -> Livre {
    Livre { titre: titre.to_string(), auteur: String::from("inconnu") }
}

fn main() {
    let l = creer_livre("Dune");
    println!("{} ({})", l.titre, l.auteur);
}
```

```sortie
Dune (inconnu)
```

| Situation | Choix conseillé |
|-----------|-----------------|
| une structure qui garde des données longtemps | `String` (elle possède) |
| une fonction qui crée un nouveau texte | renvoyer une `String` |
| un paramètre que l'on lit seulement | `&str` (emprunt) |
| une petite vue temporaire sur un texte existant | `&'a str` dans une structure |

> **Astuce :** en débutant, mettez des `String` dans vos structures. Passez aux
> références quand vous êtes à l'aise, ou quand copier coûte vraiment trop cher.

## Erreurs fréquentes

### Renvoyer une référence vers une variable locale

```rust norun
fn saluer(nom: &str) -> &str {
    let phrase = format!("Bonjour {nom}");
    &phrase
}

fn main() {
    println!("{}", saluer("Zoé"));
}
```

```text
error[E0515]: cannot return reference to local variable `phrase`
 --> src/main.rs:3:5
  |
3 |     &phrase
  |     ^^^^^^^ returns a reference to data owned by the current function
```

`phrase` disparaît à la fin de la fonction : sa référence serait pendante. Renvoyez
la `String` elle-même : `fn saluer(nom: &str) -> String`, puis `phrase` sans `&`.

### Une référence dans une structure, sans durée de vie

```rust norun
struct Livre {
    titre: &str,
}

fn main() {
    let l = Livre { titre: "Dune" };
    println!("{}", l.titre);
}
```

```text
error[E0106]: missing lifetime specifier
 --> src/main.rs:2:12
  |
2 |     titre: &str,
  |            ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
1 ~ struct Livre<'a> {
2 ~     titre: &'a str,
  |
```

Suivez l'aide (`struct Livre<'a> { titre: &'a str }`), ou plus simplement, utilisez
`titre: String`.

### Utiliser le résultat après la fin d'un des paramètres

```rust norun
fn plus_long<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

fn main() {
    let a = String::from("abricot");
    let resultat;
    {
        let b = String::from("kiwi");
        resultat = plus_long(&a, &b);
    }
    println!("{resultat}");
}
```

```text
error[E0597]: `b` does not live long enough
  --> src/main.rs:10:34
   |
 9 |         let b = String::from("kiwi");
   |             - binding `b` declared here
10 |         resultat = plus_long(&a, &b);
   |                                  ^^ borrowed value does not live long enough
11 |     }
   |     - `b` dropped here while still borrowed
12 |     println!("{resultat}");
   |                -------- borrow later used here
```

La signature dit que le résultat peut venir de `b`. Il ne doit donc pas survivre à
`b`. Utilisez `resultat` dans le bloc, ou déclarez `b` en dehors du bloc.

## Testez-vous

**1.** Qu'est-ce qu'une référence pendante ?

> **Réponse**
>
> Une référence vers une valeur qui a déjà été libérée. Rust la refuse à la
> compilation.

**2.** Pourquoi `fn majuscule(s: &str) -> &str` n'a-t-elle pas besoin de `'a` ?

> **Réponse**
>
> Elle n'a qu'un seul paramètre référence : la règle d'élision donne sa durée de vie
> au résultat.

**3.** Quelle durée de vie faut-il écrire pour une fonction sans paramètre qui
renvoie `"bonjour"` ?

> **Réponse**
>
> `'static`, car le texte est écrit dans le programme :
>
> ```rust
> fn accueil() -> &'static str {
>     "bonjour"
> }
>
> fn main() {
>     println!("{}", accueil());
> }
> ```
>
> ```sortie
> bonjour
> ```

## En résumé

- Une référence ne doit jamais vivre plus longtemps que la valeur qu'elle emprunte.
- Le vérificateur d'emprunts le contrôle à la compilation, sans coût à l'exécution.
- `'a` décrit un lien entre les paramètres et le résultat ; il ne change rien aux
  valeurs.
- Avec un seul paramètre référence, ou `&self`, on n'écrit rien : c'est l'élision.
- `'static` vaut pour tout le programme, comme les textes entre guillemets.
- En cas de doute, une `String` dans la structure évite toute annotation.

Suite : [Structures, énumérations et traits](types.md)
