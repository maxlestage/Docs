# La terminal en 10 minutos

La terminal es una ventana donde hablas con el ordenador escribiendo texto: esta página te enseña a abrirla, a moverte por ella y a ejecutar tus programas.

> **Objetivos:**
>
> - abrir la terminal en Windows, macOS o Linux;
> - saber dónde estás y moverte entre carpetas;
> - crear una carpeta para tus pruebas;
> - ahorrar tiempo con la flecha hacia arriba, Tab y Ctrl + C;
> - ejecutar un programa en Ruby, Rust o Rythme;
> - leer un mensaje de error de la terminal.

## Qué es la terminal

Normalmente usas el ordenador con el ratón: haces clic en
carpetas e iconos.

La **terminal** hace lo mismo, pero con el teclado. Escribes un
**comando**, pulsas Intro y el ordenador responde con texto.

Ahí es donde ejecutarás tus programas. Que no cunda el pánico: con una decena de
comandos basta para empezar.

## Abrir la terminal

### En Windows

Abre el menú Inicio, escribe **Terminal** y pulsa Intro. Si no
aparece nada, escribe **PowerShell**. Los dos sirven.

### En macOS

Pulsa **Cmd + Espacio**, escribe **Terminal** y pulsa Intro.
La aplicación también está en *Aplicaciones › Utilidades*.

### En Linux

El atajo **Ctrl + Alt + T** funciona en la mayoría de los sistemas. Si no,
busca **Terminal** en la lista de aplicaciones.

## El indicador de comandos

Una vez abierta, la terminal muestra una línea que espera tu comando.
Es el **indicador** (en inglés, *prompt*). Se parece a esto:

```text
alice@portatil:~$
```

o, en Windows:

```text
PS C:\Users\Alice>
```

El indicador suele mostrar la **carpeta donde estás**. En esta página, los
comandos se escriben sin el indicador: escribe solo lo que hay en el
bloque.

## Los comandos básicos

### Dónde estoy: pwd

`pwd` muestra la carpeta actual (*print working directory*).

```sh
pwd
```

```text
/home/alice
```

### Qué contiene esta carpeta: ls o dir

`ls` lista los archivos y las carpetas. En Windows, `dir` hace lo
mismo (`ls` también funciona en PowerShell).

```sh
ls
```

```text
Descargas  Documentos  Escritorio  Imágenes
```

### Entrar en una carpeta: cd

`cd` (*change directory*) te mueve a una carpeta.

```sh
cd Documentos
```

Para subir un nivel, a la carpeta padre, escribe `cd ..`:

```sh
cd ..
```

### Crear una carpeta: mkdir

`mkdir` (*make directory*) crea una carpeta. Crea una para tus pruebas:

```sh
mkdir codigo
cd codigo
```

### Limpiar la pantalla: clear

`clear` borra la pantalla. En Windows, `cls` hace lo mismo.

## Los atajos que ahorran tiempo

- **Flecha hacia arriba**: recupera el comando anterior. Púlsala varias
  veces para ir más atrás.
- **Tab**: completa un nombre. Escribe `cd Doc` y luego Tab: la terminal escribe
  `Documentos` por ti.
- **Ctrl + C**: detiene el programa en curso. Útil si ya no se detiene solo.

> **Consejo:** usa Tab todo el tiempo. Es más rápido y evita
> las erratas en los nombres de archivo.

> **Atención:** en la terminal, Ctrl + C no copia: detiene. Para
> copiar, usa Ctrl + Shift + C (Linux), Cmd + C (macOS) o la selección
> con el ratón (Windows).

## Ejecutar un programa

Primero colócate, con `cd`, en la carpeta que contiene tu archivo.
Después escribe el nombre del lenguaje seguido del nombre del archivo.

Un programa en Ruby:

```sh
ruby hola.rb
```

Un programa en Rythme:

```sh
rythme hola.ry
```

