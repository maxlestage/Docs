# Vrai, faux et les conditions

Un programme prend des décisions en se posant des questions qui se répondent par vrai ou faux : cette page montre comment comparer, combiner des questions avec et, ou, non, puis choisir un chemin avec `if` et `else`.

> **Objectifs :**
>
> - savoir ce qu'est un booléen : vrai ou faux ;
> - comparer deux valeurs avec `==`, `!=`, `<`, `>=`… ;
> - combiner des questions avec et, ou, non ;
> - choisir entre deux chemins avec `if` et `else` ;
> - choisir entre plusieurs chemins avec `elif` ;
> - éviter les erreurs classiques : `=` au lieu de `==`, le cas « égal » oublié.

## Vrai ou faux : les booléens

Certaines questions se répondent seulement par **oui** ou par **non** :
« Il pleut ? », « J'ai plus de 18 ans ? ».

En programmation, oui se dit **vrai** et non se dit **faux**. Ces deux
valeurs s'appellent des **booléens** (voir [booléen](glossaire.md#booleen)).

```python
il_pleut = True
j_ai_faim = False
print(il_pleut)
print(j_ai_faim)
```

```sortie
True
False
```

En Python, on écrit `True` et `False`, avec une majuscule. En JavaScript et
en Ruby, on écrit `true` et `false`, en minuscules.

Un booléen se range dans une variable, comme un nombre. Voir
[les variables](variables.md).

> **À retenir :** un booléen n'a que deux valeurs possibles : vrai ou faux.

## Comparer deux valeurs

Le plus souvent, on n'écrit pas `True` à la main. On l'obtient en
**comparant** deux valeurs. La réponse est un booléen.

```python
age = 20
print(age > 18)
print(age < 18)
print(age == 20)
```

```sortie
True
False
True
```

Voici les six comparaisons. Elles s'écrivent pareil en Python, JavaScript
et Ruby :

| On écrit | On lit | Exemple vrai |
|---|---|---|
| `==` | est égal à | `3 == 3` |
| `!=` | est différent de | `3 != 4` |
| `<` | est plus petit que | `2 < 5` |
| `>` | est plus grand que | `5 > 2` |
| `<=` | est plus petit **ou égal** | `5 <= 5` |
| `>=` | est plus grand **ou égal** | `5 >= 5` |

On compare aussi des textes :

```python
mot_de_passe = "soleil"
print(mot_de_passe == "soleil")
print(mot_de_passe == "Soleil")
```

```sortie
True
False
```

Pour l'ordinateur, `s` et `S` sont deux lettres différentes.

> **Attention :** `=` range une valeur dans une variable. `==` pose une
> question : « est-ce égal ? ». Ce sont deux choses très différentes.

Essayez : remplacez `age = 20` par `age = 18`. La première ligne affiche
`False` : 18 n'est pas plus grand que 18.

## Combiner des questions : et, ou, non

Parfois, une seule question ne suffit pas. « Je sors **si** il fait beau
**et** que j'ai fini mon travail. »

Il y a trois mots pour combiner des booléens. Python les écrit en anglais ;
JavaScript et Ruby utilisent des symboles :

| En français | Python | JavaScript, Ruby |
|---|---|---|
| et | `and` | `&&` |
| ou | `or` | `\|\|` |
| non | `not` | `!` |

### « et » : les deux doivent être vrais

« Il fait beau **et** j'ai fini » n'est vrai que si les **deux** sont vrais.

| Il fait beau ? | J'ai fini ? | Je sors ? |
|---|---|---|
| oui | oui | **oui** |
| oui | non | non |
| non | oui | non |
| non | non | non |

```python
beau_temps = True
travail_fini = False
print(beau_temps and travail_fini)
```

```sortie
False
```

### « ou » : un seul suffit

« J'ai du café **ou** du thé » est vrai dès qu'**au moins un** des deux
est vrai.

| Du café ? | Du thé ? | J'ai une boisson ? |
|---|---|---|
| oui | oui | **oui** |
| oui | non | **oui** |
| non | oui | **oui** |
| non | non | non |

```python
cafe = False
the = True
print(cafe or the)
```

```sortie
True
```

### « non » : l'inverse

`not` retourne la réponse : vrai devient faux, faux devient vrai.

```python
il_pleut = False
print(not il_pleut)
```

```sortie
True
```

