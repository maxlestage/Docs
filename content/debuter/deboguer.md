# Trouver et corriger ses erreurs

Tout programme a des bugs au début : cette page vous apprend à lire un message d'erreur, à afficher des valeurs pour comprendre, et à demander de l'aide efficacement.

> **Objectifs :**
>
> - reconnaître les trois sortes d'erreurs : syntaxe, exécution, logique ;
> - lire un message d'erreur ligne par ligne ;
> - afficher des valeurs pour comprendre ce que fait le programme ;
> - réduire un problème à quelques lignes ;
> - chercher un message d'erreur en ligne et demander de l'aide.

## Trois sortes d'erreurs

### Les erreurs de syntaxe

Le code est **mal écrit** : une parenthèse oubliée, un `end` manquant. Le
programme ne démarre même pas. Ici, la fonction n'a pas de `end` :

```ruby norun
def bonjour(nom)
  puts "Bonjour, #{nom} !"

bonjour("Ada")
```

```text
syntaxe.rb: --> syntaxe.rb
Unmatched keyword, missing `end' ?
> 1  def bonjour(nom)
> 4  bonjour("Ada")
syntaxe.rb:4: syntax error, unexpected end-of-input, expecting `end' or dummy end (SyntaxError)
bonjour("Ada")
              ^
```

Ruby a bien deviné : « missing `end' ? », il manque un `end`.

### Les erreurs d'exécution

Le code est bien écrit, il démarre, mais il **s'arrête en route** parce
qu'une opération est impossible. Ici, on lit l'élément numéro 3 d'une liste
qui n'en a que trois (numérotés 0, 1 et 2) :

```rust norun
fn main() {
    let notes = vec![12, 15, 9];
    println!("{}", notes[3]);
}
```

```text
thread 'main' (1707) panicked at indice.rs:3:25:
index out of bounds: the len is 3 but the index is 3
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

En Rust, on dit que le programme **panique**.

### Les erreurs de logique

Le programme marche, **mais le résultat est faux**. Il n'y a aucun message :
c'est l'erreur la plus difficile à trouver. Ici, on veut la moyenne de 10
et 20 :

```ruby
puts 10 + 20 / 2
```

```sortie
20
```

On attendait 15. La division passe avant l'addition, comme en
mathématiques : l'ordinateur a calculé `10 + 10`. Il fallait écrire
`(10 + 20) / 2`.

> **À retenir :** une erreur de syntaxe empêche de démarrer, une erreur
> d'exécution arrête en route, une erreur de logique donne un mauvais
> résultat sans rien dire.

## Lire un message d'erreur

Un message d'erreur fait peur, mais il suit toujours le même plan. Il
répond à trois questions : **quoi**, **où** et souvent **comment
corriger**.

### En Ruby

```ruby norun
prix = [12, 5, 8]
total = prix.sum
puts "Total : " + total
```

```text
prix.rb:3:in `+': no implicit conversion of Integer into String (TypeError)

puts "Total : " + total
                  ^^^^^
	from prix.rb:3:in `<main>'
```

Ligne par ligne :

1. `prix.rb:3` : le fichier et la **ligne** du problème ;
2. ``in `+'`` : pendant l'opération `+` ;
3. `no implicit conversion of Integer into String` : **quoi** — Ruby ne
   transforme pas tout seul un nombre (*Integer*) en texte (*String*) ;
4. `(TypeError)` : le **nom** de l'erreur, utile pour chercher en ligne ;
5. la ligne de code, avec `^^^^^` sous la partie fautive ;
6. `from …` : le chemin suivi par le programme jusqu'à l'erreur.

Correction : `puts "Total : #{total}"`.

### En Rust

```rust norun
fn main() {
    let age: i32 = "36";
    println!("{age}");
}
```

```text
error[E0308]: mismatched types
 --> age.rs:2:20
  |
2 |     let age: i32 = "36";
  |              ---   ^^^^ expected `i32`, found `&str`
  |              |
  |              expected due to this

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
```

1. `error[E0308]: mismatched types` : **quoi** — les types ne
   correspondent pas ; `E0308` est le code de l'erreur ;
2. `--> age.rs:2:20` : **où** — fichier, ligne 2, colonne 20 ;
3. la ligne de code, avec `^^^^` sous le problème : on attendait un
   nombre (`i32`), on a trouvé un texte (`&str`) ;
4. `---` montre la cause : c'est `i32` qui demande un nombre ;
5. la dernière ligne donne la commande pour une explication complète.

Correction : `let age: i32 = 36;`, sans guillemets.

### En Rythme

```rythme norun
notes = [12, 15, 9]
print(len(note))
```

```text
erreur[E003]: nom inconnu `note`
 --> nom.ry:2:11
  |
