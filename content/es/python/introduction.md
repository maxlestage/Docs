# Python: introducción e instalación

Esta página presenta Python, explica cómo instalarlo y te guía para escribir y ejecutar tu primer programa.

> **Objetivos:**
>
> - sabrás decir qué es Python y para qué sirve;
> - sabrás instalar Python en Windows, macOS o Linux;
> - sabrás probar una línea de código en la consola `>>>`;
> - sabrás escribir un archivo `.py` y ejecutarlo desde el terminal.

## ¿Qué es Python?

Python es un lenguaje de programación creado por Guido van Rossum, en los Países Bajos, en 1991.
Su nombre viene de un programa cómico inglés, los *Monty Python*, no de la serpiente.

Un **lenguaje de programación** es un idioma preciso que el ordenador sabe seguir.
Un [programa](../debuter/glossaire.md#programa) es un texto escrito en ese idioma:
una serie de órdenes, como una receta de cocina.

```python
print("Hola")
print("Hola")
print("Hola")
```

```sortie
Hola
Hola
Hola
```

`print` («imprimir» en inglés) muestra un texto en la pantalla. Python ejecuta las líneas
en orden, de arriba abajo.

Python es un lenguaje **interpretado**: escribes un archivo y luego lo ejecutas
directamente. Un programa llamado [intérprete](../debuter/glossaire.md#interprete)
lee tu archivo y ejecuta las órdenes una a una. No hay ninguna etapa de preparación
(de «compilación») antes.

## ¿Para qué sirve Python?

Python es uno de los lenguajes más usados del mundo. Se encuentra casi en todas partes:

- **los scripts**: renombrar 500 archivos, ordenar fotos, rellenar una hoja de cálculo;
- **el análisis de datos**: estadísticas, gráficos, tablas de números;
- **la inteligencia artificial**: la mayoría de las herramientas de aprendizaje automático;
- **la web**: sitios y servicios en línea (con Django o Flask);
- **la enseñanza**: suele ser el primer lenguaje que se aprende en el instituto y en la
  universidad.

¿Por qué tiene tanto éxito? El código se lee casi como inglés. Hay pocos símbolos que
recordar. Y Python viene con una gran
[biblioteca](../debuter/glossaire.md#biblioteca) estándar: miles de herramientas
listas para usar, sin instalar nada más.

```python
numeros = [4, 8, 15, 16, 23, 42]
print(sum(numeros))
print(max(numeros))
```

```sortie
108
42
```

Lee este código en voz alta: «la suma de los números», «el máximo de los números».

> **Nota:** este sitio también presenta otros lenguajes: compara con
> [la introducción a Ruby](../ruby/introduction.md) o
> [la introducción a Rust](../rust/introduction.md).

## Instalar Python

Este sitio usa **Python 3.13**. Cualquier versión a partir de la 3.10 basta para casi todos
los ejemplos.

> **Atención:** existe un antiguo Python 2, abandonado desde 2020. Instala
> siempre un Python **3**.

### En Windows

1. Ve al sitio oficial, **python.org**, sección *Downloads*.
2. Descarga el instalador para Windows y ejecútalo.
3. **Marca la casilla «Add python.exe to PATH»** abajo en la primera ventana.
4. Haz clic en *Install Now*.

La casilla «Add to PATH» es importante. El *PATH* es la lista de carpetas donde el
terminal busca los programas. Sin ella, el terminal no encontrará Python.

Windows también instala el **lanzador** `py`. Es un pequeño programa que encuentra el
Python correcto por ti. En Windows, puedes escribir `py` en lugar de `python`.

```sh
py --version
py hola.py
```

### En macOS

macOS no siempre trae un Python reciente. Dos opciones:

- descarga el instalador para macOS en **python.org** y ejecútalo;
- o, si usas Homebrew (un gestor de paquetes para Mac):

```sh
brew install python
```

En macOS, el comando se llama `python3`, con un 3 al final.

### En Linux

Python casi siempre está ya instalado. Si no, usa el gestor de paquetes de
tu sistema: un programa que instala los demás programas por ti.

```sh
sudo apt install python3        # Debian, Ubuntu
sudo dnf install python3        # Fedora
```

Aquí también, el comando se llama `python3`.

> **Recuerda:** según el sistema, el comando es `python3` (macOS, Linux), `python`
> o `py` (Windows). En este sitio escribimos `python3`.

## Comprobar la instalación

Abre un terminal y escribe el siguiente comando. Si el terminal es nuevo para ti,
lee primero [el terminal](../debuter/terminal.md).

```sh
python3 --version
```

Deberías ver un número de versión:

```text
Python 3.13.0
```

Si el terminal responde «command not found» o «no se reconoce» (comando
no encontrado), hay dos causas posibles:

- Python no está instalado, o la casilla «Add to PATH» no estaba marcada;
- el terminal se abrió antes de la instalación: ciérralo y vuelve a abrirlo.

## Probar Python en la consola

Escribe simplemente `python3`, sin nombre de archivo. Entras en la **consola
interactiva**, también llamada [REPL](../debuter/glossaire.md#repl). Escribes una línea
y Python responde enseguida. Es como una calculadora que entiende todo Python.

```text
$ python3
Python 3.13.0 (main, Oct  7 2024, 05:02:14) [GCC 13.2.0] on linux
Type "help", "copyright", "credits" or "license" for more information.
>>> 2 + 3
5
>>> "hola".upper()
'HOLA'
>>> len("buenos días")
11
>>> exit()
```

- El signo `>>>` es el **prompt**: Python espera tu línea.
- La línea de debajo es la respuesta de Python. En la consola no hace falta `print`.
- `exit()` (o Ctrl+D en macOS y Linux, Ctrl+Z y luego Intro en Windows) sale de la
  consola.

> **Consejo:** la consola es perfecta para probar una pequeña idea sin crear un
> archivo. Ten una abierta mientras lees este sitio.

## Tu primer programa

La consola lo olvida todo cuando sales de ella. Para guardar un programa, escríbelo en
un **archivo**. Abre tu [editor de código](../debuter/editeur.md). Crea un archivo
`hola.py` con este contenido:

```python
print("¡Hola, mundo!")
```

```sortie
¡Hola, mundo!
```

- `print(...)` muestra lo que está entre los paréntesis y luego pasa a la línea siguiente.
- El texto va entre comillas `"..."`. Sin comillas, Python creería que es un
  nombre de código.
- El archivo termina en `.py`: así se reconoce un programa de Python.

Un programa puede tener varias líneas. Python las ejecuta en orden.

```python
print("Línea 1")
print("Línea 2")
print(2 + 3)
print("2 + 3")
```

```sortie
Línea 1
Línea 2
5
2 + 3
```

`print(2 + 3)` muestra `5`: sin comillas, Python calcula. Con comillas, muestra el
texto tal cual.

## Ejecutar un programa

En el terminal, ve a la carpeta del archivo (con el comando `cd`) y luego
escribe:

```sh
python3 hola.py
```

El programa se ejecuta y muestra `¡Hola, mundo!`. Después el terminal te devuelve el
control.

> **Recuerda:** un programa de Python es un archivo `.py`. Lo ejecutas con
> `python3 nombre_del_archivo.py` (o `py nombre_del_archivo.py` en Windows).

La guía [el terminal](../debuter/terminal.md) explica cómo abrir un terminal y
cambiar de carpeta.

### La sangría importa

En Python, los espacios al principio de una línea (la **sangría**) tienen un significado.
Muestran qué líneas van juntas, como los apartados de un esquema.
Lo verás en [las condiciones y los bucles](conditions-boucles.md).

```python
for i in range(2):
    print("dentro del bucle")
print("después del bucle")
```

```sortie
dentro del bucle
dentro del bucle
después del bucle
```

Por ahora, recuerda una regla: **no pongas espacios al principio de una línea** sin
motivo.

## Leer un mensaje de error

Cuando Python no entiende tu código, se detiene y muestra un
[error](../debuter/glossaire.md#error). No pasa nada: el mensaje te ayuda.

```text
Traceback (most recent call last):
  File "/home/tu/main.py", line 1, in <module>
    Print("Hola")
    ^^^^^
NameError: name 'Print' is not defined. Did you mean: 'print'?
```

Léelo **de abajo arriba**:

- la última línea da el tipo de error (`NameError`) y la causa («el nombre `Print`
  no está definido»);
- encima, la línea con el fallo, con `^^^^^` debajo del trozo culpable;
- más arriba aún, el archivo y el número de línea (`line 1`).

La palabra *Traceback* significa «rastrear el camino»: Python muestra dónde estaba cuando
ocurrió el error. Los mensajes están en inglés. Algunas palabras aparecen a menudo:
*not defined* (no definido), *invalid* (no válido), *expected* (esperaba), *missing*
(falta), *Did you mean* (¿quisiste decir?).

## Errores frecuentes

### Una mayúscula en print

```python norun
Print("Hola")
```

```text
Traceback (most recent call last):
  File "/home/tu/main.py", line 1, in <module>
    Print("Hola")
    ^^^^^
NameError: name 'Print' is not defined. Did you mean: 'print'?
```

Python distingue entre mayúsculas y minúsculas. `Print` y `print` son dos
nombres distintos. Python incluso propone la ortografía correcta. Corrige con
`print("Hola")`.

### Olvidar los paréntesis

```python norun
print "Hola"
```

```text
  File "/home/tu/main.py", line 1
    print "Hola"
    ^^^^^^^^^^^^
SyntaxError: Missing parentheses in call to 'print'. Did you mean print(...)?
```

Así se escribía en Python 2. En Python 3, `print` es una
[función](../debuter/glossaire.md#funcion): hacen falta paréntesis.
Escribe `print("Hola")`.

### Olvidar la comilla final

```python norun
print("Hola)
```

```text
  File "/home/tu/main.py", line 1
    print("Hola)
          ^
SyntaxError: unterminated string literal (detected at line 1)
```

El texto nunca se cierra (*unterminated string*). Añade la comilla que falta:
`print("Hola")`.

### Un espacio de más al principio

```python norun
print("Línea 1")
  print("Línea 2")
```

```text
  File "/home/tu/main.py", line 2
    print("Línea 2")
IndentationError: unexpected indent
```

La línea 2 empieza con espacios sin motivo (*unexpected indent*: «sangría
inesperada»). Bórralos.

## Ponte a prueba

**Pregunta 1:** ¿qué comando comprueba que Python está instalado?

> **Respuesta**
>
> `python3 --version` (o `py --version` en Windows). Muestra el número de
> versión.

**Pregunta 2:** ¿qué muestra este programa?

```python norun
print("4 * 2")
print(4 * 2)
```

> **Respuesta**
>
> `4 * 2` y luego `8`. Entre comillas, es un texto que se muestra tal cual. Sin
> comillas, Python calcula.

**Pregunta 3:** has escrito `juego.py`. ¿Cómo lo ejecutas?

> **Respuesta**
>
> Ve a su carpeta con `cd` y luego escribe `python3 juego.py`.

## En resumen

- Python es un lenguaje interpretado y legible, usado para scripts, datos,
  IA y la web.
- Instálalo desde python.org (marca «Add to PATH» en Windows) o con tu
  gestor de paquetes.
- `python3 --version` comprueba la instalación; `python3` solo abre la consola `>>>`.
- Un programa es un archivo `.py`, que se ejecuta con `python3 nombre_del_archivo.py`.
- `print(...)` muestra un valor y pasa a la línea siguiente.
- Un mensaje de error se lee de abajo arriba: tipo, causa y luego la línea con el fallo.

Siguiente: [primeros pasos](premiers-pas.md).
