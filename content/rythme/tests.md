# Tester son code

Un test est un petit morceau de code qui vérifie qu'une fonction donne le bon résultat : en Rythme, on l'écrit à côté de la fonction, et `rythme test` les lance tous d'un coup.

> **Objectifs :**
>
> - écrire un test avec `test "nom" = ...` et `assert_eq` ;
> - écrire un test de plusieurs lignes, avec `assert` ;
> - lancer les tests avec `rythme test`, et lire le message d'un test qui échoue ;
> - tester une fonction qui renvoie un `Result`.

## Pourquoi tester

Vous écrivez une fonction, vous l'essayez avec un `print`, elle marche.
Une semaine plus tard, vous la modifiez... et vous cassez sans le voir un cas
qui marchait.

Un **test** garde la trace de ce qui doit marcher : « le double de 21 est
42 ». Au lieu de tout réessayer à la main, vous lancez une commande, et
Rythme vérifie chaque test pour vous, en une fraction de seconde.

## Un premier test

Un test commence par le mot `test`, puis son **nom** entre guillemets, puis
`=` et la vérification :

```rythme
fn double(x) = x * 2

test "le double de 21 est 42" = assert_eq(double(21), 42)

print(double(5))
```

```sortie
10
```

`assert_eq(obtenu, attendu)` vérifie que deux valeurs sont égales. Ici, le
test dit : « `double(21)` doit donner `42` ».

Lancé avec `rythme run`, comme d'habitude, le programme **ignore les
tests** : seul le `print` s'exécute. Les tests peuvent donc vivre dans le
même fichier que le code, sans rien changer à ce qu'il fait.

> **À retenir :** le nom d'un test est une phrase qui dit ce qu'on vérifie.
> Quand un test échoue, c'est ce nom qui s'affiche : il doit être clair.

## Plusieurs vérifications dans un bloc

Un test peut s'écrire sur plusieurs lignes, sans `=`, et se termine par
`end`. Il peut alors contenir plusieurs vérifications :

```rythme
fn double(x) = x * 2

test "le double garde le signe"
  assert(double(-3) < 0, "le double de -3 doit être négatif")
  assert_eq(double(0), 0)
  assert_eq(double(4), 8)
end

print(double(-3))
```

```sortie
-6
```

`assert(condition, message)` vérifie qu'une condition est vraie. Si elle est
fausse, le message s'affiche.

| Fonction | Vérifie que |
|----------|-------------|
| `assert_eq(obtenu, attendu)` | les deux valeurs sont égales |
| `assert(condition, message)` | la condition est vraie |

> **Astuce :** préférez `assert_eq` quand vous comparez deux valeurs : en cas
> d'échec, il montre ce qu'il a obtenu et ce qu'il attendait.

## Lancer les tests avec rythme test

Voici un fichier `calculs.ry`, avec deux fonctions et trois tests :

```rythme
fn double(x) = x * 2

fn moyenne(notes) = sum(notes) / len(notes)

test "le double de 21 est 42" = assert_eq(double(21), 42)

test "le double garde le signe"
  assert(double(-3) < 0, "le double de -3 doit être négatif")
  assert_eq(double(0), 0)
end

test "la moyenne de 10, 12 et 14" = assert_eq(moyenne([10, 12, 14]), 12.0)

print(double(5))
```

```sortie
10
```

Pour lancer ses tests, utilisez `rythme test` au lieu de `rythme run` :

```sh
rythme test calculs.ry
```

```text
Exécution de 3 test(s)
  ✓ le double de 21 est 42
  ✓ le double garde le signe
  ✓ la moyenne de 10, 12 et 14
3 test(s) : 3 réussi(s), 0 échoué(s) (1 ms)
```

Chaque `✓` est un test réussi. La dernière ligne fait le bilan, avec le
temps pris entre parenthèses (il change un peu d'une fois à l'autre).

Pendant `rythme test`, les lignes du premier niveau comme `print(double(5))`
sont **sautées** : seuls les tests s'exécutent.

## Quand un test échoue

Ajoutons à `calculs.ry` un test faux, exprès :

```rythme norun
test "une erreur volontaire" = assert_eq(double(3), 7)
```

```text
Exécution de 4 test(s)
  ✓ le double de 21 est 42
  ✓ le double garde le signe
  ✓ la moyenne de 10, 12 et 14
  ✗ une erreur volontaire
erreur[E015]: assertion échouée : les valeurs sont différentes
  --> calculs.ry:16:32
   |
16 | test "une erreur volontaire" = assert_eq(double(3), 7)
   |                                ^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: obtenu :  6
       attendu : 7
   = en savoir plus : `rythme explain E015`

4 test(s) : 3 réussi(s), 1 échoué(s) (0 ms)
```

Lisez-le de haut en bas :

