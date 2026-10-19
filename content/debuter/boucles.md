# Les boucles, pas à pas

Une boucle fait répéter une action à l'ordinateur sans recopier le code : cette page montre, tour par tour, comment elle avance.

> **Objectifs :**
>
> - comprendre pourquoi on utilise une boucle ;
> - suivre une boucle « pour chaque » tour par tour ;
> - compter avec `range` et savoir où il s'arrête ;
> - calculer une somme, compter, trouver le plus grand ;
> - écrire une boucle « tant que » et éviter la boucle infinie ;
> - arrêter une boucle avant la fin avec `break`.

## Répéter sans copier-coller

Vous voulez afficher « Bonjour » trois fois. Vous pouvez écrire :

```python
print("Bonjour")
print("Bonjour")
print("Bonjour")
```

```sortie
Bonjour
Bonjour
Bonjour
```

Ça marche. Mais pour 100 fois ? Ou 1 000 fois ? Recopier n'est pas une
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
main. Vous mettez tout dedans, et la machine répète le même geste.

> **À retenir :** une boucle évite de recopier le même code. On écrit
> l'action une fois, l'ordinateur la répète.

## La boucle « pour chaque »

La boucle la plus simple prend une [liste](glossaire.md#liste) et passe
sur **chaque élément**, un par un. En Python, c'est `for`.

```python
fruits = ["pomme", "poire", "kiwi"]
for fruit in fruits:
    print("J'aime la", fruit)
```

```sortie
J'aime la pomme
J'aime la poire
J'aime la kiwi
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
| 3 | `"kiwi"` | J'aime la kiwi |
| fin | – | plus d'élément : la boucle s'arrête |

Chaque passage s'appelle une [itération](glossaire.md#iteration). Le mot
fait peur, mais il veut juste dire « un tour ».

```text
fruits :  [ "pomme" , "poire" , "kiwi" ]
              ↑ tour 1   ↑ tour 2   ↑ tour 3
```

### Le décalage compte

En Python, les lignes **décalées** sous le `for` sont dans la boucle. Les
autres sont après.

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
const fruits = ["pomme", "poire", "kiwi"];
for (const fruit of fruits) {
  console.log("J'aime la", fruit);
}
```

```sortie
J'aime la pomme
J'aime la poire
J'aime la kiwi
```

```ruby
fruits = ["pomme", "poire", "kiwi"]
fruits.each do |fruit|
  puts "J'aime la #{fruit}"
end
```

```sortie
J'aime la pomme
J'aime la poire
J'aime la kiwi
```

```go
package main

import "fmt"

func main() {
	fruits := []string{"pomme", "poire", "kiwi"}
	for _, fruit := range fruits {
		fmt.Println("J'aime la", fruit)
	}
}
```

```sortie
J'aime la pomme
J'aime la poire
J'aime la kiwi
```

En Go, `range` donne deux choses : la position et l'élément. Le `_` veut
dire « la position, je n'en ai pas besoin ».

## Compter avec range

Souvent, on n'a pas de liste : on veut juste compter. `range` fabrique
une suite de nombres, un [intervalle](glossaire.md#intervalle).

```python
for i in range(5):
    print(i)
```

```sortie
0
1
2
3
4
```

Surprise : on obtient 5 nombres, mais de **0 à 4**. Pas de 1 à 5.

### Pourquoi il s'arrête un avant la fin

`range(5)` veut dire « 5 nombres, en commençant à 0 ». Le 5 est la
**limite** : on s'arrête **avant** elle.

Pensez à une règle graduée. De 0 à 5, il y a 5 centimètres. Le trait
« 5 » marque la fin, ce n'est pas un centimètre de plus.

```text
 0    1    2    3    4    5
 |----|----|----|----|----|
  1er  2e   3e   4e   5e     ← 5 cases, la limite 5 n'en est pas une
```

On peut aussi donner un départ : `range(début, limite)`.

```python
for i in range(1, 6):
    print(i)
```

```sortie
1
2
3
4
5
```

Pour aller **jusqu'à** 5, on écrit donc `6` comme limite.

Essayez : écrivez `range(1, 5)`. Vous n'aurez que 1, 2, 3, 4.

### Avancer de 2 en 2

Un troisième nombre donne le **pas** : de combien on avance à chaque tour.

```python
for i in range(0, 10, 2):
    print(i)
```

```sortie
0
2
4
6
8
```

> **À retenir :** `range(1, 6)` donne 1, 2, 3, 4, 5. La limite n'est
> jamais incluse.

### Compter dans d'autres langages

En JavaScript et en Go, on écrit les trois morceaux à la main : le
départ, la condition pour continuer, et le pas.

```javascript
for (let i = 1; i < 6; i++) {
  console.log(i);
}
```

```sortie
1
2
3
4
5
```

```go
package main

import "fmt"

func main() {
	for i := 1; i < 6; i++ {
		fmt.Println(i)
	}
}
```

```sortie
1
2
3
4
5
```

`i++` veut dire « ajoute 1 à `i` ». Ruby, lui, a un intervalle qui
**inclut** la fin avec deux points :

```ruby
(1..5).each do |i|
  puts i
end
```

```sortie
1
2
3
4
5
```

Avec trois points, `(1...5)`, Ruby s'arrête avant 5, comme Python.

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

### Tour par tour

`total = total + p` se lit : « prends l'ancien total, ajoute `p`, range le
résultat dans `total` ».

| Étape | p | total avant | total après |
|-------|---|-------------|-------------|
| départ | – | – | 0 |
| tour 1 | 3 | 0 | 3 |
| tour 2 | 5 | 3 | 8 |
| tour 3 | 2 | 8 | 10 |

Deux règles :

- le `total = 0` se met **avant** la boucle (une seule fois) ;
- le `print` se met **après** la boucle (quand tout est ajouté).

On écrit souvent `total += p` : c'est exactement la même chose, en plus
court.

Essayez : ajoutez `print(total)` **dans** la boucle (décalé). Vous voyez
le total grandir : 3, puis 8, puis 10.

> **À retenir :** pour une somme, on part de 0 avant la boucle, on ajoute
> dans la boucle, on affiche après.

## Accumuler : le compteur

Un compteur est une somme où l'on ajoute toujours **1**. Comptons les
notes au-dessus de 10.

```python
notes = [12, 8, 15, 9, 11]
compteur = 0
for note in notes:
    if note > 10:
        compteur = compteur + 1
print(compteur)
```

```sortie
3
```

| Étape | note | note > 10 ? | compteur |
|-------|------|-------------|----------|
| départ | – | – | 0 |
| tour 1 | 12 | oui | 1 |
| tour 2 | 8 | non | 1 |
| tour 3 | 15 | oui | 2 |
| tour 4 | 9 | non | 2 |
| tour 5 | 11 | oui | 3 |

Le `if` est **dans** la boucle : on pose la question à chaque tour. Les
conditions sont expliquées dans [la logique](logique.md).

## Accumuler : le plus grand

Pour trouver le plus grand, on garde dans une boîte « le plus grand vu
jusqu'ici ». On part du **premier** élément, pas de 0.

```python
notes = [12, 8, 15, 9]
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
| tour 4 | 9 | non | 15 |

Cet exemple est détaillé dans [penser comme un
programmeur](algorithmes.md), en cinq langages.

### La somme dans d'autres langages

```javascript
const prix = [3, 5, 2];
let total = 0;
for (const p of prix) {
  total += p;
}
console.log(total);
```

```sortie
10
```

```ruby
prix = [3, 5, 2]
total = 0
prix.each do |p|
  total += p
end
puts total
```

```sortie
10
```

```go
package main

import "fmt"

func main() {
	prix := []int{3, 5, 2}
	total := 0
	for _, p := range prix {
		total += p
	}
	fmt.Println(total)
}
```

```sortie
10
```

## La boucle « tant que »

Parfois, on ne sait pas **combien** de tours faire. On sait seulement
**quand s'arrêter**. C'est la boucle « tant que » : `while`.

« **Tant que** l'eau n'est pas chaude, attendre. » Vous ne savez pas
combien de minutes : vous regardez à chaque fois.

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

À chaque tour, Python pose la question `compteur <= 3`. Si la réponse est
oui, il fait un tour. Si c'est non, il sort.

| Étape | compteur | compteur <= 3 ? | Action |
|-------|----------|-----------------|--------|
| 1 | 1 | oui | affiche 1, compteur devient 2 |
| 2 | 2 | oui | affiche 2, compteur devient 3 |
| 3 | 3 | oui | affiche 3, compteur devient 4 |
| 4 | 4 | non | sort de la boucle |

### Un exemple où « tant que » est utile

Combien de fois faut-il doubler 1 pour dépasser 100 ?

```python
nombre = 1
fois = 0
while nombre <= 100:
    nombre = nombre * 2
    fois = fois + 1
print(fois, "fois, on arrive à", nombre)
```

```sortie
7 fois, on arrive à 128
```

On ne connaissait pas la réponse à l'avance : c'est le bon cas pour
`while`.

### La boucle infinie

Si rien ne change dans la boucle, la question a toujours la même réponse.
La boucle ne s'arrête **jamais**. C'est une **boucle infinie**.

```text
compteur vaut 1 → 1 <= 3 ? oui → affiche 1
compteur vaut 1 → 1 <= 3 ? oui → affiche 1
compteur vaut 1 → 1 <= 3 ? oui → affiche 1
... pour toujours
```

Pas de panique : appuyez sur **Ctrl + C** dans le
[terminal](terminal.md). Le programme s'arrête. Un exemple est dans les
erreurs fréquentes, plus bas.

> **À retenir :** dans une boucle `while`, quelque chose doit changer à
> chaque tour, sinon elle ne s'arrête jamais.

### « Tant que » dans d'autres langages

```javascript
let compteur = 1;
while (compteur <= 3) {
  console.log(compteur);
  compteur++;
}
```

```sortie
1
2
3
```

```ruby
compteur = 1
while compteur <= 3
  puts compteur
  compteur += 1
end
```

```sortie
1
2
3
```

```go
package main

import "fmt"

func main() {
	compteur := 1
	for compteur <= 3 {
		fmt.Println(compteur)
		compteur++
	}
}
```

```sortie
1
2
3
```

Go n'a pas de mot `while` : il utilise `for` avec seulement la condition.

## Arrêter tôt avec break

Vous cherchez vos clés dans des tiroirs. Dès que vous les trouvez, vous
arrêtez de chercher. Pas besoin d'ouvrir les tiroirs suivants.

`break` fait ça : il **sort de la boucle tout de suite**.

```python
tiroirs = ["chaussettes", "stylos", "clés", "piles"]
for tiroir in tiroirs:
    print("J'ouvre :", tiroir)
    if tiroir == "clés":
        print("Trouvées !")
        break
print("Je pars.")
```

```sortie
J'ouvre : chaussettes
J'ouvre : stylos
J'ouvre : clés
Trouvées !
Je pars.
```

Le tiroir « piles » n'est jamais ouvert.

| Tour | tiroir | tiroir == "clés" ? | Suite |
|------|--------|--------------------|-------|
| 1 | chaussettes | non | tour suivant |
| 2 | stylos | non | tour suivant |
| 3 | clés | oui | `break` : on sort |

Essayez : remplacez `"clés"` par `"piles"` dans le `if`. Les quatre
tiroirs sont ouverts.

`break` existe aussi en JavaScript, en Go (`break`) et en Ruby (`break`).

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

Le programme affiche `1` sans fin. Appuyez sur **Ctrl + C** pour
l'arrêter. Python affiche alors :

```text
1
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

### Un tour de trop ou de moins

On veut les nombres de 1 à 5, et on écrit `range(1, 5)`.

```python
for i in range(1, 5):
    print(i)
```

```sortie
1
2
3
4
```

Pas de message d'erreur, mais il manque le 5. La limite n'est jamais
incluse. Correction : `range(1, 6)`. Cette erreur d'un tour est si
courante qu'elle a un nom en anglais : *off-by-one*.

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

| Tour | p | total = 0 | total + p |
|------|---|-----------|-----------|
| 1 | 3 | 0 | 3 |
| 2 | 5 | 0 | 5 |
| 3 | 2 | 0 | 2 |

Correction : mettez `total = 0` **avant** le `for`, sans décalage.

## Testez-vous

**1.** Que affiche `for i in range(3): print(i)` ?

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
- `range(1, 6)` donne 1, 2, 3, 4, 5 : la limite n'est jamais incluse.
- Pour une somme : `total = 0` avant, `total += x` dedans, `print` après.
- `while` répète tant qu'une condition est vraie : quelque chose doit changer, sinon Ctrl + C.
- `break` sort de la boucle tout de suite.

Suite : [Les fonctions, sans jargon](fonctions.md).
