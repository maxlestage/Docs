# Exercices corrigés

Cette page propose 12 exercices corrigés, du plus facile au plus difficile, pour pratiquer tout le parcours Python.

## Comment utiliser cette page

Chaque exercice a un niveau :

- ★☆☆ : facile ;
- ★★☆ : moyen ;
- ★★★ : plus dur.

Lisez l'énoncé, puis écrivez votre solution dans un fichier `.py` et lancez-le avec
`python3`. Ouvrez la solution seulement après avoir essayé.

> **Astuce :** il existe souvent plusieurs bonnes solutions. Si votre programme affiche
> le bon résultat, c'est gagné !

## Les bases

### Exercice 1 : Se présenter

Niveau : ★☆☆

Créez deux variables, `prenom` et `age`. Affichez ensuite la phrase
`Je m'appelle Alice et j'ai 30 ans.`, avec vos variables.

> **Indice :** utilisez une f-string : `f"... {prenom} ..."` (voir
> [les premiers pas](premiers-pas.md)).

> **Solution**
>
> ```python
> prenom = "Alice"
> age = 30
> print(f"Je m'appelle {prenom} et j'ai {age} ans.")
> ```
>
> ```sortie
> Je m'appelle Alice et j'ai 30 ans.
> ```

### Exercice 2 : Pair ou impair

Niveau : ★☆☆

Pour le nombre `7`, affichez `7 est impair`. Pour `10`, le programme doit afficher
`10 est pair`.

> **Indice :** `n % 2` donne le reste de la division par 2 : `0` si `n` est pair (voir
> [les nombres](nombres.md)).

> **Solution**
>
> ```python
> for n in [7, 10]:
>     if n % 2 == 0:
>         print(f"{n} est pair")
>     else:
>         print(f"{n} est impair")
> ```
>
> ```sortie
> 7 est impair
> 10 est pair
> ```

### Exercice 3 : La table de 7

Niveau : ★☆☆

Affichez la table de multiplication de 7, de `7 x 1 = 7` à `7 x 5 = 35`.

> **Indice :** `range(1, 6)` donne les nombres de 1 à 5 (le 6 est exclu).

> **Solution**
>
> ```python
> for i in range(1, 6):
>     print(f"7 x {i} = {7 * i}")
> ```
>
> ```sortie
> 7 x 1 = 7
> 7 x 2 = 14
> 7 x 3 = 21
> 7 x 4 = 28
> 7 x 5 = 35
> ```

### Exercice 4 : L'aire d'un rectangle

Niveau : ★☆☆

Écrivez une fonction `aire(largeur, hauteur)` qui renvoie l'aire d'un rectangle. Si
on ne donne pas la hauteur, le rectangle est un carré : la hauteur vaut la largeur.
Affichez `aire(3, 4)` et `aire(5)`.

> **Indice :** donnez à `hauteur` la valeur par défaut `None`, puis testez-la (voir
> [les fonctions](fonctions.md)).

> **Solution**
>
> ```python
> def aire(largeur, hauteur=None):
>     if hauteur is None:
>         hauteur = largeur
>     return largeur * hauteur
>
>
> print(aire(3, 4))
> print(aire(5))
> ```
>
> ```sortie
> 12
> 25
> ```

## Texte et collections

### Exercice 5 : La moyenne

Niveau : ★☆☆

Écrivez une fonction `moyenne(notes)` qui renvoie la moyenne d'une liste, arrondie à
un chiffre après la virgule, ou `None` si la liste est vide.

> **Indice :** `sum`, `len` et `round` font tout le travail (voir
> [les collections](collections.md)).

> **Solution**
>
> ```python
> def moyenne(notes):
>     if not notes:
>         return None
>     return round(sum(notes) / len(notes), 1)
>
>
> print(moyenne([12, 15, 17]))
> print(moyenne([]))
> ```
>
> ```sortie
> 14.7
> None
> ```

### Exercice 6 : Compter les voyelles

Niveau : ★★☆

Écrivez une fonction `voyelles(texte)` qui compte les voyelles (`a e i o u y`) d'un
texte, en ignorant les majuscules.

