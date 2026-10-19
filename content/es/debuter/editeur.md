# Instalar un editor de código

Un editor de código es el programa en el que escribes tus programas: esta página te ayuda a instalar VS Code, a crear tu primer archivo y a añadir las extensiones adecuadas.

> **Objetivos:**
>
> - entender la diferencia entre un editor de código y un procesador de
>   textos;
> - instalar VS Code y abrir una carpeta;
> - crear, nombrar y guardar un archivo de código;
> - ejecutar un programa desde la terminal integrada;
> - añadir las extensiones útiles para Ruby, Rust y Rythme.

## Un editor, no un procesador de textos

Word o LibreOffice sirven para escribir cartas. Añaden fuentes,
colores, márgenes. Para el ordenador, todo eso es ruido: un
programa tiene que ser **texto plano**.

Un **editor de código** escribe texto plano, y te ayuda:

- **colorea** el código para que sea legible;
- **señala los errores** mientras escribes;
- **completa** las palabras que empiezas;
- incluye una **terminal** para ejecutar tus programas.

> **Atención:** nunca escribas código en Word. Allí las comillas y los
> espacios se transforman, y el programa deja de funcionar.

## Instalar VS Code

Esta página usa **Visual Studio Code** (VS Code). Es gratuito, existe
para todos los sistemas y sirve para los tres lenguajes.

### En Windows

Ve al sitio `code.visualstudio.com`, haz clic en **Download for
Windows** y abre el archivo descargado. Acepta las opciones propuestas.
Marca **Agregar a PATH** si aparece la casilla.

### En macOS

Descarga la versión para macOS en el mismo sitio. Abre el archivo `.zip`
y arrastra **Visual Studio Code** a la carpeta **Aplicaciones**.

### En Linux

Descarga el paquete `.deb` (Ubuntu, Debian) o `.rpm` (Fedora) del
sitio y ábrelo. También puedes usar la tienda de aplicaciones de tu
sistema: busca **Visual Studio Code**.

## Abrir una carpeta

