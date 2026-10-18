# Bien apprendre la programmation

On apprend à programmer en programmant, un peu chaque jour : cette page vous donne une méthode, des idées de projets, des conseils pour lire la documentation, poser des questions, utiliser l'IA, et un plan de quatre semaines.

> **Objectifs :**
>
> - suivre une méthode simple : lire, taper, modifier, casser ;
> - organiser votre pratique en séances courtes et régulières ;
> - trouver la réponse dans la documentation officielle ;
> - poser une bonne question, et utiliser un assistant d'IA sans vous priver d'apprendre ;
> - suivre un plan de quatre semaines dans le langage de votre choix.

## Programmer s'apprend comme un instrument

Personne n'apprend le piano en regardant des vidéos. Il faut poser les
doigts sur le clavier, se tromper, recommencer.

La programmation, c'est pareil : lire ce site ne suffit pas. Ce sont vos
doigts et vos erreurs qui vous apprennent. La bonne nouvelle : vingt
minutes par jour suffisent pour progresser vite.

## La méthode en quatre gestes

Pour chaque exemple de ce site, faites ces quatre gestes, dans l'ordre.

### 1. Lire

Lisez le code **avant** de le lancer, et devinez ce qu'il va afficher.
Écrivez votre réponse sur un papier. Prenez cet exemple Ruby :

```ruby
1.upto(3) do |n|
  puts "3 x #{n} = #{3 * n}"
end
```

Qu'affiche-t-il ? Réfléchissez, puis regardez :

```sortie
3 x 1 = 3
3 x 2 = 6
3 x 3 = 9
```

Si vous vous êtes trompé, tant mieux : vous venez d'apprendre quelque chose.

### 2. Taper le code vous-même

Ne copiez-collez pas. **Tapez** le code, lettre par lettre, dans votre
[éditeur](editeur.md). C'est plus lent, mais vos doigts retiennent la
syntaxe : où vont les parenthèses, les guillemets, le `end`.

### 3. Modifier

Changez une seule chose, et devinez le résultat avant de relancer. Par
exemple, en Rust, comptez à l'envers avec `.rev()` :

```rust
for n in (1..=3).rev() {
    println!("3 x {n} = {}", 3 * n);
}
```

```sortie
3 x 3 = 9
3 x 2 = 6
3 x 1 = 3
```

Idées de modifications : changer un nombre, ajouter une ligne, mettre une
autre table de multiplication, demander jusqu'à 10.

### 4. Casser

Enlevez exprès un morceau, et lisez le message d'erreur. En Rythme,
enlevons la dernière parenthèse :

```rythme norun
each(1..3, fn(n) = print("3 x {n} = {3 * n}")
```

```text
erreur[E001]: `,` ou `)` attendu, mais trouvé la fin du code
 --> main.ry:2:1
  |
2 | 
  | ^ `,` ou `)` attendu ici
  |
1 | each(1..3, fn(n) = print("3 x {n} = {3 * n}")
  |     - ouvert ici
```

Le jour où vous ferez cette erreur sans le vouloir, vous reconnaîtrez le
message tout de suite. Voici la version réparée :

```rythme
each(1..3, fn(n) = print("3 x {n} = {3 * n}"))
```

```sortie
3 x 1 = 3
3 x 2 = 6
3 x 3 = 9
```

> **À retenir :** lire, taper, modifier, casser. Les erreurs ne sont pas
> un échec : ce sont vos meilleures leçons. Voyez aussi
> [Trouver et corriger ses erreurs](deboguer.md).

## Pratiquer un peu chaque jour

Le cerveau retient mieux ce qu'il revoit plusieurs fois, avec des pauses
entre les révisions. C'est la **pratique espacée**.

- **Vingt à trente minutes par jour** valent mieux que cinq heures le
  dimanche.
- **Revenez sur un chapitre** deux jours plus tard, puis une semaine plus
  tard : refaites ses exemples de mémoire, sans regarder.
- **Commencez chaque séance** en réécrivant de mémoire un petit programme
  de la veille.
