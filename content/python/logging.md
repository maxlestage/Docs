# Journaliser avec logging

Cette page montre comment remplacer les `print` de contrôle par un vrai journal avec le module `logging` : des niveaux d'importance, un format fixe, un journal par module, un fichier, et des exceptions bien racontées.

> **Objectifs :**
>
> - vous saurez pourquoi un journal vaut mieux que des `print` ;
> - vous saurez utiliser les cinq niveaux, de `DEBUG` à `CRITICAL` ;
> - vous saurez régler l'affichage une seule fois avec `basicConfig` ;
> - vous saurez créer un journal par module avec `getLogger` ;
> - vous saurez écrire le journal dans un fichier ;
> - vous saurez enregistrer une exception avec sa trace.

## Pourquoi pas print ?

Le capitaine d'un bateau tient un **journal de bord**. Il y note l'heure, la météo, les
incidents. Il n'écrit pas tout à la même encre : un incident grave est souligné en rouge.
Et des mois plus tard, on relit le journal pour comprendre ce qui s'est passé.

Un programme peut tenir le même journal. Avec `print`, vous écrivez des messages, mais :

- ils sont tous pareils : impossible de savoir lequel est grave ;
- pour les cacher, il faut les effacer un par un ;
- ils vont seulement à l'écran, jamais dans un fichier ;
- ils ne disent pas d'où ils viennent.

Le module `logging` règle ces quatre problèmes.

```text
     print("ok")                       log.info("ok")
         │                                  │
         ▼                                  ▼
      l'écran                     ┌─ quel niveau ? ─┐
                                  │  assez grave ?   │── non ──► ignoré
                                  └────────┬─────────┘
                                           │ oui
                                   ┌───────┴────────┐
                                   ▼                ▼
                                l'écran        un fichier
```

