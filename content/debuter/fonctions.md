# Les fonctions, sans jargon

Une fonction est une petite machine à laquelle on donne un nom : on lui donne des ingrédients, elle fait son travail et rend un résultat.

> **Objectifs :**
>
> - comprendre ce qu'est une fonction, avec l'image d'une machine ;
> - définir une fonction, puis l'appeler ;
> - donner des paramètres à une fonction ;
> - faire la différence entre `return` et `print` ;
> - utiliser une fonction dans une autre ;
> - savoir qu'une variable créée dans une fonction reste dedans.

## Une fonction, c'est une machine

Pensez à un grille-pain. Vous mettez du pain (l'**entrée**). Il fait son
travail. Il rend du pain grillé (la **sortie**). Vous n'avez pas besoin
de savoir comment il marche dedans.

Une [fonction](glossaire.md#fonction) marche pareil :

```text
            ┌────────────────┐
   3   ───► │     double     │ ───►  6
 entrée     │  (fois deux)   │      sortie
            └────────────────┘
```

On peut aussi la voir comme une **recette** : elle a un nom (« crêpes »),
une liste d'ingrédients, et des étapes. On l'écrit une fois. Ensuite, on
dit juste « fais des crêpes ».

> **À retenir :** une fonction a un nom, reçoit des entrées et rend une
> sortie.

## Définir, puis appeler

Il y a deux moments bien séparés :

1. **définir** la fonction : écrire la recette dans le cahier ;
2. **appeler** la fonction : cuisiner avec la recette.

En Python, on définit avec `def` :

```python
def dire_bonjour():
    print("Bonjour !")
    print("Comment allez-vous ?")

dire_bonjour()
dire_bonjour()
```

```sortie
Bonjour !
Comment allez-vous ?
Bonjour !
Comment allez-vous ?
```

- `def dire_bonjour():` donne un nom à la recette ;
- les lignes **décalées** sont les étapes de la recette ;
- `dire_bonjour()`, avec les parenthèses, **lance** la recette.

### Dans quel ordre Python lit le programme

Définir ne lance **rien**. Python range la recette et passe à la suite.

| Étape | Ligne lue | Ce qui se passe |
|-------|-----------|-----------------|
| 1 | `def dire_bonjour():` | Python retient la recette, n'affiche rien |
| 2 | `dire_bonjour()` | saute dans la fonction, affiche les 2 lignes |
| 3 | retour | revient juste après l'appel |
| 4 | `dire_bonjour()` | saute encore dans la fonction, affiche les 2 lignes |

Essayez : effacez les deux appels. Le programme n'affiche plus rien. La
recette existe, mais personne ne cuisine.

> **À retenir :** `def` écrit la recette. Les parenthèses `()` la font
> marcher.

## Paramètres et arguments

Une recette peut changer selon ce qu'on lui donne. Pour saluer
quelqu'un, il faut son prénom.

```python
def saluer(prenom):
    print("Bonjour", prenom)

saluer("Léa")
saluer("Tom")
```

```sortie
Bonjour Léa
Bonjour Tom
```

Deux mots, deux places :

- `prenom` est le [paramètre](glossaire.md#parametre) : une boîte vide,
  dans la définition ;
- `"Léa"` est l'[argument](glossaire.md#argument) : la valeur qu'on met
  dans la boîte, au moment de l'appel.

| Appel | Valeur de `prenom` dans la fonction | Affiche |
|-------|-------------------------------------|---------|
| `saluer("Léa")` | `"Léa"` | Bonjour Léa |
| `saluer("Tom")` | `"Tom"` | Bonjour Tom |

Une fonction peut avoir plusieurs paramètres, séparés par des virgules.
L'ordre compte : le premier argument va dans le premier paramètre.

```python
def presenter(prenom, age):
    print(prenom, "a", age, "ans")

presenter("Léa", 12)
```

```sortie
Léa a 12 ans
```

Essayez : écrivez `presenter(12, "Léa")`. Python affiche `12 a Léa ans` :
les boîtes ont été remplies dans le mauvais ordre.

> **À retenir :** le paramètre est la boîte (dans `def`), l'argument est
> ce qu'on met dedans (dans l'appel).

## Rendre un résultat : return

`return` veut dire « voici le résultat, je te le rends ». La fonction
s'arrête et donne la valeur à celui qui l'a appelée.

```python
def double(x):
    return x * 2

resultat = double(3)
print(resultat)
print(double(10) + 1)
```

```sortie
6
21
```

L'appel `double(3)` est **remplacé** par sa réponse, comme si on avait
écrit `6`.

```text
resultat = double(3)
           └───┬───┘
               6          → resultat = 6
```

Le résultat peut servir ailleurs : dans un calcul, dans une variable, dans
un autre appel.

### return ou print : la grande confusion

C'est le piège le plus courant. Les deux semblent « donner » une valeur.
Ce n'est pas du tout pareil.

- `print` **montre** une valeur à l'écran, pour vous. Le programme ne peut
  pas la réutiliser.
- `return` **rend** une valeur au programme. Rien ne s'affiche.

Comparez ces deux fonctions :

```python
def double_affiche(x):
    print(x * 2)

def double_rend(x):
    return x * 2

a = double_affiche(3)
b = double_rend(3)
print("a vaut", a)
print("b vaut", b)
```

```sortie
6
a vaut None
b vaut 6
```

Ligne par ligne :

| Ligne | Ce qui se passe | Affiché |
|-------|-----------------|---------|
| `a = double_affiche(3)` | la fonction **montre** 6, mais ne rend rien | `6` |
| `b = double_rend(3)` | la fonction **rend** 6, sans rien montrer | rien |
| `print("a vaut", a)` | `a` est vide : Python dit `None` | `a vaut None` |
| `print("b vaut", b)` | `b` contient 6 | `b vaut 6` |

`None` veut dire « rien ». Une fonction sans `return` rend `None`.

Pensez au grille-pain. `print`, c'est une photo du toast envoyée par
message : vous voyez le toast, mais vous ne pouvez pas le manger. `return`,
c'est le toast posé dans votre assiette.

Essayez : écrivez `print(double_affiche(3) + 1)`. Python affiche 6, puis
une erreur : on ne peut pas ajouter 1 à « rien ».

> **À retenir :** `print` montre, `return` rend. Si vous voulez réutiliser
> le résultat, il faut `return`.

### Dans d'autres langages

L'idée est la même : un nom, des paramètres entre parenthèses, un
`return`.

```javascript
function double(x) {
  return x * 2;
}
console.log(double(3));
```

```sortie
6
```

```ruby
def double(x)
  return x * 2
end
puts double(3)
```

```sortie
6
```

En Ruby, la dernière ligne est rendue toute seule : on peut écrire juste
`x * 2`.

```rust
fn double(x: i32) -> i32 {
    x * 2
}

fn main() {
    println!("{}", double(3));
}
```

```sortie
6
```

Rust demande le [type](glossaire.md#type) de chaque entrée (`x: i32`, un
nombre entier) et de la sortie (`-> i32`). La dernière ligne, sans
point-virgule, est le résultat rendu.

## Une fonction qui en appelle une autre

Une fonction peut utiliser une autre fonction. C'est comme une recette
de tarte qui dit « préparez la pâte (voir page 12) ».

```python
def carre(n):
    return n * n

def somme_des_carres(a, b):
    return carre(a) + carre(b)

print(somme_des_carres(3, 4))
```

```sortie
25
```

Suivons le calcul :

| Étape | Ce qui se calcule | Résultat |
|-------|-------------------|----------|
| 1 | `somme_des_carres(3, 4)` commence, `a` = 3, `b` = 4 | – |
| 2 | `carre(3)` | 9 |
| 3 | `carre(4)` | 16 |
| 4 | `9 + 16`, rendu par `somme_des_carres` | 25 |

```text
somme_des_carres(3, 4)
   ├── carre(3)  →  9
   └── carre(4)  → 16
                    ──
                    25
```

Essayez : remplacez `return n * n` par `return n * n * n`. Le résultat
devient 27 + 64 = 91.

## Pourquoi les fonctions aident

Sans fonctions, on recopie le même calcul partout. Si le calcul change,
il faut le corriger à chaque endroit, et on en oublie un.

```python
def prix_ttc(prix):
    return prix * 1.2

print(prix_ttc(10))
print(prix_ttc(50))
print(prix_ttc(100))
```

```sortie
12.0
60.0
120.0
```

Les fonctions aident de deux façons :

- **pas de répétition** : la formule est écrite une seule fois. Si la
  taxe change, on change une seule ligne ;
- **un nom clair** : `prix_ttc(10)` se lit mieux que `10 * 1.2`. On
  comprend ce que fait le code sans lire le calcul.

> **À retenir :** dès que vous recopiez le même code, faites-en une
> fonction, avec un nom qui dit ce qu'elle fait.

## Les variables locales

Une variable créée **dans** une fonction n'existe que **dans** cette
fonction. On dit qu'elle est **locale**. Sa zone de vie s'appelle la
[portée](glossaire.md#portee).

Pensez à un brouillon dans une salle d'examen : vous écrivez dessus,
puis il reste dans la salle. Seule la copie rendue (le `return`) sort.

```python
def calculer():
    resultat = 6 * 7
    return resultat

reponse = calculer()
print(reponse)
```

```sortie
42
```

Hors de la fonction, `resultat` n'existe pas :

```python norun
def calculer():
    resultat = 6 * 7

calculer()
print(resultat)
```

```text
Traceback (most recent call last):
  File "local.py", line 5, in <module>
    print(resultat)
          ^^^^^^^^
NameError: name 'resultat' is not defined
```

C'est une bonne chose : deux fonctions peuvent chacune avoir leur
variable `total` sans se gêner. Pour sortir une valeur, utilisez
`return`.

> **À retenir :** ce qui est créé dans une fonction reste dans la
> fonction. Seul `return` fait sortir une valeur.

## Erreurs fréquentes

### Oublier les parenthèses à l'appel

Sans `()`, la fonction n'est pas lancée. On parle d'elle sans la faire
marcher.

```python norun
def dire_bonjour():
    print("Bonjour !")

print(dire_bonjour)
```

```text
<function dire_bonjour at 0x7f8377a271a0>
```

Python décrit la fonction au lieu de la lancer. (Le nombre `0x7f…` change
à chaque fois : c'est une adresse dans la mémoire.) Correction :
`dire_bonjour()`.

### Oublier return

La fonction calcule, mais ne rend rien.

```python
def double(x):
    x * 2

print(double(3))
```

```sortie
None
```

Pas de message d'erreur, mais `None` au lieu de 6. Le calcul est fait,
puis jeté. Correction : `return x * 2`.

### Le mauvais nombre d'arguments

La fonction attend un prénom, et on ne lui donne rien.

```python norun
def saluer(prenom):
    print("Bonjour", prenom)

saluer()
```

```text
Traceback (most recent call last):
  File "saluer.py", line 4, in <module>
    saluer()
    ~~~~~~^^
TypeError: saluer() missing 1 required positional argument: 'prenom'
```

Lisez la dernière ligne : « il manque 1 argument : `prenom` ». Avec un
argument de trop, `saluer("Léa", "Tom")`, le message devient :

```text
TypeError: saluer() takes 1 positional argument but 2 were given
```

Correction : donnez autant d'arguments qu'il y a de paramètres,
`saluer("Léa")`. Voir aussi [lire un message d'erreur](lire-une-erreur.md).

## Testez-vous

**1.** Que fait ce programme : `def f(): print("Salut")` et rien d'autre ?

> **Réponse**
>
> Rien ne s'affiche. La fonction est définie, mais jamais appelée. Il
> faut ajouter `f()`.

**2.** Dans `def aire(largeur, hauteur):` et `aire(3, 5)`, quels sont les
paramètres et les arguments ?

> **Réponse**
>
> Les paramètres sont `largeur` et `hauteur` (les boîtes, dans `def`). Les
> arguments sont `3` et `5` (les valeurs, dans l'appel). `largeur` reçoit
> 3, `hauteur` reçoit 5.

**3.** Pourquoi `x = ma_fonction()` donne-t-il `None` alors que la fonction
affiche bien un nombre ?

> **Réponse**
>
> Parce que la fonction utilise `print` au lieu de `return`. Elle
> **montre** le nombre, mais ne le **rend** pas. Une fonction sans
> `return` rend `None`.

## En résumé

- Une fonction est une machine : des entrées, un travail, une sortie.
- `def` écrit la recette ; `nom()` la lance.
- Le paramètre est la boîte dans `def` ; l'argument est la valeur donnée à l'appel.
- `print` montre une valeur ; `return` la rend au programme.
- Une fonction peut en appeler une autre ; elle évite de recopier du code.
- Une variable créée dans une fonction reste dans la fonction.

Suite : [Les données](donnees.md).
