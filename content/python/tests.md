# Tester son code

Cette page montre comment écrire des tests automatiques en Python, avec `assert`, le module `unittest` livré avec Python, puis pytest et doctest, pour vérifier que votre code fait bien ce que vous attendez.

> **Objectifs :**
>
> - comprendre à quoi sert un test automatique ;
> - vérifier une valeur avec `assert` ;
> - écrire une classe de test avec `unittest` et lire son résultat ;
> - connaître pytest et doctest ;
> - écrire le test avant le code, en trois étapes.

## Pourquoi tester ?

Un **test** est un petit programme qui vérifie un autre programme (voir
[le glossaire](../debuter/glossaire.md#test)). Il appelle votre code avec des valeurs
connues et compare le résultat à celui que vous attendez.

Sans tests, vous vérifiez à la main, avec des `print`, et vous recommencez après chaque
changement. Avec des tests :

- une seule commande vérifie tout, en une seconde ;
- si un changement casse quelque chose, vous le savez tout de suite ;
- les tests montrent comment utiliser votre code : c'est une documentation qui marche.

> **À retenir :** un test répond à une question simple : « avec cette entrée, est-ce que
> j'obtiens bien cette sortie ? ».

## La vérification la plus simple : assert

Le mot-clé `assert` vérifie qu'une condition est vraie. Si elle l'est, rien ne se passe.
Sinon, Python lève une `AssertionError` et arrête le programme.

```python
def double(x):
    return x * 2


assert double(4) == 8
assert double(0) == 0
assert double(-3) == -6
print("Tous les tests passent.")
```

```sortie
Tous les tests passent.
```

Ajoutez un message après une virgule : il s'affiche quand la vérification échoue. Voici
un `double` qui se trompe (il ajoute 2 au lieu de multiplier) :

```python norun
def double(x):
    return x + 2


assert double(4) == 8, "double(4) devrait valoir 8"
```

```text
Traceback (most recent call last):
  File "/home/alice/main.py", line 5, in <module>
    assert double(4) == 8, "double(4) devrait valoir 8"
           ^^^^^^^^^^^^^^
AssertionError: double(4) devrait valoir 8
```

`assert` suffit pour de petits essais. Mais le programme s'arrête au premier échec, et
vous ne savez pas si les vérifications suivantes passent. Pour aller plus loin, il faut
un vrai outil de test.

## Votre premier test avec unittest

Le module **unittest** est livré avec Python : rien à installer. Un fichier de test suit
toujours le même modèle :

1. `import unittest` charge le module ;
2. une classe de test hérite de `unittest.TestCase` (voir [l'héritage](heritage.md)) ;
3. chaque méthode dont le nom commence par `test_` est un test ;
4. `unittest.main()` lance tous les tests du fichier.

```python
import unittest


def double(x):
    return x * 2


class TestDouble(unittest.TestCase):
    def test_double_de_4(self):
        self.assertEqual(double(4), 8)

    def test_double_de_zero(self):
        self.assertEqual(double(0), 0)


programme = unittest.main(argv=["x"], exit=False)
print(programme.result.wasSuccessful())
```

```sortie
True
```

`self.assertEqual(double(4), 8)` se lit : « vérifie que `double(4)` est égal à `8` ».

> **Note :** dans un vrai fichier de test, la dernière ligne est simplement
> `unittest.main()`. Ici, `argv=["x"], exit=False` empêche `unittest` de lire les
> arguments du terminal et de quitter le programme : on peut ainsi afficher le bilan
> avec `print`.

## Lire le résultat

`unittest` n'écrit pas son rapport sur la sortie normale, mais sur la **sortie
d'erreur** (*stderr*) : c'est pour cela qu'il n'apparaît pas dans la sortie ci-dessus.
Dans votre terminal, vous voyez les deux :

```text
..
----------------------------------------------------------------------
Ran 2 tests in 0.000s

OK
True
```

- Chaque `.` est un test réussi.
- `Ran 2 tests` compte les tests lancés ; le temps change à chaque fois.
- `OK` : tout est vert.

> **Astuce :** `unittest.main(verbosity=2)` affiche le nom de chaque test. Dans le
> terminal, `python3 -m unittest -v` cherche et lance tous les fichiers `test*.py` du
> dossier.

## Lire un échec

Quand une vérification échoue, `unittest` affiche un `F` et explique le problème :

```python norun
import unittest


def double(x):
    return x + 2


class TestDouble(unittest.TestCase):
    def test_double_de_4(self):
        self.assertEqual(double(4), 8)


unittest.main()
```

```text
F
======================================================================
FAIL: test_double_de_4 (__main__.TestDouble.test_double_de_4)
----------------------------------------------------------------------
Traceback (most recent call last):
  File "/home/alice/main.py", line 10, in test_double_de_4
    self.assertEqual(double(4), 8)
    ~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^
AssertionError: 6 != 8

----------------------------------------------------------------------
Ran 1 test in 0.000s

FAILED (failures=1)
```

Le message donne tout ce qu'il faut : le test (`test_double_de_4`), la ligne (`10`) et
la comparaison ratée (`6 != 8` : votre code a donné `6`, on attendait `8`).

- `F` (*failure*, échec) : une vérification a donné un mauvais résultat.
- `E` (*error*, erreur) : le code a planté pendant le test (un nom inconnu, par exemple).

## Les assertions

Une **assertion** est une vérification. `unittest` en propose beaucoup ; voici les plus
utiles.

| Assertion | Vérifie que… |
| --- | --- |
| `assertEqual(a, b)` | `a == b` |
| `assertNotEqual(a, b)` | `a != b` |
| `assertTrue(x)` / `assertFalse(x)` | `x` est vrai / faux |
| `assertIsNone(x)` | `x is None` |
| `assertIn(x, liste)` | `x` est dans la liste |
| `assertAlmostEqual(a, b)` | deux flottants sont très proches |
| `assertRaises(Erreur)` | le code lève bien cette erreur |

```python
import unittest


class TestAssertions(unittest.TestCase):
    def test_egalite(self):
        self.assertEqual("bonjour".upper(), "BONJOUR")
        self.assertEqual(sorted([3, 1, 2]), [1, 2, 3])

    def test_vrai_faux(self):
        self.assertTrue(2 in [1, 2, 3])
        self.assertFalse([])

    def test_none_et_in(self):
        self.assertIsNone({"a": 1}.get("b"))
        self.assertIn("poire", ["pomme", "poire"])

    def test_flottants(self):
        self.assertAlmostEqual(0.1 + 0.2, 0.3)


programme = unittest.main(argv=["x"], exit=False)
print(programme.result.testsRun, "tests lancés")
print("Réussi :", programme.result.wasSuccessful())
```

```sortie
4 tests lancés
Réussi : True
```

> **Note :** `0.1 + 0.2` vaut `0.30000000000000004` en Python (voir
> [les nombres](nombres.md)). Pour comparer des flottants, préférez `assertAlmostEqual`
> à `assertEqual`.

### Tester qu'une erreur est levée

Votre code doit parfois refuser une valeur avec `raise` (voir
[les erreurs](erreurs.md)). Écrivez `with self.assertRaises(...)` : le bloc en dessous
doit lever cette erreur. Le `as` récupère l'erreur pour vérifier son message.

```python
import unittest


def retirer(solde, montant):
    if montant < 0:
        raise ValueError("montant négatif")
    return solde - montant


class TestRetirer(unittest.TestCase):
    def test_retrait_normal(self):
        self.assertEqual(retirer(100, 30), 70)

    def test_retrait_negatif_refuse(self):
        with self.assertRaises(ValueError) as contexte:
            retirer(100, -5)
        self.assertEqual(str(contexte.exception), "montant négatif")


programme = unittest.main(argv=["x"], exit=False)
print(programme.result.wasSuccessful())
```

```sortie
True
```

## Tester une classe avec setUp

Pour une classe, créez un objet dans le test, appelez ses méthodes, puis vérifiez son
état. La méthode `setUp` s'exécute **avant chaque test** : elle prépare un objet neuf.

```python
import unittest


class Compte:
    def __init__(self):
        self.solde = 0

    def deposer(self, montant):
        self.solde += montant


class TestCompte(unittest.TestCase):
    def setUp(self):
        self.compte = Compte()

    def test_solde_de_depart(self):
        self.assertEqual(self.compte.solde, 0)

    def test_deposer_augmente_le_solde(self):
        self.compte.deposer(50)
        self.compte.deposer(20)
        self.assertEqual(self.compte.solde, 70)


programme = unittest.main(argv=["x"], exit=False)
print(programme.result.wasSuccessful())
```

```sortie
True
```

Chaque test reçoit son propre `self.compte` : le dépôt du second test ne change pas le
premier.

## pytest : des tests plus courts

Beaucoup de projets Python utilisent **pytest**. Il ne fait pas partie de Python : il
s'installe avec pip (voir [organiser un projet](projet.md)).

```sh
python3 -m pip install pytest
```

Avec pytest, un test est une simple fonction `test_…`, et la vérification est un
`assert` ordinaire. Pas de classe, pas de `self.assertEqual`.

```python norun
# test_calcul.py
import pytest


def double(x):
    return x * 2


def test_double_de_4():
    assert double(4) == 8


def test_double_de_zero():
    assert double(0) == 0


def test_division_par_zero():
    with pytest.raises(ZeroDivisionError):
        1 / 0
```

La commande `pytest` cherche tous les fichiers `test_*.py` du dossier et les lance :

```sh
pytest
```

```text
============================= test session starts ==============================
platform linux -- Python 3.13.16, pytest-9.1.1, pluggy-1.7.0
rootdir: /home/alice/mon_projet
collected 3 items

test_calcul.py ...                                                       [100%]

============================== 3 passed in 0.03s ===============================
```

Quand un test échoue, pytest montre la ligne et les valeurs. Avec un `double` qui ajoute
2 :

```text
=================================== FAILURES ===================================
_______________________________ test_double_de_4 _______________________________

    def test_double_de_4():
>       assert double(4) == 8
E       assert 6 == 8
E        +  where 6 = double(4)

test_calcul.py:10: AssertionError
=========================== short test summary info ============================
FAILED test_calcul.py::test_double_de_4 - assert 6 == 8
FAILED test_calcul.py::test_double_de_zero - assert 2 == 0
========================= 2 failed, 1 passed in 0.04s ==========================
```

> **Astuce :** pytest lance aussi les tests écrits avec `unittest`. Vous pouvez
> commencer avec `unittest`, puis passer à pytest sans rien réécrire.

## doctest : des exemples qui se vérifient

Une **docstring** est le texte entre triples guillemets sous une fonction (voir
[bien écrire du Python](style.md)). Si vous y écrivez des exemples comme dans la console
Python (`>>>` puis le résultat attendu), le module `doctest` les vérifie.

```python
def moyenne(notes):
    """Renvoie la moyenne d'une liste de notes.

    >>> moyenne([10, 20])
    15.0
    >>> moyenne([12, 13, 14])
    13.0
    """
    return sum(notes) / len(notes)


import doctest

print(doctest.testmod())
```

```sortie
TestResults(failed=0, attempted=2)
```

`attempted=2` : deux exemples vérifiés ; `failed=0` : aucun échec. L'exemple sert à la
fois de documentation et de test.

## Quoi tester ?

Vous ne pouvez pas tout tester. Visez les cas qui comptent :

| Cas | Exemple pour `moyenne(notes)` |
| --- | --- |
| Le cas normal | `[10, 20]` donne `15.0` |
| Un seul élément | `[12]` donne `12.0` |
| La frontière | une note à `0`, une note à `20` |
| Le cas vide | `[]` : erreur ou valeur par défaut ? |
| Les mauvaises entrées | une note négative doit être refusée |

Quelques habitudes :

- un test vérifie **une seule idée** ;
- un bon nom de test dit ce qui est vérifié : `test_retrait_negatif_refuse` plutôt que
  `test_2` ;
- quand vous corrigez un bug, ajoutez d'abord un test qui le montre : il ne reviendra
  pas.

## Écrire le test d'abord

Une méthode de travail très répandue consiste à écrire le test **avant** le code. On
avance en trois étapes, qui se répètent : **rouge**, **vert**, **améliorer**.

Exemple : une fonction `mention(note)` qui renvoie `"Bien"` à partir de 14,
`"Passable"` à partir de 10, et `"Insuffisant"` en dessous.

### Étape 1 : rouge, le test échoue

Écrivez d'abord le test, comme si la fonction existait déjà.

```python norun
import unittest


class TestMention(unittest.TestCase):
    def test_mention_bien(self):
        self.assertEqual(mention(15), "Bien")


unittest.main()
```

Le test échoue, puisque `mention` n'existe pas encore :

```text
E
======================================================================
ERROR: test_mention_bien (__main__.TestMention.test_mention_bien)
----------------------------------------------------------------------
Traceback (most recent call last):
  File "/home/alice/main.py", line 6, in test_mention_bien
    self.assertEqual(mention(15), "Bien")
                     ^^^^^^^
NameError: name 'mention' is not defined
```

C'est normal, et c'est utile : vous savez que le test vérifie vraiment quelque chose.

### Étape 2 : vert, le test passe

Écrivez le code le plus simple qui fait passer les tests. Ajoutez un test à la fois
pour chaque cas, sans oublier la frontière (`14`).

```python
import unittest


def mention(note):
    if note >= 14:
        return "Bien"
    if note >= 10:
        return "Passable"
    return "Insuffisant"


class TestMention(unittest.TestCase):
    def test_mention_bien(self):
        self.assertEqual(mention(15), "Bien")
        self.assertEqual(mention(14), "Bien")

    def test_mention_passable(self):
        self.assertEqual(mention(10), "Passable")

    def test_mention_insuffisant(self):
        self.assertEqual(mention(7), "Insuffisant")


programme = unittest.main(argv=["x"], exit=False)
print(programme.result.wasSuccessful())
```

```sortie
True
```

### Étape 3 : améliorer le code

Maintenant que les tests vous protègent, rendez le code plus clair ou plus souple. Ici,
les seuils deviennent une liste : en ajouter un ne demande plus de nouveau `if`.
Relancez les tests : s'ils restent verts, rien n'est cassé.

```python
import unittest

SEUILS = [(14, "Bien"), (10, "Passable")]


def mention(note):
    for seuil, nom in SEUILS:
        if note >= seuil:
            return nom
    return "Insuffisant"


class TestMention(unittest.TestCase):
    def test_mention_bien(self):
        self.assertEqual(mention(15), "Bien")
        self.assertEqual(mention(14), "Bien")

    def test_mention_passable(self):
        self.assertEqual(mention(10), "Passable")

    def test_mention_insuffisant(self):
        self.assertEqual(mention(7), "Insuffisant")


programme = unittest.main(argv=["x"], exit=False)
print(programme.result.wasSuccessful())
```

```sortie
True
```

> **À retenir :** rouge (un test qui échoue), vert (le code minimal qui le fait
> passer), améliorer (sans changer le comportement). Puis on recommence avec le cas
> suivant.

## Erreurs fréquentes

### Un nom de méthode sans test_

`unittest` ne lance que les méthodes dont le nom commence par `test_`. Les autres sont
ignorées, sans message d'erreur.

```python norun
import unittest


class TestDouble(unittest.TestCase):
    def double_de_quatre(self):
        self.assertEqual(4 * 2, 8)


unittest.main()
```

```text
----------------------------------------------------------------------
Ran 0 tests in 0.000s

NO TESTS RAN
```

`Ran 0 tests` doit vous alerter. Renommez la méthode `test_double_de_quatre`.

### Des parenthèses autour de assert

`assert` n'est pas une fonction. Avec des parenthèses, la condition et le message
forment un **tuple**, qui est toujours vrai : la vérification ne sert plus à rien.

```python norun
def double(x):
    return x + 2


assert(double(4) == 8, "double(4) devrait valoir 8")
print("Fin du programme")
```

```text
/home/alice/main.py:5: SyntaxWarning: assertion is always true, perhaps remove parentheses?
  assert(double(4) == 8, "double(4) devrait valoir 8")
Fin du programme
```

Le bug passe inaperçu. Écrivez `assert double(4) == 8, "double(4) devrait valoir 8"`.

### Calculer la valeur au lieu de la passer à assertRaises

Le code qui doit lever l'erreur va **dans un bloc `with`**. Sinon, il s'exécute avant
`assertRaises`, et le test plante.

```python norun
import unittest


class TestDivision(unittest.TestCase):
    def test_division_par_zero(self):
        self.assertRaises(ZeroDivisionError, 10 / 0)


unittest.main()
```

```text
ERROR: test_division_par_zero (__main__.TestDivision.test_division_par_zero)
----------------------------------------------------------------------
Traceback (most recent call last):
  File "/home/alice/main.py", line 6, in test_division_par_zero
    self.assertRaises(ZeroDivisionError, 10 / 0)
                                         ~~~^~~
ZeroDivisionError: division by zero
```

Écrivez `with self.assertRaises(ZeroDivisionError):` puis `10 / 0` en dessous.

### Comparer des flottants avec assertEqual

```python norun
self.assertEqual(0.1 + 0.2, 0.3)
```

```text
AssertionError: 0.30000000000000004 != 0.3
```

Écrivez `self.assertAlmostEqual(0.1 + 0.2, 0.3)`.

## Testez-vous

**1.** Que signifient `.`, `F` et `E` dans le résultat de `unittest` ?

> **Réponse**
>
> `.` : un test réussi. `F` : un échec, une assertion a donné un mauvais résultat.
> `E` : une erreur, le code a planté pendant le test.

**2.** Le résumé affiche `Ran 0 tests`. Quelle est la cause la plus probable ?

> **Réponse**
>
> Les méthodes de test ne commencent pas par `test_` : `unittest` ne les voit pas.

**3.** Quelle est la différence principale entre un test `unittest` et un test pytest ?

> **Réponse**
>
> Avec `unittest`, on écrit une classe qui hérite de `TestCase` et on vérifie avec
> `self.assertEqual`. Avec pytest, une simple fonction `test_…` et un `assert` suffisent ;
> mais pytest doit être installé avec pip.

## En résumé

- Un test vérifie automatiquement qu'une entrée donne la sortie attendue.
- `assert condition, "message"` est la vérification la plus simple, sans parenthèses.
- Avec `unittest` : une classe qui hérite de `unittest.TestCase`, des méthodes `test_…`,
  `self.assertEqual` et `with self.assertRaises(...)`.
- pytest (à installer) raccourcit les tests ; doctest vérifie les exemples des
  docstrings.
- Rouge, vert, améliorer : écrivez le test, puis le code, puis rendez-le plus clair.

Suite : [Déboguer en Python](debogage.md)
