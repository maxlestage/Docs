# Une base de données avec sqlite3

Cette page montre comment ranger, retrouver, modifier et supprimer des données dans une petite base de données SQL avec le module `sqlite3`, livré avec Python.

> **Objectifs :**
>
> - vous saurez créer une table et y ajouter des lignes ;
> - vous saurez lire des lignes avec `SELECT` et `fetchall` ;
> - vous saurez modifier et supprimer des lignes ;
> - vous saurez regrouper des changements dans une transaction avec `with conn:` ;
> - vous saurez pourquoi il faut toujours passer les valeurs avec `?`.

## Une base de données, c'est quoi ?

Une base de données, c'est une armoire à fiches très bien rangée. Chaque tiroir est une
**table**. Chaque fiche est une **ligne**. Chaque case de la fiche est une **colonne**.

```text
table « amis »
┌────┬─────────┬─────┐
│ id │ nom     │ age │   ← les colonnes
├────┼─────────┼─────┤
│ 1  │ Alice   │ 30  │   ← une ligne
│ 2  │ Bob     │ 25  │   ← une autre ligne
└────┴─────────┴─────┘
```

On parle à la base dans un petit langage à part : **SQL**. Une phrase SQL s'appelle une
**requête**. Elle ressemble à de l'anglais : `SELECT nom FROM amis` veut dire
« choisis les noms dans la table amis ».