2 | print(len(note))
  |           ^^^^ non défini
  |
  = aide: vouliez-vous dire `notes` ?
  = en savoir plus : `rythme explain E003`
```

1. `erreur[E003]: nom inconnu` : **quoi**, en français ;
2. `--> nom.ry:2:11` : **où** ;
3. la ligne, avec `^^^^` sous le nom inconnu ;
4. `aide` : **comment corriger** — ici, une simple faute de frappe ;
5. `rythme explain E003` : une explication détaillée, avec des exemples.

> **Astuce :** quand il y a plusieurs erreurs, corrigez **la première**,
> puis relancez. Les suivantes sont souvent des conséquences de la
> première.

## Afficher des valeurs pour comprendre

Quand le résultat est faux, regardez ce que contiennent vos variables.
Ajoutez des affichages au milieu du programme.

### En Ruby : puts et p

`puts` affiche une valeur simplement. `p` l'affiche **telle qu'elle est
dans le code** : on voit les guillemets d'un texte et les crochets d'une
liste. Pour déboguer, préférez `p`.

```ruby
prix = [12, 5, 8]
p prix
total = prix.sum
p total
puts "Moyenne : #{total / prix.size}"
```

```sortie
[12, 5, 8]
25
Moyenne : 8
```

```ruby
puts "42"
p "42"
puts nil
p nil
```

```sortie
42
"42"

nil
```

Avec `puts`, on ne voit pas si `42` est un nombre ou un texte, ni si une
valeur est `nil`. Avec `p`, si.

### En Rust : println! et dbg!

`println!("{:?}", x)` affiche une valeur telle qu'elle est. `dbg!` fait
mieux : il affiche le fichier, la ligne, le calcul et son résultat, puis
**rend la valeur**. On peut donc le glisser au milieu d'un calcul.

```rust
fn main() {
    let prix = [12, 5, 8];
    let total: i32 = prix.iter().sum();
    println!("{:?} {total}", prix);
    let moyenne = dbg!(total / 3);
    println!("Moyenne : {moyenne}");
}
```

Dans le terminal :

```text
[12, 5, 8] 25
[main.rs:5:19] total / 3 = 8
Moyenne : 8
```

`dbg!` écrit sur la **sortie d'erreur**, à part : il ne se mélange pas aux
vrais résultats du programme si on les enregistre dans un fichier.

### En Rythme : print et debug

`print` affiche. `debug` affiche la valeur avec le fichier et la ligne, puis
la rend, comme `dbg!` :

```rythme
prix = [12, 5, 8]
total = debug(sum(prix))
print("Moyenne : {total / len(prix)}")
```

Dans le terminal :

```text
[debug main.ry:2:9] 25
Moyenne : 8.333333333333334
```

> **Attention :** une fois le bug trouvé, retirez vos `p`, `dbg!` et
> `debug`. Ils ne doivent pas rester dans le programme final.

## Le canard en plastique

Une méthode très sérieuse, malgré son nom : posez un canard en plastique (ou
n'importe quel objet) à côté de l'écran, et **expliquez-lui votre code,
ligne par ligne, à voix haute**.

« Ici, je mets le premier prix dans `total`. Puis, pour chaque prix… ah,
mais je commence à 1, pas à 0 ! »

En expliquant, on est obligé de dire ce que le code **fait vraiment**, et
non ce qu'on **croit** qu'il fait. Très souvent, on trouve le bug avant
d'avoir fini. Cela marche aussi avec un ami, même s'il ne programme pas.

## Réduire le problème

Un bug caché dans 200 lignes est difficile à trouver. Dans 5 lignes, il
saute aux yeux.

1. **Copiez** votre programme dans un nouveau fichier.
2. **Retirez** un morceau qui ne semble pas lié au problème.
3. **Relancez** : le bug est-il toujours là ?
   - oui : continuez à retirer ;
   - non : le bug était dans le morceau retiré. Remettez-le, et cherchez
     dedans.
4. Arrêtez-vous quand il ne reste que **quelques lignes**.

Remplacez aussi les données lues dans un fichier ou au clavier par des
valeurs écrites dans le code : `notes = [12, 15, 9]`.

> **Astuce :** changez **une seule chose à la fois**, puis relancez. Si
> vous changez trois choses, vous ne saurez pas laquelle a compté.

## Chercher le message en ligne

Vous n'êtes sûrement pas la première personne à rencontrer ce message.
Copiez-le dans un moteur de recherche :

- gardez la partie **générale** : `no implicit conversion of Integer into
  String` ;
- retirez ce qui est propre à **votre** programme : noms de fichiers,
  numéros de ligne, noms de variables ;
- ajoutez le **nom du langage** : `ruby`, `rust` ou `rythme`.

Les codes d'erreur se cherchent très bien : `rust E0308`. Et ils
s'expliquent sans Internet :

```sh
rustc --explain E0308
rythme explain E003
```

## Demander de l'aide

Si vous bloquez depuis longtemps, demandez : sur un forum, à un ami, à un
assistant. Une bonne question reçoit vite une bonne réponse. Elle contient :

1. **ce que vous voulez faire** : « calculer la moyenne de mes notes » ;
2. **un exemple minimal** : les quelques lignes qui montrent le problème,
   en texte (pas une photo de l'écran) ;
3. **ce que vous attendiez** : « 12 » ;
4. **ce qui se passe** : le résultat obtenu, ou le message d'erreur
   **complet**, copié-collé ;
5. **ce que vous avez déjà essayé**.

Par exemple :

```text
Je veux la moyenne de 10 et 20 en Ruby. Mon code :

    puts 10 + 20 / 2