Le même calcul en JavaScript et en Ruby :

```javascript
const beauTemps = true;
const travailFini = false;
console.log(beauTemps && travailFini, beauTemps || travailFini, !beauTemps);
```

```sortie
false true false
```

```ruby
beau_temps = true
travail_fini = false
puts beau_temps && travail_fini, beau_temps || travail_fini, !beau_temps
```

```sortie
false
true
false
```

> **À retenir :** « et » demande que tout soit vrai ; « ou » se contente
> d'un seul vrai ; « non » inverse.

## Choisir un chemin : if et else

Imaginez une route qui se sépare en deux. Un panneau pose une question.
Selon la réponse, vous prenez à gauche ou à droite. Puis les deux routes se
rejoignent.

```text
              départ
                |
        +---------------+
        | age >= 18 ?   |
        +---------------+
          oui /     \ non
             /       \
   « Vous pouvez   « Pas encore. »
      voter. »
             \       /
              \     /
                |
             la suite
```

C'est exactement ce que fait `if` (« si ») et `else` (« sinon ») :

```python
age = 20
if age >= 18:
    print("Vous pouvez voter.")
else:
    print("Pas encore.")
print("Au revoir.")
```

```sortie
Vous pouvez voter.
Au revoir.
```

Lisez-le à voix haute : « **si** l'âge est 18 ou plus, affiche… ;
**sinon**, affiche… ». Une seule des deux branches est jouée. La dernière
ligne, elle, est toujours jouée : c'est « la suite ».