1. `✗` marque le test qui a échoué, avec son nom ;
2. la ligne et la colonne du problème : `calculs.ry:16:32` ;
3. ce que la fonction a **obtenu** (`6`) et ce qui était **attendu** (`7`) ;
4. le bilan : 3 réussis, 1 échoué.

Ici, c'est le test qui se trompe : le double de 3 est bien 6. Parfois, c'est
la fonction : le test vous a alors évité un bug.

Avec `assert`, le message que vous avez écrit s'affiche. Ce test-ci se
trompe lui aussi : il aurait dû vérifier `not majeur?(17)`.

```rythme norun
fn majeur?(age) = age >= 18

test "18 ans, c'est majeur" = assert(majeur?(18), "18 ans devrait être majeur")
test "17 ans, c'est mineur" = assert(majeur?(17), "17 ans devrait être mineur")
```

```text
Exécution de 2 test(s)
  ✓ 18 ans, c'est majeur
  ✗ 17 ans, c'est mineur
erreur[E015]: assertion échouée : 17 ans devrait être mineur
 --> age.ry:4:31
  |
4 | test "17 ans, c'est mineur" = assert(majeur?(17), "17 ans devrait être mineur")
  |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cette condition est fausse
  |
  = en savoir plus : `rythme explain E015`

2 test(s) : 1 réussi(s), 1 échoué(s) (0 ms)
```

> **Note :** quand tous les tests réussissent, `rythme test` se termine avec
> le code 0 ; sinon avec le code 1. Les outils d'intégration continue (qui
> lancent les tests à chaque modification) s'en servent.

## Choisir des tests avec --filter

Quand un fichier a beaucoup de tests, `--filter` lance seulement ceux dont le
nom contient un mot (sans tenir compte des majuscules) :

```sh
rythme test calculs.ry --filter double
```

```text
Exécution de 2 test(s) sur 4
  ✓ le double de 21 est 42
  ✓ le double garde le signe
2 test(s) : 2 réussi(s), 0 échoué(s), 2 écarté(s) par le filtre (0 ms)
```

Encore une raison de donner aux tests des noms clairs.

## Relancer à chaque modification avec --watch

```sh
rythme test calculs.ry --watch
```

`--watch` lance les tests, puis **attend**. Chaque fois que vous
enregistrez le fichier, les tests sont relancés tout seuls :

```text
Surveillance de 1 fichier(s) : les tests sont relancés quand l'un d'eux change (Ctrl+C pour arrêter).
```

Gardez le terminal ouvert à côté de votre éditeur : vous voyez le résultat
à chaque enregistrement. `Ctrl+C` arrête la surveillance.

## Tester les erreurs et les Results

Une fonction qui peut échouer renvoie un `Result` (voir
[gérer les erreurs](erreurs.md)). On teste les deux cas avec `assert_eq`, en
écrivant le `Ok(...)` ou le `Err(...)` attendu :

```rythme
fn diviser(a, b) = if b == 0 then Err("division par zéro") else Ok(a / b)

test "diviser 10 par 4" = assert_eq(diviser(10, 4), Ok(2.5))

test "diviser par zéro donne une erreur" = assert_eq(diviser(1, 0), Err("division par zéro"))

test "deux divisions à la suite"
  moitie = diviser(100, 2)?
  dixieme = diviser(moitie, 5)?
  assert_eq(dixieme, 10.0)
end

print(diviser(10, 4), diviser(1, 0))
```

```sortie
Ok(2.5) Err("division par zéro")
```

Le troisième test utilise `?`, comme dans une fonction : si une étape donne
`Err(...)`, le test s'arrête et **échoue**, en montrant l'erreur. Par
exemple, avec ce test, à la ligne 13 du fichier `banque.ry` :

```rythme norun
test "une division ratée"
  x = diviser(100, 0)?
  assert_eq(x, 1)
end
```

```text
  ✗ une division ratée
erreur[E015]: le test s'est terminé avec Err("division par zéro")
  --> banque.ry:13:1
   |
13 | test "une division ratée"
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = aide: un test réussit s'il se termine sans erreur, et que sa valeur n'est ni `false`, ni `Err(...)`, ni `None`
   = en savoir plus : `rythme explain E015`
```

La règle est simple : un test **réussit** s'il se termine sans erreur, et que
son résultat n'est ni `false`, ni `Err(...)`, ni `None`.

## Où ranger ses tests

**À côté du code.** C'est le plus simple : les tests d'une fonction sont
juste sous elle, dans le même fichier. Quand vous modifiez la fonction, ses
tests sont sous vos yeux.

**Dans un fichier à part.** Pour un module (voir
[les modules](modules.md)), on peut aussi écrire les tests dans un autre
fichier, qui l'utilise avec `use`. Voici `geometrie_test.ry`, à côté de
`geometrie.ry` :

