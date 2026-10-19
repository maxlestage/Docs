# Les boucles, pas à pas

Une boucle fait répéter une action à l'ordinateur sans recopier le code : cette page montre, tour par tour, comment elle avance.

> **Objectifs :**
>
> - comprendre pourquoi on utilise une boucle ;
> - suivre une boucle « pour chaque » tour par tour ;
> - compter avec `range` et savoir où il s'arrête ;
> - calculer une somme, compter, trouver le plus grand ;
> - écrire une boucle « tant que » sans qu'elle tourne pour toujours ;
> - arrêter une boucle avant la fin avec `break`.

## Répéter sans copier-coller

Pour afficher « Bonjour » trois fois, on peut écrire trois fois
`print("Bonjour")`. Mais pour 1 000 fois ? Recopier n'est pas une
solution.

Une [boucle](glossaire.md#boucle) dit à l'ordinateur : « refais ceci
plusieurs fois ». On écrit l'action **une seule fois**.

```python
for tour in range(3):
    print("Bonjour")
```

```sortie
Bonjour
Bonjour
Bonjour
```

Pour 1 000 fois, on change seulement `3` en `1000`.

Pensez à une machine à laver : vous ne lavez pas chaque chaussette à la
main. La machine répète le même geste pour vous.

> **À retenir :** une boucle évite de recopier le même code. On écrit
> l'action une fois, l'ordinateur la répète.

## La boucle « pour chaque »

La boucle la plus simple prend une [liste](glossaire.md#liste) et passe
sur **chaque élément**, un par un. En Python, c'est `for`.

```python
fruits = ["pomme", "poire", "banane"]
for fruit in fruits:
    print("J'aime la", fruit)
```

```sortie
J'aime la pomme
J'aime la poire
J'aime la banane
```

Lisez-la à voix haute : « **pour chaque** fruit **dans** fruits,
affiche… ».

### Ce qui se passe, tour par tour

La variable `fruit` est une boîte. À chaque tour, la boucle met **un
nouvel élément** dedans, puis exécute le code décalé.

| Tour | fruit | Ce qui s'affiche |
|------|-------|------------------|
| 1 | `"pomme"` | J'aime la pomme |
| 2 | `"poire"` | J'aime la poire |
| 3 | `"banane"` | J'aime la banane |
| fin | – | plus d'élément : la boucle s'arrête |

Un tour s'appelle aussi une [itération](glossaire.md#iteration). Le mot
fait peur, mais il veut juste dire « un tour ».

### Le décalage compte

En Python, les lignes **décalées** sous le `for` sont dans la boucle. Les
autres viennent après.

```python
for fruit in ["pomme", "poire"]:
    print("Dans la boucle :", fruit)
print("Après la boucle")
```

```sortie
Dans la boucle : pomme
Dans la boucle : poire
Après la boucle
```

« Après la boucle » s'affiche **une seule fois** : la ligne n'est pas
décalée. Ce décalage s'appelle l'[indentation](glossaire.md#indentation).

Essayez : décalez la dernière ligne de 4 espaces. Elle s'affiche alors
deux fois, une par tour.

> **À retenir :** « pour chaque » passe sur tous les éléments, un par un,
> et s'arrête tout seul à la fin de la liste.

### La même boucle dans d'autres langages

L'idée est la même partout. Seuls les mots changent.

```javascript
for (const fruit of ["pomme", "poire"]) {
  console.log("J'aime la", fruit);
}
```

```sortie
J'aime la pomme
J'aime la poire
```

```ruby
["pomme", "poire"].each do |fruit|
  puts "J'aime la #{fruit}"
end
```

```sortie
J'aime la pomme
J'aime la poire
```

```go
package main

import "fmt"

func main() {
	for _, fruit := range []string{"pomme", "poire"} {
		fmt.Println("J'aime la", fruit)
	}
}
```

```sortie
J'aime la pomme
J'aime la poire
```

En Go, `range` donne deux choses : la position et l'élément. Le `_` veut
dire « la position, je n'en ai pas besoin ».

## Compter avec range

Souvent, on n'a pas de liste : on veut juste compter. `range` fabrique
une suite de nombres, un [intervalle](glossaire.md#intervalle).

```python
for i in range(4):
    print(i)
```

```sortie
0
1
2
3
```

Surprise : on obtient 4 nombres, mais de **0 à 3**. Pas de 1 à 4.

### Pourquoi il s'arrête un avant la fin

`range(4)` veut dire « 4 nombres, en commençant à 0 ». Le 4 est la
**limite** : on s'arrête **avant** elle.

Pensez à une règle graduée. De 0 à 4, il y a 4 centimètres. Le trait
« 4 » marque la fin : ce n'est pas un centimètre de plus.

```text
 0    1    2    3    4
 |----|----|----|----|
  1er  2e   3e   4e      ← 4 cases ; la limite 4 n'en est pas une
```

On peut aussi donner un départ : `range(départ, limite)`.

```python
for i in range(1, 4):
    print(i)
```

```sortie
1
2
3
```

Pour aller **jusqu'à** 3, on écrit donc `4` comme limite. Un troisième
nombre donne le pas : `range(0, 10, 2)` donne 0, 2, 4, 6, 8.

Essayez : écrivez `range(1, 3)`. Vous n'aurez que 1 et 2.

> **À retenir :** `range(1, 4)` donne 1, 2, 3. La limite n'est jamais
> incluse.

### Compter dans d'autres langages

En JavaScript et en Go, on écrit trois morceaux : le départ, la condition
pour continuer, et le pas. `for (let i = 1; i < 4; i++)` donne aussi 1, 2,
3 (`i++` veut dire « ajoute 1 à `i` »).

En Ruby, `(1..3)` va de 1 à 3, fin **incluse** (deux points). Avec trois
points, `(1...3)`, il s'arrête avant, comme Python.

## Accumuler : la somme

Voici l'usage le plus courant d'une boucle : **additionner** des nombres.
On garde une boîte `total`, et on y ajoute chaque nombre.

C'est comme une tirelire. Elle est vide au départ. À chaque pièce, le
contenu grossit.

```python
prix = [3, 5, 2]
total = 0
for p in prix:
    total = total + p
print(total)
```

```sortie
10
```

`total = total + p` se lit : « prends l'ancien total, ajoute `p`, range le
résultat dans `total` ».

| Étape | p | total avant | total après |
|-------|---|-------------|-------------|
| départ | – | – | 0 |
| tour 1 | 3 | 0 | 3 |
| tour 2 | 5 | 3 | 8 |
| tour 3 | 2 | 8 | 10 |

On écrit souvent `total += p` : c'est la même chose, en plus court.

Essayez : ajoutez `print(total)` **dans** la boucle (décalé). Vous voyez
la tirelire grossir : 3, puis 8, puis 10.

> **À retenir :** pour une somme, on met `total = 0` **avant** la boucle,
> on ajoute **dans** la boucle, on affiche **après**.

## Accumuler : le compteur et le plus grand

Un **compteur** est une somme où l'on ajoute toujours 1. Comptons les
notes au-dessus de 10.

```python
notes = [12, 8, 15]
compteur = 0
for note in notes:
    if note > 10:
        compteur += 1
print(compteur)
```

```sortie
2
```

| Étape | note | note > 10 ? | compteur |
|-------|------|-------------|----------|
| départ | – | – | 0 |
| tour 1 | 12 | oui | 1 |
| tour 2 | 8 | non | 1 |
| tour 3 | 15 | oui | 2 |

Le `if` est **dans** la boucle : on pose la question à chaque tour. Les
conditions sont expliquées dans [la logique](logique.md).

Pour le **plus grand**, on garde « le plus grand vu jusqu'ici ». On part
du **premier** élément, pas de 0.

```python
notes = [12, 8, 15]
plus_grand = notes[0]
for note in notes:
    if note > plus_grand:
        plus_grand = note
print(plus_grand)
```

```sortie
15
```

| Étape | note | note > plus_grand ? | plus_grand |
|-------|------|---------------------|------------|
| départ | – | – | 12 |
| tour 1 | 12 | non | 12 |
| tour 2 | 8 | non | 12 |
| tour 3 | 15 | oui | 15 |

Cet exemple est détaillé, en cinq langages, dans [penser comme un
programmeur](algorithmes.md).

## La boucle « tant que »

Parfois, on ne sait pas **combien** de tours faire. On sait seulement
**quand s'arrêter**. C'est la boucle « tant que » : `while`.

« **Tant que** l'eau n'est pas chaude, attendre. » Vous ne savez pas
combien de minutes : vous vérifiez à chaque fois.

```python
compteur = 1
while compteur <= 3:
    print(compteur)
    compteur = compteur + 1
print("Fini")
```

```sortie
1
2
3
Fini
```

Avant chaque tour, Python pose la question `compteur <= 3`. Oui : il fait
un tour. Non : il sort.

| Étape | compteur | compteur <= 3 ? | Action |
|-------|----------|-----------------|--------|
| 1 | 1 | oui | affiche 1, compteur devient 2 |
| 2 | 2 | oui | affiche 2, compteur devient 3 |
| 3 | 3 | oui | affiche 3, compteur devient 4 |
| 4 | 4 | non | sort de la boucle |

### La boucle infinie

Si rien ne change dans la boucle, la question a toujours la même réponse.
La boucle ne s'arrête **jamais** : c'est une **boucle infinie**.

Pas de panique : appuyez sur **Ctrl + C** dans le
[terminal](terminal.md), et le programme s'arrête. Un exemple est dans les
erreurs fréquentes, plus bas.

> **À retenir :** dans une boucle `while`, quelque chose doit changer à
> chaque tour, sinon elle ne s'arrête jamais.

### « Tant que » dans d'autres langages

```javascript
let compteur = 1;
while (compteur <= 2) {
  console.log(compteur);
  compteur++;
}
```

```sortie
1
2
```

Go n'a pas de mot `while` : il écrit `for compteur <= 2 { … }`.

## Arrêter tôt avec break

Vous cherchez vos clés dans des tiroirs. Dès que vous les trouvez, vous
arrêtez. Pas besoin d'ouvrir les tiroirs suivants.

`break` fait ça : il **sort de la boucle tout de suite**.

```python
tiroirs = ["stylos", "clés", "piles"]
for tiroir in tiroirs:
    print("J'ouvre :", tiroir)
    if tiroir == "clés":
        print("Trouvées !")
        break
print("Je pars.")
```

```sortie
J'ouvre : stylos
J'ouvre : clés
Trouvées !
Je pars.
```

Le tiroir « piles » n'est jamais ouvert. `break` s'écrit pareil en
JavaScript, en Ruby et en Go.

Essayez : remplacez `"clés"` par `"piles"` dans le `if`. Les trois
tiroirs sont ouverts.

> **À retenir :** `break` arrête la boucle immédiatement. Le programme
> continue après la boucle.

## Erreurs fréquentes

### La boucle infinie

On oublie de faire avancer le compteur.

```python norun
compteur = 1
while compteur <= 3:
    print(compteur)
```

Le programme affiche `1` sans fin. Appuyez sur **Ctrl + C**. Python
affiche alors :

```text
1
1
Traceback (most recent call last):
  File "boucle.py", line 3, in <module>
    print(compteur)
    ~~~~~^^^^^^^^^^
KeyboardInterrupt
```

`KeyboardInterrupt` veut dire « arrêté au clavier » : c'est vous. Pour
corriger, ajoutez `compteur = compteur + 1` dans la boucle (décalé).

### Un tour de moins

On veut les nombres de 1 à 3, et on écrit `range(1, 3)`.

```python
for i in range(1, 3):
    print(i)
```

```sortie
1
2
```

Pas de message d'erreur, mais il manque le 3 : la limite n'est jamais
incluse. Correction : `range(1, 4)`. Cette erreur d'un tour est si
courante qu'elle a un nom anglais : *off-by-one* (« décalé de un »).

### Remettre le total à zéro dans la boucle

Le `total = 0` est décalé : il est **dans** la boucle.

```python
prix = [3, 5, 2]
for p in prix:
    total = 0
    total = total + p
print(total)
```

```sortie
2
```

On attendait 10. À chaque tour, la tirelire est vidée, puis on ajoute une
seule pièce. À la fin, il ne reste que le dernier prix.

| Tour | p | après `total = 0` | après `total + p` |
|------|---|-------------------|-------------------|
| 1 | 3 | 0 | 3 |
| 2 | 5 | 0 | 5 |
| 3 | 2 | 0 | 2 |

Correction : mettez `total = 0` **avant** le `for`, sans décalage.

## Testez-vous

**1.** Qu'affiche `for i in range(3): print(i)` ?

> **Réponse**
>
> `0`, puis `1`, puis `2`, chacun sur une ligne. `range(3)` commence à 0
> et s'arrête avant 3.

**2.** Quand faut-il préférer `while` à `for` ?

> **Réponse**
>
> Quand on ne sait pas à l'avance combien de tours faire, mais qu'on sait
> quand s'arrêter. Par exemple : « tant que le nombre est plus petit que
> 100 ».

**3.** Avec `nombres = [4, 1, 6]`, `total = 0` avant la boucle et
`total += n` dedans, quelles valeurs prend `total` ?

> **Réponse**
>
> 0 au départ, puis 4, puis 5, puis 11. Le résultat est 11.

## En résumé

- Une boucle répète une action sans recopier le code.
- `for` passe sur chaque élément d'une liste, un par un.
- `range(1, 4)` donne 1, 2, 3 : la limite n'est jamais incluse.
- Pour une somme : `total = 0` avant, `total += x` dedans, `print` après.
- `while` répète tant qu'une condition est vraie : quelque chose doit changer, sinon Ctrl + C.
- `break` sort de la boucle tout de suite.

Suite : [Les fonctions, sans jargon](fonctions.md).
