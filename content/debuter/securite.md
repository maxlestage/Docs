# Les bases de la sécurité

Un programme doit protéger les données qu'on lui confie : cette page vous apprend les bons réflexes pour les mots de passe, les secrets, la base de données et tout ce que tape l'utilisateur.

> **Objectifs :**
>
> - choisir un bon mot de passe et ne jamais le garder en clair ;
> - calculer une empreinte avec `hashlib` et un sel ;
> - garder les secrets hors du code, dans des variables d'environnement ;
> - empêcher l'injection SQL avec des `?` ;
> - vérifier tout ce que tape l'utilisateur.

## Pourquoi c'est votre affaire

Vous fermez votre porte à clé, même si personne ne veut vous cambrioler.
En programmation, c'est pareil. Des robots essaient chaque jour des
millions de mots de passe sur tous les sites, même les tout petits.

La bonne nouvelle : quelques réflexes simples évitent presque tous les
problèmes. Vous allez les voir un par un.

## Un bon mot de passe

Un mot de passe court se devine vite, même s'il est compliqué. Un
ordinateur essaie toutes les combinaisons : plus il y en a, plus c'est long.
Comparons :

```python
lettres = 26 ** 8          # 8 lettres minuscules
phrase = 2000 ** 5         # 5 mots pris parmi 2000 mots courants
print(f"8 lettres : {lettres:,} combinaisons")
print(f"5 mots    : {phrase:,} combinaisons")
print(f"La phrase est {phrase // lettres:,} fois plus dure à deviner.")
```

```sortie
8 lettres : 208,827,064,576 combinaisons
5 mots    : 32,000,000,000,000,000 combinaisons
La phrase est 153,236 fois plus dure à deviner.
```

`**` veut dire « puissance » : `26 ** 8` est 26 multiplié 8 fois par
lui-même. Et `:,` sépare les milliers pour lire plus facilement.

Retenez donc :

- une **phrase** de plusieurs mots (`cheval-lampe-orage-tulipe-violon`)
  est plus solide et plus facile à retenir que `Xk7$p2!a` ;
- **un mot de passe différent par site** : si un site se fait voler, les
  autres restent protégés ;
- un **gestionnaire de mots de passe** (Bitwarden, KeePassXC…) les retient
  pour vous. Vous ne retenez plus qu'une seule phrase : la sienne.

Essayez : remplacez `2000 ** 5` par `2000 ** 3` (3 mots). La phrase reste
plus solide que 8 lettres, mais beaucoup moins qu'avec 5 mots.

> **À retenir :** longueur > complication. Une longue phrase bat un mot
> court et bizarre.

## Ne jamais garder un mot de passe en clair

Votre programme gère des comptes ? Il ne doit **jamais** enregistrer le
mot de passe lui-même. Si la base de données est volée, tous les mots de
passe seraient lisibles.

On garde à la place une **empreinte** (en anglais *hash*). C'est comme une
empreinte digitale : elle reconnaît une personne, mais on ne peut pas
refabriquer la personne à partir de l'empreinte.

### Une empreinte avec hashlib