```rythme norun
use geometrie

test "l'aire d'un rectangle de 2 sur 3" = assert_eq(geometrie.aire_rectangle(2, 3), 6)

test "l'aire d'un carré de côté 4" = assert_eq(geometrie.aire_carre(4), 16)
```

**Tout un dossier d'un coup.** `rythme test .` lance les tests de tous les
fichiers `.ry` du dossier (et de ses sous-dossiers) :

```sh
rythme test .
```

```text
Exécution de 3 test(s) de ./calculs.ry
  ✓ le double de 21 est 42
  ✓ le double garde le signe
  ✓ la moyenne de 10, 12 et 14
Exécution de 2 test(s) de ./geometrie_test.ry
  ✓ l'aire d'un rectangle de 2 sur 3
  ✓ l'aire d'un carré de côté 4
2 fichier(s), 5 test(s) : 5 réussi(s), 0 échoué(s) (1 ms)
```

Dans un projet créé avec `rythme new`, les tests vont dans le dossier
`tests/`, et `rythme test` tout court les lance (voir
[organiser un projet](projet.md)).

## Erreurs fréquentes

### Oublier les guillemets autour du nom

```rythme norun
fn double(x) = x * 2

test double = assert_eq(double(21), 42)
```

```text
erreur[E001]: le nom du test, sous forme de texte attendu, mais trouvé le nom `double`
 --> calculs.ry:3:6
  |
3 | test double = assert_eq(double(21), 42)
  |      ^^^^^^ le nom du test, sous forme de texte attendu ici
  |
  = aide: exemple : `test "l'addition marche" = assert_eq(1 + 1, 2)`
  = en savoir plus : `rythme explain E001`
```

Correction : le nom est un texte, `test "le double de 21" = ...`.

### Écrire un test dans une fonction

```rythme norun
fn double(x)
  test "le double" = assert_eq(x * 2, 42)
  x * 2
end
```

```text
erreur[E001]: `test` ne peut être utilisé qu'au premier niveau d'un fichier
 --> calculs.ry:2:3
  |
2 |   test "le double" = assert_eq(x * 2, 42)
  |   ^^^^
  |
  = aide: déplacez-le en dehors de la fonction ou du bloc
  = en savoir plus : `rythme explain E001`
```

Correction : écrivez le test **après** la fonction, au premier niveau, sans
retrait : `test "le double de 21" = assert_eq(double(21), 42)`.

### Comparer avec == au lieu d'assert_eq

Un test qui finit par `false` échoue, mais sans dire pourquoi :

```rythme norun
fn double(x) = x * 2

test "le double de 3" = double(3) == 7
```

```text
Exécution de 1 test(s)
  ✗ le double de 3
erreur[E015]: le test a renvoyé `false`
 --> calculs.ry:3:1
  |
3 | test "le double de 3" = double(3) == 7
  | ^^^^^^^^^^^^^^^^^^^^^
  |
  = aide: utilisez `assert_eq(obtenu, attendu)` pour voir les valeurs
  = en savoir plus : `rythme explain E015`

1 test(s) : 0 réussi(s), 1 échoué(s) (0 ms)
```

Correction : `test "le double de 3" = assert_eq(double(3), 6)`. En cas
d'échec, `assert_eq` montre la valeur obtenue et la valeur attendue.

## Testez-vous

1. Que fait `rythme run` d'un fichier qui contient des tests ?

> **Réponse**
>
> Il ignore les tests et lance le programme normalement. Seul
> `rythme test` exécute les tests (et saute alors les autres lignes du
> premier niveau, comme les `print`).

2. Écrivez un test qui vérifie que `upper("rythme")` donne `"RYTHME"`.

> **Réponse**
>
> ```rythme
> test "upper met en majuscules" = assert_eq(upper("rythme"), "RYTHME")
> ```

3. Comment lancer seulement les tests dont le nom contient « moyenne » ?

> **Réponse**
>
> `rythme test calculs.ry --filter moyenne`.

## En résumé

- `test "nom" = assert_eq(obtenu, attendu)` écrit un test sur une ligne ;
  `test "nom"` ... `end` en écrit un de plusieurs lignes.
- `assert_eq` compare deux valeurs, `assert(condition, message)` vérifie une
  condition.
- `rythme test fichier.ry` lance les tests ; `rythme run` les ignore.
- Un test qui échoue montre son nom, la ligne, et ce qui était obtenu et
  attendu.
- `--filter mot` choisit des tests, `--watch` les relance à chaque
  modification, `rythme test .` lance tout le dossier.
- Un test échoue s'il s'arrête sur une erreur ou finit par `false`,
  `Err(...)` ou `None` : `?` y est permis.

Suite : [Bien écrire du Rythme](style.md)
