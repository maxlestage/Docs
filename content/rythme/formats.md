# Données JSON

JSON est le format de texte le plus répandu pour échanger des données : cette page montre comment Rythme l'écrit avec `to_json` et le relit avec `parse_json`.

> **Objectifs :**
>
> - vous saurez ce qu'est le format JSON et à quoi il ressemble ;
> - vous saurez transformer une valeur en JSON, sur une ligne ou joliment présenté ;
> - vous saurez relire du JSON et traiter le `Result` obtenu ;
> - vous saurez lire et écrire un fichier de configuration JSON ;
> - vous saurez comprendre une erreur de JSON grâce à sa ligne et sa colonne.

## Qu'est-ce que JSON ?

JSON (« JavaScript Object Notation ») est une façon d'écrire des données
**sous forme de texte**, que presque tous les langages et tous les sites web
comprennent. C'est la langue commune des programmes, comme l'anglais dans
un aéroport.

Un document JSON ressemble beaucoup aux valeurs de Rythme :

```text
{
  "nom": "Ada",
  "age": 36,
  "langues": ["fr", "en"],
  "actif": true,
  "chef": null
}
```

Il n'y a que six sortes de valeurs :

| JSON | Exemple | En Rythme |
|------|---------|-----------|
| objet | `{"nom": "Ada"}` | un [enregistrement](../debuter/glossaire.md#enregistrement) `{nom: "Ada"}` |
| tableau | `[1, 2, 3]` | une liste `[1, 2, 3]` |
| texte | `"bonjour"` | un texte `"bonjour"` |
| nombre | `36`, `2.5` | un Int ou un Float |
| booléen | `true`, `false` | `true`, `false` |
| null | `null` | `None` |

Dans un objet JSON, les noms des champs sont toujours entre guillemets.

## Écrire du JSON avec to_json

`to_json(valeur)` donne le texte JSON d'une valeur, sur une seule ligne :

```rythme
ada = {nom: "Ada", age: 36, langues: ["fr", "en"], actif: true, chef: None}
print(to_json(ada))
print(to_json([1, 2.5, "x", :ok]))
```

```sortie
{"nom": "Ada", "age": 36, "langues": ["fr", "en"], "actif": true, "chef": null}
[1, 2.5, "x", "ok"]
```

`None` devient `null`, et un symbole comme `:ok` devient le texte `"ok"`.

## Un JSON lisible : l'indentation

Sur une ligne, un gros document devient illisible. Donnez un deuxième
argument, le nombre d'espaces de décalage : chaque élément va alors sur sa
propre ligne.

```rythme
ada = {nom: "Ada", age: 36, langues: ["fr", "en"]}
print(to_json(ada, 2))
```

```sortie
{
  "nom": "Ada",
  "age": 36,
  "langues": [
    "fr",
    "en"
  ]
}
```

On parle d'**indentation** : le décalage montre ce qui est rangé dans quoi.
Deux ou quatre espaces sont les choix habituels (16 au plus).

## Relire du JSON avec parse_json

`parse_json(texte)` fait le chemin inverse : il lit un texte JSON et donne
une valeur Rythme. Mais le texte peut être mal écrit ! `parse_json` donne
donc un [Result](../debuter/glossaire.md#result) : `Ok(valeur)` si tout va
bien, `Err(message)` sinon.

```rythme
print(parse_json("[1, 2, null, true]"))
print(parse_json("42"))
print(parse_json("pas du json"))
```

```sortie
Ok([1, 2, None, true])
Ok(42)
Err("JSON, ligne 1, colonne 1 : on attendait une valeur, mais on trouve `p`")
```

Le tableau devient une liste, et `null` devient `None`.

Pour écrire un objet JSON **dans un texte Rythme**, attention aux accolades :
`{` sert à insérer une valeur (`"{nom}"`). Écrivez `\{` et `\}` pour de
vraies accolades, et `\"` pour un guillemet :

```rythme
texte = "\{\"nom\": \"Ada\", \"age\": 36\}"
print(texte)
print(parse_json(texte))
```

```sortie
{"nom": "Ada", "age": 36}
Ok({nom: "Ada", age: 36})
```

Dans la vraie vie, le JSON vient presque toujours d'un fichier : ces
barres obliques ne sont alors plus nécessaires.

## Utiliser la valeur lue

Pour utiliser la valeur, ouvrez le `Result` avec `match` (voir
[gérer les erreurs](erreurs.md)). Un objet JSON devient un enregistrement :
on lit ses champs avec un point.

```rythme
texte = "\{\"nom\": \"Ada\", \"notes\": [17, 19, 15]\}"

match parse_json(texte)
  Ok(eleve) -> print("{eleve.nom} : moyenne {sum(eleve.notes) / len(eleve.notes)}")
  Err(message) -> print("JSON illisible : {message}")
end
```

```sortie
Ada : moyenne 17.0
```

Un champ dont le nom contient une espace se lit avec des crochets, et un
champ qui peut manquer avec `get` :

```rythme
texte = "\{\"nom complet\": \"Ada Lovelace\"\}"
personne = unwrap_or(parse_json(texte), {})
print(personne["nom complet"], get(personne, "ville"))
```

```sortie
Ada Lovelace None
```

## Écrire et relire un fichier JSON

Pour garder des données d'une exécution à l'autre, on écrit le JSON dans un
fichier avec `write_file`, et on le relit avec `read_file` (voir
[fichiers](fichiers.md)). Chaque étape peut échouer : `?` transmet l'erreur.

```rythme
fn sauver(chemin, valeur) = write_file(chemin, to_json(valeur, 2))

fn charger(chemin)
  texte = read_file(chemin)?
  parse_json(texte)
end

eleves = [{nom: "Ada", note: 17}, {nom: "Alan", note: 12}]
sauver("eleves.json", eleves)
print(read_file("eleves.json") |> unwrap)

match charger("eleves.json")
  Ok(liste) -> print("{len(liste)} élèves relus, dont {liste[0].nom}")
  Err(e) -> print("Problème : {e}")
end
print(charger("absent.json"))
```

```sortie
[
  {
    "nom": "Ada",
    "note": 17
  },
  {
    "nom": "Alan",
    "note": 12
  }
]
2 élèves relus, dont Ada
Err("fichier introuvable : absent.json")
```

Un seul `Result` sort de `charger` : il dit soit que tout va bien, soit
quelle étape a échoué (fichier absent ou JSON mal écrit).

## Un petit fichier de configuration

Beaucoup de programmes lisent leurs réglages dans un fichier
`config.json`. L'utilisateur n'y écrit que ce qu'il veut changer ; le
programme complète avec des valeurs par défaut.

Pour l'exemple, le programme crée d'abord le fichier :

```rythme
write_file("config.json", to_json({port: 3000, langue: "en"}, 2))

defauts = {port: 8080, langue: "fr", debug: false}

fn lire_config(chemin)
  texte = read_file(chemin)?
  choix = parse_json(texte)?
  Ok({..defauts, ..choix})
end

match lire_config("config.json")
  Ok(config) -> print("port {config.port}, langue {config.langue}, debug {config.debug}")
  Err(e) -> print("configuration illisible : {e}")
end
```

```sortie
port 3000, langue en, debug false
```

`{..defauts, ..choix}` recopie d'abord les valeurs par défaut, puis les
remplace par celles du fichier. `debug` n'était pas dans le fichier : il
garde sa valeur par défaut.

> **Astuce :** pour une configuration qui doit toujours marcher, remplacez
> l'erreur par les valeurs par défaut :
> `config = unwrap_or(lire_config("config.json"), defauts)`.

## Les erreurs de JSON

Un fichier JSON écrit à la main contient vite une faute : une virgule en
trop, un guillemet oublié. Le message de `Err` donne la **ligne** et la
**colonne** du problème, comme un plan avec une croix « vous êtes ici » :

```rythme
texte = "\{
  \"nom\": \"Ada\",
  \"age\": 36,
\}"
print(parse_json(texte))
print(parse_json("[1, 2"))
```

```sortie
Err("JSON, ligne 4, colonne 1 : on attendait un nom entre guillemets, mais on trouve `}`")
Err("JSON, ligne 1, colonne 6 : on attendait `,` ou `]`, mais le texte se termine")
```

