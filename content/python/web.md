# Python pour le web

Cette page explique comment fonctionne le web, comment Python lit des données sur Internet, et comment écrire un petit site ou une API avec Flask, FastAPI ou Django.

> **Objectifs :**
>
> - vous comprendrez ce qu'est une requête et une réponse HTTP ;
> - vous saurez lire une page ou une API avec `urllib` et `requests` ;
> - vous saurez préparer et lire des données JSON échangées avec une API ;
> - vous saurez écrire un petit site avec Flask, et une API avec FastAPI ;
> - vous saurez à quoi sert Django et quand le choisir.

## Le web en quelques lignes

Le web repose sur un dialogue simple entre deux programmes :

- le **[client](../debuter/glossaire.md#client)** (votre navigateur, ou un programme
  Python) envoie une **requête** : « donne-moi la page `/livres` » ;
- le **[serveur](../debuter/glossaire.md#serveur)** renvoie une **réponse** : un code
  (200 = tout va bien), puis le contenu (du HTML, du JSON, une image…).

C'est comme commander au comptoir : vous demandez, le serveur répond, et la
conversation s'arrête là. La langue utilisée s'appelle
[HTTP](../debuter/glossaire.md#http). Chaque ressource a une adresse, une
[URL](../debuter/glossaire.md#url) : `https://exemple.fr/livres?auteur=hugo`.

Pour aller plus loin sur le fonctionnement général, lisez [le web](../debuter/web.md).

### À quoi ressemble HTTP

HTTP est du **texte**. Une requête ressemble à ceci :

```text
GET /livres?auteur=hugo HTTP/1.1
Host: exemple.fr
Accept: application/json
```

Et la réponse :

```text
HTTP/1.1 200 OK
Content-Type: application/json

[{"titre": "Les Misérables"}]
```

La première ligne de la requête donne la **méthode** et le chemin ; les lignes
suivantes sont les **en-têtes** (des informations en plus). La réponse commence par le
**code de statut**, puis ses en-têtes, une ligne vide, et enfin le **corps** (le
contenu).

### Les méthodes et les codes à connaître

| Méthode | Sens |
|---------|------|
| `GET` | lire une ressource |
| `POST` | créer une ressource (envoyer un formulaire, ajouter un élément) |
| `PUT` / `PATCH` | modifier une ressource |
| `DELETE` | supprimer une ressource |

| Code | Sens |
|------|------|
| `200` | OK |
| `201` | créé |
| `301`, `302` | déplacé ailleurs (redirection) |
| `400` | requête mal formée |
| `401`, `403` | pas identifié, pas autorisé |
| `404` | introuvable |
| `500` | erreur dans le serveur |

Les codes `2xx` sont des réussites, les `4xx` des erreurs du client, les `5xx` des
erreurs du serveur.

## Lire une réponse HTTP à la main

Puisque HTTP est du texte, Python peut en fabriquer et en lire avec les outils que
vous connaissez déjà. Cet exemple découpe une réponse en statut, en-têtes et corps,
puis lit le corps [JSON](../debuter/glossaire.md#json).

```python
import json

reponse = (
    "HTTP/1.1 200 OK\r\n"
    "Content-Type: application/json\r\n"
    "Content-Length: 58\r\n"
    "\r\n"
    '{"ville": "Lyon", "temperature": 21.5, "ciel": "dégagé"}'
)

tete, corps = reponse.split("\r\n\r\n", 1)
lignes = tete.split("\r\n")

version, code, message = lignes[0].split(" ", 2)
print("Code :", int(code), message)

entetes = {}
for ligne in lignes[1:]:
    nom, valeur = ligne.split(": ", 1)
    entetes[nom.lower()] = valeur
print("Type :", entetes["content-type"])

meteo = json.loads(corps)
print(f"À {meteo['ville']}, il fait {meteo['temperature']} °C, ciel {meteo['ciel']}.")
```

```sortie
Code : 200 OK
Type : application/json
À Lyon, il fait 21.5 °C, ciel dégagé.
```

En HTTP, les lignes se terminent par `\r\n`, et une ligne vide sépare les en-têtes du
corps. En pratique, vous ne ferez jamais ce découpage vous-même : les bibliothèques
s'en chargent. Mais savoir ce qu'il y a dessous aide beaucoup à comprendre les erreurs.

### Construire l'adresse et le contenu d'une requête

Pour parler à une [API](../debuter/glossaire.md#api), on prépare deux choses : l'URL
(avec ses paramètres) et, pour un `POST`, un corps JSON. `urllib.parse` construit les
URL proprement, en encodant espaces et accents.

```python
import json
from urllib.parse import urlencode, urlparse, parse_qs

parametres = {"q": "crème brûlée", "page": 2}
url = "https://exemple.fr/recettes?" + urlencode(parametres)
print(url)

morceaux = urlparse(url)
print(morceaux.netloc, morceaux.path)
print(parse_qs(morceaux.query))

nouvelle = {"titre": "Tarte aux pommes", "minutes": 45, "ingredients": ["pommes", "pâte"]}
corps = json.dumps(nouvelle, ensure_ascii=False)
print(corps)
print(len(corps.encode("utf-8")), "octets à envoyer")
```

```sortie
https://exemple.fr/recettes?q=cr%C3%A8me+br%C3%BBl%C3%A9e&page=2
exemple.fr /recettes
{'q': ['crème brûlée'], 'page': ['2']}
{"titre": "Tarte aux pommes", "minutes": 45, "ingredients": ["pommes", "pâte"]}
80 octets à envoyer
```

`urlencode` a remplacé `è` par `%C3%A8` et l'espace par `+` : une URL ne peut contenir
que certains caractères.

## Lire le web avec urllib

La bibliothèque standard contient `urllib.request`. `urlopen` envoie une requête `GET`
et renvoie la réponse, qu'on lit comme un fichier.

```python norun
import json
from urllib.request import urlopen

with urlopen("https://api.github.com/repos/python/cpython") as reponse:
    print(reponse.status)
    print(reponse.headers["Content-Type"])
    donnees = json.load(reponse)

print(donnees["full_name"])
print(donnees["language"])
```

```text
200
application/json; charset=utf-8
python/cpython
Python
```

`urllib` marche partout sans rien installer, mais il est un peu verbeux, surtout pour
envoyer des données ou gérer les erreurs.

## Lire le web avec requests

`requests` est le [paquet](../debuter/glossaire.md#paquet) le plus utilisé pour faire
des requêtes HTTP. Il ne fait pas partie de Python : installez-le avec
`pip install requests` (voir [les modules](modules.md)).

```python norun
import requests

reponse = requests.get(
    "https://api.github.com/search/repositories",
    params={"q": "language:python", "per_page": 3},
    timeout=10,
)
print(reponse.status_code)
for depot in reponse.json()["items"]:
    print(depot["full_name"])
```

```text
200
public-apis/public-apis
donnemartin/system-design-primer
vinta/awesome-python
```

- `params` construit la fin de l'URL (`?q=...&per_page=3`) ;
- `timeout=10` abandonne après 10 secondes sans réponse : mettez-le toujours ;
- `.json()` lit directement le corps JSON.

Pour envoyer des données, `requests.post(url, json=donnees)` transforme le dictionnaire
en JSON tout seul. `reponse.raise_for_status()` lève une erreur si le code est `4xx` ou
`5xx`.

```python norun
import requests

reponse = requests.post("https://exemple.fr/api/recettes", json={"titre": "Crêpes"}, timeout=10)
reponse.raise_for_status()
print(reponse.status_code, reponse.json())
```

```text
201 {'id': 17, 'titre': 'Crêpes'}
```

## Un site avec Flask

Flask est un petit **framework web** : une boîte à outils qui reçoit les requêtes et
appelle **vos** fonctions. Installation : `pip install flask`.

On associe une adresse à une fonction avec le décorateur `@app.route` (voir
[les décorateurs](decorateurs.md)). Ce que la fonction renvoie devient la réponse.

```python norun
# app.py
from flask import Flask, jsonify, request

app = Flask(__name__)

LIVRES = [
    {"id": 1, "titre": "Le Petit Prince", "auteur": "Saint-Exupéry"},
    {"id": 2, "titre": "Les Misérables", "auteur": "Hugo"},
]

@app.route("/")
def accueil():
    return "Bienvenue à la bibliothèque !"

@app.route("/bonjour/<nom>")
def bonjour(nom):
    return f"Bonjour, {nom} !"

@app.route("/livres/<int:id>")
def livre(id):
    for l in LIVRES:
        if l["id"] == id:
            return jsonify(l)
    return jsonify({"erreur": "livre introuvable"}), 404

@app.route("/livres", methods=["POST"])
def ajouter():
    nouveau = request.get_json()
    nouveau["id"] = len(LIVRES) + 1
    LIVRES.append(nouveau)
    return jsonify(nouveau), 201
```

Lancez le serveur de développement :

```sh
flask --app app run
```

```text
 * Serving Flask app 'app'
 * Debug mode: off
WARNING: This is a development server. Do not use it in a production deployment. Use a production WSGI server instead.
 * Running on http://127.0.0.1:5000
Press CTRL+C to quit
```

Ouvrez `http://127.0.0.1:5000/bonjour/Alice` dans votre navigateur, ou testez avec
`curl` dans un autre terminal :

```text
$ curl http://127.0.0.1:5000/
Bienvenue à la bibliothèque !
$ curl http://127.0.0.1:5000/bonjour/Alice
Bonjour, Alice !
$ curl http://127.0.0.1:5000/livres/1
{"auteur":"Saint-Exup\u00e9ry","id":1,"titre":"Le Petit Prince"}
$ curl http://127.0.0.1:5000/livres/9
{"erreur":"livre introuvable"}
```

Points clés :

- `<nom>` dans la route devient un paramètre de la fonction ; `<int:id>` le convertit
  en entier ;
- renvoyer une chaîne donne une page de texte ou de HTML ; `jsonify` donne du JSON
  (les accents y sont codés, `\u00e9` pour `é`, comme avec `json.dumps`) ;
- un tuple `(contenu, 404)` choisit le code de statut ;
- `methods=["POST"]` accepte les envois ; `request.get_json()` lit le corps JSON.

> **Astuce :** pour de vraies pages HTML, Flask utilise des **gabarits** (*templates*)
> avec `render_template("page.html", titre=...)`, rangés dans un dossier `templates/`.

## Une API avec FastAPI

FastAPI est spécialisé dans les API JSON. Il s'appuie sur les
[annotations de type](typage.md) : il vérifie lui-même les données reçues et génère
une documentation interactive. Installation : `pip install "fastapi[standard]"`.

```python norun
# api.py
from fastapi import FastAPI, HTTPException
from pydantic import BaseModel

app = FastAPI()

class Livre(BaseModel):
    titre: str
    auteur: str
    annee: int

LIVRES = {1: Livre(titre="Le Petit Prince", auteur="Saint-Exupéry", annee=1943)}

@app.get("/livres/{id}")
def lire_livre(id: int):
    if id not in LIVRES:
        raise HTTPException(status_code=404, detail="livre introuvable")
    return LIVRES[id]

@app.post("/livres", status_code=201)
def ajouter_livre(livre: Livre):
    nouvel_id = len(LIVRES) + 1
    LIVRES[nouvel_id] = livre
    return {"id": nouvel_id, **livre.model_dump()}
```

```sh
fastapi dev api.py
```

```text
$ curl http://127.0.0.1:8000/livres/1
{"titre":"Le Petit Prince","auteur":"Saint-Exupéry","annee":1943}
$ curl http://127.0.0.1:8000/livres/9
{"detail":"livre introuvable"}
$ curl -X POST http://127.0.0.1:8000/livres -H "Content-Type: application/json" \
       -d '{"titre": "Germinal", "auteur": "Zola", "annee": "ancien"}'
{"detail":[{"type":"int_parsing","loc":["body","annee"],"msg":"Input should be a valid integer, unable to parse string as an integer","input":"ancien"}]}
```

La classe `Livre` décrit le JSON attendu. Quand `annee` n'est pas un entier, FastAPI
répond tout seul avec le code `422` et explique le problème. La page
`http://127.0.0.1:8000/docs` permet d'essayer l'API depuis le navigateur.

## Django en bref

Django est un framework **complet** : base de données, comptes utilisateurs, page
d'administration, formulaires, sécurité… tout est inclus. Il convient aux gros sites
avec beaucoup de pages et de données. Il demande plus d'apprentissage que Flask.

```sh
pip install django
django-admin startproject bibliotheque
cd bibliotheque
python manage.py startapp livres
python manage.py runserver
```

Un projet Django est découpé en plusieurs fichiers. Une **vue** (la fonction qui répond) :

```python norun
# livres/views.py
from django.http import HttpResponse

def accueil(request):
    return HttpResponse("Bienvenue à la bibliothèque !")
```

Et l'association adresse → vue :

```python norun
# bibliotheque/urls.py
from django.urls import path
from livres import views

urlpatterns = [
    path("", views.accueil),
]
```

```text
$ curl http://127.0.0.1:8000/
Bienvenue à la bibliothèque !
```

Les données se décrivent avec des **modèles** : des classes que Django transforme en
tables de base de données.

```python norun
# livres/models.py
from django.db import models

class Livre(models.Model):
    titre = models.CharField(max_length=200)
    auteur = models.CharField(max_length=100)
    annee = models.IntegerField()
```

### Lequel choisir ?

| Besoin | Choix conseillé |
|--------|-----------------|
| Lire des données sur Internet | `requests` (ou `urllib` sans installation) |
| Un petit site ou un prototype | Flask |
| Une API JSON moderne, avec vérification des données | FastAPI |
| Un gros site avec base de données, comptes et administration | Django |

## Erreurs fréquentes

### Le paquet n'est pas installé

```python norun
from flask import Flask
```

```text
Traceback (most recent call last):
  File "main.py", line 1, in <module>
    from flask import Flask
ModuleNotFoundError: No module named 'flask'
```

Flask, FastAPI, Django et requests ne font pas partie de Python. Installez-les avec
`pip install flask`, de préférence dans un environnement virtuel (voir
[les modules](modules.md)).

### Une vue qui ne renvoie rien

```python norun
from flask import Flask

app = Flask(__name__)

@app.route("/")
def accueil():
    message = "Bienvenue !"
```

```text
TypeError: The view function for 'accueil' did not return a valid response. The function either returned None or ended without a return statement.
```

La fonction prépare le message mais oublie de le renvoyer. Ajoutez `return message`.

### Une clé absente dans le JSON reçu

```python norun
import json

donnees = json.loads('{"titre": "Germinal"}')
print(donnees["auteur"])
```

```text
Traceback (most recent call last):
  File "main.py", line 4, in <module>
    print(donnees["auteur"])
          ~~~~~~~^^^^^^^^^^
KeyError: 'auteur'
```

Les données qui viennent du réseau peuvent être incomplètes. Utilisez
`donnees.get("auteur", "inconnu")`, ou vérifiez avec `if "auteur" in donnees`.

## Testez-vous

**Question 1 :** que signifient les codes `200`, `404` et `500` ?

> **Réponse**
>
> `200` : tout va bien. `404` : la ressource est introuvable (erreur du client).
> `500` : le serveur a planté (erreur du serveur).

**Question 2 :** dans Flask, comment une route récupère-t-elle le `42` de l'adresse
`/produits/42` sous forme d'entier ?

> **Réponse**
>
> Avec `@app.route("/produits/<int:id>")` : Flask passe `id=42` (un `int`) à la
> fonction.

**Question 3 :** un serveur renvoie le corps `{"nom": "Alice", "points": [12, 8, 15]}`.
Affichez le nom et le total des points.

> **Réponse**
>
> ```python
> import json
>
> corps = '{"nom": "Alice", "points": [12, 8, 15]}'
> joueur = json.loads(corps)
> print(joueur["nom"], sum(joueur["points"]))
> ```
>
> ```sortie
> Alice 35
> ```

## En résumé

- Le web est un dialogue requête → réponse en HTTP : une méthode, une URL, un code de
  statut, des en-têtes et un corps.
- `urllib.request` lit le web sans rien installer ; `requests` est plus confortable
  (mettez toujours un `timeout`).
- Les API échangent du JSON : `json.dumps` pour envoyer, `json.loads` pour lire,
  `urlencode` pour les paramètres d'URL.
- Flask associe des routes à des fonctions ; FastAPI vérifie les données grâce aux
  types ; Django fournit tout pour les gros sites.
- Les données reçues peuvent manquer : utilisez `.get()` et gérez les codes d'erreur.

Suite : [tester son code](tests.md). Comparez avec
[le web en Ruby](../ruby/web.md).