En Python, les deux-points `:` et le **décalage** des lignes (on dit
l'**indentation**, voir [indentation](glossaire.md#indentation)) montrent
ce qui est **dans** le `if`.

Essayez : mettez `age = 15`. Le programme affiche « Pas encore. » puis
« Au revoir. ».

Le `else` est facultatif. Sans lui, s'il n'y a rien à faire quand c'est
faux, on ne fait rien :

```python
panier = 0
if panier == 0:
    print("Votre panier est vide.")
```

```sortie
Votre panier est vide.
```

### En JavaScript et en Ruby

JavaScript met la question entre parenthèses et les branches entre
accolades `{ }` :

```javascript
const age = 20;
if (age >= 18) {
  console.log("Vous pouvez voter.");
} else {
  console.log("Pas encore.");
}
```

```sortie
Vous pouvez voter.
```

Ruby ferme la condition par `end` :

```ruby
age = 20
if age >= 18
  puts "Vous pouvez voter."
else
  puts "Pas encore."
end
```

```sortie
Vous pouvez voter.
```

> **À retenir :** `if` pose une question ; selon la réponse, une seule
> branche est jouée.

## Plus de deux chemins : elif

Une note de 0 à 20 donne une mention. Il y a **quatre** chemins, pas deux.

Une première idée : mettre un `if` dans le `else`, puis encore un, etc.
On parle de conditions **imbriquées** :

```python
note = 13
if note >= 16:
    print("Très bien")
else:
    if note >= 12:
        print("Bien")
    else:
        if note >= 10:
            print("Passable")
        else:
            print("Insuffisant")
```

```sortie
Bien
```

Ça marche, mais le code part de plus en plus vers la droite. C'est dur à
lire. Python a un mot pour « sinon, si » : `elif`.

```python
note = 13
if note >= 16:
    print("Très bien")
elif note >= 12:
    print("Bien")
elif note >= 10:
    print("Passable")
else:
    print("Insuffisant")
```

```sortie
Bien
```

L'ordinateur pose les questions **dans l'ordre**, et s'arrête à la
**première** réponse « oui » :

| Étape | Question | Réponse pour 13 | Que se passe-t-il ? |
|---|---|---|---|
| 1 | `note >= 16` | non | question suivante |
| 2 | `note >= 12` | oui | affiche « Bien », et c'est fini |
| 3 | `note >= 10` | pas posée | – |

13 est aussi plus grand que 10, mais la question n'est jamais posée.
L'ordre des questions compte donc : mettez la plus exigeante en premier.

En JavaScript, on écrit `else if` ; en Ruby, `elsif` (sans « e ») :

```javascript
const note = 13;
if (note >= 16) {
  console.log("Très bien");
} else if (note >= 12) {
  console.log("Bien");
} else {
  console.log("Insuffisant");
}
```

```sortie
Bien
```

```ruby
note = 13
if note >= 16
  puts "Très bien"
elsif note >= 12
  puts "Bien"
else
  puts "Insuffisant"
end
```

```sortie
Bien
```

Essayez : mettez `note = 16`, puis `note = 9`. Suivez les questions une à
une avant de lancer le programme.

> **À retenir :** avec `elif`, la première condition vraie gagne ; les
> suivantes sont ignorées.

## Erreurs fréquentes

### Écrire `=` au lieu de `==`

`=` range une valeur ; il ne pose pas de question. Python refuse :

```python norun
x = 5
if x = 5:
    print("cinq")
```

```text
  File "cinq.py", line 2
    if x = 5:
       ^^^^^
SyntaxError: invalid syntax. Maybe you meant '==' or ':=' instead of '='?
```

« Maybe you meant '=='? » : « vouliez-vous dire `==` ? ». JavaScript, lui,
ne dit **rien**, et c'est pire :

```javascript
let x = 3;
if (x = 0) {
  console.log("zéro");
} else {
  console.log("pas zéro");
}
console.log(x);
```

```sortie
pas zéro
0
```

`x = 0` a rangé 0 dans `x` : la valeur de `x` est perdue. La correction :
`if (x == 0)`, ou mieux en JavaScript, `if (x === 0)`.

### Tester une plage avec `0 < x < 10` en JavaScript

En maths, `0 < x < 10` veut dire « x est entre 0 et 10 ». JavaScript le lit
autrement :

```javascript
const x = 50;
if (0 < x < 10) {
  console.log("entre 0 et 10");
}
```

```sortie
entre 0 et 10
```

50 n'est pas entre 0 et 10 ! JavaScript calcule d'abord `0 < 50`, qui vaut
`true`. Puis il calcule `true < 10`. Il transforme `true` en `1`, et
`1 < 10` est vrai.

La correction : deux questions reliées par « et ».

```javascript
const x = 50;
if (0 < x && x < 10) {
  console.log("entre 0 et 10");
} else {
  console.log("en dehors");
}
```

```sortie
en dehors
```

En Python, `0 < x < 10` marche comme en maths. En Ruby, il provoque une
erreur (`undefined method '<' for true`) : écrivez `0 < x && x < 10`.

### Oublier le cas « égal »

Une boutique offre la livraison **à partir de** 50 €.

```python
panier = 50
if panier > 50:
    print("Livraison offerte")
else:
    print("Livraison : 5 €")
```

```sortie
Livraison : 5 €
```

Il n'y a pas de message d'erreur : c'est une **erreur de logique**. Avec
50 € pile, le client devrait avoir la livraison offerte. « À partir de 50 »
inclut 50 : il fallait `>=`.

```python
panier = 50
if panier >= 50:
    print("Livraison offerte")
else:
    print("Livraison : 5 €")
```

```sortie
Livraison offerte
```

> **Astuce :** testez toujours la valeur **exacte** de la limite (ici 50),
> plus une juste en dessous et une juste au-dessus.

## Testez-vous

**1.** Que vaut `(3 > 2) and (1 == 2)` ?

> **Réponse**
>
> `False`. `3 > 2` est vrai, mais `1 == 2` est faux. Avec « et », il faut
> que les deux soient vrais.

**2.** Avec `note = 10`, qu'affiche le programme des mentions avec `elif` ?

> **Réponse**
>
> « Passable ». `10 >= 16` : non. `10 >= 12` : non. `10 >= 10` : oui, car
> `>=` inclut l'égalité.

**3.** Quelle est la différence entre `=` et `==` ?

> **Réponse**
>
> `=` range une valeur dans une variable (`age = 20`). `==` compare deux
> valeurs et répond vrai ou faux (`age == 20`).

## En résumé

- Un booléen vaut vrai ou faux : `True`/`False` en Python, `true`/`false` en JavaScript et Ruby.
- Une comparaison (`==`, `!=`, `<`, `<=`, `>`, `>=`) répond par un booléen.
- « et » veut tout vrai, « ou » un seul vrai, « non » inverse.
- `if` / `else` choisit une branche ; `elif` enchaîne les questions, et la première vraie gagne.
- Attention à `=` au lieu de `==`, et au cas « égal » : `>` ou `>=` ?

Suite : [Les boucles](boucles.md)
