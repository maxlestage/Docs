# Travailler avec du texte

Cette page explique comment manipuler du texte en Python : découper, chercher, remplacer, mettre en forme et comprendre les caractères Unicode.

> **Objectifs :**
>
> - vous saurez lire une lettre ou un morceau de texte avec les indices et les tranches ;
> - vous saurez transformer un texte avec `upper`, `strip`, `replace`... ;
> - vous saurez découper un texte avec `split` et le recoller avec `join` ;
> - vous saurez chercher dans un texte avec `in`, `find` et `count` ;
> - vous saurez aligner du texte dans une f-string et écrire des caractères spéciaux.

## Le type str

Presque tous les programmes manipulent du texte : un prénom, un message, une adresse
e-mail, une ligne lue dans un fichier. Savoir le ranger et le transformer est donc
indispensable.

Un texte en Python est une [chaîne de caractères](../debuter/glossaire.md#chaine-de-caracteres),
de type `str`. Imaginez un collier de perles : chaque perle est un caractère (une
lettre, un chiffre, un espace, un symbole), et les perles sont dans un ordre précis.

```python
message = "Bonjour"
autre = 'Salut'
citation = "C'est l'heure"
print(message, autre, citation)
print(len(message))
```

```sortie
Bonjour Salut C'est l'heure
7
```

Ligne par ligne :

- `message`, `autre` et `citation` reçoivent chacun un texte, écrit entre guillemets ;
- le premier `print` affiche les trois textes à la suite, séparés par un espace ;
- `len(message)` compte les caractères de « Bonjour » : il y en a 7.

- Guillemets doubles ou simples, au choix. Prenez les doubles si le texte contient une
  apostrophe.
- `len` (*length*, longueur) donne le nombre de caractères.

On colle deux textes avec `+` et on répète un texte avec `*` :

```python
print("Bon" + "jour")
print("ha" * 3)
print("-" * 20)
```

```sortie
Bonjour
hahaha
--------------------
```

## Lire un caractère : les indices

Vous avez parfois besoin d'un seul caractère : l'initiale d'un prénom, le dernier
chiffre d'un code. Les indices servent à cela, comme les numéros des cases d'un casier.

Chaque caractère a une position, son [indice](../debuter/glossaire.md#indice). Le
premier est à l'indice **0**. Les indices négatifs comptent depuis la fin : `-1` est le
dernier.

```text
 P   y   t   h   o   n
 0   1   2   3   4   5
-6  -5  -4  -3  -2  -1
```

```python
mot = "Python"
print(mot[0])
print(mot[3])
print(mot[-1])
print(mot[-2])
```

```sortie
P
h
n
o
```

**À vous :** écrivez un programme qui range `prenom = "Camille"` dans une variable,
puis affiche sa première lettre et sa dernière lettre.

> **Solution**
>
> ```python
> prenom = "Camille"
> print(prenom[0])
> print(prenom[-1])
> ```
>
> ```sortie
> C
> e
> ```

## Découper : les tranches

Un seul caractère ne suffit pas toujours : vous voulez parfois les trois premières
lettres d'un mot, ou l'année à la fin d'une date. Une tranche découpe un morceau, comme
une part dans un gâteau.

Une **tranche** (*slice*) prend un morceau du texte : `texte[début:fin]`. Le début est
inclus, la fin est **exclue**, comme pour `range`.

```python
mot = "Python"
print(mot[0:3])
print(mot[2:])
print(mot[:4])
print(mot[-3:])
print(mot[::2])
print(mot[::-1])
```

```sortie
Pyt
thon
Pyth
hon
Pto
nohtyP
```

Ligne par ligne :

- `mot[0:3]` prend les indices 0, 1 et 2 : « Pyt ». L'indice 3 est exclu ;
- `mot[2:]` part de l'indice 2 et va jusqu'au bout : « thon » ;
- `mot[-3:]` part du troisième caractère avant la fin : « hon » ;
- `mot[::-1]` lit le mot de la fin vers le début : « nohtyP ».

- sans début, on part du début ; sans fin, on va jusqu'au bout ;
- un troisième nombre donne le **pas** : `[::2]` prend une lettre sur deux ;
- `[::-1]` parcourt à l'envers : c'est l'astuce classique pour retourner un texte.

## Un texte ne se modifie pas

Cette règle explique une surprise fréquente chez les débutants : vous appelez une
méthode sur un texte, et pourtant votre variable ne change pas. Voici pourquoi.

Un `str` est [immuable](../debuter/glossaire.md#immuable) : une fois créé, il ne
change plus. Toutes les opérations sur un texte renvoient un **nouveau** texte.
L'original reste intact.

```python
nom = "alice"
majuscule = nom.upper()
print(nom)
print(majuscule)
nom = nom.capitalize()
print(nom)
```

```sortie
alice
ALICE
Alice
```

Ligne par ligne :

- `nom` contient « alice » ;
- `nom.upper()` fabrique un nouveau texte, « ALICE », rangé dans `majuscule`. `nom` n'a
  pas bougé : le premier `print` affiche toujours « alice » ;
- `nom = nom.capitalize()` fabrique « Alice » et le range dans `nom`, à la place de
  l'ancien texte : le dernier `print` affiche « Alice ».

Pour « changer » une variable texte, rangez le nouveau texte dans la variable :
`nom = nom.capitalize()`.

## Changer la casse et nettoyer

Les textes tapés par des personnes sont rarement propres : des majuscules au hasard,
des espaces en trop. Avant de les comparer ou de les afficher, on les remet en ordre,
comme on range un tiroir en désordre.

Une [méthode](../debuter/glossaire.md#methode) est une fonction attachée à une valeur.
On l'appelle avec un point : `texte.methode()`. Voici les plus utiles :

```python
titre = "le petit prince"
print(titre.upper())
print(titre.title())
print(titre.capitalize())
print("BONJOUR".lower())

saisie = "   alice@exemple.fr  \n"
print(f"[{saisie.strip()}]")
print(f"[{saisie.lstrip()}]".replace("\n", "↵"))
print("---titre---".strip("-"))
```

```sortie
LE PETIT PRINCE
Le Petit Prince
Le petit prince
bonjour
[alice@exemple.fr]
[alice@exemple.fr  ↵]
titre
```

Ligne par ligne :

- `titre` ne change jamais : chaque méthode renvoie un nouveau texte, aussitôt affiché ;
- `saisie` contient trois espaces au début, deux à la fin et un retour à la ligne `\n` ;
- `strip()` enlève tout cela des deux côtés. Les crochets montrent où le texte commence
  et où il finit ;
- `lstrip()` n'enlève que le début. Le `replace` change le retour à la ligne en `↵`,
  pour qu'on le voie.

- `upper` et `lower` : tout en majuscules, tout en minuscules ;
- `title` : une majuscule à chaque mot ; `capitalize` : seulement à la première lettre ;
- `strip` enlève les espaces et retours à la ligne aux deux bouts ; `lstrip` et
  `rstrip` d'un seul côté. Avec un argument, il enlève ces caractères-là.

> **Astuce :** pensez à `strip()` dès que vous lisez un texte tapé par un utilisateur
> ou lu dans un fichier : il y a souvent des espaces en trop.

**À vous :** écrivez un programme qui range `saisie = "  bOB  "` dans une variable, puis
affiche le prénom sans les espaces, avec une majuscule seulement au début : « Bob ».

> **Solution**
>
> ```python
> saisie = "  bOB  "
> prenom = saisie.strip().capitalize()
> print(prenom)
> ```
>
> ```sortie
> Bob
> ```

## Découper et recoller : split et join

Un texte contient souvent plusieurs informations collées : les mots d'une phrase, ou
les champs d'une ligne de tableur séparés par des `;`. `split` les sépare pour les
traiter une par une, et `join` les rassemble.

`split` coupe un texte en morceaux et renvoie une [liste](../debuter/glossaire.md#liste).
Sans argument, il coupe aux espaces.

```python
phrase = "le chat dort sur le canapé"
mots = phrase.split()
print(mots)
print(len(mots))

ligne = "Alice;30;Lyon"
print(ligne.split(";"))
```

```sortie
['le', 'chat', 'dort', 'sur', 'le', 'canapé']
6
['Alice', '30', 'Lyon']
```

Ligne par ligne :

- `phrase.split()` coupe à chaque espace et donne une liste de 6 mots, rangée dans
  `mots` ;
- `len(mots)` compte les éléments de la liste, pas les lettres : 6 ;
- `ligne.split(";")` coupe à chaque `;` : le prénom, l'âge et la ville sont séparés.
  Remarquez que `'30'` reste un texte.

`join` fait l'inverse : il colle les morceaux d'une liste, avec un séparateur. On
l'appelle **sur le séparateur**.

```python
mots = ["un", "deux", "trois"]
print(" ".join(mots))
print(", ".join(mots))
print("-".join("abc"))
```

```sortie
un deux trois
un, deux, trois
a-b-c
```

> **À retenir :** `texte.split(sep)` donne une liste ; `sep.join(liste)` redonne un
> texte.

**À vous :** écrivez un programme qui range `date = "19/10/2026"` dans une variable, la
découpe aux `/`, affiche la liste obtenue, puis recolle les morceaux avec des tirets.

> **Solution**
>
> ```python
> date = "19/10/2026"
> morceaux = date.split("/")
> print(morceaux)
> print("-".join(morceaux))
> ```
>
> ```sortie
> ['19', '10', '2026']
> 19-10-2026
> ```

## Remplacer

Corriger un mot partout dans un texte, enlever les espaces d'un nombre : c'est le
« rechercher et remplacer » d'un traitement de texte, en une seule ligne.

`replace(ancien, nouveau)` remplace toutes les occurrences d'un morceau de texte.

```python
phrase = "J'aime le thé. Le thé est chaud."
print(phrase.replace("thé", "café"))
print(phrase.replace("thé", "café", 1))
print("1 000 000".replace(" ", ""))
```

```sortie
J'aime le café. Le café est chaud.
J'aime le café. Le thé est chaud.
1000000
```

Le troisième argument limite le nombre de remplacements.

## Chercher dans un texte

Avant d'utiliser un texte, on veut souvent le vérifier : une adresse e-mail
contient-elle un `@` ? Un nom de fichier finit-il par `.txt` ? Python offre plusieurs
façons de poser ces questions.

```python
email = "alice@exemple.fr"
print("@" in email)
print("bob" in email)
print(email.startswith("alice"))
print(email.endswith(".fr"))
print(email.find("@"))
print(email.find("#"))
print("banana".count("a"))
```

```sortie
True
False
True
True
5
-1
3
```

- `in` répond `True` ou `False` : « ce morceau est-il dedans ? » ;
- `startswith` et `endswith` testent le début et la fin ;
- `find` donne l'indice de la première occurrence, ou `-1` si absent ;
- `count` compte les occurrences.

Toutes ces recherches distinguent majuscules et minuscules. Pour les ignorer, passez
tout en minuscules avant :

```python
reponse = "OUI"
print(reponse.lower() == "oui")
```

```sortie
True
```

Quelques méthodes testent la nature du texte : `isdigit()` (que des chiffres),
`isalpha()` (que des lettres), `isspace()` (que des espaces).

```python
print("2026".isdigit(), "12a".isdigit(), "Été".isalpha())
```

```sortie
True False True
```

**À vous :** écrivez un programme qui range `fichier = "photo_vacances.jpg"` dans une
variable, affiche si son nom finit par `.jpg`, puis combien de fois il contient la
lettre `a`.

> **Solution**
>
> ```python
> fichier = "photo_vacances.jpg"
> print(fichier.endswith(".jpg"))
> print(fichier.count("a"))
> ```
>
> ```sortie
> True
> 2
> ```

## Aligner du texte

Pour afficher un ticket de caisse ou un tableau, les colonnes doivent tomber les unes
sous les autres. Sans alignement, chaque ligne a sa propre longueur et tout est décalé.

Dans une f-string, `:` suivi d'un alignement et d'une largeur place le texte dans une
colonne :

- `<` à gauche, `>` à droite, `^` au centre ;
- un caractère avant le signe sert de remplissage.

```python
produits = [("Pain", 1.2), ("Fromage", 4.5), ("Confiture", 3.75)]
for nom, prix in produits:
    print(f"{nom:<12}{prix:>6.2f} €")
print(f"{'FIN':*^18}")
```

```sortie
Pain          1.20 €
Fromage       4.50 €
Confiture     3.75 €
*******FIN********
```

Ligne par ligne :

- `produits` est une liste de paires : un nom et un prix ;
- à chaque tour, la boucle range le nom dans `nom` et le prix dans `prix` ;
- le `print` place le nom dans une colonne de 12 caractères, puis le prix dans une
  colonne de 6 ;
- la dernière ligne centre « FIN » sur 18 caractères et remplit le reste avec des `*`.

`{nom:<12}` réserve 12 caractères, texte à gauche. `{prix:>6.2f}` réserve 6
caractères, nombre à droite avec 2 décimales. Les formats de nombres sont détaillés
dans [nombres et calculs](nombres.md).

## Les textes sur plusieurs lignes

Un poème, une adresse postale, un long message d'aide : certains textes ont
naturellement plusieurs lignes. Les triples guillemets évitent de tout écrire sur une
seule ligne interminable.

Entre **triples** guillemets `"""..."""`, un texte peut s'étendre sur plusieurs lignes.
Les retours à la ligne sont gardés.

```python
poeme = """Le ciel est, par-dessus le toit,
Si bleu, si calme !"""
print(poeme)
print(len(poeme.splitlines()))
```

```sortie
Le ciel est, par-dessus le toit,
Si bleu, si calme !
2
```

`splitlines()` découpe le texte en une liste de lignes.

## Les caractères spéciaux

Comment mettre un retour à la ligne, ou un guillemet, dans un texte qui est lui-même
entre guillemets ? Il faut une façon spéciale de les écrire.

Certains caractères s'écrivent avec une barre oblique inverse `\`, suivie d'une lettre :
c'est une **séquence d'échappement**.

| Séquence | Sens |
| --- | --- |
| `\n` | retour à la ligne |
| `\t` | tabulation |
| `\"` et `\'` | un guillemet dans le texte |
| `\\` | une barre oblique inverse |

```python
print("Ligne 1\nLigne 2")
print("Nom\tÂge")
print("Il a dit : \"Bonjour\".")
print("C:\\dossier\\fichier.txt")
print(r"C:\dossier\nouveau")
```

```sortie
Ligne 1
Ligne 2
Nom	Âge
Il a dit : "Bonjour".
C:\dossier\fichier.txt
C:\dossier\nouveau
```

Ligne par ligne :

- `\n` coupe la ligne : « Ligne 2 » s'affiche en dessous ;
- `\t` insère une tabulation, un grand espace qui aide à aligner ;
- `\"` met un guillemet dans le texte sans le fermer, et `\\` affiche une seule barre.

Le `r` devant le dernier texte en fait une chaîne **brute** (*raw*) : les `\` y restent
tels quels. Pratique pour les chemins Windows et les
[expressions régulières](regex.md).

## Unicode : ord et chr

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

Un ordinateur ne stocke que des nombres. [Unicode](../debuter/glossaire.md#unicode)
donne un numéro à chaque caractère du monde : lettres accentuées, alphabets, emojis.
`ord` donne le numéro d'un caractère, `chr` fait l'inverse.

```python
print(ord("A"), ord("a"), ord("é"), ord("€"))
print(chr(65), chr(233), chr(8364))
print(len("été"), len("😀"))
```

```sortie
65 97 233 8364
A é €
3 1
```

En Python, un caractère compte pour 1, même un emoji. Les textes Python sont en Unicode
dès le départ : vous pouvez écrire en français, en grec ou en japonais sans rien
configurer.

Avec `ord` et `chr`, on peut décaler les lettres. Voici le chiffre de César, qui
décale chaque lettre de 3 rangs :

```python
def cesar(texte, decalage):
    resultat = ""
    for lettre in texte:
        if "a" <= lettre <= "z":
            position = (ord(lettre) - ord("a") + decalage) % 26
            resultat += chr(ord("a") + position)
        else:
            resultat += lettre
    return resultat

secret = cesar("bonjour zoe", 3)
print(secret)
print(cesar(secret, -3))
```

```sortie
erqmrxu crh
bonjour zoe
```

## Erreurs fréquentes

### Modifier une lettre d'un texte

```python norun
mot = "chat"
mot[0] = "C"
```

```text
Traceback (most recent call last):
  File "/home/vous/main.py", line 2, in <module>
    mot[0] = "C"
    ~~~^^^
TypeError: 'str' object does not support item assignment
```

Un texte est immuable. Construisez un nouveau texte : `mot = "C" + mot[1:]`, ou
`mot = mot.capitalize()`.

### Un indice trop grand

```python norun
mot = "chat"
print(mot[10])
```

```text
Traceback (most recent call last):
  File "/home/vous/main.py", line 2, in <module>
    print(mot[10])
          ~~~^^^^
IndexError: string index out of range
```

`"chat"` a 4 lettres : les indices vont de 0 à 3 (*out of range* : hors limites). Le
dernier caractère est `mot[-1]` ou `mot[len(mot) - 1]`.

### Appeler une méthode de texte sur une liste

```python norun
texte = "a,b"
print(texte.split(",").upper())
```

```text
Traceback (most recent call last):
  File "/home/vous/main.py", line 2, in <module>
    print(texte.split(",").upper())
          ^^^^^^^^^^^^^^^^^^^^^^
AttributeError: 'list' object has no attribute 'upper'
```

`split` renvoie une **liste**, et une liste n'a pas de méthode `upper`. Mettez en
majuscules avant de découper (`texte.upper().split(",")`), ou chaque morceau.

### Croire qu'une méthode modifie le texte

```python norun
nom = "  alice  "
nom.strip()
print(f"[{nom}]")
```

```text
[  alice  ]
```

Pas d'erreur, mais rien n'a changé : `strip()` renvoie un nouveau texte, qui est
perdu. Écrivez `nom = nom.strip()`.

## Testez-vous

**Question 1 :** que vaut `"Bonjour"[1:4]` ?

> **Réponse**
>
> `"onj"` : les caractères aux indices 1, 2 et 3. L'indice 4 est exclu.

**Question 2 :** transformez `"pomme,poire,kiwi"` en `"pomme / poire / kiwi"`.

> **Réponse**
>
> ```python
> fruits = "pomme,poire,kiwi"
> print(" / ".join(fruits.split(",")))
> ```
>
> ```sortie
> pomme / poire / kiwi
> ```
>
> `fruits.replace(",", " / ")` marche aussi.

**Question 3 :** comment savoir si un mot est un palindrome (il se lit pareil dans les
deux sens) ?

> **Réponse**
>
> ```python
> mot = "radar"
> print(mot == mot[::-1])
> ```
>
> ```sortie
> True
> ```

## En résumé

- Un `str` est une suite de caractères, immuable : chaque opération renvoie un nouveau
  texte.
- `texte[i]` lit un caractère (0 en premier, -1 en dernier) ; `texte[a:b]` en lit une
  tranche, fin exclue.
- `upper`, `lower`, `title`, `strip` et `replace` transforment ; `split` et `join`
  découpent et recollent.
- `in`, `find`, `startswith` et `count` cherchent.
- `f"{x:<10}"` aligne ; `\n` et `\t` sont des caractères spéciaux ; `ord` et `chr`
  passent du caractère à son numéro Unicode.

Suite : [listes et tuples](collections.md).