Ligne 4, colonne 1 : l'accolade arrive alors qu'on attendait un nouveau
champ, à cause de la virgule en trop après `36`. En JSON, le dernier
élément n'a jamais de virgule après lui.

## Erreurs fréquentes

### Écrire des accolades sans barre oblique

```rythme norun
texte = "{"nom": "Ada"}"
print(parse_json(texte))
```

```text
erreur[E001]: `}` attendu, mais trouvé `:`
 --> main.ry:1:16
  |
1 | texte = "{"nom": "Ada"}"
  |                ^ `}` attendu ici
  |
  = aide: dans un texte, `{` commence une valeur insérée : pour écrire des accolades, utilisez `\{` et `\}`, comme "\{\"a\": 1\}"
  = en savoir plus : `rythme explain E001`
```

Dans un texte Rythme, `{` insère une valeur et `"` termine le texte.
Écrivez `"\{\"nom\": \"Ada\"\}"`, ou lisez le JSON depuis un fichier.

### Lire un champ du Result sans l'ouvrir

```rythme norun
texte = "\{\"nom\": \"Ada\"\}"
donnees = parse_json(texte)
print(donnees.nom)
```

```text
erreur[E010]: aucune variante de `Result` n'a de champ `nom`
 --> main.ry:3:15
  |
3 | print(donnees.nom)
  |               ^^^ champ inconnu
  |
  = note: leurs champs sont : `value`, `error`
  = aide: une valeur du type `Result` peut être une autre variante : utilisez `match` pour traiter chaque cas
  = en savoir plus : `rythme explain E010`
```

`parse_json` donne `Ok({nom: "Ada"})`, pas l'enregistrement lui-même.
Ouvrez-le avec `match`, avec `?` dans une fonction, ou avec `unwrap_or`.

### Forcer avec unwrap un JSON mal écrit

```rythme norun
texte = "[1, 2, 3,]"
liste = unwrap(parse_json(texte))
print(liste)
```

```text
erreur[E013]: `unwrap` a trouvé une erreur : JSON, ligne 1, colonne 10 : on attendait une valeur, mais on trouve `]`
 --> main.ry:2:9
  |
2 | liste = unwrap(parse_json(texte))
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = aide: traitez l'erreur avec `match`, transmettez-la avec `?`, ou donnez une valeur par défaut : `unwrap_or(x, defaut)`
  = en savoir plus : `rythme explain E013`
```

