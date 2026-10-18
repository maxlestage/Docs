# Le vérificateur et les messages d'erreur

Avant d'exécuter la moindre ligne, Rythme relit tout votre programme pour y chercher des erreurs : cette page explique ce qu'il vérifie, comment lire ses messages, et comment s'en servir pour corriger vite.

> **Objectifs :**
>
> - vous saurez ce que vérifie Rythme avant d'exécuter un programme, et lancer `rythme check` ;
> - vous saurez lire un message : où, pourquoi, comment corriger ;
> - vous saurez faire la différence entre une erreur (`E…`) et un avertissement (`W…`) ;
> - vous saurez obtenir l'explication d'un code avec `rythme explain` ;
> - vous saurez reconnaître les fautes de frappe et les habitudes venues d'autres langages.

## Vérifier avant d'exécuter

Pensez au correcteur d'orthographe d'un traitement de texte : il souligne
les fautes **avant** que vous n'envoyiez la lettre. Rythme fait pareil avec
votre code. Chaque programme est lu et vérifié **en entier** avant de
démarrer, par `rythme fichier.ry` comme par `rythme test`. S'il y a une
erreur, rien ne s'exécute.

`rythme check` fait seulement cette vérification, sans rien lancer :

```rythme
fn double(x) = x * 2

print(double(21))
```

```sortie
42
```

```text
$ rythme check ok.ry
✓ ok.ry : aucun problème trouvé
```

Le vérificateur regarde **tout** le code, même celui qui ne s'exécute pas
à chaque fois. Ici, la fonction `rapport` n'est jamais appelée, et
pourtant ses erreurs sont trouvées :

```rythme norun
fn aire(largeur, hauteur) = largeur * hauteru

fn rapport()
  total = aire(2)
  print(totl)
end

prix = 10
prix = 12
```

```text
$ rythme check main.ry
erreur[E003]: nom inconnu `hauteru`
 --> main.ry:1:39
  |
1 | fn aire(largeur, hauteur) = largeur * hauteru
  |                                       ^^^^^^^ non défini
  |
  = aide: vouliez-vous dire `hauteur` ?
  = en savoir plus : `rythme explain E003`

erreur[E004]: `aire` attend 2 arguments, mais en a reçu 1
 --> main.ry:4:11
  |
4 |   total = aire(2)
  |           ^^^^^^^ 1 argument(s) ici
  |
1 | fn aire(largeur, hauteur) = largeur * hauteru
  |    ---- définie ici
  |
  = aide: utilisation : `aire(largeur, hauteur)`
  = en savoir plus : `rythme explain E004`

erreur[E003]: nom inconnu `totl`
 --> main.ry:5:9
  |
5 |   print(totl)
  |         ^^^^ non défini
  |
  = aide: vouliez-vous dire `total` ?
  = en savoir plus : `rythme explain E003`

erreur[E011]: `prix` est déjà défini
 --> main.ry:9:1
  |
9 | prix = 12
  | ^^^^ défini à nouveau ici
  |
8 | prix = 10
  | ---- défini une première fois ici
  |
  = aide: les valeurs ne changent jamais en Rythme : donnez un nouveau nom à la nouvelle valeur, comme `total2 = total + 1`
  = en savoir plus : `rythme explain E011`

avertissement[W001]: `total` n'est jamais utilisé
 --> main.ry:4:3
  |
4 |   total = aire(2)
  |   ^^^^^
  |
  = aide: supprimez-le, ou nommez-le `_total` si c'est voulu

✗ main.ry : 4 erreur(s)
```

Tous les problèmes arrivent **d'un coup**, les erreurs d'abord : vous
pouvez tout corriger en une fois. Seule exception : une erreur de
[syntaxe](../debuter/glossaire.md#syntaxe) (une parenthèse oubliée, par
exemple) arrête la lecture à la première, car la suite ne peut plus être
comprise.

## Ce que le vérificateur trouve

Avant l'exécution, Rythme trouve notamment :

- les noms inconnus, souvent des fautes de frappe (E003) ;
- les valeurs utilisées avant d'être définies (E003) ;
- les appels avec trop ou trop peu d'arguments (E004) ;
- les noms définis deux fois, et les tentatives de modifier une valeur (E011) ;
- les lignes qui calculent une valeur sans rien en faire (E024) ;
- les opérations qui échoueraient à coup sûr, comme `"a" * 2` (E005) ;
- les `if` sans `else` (E019) et les mots d'autres langages (E001).