Un programa en Rust se guarda en un **proyecto**. En la carpeta del proyecto,
este comando compila y luego ejecuta el programa:

```sh
cargo run
```

Cada lenguaje se instala una sola vez. Consulta las páginas de introducción:
[Ruby](../ruby/introduction.md), [Rust](../rust/introduction.md) y
[Rythme](../rythme/introduction.md).

## Leer un error

Si algo va mal, la terminal muestra un mensaje. Por ejemplo, si
el archivo no existe:

```text
ruby: No such file or directory -- hla.rb (LoadError)
```

Léelo con calma. Aquí, el nombre está mal escrito: `hla` en lugar de
`hola`. Las causas más frecuentes:

- una errata en el nombre;
- no estás en la carpeta correcta (compruébalo con `pwd` y `ls`);
- el lenguaje no está instalado: la terminal dice «command not found» o
  «no se reconoce».

## Windows y macOS/Linux

La mayoría de los comandos son iguales. Estas son las diferencias:

| Acción                    | macOS y Linux  | Windows (PowerShell)  |
|---------------------------|----------------|-----------------------|
| ¿Dónde estoy?             | `pwd`          | `pwd`                 |
| Listar los archivos       | `ls`           | `dir` (o `ls`)        |
| Entrar en una carpeta     | `cd carpeta`   | `cd carpeta`          |
| Subir un nivel            | `cd ..`        | `cd ..`               |
| Crear una carpeta         | `mkdir codigo` | `mkdir codigo`        |
| Borrar la pantalla        | `clear`        | `cls` (o `clear`)     |
| Separador de rutas        | `/`            | `\`                   |

En Windows, una ruta se escribe con barras invertidas:

```powershell
cd C:\Users\Alice\Documents\codigo
```

## Errores frecuentes

### Un nombre de carpeta con un espacio

La terminal separa las palabras con espacios. Aquí, cree que recibe dos
carpetas, `Mis` y `Documentos`:

```sh
cd Mis Documentos
```

```text
bash: cd: too many arguments
```

Pon el nombre entre comillas: `cd "Mis Documentos"`. O escribe `cd Mis`
y luego Tab: la terminal completa el nombre correctamente. Para tus propias
carpetas, evita los espacios: `mis_documentos`.

### Una errata en un nombre

```sh
cd Documentoz
```

```text
bash: cd: Documentoz: No such file or directory
```

La carpeta no existe con ese nombre. Escribe `ls` para ver los nombres exactos,
y usa Tab para no volver a equivocarte.

### Un comando desconocido

```sh
rubyy hola.rb
```

```text
bash: rubyy: command not found
```

O el nombre del comando está mal escrito (aquí, `rubyy` en lugar de `ruby`),
o el lenguaje no está instalado. Compruébalo con `ruby --version`.

## Ponte a prueba

**1.** Ya no sabes en qué carpeta estás. ¿Qué comando
escribes?

> **Respuesta**
>
> `pwd`, que muestra la carpeta actual. Luego `ls` (o `dir`) para ver lo
> que contiene.

**2.** ¿Cómo se sube a la carpeta padre?

> **Respuesta**
>
> Con `cd ..`: los dos puntos indican la carpeta padre.

**3.** Tu programa se ejecuta sin fin. ¿Cómo lo detienes?

> **Respuesta**
>
> Con **Ctrl + C** en la terminal. Atención: en la terminal, este
> atajo detiene, no copia.

## En resumen

- La terminal permite hablar con el ordenador escribiendo comandos.
- `pwd` dice dónde estás, `ls` (o `dir`) lista, `cd` te mueve, `mkdir` crea una carpeta.
- La flecha hacia arriba recupera un comando, Tab completa, Ctrl + C detiene.
- Un programa se ejecuta con `ruby archivo.rb`, `rythme archivo.ry` o `cargo run`.
- Un mensaje de error se lee con calma: casi siempre dice qué corregir.

Siguiente: [instalar un editor de código](editeur.md).