> **Indice :** `texte.lower()` met tout en minuscules, et `lettre in "aeiouy"` teste si
> une lettre est une voyelle (voir [le texte](texte.md)).

> **Solution**
>
> ```python
> def voyelles(texte):
>     return sum(1 for lettre in texte.lower() if lettre in "aeiouy")
>
>
> print(voyelles("Bonjour"))
> print(voyelles("PYTHON"))
> ```
>
> ```sortie
> 3
> 2
> ```

### Exercice 7 : Le palindrome

Niveau : ★★☆

Un palindrome se lit pareil dans les deux sens, comme `"Radar"` ou
`"Engage le jeu que je le gagne"`. Écrivez `est_palindrome(texte)`, qui ignore les
majuscules et les espaces.

> **Indice :** `texte[::-1]` renvoie le texte à l'envers ; `replace(" ", "")` retire les
> espaces.

> **Solution**
>
> ```python
> def est_palindrome(texte):
>     propre = texte.lower().replace(" ", "")
>     return propre == propre[::-1]
>
>
> print(est_palindrome("Radar"))
> print(est_palindrome("Engage le jeu que je le gagne"))
> print(est_palindrome("Python"))
> ```
>
> ```sortie
> True
> True
> False
> ```

### Exercice 8 : Compter les mots

Niveau : ★★☆

Comptez combien de fois chaque mot apparaît dans la phrase
`"le chat et le chien et le lapin"`, puis affichez les mots du plus fréquent au moins
fréquent.

> **Indice :** un [dictionnaire](dictionnaires.md) associe chaque mot à son compteur ;
> `compteurs.get(mot, 0)` vaut 0 pour un mot nouveau. `collections.Counter` fait la même
> chose en une ligne.

> **Solution**
>
> ```python
> from collections import Counter
>
> phrase = "le chat et le chien et le lapin"
>
> compteurs = {}
> for mot in phrase.split():
>     compteurs[mot] = compteurs.get(mot, 0) + 1
>
> for mot, nombre in sorted(compteurs.items(), key=lambda paire: -paire[1]):
>     print(f"{mot} : {nombre}")
>
> print(Counter(phrase.split()).most_common(2))
> ```
>
> ```sortie
> le : 3
> et : 2
> chat : 1
> chien : 1
> lapin : 1
> [('le', 3), ('et', 2)]
> ```

### Exercice 9 : Garder et doubler

Niveau : ★★☆

À partir de la liste `[3, -1, 4, 0, -5, 9]`, construisez en une seule ligne la liste des
nombres positifs, doublés. Construisez aussi un dictionnaire qui associe chaque nombre
positif à son carré.

> **Indice :** une [compréhension](comprehensions.md) filtre avec `if` et transforme en
> même temps.

> **Solution**
>
> ```python
> nombres = [3, -1, 4, 0, -5, 9]
>
> print([n * 2 for n in nombres if n > 0])
> print({n: n * n for n in nombres if n > 0})
> ```
>
> ```sortie
> [6, 8, 18]
> {3: 9, 4: 16, 9: 81}
> ```

## Pour aller plus loin

### Exercice 10 : FizzBuzz

Niveau : ★★☆

Pour les nombres de 1 à 15, affichez `Fizz` si le nombre est divisible par 3, `Buzz`
s'il est divisible par 5, `FizzBuzz` s'il est divisible par les deux, et le nombre
sinon. Affichez le tout sur une ligne, séparé par des espaces.

> **Indice :** testez d'abord « divisible par 15 », sinon ce cas n'arrive jamais.

> **Solution**
>
> ```python
> def fizzbuzz(n):
>     if n % 15 == 0:
>         return "FizzBuzz"
>     if n % 3 == 0:
>         return "Fizz"
>     if n % 5 == 0:
>         return "Buzz"
>     return str(n)
>
>
> print(" ".join(fizzbuzz(n) for n in range(1, 16)))
> ```
>
> ```sortie
> 1 2 Fizz 4 Buzz Fizz 7 8 Fizz Buzz 11 Fizz 13 14 FizzBuzz
> ```

