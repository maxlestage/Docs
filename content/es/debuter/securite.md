# Las bases de la seguridad

Un programa debe proteger los datos que se le confían: esta página te enseña los buenos hábitos para las contraseñas, los secretos, la base de datos y todo lo que escribe el usuario.

> **Objetivos:**
>
> - elegir una buena contraseña y no guardarla nunca en claro;
> - calcular un hash con `hashlib` y una sal;
> - mantener los secretos fuera del código, en variables de entorno;
> - impedir la inyección SQL con `?`;
> - comprobar todo lo que escribe el usuario.

## Por qué es asunto tuyo

Cierras tu puerta con llave, aunque nadie quiera robarte. En programación
pasa lo mismo: cada día, unos robots prueban millones de contraseñas en
todos los sitios. Unos pocos hábitos sencillos evitan casi todos los
problemas.

## Una buena contraseña

Una contraseña corta se adivina rápido, aunque sea complicada. Un
ordenador prueba todas las combinaciones: cuantas más hay, más tarda.
Comparemos:

```python
letras = 26 ** 8           # 8 letras minúsculas
frase = 2000 ** 5          # 5 palabras elegidas entre 2000 palabras comunes
print(f"8 letras   : {letras:,} combinaciones")
print(f"5 palabras : {frase:,} combinaciones")
print(f"La frase es {frase // letras:,} veces más difícil de adivinar.")
```

```sortie
8 letras   : 208,827,064,576 combinaciones
5 palabras : 32,000,000,000,000,000 combinaciones
La frase es 153,236 veces más difícil de adivinar.
```

`**` significa «potencia» (`26 ** 8`: 26 multiplicado 8 veces por sí
mismo) y `:,` separa los miles. Así que recuerda:

- una **frase** de varias palabras (`caballo-farol-tormenta-tulipan-violin`)
  es más sólida y más fácil de recordar que `Xk7$p2!a`;
- **una contraseña distinta para cada sitio**: si roban un sitio, los
  demás siguen protegidos;
- un **gestor de contraseñas** (Bitwarden, KeePassXC…) las recuerda por
  ti. Solo tienes que recordar una frase: la del gestor.

Prueba: cambia `2000 ** 5` por `2000 ** 3` (3 palabras). La frase sigue
siendo más sólida que 8 letras, pero mucho menos que con 5 palabras.

> **Recuerda:** una frase larga gana a una palabra corta y rara.

## No guardar nunca una contraseña en claro

¿Tu programa gestiona cuentas? **Nunca** debe guardar la contraseña en sí:
si roban la base, todo sería legible.

En su lugar se guarda un **hash** (una huella). Es como una huella
dactilar: reconoce a una persona, pero no se puede reconstruir a la
persona a partir de la huella.

### Un hash con hashlib