J'attends 15, mais j'obtiens 20. J'ai vérifié que 10 + 20 donne bien 30.
Pourquoi ?
```

Souvent, en préparant l'exemple minimal, vous trouverez la réponse
vous-même.

## Erreurs fréquentes

### Lire seulement la dernière ligne du message

En Ruby, le message commence en haut et le chemin `from …` suit. En Rust,
il y a souvent plusieurs erreurs à la suite :

```rust norun
fn main() {
    let total = 0;
    total = total + 5;
    println!("{total}");
}
```

```text
error[E0384]: cannot assign twice to immutable variable `total`
 --> mu.rs:3:5
  |
2 |     let total = 0;
  |         ----- first assignment to `total`
3 |     total = total + 5;
  |     ^^^^^^^^^^^^^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
2 |     let mut total = 0;
  |         +++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0384`.
```

La dernière ligne ne dit presque rien. Tout est **en haut** : le problème
(`cannot assign twice`) et même la correction (`help: … let mut total`).

### Chercher au mauvais endroit

La ligne indiquée n'est pas toujours celle à corriger. Pour un `end`
oublié, Ruby se plaint à la **fin** du fichier :

```text
syntaxe.rb:4: syntax error, unexpected end-of-input, expecting `end' or dummy end (SyntaxError)
```

Le problème est plus haut, à la ligne 1 : la fonction qui n'est jamais
fermée. Si la ligne indiquée semble juste, regardez les lignes **au-dessus**.

### Ne pas voir la différence entre un nombre et un texte

`puts` affiche `42` pour le nombre comme pour le texte `"42"` :

```ruby norun
age = "36"
puts age + 1
```

```text
main.rb:2:in `+': no implicit conversion of Integer into String (TypeError)

puts age + 1
           ^
	from main.rb:2:in `<main>'
```

Un `p age` aurait affiché `"36"`, avec les guillemets : c'est un texte.
Convertissez-le avec `age.to_i`, ou écrivez `age = 36`.

## Testez-vous

**1.** Votre programme affiche `20` au lieu de `15`, sans message. Quelle
sorte d'erreur est-ce ?

> **Réponse**
>
> Une erreur de logique : le programme marche, mais le résultat est faux.
> Affichez les valeurs intermédiaires avec `p`, `dbg!` ou `debug` pour
> trouver l'étape qui se trompe.

**2.** Dans le message `--> age.rs:2:20`, que veulent dire `2` et `20` ?

> **Réponse**
>
> L'erreur est dans le fichier `age.rs`, à la ligne 2, colonne 20 (le
> vingtième caractère de la ligne).

**3.** Que doit contenir une bonne demande d'aide ?

> **Réponse**
>
> Ce que vous voulez faire, un exemple minimal en texte, le résultat
> attendu, le résultat obtenu (ou le message d'erreur complet) et ce que
> vous avez déjà essayé.

## En résumé

- Trois sortes d'erreurs : de syntaxe (ne démarre pas), d'exécution (s'arrête en route), de logique (résultat faux).
- Un message d'erreur dit quoi, où (fichier, ligne, colonne) et souvent comment corriger : lisez-le depuis le haut.
- Affichez les valeurs avec `p` en Ruby, `dbg!` en Rust, `debug` en Rythme.
- Expliquez votre code au canard, et réduisez le problème à quelques lignes.
- Cherchez la partie générale du message en ligne ; pour demander de l'aide, donnez un exemple minimal.

Suite : [sauvegarder son travail avec Git](git.md).