### Exercice 11 : Le compte en banque

Niveau : ★★★

Écrivez une classe `Compte` avec un `titulaire` et un `solde` (0 au départ), et deux
méthodes : `deposer(montant)` et `retirer(montant)`. Un montant négatif ou nul lève une
`ValueError`, et un retrait plus grand que le solde aussi. Testez avec `try` / `except`.

> **Indice :** voir [les classes](types.md) et [les erreurs](erreurs.md). `raise
> ValueError("message")` lève l'erreur.

> **Solution**
>
> ```python
> class Compte:
>     def __init__(self, titulaire):
>         self.titulaire = titulaire
>         self.solde = 0
>
>     def deposer(self, montant):
>         if montant <= 0:
>             raise ValueError("le montant doit être positif")
>         self.solde += montant
>
>     def retirer(self, montant):
>         if montant <= 0:
>             raise ValueError("le montant doit être positif")
>         if montant > self.solde:
>             raise ValueError("solde insuffisant")
>         self.solde -= montant
>
>     def __str__(self):
>         return f"{self.titulaire} : {self.solde} €"
>
>
> compte = Compte("Alice")
> compte.deposer(100)
> compte.retirer(30)
> print(compte)
>
> for montant in [-5, 500]:
>     try:
>         compte.retirer(montant)
>     except ValueError as erreur:
>         print(f"Refusé ({montant}) : {erreur}")
>
> print(compte)
> ```
>
> ```sortie
> Alice : 70 €
> Refusé (-5) : le montant doit être positif
> Refusé (500) : solde insuffisant
> Alice : 70 €
> ```

### Exercice 12 : Un inventaire en JSON

Niveau : ★★★

Un magasin décrit son stock en JSON :
`[{"nom": "pomme", "prix": 0.5, "quantite": 40}, {"nom": "kiwi", "prix": 0.8,
"quantite": 0}, {"nom": "poire", "prix": 0.7, "quantite": 12}]`.

Lisez ce texte, transformez chaque article en `dataclass` `Article`, puis affichez les
articles en stock, triés par nom, avec leur valeur (`prix * quantite`). Terminez par la
valeur totale du stock.

> **Indice :** `json.loads` transforme le texte en liste de dictionnaires (voir
> [les formats de données](formats.md)) ; `Article(**d)` crée un objet à partir d'un
> dictionnaire dont les clés sont les noms des champs.

> **Solution**
>
> ```python
> import json
> from dataclasses import dataclass
>
> TEXTE = """[
>   {"nom": "pomme", "prix": 0.5, "quantite": 40},
>   {"nom": "kiwi", "prix": 0.8, "quantite": 0},
>   {"nom": "poire", "prix": 0.7, "quantite": 12}
> ]"""
>
>
> @dataclass
> class Article:
>     nom: str
>     prix: float
>     quantite: int
>
>     def valeur(self) -> float:
>         return self.prix * self.quantite
>
>
> articles = [Article(**d) for d in json.loads(TEXTE)]
> en_stock = sorted((a for a in articles if a.quantite > 0), key=lambda a: a.nom)
>
> for article in en_stock:
>     print(f"{article.nom:<6} {article.quantite:>3} x {article.prix:.2f} = {article.valeur():6.2f}")
>
> total = sum(article.valeur() for article in articles)
> print(f"Valeur du stock : {total:.2f} €")
> ```
>
> ```sortie
> poire   12 x 0.70 =   8.40
> pomme   40 x 0.50 =  20.00
> Valeur du stock : 28.40 €
> ```

## En résumé

- Commencez par les exercices ★☆☆, puis montez en difficulté.
- Essayez toujours seul avant d'ouvrir la solution.
- Les exercices 1 à 4 reprennent les bases : variables, conditions, boucles, fonctions.
- Les exercices 5 à 9 travaillent le texte, les listes, les dictionnaires et les
  compréhensions.
- Les exercices 10 à 12 combinent tout, avec les classes, les erreurs, les dataclasses
  et JSON.

Suite : [Exercices corrigés : deuxième série](exercices-2.md)