Ce qui dépend des valeurs reçues pendant l'exécution (une division par une
valeur qui vaudra zéro, un indice trop grand...) ne peut être trouvé
qu'en lançant le programme. Le vérificateur ne signale que ce dont il est
**sûr**.

## Lire un message : où, pourquoi, comment

Prenons le message E004 ci-dessus, morceau par morceau :

```text
erreur[E004]: `aire` attend 2 arguments, mais en a reçu 1
 --> main.ry:4:11
  |
4 |   total = aire(2)
  |           ^^^^^^^ 1 argument(s) ici
  |
1 | fn aire(largeur, hauteur) = largeur * hauteru
  |    ---- définie ici
  |
  = aide: utilisation : `aire(largeur, hauteur)`
  = en savoir plus : `rythme explain E004`
```

1. **Pourquoi** : la première ligne donne le code (`E004`) et ce qui ne va
   pas, en une phrase.
2. **Où** : `main.ry:4:11` veut dire fichier `main.ry`, ligne 4, colonne 11.
   Puis la ligne elle-même, avec des `^` sous l'endroit exact.
3. **Le contexte** : un autre endroit lié, marqué de `-` : ici, la
   définition de `aire`, pour comparer.
4. **Comment corriger** : les lignes `= aide:` proposent une solution, et
   `= note:` donne une information de plus.
5. **Pour aller plus loin** : la commande qui explique ce code.

Lisez dans cet ordre : la phrase, l'endroit, puis l'aide. La plupart du
temps, l'aide suffit.

## Erreurs et avertissements

Un message commence par `erreur` ou par `avertissement` :

- une **erreur** (code `E…`) empêche le programme de démarrer ;
- un **avertissement** (code `W…`) signale un problème *probable*, mais le
  programme s'exécute quand même.

```rythme norun
fn moitie(x)
  inutile = x + 1
  x / 2
end

print(moitie(10))
```

```text
$ rythme moitie.ry
avertissement[W001]: `inutile` n'est jamais utilisé
 --> moitie.ry:2:3
  |
2 |   inutile = x + 1
  |   ^^^^^^^
  |
  = aide: supprimez-le, ou nommez-le `_inutile` si c'est voulu

5.0
```

Le programme a bien affiché `5.0`. Mais un avertissement cache souvent un
oubli : ici, un calcul qui ne sert à rien. Prenez l'habitude de corriger
aussi les avertissements. Si un nom inutilisé est voulu, commencez-le par
`_` : `_inutile`.

## Les fautes de frappe

Quand un nom est inconnu, Rythme cherche le nom existant le plus proche :

```text
erreur[E003]: nom inconnu `pirnt`
 --> main.ry:1:1
  |
1 | pirnt("bonjour")
  | ^^^^^ non défini
  |
  = aide: vouliez-vous dire `print` ?
  = en savoir plus : `rythme explain E003`
```

Il connaît aussi les noms d'autres langages. `lenght` (faute de frappe
fréquente) ou `strlen` (le nom en C) donnent la même aide :

```text
erreur[E003]: nom inconnu `strlen`
 --> main.ry:1:7
  |
1 | print(strlen("abc"))
  |       ^^^^^^ non défini
  |
  = aide: en Rythme, cela s'appelle `len`
  = en savoir plus : `rythme explain E003`
```

## Les habitudes d'autres langages

Si vous avez déjà programmé, certains réflexes ne marchent pas en Rythme.
Chacun reçoit un message qui explique quoi écrire à la place.

**Les boucles.** `for` et `while` n'existent pas :

```text
erreur[E001]: `for` n'est pas utilisé en Rythme
 --> main.ry:1:1
  |
1 | for x in [1, 2, 3]
  | ^^^ vient d'un autre langage
  |
  = aide: Rythme n'a pas de boucles. Travaillez sur toute la liste à la place :
        1..5 |> each(fn(i) = print(i))
        [1, 2, 3] |> map(fn(x) = x * 2)
      ou écrivez une fonction récursive. Voir `rythme help functional`.
  = en savoir plus : `rythme explain E001`
```

En suivant l'aide :

```rythme
[1, 2, 3] |> each(fn(x) = print(x))
```

```sortie
1
2
3
```