Un programa vive en una **carpeta**. Crea una carpeta `codigo` (consulta
[la terminal en 10 minutos](terminal.md#crear-una-carpeta-mkdir)) y, en
VS Code: menú **File › Open Folder…** (Archivo › Abrir carpeta), y
elígela.

La carpeta aparece a la izquierda, en el **explorador**. Ahí se verán
todos tus archivos.

> **Consejo:** abre siempre una carpeta, no un archivo suelto. El editor
> entiende mejor tu proyecto, y la terminal se abre en el sitio correcto.

## Crear un archivo

En el explorador, haz clic en el icono **Nuevo archivo** y escribe su
nombre, por ejemplo `hola.rb`. Escribe dentro:

```ruby
puts "¡Hola!"
```

```sortie
¡Hola!
```

### Los nombres de archivo

La **extensión**, el final del nombre, indica el lenguaje:

| Lenguaje | Extensión | Ejemplo       |
|----------|-----------|---------------|
| Ruby     | `.rb`     | `hola.rb`     |
| Rust     | `.rs`     | `main.rs`     |
| Rythme   | `.ry`     | `hola.ry`     |

Elige nombres sencillos: en minúsculas, sin espacios ni tildes. Para
separar las palabras, usa `_`: `calcular_media.rb`.

## Guardar

Un archivo sin guardar no existe para la terminal. Guarda con
**Ctrl + S** (Windows, Linux) o **Cmd + S** (macOS).

Un punto blanco en la pestaña del archivo indica cambios sin
guardar.

### El guardado automático

Para olvidarte de ello, activa el menú **File › Auto Save**. VS Code
guarda entonces solo, unos instantes después de cada cambio.

> **Recuerda:** si tu programa no ve tus cambios, comprueba
> primero que el archivo está guardado.

## La terminal integrada

VS Code incluye una terminal. Ábrela con el menú **Terminal › New
Terminal**, o con **Ctrl + `** (la tecla del acento grave).

Se abre en la parte de abajo de la ventana, ya colocada en tu carpeta. Ejecuta
tu programa:

```sh
ruby hola.rb
```

```text
¡Hola!
```

## Las extensiones útiles

Una **extensión** añade funciones a VS Code. Haz clic en el icono de
**Extensiones** a la izquierda (cuatro cuadraditos), busca un nombre y haz clic
en **Install**.

- **Ruby LSP** (de Shopify): colores, autocompletado y errores para Ruby.
- **rust-analyzer**: la ayuda imprescindible para Rust. Muestra los tipos
  y los errores incluso antes de compilar.
- **Rythme**: colores, ayuda al pasar el ratón, errores explicados en español y
  un botón **▶ Ejecutar** encima del código. Instala primero Rythme
  (consulta la [introducción a Rythme](../rythme/introduction.md)).

> **Nota:** si la extensión Rythme todavía no aparece en la búsqueda,
> el repositorio de Rythme explica cómo instalarla desde su carpeta
> `editors/vscode`.

## Otros editores

VS Code no es la única opción. Aquí tienes otras tres:

- **Zed**: muy rápido y moderno, con una extensión para Rythme.
- **Sublime Text**: ligero y rápido, de pago tras un periodo de prueba.
- **Neovim**: se usa por completo con el teclado, en la terminal; potente,
  pero lleva tiempo aprenderlo.

Para empezar, quédate con VS Code: ya cambiarás más adelante si
quieres.

## Errores frecuentes

### Comillas transformadas

Un código copiado desde Word, un correo electrónico o ciertos sitios contiene
comillas «tipográficas» (`“ ”`) en lugar de comillas rectas (`"`):

```ruby norun
puts “Hola”
```

```text
guil.rb:1:in `<main>': undefined local variable or method `“Hola”' for main (NameError)
```

Ruby no reconoce `“` como comilla. Vuelve a escribir las comillas en
el editor de código: `puts "Hola"`.

### Olvidar guardar

Cambias `"¡Hola!"` por `"¡Buenas!"`, vuelves a ejecutar… y la terminal
sigue mostrando:

```text
¡Hola!
```

No hay ningún error: la terminal lee el archivo **guardado**, no lo
que hay en pantalla. Mira el punto blanco en la pestaña, guarda con
Ctrl + S (Cmd + S) y vuelve a ejecutar. O activa *Auto Save*.

### Una extensión oculta

En Windows o macOS, un archivo creado fuera del editor puede
llamarse en realidad `hola.rb.txt`. La terminal no lo encuentra:

```sh
ruby hola.rb
```

```text
ruby: No such file or directory -- hola.rb (LoadError)
```

Crea tus archivos desde el explorador de VS Code, que muestra el nombre
completo, y compruébalo con `ls` en la terminal.

## Ponte a prueba

**1.** ¿Por qué no escribir el código en Word?

> **Respuesta**
>
> Word no escribe texto plano: añade formato y
> transforma las comillas y los espacios. El programa deja de funcionar.

**2.** ¿Qué extensión de archivo se usa para un programa en Rythme?
¿Y para Ruby?

> **Respuesta**
>
> `.ry` para Rythme, `.rb` para Ruby (y `.rs` para Rust).

**3.** ¿Por qué abrir una carpeta en lugar de un archivo suelto en VS Code?

> **Respuesta**
>
> El editor entiende mejor el proyecto entero, el explorador muestra todos sus
> archivos, y la terminal integrada se abre directamente en el sitio correcto.

## En resumen

- Un editor de código escribe texto plano y te ayuda a leerlo y a corregirlo.
- Instala VS Code y luego abre una carpeta con *File › Open Folder*.
- La extensión del archivo indica el lenguaje: `.rb`, `.rs` o `.ry`.
- Guarda con Ctrl + S (Cmd + S en macOS), o activa *Auto Save*.
- Añade las extensiones Ruby LSP, rust-analyzer y Rythme.

Siguiente: [encontrar y corregir tus errores](deboguer.md).