La virgule après `3` est de trop. Et comme un fichier peut toujours être
abîmé, traitez le cas `Err` au lieu d'utiliser `unwrap`.

## Testez-vous

**Question 1 :** que devient `{nom: "Ada", chef: None}` avec `to_json` ?

> **Réponse**
>
> `{"nom": "Ada", "chef": null}` : les noms des champs sont mis entre
> guillemets, et `None` devient `null`.

**Question 2 :** pourquoi `parse_json` donne-t-il un `Result` et pas
directement la valeur ?

> **Réponse**
>
> Parce que le texte peut ne pas être du JSON correct. Le `Result` oblige à
> prévoir ce cas : `Ok(valeur)` ou `Err(message)`.

**Question 3 :** que fait `to_json(valeur, 4)` ?

> **Réponse**
>
> Il écrit le JSON avec un élément par ligne, décalé de 4 espaces par
> niveau, pour qu'un humain le lise facilement.

## En résumé

- JSON écrit des données en texte : objets, tableaux, textes, nombres,
  booléens et `null`.
- `to_json(valeur)` écrit sur une ligne, `to_json(valeur, 2)` joliment
  indenté.
- `parse_json(texte)` donne un `Result` ; les objets deviennent des
  enregistrements, les tableaux des listes, `null` devient `None`.
- `read_file`, `parse_json` et `?` se combinent pour charger un fichier ;
  `{..defauts, ..choix}` complète une configuration.
- Une erreur de JSON indique sa ligne et sa colonne.

Suite : [les expressions régulières](regex.md).