**`return`.** La dernière expression d'une fonction est son résultat :

```text
erreur[E001]: `return` n'est pas utilisé en Rythme
 --> main.ry:2:3
  |
2 |   return x * 2
  |   ^^^^^^ vient d'un autre langage
  |
  = aide: pas besoin de `return` : la dernière expression d'une fonction est son résultat.
      Pour arrêter plus tôt avec une erreur, renvoyez `Err(...)` et utilisez `?`.
  = en savoir plus : `rythme explain E001`
```

**`null`.** Une valeur absente est `None` (voir [gérer les erreurs](erreurs.md)) :

```text
erreur[E001]: `null` n'est pas utilisé en Rythme
 --> main.ry:1:7
  |
1 | nom = null
  |       ^^^^ vient d'un autre langage
  |
  = aide: Rythme n'a pas de null : utilisez `None` pour « pas de valeur » et `Some(x)` pour une valeur
  = en savoir plus : `rythme explain E001`
```

**`+=`.** Une valeur ne change jamais : on en crée une nouvelle, avec un
nouveau nom.

```text
erreur[E011]: les valeurs ne peuvent pas être modifiées en Rythme
 --> main.ry:2:7
  |
2 | total += 5
  |       ^^
  |
  = aide: les valeurs ne changent jamais en Rythme : donnez un nouveau nom à la nouvelle valeur, comme `total2 = total + 1`
  = en savoir plus : `rythme explain E011`
```

```rythme
total = 0
total_final = total + 5

print(total_final)
```

```sortie
5
```

De même, `&&` reçoit l'aide « utilisez `and` », et `'bonjour'` l'aide
« les textes s'écrivent entre guillemets droits ».

## Le vérificateur devine ce que veulent vos fonctions

Le vérificateur regarde aussi **comment** une fonction utilise ses
paramètres. `cote * cote` demande des nombres : appeler `aire` avec un
texte échouerait à chaque fois, et c'est dit avant l'exécution.

```text
erreur[E005]: impossible d'utiliser `*` avec Text et Text
 --> main.ry:2:7
  |
2 | print(aire("grand"))
  |       ^^^^^^^^^^^^^ `aire` s'arrête sur cette erreur, à chaque fois
  |
1 | fn aire(cote) = cote * cote
  |                      - dans `aire` : Text "grand" * Text "grand"
  |
  = note: d'après la façon dont `aire` utilise `cote`, elle a besoin de : `fn aire(cote: Int|Float) -> Int|Float`
  = en savoir plus : `rythme explain E005`
```

La note se lit ainsi : `cote` doit être un `Int` ou un `Float`, et la
fonction donne un `Int` ou un `Float`.

## Les codes et rythme explain

Chaque erreur et chaque avertissement a un code. `rythme explain CODE`
donne une explication plus longue, avec des exemples justes et faux :

```text
$ rythme explain E005
E005 — Mauvaise sorte de valeur

L'opération ne fonctionne pas avec cette sorte de valeur. Rythme ne convertit
jamais les valeurs en silence : "1" + 1 est une erreur, ni "11" ni 2.

Faux :   "âge : " + 36
Juste :  "âge : {36}"             (insérer la valeur dans le texte)
Juste :  "âge : " + text(36)      (la convertir d'abord)

Les conditions doivent être true ou false :
Faux :   if len(liste) then ... else ...
Juste :  if len(liste) > 0 then ... else ...

Quand la nature des valeurs est certaine, `rythme check` trouve cette erreur
avant l'exécution, même dans du code qui ne s'exécute pas à chaque fois, et
dans les appels qui donnent à une fonction ce qu'elle ne peut pas utiliser :
`aire("grand")` quand `aire(cote)` calcule `cote * cote`.
```

`rythme explain` sans code liste tous les codes, et `rythme help E005` fait
la même chose. Les codes les plus fréquents :

