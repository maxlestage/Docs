# Rythme : introduction et installation

Rythme est un petit langage purement fonctionnel, au goût de Ruby, pensé pour apprendre à programmer sans se perdre : cette page explique ce qu'il est, comment l'installer et comment lancer votre premier programme.

## Qu'est-ce que Rythme

Rythme est un langage de programmation. Son interpréteur est écrit en Rust.
Les programmes sont des fichiers texte qui finissent par `.ry`.

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

Lisez-le de haut en bas : on prend la liste, on garde les nombres pairs,
on met chacun au carré, on fait la somme, on l'affiche.

## La philosophie du langage

Rythme veut être **plus simple que Python ou Ruby**. Il a seulement 15
mots-clés, et une seule façon de faire chaque chose.

- **Purement fonctionnel.** Une valeur ne change jamais. Il n'y a pas de
  boucles : on travaille sur des listes entières, ou avec des fonctions qui
  s'appellent elles-mêmes.
- **Sûr comme Rust.** Pas de `null`, pas d'exceptions. Une valeur qui peut
  manquer est une `Option`, une opération qui peut échouer donne un
  `Result`.
- **Agréable comme Ruby.** Des blocs `do |x| ... end`, des appels sans
  parenthèses (`puts "salut"`), des noms comme `even?`.
- **De l'aide partout.** Chaque erreur dit où elle est, pourquoi, et comment
  la corriger. Le programme est vérifié avant d'être lancé.
- **Six langues.** Les messages existent en français, anglais, espagnol,
  allemand, portugais et chinois.

> **Note :** les mots-clés et les fonctions intégrées sont en anglais
> (`if`, `map`, `print`), mais vos propres noms peuvent être en français,
> accents compris : `prénom`, `âge`.

## Installer Rythme

Sous **Linux et macOS**, ce script télécharge le programme, vérifie sa somme
SHA-256 et le place dans `~/.local/bin` :

```sh
curl -fsSL https://raw.githubusercontent.com/maxlestage/Rythme/master/install.sh | sh
```

Sous **Windows**, dans PowerShell :

```text
irm https://raw.githubusercontent.com/maxlestage/Rythme/master/install.ps1 | iex
```

Si vous avez déjà [Rust](https://rustup.rs), vous pouvez aussi construire
la version la plus récente :

```sh
cargo install --git https://github.com/maxlestage/Rythme
```

Vérifiez ensuite que tout marche :

```sh
rythme version
```

```text
rythme 0.1.0
```

## Essayer sans rien installer

Le [site de Rythme](https://maxlestage.github.io/Rythme/) a un **playground** : vous
écrivez du code dans la page, vous le lancez, et vous voyez le résultat. C'est
le vrai interpréteur, qui tourne dans le navigateur.

## Le mode interactif

Tapez `rythme` sans rien d'autre : c'est le mode interactif (le REPL). Vous
tapez une ligne, Rythme affiche le résultat.

```text
Rythme 0.1.0 — :help pour l'aide, :quit pour quitter
ry> 1 + 2
3
ry> nom = "Ada"
ry> "Bonjour, {nom}"
"Bonjour, Ada"
ry> :quit
```

> **Astuce :** dans le mode interactif, `:doc map` affiche l'aide de la
> fonction `map`, et `:quit` fait quitter.

## Votre premier programme

Créez un fichier `bonjour.ry` avec ce contenu :

```rythme
# Mon premier programme
print("Bonjour, le monde !")
```

```sortie
Bonjour, le monde !
```

Puis lancez-le dans un terminal :

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
chacune. Vous verrez à quoi elles ressemblent dans
[Gérer les erreurs](erreurs.md).

## Obtenir de l'aide

Rythme a son aide intégrée, en français :

```sh
rythme help            # la liste des sujets
rythme help map        # l'aide d'une fonction
rythme help builtins   # toutes les fonctions intégrées
rythme learn           # une visite guidée du langage
```

Pour choisir la langue des messages, ajoutez `--lang fr` avant la commande,
ou définissez la variable `RYTHME_LANG=fr`.

> **À retenir :** `rythme` ouvre le mode interactif, `rythme run fichier.ry`
> lance un programme, `rythme check fichier.ry` le vérifie, et
> `rythme help` répond à vos questions.

## Et ensuite

Continuez avec les [premiers pas](premiers-pas.md). Si vous connaissez déjà
Ruby ou Rust, comparez avec [l'introduction à Ruby](../ruby/introduction.md)
et [l'introduction à Rust](../rust/introduction.md).
