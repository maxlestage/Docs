# Faire plusieurs choses à la fois

Cette page montre comment lancer plusieurs tâches en même temps avec les threads et les processus, et comment éviter qu'elles se marchent sur les pieds.

> **Objectifs :**
>
> - vous saurez lancer un thread et attendre sa fin avec `join` ;
> - vous comprendrez ce qu'est le GIL et ce qu'il change pour vos programmes ;
> - vous saurez utiliser `ThreadPoolExecutor` et `ProcessPoolExecutor` ;
> - vous saurez protéger une donnée partagée avec un `Lock` ;
> - vous saurez faire communiquer des threads avec une `queue.Queue`.

## Pourquoi faire plusieurs choses à la fois ?

Imaginez une cuisine. Un seul cuisinier qui attend que l'eau bouille sans rien faire
d'autre perd du temps. Pendant que l'eau chauffe, il pourrait couper les légumes.

Un programme attend souvent : une réponse d'un site web, la lecture d'un disque, un
délai. Faire avancer d'autres tâches pendant ces attentes, c'est la
[concurrence](../debuter/glossaire.md#concurrence).

Python propose trois outils, que cette page et la suivante présentent :

- les **threads** : plusieurs cuisiniers dans la même cuisine ;
- les **processus** : plusieurs cuisines séparées ;
- l'**asynchrone** (chapitre suivant) : un seul cuisinier très bien organisé.

## Lancer un thread

Un [thread](../debuter/glossaire.md#thread) (« fil d'exécution ») est une suite
d'instructions qui avance en parallèle du reste du programme. On le crée avec
`threading.Thread`, en lui donnant la fonction à exécuter (`target`) et ses arguments
(`args`).

- `start()` lance le thread ;
- `join()` attend qu'il ait fini.

```python
import threading
import time

resultats = {}

def preparer(plat, minutes):
    time.sleep(minutes / 10)
    resultats[plat] = f"{plat} prêt"

t1 = threading.Thread(target=preparer, args=("soupe", 2))
t2 = threading.Thread(target=preparer, args=("gâteau", 1))
t1.start()
t2.start()

t1.join()
t2.join()

for plat in sorted(resultats):
    print(resultats[plat])
```

```sortie
gâteau prêt
soupe prêt
```

Les deux plats cuisent en même temps. Sans `join`, le programme pourrait afficher les
résultats **avant** que les threads aient fini.

> **Attention :** `args` attend un tuple. Pour un seul argument, n'oubliez pas la
> virgule : `args=("soupe",)`.

### Gagner du temps pendant les attentes

Comparons trois « téléchargements » simulés d'une seconde chacun, l'un après l'autre
puis en même temps.

```python
import threading
import time

def telecharger(nom):
    time.sleep(1)

debut = time.perf_counter()
for nom in ["a", "b", "c"]:
    telecharger(nom)
print("L'un après l'autre :", round(time.perf_counter() - debut), "s")

debut = time.perf_counter()
threads = [threading.Thread(target=telecharger, args=(nom,)) for nom in ["a", "b", "c"]]
for t in threads:
    t.start()
for t in threads:
    t.join()
print("En même temps :", round(time.perf_counter() - debut), "s")
```

```sortie
L'un après l'autre : 3 s
En même temps : 1 s
```

`time.perf_counter()` mesure le temps écoulé. Les trois attentes se chevauchent : on
passe de 3 secondes à 1.

## Le GIL, expliqué simplement

Python (la version habituelle, CPython) a un verrou interne appelé **GIL** (*Global
Interpreter Lock*). Il ne laisse **qu'un seul thread à la fois** exécuter du code
Python.

Revenons à la cuisine : il y a plusieurs cuisiniers, mais un seul couteau. Quand un
cuisinier attend (l'eau qui bout, le four), il pose le couteau et un autre le prend.
Mais si tous veulent couper en même temps, ils font la queue.

Conséquences :

- pour des tâches qui **attendent** (réseau, disque, `sleep`), les threads font gagner
  beaucoup de temps : le GIL est relâché pendant l'attente ;
- pour des tâches qui **calculent** (boucles, maths), les threads ne vont pas plus vite :
  un seul calcule à la fois. Il faut alors des **processus**.

> **Note :** Python 3.13 propose une version expérimentale sans GIL (*free-threaded*),
> qu'il faut installer à part. La version habituelle garde le GIL.

## concurrent.futures : des équipes toutes prêtes

Créer et attendre chaque thread à la main devient vite lourd. Le module
`concurrent.futures` fournit des **pools** : des équipes de travailleurs qui se
partagent une liste de tâches.

### ThreadPoolExecutor

`map(fonction, valeurs)` applique la fonction à chaque valeur, en parallèle. Les
résultats reviennent **dans l'ordre des valeurs**, même si certaines tâches finissent
avant d'autres.

```python
import time
from concurrent.futures import ThreadPoolExecutor

def taille_page(adresse):
    time.sleep(0.1 * len(adresse) % 3)
    return len(adresse) * 100

adresses = ["exemple.fr", "python.org", "wikipedia.org", "a.io"]

with ThreadPoolExecutor(max_workers=4) as equipe:
    tailles = equipe.map(taille_page, adresses)
    for adresse, taille in zip(adresses, tailles):
        print(f"{adresse:15} {taille} octets")
```

```sortie
exemple.fr      1000 octets
python.org      1000 octets
wikipedia.org   1300 octets
a.io            400 octets
```

Le bloc `with` attend que toutes les tâches soient finies avant de continuer.

### submit et les futurs

`submit` lance **une** tâche et renvoie un **futur** : un ticket de vestiaire qui
donnera le résultat plus tard. `result()` attend ce résultat.

```python
from concurrent.futures import ThreadPoolExecutor

def carre(x):
    return x * x

with ThreadPoolExecutor() as equipe:
    futurs = [equipe.submit(carre, n) for n in range(1, 6)]
    print([f.result() for f in futurs])
```

```sortie
[1, 4, 9, 16, 25]
```

### ProcessPoolExecutor

Pour des **calculs**, utilisez `ProcessPoolExecutor` : chaque travailleur est un
[processus](../debuter/glossaire.md#processus) séparé, avec son propre Python et son
propre GIL. Les calculs avancent vraiment en même temps, sur plusieurs cœurs du
[processeur](../debuter/glossaire.md#processeur).

```python
from concurrent.futures import ProcessPoolExecutor

def nombre_de_premiers(limite):
    compte = 0
    for n in range(2, limite):
        if all(n % d for d in range(2, int(n ** 0.5) + 1)):
            compte += 1
    return compte

if __name__ == "__main__":
    limites = [1000, 10000, 50000]
    with ProcessPoolExecutor() as equipe:
        for limite, compte in zip(limites, equipe.map(nombre_de_premiers, limites)):
            print(f"{compte} nombres premiers sous {limite}")
```

```sortie
168 nombres premiers sous 1000
1229 nombres premiers sous 10000
5133 nombres premiers sous 50000
```

> **À retenir :** avec des processus, mettez le code principal sous
> `if __name__ == "__main__":`. Sous Windows et macOS, chaque processus relit votre
> fichier : sans cette ligne, il relancerait des processus sans fin.

Les processus coûtent plus cher à démarrer que les threads, et les données doivent
être copiées de l'un à l'autre. Réservez-les aux calculs lourds.

## Partager une donnée : le Lock

Quand plusieurs threads modifient la **même** variable, ils peuvent se gêner.
`compteur += 1` se fait en trois temps : lire la valeur, ajouter 1, écrire. Si deux
threads lisent la même valeur avant d'écrire, un ajout est perdu. On appelle cela une
**situation de concurrence** (*race condition*). Le résultat n'est alors plus garanti.

Un `Lock` (verrou) règle le problème : c'est la clé des toilettes d'un café. Un seul
thread à la fois peut entrer dans la partie protégée ; les autres attendent la clé.

```python
import threading

compteur = 0
verrou = threading.Lock()

def compter():
    global compteur
    for _ in range(100_000):
        with verrou:
            compteur += 1

threads = [threading.Thread(target=compter) for _ in range(4)]
for t in threads:
    t.start()
for t in threads:
    t.join()

print(compteur)
```

```sortie
400000
```

`with verrou:` prend la clé au début du bloc et la rend à la fin, même en cas
d'erreur.

> **Astuce :** le plus simple reste de ne **pas** partager. Faites renvoyer un résultat
> à chaque tâche (avec `map`), puis combinez les résultats à la fin.

## Faire communiquer des threads : queue.Queue

Une `queue.Queue` (« file d'attente ») est un tapis roulant entre threads. Un thread
**producteur** pose des objets avec `put` ; un thread **consommateur** les prend avec
`get`, dans l'ordre d'arrivée. La file gère elle-même les verrous.

```python
import queue
import threading

tapis = queue.Queue()

def cuisinier():
    while True:
        commande = tapis.get()
        if commande is None:
            break
        print("Je prépare :", commande)

serveur = threading.Thread(target=cuisinier)
serveur.start()

for plat in ["pizza", "salade", "tarte"]:
    tapis.put(plat)
tapis.put(None)

serveur.join()
print("Service terminé.")
```

```sortie
Je prépare : pizza
Je prépare : salade
Je prépare : tarte
Service terminé.
```

`get` attend tant que la file est vide. La valeur `None` sert de signal de fin : le
cuisinier sort de sa boucle.

## Que choisir ?

| Votre programme… | Outil | Pourquoi |
|------------------|-------|----------|
| attend beaucoup (réseau, fichiers), avec quelques dizaines de tâches | `ThreadPoolExecutor` | simple, le GIL est relâché pendant l'attente |
| calcule beaucoup (images, maths, gros traitements) | `ProcessPoolExecutor` | plusieurs cœurs, chacun son GIL |
| attend beaucoup, avec des milliers de connexions | `asyncio` | très léger, voir [l'asynchrone](async.md) |
| fait passer des données d'une étape à l'autre | `threading` + `queue.Queue` | producteur et consommateur |
| est rapide et simple | rien ! | le code séquentiel est le plus facile à lire et à déboguer |

## Erreurs fréquentes

### Appeler la fonction au lieu de la donner

```python norun
import threading
import time

def telecharger(nom):
    time.sleep(1)
    print("fini :", nom)

t1 = threading.Thread(target=telecharger("a"))
t2 = threading.Thread(target=telecharger("b"))
t1.start()
t2.start()
```

```text
fini : a
fini : b
```

Pas de message d'erreur, mais le programme prend 2 secondes au lieu d'une :
`telecharger("a")` s'exécute **tout de suite**, dans le programme principal, et le
thread reçoit son résultat (`None`). Donnez la fonction sans parenthèses :
`target=telecharger, args=("a",)`.

### Démarrer deux fois le même thread

```python norun
import threading
t = threading.Thread(target=print, args=("a",))
t.start(); t.join()
t.start()
```

```text
a
Traceback (most recent call last):
  File "main.py", line 4, in <module>
    t.start()
    ~~~~~~~^^
  File "/usr/lib/python3.13/threading.py", line 970, in start
    raise RuntimeError("threads can only be started once")
RuntimeError: threads can only be started once
```

Un thread ne sert qu'une fois. Créez-en un nouveau, ou utilisez un pool.

### Envoyer une lambda à un processus

```python norun
from concurrent.futures import ProcessPoolExecutor

if __name__ == "__main__":
    with ProcessPoolExecutor() as ex:
        print(list(ex.map(lambda x: x * x, [1, 2, 3])))
```

```text
Traceback (most recent call last):
  ...
_pickle.PicklingError: Can't pickle <function <lambda> at 0x7f309fc5b2e0>: attribute lookup <lambda> on __main__ failed
```

Pour envoyer une fonction à un autre processus, Python doit la retrouver par son nom.
Une `lambda` n'en a pas. Définissez une vraie fonction avec `def` au niveau du module.

## Testez-vous

**Question 1 :** votre programme télécharge 50 pages web. Threads ou processus ?

> **Réponse**
>
> Des threads (`ThreadPoolExecutor`) : le programme passe son temps à attendre le
> réseau, et le GIL est relâché pendant ces attentes.

**Question 2 :** dans quel ordre `equipe.map(f, [3, 1, 2])` renvoie-t-il les résultats ?

> **Réponse**
>
> Dans l'ordre des valeurs données : `f(3)`, puis `f(1)`, puis `f(2)`, quel que soit
> l'ordre dans lequel les tâches finissent.

**Question 3 :** avec `ThreadPoolExecutor`, calculez la longueur de chaque mot de
`["chat", "éléphant", "ours"]` et affichez la liste.

> **Réponse**
>
> ```python
> from concurrent.futures import ThreadPoolExecutor
>
> with ThreadPoolExecutor() as equipe:
>     print(list(equipe.map(len, ["chat", "éléphant", "ours"])))
> ```
>
> ```sortie
> [4, 8, 4]
> ```

## En résumé

- `threading.Thread(target=f, args=(...))`, puis `start()` et `join()`.
- Le GIL laisse un seul thread exécuter du Python à la fois : les threads aident pour
  les attentes, pas pour les calculs.
- `ThreadPoolExecutor` pour les attentes, `ProcessPoolExecutor` pour les calculs ;
  `map` rend les résultats dans l'ordre.
- Une donnée partagée et modifiée se protège avec un `Lock` (`with verrou:`).
- `queue.Queue` transmet des objets d'un thread à l'autre, sans verrou à écrire.

Suite : [la programmation asynchrone](async.md). Comparez avec
[la concurrence en Rust](../rust/concurrence.md).