| Code | En une ligne |
|------|--------------|
| E001 | erreur de syntaxe, ou mot d'un autre langage (`for`, `return`, `null`) |
| E002 | un texte jamais fermé : il manque un `"` |
| E003 | nom inconnu : faute de frappe, ou nom utilisé avant sa définition |
| E004 | mauvais nombre d'arguments |
| E005 | mauvaise sorte de valeur (`"a" + 1`, une condition qui n'est pas un Bool) |
| E006 | aucun cas d'un `match` ou d'une fonction ne correspond |
| E007 | indice hors limites (`liste[10]`) |
| E008 | division par zéro |
| E010 | champ inconnu dans un enregistrement |
| E011 | nom déjà défini, ou tentative de modifier une valeur (`x += 1`) |
| E013 | `unwrap` sur `None` ou `Err(...)` |
| E016 | `?` hors d'une fonction, ou sur autre chose qu'une Option ou un Result |
| E019 | `if` sans `else` |
| E020 | il manque un `end` |
| E024 | une ligne calcule une valeur dont rien ne se sert |
| W001 | un nom local jamais utilisé |
| W002 | un cas oublié dans un `match` ou une fonction |
| W003 | une définition cache une fonction intégrée (`len = 3`) |
| W005 | `valeur.nom()` alors qu'aucune fonction ne s'appelle `nom` |
| W008 | le résultat d'une fonction est perdu (`push(liste, x)` seul sur sa ligne) |

## Erreurs fréquentes

### Chercher l'erreur seulement à la ligne indiquée

```rythme norun
print("a"
print("b")
```

```text
erreur[E001]: `,` ou `)` attendu, mais trouvé le nom `print`
 --> main.ry:2:1
  |
2 | print("b")
  | ^^^^^ `,` ou `)` attendu ici
  |
  = aide: séparez les éléments par des virgules
  = en savoir plus : `rythme explain E001`
```

Le message montre la ligne 2, mais l'oubli est à la ligne 1 : la
parenthèse n'y est jamais fermée. Rythme signale l'endroit où il s'est
rendu compte du problème. Quand la ligne indiquée semble juste, regardez
celle d'avant.

### Ignorer un avertissement

```rythme norun
liste = [3, 1, 2]
push(liste, 4)
print(liste)
```

```text
avertissement[W008]: le résultat de `push` est perdu
 --> main.ry:2:1
  |
2 | push(liste, 4)
  | ^^^^^^^^^^^^^^ cette valeur est perdue
  |
  = aide: les valeurs ne changent jamais : `push(list, x)` donne une nouvelle valeur, et ne modifie rien.
      Pour la garder, donnez-lui un nom : `nouveau = push(list, x)`

[3, 1, 2]
```

Le programme tourne, mais affiche `[3, 1, 2]` : le 4 n'est pas là.
L'avertissement l'avait dit. Correction : `plus = push(liste, 4)`, puis
`print(plus)`.

### Écrire une fonction avec une majuscule

```rythme norun
print(Upper("x"))
```

```text
erreur[E003]: nom inconnu `Upper`
 --> main.ry:1:7
  |
1 | print(Upper("x"))
  |       ^^^^^ non défini
  |
  = aide: pour créer votre propre sorte de valeur, définissez un type : `type Nom = Upper(champ) | Autre`
  = en savoir plus : `rythme explain E003`
```

Un nom qui commence par une majuscule est un type ou un constructeur, pas
une fonction. Les fonctions s'écrivent en minuscules : `upper("x")`.

## Testez-vous

**Question 1 :** un programme a deux avertissements et aucune erreur.
S'exécute-t-il ?

> **Réponse**
>
> Oui. Les avertissements sont affichés, puis le programme s'exécute. Seule
> une erreur empêche le démarrage.

**Question 2 :** que veut dire `--> notes.ry:12:5` ?

> **Réponse**
>
> Le problème est dans le fichier `notes.ry`, à la ligne 12, colonne 5.

**Question 3 :** vous voyez `erreur[E016]` et ne comprenez pas le
message. Que faites-vous ?

> **Réponse**
>
> `rythme explain E016`, comme le propose la dernière ligne du message :
> vous obtenez une explication détaillée, avec des exemples.

## En résumé

- Rythme vérifie tout le programme avant de l'exécuter ; `rythme check` ne
  fait que cette vérification.
- Un message dit **pourquoi** (la première ligne), **où** (fichier, ligne,
  colonne, `^`) et **comment corriger** (`= aide:`).
- Une erreur `E…` empêche le démarrage ; un avertissement `W…` non, mais il
  signale souvent un vrai oubli.
- Les fautes de frappe et les mots d'autres langages (`for`, `return`,
  `null`, `+=`) reçoivent une suggestion.
- `rythme explain CODE` explique chaque code, avec des exemples.

Suite : [comprendre un calcul étape par étape](etapes.md).
