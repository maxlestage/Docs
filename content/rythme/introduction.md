# Rythme : introduction et installation

Rythme est un petit langage purement fonctionnel, au goût de Ruby, pensé pour apprendre à programmer sans se perdre : cette page explique ce qu'il est, comment l'installer et comment lancer votre premier programme.

> **Objectifs :**
>
> - vous saurez dire ce qu'est Rythme et ce qui le rend différent ;
> - vous saurez installer Rythme, ou l'essayer sans rien installer ;
> - vous saurez essayer une ligne de code dans le mode interactif ;
> - vous saurez écrire un fichier `.ry`, le vérifier et le lancer ;
> - vous saurez lire un premier message d'erreur.

## Qu'est-ce que Rythme

Un **langage de programmation** est une langue précise, que l'ordinateur sait
suivre. Un **programme** est un texte écrit dans cette langue : une suite
d'ordres, comme une recette de cuisine.

Rythme est un de ces langages. Les programmes Rythme sont des fichiers texte
qui finissent par `.ry`. Un programme, l'**interpréteur** `rythme`, les lit et
les exécute (voir [interpréteur](../debuter/glossaire.md#interpreteur)). Cet
interpréteur est écrit dans un autre langage, Rust.

Voici un programme complet :

```rythme
fn carre(x) = x * x

[1, 2, 3, 4]
  |> filter(is_even)
  |> map(carre)
  |> sum
  |> print
```

```sortie
20
```

Lisez-le de haut en bas, comme une chaîne de montage :

1. `fn carre(x) = x * x` définit une [fonction](../debuter/glossaire.md#fonction)
   `carre` : elle reçoit un nombre `x` et donne `x * x` ;
2. `[1, 2, 3, 4]` est une [liste](../debuter/glossaire.md#liste) de nombres ;
3. `|>` passe la valeur à l'étape suivante : on garde les nombres pairs
   (`[2, 4]`), on met chacun au carré (`[4, 16]`), on fait la somme (`20`),
   et on l'affiche.

Vous n'avez pas besoin de tout comprendre maintenant : chaque morceau a son
chapitre.

## La philosophie du langage

Rythme veut être **plus simple que Python ou Ruby**. Il a seulement 15
**mots-clés** (les mots réservés au langage, comme `if` ou `fn`), et une
seule façon de faire chaque chose.

- **Purement fonctionnel.** Une valeur ne change jamais : elle est
  [immuable](../debuter/glossaire.md#immuable). Il n'y a pas de boucles : on
  travaille sur des listes entières, ou avec des fonctions qui s'appellent
  elles-mêmes.
- **Sûr comme Rust.** Pas de [`null`](../debuter/glossaire.md#null), pas
  d'exceptions. Une valeur qui peut manquer est une `Option`, une opération
  qui peut échouer donne un `Result`.
- **Agréable comme Ruby.** Des blocs `do |x| ... end`, des appels sans
  parenthèses (`puts "salut"`), des noms comme `even?`.
- **De l'aide partout.** Chaque erreur dit où elle est, pourquoi, et comment
  la corriger. Le programme est vérifié avant d'être lancé.
- **Six langues.** Les messages existent en français, anglais, espagnol,
  allemand, portugais et chinois.

Pensez à Rythme comme à une cuisine bien rangée : moins d'ustensiles, mais
chacun a sa place, et rien ne change de place dans votre dos.

> **Note :** les mots-clés et les fonctions intégrées sont en anglais
> (`if`, `map`, `print`), mais vos propres noms peuvent être en français,
> accents compris : `prénom`, `âge`.

## Installer Rythme

On installe Rythme en tapant une commande dans un **terminal**, la fenêtre
où l'on écrit des commandes au lieu de cliquer. Si vous n'en avez jamais
utilisé, lisez d'abord [le terminal](../debuter/terminal.md).

Sous **Linux et macOS**, cette commande télécharge Rythme et l'installe :

```sh
curl -fsSL https://raw.githubusercontent.com/maxlestage/Rythme/master/install.sh | sh
```

Sous **Windows**, dans PowerShell :

```text
irm https://raw.githubusercontent.com/maxlestage/Rythme/master/install.ps1 | iex
```

Vérifiez ensuite que tout marche :

```sh
rythme version
```

```text
rythme 0.1.0
```

Si le terminal répond « commande introuvable », fermez-le, rouvrez-le, et
réessayez : il doit relire la liste des programmes installés.

## Essayer sans rien installer

Le [site de Rythme](https://maxlestage.github.io/Rythme/) a un **playground** : vous
écrivez du code dans la page, vous le lancez, et vous voyez le résultat. C'est
le vrai interpréteur, qui tourne dans le navigateur. Seule différence : il ne
peut pas lire ni écrire de fichiers sur votre ordinateur.

## Le mode interactif

Tapez `rythme` sans rien d'autre : c'est le **mode interactif** (on dit
aussi [REPL](../debuter/glossaire.md#repl)). Vous tapez une ligne, Rythme
affiche tout de suite le résultat. C'est une calculatrice très savante.

```text
Rythme 0.1.0 — :help pour l'aide, :quit pour quitter
ry> 1 + 2
3
ry> nom = "Ada"
ry> "Bonjour, {nom}"
"Bonjour, Ada"
ry> :quit
```

`ry>` est l'**invite** : Rythme attend votre ligne. Vous ne la tapez pas.

> **Astuce :** dans le mode interactif, `:doc map` affiche l'aide de la
> fonction `map`, et `:quit` fait quitter.

## Votre premier programme

Le mode interactif oublie tout quand on le quitte. Pour garder un programme,
on l'écrit dans un fichier. Créez avec votre
[éditeur de code](../debuter/editeur.md) un fichier `bonjour.ry` avec ce
contenu :

```rythme
# Mon premier programme
print("Bonjour, le monde !")
```

```sortie
Bonjour, le monde !
```

- La ligne qui commence par `#` est un **commentaire** : une note pour les
  humains, que Rythme ignore.
- `print` est une fonction qui affiche ce qu'on lui donne entre parenthèses.
- `"Bonjour, le monde !"` est un **texte** : il s'écrit entre guillemets
  droits `"`.

Puis lancez-le dans un terminal, depuis le dossier du fichier (voir
[le terminal](../debuter/terminal.md)) :

```sh
rythme run bonjour.ry
```

`rythme bonjour.ry` marche aussi, sans le mot `run`.

## Vérifier sans lancer

`rythme check` cherche les problèmes sans exécuter le programme :

```sh
rythme check bonjour.ry
```

```text
✓ bonjour.ry : aucun problème trouvé
```

S'il trouve des erreurs, il les montre toutes d'un coup, avec une aide pour
chacune. `rythme run` fait aussi cette vérification avant de lancer : un
programme qui a une erreur ne démarre pas du tout.

## Obtenir de l'aide

Rythme a son aide intégrée, en français :

```sh
rythme help            # la liste des sujets
rythme help map        # l'aide d'une fonction
rythme help builtins   # toutes les fonctions intégrées
rythme learn           # une visite guidée du langage
rythme explain E003    # l'explication d'un code d'erreur
```

Pour choisir la langue des messages, ajoutez `--lang fr` avant la commande,
ou définissez la variable `RYTHME_LANG=fr`.

## Erreurs fréquentes

Dans les messages ci-dessous, le fichier s'appelle `main.ry`. Chaque message
donne le fichier, la ligne et la colonne (`main.ry:1:7` : ligne 1, 7ᵉ
caractère), puis montre l'endroit exact avec `^`.

### Oublier les guillemets

```rythme norun
print(Bonjour)
```

```text
erreur[E003]: nom inconnu `Bonjour`
 --> main.ry:1:7
  |
1 | print(Bonjour)
  |       ^^^^^^^ non défini
  |
  = aide: pour créer votre propre sorte de valeur, définissez un type : `type Nom = Bonjour(champ) | Autre`
  = en savoir plus : `rythme explain E003`
```

Sans guillemets, `Bonjour` est lu comme un **nom** de valeur, et ce nom
n'existe pas. Pour un texte, mettez des guillemets : `print("Bonjour")`.

### Des apostrophes au lieu des guillemets

```rythme norun
print('Bonjour')
```

```text
erreur[E018]: caractère inattendu `'`
 --> main.ry:1:7
  |
1 | print('Bonjour')
  |       ^
  |
  = aide: les textes s'écrivent entre guillemets droits : "bonjour"
  = en savoir plus : `rythme explain E018`
```

D'autres langages acceptent `'...'`. Rythme veut toujours des guillemets
droits : `print("Bonjour")`.

### Oublier le guillemet de fin

```rythme norun
print("Bonjour)
```

```text
erreur[E002]: ce texte n'est jamais fermé
 --> main.ry:1:7
  |
1 | print("Bonjour)
  |       ^ le texte commence ici
  |
  = aide: ajoutez un `"` à la fin du texte
  = en savoir plus : `rythme explain E002`
```

Rythme lit tout ce qui suit le premier `"` comme du texte, jusqu'au bout du
fichier. Fermez le texte : `print("Bonjour")`.

## Testez-vous

**Question 1 :** quelle commande vérifie un programme `jeu.ry` sans le
lancer ?

> **Réponse**
>
> `rythme check jeu.ry`. Pour le lancer, c'est `rythme run jeu.ry` (ou
> `rythme jeu.ry`).

**Question 2 :** qu'affiche ce programme ?

```rythme norun
# print("un")
print("deux")
```

> **Réponse**
>
> Seulement `deux`. La première ligne commence par `#` : c'est un
> commentaire, Rythme l'ignore.

**Question 3 :** dans le premier exemple, pourquoi le résultat est-il 20 et
pas 30 (1 + 4 + 9 + 16) ?

> **Réponse**
>
> Parce que `filter(is_even)` garde seulement les nombres pairs, 2 et 4,
> avant de les mettre au carré : 4 + 16 = 20.

## En résumé

- Rythme est un petit langage purement fonctionnel : les valeurs ne
  changent jamais, et il n'y a pas de boucles.
- Les programmes sont des fichiers `.ry`, lancés avec `rythme run fichier.ry`.
- `rythme` tout seul ouvre le mode interactif ; `rythme check fichier.ry`
  vérifie un programme sans le lancer.
- Un texte s'écrit entre guillemets droits `"`, et `#` commence un
  commentaire.
- Chaque message d'erreur donne l'endroit, la cause et une aide ;
  `rythme help` et `rythme explain` en disent plus, en français.

Suite : [les premiers pas](premiers-pas.md). Si vous connaissez déjà Ruby ou
Rust, comparez avec [l'introduction à Ruby](../ruby/introduction.md) et
[l'introduction à Rust](../rust/introduction.md).
