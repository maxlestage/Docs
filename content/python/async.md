# La programmation asynchrone

Cette page montre comment un seul programme peut mener de front beaucoup d'attentes grâce à `async`, `await` et le module `asyncio`.

> **Objectifs :**
>
> - vous saurez écrire une coroutine avec `async def` et la lancer avec `asyncio.run` ;
> - vous saurez attendre avec `await` sans bloquer le reste du programme ;
> - vous saurez lancer plusieurs coroutines ensemble avec `asyncio.gather` ;
> - vous saurez créer des tâches et limiter une attente avec un délai maximum ;
> - vous saurez choisir entre asynchrone, threads et processus.

## Un seul cuisinier bien organisé

Au [chapitre précédent](concurrence.md), plusieurs cuisiniers (les threads) travaillaient
en même temps. La programmation [asynchrone](../debuter/glossaire.md#async) prend une
autre approche : **un seul** cuisinier, mais qui ne reste jamais planté devant une
casserole. Il met l'eau à chauffer, puis coupe les légumes, puis revient à l'eau quand
elle bout.

Le secret : chaque tâche **dit** quand elle attend. Le cuisinier (appelé **boucle
d'événements**, *event loop*) en profite pour faire avancer une autre tâche.

## async def et await

Une fonction écrite avec `async def` est une **coroutine** : une fonction qui peut se
mettre en pause. À l'intérieur, `await` (« attendre ») marque un point de pause : « je
patiente ici, occupe-toi des autres en attendant ».

`asyncio.run` démarre la boucle d'événements et exécute la coroutine principale.

```python
import asyncio

async def saluer():
    print("Bonjour...")
    await asyncio.sleep(0.1)
    print("... tout le monde !")

asyncio.run(saluer())
```

```sortie
Bonjour...
... tout le monde !
```

`asyncio.sleep` est la version asynchrone de `time.sleep` : elle attend **sans bloquer**
le cuisinier.

> **À retenir :** `await` ne s'écrit que **dans** une fonction `async def`. Et une
> coroutine ne s'exécute que si on l'attend avec `await` (ou qu'on la lance avec
> `asyncio.run`).

### Une coroutine renvoie une valeur

Comme une fonction normale, une coroutine peut renvoyer une valeur. On la récupère
avec `await`.

```python
import asyncio

async def prix_ttc(prix_ht):
    await asyncio.sleep(0.1)
    return round(prix_ht * 1.2, 2)

async def main():
    prix = await prix_ttc(50)
    print("Prix TTC :", prix)

asyncio.run(main())
```

```sortie
Prix TTC : 60.0
```

On écrit presque toujours une coroutine `main` qui organise le reste, et un seul
`asyncio.run(main())` à la fin du fichier.

## Attendre l'une après l'autre ou ensemble

Si vous attendez deux coroutines l'une après l'autre, les attentes s'additionnent.
`asyncio.gather` les lance **ensemble** et attend qu'elles aient toutes fini.

```python
import asyncio
import time

async def cuire(plat, secondes):
    await asyncio.sleep(secondes)
    return f"{plat} cuit"

async def main():
    debut = time.perf_counter()
    await cuire("riz", 1)
    await cuire("légumes", 1)
    print("L'un après l'autre :", round(time.perf_counter() - debut), "s")

    debut = time.perf_counter()
    await asyncio.gather(cuire("riz", 1), cuire("légumes", 1))
    print("Ensemble :", round(time.perf_counter() - debut), "s")

asyncio.run(main())
```

```sortie
L'un après l'autre : 2 s
Ensemble : 1 s
```

### L'ordre des résultats de gather

`gather` renvoie une liste de résultats **dans l'ordre des arguments**, pas dans
l'ordre où les coroutines finissent. Les `print` faits pendant l'exécution, eux,
suivent l'ordre réel.

```python
import asyncio

async def livrer(colis, delai):
    await asyncio.sleep(delai)
    print("Arrivé :", colis)
    return colis.upper()

async def main():
    resultats = await asyncio.gather(
        livrer("lent", 0.3),
        livrer("rapide", 0.1),
        livrer("moyen", 0.2),
    )
    print("Résultats :", resultats)

asyncio.run(main())
```

```sortie
Arrivé : rapide
Arrivé : moyen
Arrivé : lent
Résultats : ['LENT', 'RAPIDE', 'MOYEN']
```

Avec une liste de coroutines, ajoutez une étoile pour les « déballer » :
`await asyncio.gather(*coroutines)`.

```python
import asyncio

async def carre(n):
    await asyncio.sleep(0.01 * (5 - n))
    return n * n

async def main():
    print(await asyncio.gather(*[carre(n) for n in range(1, 5)]))

asyncio.run(main())
```

```sortie
[1, 4, 9, 16]
```

## Les tâches

`asyncio.create_task` lance une coroutine **en arrière-plan** et renvoie une tâche
(`Task`). La coroutine commence dès que `main` fait une pause ; on récupère son
résultat plus tard avec `await tache`.

```python
import asyncio

async def telecharger(nom, delai):
    await asyncio.sleep(delai)
    return f"{nom} téléchargé"

async def main():
    tache = asyncio.create_task(telecharger("photo.jpg", 0.2))
    print("Téléchargement lancé, je fais autre chose...")
    await asyncio.sleep(0.1)
    print("Autre chose fini.")
    print(await tache)

asyncio.run(main())
```

```sortie
Téléchargement lancé, je fais autre chose...
Autre chose fini.
photo.jpg téléchargé
```

### TaskGroup

Depuis Python 3.11, `asyncio.TaskGroup` regroupe des tâches : à la sortie du bloc
`async with`, toutes sont terminées. Si l'une échoue, les autres sont annulées.

```python
import asyncio

async def compter(nom, n):
    for i in range(1, n + 1):
        await asyncio.sleep(0.1)
    return f"{nom} a compté jusqu'à {n}"

async def main():
    async with asyncio.TaskGroup() as groupe:
        t1 = groupe.create_task(compter("Léa", 2))
        t2 = groupe.create_task(compter("Hugo", 3))
    print(t1.result())
    print(t2.result())

asyncio.run(main())
```

```sortie
Léa a compté jusqu'à 2
Hugo a compté jusqu'à 3
```

## Limiter le temps d'attente

Un serveur qui ne répond pas ne doit pas bloquer votre programme pour toujours.
`asyncio.timeout(secondes)` fixe un délai maximum ; s'il est dépassé, une
`TimeoutError` est levée.

```python
import asyncio

async def requete(delai):
    await asyncio.sleep(delai)
    return "réponse"

async def main():
    for delai in [0.1, 2]:
        try:
            async with asyncio.timeout(0.5):
                print(await requete(delai))
        except TimeoutError:
            print(f"Trop long (plus de 0,5 s) pour un délai de {delai} s")

asyncio.run(main())
```

```sortie
réponse
Trop long (plus de 0,5 s) pour un délai de 2 s
```

`asyncio.wait_for(coroutine, timeout=0.5)` fait la même chose, en une seule
expression.

## Un exemple complet : vérifier des sites

Voici le schéma typique : plusieurs « requêtes » simulées, un délai maximum pour
chacune, et un résumé à la fin. Les délais remplacent le temps de réponse de vrais
serveurs.

```python
import asyncio

TEMPS_DE_REPONSE = {"exemple.fr": 0.2, "lent.com": 3, "python.org": 0.1}

async def verifier(site):
    try:
        async with asyncio.timeout(0.5):
            await asyncio.sleep(TEMPS_DE_REPONSE[site])
            return site, "en ligne"
    except TimeoutError:
        return site, "ne répond pas"

async def main():
    resultats = await asyncio.gather(*[verifier(s) for s in TEMPS_DE_REPONSE])
    for site, etat in resultats:
        print(f"{site:12} {etat}")

asyncio.run(main())
```

```sortie
exemple.fr   en ligne
lent.com     ne répond pas
python.org   en ligne
```

Le programme entier dure environ 0,5 seconde, pas 3,3 : les trois vérifications se
font en même temps.

Pour de vraies requêtes web asynchrones, on utilise un paquet externe comme `httpx` ou
`aiohttp` (voir [Python pour le web](web.md)).

## Asynchrone, threads ou processus ?

| | `asyncio` | threads | processus |
|--|-----------|---------|-----------|
| Combien de « travailleurs » | un seul, qui jongle | plusieurs, avec le GIL | plusieurs, sans partage |
| Idéal pour | des milliers d'attentes (réseau) | quelques dizaines d'attentes | des calculs lourds |
| Qui décide des pauses | vous, avec `await` | le système, n'importe quand | le système |
| Coût d'une tâche | très faible | moyen | élevé |
| Bibliothèques | doivent être asynchrones | n'importe lesquelles | n'importe lesquelles |
| Risque principal | une fonction bloquante fige tout | accès simultanés à une donnée | lenteur au démarrage |

> **Attention :** dans une coroutine, n'utilisez jamais une fonction qui bloque
> longtemps (`time.sleep`, `requests.get`, un gros calcul) : le cuisinier unique reste
> bloqué et plus rien n'avance. Si vous devez appeler une telle fonction, utilisez
> `await asyncio.to_thread(fonction, arguments)`.

```python
import asyncio
import time

def ancienne_fonction(x):
    time.sleep(0.1)
    return x * 10

async def main():
    resultats = await asyncio.gather(
        asyncio.to_thread(ancienne_fonction, 1),
        asyncio.to_thread(ancienne_fonction, 2),
    )
    print(resultats)

asyncio.run(main())
```

```sortie
[10, 20]
```

## Erreurs fréquentes

### Oublier await

```python norun
import asyncio

async def dire():
    print("coucou")

dire()
```

```text
main.py:6: RuntimeWarning: coroutine 'dire' was never awaited
  dire()
RuntimeWarning: Enable tracemalloc to get the object allocation traceback
```

Appeler `dire()` crée une coroutine, mais ne l'exécute pas : `coucou` n'est jamais
affiché. Écrivez `asyncio.run(dire())`, ou `await dire()` dans une autre coroutine.

### await en dehors d'une coroutine

```python norun
import asyncio

async def f():
    return 1

x = await f()
```

```text
  File "main.py", line 6
    x = await f()
        ^^^^^^^^^
SyntaxError: 'await' outside function
```

`await` n'est permis que dans une fonction `async def`. Au niveau du fichier, utilisez
`asyncio.run(f())`.

### Appeler asyncio.run dans une coroutine

```python norun
import asyncio

async def main():
    asyncio.run(asyncio.sleep(0))

asyncio.run(main())
```

```text
Traceback (most recent call last):
  ...
RuntimeError: asyncio.run() cannot be called from a running event loop
```

`asyncio.run` démarre la boucle ; il n'en faut qu'une. Dans une coroutine, écrivez
simplement `await asyncio.sleep(0)`.

### Utiliser time.sleep au lieu d'asyncio.sleep

```python norun
import asyncio
import time

async def cuire(plat):
    time.sleep(1)
    return plat

async def main():
    debut = time.perf_counter()
    await asyncio.gather(cuire("riz"), cuire("légumes"))
    print(round(time.perf_counter() - debut), "s")

asyncio.run(main())
```

```text
2 s
```

Aucun message d'erreur, mais rien ne se fait en même temps : `time.sleep` bloque le
cuisinier. Remplacez-le par `await asyncio.sleep(1)`, et le programme dure 1 seconde.

## Testez-vous

**Question 1 :** que se passe-t-il si on écrit `asyncio.sleep(1)` sans `await` dans une
coroutine ?

> **Réponse**
>
> Rien n'est attendu : la coroutine `sleep` est créée puis abandonnée, et Python
> affiche l'avertissement `coroutine 'sleep' was never awaited`.

**Question 2 :** trois coroutines durent 0,3 s, 0,1 s et 0,2 s. Combien de temps dure
`gather` sur les trois, et dans quel ordre sont les résultats ?

> **Réponse**
>
> Environ 0,3 s (la plus longue). Les résultats sont dans l'ordre des arguments, pas
> dans l'ordre de fin.

**Question 3 :** écrivez une coroutine `doubler(n)` qui attend 0,1 s puis renvoie
`2 * n`, et affichez les doubles de 1, 2 et 3 avec `gather`.

> **Réponse**
>
> ```python
> import asyncio
>
> async def doubler(n):
>     await asyncio.sleep(0.1)
>     return 2 * n
>
> async def main():
>     print(await asyncio.gather(doubler(1), doubler(2), doubler(3)))
>
> asyncio.run(main())
> ```
>
> ```sortie
> [2, 4, 6]
> ```

## En résumé

- `async def` crée une coroutine ; `await` y marque une pause qui laisse travailler les
  autres.
- `asyncio.run(main())` démarre la boucle d'événements, une seule fois.
- `asyncio.gather` lance plusieurs coroutines ensemble ; les résultats suivent l'ordre
  des arguments.
- `asyncio.create_task` et `TaskGroup` lancent des tâches en arrière-plan.
- `asyncio.timeout` limite une attente et lève `TimeoutError`.
- L'asynchrone excelle pour des milliers d'attentes ; jamais de `time.sleep` dedans.

Suite : [Python pour le web](web.md). Comparez avec
[l'asynchrone en Rust](../rust/async.md).