El [módulo](glossaire.md#modulo) `hashlib` calcula un hash:

```python
import hashlib

for palabra in ["sol", "sol", "Sol"]:
    huella = hashlib.sha256(palabra.encode()).hexdigest()
    print(f"{palabra:4} → {huella[:16]}…")
```

```sortie
sol  → 8db59feb4d217f26…
sol  → 8db59feb4d217f26…
Sol  → 361847a8cfe5a8dd…
```

| Palabra | Hash (inicio)      | Lo que se observa                           |
|---------|--------------------|---------------------------------------------|
| `sol`   | `8db59feb4d217f26` | siempre el mismo para la misma palabra      |
| `sol`   | `8db59feb4d217f26` | idéntico: así que se puede **comparar**     |
| `Sol`   | `361847a8cfe5a8dd` | cambia una sola letra y todo cambia         |

`.encode()` convierte el texto en [bytes](glossaire.md#byte), lo único
que acepta `hashlib`. `[:16]` solo muestra el inicio.

### Añadir una sal

Queda un problema: los piratas tienen listas de hashes ya preparadas
(`sol` → `8db59feb…`). La solución: una **sal**, un pequeño texto que se
añade a la contraseña, distinto para cada persona y guardado junto al
hash. También usamos `pbkdf2_hmac`, una función lenta a propósito, para
desanimar a los piratas.

```python
import hashlib
import hmac

def huella(contrasena, sal):
    return hashlib.pbkdf2_hmac("sha256", contrasena.encode(), sal, 100_000)

# En la vida real, la sal es aleatoria: sal = os.urandom(16)
# Aquí es fija para que el resultado sea siempre el mismo.
sal = b"sal-de-alicia"
guardada = huella("caballo-farol-tormenta", sal)
print("Guardado en la base:", guardada.hex()[:16], "…")

def comprobar(intento):
    return hmac.compare_digest(huella(intento, sal), guardada)

print("caballo-farol-tormenta:", comprobar("caballo-farol-tormenta"))
print("caballo-farol         :", comprobar("caballo-farol"))
```

```sortie
Guardado en la base: 7396e9a247262d12 …
caballo-farol-tormenta: True
caballo-farol         : False
```

Sigamos la comprobación paso a paso:

| Paso | Lo que hace el programa                              | Resultado      |
|------|------------------------------------------------------|----------------|
| 1    | la persona escribe su contraseña                     | `intento`      |
| 2    | se calcula el hash de `intento` con la misma sal     | un hash        |
| 3    | se compara con el hash guardado                      | `True`/`False` |

La contraseña en claro **nunca** se guarda. `hmac.compare_digest` compara
sin que un pirata pueda medir el tiempo que tarda: prefiérelo a `==`.

Prueba: cambia la sal por `b"sal-de-bob"`. El hash cambia, la comprobación funciona.

> **Recuerda:** se guarda un hash con sal, nunca la contraseña.

## Los secretos fuera del código

Un **secreto** (contraseña de la base, clave de [API](glossaire.md#api)…)
nunca se escribe en el código: el código acaba en [GitHub](git.md).

Se pone en una **variable de entorno**: una etiqueta que el
[sistema](glossaire.md#sistema-operativo) le da al programa al arrancar.

```python
import os

clave = os.environ.get("CLAVE_CLIMA")
if clave is None:
    print("Error: la variable CLAVE_CLIMA no está definida.")
else:
    print("Clave encontrada, tiene", len(clave), "caracteres.")
```

```sortie
Error: la variable CLAVE_CLIMA no está definida.
```

Para definir la variable, en el [terminal](terminal.md), antes de ejecutar:

```sh
export CLAVE_CLIMA="mi-clave-secreta"      # macOS y Linux
set CLAVE_CLIMA=mi-clave-secreta           # Windows (símbolo del sistema)
python3 clima.py
```

```text
Clave encontrada, tiene 16 caracteres.
```

Muchos proyectos guardan estas variables en un archivo `.env`. Este
archivo **nunca** debe ir a Git: escribe `.env` en tu
[`.gitignore`](git.md#ignorar-archivos-gitignore).

> **Recuerda:** el código dice **dónde** encontrar el secreto, nunca
> **cuál** es el secreto.

## La inyección SQL

En [las bases de datos](bases-de-donnees.md), viste que no hay que pegar
texto dentro de una orden SQL. Aquí tienes el porqué.

### El código vulnerable

Un sitio comprueba un nombre de usuario pegando lo que se escribe dentro
de la orden SQL. Un pirata escribe un texto raro en lugar de un nombre:

```python
import sqlite3

con = sqlite3.connect(":memory:")
con.execute("CREATE TABLE cuentas (nombre TEXT, secreto TEXT)")
con.executemany("INSERT INTO cuentas VALUES (?, ?)",
                [("alice", "código 1234"), ("bob", "código 9876")])

entrada = "x' OR '1'='1"
consulta = "SELECT nombre, secreto FROM cuentas WHERE nombre = '" + entrada + "'"
print(consulta)
print(con.execute(consulta).fetchall())
```

```sortie
SELECT nombre, secreto FROM cuentas WHERE nombre = 'x' OR '1'='1'
[('alice', 'código 1234'), ('bob', 'código 9876')]
```

Mira la orden que se ha fabricado. El pirata cerró el apóstrofo y luego
añadió `OR '1'='1'`, que es **siempre verdadero**. Así que la base devuelve
**todas** las cuentas, con sus secretos.

```text
WHERE nombre = 'x' OR '1'='1'
      └────┬─────┘    └──┬──┘
         falso   siempre verdadero   →  falso O verdadero = verdadero para cada fila
```

### La corrección: los `?`

Con un `?`, la base recibe la orden y la entrada **por separado**. La
entrada sigue siendo un simple texto, nunca SQL:

```python
import sqlite3

con = sqlite3.connect(":memory:")
con.execute("CREATE TABLE cuentas (nombre TEXT, secreto TEXT)")
con.executemany("INSERT INTO cuentas VALUES (?, ?)",
                [("alice", "código 1234"), ("bob", "código 9876")])

entrada = "x' OR '1'='1"
resultado = con.execute("SELECT nombre, secreto FROM cuentas WHERE nombre = ?", (entrada,))
print(resultado.fetchall())
```

```sortie
[]
```

La base busca un usuario que se llame de verdad `x' OR '1'='1`: no hay
ninguno. Lista vacía, ataque fallido.

Prueba: cambia `entrada` por `"alice"`. Obtienes
`[('alice', 'código 1234')]`.

> **Recuerda:** nunca `+` ni f-string para fabricar SQL: usa `?`.

## No fiarse nunca de lo que te envían

Todo lo que viene de fuera puede ser falso, por error o a propósito: un
formulario, un archivo, una URL. Compruébalo **antes** de usarlo. Es el
portero a la entrada de una sala: mira cada entrada.

```python
def leer_edad(texto):
    texto = texto.strip()
    if not texto.isdigit():
        return "rechazado: no es un número"
    edad = int(texto)
    if edad > 130:
        return "rechazado: edad imposible"
    return f"aceptado: {edad} años"

for entrada in ["42", " 7 ", "-3", "doce", "999", "'; DROP TABLE"]:
    print(f"{entrada!r:16} {leer_edad(entrada)}")
```

```sortie
'42'             aceptado: 42 años
' 7 '            aceptado: 7 años
'-3'             rechazado: no es un número
'doce'           rechazado: no es un número
'999'            rechazado: edad imposible
"'; DROP TABLE"  rechazado: no es un número
```

`strip()` quita los espacios de alrededor, `isdigit()` comprueba que solo
hay cifras, `!r` muestra las comillas para ver los espacios.

Prueba: añade `"130"` a la lista. Se acepta: el límite es «más de 130».

> **Recuerda:** se comprueba la **forma** (¿un número?) y el **valor** (¿no más de 130?).

## HTTPS en dos frases

`https://` (con una **s**) cifra lo que circula entre el navegador y el
[servidor](glossaire.md#servidor): nadie en el camino puede leer una
contraseña o un número de tarjeta. Nunca envíes datos personales a una
dirección que empiece por `http://` sin **s**.

## Las actualizaciones

Cada semana se descubren fallos de seguridad; las actualizaciones los
corrigen. Actualiza tu sistema, tu navegador, tu lenguaje y los
[paquetes](glossaire.md#paquete) de tus proyectos.

## Las 5 reglas que recordar

```text
1. Una frase de contraseña larga, distinta para cada sitio, en un gestor.
2. Nunca una contraseña en claro: un hash con sal.
3. Nunca un secreto en el código ni en Git: variables de entorno.
4. Nunca SQL fabricado con +: siempre ?.
5. Nunca fiarse de una entrada: se comprueba, y se actualiza todo.
```

## Errores frecuentes

### Un secreto enviado a Git

Escribes una clave en `config.py`, haces un commit y luego borras el
archivo en un segundo commit. Crees que estás tranquilo:

```sh
git show HEAD~1:config.py
```

```text
CLAVE_API = "sk-secret-123456"
```

Este comando muestra `config.py` tal como estaba en el commit anterior: la
clave sigue ahí. La corrección: considera que la clave está **robada**,
**revócala** (anúlala) en el sitio que te la dio y guarda la nueva en una
variable de entorno.

### Comparar la contraseña escrita con el hash

Sí que se guarda un hash… pero se compara con el texto escrito:

```python
import hashlib

guardada = hashlib.sha256("sol".encode()).hexdigest()
intento = "sol"

if intento == guardada:
    print("¡Bienvenido!")
else:
    print("Contraseña incorrecta.")
```

```sortie
Contraseña incorrecta.
```

Se rechaza la contraseña correcta: se compara `sol` con `8db59feb…`. Hay
que calcular el hash del intento y luego comparar dos hashes
(`comprobar`).

### Olvidar .encode()

```python norun
import hashlib

huella = hashlib.sha256("sol")
```

```text
Traceback (most recent call last):
  File "main.py", line 3, in <module>
    huella = hashlib.sha256("sol")
TypeError: Strings must be encoded before hashing
```

`hashlib` quiere bytes, no texto. Escribe
`hashlib.sha256("sol".encode())`.

## Ponte a prueba

**1.** ¿Por qué guardar un hash en lugar de la contraseña?

> **Respuesta**
>
> Si roban la base, el pirata solo tiene hashes, que no le sirven de nada.

**2.** Una clave ya se ha enviado a GitHub. ¿Basta con borrarla y hacer otro commit?

> **Respuesta**
>
> No, sigue en el historial: revócala y crea una nueva.

**3.** ¿Cuál es el peligro de `"… WHERE nombre = '" + entrada + "'"`?

> **Respuesta**
>
> La inyección SQL: la entrada puede contener SQL (`x' OR '1'='1`). Escribe
> `WHERE nombre = ?` y pasa `(entrada,)` aparte.

## En resumen

- Una frase de contraseña larga, única para cada sitio, guardada en un gestor.
- Se guarda un hash con sal (`hashlib.pbkdf2_hmac`), nunca la contraseña.
- Los secretos van en variables de entorno, nunca en el código ni en Git.
- Los `?` impiden la inyección SQL; se comprueba cada entrada antes de usarla.
- `https://` cifra los intercambios; las actualizaciones tapan los fallos.

Siguiente: [aprender bien a programar](apprendre.md).