SQLite est une base de données qui tient dans un seul fichier, sans serveur à installer.
Python sait déjà lui parler grâce au [module](../debuter/glossaire.md#module) `sqlite3`.
La page [les bases de données](../debuter/bases-de-donnees.md) explique l'idée générale.

> **Note :** dans cette page, la base est créée en
> [mémoire vive](../debuter/glossaire.md#memoire-vive) avec `":memory:"`. Elle disparaît
> à la fin du programme. Pour la garder, donnez un nom de fichier :
> `sqlite3.connect("carnet.db")`.

## Ouvrir une base et créer une table

Trois étapes : se connecter, créer la table, vérifier.

```python
import sqlite3

conn = sqlite3.connect(":memory:")
conn.execute("CREATE TABLE amis (id INTEGER PRIMARY KEY, nom TEXT, age INTEGER)")
print("Table créée")
```

```sortie
Table créée
```

Ligne par ligne :

| Code | Ce qu'il fait |
|---|---|
| `sqlite3.connect(":memory:")` | ouvre une base et renvoie une **connexion** |
| `conn.execute("...")` | envoie une requête SQL à la base |
| `CREATE TABLE amis (...)` | crée le tiroir `amis` avec trois colonnes |
| `id INTEGER PRIMARY KEY` | un numéro unique, donné tout seul par la base |
| `nom TEXT`, `age INTEGER` | une colonne de texte, une colonne de nombres entiers |

La connexion, c'est le téléphone : tant qu'elle est ouverte, vous pouvez parler à la
base.

## Ajouter une ligne avec ?

Pour ajouter une fiche, on utilise `INSERT INTO`. Les valeurs ne sont **pas** écrites
dans le texte de la requête. On met un `?` à leur place, et on donne les valeurs à côté,
dans un [tuple](../debuter/glossaire.md#tuple).

```python
import sqlite3

conn = sqlite3.connect(":memory:")
conn.execute("CREATE TABLE amis (id INTEGER PRIMARY KEY, nom TEXT, age INTEGER)")

conn.execute("INSERT INTO amis (nom, age) VALUES (?, ?)", ("Alice", 30))
conn.execute("INSERT INTO amis (nom, age) VALUES (?, ?)", ("Bob", 25))

print(conn.execute("SELECT * FROM amis").fetchall())
```

```sortie
[(1, 'Alice', 30), (2, 'Bob', 25)]
```

Le premier `?` reçoit `"Alice"`, le second reçoit `30`. C'est comme un formulaire à
trous : la requête est le formulaire, le tuple remplit les trous dans l'ordre.

```text
"INSERT INTO amis (nom, age) VALUES (?,      ?)"
                                     ▲       ▲
                                     │       │
                                 ("Alice",   30)
```

Nous n'avons pas donné d'`id` : la base a mis `1`, puis `2`, toute seule.

> **À retenir :** jamais de valeur collée dans le texte de la requête. Toujours un `?`,
> et les valeurs dans un tuple à côté. La section
> [se protéger des injections SQL](#se-proteger-des-injections-sql) explique pourquoi.

> **Essayez :** ajoutez une troisième ligne pour `"Chloé"`, 41 ans. La liste affichée a
> alors trois tuples, et Chloé a l'`id` 3.

### Ajouter plusieurs lignes d'un coup

`executemany` fait la même requête pour chaque élément d'une liste.

```python
import sqlite3

conn = sqlite3.connect(":memory:")
conn.execute("CREATE TABLE amis (id INTEGER PRIMARY KEY, nom TEXT, age INTEGER)")

amis = [("Alice", 30), ("Bob", 25), ("Chloé", 41)]
conn.executemany("INSERT INTO amis (nom, age) VALUES (?, ?)", amis)

print(conn.execute("SELECT COUNT(*) FROM amis").fetchone())
```

```sortie
(3,)
```

`COUNT(*)` compte les lignes. Le résultat est un tuple d'**une** valeur : `(3,)`.

## Lire des lignes

`SELECT` choisit les lignes. Ensuite, deux façons de les récupérer :

- `fetchall()` donne **toutes** les lignes, dans une liste ;
- `fetchone()` donne **la prochaine** ligne seulement, ou `None` s'il n'y en a plus.

Chaque ligne arrive sous forme de tuple.

```python
import sqlite3

conn = sqlite3.connect(":memory:")
conn.execute("CREATE TABLE amis (nom TEXT, age INTEGER)")
conn.executemany("INSERT INTO amis VALUES (?, ?)",
                 [("Alice", 30), ("Bob", 25), ("Chloé", 41)])

lignes = conn.execute("SELECT nom, age FROM amis ORDER BY age").fetchall()
for nom, age in lignes:
    print(nom, age)
```

```sortie
Bob 25
Alice 30
Chloé 41
```

`ORDER BY age` trie par âge. Sans `ORDER BY`, la base peut rendre les lignes dans
n'importe quel ordre : ajoutez-le dès que l'ordre compte.

### Filtrer avec WHERE

`WHERE` garde seulement les lignes qui passent un test. Là aussi, la valeur passe par un
`?`.

```python
import sqlite3

conn = sqlite3.connect(":memory:")
conn.execute("CREATE TABLE amis (nom TEXT, age INTEGER)")
conn.executemany("INSERT INTO amis VALUES (?, ?)",
                 [("Alice", 30), ("Bob", 25), ("Chloé", 41)])

curseur = conn.execute("SELECT nom FROM amis WHERE age >= ? ORDER BY nom", (30,))
print(curseur.fetchall())

curseur = conn.execute("SELECT age FROM amis WHERE nom = ?", ("Bob",))
print(curseur.fetchone())
print(curseur.fetchone())
```

```sortie
[('Alice',), ('Chloé',)]
(25,)
None
```

Le deuxième `fetchone()` renvoie `None` : il n'y avait qu'une seule ligne.

> **Attention :** `(30,)` avec une virgule est un tuple d'une valeur. `(30)` sans virgule
> est juste le nombre 30. Voyez [les erreurs fréquentes](#erreurs-frequentes).

## Des lignes avec des noms : sqlite3.Row

Avec un tuple, il faut se souvenir que le nom est à la place 0 et l'âge à la place 1.
C'est fragile. En réglant `row_factory`, chaque ligne devient un objet qu'on lit **par le
nom de la colonne**, comme un [dictionnaire](../debuter/glossaire.md#dictionnaire).

```python
import sqlite3

conn = sqlite3.connect(":memory:")
conn.row_factory = sqlite3.Row
conn.execute("CREATE TABLE amis (nom TEXT, age INTEGER)")
conn.execute("INSERT INTO amis VALUES (?, ?)", ("Alice", 30))

ligne = conn.execute("SELECT nom, age FROM amis").fetchone()
print(ligne["nom"])
print(ligne["age"])
print(ligne.keys())
print(dict(ligne))
```

```sortie
Alice
30
['nom', 'age']
{'nom': 'Alice', 'age': 30}
```

> **À retenir :** mettez `conn.row_factory = sqlite3.Row` juste après `connect`. Vos
> lignes se lisent alors avec `ligne["nom"]`, ce qui est bien plus clair que `ligne[0]`.

## Modifier et supprimer

`UPDATE` change des lignes, `DELETE` en supprime. Les deux prennent un `WHERE`.
`rowcount` dit combien de lignes ont été touchées.

```python
import sqlite3

conn = sqlite3.connect(":memory:")
conn.execute("CREATE TABLE amis (nom TEXT, age INTEGER)")
conn.executemany("INSERT INTO amis VALUES (?, ?)",
                 [("Alice", 30), ("Bob", 25), ("Chloé", 41)])

curseur = conn.execute("UPDATE amis SET age = ? WHERE nom = ?", (31, "Alice"))
print("modifiées :", curseur.rowcount)

curseur = conn.execute("DELETE FROM amis WHERE nom = ?", ("Bob",))
print("supprimées :", curseur.rowcount)

print(conn.execute("SELECT * FROM amis ORDER BY nom").fetchall())
```

```sortie
modifiées : 1
supprimées : 1
[('Alice', 31), ('Chloé', 41)]
```

Suivons la table à chaque étape :

| Étape | Requête | Contenu de la table |
|---|---|---|
| 1 | `INSERT` ×3 | Alice 30, Bob 25, Chloé 41 |
| 2 | `UPDATE … WHERE nom = 'Alice'` | Alice **31**, Bob 25, Chloé 41 |
| 3 | `DELETE … WHERE nom = 'Bob'` | Alice 31, Chloé 41 |

> **Attention :** un `UPDATE` ou un `DELETE` **sans** `WHERE` touche **toutes** les
> lignes. `DELETE FROM amis` vide le tiroir entier. Relisez toujours votre `WHERE`.

## Les transactions avec with conn:

Une **transaction**, c'est un paquet de changements qui réussit **en entier** ou pas du
tout. Exemple : un virement. On retire 50 € d'un compte et on les ajoute à l'autre. Si
le programme plante au milieu, l'argent ne doit pas disparaître.

`with conn:` s'occupe de tout :

- si le bloc se termine bien, les changements sont **validés** (`commit`) ;
- si une [exception](../debuter/glossaire.md#exception) arrive, ils sont **annulés**
  (`rollback`).

```python
import sqlite3

conn = sqlite3.connect(":memory:")
conn.execute("CREATE TABLE comptes (nom TEXT, solde INTEGER)")
conn.executemany("INSERT INTO comptes VALUES (?, ?)", [("Alice", 100), ("Bob", 20)])
conn.commit()

def virement(depuis, vers, montant):
    with conn:
        conn.execute("UPDATE comptes SET solde = solde - ? WHERE nom = ?", (montant, depuis))
        solde = conn.execute("SELECT solde FROM comptes WHERE nom = ?", (depuis,)).fetchone()[0]
        if solde < 0:
            raise ValueError("solde insuffisant")
        conn.execute("UPDATE comptes SET solde = solde + ? WHERE nom = ?", (montant, vers))

virement("Alice", "Bob", 50)
print(conn.execute("SELECT * FROM comptes ORDER BY nom").fetchall())

try:
    virement("Bob", "Alice", 500)
except ValueError as erreur:
    print("Refusé :", erreur)
print(conn.execute("SELECT * FROM comptes ORDER BY nom").fetchall())
```

```sortie
[('Alice', 50), ('Bob', 70)]
Refusé : solde insuffisant
[('Alice', 50), ('Bob', 70)]
```

Pour le deuxième virement, Bob est d'abord passé à `-430`. Puis l'erreur a annulé tout
le paquet : Bob est revenu à `70`.

```text
virement("Bob", "Alice", 500)
  BEGIN ─► Bob : 70 → -430 ─► solde < 0 ? oui ─► ValueError
                                                     │
                                       ROLLBACK ◄────┘   Bob revient à 70
```

> **À retenir :** sans `commit`, les changements faits dans un fichier sont perdus à la
> fermeture. Mettez vos écritures dans un `with conn:` : il valide ou annule pour vous.
> Le chapitre [les erreurs](erreurs.md) explique `try` et `except`.

## Se protéger des injections SQL

Imaginez une recherche par nom. On est tenté de coller le nom dans la requête avec une
f-string. Voici ce qui arrive si quelqu'un tape un nom un peu spécial :

```python
import sqlite3

conn = sqlite3.connect(":memory:")
conn.execute("CREATE TABLE amis (nom TEXT, secret TEXT)")
conn.executemany("INSERT INTO amis VALUES (?, ?)",
                 [("Alice", "chat"), ("Bob", "1234")])

saisie = "x' OR '1'='1"
requete = f"SELECT * FROM amis WHERE nom = '{saisie}'"
print(requete)
print(conn.execute(requete).fetchall())
```

```sortie
SELECT * FROM amis WHERE nom = 'x' OR '1'='1'
[('Alice', 'chat'), ('Bob', '1234')]
```

La saisie a **changé le sens** de la requête. `OR '1'='1'` est toujours vrai : la base
renvoie toutes les lignes, avec tous les secrets. C'est une **injection SQL**, une des
attaques les plus connues (voir [la sécurité](../debuter/securite.md)).

Avec `?`, la valeur reste une simple valeur. Elle n'est jamais lue comme du SQL :

```python
import sqlite3

conn = sqlite3.connect(":memory:")
conn.execute("CREATE TABLE amis (nom TEXT, secret TEXT)")
conn.executemany("INSERT INTO amis VALUES (?, ?)",
                 [("Alice", "chat"), ("Bob", "1234")])

saisie = "x' OR '1'='1"
print(conn.execute("SELECT * FROM amis WHERE nom = ?", (saisie,)).fetchall())
```

```sortie
[]
```

La base cherche quelqu'un qui s'appelle vraiment `x' OR '1'='1`. Personne : liste vide.

> **À retenir :** le `?` est une boîte fermée. Ce qu'on met dedans ne peut pas sortir
> pour devenir du code SQL.

## Un exemple complet : le carnet d'adresses

Mettons tout ensemble dans de petites [fonctions](fonctions.md).

```python
import sqlite3

def ouvrir():
    conn = sqlite3.connect(":memory:")
    conn.row_factory = sqlite3.Row
    conn.execute("""CREATE TABLE contacts (
        id INTEGER PRIMARY KEY,
        nom TEXT NOT NULL,
        telephone TEXT
    )""")
    return conn

def ajouter(conn, nom, telephone):
    with conn:
        conn.execute("INSERT INTO contacts (nom, telephone) VALUES (?, ?)", (nom, telephone))

def chercher(conn, debut):
    requete = "SELECT * FROM contacts WHERE nom LIKE ? ORDER BY nom"
    return conn.execute(requete, (debut + "%",)).fetchall()

def changer_telephone(conn, nom, telephone):
    with conn:
        conn.execute("UPDATE contacts SET telephone = ? WHERE nom = ?", (telephone, nom))

def afficher(conn):
    for c in conn.execute("SELECT * FROM contacts ORDER BY nom"):
        print(f"{c['id']}. {c['nom']} : {c['telephone']}")

carnet = ouvrir()
ajouter(carnet, "Alice Martin", "06 11 22 33 44")
ajouter(carnet, "Bob Durand", "07 55 66 77 88")
ajouter(carnet, "Alain Petit", "06 99 00 11 22")
afficher(carnet)

print("--- commence par Al ---")
for c in chercher(carnet, "Al"):
    print(c["nom"])

changer_telephone(carnet, "Bob Durand", "07 00 00 00 00")
print("--- après modification ---")
afficher(carnet)
```

```sortie
3. Alain Petit : 06 99 00 11 22
1. Alice Martin : 06 11 22 33 44
2. Bob Durand : 07 55 66 77 88
--- commence par Al ---
Alain Petit
Alice Martin
--- après modification ---
3. Alain Petit : 06 99 00 11 22
1. Alice Martin : 06 11 22 33 44
2. Bob Durand : 07 00 00 00 00
```

Quelques détails :

- les numéros `id` suivent l'ordre d'ajout, mais l'affichage suit l'ordre alphabétique
  grâce à `ORDER BY nom` ;
- `NOT NULL` interdit un nom vide ([`NULL`](../debuter/glossaire.md#null) est la valeur
  « rien » de SQL, comme `None` en Python) ;
- `LIKE ?` avec `"Al%"` cherche les noms qui **commencent** par `Al` : `%` veut dire
  « n'importe quelle suite de lettres » ;
- on peut parcourir le résultat de `execute` directement avec `for`, sans `fetchall`.

> **Essayez :** ajoutez une fonction `supprimer(conn, nom)` avec un
> `DELETE FROM contacts WHERE nom = ?` dans un `with conn:`. Supprimez Bob, puis
> affichez le carnet : il reste deux contacts.

## Erreurs fréquentes

### Oublier la virgule d'un tuple à une valeur

```python norun
import sqlite3
conn = sqlite3.connect(":memory:")
conn.execute("CREATE TABLE amis (nom TEXT)")
conn.execute("SELECT * FROM amis WHERE nom = ?", ("Alice"))
```

```text
Traceback (most recent call last):
  File "main.py", line 4, in <module>
    conn.execute("SELECT * FROM amis WHERE nom = ?", ("Alice"))
    ~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
sqlite3.ProgrammingError: Incorrect number of bindings supplied. The current statement uses 1, and there are 5 supplied.
```

`("Alice")` sans virgule n'est pas un tuple : c'est le texte `"Alice"`. Python le voit
comme 5 lettres, donc 5 valeurs pour 1 seul `?`. Écrivez `("Alice",)` avec une virgule.

### Lire une table qui n'existe pas

```python norun
import sqlite3
conn = sqlite3.connect(":memory:")
conn.execute("SELECT * FROM amis")
```

```text
Traceback (most recent call last):
  File "main.py", line 3, in <module>
    conn.execute("SELECT * FROM amis")
    ~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^
sqlite3.OperationalError: no such table: amis
```

La table n'a pas été créée, ou son nom est mal écrit. Une base `":memory:"` est vide à
chaque lancement : créez la table d'abord. Pour un fichier, `CREATE TABLE IF NOT EXISTS`
ne crée la table que si elle manque.

### Lire une colonne par son nom sans sqlite3.Row

```python norun
import sqlite3
conn = sqlite3.connect(":memory:")
conn.execute("CREATE TABLE amis (nom TEXT, age INTEGER)")
conn.execute("INSERT INTO amis VALUES (?, ?)", ("Alice", 30))
ligne = conn.execute("SELECT nom, age FROM amis").fetchone()
print(ligne["nom"])
```

```text
Traceback (most recent call last):
  File "main.py", line 6, in <module>
    print(ligne["nom"])
          ~~~~~^^^^^^^
TypeError: tuple indices must be integers or slices, not str
```

Par défaut, une ligne est un tuple : on la lit avec `ligne[0]`. Pour écrire
`ligne["nom"]`, ajoutez `conn.row_factory = sqlite3.Row` après `connect`.

## Testez-vous

**Question 1 :** pourquoi ne faut-il jamais construire une requête avec une f-string ?

> **Réponse**
>
> Parce qu'une valeur tapée par quelqu'un peut contenir du SQL et changer le sens de la
> requête : c'est une injection SQL. Avec `?`, la valeur reste toujours une simple
> valeur.

**Question 2 :** qu'affiche ce programme ?

```python norun
import sqlite3
conn = sqlite3.connect(":memory:")
conn.execute("CREATE TABLE notes (valeur INTEGER)")
conn.executemany("INSERT INTO notes VALUES (?)", [(12,), (8,), (15,)])
print(conn.execute("SELECT valeur FROM notes WHERE valeur >= ? ORDER BY valeur", (10,)).fetchall())
```

> **Réponse**
>
> ```python
> import sqlite3
> conn = sqlite3.connect(":memory:")
> conn.execute("CREATE TABLE notes (valeur INTEGER)")
> conn.executemany("INSERT INTO notes VALUES (?)", [(12,), (8,), (15,)])
> print(conn.execute("SELECT valeur FROM notes WHERE valeur >= ? ORDER BY valeur", (10,)).fetchall())
> ```
>
> ```sortie
> [(12,), (15,)]
> ```
>
> `WHERE` garde 12 et 15, `ORDER BY` les trie. Chaque ligne est un tuple d'une valeur.

**Question 3 :** dans un `with conn:`, que se passe-t-il si une exception arrive au
milieu ?

> **Réponse**
>
> Tous les changements faits depuis le début du bloc sont annulés (`rollback`). La base
> revient comme avant le bloc, puis l'exception continue son chemin.

## En résumé

- `sqlite3.connect(":memory:")` ouvre une base en mémoire ; un nom de fichier la garde
  sur le disque.
- `conn.execute(requete, valeurs)` envoie du SQL ; `executemany` répète pour une liste.
- `SELECT` puis `fetchall()` ou `fetchone()` pour lire ; `UPDATE` et `DELETE` avec un
  `WHERE` pour modifier et supprimer.
- `conn.row_factory = sqlite3.Row` permet d'écrire `ligne["nom"]`.
- `with conn:` valide les changements, ou les annule tous en cas d'erreur.
- Toujours `?` pour les valeurs, jamais de f-string : c'est ce qui protège des
  injections SQL.

Suite : [Analyser des données](donnees.md)
