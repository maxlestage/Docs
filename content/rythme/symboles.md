# Les symboles et quand les utiliser

Un symbole comme `:fait` ou `:droite` est un petit nom utilisé comme valeur : cette page montre comment l'écrire, le comparer, le reconnaître avec `match`, et quand préférer un texte ou un `type`.

> **Objectifs :**
>
> - vous saurez écrire un symbole et reconnaître son type, `Symbol` ;
> - vous saurez comparer des symboles et les reconnaître avec `match` ;
> - vous saurez ranger des symboles dans des enregistrements et des dictionnaires ;
> - vous saurez pourquoi un symbole n'est jamais égal à un texte ;
> - vous saurez choisir entre un symbole, un texte et un `type`.

## Un nom utilisé comme valeur

Un [symbole](../debuter/glossaire.md#symbole) est un nom précédé de deux
points : `:oui`, `:fait`, `:droite`. Ce n'est pas un nom qui désigne une
valeur. C'est **la valeur elle-même**.

Pensez au jeton d'un vestiaire. Le jeton ne contient rien : il sert à être
reconnu. Un symbole, c'est pareil : il ne sert qu'à dire « c'est
celui-là ».

```rythme
reponse = :oui
print(reponse, type_of(reponse))
```

```sortie
:oui Symbol
```

Lisez la première ligne : « le nom `reponse` désigne le symbole `:oui` ».

```text
 reponse ──► [ :oui ]
 (un nom)    (une valeur : le symbole oui)
```

`type_of` répond `Symbol` : c'est une sorte de valeur à part, comme `Int`
ou `Text`.

```rythme
print(type_of(:oui), type_of("oui"), type_of(1))
```

```sortie
Symbol Text Int
```

Essayez : remplacez `:oui` par `:peut_etre`. Rythme l'accepte sans rien
déclarer : un symbole n'a pas besoin d'être créé avant d'être utilisé.

> **À retenir :** `:oui` est une valeur toute prête. Les deux points sont
> collés au nom.

## Écrire un symbole

Un symbole suit les mêmes règles qu'un nom : des lettres, des chiffres et
`_`, et les accents sont permis.

```rythme
print(:fait, :a_faire, :été, :etape2)
```

```sortie
:fait :a_faire :été :etape2
```

Trois règles à connaître :

- les deux points sont **collés** au nom : `:fait`, jamais `: fait` ;
- majuscules et minuscules comptent : `:Fait` et `:fait` sont deux
  symboles différents ;
- par habitude, on écrit les symboles en minuscules, avec `_` entre les
  mots : `:en_cours`.

## Comparer des symboles

Deux symboles sont égaux quand ils ont le même nom. On les compare avec
`==` et `!=`, comme des nombres :

```rythme
reponse = :oui
print(reponse == :oui, reponse == :non, reponse != :non)
```

```sortie
true false true
```

| Comparaison | Question | Résultat |
|-------------|----------|----------|
| `reponse == :oui` | est-ce le jeton `oui` ? | `true` |
| `reponse == :non` | est-ce le jeton `non` ? | `false` |
| `reponse != :non` | est-ce autre chose que `non` ? | `true` |

Les symboles se rangent aussi dans l'ordre alphabétique de leurs noms :

```rythme
print(sort([:poire, :abricot, :kiwi]), :abricot < :kiwi)
```

```sortie
[:abricot, :kiwi, :poire] true
```

## Un symbole n'est pas un texte

`:oui` et `"oui"` se ressemblent, mais ce sont deux sortes de valeurs.
Rythme ne les confond jamais : un symbole n'est **jamais** égal à un texte.

```rythme
reponse = :oui
print(reponse == "oui")
```

```sortie
false
```

Rythme vous prévient, avec un avertissement :

```text
avertissement[W006]: cette comparaison est toujours fausse : Symbol et Text ne sont jamais égaux
 --> main.ry:2:16
  |
2 | print(reponse == "oui")
  |                ^^ Symbol == Text "oui"
  |
  = aide: des valeurs de natures différentes ne sont jamais égales : Rythme ne convertit jamais les valeurs en silence
```

Pour mettre un symbole dans un texte, l'insertion `{...}` et `text`
marchent. Le symbole garde ses deux points :

```rythme
print("réponse : {:oui}", text(:oui), len(text(:oui)))
```

```sortie
réponse : :oui :oui 4
```

`text(:oui)` est le texte `":oui"` : quatre caractères, deux points
compris.

Quand on écrit du [JSON](../debuter/glossaire.md#json) avec `to_json`, un
symbole devient un texte ordinaire, sans les deux points :

```rythme
print(to_json({etat: :fait, nombre: 2}))
```

```sortie
{"etat": "fait", "nombre": 2}
```

> **À retenir :** comparez un symbole avec un symbole : `etat == :fait`,
> jamais `etat == "fait"`.

## Reconnaître un symbole avec match

Un symbole est fait pour être reconnu. `match` le compare à chaque cas,
de haut en bas :

```rythme
fn icone(etat) = match etat
  :fait -> "[x]"
  :en_cours -> "[~]"
  _ -> "[ ]"
end

print(icone(:fait), icone(:en_cours), icone(:a_faire))
```

```sortie
[x] [~] [ ]
```

Suivons `icone(:en_cours)` pas à pas :

| Étape | Cas essayé | Correspond ? | Résultat |
|-------|-----------|--------------|----------|
| 1 | `:fait` | non | |
| 2 | `:en_cours` | oui | `"[~]"` |

Le cas `_` attrape tous les autres symboles, et même les autres valeurs.

On peut aussi écrire une fonction **par cas**, une ligne par symbole (voir
[définir par cas](fonctions.md#definir-par-cas)) :

```rythme
fn traduire(:oui) = "yes"
fn traduire(:non) = "no"
fn traduire(_) = "?"

print(traduire(:oui), traduire(:non), traduire(:bof))
```

```sortie
yes no ?
```

Essayez : enlevez la ligne `fn traduire(_) = "?"`. `traduire(:bof)` arrête
alors le programme : aucun cas ne le reconnaît.

## Des symboles dans les enregistrements

Un symbole est idéal pour décrire l'**état** de quelque chose : une tâche
faite ou pas, une commande en attente ou livrée.

```rythme
tache = {nom: "courses", etat: :a_faire}
tache2 = {..tache, etat: :fait}
print(tache.etat, tache2.etat)
```

```sortie
:a_faire :fait
```

Remarquez les deux `:` dans `etat: :fait`. Le premier sépare le nom du
champ de sa valeur. Le second commence le symbole.

```text
 { etat:   :fait }
       ▲   ▲
       │   └── le début du symbole :fait
       └────── « le champ etat vaut... »
```

Avec une liste de tâches, on peut filtrer selon l'état :

```rythme
taches = [
  {nom: "courses", etat: :fait},
  {nom: "ménage", etat: :a_faire},
  {nom: "devoirs", etat: :en_cours},
  {nom: "vaisselle", etat: :fait},
]
fn icone(:fait) = "[x]"
fn icone(:en_cours) = "[~]"
fn icone(_) = "[ ]"
taches |> each(fn(t) = print("{icone(t.etat)} {t.nom}"))
faites = taches |> filter(fn(t) = t.etat == :fait)
print("{len(faites)} tâches faites sur {len(taches)}")
```

```sortie
[x] courses
[ ] ménage
[~] devoirs
[x] vaisselle
2 tâches faites sur 4
```

Essayez : passez le ménage à `:fait`. La dernière ligne affiche alors
`3 tâches faites sur 4`.

## Des symboles comme clés de dictionnaire

Un symbole fait aussi une bonne clé de
[dictionnaire](../debuter/glossaire.md#dictionnaire) (voir
[dictionnaires et ensembles](dictionnaires.md)) :

```rythme
stock = dict([[:pomme, 3], [:poire, 0], [:kiwi, 5]])
print(stock[:kiwi])
print(get(stock, :banane) |> unwrap_or(0))
print(keys(stock))
```

```sortie
5
0
[:kiwi, :poire, :pomme]
```

Le dictionnaire est rangé dans l'ordre des noms des symboles : `kiwi`,
`poire`, `pomme`.

> **Note :** si vous venez de Ruby, `{pomme: 3}` en Rythme n'est pas un
> dictionnaire avec la clé `:pomme`. C'est un enregistrement avec un champ
> `pomme`. Pour des clés symboles, écrivez `dict([[:pomme, 3]])`.

## Un exemple : se déplacer sur une grille

Les symboles décrivent bien une **direction**. Voici un pion qui part de
la case `{x: 0, y: 0}` :

```rythme
fn bouger(p, :haut) = {..p, y: p.y - 1}
fn bouger(p, :bas) = {..p, y: p.y + 1}
fn bouger(p, :gauche) = {..p, x: p.x - 1}
fn bouger(p, :droite) = {..p, x: p.x + 1}

depart = {x: 0, y: 0}
p1 = bouger(depart, :droite)
p2 = bouger(p1, :droite)
p3 = bouger(p2, :bas)
print(depart, p1, p2, p3)
```

```sortie
{x: 0, y: 0} {x: 1, y: 0} {x: 2, y: 0} {x: 2, y: 1}
```

| Étape | Direction | x | y |
|-------|-----------|---|---|
| départ | | 0 | 0 |
| 1 | `:droite` | 1 | 0 |
| 2 | `:droite` | 2 | 0 |
| 3 | `:bas` | 2 | 1 |

Comme le chemin est une liste de symboles, `fold` peut faire tous les pas
d'un coup (voir [penser fonctionnel](fonctionnel.md)) :

```rythme
fn bouger(p, :haut) = {..p, y: p.y - 1}
fn bouger(p, :bas) = {..p, y: p.y + 1}
fn bouger(p, :gauche) = {..p, x: p.x - 1}
fn bouger(p, :droite) = {..p, x: p.x + 1}

chemin = [:droite, :droite, :bas]
print(fold(chemin, {x: 0, y: 0}, bouger))
```

```sortie
{x: 2, y: 1}
```

## Passer d'un texte à un symbole

Ce que tape l'utilisateur est toujours un texte. Pour en faire un symbole,
écrivez une fonction par cas. Elle donne une
[option](../debuter/glossaire.md#option), car le texte peut ne
correspondre à rien :

```rythme
fn vers_reponse("oui") = Some(:oui)
fn vers_reponse("non") = Some(:non)
fn vers_reponse(_) = None

print(map(["oui", "non", "bof"], vers_reponse))
```

```sortie
[Some(:oui), Some(:non), None]
```

Une fois le symbole obtenu, le reste du programme ne manipule plus que
`:oui` et `:non`. Les fautes de frappe sont arrêtées à l'entrée.

## Symbole, texte ou type ?

Les trois peuvent décrire un état. Voici comment choisir :

| Vous avez... | Choisissez | Exemple |
|--------------|------------|---------|
| des mots à afficher ou venus de l'utilisateur | un texte | `"Léa"`, `"bonjour"` |
| quelques étiquettes, pour un petit programme | un symbole | `:fait`, `:droite` |
| une liste fermée de cas, à ne jamais oublier | un `type` | `type Etat = AFaire \| Fait` |

Le symbole est rapide à écrire. Mais Rythme ne connaît pas la liste de vos
symboles : une faute de frappe passe sans bruit.

```rythme
fn icone(:fait) = "[x]"
fn icone(:en_cours) = "[~]"
fn icone(_) = "[ ]"

tache = {nom: "devoirs", etat: :en_cour}
print(icone(tache.etat))
```

```sortie
[ ]
```

Il manque un `s` à `:en_cour`. Le programme ne dit rien, et affiche un
mauvais résultat.

Avec un [type](types.md), Rythme connaît tous les cas. Une faute de frappe
est une erreur, et un cas oublié dans un `match` donne un avertissement :

```rythme
type Etat = AFaire | EnCours | Fait

fn icone(e) = match e
  AFaire -> "[ ]"
  EnCours -> "[~]"
  Fait -> "[x]"
end

print(icone(EnCours), icone(Fait))
```

```sortie
[~] [x]
```

Écrire `EnCour` ici serait l'erreur E003, avec « vouliez-vous dire
`EnCours` ? ».

> **À retenir :** un symbole pour aller vite, un `type` quand les cas sont
> connus d'avance et qu'aucun ne doit être oublié.

## Erreurs fréquentes

### Mettre une espace après les deux points

```rythme norun
etat = : fait
```

```text
erreur[E001]: une expression attendu, mais trouvé `:`
 --> main.ry:1:8
  |
1 | etat = : fait
  |        ^ une expression attendu ici
  |
  = aide: Rythme n'utilise pas `:` pour commencer un bloc ; voir `rythme help syntax`
  = en savoir plus : `rythme explain E001`
```

Collez les deux points au nom : `etat = :fait`.

### Oublier l'espace dans un enregistrement

```rythme norun
tache = {etat:fait}
```

```text
erreur[E003]: nom inconnu `fait`
 --> main.ry:1:15
  |
1 | tache = {etat:fait}
  |               ^^^^ non défini
  |
  = aide: définissez-le avant de l'utiliser : `fait = ...` ou `fn fait(...) = ...`
  = en savoir plus : `rythme explain E003`
```

Juste après un nom de champ, `:` sépare le champ de sa valeur. Ici, la
valeur est le **nom** `fait`, qui n'existe pas. Écrivez `{etat: :fait}`,
avec deux `:` et une espace entre eux.

### Un match sans cas pour tout le reste

```rythme norun
fn icone(etat) = match etat
  :fait -> "[x]"
  :en_cours -> "[~]"
end

print(icone(:fait))
print(icone(:a_faire))
```

```text
[x]
erreur[E006]: aucun cas de ce `match` ne traite Symbol :a_faire
 --> main.ry:1:24
  |
1 | fn icone(etat) = match etat
  |                        ^^^^ cette valeur
  |
  = aide: ajoutez un cas pour cette valeur, ou un dernier cas `_ -> ...` qui traite tout le reste
  = appels en cours (le plus récent d'abord) :
      dans `icone` appelée en main.ry:7:7
  = en savoir plus : `rythme explain E006`
```

Le premier appel marche, le second arrête le programme. Rythme ne peut pas
connaître tous les symboles possibles : ajoutez un dernier cas
`_ -> "[ ]"`.

### Calculer avec un symbole

```rythme norun
print(:pomme + :poire)
```

```text
erreur[E005]: impossible d'utiliser `+` avec Symbol et Symbol
 --> main.ry:1:14
  |
1 | print(:pomme + :poire)
  |              ^ Symbol :pomme + Symbol :poire
  |
  = en savoir plus : `rythme explain E005`
```

Un symbole n'est ni un nombre ni un texte : on le compare, on le reconnaît,
on ne calcule pas avec. Pour un texte, passez par `text(:pomme)`.

## Testez-vous

**Question 1 :** que valent `:oui == :oui`, `:oui == :Oui` et
`:oui == "oui"` ?

> **Réponse**
>
> `true`, `false` et `false`. Les majuscules comptent, et un symbole n'est
> jamais égal à un texte.

**Question 2 :** qu'affiche ce programme ?

```rythme norun
fn signal(:vert) = "passez"
fn signal(:rouge) = "stop"
fn signal(_) = "prudence"
print(signal(:rouge), signal(:orange))
```

> **Réponse**
>
> `stop prudence`. `:rouge` a sa propre ligne ; `:orange` n'en a pas, donc
> il tombe sur `_`.

**Question 3 :** l'état d'une commande peut être « en attente »,
« envoyée » ou « livrée », et aucun cas ne doit être oublié. Symbole ou
`type` ?

> **Réponse**
>
> Un `type` : `type Commande = EnAttente | Envoyee | Livree`. Rythme
> connaît alors tous les cas : il refuse une faute de frappe et signale un
> cas oublié dans un `match` (W002).

## En résumé

- Un symbole s'écrit `:` collé à un nom : `:fait`. Son type est `Symbol`.
- Deux symboles sont égaux quand ils ont le même nom ; un symbole n'est
  jamais égal à un texte.
- `match` et les fonctions par cas reconnaissent les symboles ; gardez un
  dernier cas `_`.
- Dans un enregistrement, écrivez `{etat: :fait}`, avec une espace entre
  les deux `:`.
- Un symbole pour aller vite ; un `type` quand les cas sont connus et ne
  doivent pas être oubliés.

Suite : [Le filtrage par motif](pattern-matching.md)