- **Arrêtez-vous** quand vous êtes fatigué : le cerveau continue de ranger
  pendant le repos et le sommeil.

> **Astuce :** les sections « Testez-vous » de chaque chapitre sont faites
> pour la révision. Répondez sans regarder, puis vérifiez.

## Des petits projets, par niveau

Un projet à vous motive bien plus qu'un exercice. Choisissez-en un petit,
finissez-le, puis passez au suivant.

| Niveau        | Idées de projets                                                       |
|---------------|-------------------------------------------------------------------------|
| Tout débutant | une table de multiplication ; un convertisseur euros ↔ dollars ; un compteur de voyelles dans une phrase |
| Débutant      | un jeu « devinez le nombre » ; une liste de courses ; un calcul de moyenne de notes |
| Intermédiaire | un carnet d'adresses enregistré dans un fichier ; un générateur de mots de passe ; un compteur de mots d'un livre |
| Avancé        | un petit jeu en terminal (pendu, morpion) ; un outil en ligne de commande ; un petit site ou une API |

Les pages de projets de chaque langage vous guident pas à pas :
[Ruby](../ruby/projets.md), [Rust](../rust/projets.md) et
[Rythme](../rythme/projets.md).

## Lire la documentation

La documentation officielle est la source la plus juste. Elle fait un peu
peur au début ; prenez l'habitude d'y aller tôt.

