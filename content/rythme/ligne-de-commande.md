# Écrire un outil en ligne de commande

Un programme Rythme peut recevoir des mots et des options dans le terminal, lire ce qu'on lui envoie, et dire s'il a réussi : cette page construit, pas à pas, un petit outil qui compte les mots d'un fichier.

> **Objectifs :**
>
> - vous saurez lire les mots donnés après le nom du fichier avec `args()` ;
> - vous saurez lire des options comme `--nom=Ada` avec `options()`, qui renvoie un `Result` ;
> - vous saurez lire au clavier avec `input` et `input_lines`, et une variable d'environnement avec `env` ;
> - vous saurez arrêter un programme avec un code de sortie, grâce à `exit` ;
> - vous saurez assembler le tout dans un outil complet.

## Des mots après le nom du fichier

Dans le [terminal](../debuter/glossaire.md#terminal), vous lancez un
programme avec `rythme fichier.ry`. Tout ce que vous écrivez **après** le nom
du fichier va au programme. `args()` donne ces mots, sous forme de liste de
textes :

```rythme norun
print(args())
```

```text
$ rythme montrer.ry Ada 36 "deux mots"
["Ada", "36", "deux mots"]
```

Trois remarques :

- les mots sont séparés par des espaces ; des guillemets gardent plusieurs
  mots ensemble (`"deux mots"`) ;
- `"36"` est un texte : pour un nombre, utilisez `parse_int` ;
- sans rien après le nom du fichier, `args()` vaut `[]`.

Un programme qui reçoit des mots doit donc toujours prévoir le cas où il
n'en reçoit pas. Un `match` sur la liste le fait proprement :

```rythme
nom = match args()
  [premier, ..] -> premier
  [] -> "le monde"
end

print("Bonjour, {nom} !")
```

```sortie
Bonjour, le monde !
```

Lancé sans mot, comme ici, le programme utilise sa valeur par défaut. Avec
un mot :

```text
$ rythme saluer.ry Ada
Bonjour, Ada !
$ rythme saluer.ry "Ada Lovelace" Grace
Bonjour, Ada Lovelace !
```

## Des options

Beaucoup d'outils acceptent des **options** : des mots qui commencent par
`-` ou `--`, comme `--majuscules` ou `--nom=Ada`. `options()` lit les mots
de `args()` et les range dans un enregistrement :

- `--nom=valeur` donne le champ `nom` avec le texte `valeur` ;
- `--nom` et `-n` donnent `true` ;
- un `-` dans un nom devient `_` : `--dry-run` donne `dry_run` ;
- les autres mots, dans l'ordre, vont dans le champ `args`.

Des options peuvent être mal écrites. `options()` renvoie donc un
[Result](../debuter/glossaire.md#result) : `Ok(enregistrement)` ou
`Err(message)`. Pour essayer sans terminal, donnez-lui une liste de mots :

```rythme
print(options(["--majuscules", "--nom=Ada", "notes.txt", "-v"]))
print(options(["--dry-run"]))
print(options(["--nom=Ada", "--nom=Bob"]))
```

```sortie
Ok({args: ["notes.txt"], majuscules: true, nom: "Ada", v: true})
Ok({args: [], dry_run: true})
Err("l'option `nom` est donnée deux fois")
```

Dans un vrai outil, on ouvre le `Result` avec `match` :

```rythme norun
match options()
  Ok(opts) -> print(opts)
  Err(message) -> print("Erreur : {message}")
end
```

```text
$ rythme options.ry --majuscules --nom=Ada notes.txt -v
{args: ["notes.txt"], majuscules: true, nom: "Ada", v: true}
$ rythme options.ry -abc
Erreur : `-abc` n'est pas une option que Rythme comprend : écrivez `--abc` pour un nom long, ou `-a -b` pour plusieurs options d'une lettre
```

## Une option qui peut manquer

Une option absente n'est pas un champ de l'enregistrement. Pour la lire
sans risque :

- `has(opts, "nom")` dit si elle est là (un Bool) ;
- `get(opts, "nom")` donne une [Option](../debuter/glossaire.md#option) :
  `Some(valeur)` ou `None` ;
- `unwrap_or(get(opts, "nom"), defaut)` donne sa valeur, ou une valeur par
  défaut.

```rythme
fn saluer(mots)
  opts = options(mots)?
  nom = unwrap_or(get(opts, "nom"), "le monde")
  message = "Bonjour, {nom} !"
  Ok(if has(opts, "majuscules") then upper(message) else message)
end

print(saluer([]))
print(saluer(["--nom=Ada"]))
print(saluer(["--nom=Ada", "--majuscules"]))
```

```sortie
Ok("Bonjour, le monde !")
Ok("Bonjour, Ada !")
Ok("BONJOUR, ADA !")
```

Recevoir la liste de mots en paramètre, plutôt que d'appeler `args()` dans
la fonction, permet de l'essayer avec n'importe quels mots, et de la
[tester](tests.md). Le programme l'appelle avec `saluer(args())`.

## Dire si tout s'est bien passé : exit

Chaque programme se termine avec un **code de sortie**, un nombre que le
terminal et les scripts peuvent lire. La convention est simple : 0 veut dire
« tout va bien », un autre nombre veut dire « problème ».

Rythme sort avec 0 à la fin normale du programme, et avec 1 après une
erreur. `exit(code)` arrête le programme **tout de suite**, avec le code de
votre choix, de 0 à 255. Dans un terminal, `echo $?` affiche le code de la
dernière commande :

```rythme norun
print("Je m'arrête avec le code 3.")
exit(3)
print("jamais affiché")
```

```text
$ rythme fin.ry
Je m'arrête avec le code 3.
$ echo $?
3
```

Un script qui enchaîne plusieurs commandes peut ainsi savoir si votre outil
a réussi, et s'arrêter sinon.

## Lire au clavier : input

`input(invite)` affiche l'invite, attend que l'utilisateur tape une ligne
et appuie sur Entrée, puis renvoie cette ligne, sous forme de texte. Le
programme dépend de ce que tape la personne : on le montre donc lancé dans
un vrai terminal.

```rythme norun
texte = input("Votre âge ? ")

match parse_int(texte)
  Ok(age) -> print("Dans 10 ans, vous aurez {age + 10} ans.")
  Err(message) -> print("Oups : {message}")
end
```

```text
$ rythme age.ry
Votre âge ? 36
Dans 10 ans, vous aurez 46 ans.
$ rythme age.ry
Votre âge ? trente
Oups : "trente" n'est pas un nombre entier
```

Ce que tape l'utilisateur est toujours un texte : `parse_int` le transforme
en nombre, et son `Result` prévoit une saisie qui n'en est pas un.

## Lire tout ce qui arrive : input_lines

Dans un terminal, `<` envoie un fichier au programme comme s'il était tapé
au clavier, et `|` envoie la sortie d'une autre commande. `input_lines()`
lit **toutes** les lignes reçues ainsi, jusqu'à la fin, dans un `Result` :

```rythme norun
match input_lines()
  Ok(lignes) -> print(len(lignes), "ligne(s) lue(s) :", lignes)
  Err(message) -> print("Erreur : {message}")
end
```

```text
$ rythme lignes.ry < noms.txt
3 ligne(s) lue(s) : ["Ada", "Grace", "Alan"]
$ printf 'pomme\npoire\n' | rythme lignes.ry
2 ligne(s) lue(s) : ["pomme", "poire"]
```

Lancé sans `<` ni `|`, le programme attend que vous tapiez des lignes ;
Ctrl-D (Ctrl-Z puis Entrée sous Windows) dit « c'est fini ».

## Les variables d'environnement : env

Le système garde des **variables d'environnement** : des réglages nommés,
comme `HOME` (votre dossier personnel). `env(nom)` en lit une, sous forme
d'Option, car elle peut ne pas exister :

```rythme
couleur = unwrap_or(env("COULEUR"), "bleu")

print("Couleur : {couleur}")
```

```sortie
Couleur : bleu
```

Dans un terminal, on peut donner une valeur à une variable juste pour une
commande, en l'écrivant devant :

```text
$ COULEUR=vert rythme couleur.ry
Couleur : vert
```

## Un outil complet : compter les mots

Assemblons tout. `compter.ry` lit un fichier dont le nom est donné en
argument, et affiche son nombre de lignes, de mots et de caractères. Avec
l'option `--frequents`, il montre aussi les trois mots les plus fréquents.

Pour que l'exemple tourne tel quel, il écrit d'abord son propre fichier,
`poeme.txt`, et prend des mots par défaut quand `args()` est vide :

```rythme
write_file("poeme.txt", "Le chat dort.\nLe chien dort aussi.\nLe soleil brille.")

## Les lignes, les mots et les caractères d'un texte.
fn compter(texte) = {
  lignes: len(lines(texte)),
  mots: len(words(texte)),
  caracteres: len(texte),
}

## Les `n` mots les plus fréquents, avec leur nombre.
fn frequents(texte, n)
  words(texte)
    |> map(lower)
    |> group_by(fn(m) = m)
    |> entries
    |> map(fn([mot, liste]) = [mot, len(liste)])
    |> sort_by(fn([mot, nombre]) = [-nombre, mot])
    |> take(n)
end

fn lancer(mots)
  opts = options(mots)?
  chemin = first(opts.args) |> ok_or("donnez un fichier : rythme compter.ry FICHIER")?
  texte = read_file(chemin)?
  c = compter(texte)
  print("{chemin} : {c.lignes} ligne(s), {c.mots} mot(s), {c.caracteres} caractère(s)")
  if has(opts, "frequents") then
    frequents(texte, 3) |> each(fn([mot, nombre]) = print("  {mot} : {nombre}"))
  else ()
  Ok(())
end

mots = if is_empty(args()) then ["--frequents", "poeme.txt"] else args()

match lancer(mots)
  Ok(_) -> ()
  Err(message) -> do
    print("Erreur : {message}")
    exit(1)
  end
end
```

```sortie
poeme.txt : 3 ligne(s), 10 mot(s), 52 caractère(s)
  le : 3
  dort : 2
  aussi : 1
```

Comment il est construit :

1. `compter` et `frequents` ne font que calculer : elles ne savent rien du
   terminal, et se testent facilement ;
2. `lancer` fait le travail avec le monde extérieur. Chaque étape qui peut
   échouer (les options, le nom du fichier, la lecture) est suivie de `?` :
   à la première erreur, `lancer` renvoie le `Err` ;
3. `ok_or` transforme l'Option de `first` en `Result`, avec un message
   clair ;
4. tout en bas, un seul `match` affiche l'erreur et sort avec le code 1.

Dans la version de tous les jours, supprimez la ligne `write_file(...)` et
donnez directement `args()` à `lancer` : `match lancer(args())`.

```text
$ rythme compter.ry poeme.txt
poeme.txt : 3 ligne(s), 10 mot(s), 52 caractère(s)
$ rythme compter.ry absent.txt
Erreur : fichier introuvable : absent.txt
$ echo $?
1
```

## Erreurs fréquentes

### Prendre args()[0] sans vérifier

```rythme norun
fichier = args()[0]
print(fichier)
```

```text
erreur[E007]: l'indice 0 est hors limites : la liste est vide
 --> main.ry:1:18
  |
1 | fichier = args()[0]
  |                  ^ aucun élément ici
  |
  = aide: pour obtenir une Option au lieu d'une erreur, utilisez `get(x, 0)`
  = en savoir plus : `rythme explain E007`
```

Lancé sans argument, le programme s'arrête. Utilisez un `match` sur
`args()`, ou `first(args())`, qui donne une Option.

### Oublier que options() renvoie un Result

```rythme norun
opts = options(["--nom=Ada"])
print(opts.nom)
```

```text
erreur[E010]: aucune variante de `Result` n'a de champ `nom`
 --> main.ry:2:12
  |
2 | print(opts.nom)
  |            ^^^ champ inconnu
  |
  = note: leurs champs sont : `value`, `error`
  = aide: une valeur du type `Result` peut être une autre variante : utilisez `match` pour traiter chaque cas
  = en savoir plus : `rythme explain E010`
```

`opts` vaut `Ok({...})`, une boîte. Ouvrez-la avec `match`, ou avec `?`
dans une fonction : `opts = options(mots)?`.

### Donner un code de sortie trop grand

```rythme norun
exit(300)
```

```text
erreur[E023]: un code de sortie va de 0 à 255, pas 300
 --> main.ry:1:1
  |
1 | exit(300)
  | ^^^^^^^^^
  |
  = en savoir plus : `rythme explain E023`
```

Les systèmes n'acceptent que les codes de 0 à 255. Gardez 0 pour le succès
et de petits nombres (1, 2...) pour les problèmes.

## Testez-vous

**Question 1 :** que donne `options(["--verbose", "rapport.txt"])` ?

> **Réponse**
>
> `Ok({args: ["rapport.txt"], verbose: true})`. `--verbose` devient un
> champ qui vaut `true`, et le mot qui n'est pas une option va dans `args`.

**Question 2 :** pourquoi `env("HOME")` renvoie-t-il une Option ?

> **Réponse**
>
> Parce que la variable peut ne pas exister. `Some(texte)` quand elle
> existe, `None` sinon : `unwrap_or` donne alors une valeur par défaut.

**Question 3 :** quel code de sortie un script doit-il attendre d'un outil
qui a réussi ?

> **Réponse**
>
> 0. Rythme sort avec 0 à la fin normale du programme, avec 1 après une
> erreur, et avec le code donné à `exit(code)`.

## En résumé

- `args()` donne la liste des mots écrits après le nom du fichier ; elle
  peut être vide.
- `options()` range les `--nom=valeur` et les `--drapeau` dans un
  enregistrement, dans un `Result` ; `get` et `has` lisent une option qui
  peut manquer.
- `input(invite)` lit une ligne au clavier, `input_lines()` tout ce qui
  arrive par `<` ou `|`, `env(nom)` une variable d'environnement.
- `exit(code)` arrête le programme : 0 pour le succès, un autre code pour
  un problème.
- Séparez les fonctions qui calculent de celle qui parle au terminal : les
  premières se testent sans terminal.

Suite : [le vérificateur et les messages d'erreur](verificateur.md).
Comparez avec la ligne de commande en [Ruby](../ruby/ligne-de-commande.md) et
en [Rust](../rust/ligne-de-commande.md).
