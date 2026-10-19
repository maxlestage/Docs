# Qu'est-ce qu'une base de données ?

Une base de données range des informations en tableaux pour les retrouver vite et sans erreur : cette page vous montre les tables, les lignes, les colonnes, et les quatre ordres SQL de base, avec Python.

> **Objectifs :**
>
> - comprendre pourquoi un simple fichier texte ne suffit pas toujours ;
> - lire une table : ses lignes, ses colonnes et sa clé ;
> - écrire les ordres SQL `SELECT`, `INSERT`, `UPDATE` et `DELETE` ;
> - filtrer avec `WHERE` et trier avec `ORDER BY` ;
> - relier deux tables avec `JOIN`.

## Pourquoi pas un simple fichier texte ?

Imaginez que vous notez vos livres dans un fichier texte : une ligne par
livre, avec un `;` entre le titre, l'auteur et l'année. Cela marche…
jusqu'au jour où un titre contient lui-même un `;`. Regardez :

```python
lignes = [
    "Le Petit Prince;Saint-Exupéry;1943",
    "Paris; mode d'emploi;Dupont;2001",
]
for ligne in lignes:
    morceaux = ligne.split(";")
    print(len(morceaux), "morceaux :", morceaux)
```

```sortie
3 morceaux : ['Le Petit Prince', 'Saint-Exupéry', '1943']
4 morceaux : ['Paris', " mode d'emploi", 'Dupont', '2001']
```

La deuxième ligne est coupée en 4 au lieu de 3 : le programme croit que
l'auteur s'appelle « mode d'emploi ». Autres soucis : pour trouver un
livre, il faut lire **tout** le fichier, et rien n'empêche d'écrire
« mille neuf cent » à la place d'une année.

Une **base de données** règle ces problèmes : ce programme spécialisé garde
vos données, les vérifie et les retrouve très vite.

> **À retenir :** un fichier texte est un cahier ; une base de données, un classeur tenu par un bibliothécaire.

## Une table, c'est comme un tableur

Une base de données contient des **tables**. Une table ressemble à une
feuille de tableur (comme Excel ou LibreOffice Calc) :

```text
Table « livres »
┌────┬─────────────────────┬───────────────┬───────┐
│ id │ titre               │ auteur        │ annee │  ← les colonnes
├────┼─────────────────────┼───────────────┼───────┤
│ 1  │ Le Petit Prince     │ Saint-Exupéry │ 1943  │  ← une ligne
│ 2  │ Les Misérables      │ Hugo          │ 1862  │
│ 3  │ Notre-Dame de Paris │ Hugo          │ 1831  │
│ 4  │ L'Étranger          │ Camus         │ 1942  │
└────┴─────────────────────┴───────────────┴───────┘
```

