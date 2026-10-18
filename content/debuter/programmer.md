# Qu'est-ce que programmer ?

Programmer, c'est écrire des instructions qu'un ordinateur suit à la lettre : cette page explique ce qu'est un programme, un langage, un bug, et comment on apprend.

> **Objectifs :**
>
> - savoir ce qu'est un programme et un langage de programmation ;
> - reconnaître le code source et ses extensions de fichier ;
> - lire un même petit programme en Ruby, en Rust et en Rythme ;
> - distinguer un langage interprété d'un langage compilé ;
> - savoir ce qu'est un bug, et comment on apprend à programmer.

## Un programme, c'est une liste d'instructions

Un ordinateur ne devine rien. Il fait exactement ce qu'on lui dit, dans
l'ordre, très vite.

Un **programme** est une liste d'instructions écrites pour lui. Par exemple :

```text
1. Demander le prénom de la personne.
2. Afficher « Bonjour » suivi de ce prénom.
3. S'arrêter.
```

**Programmer**, c'est écrire ces instructions. La personne qui programme
s'appelle un développeur ou une développeuse.

## Comme une recette de cuisine

Une recette ressemble beaucoup à un programme :

- une liste d'**ingrédients** : ce sont les **données** (des nombres, des
  textes) ;
- des **étapes** dans un ordre précis : ce sont les **instructions** ;
- des **choix** : « si la pâte est trop épaisse, ajoutez du lait » ;
- des **répétitions** : « remuez jusqu'à ce que ce soit lisse ».

La grande différence : un cuisinier comprend « une pincée de sel ».
L'ordinateur, non. Il faut tout dire, sans ambiguïté.

> **À retenir :** un programme est une recette très précise, écrite pour
> quelqu'un qui obéit sans réfléchir.

## Le code source

Les instructions s'écrivent dans un **fichier texte**, comme une note. Ce
texte s'appelle le **code source**, ou simplement le **code**.

Le nom du fichier finit par une **extension** qui dit dans quel langage il
est écrit : `.rb` pour Ruby, `.rs` pour Rust, `.ry` pour Rythme.

Pour écrire ce texte, on utilise un **éditeur de code** : voyez
[Installer un éditeur de code](editeur.md).

## Les langages de programmation

L'ordinateur ne comprend pas le français. Il comprend un **langage de
programmation** : un vocabulaire réduit, avec des règles strictes. Ces règles
s'appellent la **syntaxe**.

Il existe des centaines de langages. Cette documentation en présente trois :
Ruby, Rust et Rythme. Ils disent souvent la même chose avec des mots
différents.

## Un même programme dans trois langages

Ce petit programme range un prénom dans une **variable**, dit bonjour, puis
affiche le résultat d'un calcul.

En Ruby :

```ruby
prenom = "Ada"
puts "Bonjour, #{prenom} !"
puts 2 + 3
```

```sortie
Bonjour, Ada !
5
```

En Rust :

```rust
let prenom = "Ada";
println!("Bonjour, {prenom} !");
println!("{}", 2 + 3);
```

```sortie
Bonjour, Ada !
5
```

En Rythme :

```rythme
prenom = "Ada"
print("Bonjour, {prenom} !")
print(2 + 3)
```

```sortie
Bonjour, Ada !
5
```

Les trois font la même chose. Seuls les mots changent : `puts`, `println!`
ou `print` veulent tous dire « affiche ».

## Interprété ou compilé

L'ordinateur doit transformer votre texte en actions. Il y a deux façons de
faire.

- **Interprété** : un programme, l'**interpréteur**, lit votre code et
  l'exécute tout de suite, ligne après ligne. C'est le cas de **Ruby** et de
  **Rythme**.
- **Compilé** : un programme, le **compilateur**, traduit d'abord tout votre
  code en langage machine. Ensuite seulement, on lance le résultat. C'est le
  cas de **Rust**.

Pensez à un livre étranger. L'interprète vous le lit à voix haute, en
traduisant au fur et à mesure. Le traducteur, lui, écrit d'abord tout le
livre en français : c'est plus long au début, mais la lecture est ensuite
très rapide.

> **Note :** un compilateur vérifie beaucoup de choses avant de lancer quoi
> que ce soit. Il trouve des erreurs plus tôt, mais il est aussi plus
> exigeant.

## Qu'est-ce qu'un bug ?

