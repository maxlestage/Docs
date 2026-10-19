# Rythme: introducción e instalación

Rythme es un pequeño lenguaje puramente funcional, con sabor a Ruby, pensado para aprender a programar sin perderse: esta página explica qué es, cómo instalarlo y cómo ejecutar tu primer programa.

> **Objetivos:**
>
> - sabrás decir qué es Rythme y qué lo hace diferente;
> - sabrás instalar Rythme, o probarlo sin instalar nada;
> - sabrás probar una línea de código en el modo interactivo;
> - sabrás escribir un archivo `.ry`, comprobarlo y ejecutarlo;
> - sabrás leer un primer mensaje de error.

## Qué es Rythme

Un **lenguaje de programación** es un idioma preciso que el ordenador sabe
seguir. Un **programa** es un texto escrito en ese idioma: una serie de
órdenes, como una receta de cocina.

Rythme es uno de esos lenguajes. Los programas de Rythme son archivos de
texto que terminan en `.ry`. Un programa, el **intérprete** `rythme`, los lee
y los ejecuta (ver [intérprete](../debuter/glossaire.md#interprete)). Este
intérprete está escrito en otro lenguaje, Rust.

Aquí tienes un programa completo:

```rythme
fn cuadrado(x) = x * x

[1, 2, 3, 4]
  |> filter(is_even)
  |> map(cuadrado)
  |> sum
  |> print
```

```sortie
20
```

Léelo de arriba abajo, como una cadena de montaje:

1. `fn cuadrado(x) = x * x` define una [función](../debuter/glossaire.md#funcion)
   `cuadrado`: recibe un número `x` y da `x * x`;
2. `[1, 2, 3, 4]` es una [lista](../debuter/glossaire.md#lista) de números;
3. `|>` pasa el valor a la etapa siguiente: nos quedamos con los números
   pares (`[2, 4]`), elevamos cada uno al cuadrado (`[4, 16]`), hacemos la
   suma (`20`) y la mostramos.

No hace falta que lo entiendas todo ahora: cada pieza tiene su capítulo.

## La filosofía del lenguaje

Rythme quiere ser **más sencillo que Python o Ruby**. Tiene solo 15
**palabras clave** (las palabras reservadas del lenguaje, como `if` o `fn`),
y una sola manera de hacer cada cosa.

- **Puramente funcional.** Un valor no cambia nunca: es
  [inmutable](../debuter/glossaire.md#inmutable). No hay bucles: se trabaja
  con listas enteras, o con funciones que se llaman a sí mismas.
- **Seguro como Rust.** Sin [`null`](../debuter/glossaire.md#null), sin
  excepciones. Un valor que puede faltar es un `Option`, una operación que
  puede fallar da un `Result`.
- **Agradable como Ruby.** Bloques `do |x| ... end`, llamadas sin
  paréntesis (`puts "hola"`), nombres como `even?`.
- **Ayuda en todas partes.** Cada error dice dónde está, por qué y cómo
  corregirlo. El programa se comprueba antes de ejecutarse.
- **Seis idiomas.** Los mensajes existen en francés, inglés, español,
  alemán, portugués y chino.

Piensa en Rythme como en una cocina bien ordenada: menos utensilios, pero
cada uno tiene su sitio, y nada cambia de sitio a tus espaldas.

> **Nota:** las palabras clave y las funciones integradas están en inglés
> (`if`, `map`, `print`), pero tus propios nombres pueden estar en español,
> con acentos incluidos: `canción`, `año`.

## Instalar Rythme

Rythme se instala escribiendo un comando en una **terminal**, la ventana
donde se escriben comandos en lugar de hacer clic. Si nunca has usado una,
lee primero [la terminal](../debuter/terminal.md).

En **Linux y macOS**, este comando descarga Rythme y lo instala:

```sh
curl -fsSL https://raw.githubusercontent.com/maxlestage/Rythme/master/install.sh | sh
```

En **Windows**, en PowerShell:

```text
irm https://raw.githubusercontent.com/maxlestage/Rythme/master/install.ps1 | iex
```

Después, comprueba que todo funciona:

```sh
rythme version
```

```text
rythme 0.1.0
```

Si la terminal responde «comando no encontrado», ciérrala, vuelve a abrirla
e inténtalo de nuevo: tiene que volver a leer la lista de programas
instalados.

## Probar sin instalar nada

El [sitio de Rythme](https://maxlestage.github.io/Rythme/) tiene un **playground**:
escribes código en la página, lo ejecutas y ves el resultado. Es el
intérprete de verdad, que funciona en el navegador. Única diferencia: no
puede leer ni escribir archivos en tu ordenador.

## El modo interactivo

Escribe `rythme` sin nada más: es el **modo interactivo** (también se dice
[REPL](../debuter/glossaire.md#repl)). Escribes una línea y Rythme muestra
el resultado enseguida. Es una calculadora muy sabia.

```text
Rythme 0.1.0 — :help para la ayuda, :quit para salir
ry> 1 + 2
3
ry> nombre = "Ada"
ry> "Hola, {nombre}"
"Hola, Ada"
ry> :quit
```

`ry>` es el **indicador**: Rythme espera tu línea. No lo escribes tú.

> **Consejo:** en el modo interactivo, `:doc map` muestra la ayuda de la
> función `map`, y `:quit` sirve para salir.

## Tu primer programa

El modo interactivo lo olvida todo al salir. Para guardar un programa, se
escribe en un archivo. Crea con tu
[editor de código](../debuter/editeur.md) un archivo `hola.ry` con este
contenido:

```rythme
# Mi primer programa
print("¡Hola, mundo!")
```

```sortie
¡Hola, mundo!
```

- La línea que empieza por `#` es un **comentario**: una nota para las
  personas, que Rythme ignora.
- `print` es una función que muestra lo que le das entre paréntesis.
- `"¡Hola, mundo!"` es un **texto**: se escribe entre comillas rectas `"`.

Luego ejecútalo en una terminal, desde la carpeta del archivo (ver
[la terminal](../debuter/terminal.md)):

```sh
rythme run hola.ry
```

`rythme hola.ry` también funciona, sin la palabra `run`.

## Comprobar sin ejecutar

`rythme check` busca problemas sin ejecutar el programa:

```sh
rythme check hola.ry
```

```text
✓ hola.ry: no se encontró ningún problema
```

Si encuentra errores, los muestra todos de una vez, con una ayuda para cada
uno. `rythme run` también hace esta comprobación antes de ejecutar: un
programa que tiene un error no arranca en absoluto.

## Obtener ayuda

Rythme tiene su ayuda integrada, en español:

```sh
rythme help            # la lista de temas
rythme help map        # la ayuda de una función
rythme help builtins   # todas las funciones integradas
rythme learn           # una visita guiada por el lenguaje
rythme explain E003    # la explicación de un código de error
```

Para elegir el idioma de los mensajes, añade `--lang es` antes del comando,
o define la variable `RYTHME_LANG=es`.

## Errores frecuentes

En los mensajes de abajo, el archivo se llama `main.ry`. Cada mensaje da el
archivo, la línea y la columna (`main.ry:1:7`: línea 1, carácter 7), y
luego muestra el lugar exacto con `^`.

### Olvidar las comillas

```rythme norun
print(Hola)
```

```text
error[E003]: nombre desconocido `Hola`
 --> main.ry:1:7
  |
1 | print(Hola)
  |       ^^^^ no definido
  |
  = ayuda: para crear tu propio tipo de valor, define un tipo: `type Nombre = Hola(campo) | Otro`
  = más sobre este error: `rythme explain E003`
```

Sin comillas, `Hola` se lee como un **nombre** de valor, y ese nombre no
existe. Para un texto, pon comillas: `print("Hola")`.

### Comillas simples en lugar de comillas dobles

```rythme norun
print('Hola')
```

```text
error[E018]: carácter inesperado `'`
 --> main.ry:1:7
  |
1 | print('Hola')
  |       ^
  |
  = ayuda: los textos van entre comillas dobles: "hola"
  = más sobre este error: `rythme explain E018`
```

Otros lenguajes aceptan `'...'`. Rythme siempre quiere comillas dobles
rectas: `print("Hola")`.

### Olvidar la comilla final

```rythme norun
print("Hola)
```

```text
error[E002]: este texto nunca se cierra
 --> main.ry:1:7
  |
1 | print("Hola)
  |       ^ el texto empieza aquí
  |
  = ayuda: añade un `"` al final del texto
  = más sobre este error: `rythme explain E002`
```

Rythme lee todo lo que sigue a la primera `"` como texto, hasta el final del
archivo. Cierra el texto: `print("Hola")`.

## Ponte a prueba

**Pregunta 1:** ¿qué comando comprueba un programa `juego.ry` sin
ejecutarlo?

> **Respuesta**
>
> `rythme check juego.ry`. Para ejecutarlo, es `rythme run juego.ry` (o
> `rythme juego.ry`).

**Pregunta 2:** ¿qué muestra este programa?

```rythme norun
# print("uno")
print("dos")
```

> **Respuesta**
>
> Solo `dos`. La primera línea empieza por `#`: es un comentario, Rythme lo
> ignora.

**Pregunta 3:** en el primer ejemplo, ¿por qué el resultado es 20 y no 30
(1 + 4 + 9 + 16)?

> **Respuesta**
>
> Porque `filter(is_even)` se queda solo con los números pares, 2 y 4,
> antes de elevarlos al cuadrado: 4 + 16 = 20.

## En resumen

- Rythme es un pequeño lenguaje puramente funcional: los valores no cambian
  nunca, y no hay bucles.
- Los programas son archivos `.ry`, que se ejecutan con
  `rythme run archivo.ry`.
- `rythme` solo abre el modo interactivo; `rythme check archivo.ry`
  comprueba un programa sin ejecutarlo.
- Un texto se escribe entre comillas rectas `"`, y `#` empieza un
  comentario.
- Cada mensaje de error da el lugar, la causa y una ayuda; `rythme help` y
  `rythme explain` cuentan más, en español.

Siguiente: [los primeros pasos](premiers-pas.md). Si ya conoces Ruby o
Rust, compara con [la introducción a Ruby](../ruby/introduction.md) y
[la introducción a Rust](../rust/introduction.md).