Python sait calculer une empreinte avec le [module](glossaire.md#module)
`hashlib` :

```python
import hashlib

for mot in ["soleil", "soleil", "Soleil"]:
    empreinte = hashlib.sha256(mot.encode()).hexdigest()
    print(f"{mot:7} → {empreinte[:16]}…")
```

```sortie
soleil  → 1ca8dbc7f6e4a26d…
soleil  → 1ca8dbc7f6e4a26d…
Soleil  → 18b9aec0c0d8c6e4…
```

Trois choses à voir :

| Mot      | Empreinte (début)  | Ce qu'on remarque                         |
|----------|--------------------|-------------------------------------------|
| `soleil` | `1ca8dbc7f6e4a26d` | toujours la même pour le même mot         |
| `soleil` | `1ca8dbc7f6e4a26d` | identique : on peut donc **comparer**     |
| `Soleil` | `18b9aec0c0d8c6e4` | une seule lettre change, tout change      |

`.encode()` transforme le texte en [octets](glossaire.md#octet) : `hashlib`
ne travaille que sur des octets. Et `[:16]` ne montre que le début, pour
que ça tienne sur la ligne.

### Ajouter un sel

Un problème demeure : deux personnes qui ont `soleil` ont la même
empreinte. Et les pirates ont des listes d'empreintes toutes prêtes.

La solution : un **sel**. C'est un petit texte ajouté au mot de passe,
différent pour chaque personne. On le garde à côté de l'empreinte. Et on
utilise une fonction faite exprès, lente à calculer pour décourager les
pirates : `pbkdf2_hmac`.

```python
import hashlib
import hmac

def empreinte(mot_de_passe, sel):
    return hashlib.pbkdf2_hmac("sha256", mot_de_passe.encode(), sel, 100_000)

# En vrai, le sel est tiré au hasard : sel = os.urandom(16)
# Ici, il est fixe pour que le résultat soit toujours le même.
sel = b"sel-de-alice"
gardee = empreinte("cheval-lampe-orage", sel)
print("Gardé dans la base :", gardee.hex()[:16], "…")

def verifier(essai):
    return hmac.compare_digest(empreinte(essai, sel), gardee)

print("cheval-lampe-orage :", verifier("cheval-lampe-orage"))
print("cheval-lampe       :", verifier("cheval-lampe"))
```

```sortie
Gardé dans la base : 37a1b9a5a2a72e2a …
cheval-lampe-orage : True
cheval-lampe       : False
```

Suivons la vérification pas à pas :

| Étape | Ce que fait le programme                          | Résultat     |
|-------|---------------------------------------------------|--------------|
| 1     | la personne tape son mot de passe                 | `essai`      |
| 2     | on calcule l'empreinte de `essai` avec le même sel | une empreinte |
| 3     | on la compare à l'empreinte gardée                 | `True`/`False` |

Le mot de passe en clair n'est **jamais** enregistré. `hmac.compare_digest`
compare deux empreintes d'une manière qu'un pirate ne peut pas mesurer au
chronomètre : utilisez-le plutôt que `==`.

Essayez : changez le sel en `b"sel-de-bob"`. L'empreinte gardée change
complètement, mais la vérification marche toujours.

> **À retenir :** on garde une empreinte salée, jamais le mot de passe.

## Les secrets hors du code

Un **secret**, c'est un mot de passe de base de données, une clé
d'[API](glossaire.md#api)… Ne l'écrivez jamais dans votre code : le code
finit sur [GitHub](git.md), chez un collègue, dans une capture d'écran.

On le met dans une **variable d'environnement** : une étiquette que le
[système](glossaire.md#systeme-d-exploitation) donne au programme quand il
démarre. Le code lit l'étiquette, sans connaître la valeur à l'avance.

```python
import os

cle = os.environ.get("CLE_METEO")
if cle is None:
    print("Erreur : la variable CLE_METEO n'est pas définie.")
else:
    print("Clé trouvée, elle fait", len(cle), "caractères.")
```

```sortie
Erreur : la variable CLE_METEO n'est pas définie.
```

Ici, la variable n'existe pas : le programme le dit clairement. Pour la
définir, dans le [terminal](terminal.md), avant de lancer le programme :

```sh
export CLE_METEO="ma-cle-secrete"      # macOS et Linux
set CLE_METEO=ma-cle-secrete           # Windows (invite de commandes)
python3 meteo.py
```

```text
Clé trouvée, elle fait 14 caractères.
```

Beaucoup de projets rangent ces variables dans un fichier `.env`. Ce
fichier ne doit **jamais** aller dans Git : écrivez `.env` dans votre
[`.gitignore`](git.md#ignorer-des-fichiers-gitignore).

> **À retenir :** le code dit **où** trouver le secret, jamais **quel** est
> le secret.

## L'injection SQL

Vous avez vu, dans [les bases de données](bases-de-donnees.md), qu'il ne
faut pas fabriquer un ordre SQL en collant du texte. Voici pourquoi, avec
une vraie attaque.

### Le code vulnérable

Un site vérifie un nom d'utilisateur en collant ce qu'on tape dans l'ordre
SQL. Un pirate tape un texte bizarre à la place d'un nom :

```python
import sqlite3

con = sqlite3.connect(":memory:")
con.execute("CREATE TABLE comptes (nom TEXT, secret TEXT)")
con.executemany("INSERT INTO comptes VALUES (?, ?)",
                [("alice", "code 1234"), ("bob", "code 9876")])

saisie = "x' OR '1'='1"
requete = "SELECT nom, secret FROM comptes WHERE nom = '" + saisie + "'"
print(requete)
print(con.execute(requete).fetchall())
```

```sortie
SELECT nom, secret FROM comptes WHERE nom = 'x' OR '1'='1'
[('alice', 'code 1234'), ('bob', 'code 9876')]
```

Regardez l'ordre fabriqué. Le pirate a fermé l'apostrophe, puis ajouté
`OR '1'='1'`, qui est **toujours vrai**. La base renvoie donc **tous** les
comptes, avec leurs secrets.

```text
WHERE nom = 'x' OR '1'='1'
      └──┬────┘    └──┬──┘
       faux        toujours vrai   →  faux OU vrai = vrai pour chaque ligne
```

### La correction : les `?`

Avec un `?`, la base reçoit l'ordre et la saisie **séparément**. La saisie
reste un simple texte, jamais du SQL :

```python
import sqlite3

con = sqlite3.connect(":memory:")
con.execute("CREATE TABLE comptes (nom TEXT, secret TEXT)")
con.executemany("INSERT INTO comptes VALUES (?, ?)",
                [("alice", "code 1234"), ("bob", "code 9876")])

saisie = "x' OR '1'='1"
resultat = con.execute("SELECT nom, secret FROM comptes WHERE nom = ?", (saisie,))
print(resultat.fetchall())
```

```sortie
[]
```

La base cherche un utilisateur qui s'appelle vraiment `x' OR '1'='1` : il
n'y en a pas. Liste vide, attaque ratée.

Essayez : remplacez `saisie` par `"alice"`. Vous obtenez
`[('alice', 'code 1234')]`.

> **À retenir :** jamais de `+` ni de f-string pour fabriquer du SQL.
> Toujours des `?`.

## Ne jamais faire confiance à ce qu'on vous envoie

Tout ce qui vient de l'extérieur peut être faux, par erreur ou exprès : un
formulaire, un fichier, une URL. Vérifiez-le **avant** de vous en servir.
C'est le videur à l'entrée d'une salle : il regarde chaque billet.

```python
def lire_age(texte):
    texte = texte.strip()
    if not texte.isdigit():
        return "refusé : ce n'est pas un nombre"
    age = int(texte)
    if age > 130:
        return "refusé : âge impossible"
    return f"accepté : {age} ans"

for saisie in ["42", " 7 ", "-3", "douze", "999", "'; DROP TABLE"]:
    print(f"{saisie!r:16} {lire_age(saisie)}")
```

```sortie
'42'             accepté : 42 ans
' 7 '            accepté : 7 ans
'-3'             refusé : ce n'est pas un nombre
'douze'          refusé : ce n'est pas un nombre
'999'            refusé : âge impossible
"'; DROP TABLE"  refusé : ce n'est pas un nombre
```

`strip()` enlève les espaces autour, `isdigit()` vérifie qu'il n'y a que
des chiffres. `!r` affiche le texte avec ses guillemets, pour voir les
espaces.

Essayez : ajoutez `"130"` à la liste. Il est accepté : la limite est
« plus de 130 ».

> **À retenir :** on vérifie que la donnée a la bonne **forme** (un
> nombre ?) et une valeur **raisonnable** (entre 0 et 130 ?).

## HTTPS en deux phrases

`https://` (avec un **s**) chiffre ce qui circule entre le navigateur et le
[serveur](glossaire.md#serveur) : personne sur le chemin ne peut lire un
mot de passe ou un numéro de carte. N'envoyez jamais de données
personnelles vers une adresse en `http://` sans **s**.

## Les mises à jour

Des failles sont trouvées chaque semaine dans les logiciels. Les mises à
jour les corrigent. Mettez donc à jour votre système, votre navigateur,
votre langage et les [paquets](glossaire.md#paquet) de vos projets. Un
paquet abandonné depuis des années est un risque : préférez-en un qui vit.

## Les 5 règles à retenir

```text
1. Une longue phrase de passe, différente par site, dans un gestionnaire.
2. Jamais de mot de passe en clair : une empreinte salée.
3. Jamais de secret dans le code ni dans Git : des variables d'environnement.
4. Jamais de SQL fabriqué avec + : toujours des ?.
5. Jamais confiance dans une saisie : on vérifie, puis on met à jour.
```

## Erreurs fréquentes

### Un secret envoyé dans Git

Vous écrivez une clé dans `config.py`, vous faites un commit, puis vous
supprimez le fichier dans un second commit. Vous pensez être tranquille :

```sh
git show HEAD~1:config.py
```

```text
CLE_API = "sk-secret-123456"
```

`git show HEAD~1:config.py` veut dire « montre `config.py` tel qu'il était
au commit d'avant ». La clé est toujours là, pour tous ceux qui ont accès
au dépôt. La correction : considérez la clé comme **volée**. Allez sur le
site qui l'a fournie, **révoquez-la** (annulez-la) et créez-en une nouvelle,
rangée cette fois dans une variable d'environnement.

### Comparer le mot de passe tapé à l'empreinte

On garde bien une empreinte… mais on la compare au texte tapé :

```python
import hashlib

gardee = hashlib.sha256("soleil".encode()).hexdigest()
essai = "soleil"

if essai == gardee:
    print("Bienvenue !")
else:
    print("Mot de passe incorrect.")
```

```sortie
Mot de passe incorrect.
```

Pas d'erreur, mais le bon mot de passe est refusé : on compare `soleil` à
`1ca8dbc7…`. Il faut d'abord calculer l'empreinte de l'essai, avec le même
sel, puis comparer les deux empreintes, comme dans `verifier` plus haut.

### Oublier .encode()

```python norun
import hashlib

empreinte = hashlib.sha256("soleil")
```

```text
Traceback (most recent call last):
  File "main.py", line 3, in <module>
    empreinte = hashlib.sha256("soleil")
TypeError: Strings must be encoded before hashing
```

`hashlib` veut des octets, pas du texte. Écrivez
`hashlib.sha256("soleil".encode())`.

## Testez-vous

**1.** Pourquoi garder une empreinte plutôt que le mot de passe ?

> **Réponse**
>
> Si la base est volée, le pirate n'a que des empreintes : il ne peut pas
> retrouver les mots de passe à partir d'elles.

**2.** Une clé d'API est dans votre code, déjà envoyée sur GitHub. Suffit-il
de l'effacer et de refaire un commit ?

> **Réponse**
>
> Non : elle reste dans l'historique. Il faut la révoquer, en créer une
> nouvelle et la ranger dans une variable d'environnement.

**3.** Quel est le danger de
`"SELECT * FROM comptes WHERE nom = '" + saisie + "'"` ?

> **Réponse**
>
> L'injection SQL : la saisie peut contenir du SQL (`x' OR '1'='1`) qui
> change le sens de l'ordre. Il faut écrire `WHERE nom = ?` et passer
> `(saisie,)` à part.

## En résumé

- Une longue phrase de passe, unique par site, rangée dans un gestionnaire.
- On garde une empreinte salée (`hashlib.pbkdf2_hmac`), jamais le mot de passe.
- Les secrets vont dans des variables d'environnement, jamais dans le code ni dans Git.
- Les `?` empêchent l'injection SQL ; on vérifie chaque saisie avant de s'en servir.
- `https://` chiffre les échanges ; les mises à jour bouchent les failles.

Suite : [Bien apprendre la programmation](apprendre.md).