Le chapitre [déboguer](debogage.md#garder-une-trace-avec-logging) a donné un premier
aperçu. Ici, on va plus loin.

> **À retenir :** `print` sert à montrer le **résultat** à l'utilisateur. `logging` sert
> à raconter **ce que fait** le programme, pour vous ou pour la personne qui le
> surveille.

## Un premier journal

Trois lignes suffisent pour commencer.

```python
import logging
import sys

logging.basicConfig(stream=sys.stdout, level=logging.INFO,
                    format="%(levelname)s:%(name)s:%(message)s")
log = logging.getLogger("boutique")

log.info("ouverture de la boutique")
log.warning("plus que 2 baguettes")
print("Bonjour !")
```

```sortie
INFO:boutique:ouverture de la boutique
WARNING:boutique:plus que 2 baguettes
Bonjour !
```

Ligne par ligne :

| Code | Ce qu'il fait |
|---|---|
| `logging.basicConfig(...)` | règle le journal, **une fois**, au début du programme |
| `stream=sys.stdout` | les messages vont sur la sortie normale (par défaut : la sortie d'erreur) |
| `level=logging.INFO` | on garde les messages `INFO` et plus graves |
| `format="..."` | la forme de chaque ligne (voir plus bas) |
| `logging.getLogger("boutique")` | prend un journal nommé `boutique` |
| `log.info(...)`, `log.warning(...)` | écrit un message, avec son niveau |

> **Note :** dans cette page, `stream=sys.stdout` est là pour que vous voyiez les
> messages mélangés aux `print`. Dans un vrai programme, on laisse souvent la sortie
> d'erreur par défaut : le journal ne se mélange pas alors au résultat.

## Les cinq niveaux

Chaque message a un **niveau** : son degré de gravité. C'est comme les alertes météo,
du vert au rouge.

| Niveau | Nombre | Quand l'utiliser | Exemple |
|---|---|---|---|
| `DEBUG` | 10 | les petits détails pour la mise au point | « x vaut 3 » |
| `INFO` | 20 | les étapes normales | « fichier chargé » |
| `WARNING` | 30 | bizarre, mais on continue | « stock bas » |
| `ERROR` | 40 | une opération a échoué | « paiement refusé » |
| `CRITICAL` | 50 | le programme ne peut plus continuer | « disque plein » |

Le réglage `level` est un **seuil**. Un message passe s'il est **au moins aussi grave**
que le seuil.

```text
 seuil = WARNING (30)

 CRITICAL 50  ████████████  passe
 ERROR    40  ██████████    passe
 WARNING  30  ████████      passe   ◄── seuil
 INFO     20  ██████        ignoré
 DEBUG    10  ████          ignoré
```

Regardez le même programme avec le seuil `WARNING` :

```python
import logging
import sys

logging.basicConfig(stream=sys.stdout, level=logging.WARNING,
                    format="%(levelname)s:%(name)s:%(message)s")
log = logging.getLogger("meteo")

log.debug("capteur lu : 18.4")
log.info("relevé terminé")
log.warning("vent fort")
log.error("pluviomètre en panne")
log.critical("station détruite")
```

```sortie
WARNING:meteo:vent fort
ERROR:meteo:pluviomètre en panne
CRITICAL:meteo:station détruite
```

Suivons chaque message :

| Message | Niveau | Seuil | 30 ≤ niveau ? | Affiché ? |
|---|---|---|---|---|
| capteur lu | 10 | 30 | non | non |
| relevé terminé | 20 | 30 | non | non |
| vent fort | 30 | 30 | oui | oui |
| pluviomètre en panne | 40 | 30 | oui | oui |
| station détruite | 50 | 30 | oui | oui |

> **Essayez :** remplacez `level=logging.WARNING` par `level=logging.DEBUG`. Les cinq
> lignes s'affichent. Avec `level=logging.CRITICAL`, il n'en reste qu'une.

Sans `level`, le seuil par défaut est `WARNING`. C'est pour ça que vos `log.info`
semblent parfois « ne rien faire ».

> **À retenir :** on écrit **tous** les messages dans le code, chacun avec le bon
> niveau. Puis **un seul** réglage, `level`, choisit ceux qu'on voit. On ne supprime
> plus de lignes pour faire le ménage.

## Le format d'une ligne

Le `format` est un modèle à trous. Chaque `%(nom)s` est remplacé par une information sur
le message.

| Trou | Remplacé par |
|---|---|
| `%(levelname)s` | le niveau : `INFO`, `ERROR`… |
| `%(name)s` | le nom du journal : `boutique` |
| `%(message)s` | le texte du message |
| `%(funcName)s` | le nom de la fonction qui a écrit |
| `%(lineno)d` | le numéro de ligne |
| `%(asctime)s` | la date et l'heure |

```python
import logging
import sys

logging.basicConfig(stream=sys.stdout, level=logging.DEBUG,
                    format="[%(levelname)s] %(funcName)s : %(message)s")
log = logging.getLogger("caisse")

def encaisser(montant):
    log.debug("montant reçu %s", montant)
    log.info("encaissé")

encaisser(12)
```

```sortie
[DEBUG] encaisser : montant reçu 12
[INFO] encaisser : encaissé
```

> **Note :** dans un vrai programme, on ajoute presque toujours `%(asctime)s` pour savoir
> **quand** c'est arrivé. Cette page ne l'utilise pas : l'heure change à chaque
> lancement, et les sorties montrées ici doivent rester identiques.

## Passer des valeurs avec %s

Pour mettre une valeur dans un message, écrivez `%s` dans le texte et donnez la valeur
après, séparée par une virgule.

```python
import logging
import sys

logging.basicConfig(stream=sys.stdout, level=logging.INFO,
                    format="%(levelname)s:%(name)s:%(message)s")
log = logging.getLogger("panier")

article = "pommes"
quantite = 3
log.info("ajout de %s %s", quantite, article)
log.debug("calcul très long : %s", sum(range(1000)))
```

```sortie
INFO:panier:ajout de 3 pommes
```

Le premier `%s` reçoit `quantite`, le second reçoit `article`. C'est le même principe
que le formulaire à trous de [sqlite3](sqlite.md).

Pourquoi pas une f-string ? Avec `%s`, le texte n'est fabriqué **que si** le message
passe le seuil. Un message `DEBUG` ignoré ne coûte presque rien. Une f-string, elle, est
toujours fabriquée, même pour rien.

> **Attention :** `sum(range(1000))` est quand même calculé, car Python évalue les
> arguments avant l'appel. Seule la mise en forme du texte est évitée.

> **À retenir :** écrivez `log.info("total : %s", total)`, pas
> `log.info(f"total : {total}")`. Autant de `%s` que de valeurs.

## Un journal par module

Dans un gros programme, chaque [module](modules.md) a son propre journal. On lui donne
le nom du module avec `__name__`. Ainsi, chaque ligne dit **d'où** elle vient.

```python norun
# fichier panier.py
import logging

log = logging.getLogger(__name__)   # le nom sera "panier"

def ajouter(article):
    log.info("ajout de %s", article)
```

Dans le programme principal, `__name__` vaut `"__main__"`. Seul ce fichier principal
appelle `basicConfig`. Les modules, eux, se contentent de `getLogger(__name__)`.

```text
programme principal  ── basicConfig(...)  ← un seul réglage
   │
   ├── panier.py     ── getLogger("panier")
   ├── paiement.py   ── getLogger("paiement")
   └── stock.py      ── getLogger("stock")
```

### Des noms en famille

Les noms avec un point forment une famille : `app.db` est l'enfant de `app`. On peut
régler le niveau de chaque journal séparément.

```python
import logging
import sys

logging.basicConfig(stream=sys.stdout, level=logging.INFO,
                    format="%(levelname)s:%(name)s:%(message)s")

log_app = logging.getLogger("app")
log_db = logging.getLogger("app.db")
log_web = logging.getLogger("app.web")

log_db.setLevel(logging.DEBUG)

log_app.info("démarrage")
log_db.debug("requête SELECT envoyée")
log_web.debug("page demandée")
log_web.info("page envoyée")
print(log_db.parent.name)
```

```sortie
INFO:app:démarrage
DEBUG:app.db:requête SELECT envoyée
INFO:app.web:page envoyée
app
```

| Journal | Niveau réglé | Seuil utilisé | `debug` affiché ? |
|---|---|---|---|
| `app` | rien | celui de `basicConfig` : INFO | non |
| `app.db` | DEBUG | DEBUG | oui |
| `app.web` | rien | INFO (hérité) | non |

C'est pratique : vous enquêtez sur la base de données ? Vous mettez **seulement**
`app.db` en `DEBUG`, sans être noyé sous les détails du reste.

> **Essayez :** ajoutez `log_web.setLevel(logging.WARNING)`. La ligne `page envoyée`
> disparaît aussi.

> **À retenir :** dans chaque module, `log = logging.getLogger(__name__)`. Un seul
> `basicConfig`, dans le programme principal.

## Écrire le journal dans un fichier

Un journal à l'écran disparaît quand on ferme le terminal. Pour le garder, on l'envoie
dans un [fichier](fichiers.md).

### Avec basicConfig

Le plus simple : `filename=` à la place de `stream=`.

```python
import logging

logging.basicConfig(filename="journal.log", level=logging.INFO,
                    format="%(levelname)s:%(name)s:%(message)s", encoding="utf-8")
log = logging.getLogger("sauvegarde")

log.info("copie de 3 fichiers")
log.warning("1 fichier ignoré")
logging.shutdown()

with open("journal.log", encoding="utf-8") as f:
    print(f.read(), end="")
```

```sortie
INFO:sauvegarde:copie de 3 fichiers
WARNING:sauvegarde:1 fichier ignoré
```

Rien ne s'affiche pendant le programme : tout part dans `journal.log`. On relit le
fichier à la fin pour vérifier. `logging.shutdown()` ferme proprement le fichier.

> **Note :** par défaut, le fichier est complété à chaque lancement (mode `"a"`). Ajoutez
> `filemode="w"` pour repartir d'un fichier vide.

### Avec des handlers : l'écran et le fichier

Un **handler** (« gestionnaire ») est une destination pour les messages. Un journal
peut avoir plusieurs handlers, chacun avec son propre seuil. C'est comme un courrier
envoyé en copie : une copie pour vous, une copie pour les archives.

Exemple courant : à l'écran, seulement les problèmes ; dans le fichier, tout.

```python
import logging
import sys

log = logging.getLogger("robot")
log.setLevel(logging.DEBUG)
forme = logging.Formatter("%(levelname)s:%(name)s:%(message)s")

ecran = logging.StreamHandler(sys.stdout)
ecran.setLevel(logging.WARNING)
ecran.setFormatter(forme)
log.addHandler(ecran)

fichier = logging.FileHandler("robot.log", mode="w", encoding="utf-8")
fichier.setLevel(logging.DEBUG)
fichier.setFormatter(forme)
log.addHandler(fichier)

log.debug("moteur gauche : 40 %")
log.info("avance de 2 m")
log.warning("obstacle détecté")

fichier.close()
print("--- contenu de robot.log ---")
with open("robot.log", encoding="utf-8") as f:
    print(f.read(), end="")
```

```sortie
WARNING:robot:obstacle détecté
--- contenu de robot.log ---
DEBUG:robot:moteur gauche : 40 %
INFO:robot:avance de 2 m
WARNING:robot:obstacle détecté
```

Suivons chaque message dans les deux handlers :

| Message | Niveau | Écran (seuil WARNING) | Fichier (seuil DEBUG) |
|---|---|---|---|
| moteur gauche | DEBUG | non | oui |
| avance de 2 m | INFO | non | oui |
| obstacle détecté | WARNING | oui | oui |

```text
                    log.warning("obstacle détecté")
                               │
                     journal "robot" (seuil DEBUG)
                       ┌───────┴────────┐
                       ▼                ▼
             StreamHandler          FileHandler
             seuil WARNING          seuil DEBUG
                   │                    │
                l'écran             robot.log
```

Il y a donc **deux** filtres : le seuil du journal (`log.setLevel`), puis le seuil de
chaque handler. Un message doit passer les deux.

> **Essayez :** passez le seuil de l'écran à `logging.DEBUG`. Les trois messages
> s'affichent alors aussi à l'écran, avant le contenu du fichier.

> **À retenir :** `basicConfig(filename=...)` pour un simple fichier. Des handlers quand
> vous voulez plusieurs destinations avec des seuils différents.

## Journaliser une exception

Quand une [exception](../debuter/glossaire.md#exception) est attrapée, il faut garder
une trace de **ce qui** a échoué. Deux façons.

### Le message de l'erreur avec log.error

```python
import logging
import sys

logging.basicConfig(stream=sys.stdout, level=logging.INFO,
                    format="%(levelname)s:%(name)s:%(message)s")
log = logging.getLogger("import")

lignes = ["12", "8", "douze", "5"]
total = 0
for numero, texte in enumerate(lignes, start=1):
    try:
        total += int(texte)
    except ValueError as erreur:
        log.error("ligne %s ignorée : %s", numero, erreur)

log.info("total = %s", total)
```

```sortie
ERROR:import:ligne 3 ignorée : invalid literal for int() with base 10: 'douze'
INFO:import:total = 25
```

| Tour | `numero` | `texte` | `int(texte)` | `total` |
|---|---|---|---|---|
| 1 | 1 | "12" | 12 | 12 |
| 2 | 2 | "8" | 8 | 20 |
| 3 | 3 | "douze" | `ValueError` → message ERROR | 20 |
| 4 | 4 | "5" | 5 | 25 |

Le programme ne s'arrête pas. Mais la ligne fautive est notée dans le journal.

### Toute la trace avec log.exception

`log.exception` écrit un message de niveau `ERROR` **et** la trace complète de
l'erreur, comme Python l'aurait affichée. On l'appelle seulement dans un `except`.

```python norun
import logging
import sys

logging.basicConfig(stream=sys.stdout, level=logging.INFO,
                    format="%(levelname)s:%(name)s:%(message)s")
log = logging.getLogger("calcul")

def diviser(a, b):
    return a / b

try:
    diviser(10, 0)
except ZeroDivisionError:
    log.exception("la division a échoué")
print("le programme continue")
```

```text
ERROR:calcul:la division a échoué
Traceback (most recent call last):
  File "main.py", line 12, in <module>
    diviser(10, 0)
    ~~~~~~~^^^^^^^
  File "main.py", line 9, in diviser
    return a / b
           ~~^~~
ZeroDivisionError: division by zero
le programme continue
```

La trace dit exactement où chercher : ligne 9, dans `diviser`. Et le programme continue.
Le chapitre [les erreurs](erreurs.md) explique `try` et `except`.

> **À retenir :** dans un `except`, `log.exception("...")` garde la trace complète.
> Sans trace, une erreur notée « ça a raté » ne vous aidera pas à la corriger.

## Un exemple complet

Un petit programme qui traite des commandes. Il raconte ce qu'il fait, sans un seul
`print` de contrôle.

```python
import logging
import sys

logging.basicConfig(stream=sys.stdout, level=logging.INFO,
                    format="%(levelname)s:%(name)s:%(message)s")
log = logging.getLogger("commandes")

stock = {"stylo": 10, "cahier": 2}

def commander(article, quantite):
    log.debug("demande : %s x %s", quantite, article)
    if article not in stock:
        log.error("article inconnu : %s", article)
        return False
    if stock[article] < quantite:
        log.warning("stock insuffisant pour %s (%s demandés, %s en stock)",
                    article, quantite, stock[article])
        return False
    stock[article] -= quantite
    log.info("%s x %s envoyés", quantite, article)
    return True

commander("stylo", 3)
commander("cahier", 5)
commander("gomme", 1)
print("Stock final :", stock)
```

```sortie
INFO:commandes:3 x stylo envoyés
WARNING:commandes:stock insuffisant pour cahier (5 demandés, 2 en stock)
ERROR:commandes:article inconnu : gomme
Stock final : {'stylo': 7, 'cahier': 2}
```

| Appel | Article connu ? | Assez de stock ? | Niveau écrit | `stock` après |
|---|---|---|---|---|
| `commander("stylo", 3)` | oui | oui (10 ≥ 3) | INFO | stylo 7, cahier 2 |
| `commander("cahier", 5)` | oui | non (2 < 5) | WARNING | stylo 7, cahier 2 |
| `commander("gomme", 1)` | non | — | ERROR | stylo 7, cahier 2 |

> **Essayez :** passez `level` à `logging.DEBUG`. Chaque commande affiche d'abord sa
> ligne `DEBUG:commandes:demande : …`.

## Erreurs fréquentes

### Plus de %s que de valeurs (ou l'inverse)

```python norun
import logging
import sys
logging.basicConfig(stream=sys.stdout, format="%(levelname)s:%(name)s:%(message)s")
log = logging.getLogger("app")
log.warning("Bonjour %s", "Léa", "Hugo")
```

```text
--- Logging error ---
Traceback (most recent call last):
  File "/usr/lib/python3.13/logging/__init__.py", line 1151, in emit
    msg = self.format(record)
  ...
TypeError: not all arguments converted during string formatting
Call stack:
  File "main.py", line 5, in <module>
    log.warning("Bonjour %s", "Léa", "Hugo")
Message: 'Bonjour %s'
Arguments: ('Léa', 'Hugo')
```

Un seul `%s` mais deux valeurs. Le programme ne plante pas, mais le message est perdu.
Comptez : autant de `%s` que de valeurs, `"Bonjour %s et %s"`.

### Appeler basicConfig deux fois

```python
import logging
import sys

logging.basicConfig(stream=sys.stdout, level=logging.WARNING,
                    format="%(levelname)s:%(name)s:%(message)s")
logging.basicConfig(stream=sys.stdout, level=logging.DEBUG,
                    format="%(levelname)s:%(name)s:%(message)s")
log = logging.getLogger("app")
log.debug("je suis là ?")
log.warning("moi, oui")
```

```sortie
WARNING:app:moi, oui
```

Pas d'erreur, mais le deuxième `basicConfig` est ignoré : le journal était déjà réglé.
Un seul `basicConfig`, au tout début. Pour vraiment tout régler à nouveau, ajoutez
`force=True`.

### log.exception en dehors d'un except

```python
import logging
import sys

logging.basicConfig(stream=sys.stdout, format="%(levelname)s:%(name)s:%(message)s")
logging.getLogger("app").exception("ça ne va pas")
```

```sortie
ERROR:app:ça ne va pas
NoneType: None
```

Aucune exception n'est en cours : il n'y a pas de trace à montrer, d'où le curieux
`NoneType: None`. Hors d'un `except`, utilisez `log.error`.

## Testez-vous

**Question 1 :** le seuil est `INFO`. Quels messages s'affichent parmi `debug`, `info`,
`warning` et `error` ?

> **Réponse**
>
> `info`, `warning` et `error`. Seul `debug` (10) est sous le seuil `INFO` (20).

**Question 2 :** qu'affiche ce programme ?

```python norun
import logging
import sys
logging.basicConfig(stream=sys.stdout, level=logging.ERROR,
                    format="%(levelname)s:%(name)s:%(message)s")
log = logging.getLogger("test")
log.warning("attention")
log.error("panne %s", 42)
```

> **Réponse**
>
> ```python
> import logging
> import sys
> logging.basicConfig(stream=sys.stdout, level=logging.ERROR,
>                     format="%(levelname)s:%(name)s:%(message)s")
> log = logging.getLogger("test")
> log.warning("attention")
> log.error("panne %s", 42)
> ```
>
> ```sortie
> ERROR:test:panne 42
> ```
>
> `WARNING` (30) est sous le seuil `ERROR` (40) : il est ignoré. Le `%s` reçoit 42.

**Question 3 :** vous voulez voir les erreurs à l'écran, mais garder **tous** les
détails dans un fichier. Que faites-vous ?

> **Réponse**
>
> Un journal au seuil `DEBUG` avec deux handlers : un `StreamHandler` au seuil
> `WARNING` (ou `ERROR`) pour l'écran, et un `FileHandler` au seuil `DEBUG` pour le
> fichier.

## En résumé

- `print` montre le résultat ; `logging` raconte ce que fait le programme.
- Cinq niveaux : `DEBUG` < `INFO` < `WARNING` < `ERROR` < `CRITICAL`. Le seuil `level`
  choisit ce qui passe ; par défaut, c'est `WARNING`.
- Un seul `basicConfig`, au début du programme principal ; `getLogger(__name__)` dans
  chaque module.
- Des valeurs avec `%s` : `log.info("total : %s", total)`.
- `filename=` ou un `FileHandler` pour garder le journal dans un fichier.
- Dans un `except`, `log.exception` garde la trace complète de l'erreur.

Suite : [Bien écrire du Python](style.md)