- une **colonne** est une sorte d'information (le titre, l'année…). Chaque
  colonne a un [type](glossaire.md#type) : du texte, un nombre entier… ;
- une **ligne** est une chose rangée : ici, un livre ;
- la **clé** (ici `id`) est un numéro unique par ligne, comme un numéro de
  sécurité sociale : deux livres peuvent avoir le même titre, jamais le même `id`.

> **À retenir :** une table = des colonnes et des lignes ; la clé désigne une ligne sans erreur.

## SQL : la langue des bases de données

On parle à une base de données avec un langage : **SQL** (on dit « S-Q-L »
ou « sequel »). Ses phrases ressemblent à de l'anglais simple :

| Ordre SQL | En français          | Exemple                                  |
|-----------|----------------------|------------------------------------------|
| `SELECT`  | « montre-moi »       | `SELECT titre FROM livres`               |
| `INSERT`  | « ajoute »           | `INSERT INTO livres (titre) VALUES (…)`  |
| `UPDATE`  | « modifie »          | `UPDATE livres SET annee = 1943`         |
| `DELETE`  | « supprime »         | `DELETE FROM livres`                     |
| `WHERE`   | « seulement si… »    | `WHERE auteur = 'Hugo'`                  |
| `ORDER BY`| « trié par… »        | `ORDER BY annee`                         |

Pour essayer, nous utilisons **SQLite**, livré avec Python : rien à installer.
Avec `":memory:"`, la base vit dans la [mémoire vive](glossaire.md#memoire-vive)
et disparaît à la fin du programme : parfait pour s'entraîner.

## Créer une table et ajouter des lignes

Trois étapes : se connecter, créer la table, ajouter des lignes :

```python
import sqlite3

con = sqlite3.connect(":memory:")
con.execute("CREATE TABLE livres (id INTEGER PRIMARY KEY, titre TEXT, auteur TEXT, annee INTEGER)")
con.execute("INSERT INTO livres (titre, auteur, annee) VALUES (?, ?, ?)",
            ("Le Petit Prince", "Saint-Exupéry", 1943))

for ligne in con.execute("SELECT * FROM livres"):
    print(ligne)
```

```sortie
(1, 'Le Petit Prince', 'Saint-Exupéry', 1943)
```

Lisons-le ligne par ligne :

| Ligne du programme        | Ce qui se passe                                        |
|---------------------------|--------------------------------------------------------|
| `sqlite3.connect(…)`      | ouvre une base vide                                     |
| `CREATE TABLE livres (…)` | crée la table ; `id` est la clé, numérotée toute seule |
| `INSERT INTO … VALUES`    | ajoute une ligne                                        |
| `SELECT * FROM livres`    | « montre-moi toutes les colonnes de la table livres »   |

Les `?` sont des **places réservées**. Python y glisse les valeurs, dans
l'ordre. C'est la bonne habitude : la page sur
[la sécurité](securite.md) explique pourquoi.

Essayez : copiez l'`INSERT` avec `("Les Misérables", "Hugo", 1862)`.
Une deuxième ligne apparaît, avec l'`id` 2.

## Choisir des lignes avec WHERE

`WHERE` ne garde que certaines lignes. Ici, `executemany` ajoute tous les
livres d'un coup :

```python
import sqlite3

con = sqlite3.connect(":memory:")
con.execute("CREATE TABLE livres (id INTEGER PRIMARY KEY, titre TEXT, auteur TEXT, annee INTEGER)")
con.executemany("INSERT INTO livres (titre, auteur, annee) VALUES (?, ?, ?)", [
    ("Le Petit Prince", "Saint-Exupéry", 1943), ("Les Misérables", "Hugo", 1862),
    ("Notre-Dame de Paris", "Hugo", 1831), ("L'Étranger", "Camus", 1942),
])

for ligne in con.execute("SELECT titre, annee FROM livres WHERE auteur = 'Hugo'"):
    print(ligne)
```

```sortie
('Les Misérables', 1862)
('Notre-Dame de Paris', 1831)
```

La base regarde chaque ligne et pose la question « `auteur` vaut-il
`'Hugo'` ? » :

| id | auteur        | `auteur = 'Hugo'` ? | Gardée ? |
|----|---------------|---------------------|----------|
| 1  | Saint-Exupéry | non                 | non      |
| 2  | Hugo          | oui                 | **oui**  |
| 3  | Hugo          | oui                 | **oui**  |
| 4  | Camus         | non                 | non      |

En SQL, le texte s'écrit entre apostrophes `'…'`, et l'égalité est un seul
`=` (pas `==` comme en Python).

Essayez : remplacez la condition par `WHERE annee > 1900`. Vous obtenez
*Le Petit Prince* et *L'Étranger*.

> **À retenir :** `SELECT` choisit les colonnes, `WHERE` choisit les lignes.

## Trier avec ORDER BY

`ORDER BY` trie le résultat. Ajoutez `DESC` pour l'ordre inverse (du plus
grand au plus petit) :

```python
import sqlite3

con = sqlite3.connect(":memory:")
con.execute("CREATE TABLE livres (id INTEGER PRIMARY KEY, titre TEXT, annee INTEGER)")
con.executemany("INSERT INTO livres (titre, annee) VALUES (?, ?)", [
    ("Le Petit Prince", 1943), ("Les Misérables", 1862), ("L'Étranger", 1942),
])

for titre, annee in con.execute("SELECT titre, annee FROM livres ORDER BY annee"):
    print(annee, titre)
```

```sortie
1862 Les Misérables
1942 L'Étranger
1943 Le Petit Prince
```

Chaque ligne est un [tuple](glossaire.md#tuple), ouvert en deux variables
par `for titre, annee in …`.

Essayez : écrivez `ORDER BY annee DESC`. L'ordre s'inverse : 1943 d'abord.

## Modifier et supprimer : UPDATE et DELETE

`UPDATE` change des valeurs, `DELETE` enlève des lignes. Les deux
s'utilisent **toujours** avec un `WHERE`, pour viser les bonnes lignes :

```python
import sqlite3

con = sqlite3.connect(":memory:")
con.execute("CREATE TABLE livres (id INTEGER PRIMARY KEY, titre TEXT, annee INTEGER)")
con.executemany("INSERT INTO livres (titre, annee) VALUES (?, ?)", [
    ("Le Petit Prince", 1900), ("Les Misérables", 1862), ("Livre en double", 2000),
])

con.execute("UPDATE livres SET annee = ? WHERE id = ?", (1943, 1))
con.execute("DELETE FROM livres WHERE id = 3")

for ligne in con.execute("SELECT * FROM livres"):
    print(ligne)
```

```sortie
(1, 'Le Petit Prince', 1943)
(2, 'Les Misérables', 1862)
```

| id | Avant                  | Ordre    | Après                  |
|----|------------------------|----------|------------------------|
| 1  | Le Petit Prince, 1900  | `UPDATE` | Le Petit Prince, 1943  |
| 2  | Les Misérables, 1862   | (rien)   | Les Misérables, 1862   |
| 3  | Livre en double, 2000  | `DELETE` | (disparue)             |

> **À retenir :** `UPDATE` et `DELETE` sans `WHERE` touchent **toutes** les lignes.

## Relier deux tables avec JOIN

Écrire « Hugo » sur chaque livre répète la même information. On range donc
les auteurs dans **leur propre table**, et chaque livre garde seulement le
**numéro** de son auteur :

```text
   Table « auteurs »              Table « livres »
┌────┬───────────────┐     ┌────┬─────────────────────┬───────────┐
│ id │ nom           │     │ id │ titre               │ auteur_id │
├────┼───────────────┤     ├────┼─────────────────────┼───────────┤
│ 1  │ Saint-Exupéry │◄────┤ 1  │ Le Petit Prince     │ 1         │
│ 2  │ Hugo          │◄─┬──┤ 2  │ Les Misérables      │ 2         │
└────┴───────────────┘  └──┤ 3  │ Notre-Dame de Paris │ 2         │
                           └────┴─────────────────────┴───────────┘
```

Un `JOIN` (« joindre ») recolle les deux tables : pour chaque livre, la
base cherche l'auteur dont l'`id` vaut `auteur_id`.

```python
import sqlite3

con = sqlite3.connect(":memory:")
con.execute("CREATE TABLE auteurs (id INTEGER PRIMARY KEY, nom TEXT)")
con.execute("CREATE TABLE livres (id INTEGER PRIMARY KEY, titre TEXT, auteur_id INTEGER)")
con.executemany("INSERT INTO auteurs (nom) VALUES (?)", [("Saint-Exupéry",), ("Hugo",)])
con.executemany("INSERT INTO livres (titre, auteur_id) VALUES (?, ?)", [
    ("Le Petit Prince", 1), ("Les Misérables", 2), ("Notre-Dame de Paris", 2),
])

requete = """SELECT livres.titre, auteurs.nom FROM livres
             JOIN auteurs ON livres.auteur_id = auteurs.id
             ORDER BY livres.titre"""
for titre, nom in con.execute(requete):
    print(f"{titre} — {nom}")
```

```sortie
Le Petit Prince — Saint-Exupéry
Les Misérables — Hugo
Notre-Dame de Paris — Hugo
```

`livres.titre` veut dire « la colonne `titre` de la table `livres` ».

Essayez : ajoutez `WHERE auteurs.nom = 'Hugo'` juste avant `ORDER BY`.
Seuls les deux livres de Hugo restent.

> **À retenir :** chaque information est écrite **une seule fois** ; `JOIN` les rassemble.

## SQL, NoSQL et les outils

Les bases **SQL** (dites « relationnelles ») rangent tout en tables, comme
ici. Les bases **NoSQL** rangent les données autrement, souvent comme des
documents [JSON](glossaire.md#json) : plus souples, mais moins strictes.

Pour commencer, SQLite suffit : il est déjà dans Python. Plus tard, vous
installerez peut-être **DB Browser for SQLite** (pour voir vos tables à
l'écran), puis **PostgreSQL** ou **MySQL**, de grosses bases qui tournent
sur un [serveur](glossaire.md#serveur) pour les sites web. Pour aller plus
loin : [SQLite en Python](../python/sqlite.md) et
[les bases de données en Ruby](../ruby/base-de-donnees.md).

## Erreurs fréquentes

### Oublier le WHERE dans un UPDATE

Vous voulez corriger l'année d'un seul livre. Mais vous oubliez le `WHERE` :

```python
import sqlite3

con = sqlite3.connect(":memory:")
con.execute("CREATE TABLE livres (id INTEGER PRIMARY KEY, titre TEXT, annee INTEGER)")
con.executemany("INSERT INTO livres (titre, annee) VALUES (?, ?)", [
    ("Le Petit Prince", 1900), ("Les Misérables", 1862), ("L'Étranger", 1942),
])

con.execute("UPDATE livres SET annee = 1943")  # oups : pas de WHERE

for ligne in con.execute("SELECT * FROM livres"):
    print(ligne)
```

```sortie
(1, 'Le Petit Prince', 1943)
(2, 'Les Misérables', 1943)
(3, "L'Étranger", 1943)
```

Aucun message d'erreur : SQL a fait exactement ce qu'on lui a dit. Les
**trois** livres ont maintenant 1943. La correction :
`UPDATE livres SET annee = 1943 WHERE id = 1`. Astuce : lancez d'abord un
`SELECT` avec le même `WHERE` pour voir quelles lignes seront touchées.

### Une faute de frappe dans un ordre SQL

```python norun
import sqlite3

con = sqlite3.connect(":memory:")
con.execute("SELEC * FROM livres")
```

```text
Traceback (most recent call last):
  File "main.py", line 4, in <module>
    con.execute("SELEC * FROM livres")
    ~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
sqlite3.OperationalError: near "SELEC": syntax error
```

`near "SELEC"` veut dire « près du mot `SELEC` » : c'est là que la base ne
comprend plus. Corrigez en `SELECT`.

### Fabriquer l'ordre SQL en collant du texte

On est tenté de coller le texte de l'utilisateur dans l'ordre SQL avec `+`.
Avec un nom qui contient une apostrophe, tout casse :

```python norun
import sqlite3

con = sqlite3.connect(":memory:")
con.execute("CREATE TABLE eleves (id INTEGER PRIMARY KEY, nom TEXT)")
nom = "Zoé L'Hermite"
con.execute("INSERT INTO eleves (nom) VALUES ('" + nom + "')")
```

```text
Traceback (most recent call last):
  File "main.py", line 6, in <module>
    con.execute("INSERT INTO eleves (nom) VALUES ('" + nom + "')")
    ~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
sqlite3.OperationalError: near "Hermite": syntax error
```

L'apostrophe de « L'Hermite » ferme le texte trop tôt. Pire : un pirate
peut écrire du SQL à la place d'un nom (l'**injection SQL**, voir
[la sécurité](securite.md)). La correction, toujours la même, des `?` :
`con.execute("INSERT INTO eleves (nom) VALUES (?)", (nom,))`.

## Testez-vous

**1.** Dans une table, à quoi sert la clé (`id`) ?

> **Réponse**
>
> À désigner une ligne sans erreur : chaque ligne a sa propre clé.

**2.** Que fait `DELETE FROM livres` sans `WHERE` ?

> **Réponse**
>
> Il supprime **toutes** les lignes de la table. Il faut un `WHERE` pour
> viser seulement certaines lignes, par exemple `WHERE id = 3`.

**3.** Quel ordre SQL montre les titres parus après 1900, du plus récent au plus ancien ?

> **Réponse**
>
> `SELECT titre FROM livres WHERE annee > 1900 ORDER BY annee DESC`

## En résumé

- Une base de données range les informations en tables : des colonnes, des lignes, une clé.
- SQL parle à la base : `SELECT` lit, `INSERT` ajoute, `UPDATE` modifie, `DELETE` supprime.
- `WHERE` choisit les lignes, `ORDER BY` les trie ; sans `WHERE`, tout est touché.
- `JOIN` relie deux tables grâce à un numéro commun.
- Passez toujours les valeurs avec des `?`, jamais en collant du texte.

Suite : [Les bases de la sécurité](securite.md).