Un **bug** est une erreur dans un programme. Le programme ne marche pas, ou
il ne fait pas ce que vous vouliez.

Il y a deux grandes sortes de bugs :

- l'ordinateur **refuse** votre code : une faute de syntaxe, comme une
  parenthèse oubliée. Il vous affiche un message d'erreur ;
- le code **marche, mais donne un mauvais résultat** : vous avez écrit
  `a - b` au lieu de `a + b`. L'ordinateur ne peut pas le deviner.

Chercher et corriger un bug s'appelle le **débogage**.

> **À retenir :** tout le monde fait des bugs, tout le temps, même les
> experts. Ce n'est pas un échec : c'est le travail normal.

## Comment on apprend à programmer

On n'apprend pas à programmer en lisant seulement. On apprend en essayant.

1. **Lisez** un exemple court.
2. **Tapez-le** vous-même, sans copier-coller.
3. **Lancez-le** et regardez ce qui se passe.
4. **Changez** quelque chose : un nombre, un mot. Relancez.
5. **Trompez-vous**, et lisez le message d'erreur.

Un message d'erreur n'est pas un reproche. C'est un indice. Il dit souvent
**où** est le problème (le fichier et la ligne) et **pourquoi**.

> **Astuce :** lisez toujours la première ligne d'un message d'erreur en
> entier, lentement. La réponse s'y trouve souvent.

## Et ensuite

Avant d'écrire du code, il faut apprendre à découper un problème en
étapes : c'est l'objet de [penser comme un programmeur](algorithmes.md).
Pour lancer un programme, il faudra ensuite ouvrir un terminal : voyez
[le terminal en 10 minutes](terminal.md). Les mots nouveaux sont expliqués
dans le [glossaire](glossaire.md).

## Erreurs fréquentes

### Écrire les instructions en français

L'ordinateur ne connaît que les mots de son langage. `afficher` n'existe pas
en Ruby :

```ruby norun
afficher "Bonjour"
```

```text
fr.rb:1:in `<main>': undefined method `afficher' for main (NoMethodError)

afficher "Bonjour"
^^^^^^^^
```

En Ruby, « afficher » se dit `puts` : `puts "Bonjour"`.

### Oublier les guillemets autour d'un texte

Sans guillemets, le langage croit que `Bonjour` est un nom qu'il devrait
connaître :

```ruby norun
puts Bonjour
```

```text
q.rb:1:in `<main>': uninitialized constant Bonjour (NameError)

puts Bonjour
     ^^^^^^^
```

Un texte s'écrit toujours entre guillemets : `puts "Bonjour"`.

### Changer une majuscule

Pour l'ordinateur, `prenom` et `Prenom` sont deux noms différents :

```ruby norun
prenom = "Ada"
puts "Bonjour, " + Prenom
```

```text
pm.rb:2:in `<main>': uninitialized constant Prenom (NameError)

puts "Bonjour, " + Prenom
                   ^^^^^^
```

Écrivez le nom exactement comme vous l'avez créé : `prenom`.

## Testez-vous

**1.** Quel est le point commun entre un programme et une recette de
cuisine ?

> **Réponse**
>
> Les deux sont des listes d'instructions dans un ordre précis, avec des
> données (les ingrédients), des choix et des répétitions. Mais le
> programme doit être bien plus précis : l'ordinateur ne devine rien.

**2.** Quelle est la différence entre un langage interprété et un langage
compilé ? Donnez un exemple de chaque.

> **Réponse**
>
> Un interpréteur lit et exécute le code tout de suite (Ruby, Rythme). Un
> compilateur traduit d'abord tout le code, puis on lance le résultat
> (Rust).

**3.** Votre programme se lance mais affiche un mauvais résultat. Est-ce un
bug ?

> **Réponse**
>
> Oui. Un bug n'est pas seulement un programme qui plante : c'est aussi un
> programme qui ne fait pas ce que vous vouliez.

## En résumé

- Un programme est une liste d'instructions précises, comme une recette.
- Le code source est un fichier texte écrit dans un langage de programmation.
- Ruby et Rythme sont interprétés ; Rust est compilé avant d'être lancé.
- Un bug est une erreur dans le programme, et tout le monde en fait.
- On apprend en essayant, en se trompant et en lisant les messages d'erreur.

Suite : [penser comme un programmeur](algorithmes.md).