- **Ruby** : [ruby-doc.org](https://ruby-doc.org/) décrit chaque classe.
  Cherchez par exemple `String` pour voir toutes les méthodes des textes.
- **Rust** : [doc.rust-lang.org/std](https://doc.rust-lang.org/std/)
  décrit la bibliothèque standard. La commande `rustup doc --std` l'ouvre
  même sans Internet.
- **Rythme** : la commande `rythme help` liste les sujets, et
  `rythme help` suivi d'un nom explique une fonction.

```sh
rythme help reverse
```

```text
reverse(x)
  La liste ou le texte dans l'ordre inverse.
  Exemple :
    reverse([1, 2, 3])         # [3, 2, 1]
```

Le code lui-même peut aussi vous renseigner. En Ruby, `methods` liste ce
qu'une valeur sait faire :

```ruby
p "bonjour".methods.grep(/case/).sort
puts 5.respond_to?(:even?)
```

```sortie
[:casecmp, :casecmp?, :downcase, :downcase!, :swapcase, :swapcase!, :upcase, :upcase!]
true
```

> **Astuce :** dans une page de documentation, lisez d'abord l'**exemple**.
> Il en dit souvent plus long que la description.

## Poser une bonne question

Vous êtes bloqué depuis vingt minutes ? Demandez de l'aide. Une bonne
question reçoit une réponse rapide. Elle contient :

1. **ce que vous voulez faire** : « je veux afficher les nombres pairs
   d'une liste » ;
2. **le code**, le plus court possible, qui montre le problème (copié en
   texte, pas en photo) ;
3. **le message d'erreur complet**, copié tel quel ;
4. **ce que vous avez déjà essayé**.

Souvent, en écrivant la question, vous trouvez la réponse vous-même. C'est
la méthode du [canard en plastique](deboguer.md#le-canard-en-plastique).

## Les communautés en français

Vous n'êtes pas seul. Des personnes aident gratuitement, en français :

- **Zeste de Savoir** (zestedesavoir.com) : des tutoriels et un forum
  bienveillant pour les débutants ;
- **les forums de Developpez** (developpez.net) : des sections Ruby et Rust ;
- **les rencontres locales** (*meetups*) : Paris.rb pour Ruby, des groupes
  Rust dans plusieurs grandes villes, souvent annoncés sur meetup.com ;
- **Rythme** : les questions et les bugs se posent sur la page GitHub du
  projet.

Beaucoup de ressources restent en anglais. Un traducteur en ligne vous aide
à les lire : ne vous en privez pas.

## Utiliser un assistant d'IA avec sagesse

Un assistant d'IA (un chatbot qui écrit du code) peut être un excellent
professeur, ou vous empêcher d'apprendre. Tout dépend de la façon de s'en
servir.

**Bonnes façons :**

- lui demander d'**expliquer** un message d'erreur ou un morceau de code ;
- lui demander un **indice**, pas la solution : « donne-moi une piste,
  sans le code » ;
- lui faire **relire** votre code une fois qu'il marche, pour apprendre à
  mieux l'écrire ;
- lui demander de vous **poser des questions** pour vérifier que vous avez
  compris.

**Mauvaises façons :**

- copier-coller sa réponse sans la comprendre ;
- lui faire faire les exercices : c'est vous qui devez progresser, pas lui.

> **Attention :** une IA se trompe parfois avec beaucoup d'assurance. Elle
> peut inventer une méthode qui n'existe pas. Testez toujours son code, et
> vérifiez dans la documentation.

La règle d'or : **si vous ne pouvez pas expliquer chaque ligne, ne
l'utilisez pas.**

## Un plan de quatre semaines

Choisissez **un** langage (voyez [Quel langage choisir ?](choisir.md)) et
suivez son plan. Comptez quatre à cinq séances par semaine. À la fin de
chaque semaine, faites un des [petits projets](#des-petits-projets-par-niveau).

### En Ruby

| Semaine | Chapitres                                                                                                   | À la fin, vous savez…                   |
|---------|-------------------------------------------------------------------------------------------------------------|-----------------------------------------|
| 1       | [Introduction](../ruby/introduction.md), [Premiers pas](../ruby/premiers-pas.md), [Nombres](../ruby/nombres.md), [Conditions et boucles](../ruby/conditions-boucles.md) | calculer, choisir, répéter              |
| 2       | [Fonctions](../ruby/fonctions.md), [Texte](../ruby/texte.md), [Collections](../ruby/collections.md), [Intervalles](../ruby/intervalles.md) | ranger des données et les transformer   |
| 3       | [Blocs](../ruby/blocs.md), [Erreurs](../ruby/erreurs.md), [Fichiers](../ruby/fichiers.md), [Exercices](../ruby/exercices.md) | lire un fichier et gérer les erreurs    |
| 4       | [Types et classes](../ruby/types.md), [Tests](../ruby/tests.md), [Projet guidé](../ruby/projet.md)          | finir un vrai petit programme           |

### En Rust

| Semaine | Chapitres                                                                                                   | À la fin, vous savez…                   |
|---------|-------------------------------------------------------------------------------------------------------------|-----------------------------------------|
| 1       | [Introduction](../rust/introduction.md), [Premiers pas](../rust/premiers-pas.md), [Nombres](../rust/nombres.md), [Conditions et boucles](../rust/conditions-boucles.md) | compiler, calculer, choisir, répéter    |
| 2       | [Fonctions](../rust/fonctions.md), [Texte](../rust/texte.md), [Collections](../rust/collections.md), [Possession](../rust/possession.md) | comprendre qui possède une valeur       |
| 3       | [Types](../rust/types.md), [Pattern matching](../rust/pattern-matching.md), [Erreurs](../rust/erreurs.md), [Itérateurs](../rust/iterateurs.md) | modéliser des données et gérer les erreurs |
| 4       | [Modules](../rust/modules.md), [Fichiers](../rust/fichiers.md), [Tests](../rust/tests.md), [Projet guidé](../rust/projet.md) | finir un vrai petit programme           |

### En Rythme

| Semaine | Chapitres                                                                                                   | À la fin, vous savez…                   |
|---------|-------------------------------------------------------------------------------------------------------------|-----------------------------------------|
| 1       | [Introduction](../rythme/introduction.md), [Premiers pas](../rythme/premiers-pas.md), [Nombres](../rythme/nombres.md), [Conditions et boucles](../rythme/conditions-boucles.md) | calculer, choisir, répéter              |
| 2       | [Fonctions](../rythme/fonctions.md), [Texte](../rythme/texte.md), [Collections](../rythme/collections.md), [Le style fonctionnel](../rythme/fonctionnel.md) | transformer des listes sans boucle      |
| 3       | [Récursion](../rythme/recursion.md), [Pattern matching](../rythme/pattern-matching.md), [Erreurs](../rythme/erreurs.md), [Fichiers](../rythme/fichiers.md) | découper un problème et gérer les erreurs |
| 4       | [Types](../rythme/types.md), [Tests](../rythme/tests.md), [Exercices](../rythme/exercices.md), [Projet guidé](../rythme/projet.md) | finir un vrai petit programme           |

> **Note :** prendre deux semaines au lieu d'une n'est pas un problème. Le
> rythme régulier compte plus que la vitesse.

## Erreurs fréquentes

### Copier du code avec des guillemets typographiques

Un code copié depuis un traitement de texte ou une page web contient
parfois des guillemets courbes `“ ”` au lieu des guillemets droits `"` :

```ruby norun
puts “Bonjour”
```

```text
main.rb:1:in `<main>': undefined local variable or method `“Bonjour”' for main (NameError)
```

En Rythme, le message vous dit directement quoi faire :

```rythme norun
print(“Bonjour”)
```

```text
erreur[E018]: caractère inattendu `“`
 --> main.ry:1:7
  |
1 | print(“Bonjour”)
  |       ^
  |
  = aide: utilisez des guillemets droits `"` pour les textes
```

Tapez le code vous-même : c'est la meilleure protection.

### Faire confiance à une méthode inventée

Un assistant d'IA, ou un site peu sérieux, vous propose une méthode qui
semble logique, mais qui n'existe pas :

```ruby norun
puts "bonjour le monde".capitalize_words
```

```text
main.rb:1:in `<main>': undefined method `capitalize_words' for an instance of String (NoMethodError)

puts "bonjour le monde".capitalize_words
                       ^^^^^^^^^^^^^^^^^
Did you mean?  capitalize
```

Vérifiez dans la documentation. Ici, on met une majuscule à chaque mot
avec `"bonjour le monde".split.map(&:capitalize).join(" ")`.

### Mélanger les langages

Après avoir lu du Ruby ou du Python, on oublie facilement le `!` de
`println!` en Rust :

```rust norun
fn main() {
    println("Bonjour");
}
```

```text
error[E0423]: expected function, found macro `println`
 --> main.rs:2:5
  |
2 |     println("Bonjour");
  |     ^^^^^^^ not a function
  |
help: use `!` to invoke the macro
  |
2 |     println!("Bonjour");
  |            +
```

Apprenez un langage à la fois, et gardez son aide-mémoire sous la main :
[Ruby](../ruby/aide-memoire.md), [Rust](../rust/aide-memoire.md),
[Rythme](../rythme/aide-memoire.md).

## Testez-vous

**1.** Quels sont les quatre gestes de la méthode, dans l'ordre ?

> **Réponse**
>
> Lire (et deviner le résultat), taper le code soi-même, le modifier, le
> casser exprès pour lire l'erreur.

**2.** Vaut-il mieux programmer quatre heures le samedi ou trente minutes
chaque jour ?

> **Réponse**
>
> Trente minutes chaque jour : la pratique espacée aide le cerveau à
> retenir, et les révisions régulières fixent ce que vous avez appris.

**3.** Un assistant d'IA vous donne un code qui marche, mais vous ne
comprenez pas deux lignes. Que faites-vous ?

> **Réponse**
>
> Vous lui demandez de les expliquer, vous vérifiez dans la documentation,
> puis vous réécrivez le code vous-même. Pas de ligne que vous ne savez
> pas expliquer.

## En résumé

- On apprend en pratiquant : lire, taper, modifier, casser.
- Un peu chaque jour, avec des révisions espacées, vaut mieux qu'une longue séance.
- De petits projets finis motivent plus que de grands projets abandonnés.
- La documentation officielle (ruby-doc.org, doc.rust-lang.org/std, `rythme help`) est votre meilleure amie.
- Une bonne question contient le but, le code court, l'erreur complète et vos essais.
- L'IA aide à comprendre ; elle ne doit pas faire le travail à votre place.

Suite : [quel langage choisir ?](choisir.md).
