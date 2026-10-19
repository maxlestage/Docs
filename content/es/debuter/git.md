# Guardar tu trabajo con Git

Git guarda el historial de tu código y te permite volver atrás: esta página te enseña a instalarlo, a guardar tus cambios y a enviarlos a GitHub.

> **Objetivos:**
>
> - entender para qué sirve Git;
> - crear un repositorio con `git init`;
> - guardar una versión con `git add` y `git commit`;
> - ver el estado y el historial con `git status` y `git log`;
> - ignorar archivos con `.gitignore`;
> - enviar tu código a GitHub y recuperarlo en otro lugar.

## Por qué Git

Quizá ya hayas visto carpetas como estas:

```text
proyecto.rb
proyecto_v2.rb
proyecto_v2_final.rb
proyecto_v2_final_DE_VERDAD.rb
```

**Git** hace este trabajo por ti, y de forma ordenada. Es un programa de
**control de versiones**:

- guarda **cada versión** de tu código, con la fecha y un mensaje;
- te permite **volver atrás** si lo has roto todo;
- te permite enviar tu código **a Internet** (GitHub), para guardarlo,
  enseñarlo o trabajar en equipo.

Una carpeta seguida por Git se llama **repositorio** (en inglés,
*repository*, a menudo abreviado *repo*). Una versión guardada se llama
**commit**.

> **Recuerda:** un commit es una foto de tu proyecto en un momento dado.
> Git guarda el álbum completo.

## Instalar Git

### En Windows

Descarga Git desde el sitio `git-scm.com` y lanza la instalación. Acepta
las opciones propuestas.

### En macOS

Escribe `git --version` en el terminal. Si Git no está instalado, macOS
te propone instalarlo: acepta.

### En Linux

Instala el paquete `git` con el gestor de tu sistema, por ejemplo en
Ubuntu o Debian:

```sh
sudo apt install git
```

### Comprobar y presentarte

Comprueba la instalación:

```sh
git --version
```

```text
git version 2.43.0
```

Después dile a Git quién eres, de una vez por todas. Este nombre y este
correo aparecerán en cada commit:

```sh
git config --global user.name "Alicia Martín"
git config --global user.email "alicia@example.com"
```

## Crear un repositorio: git init

Sitúate en la carpeta de tu proyecto (consulta
[el terminal](terminal.md)) y escribe:

```sh
cd code
git init
```

```text
Initialized empty Git repository in /home/alicia/code/.git/
```

Git ha creado una carpeta oculta `.git`. Ahí guarda todo el historial. No
la toques.

## Ver el estado: git status

`git status` dice qué ha cambiado desde el último commit. Crea un archivo
`hola.rb` y luego:

```sh
git status
```

```text
On branch main

No commits yet

Untracked files:
  (use "git add <file>..." to include in what will be committed)
	hola.rb

nothing added to commit but untracked files present (use "git add" to track)
```

*Untracked files*: Git ve el archivo, pero todavía no lo sigue.

> **Consejo:** escribe `git status` antes y después de cada comando.
> Siempre te dice dónde estás y, a menudo, qué hacer después.

## Preparar: git add

`git add` elige los archivos que irán en el próximo commit. Es como meter
objetos en una caja antes de cerrarla.

```sh
git add hola.rb
```

Para añadir todos los archivos modificados de la carpeta:

```sh
git add .
```

`git status` muestra ahora el archivo preparado:

```text
Changes to be committed:
  (use "git rm --cached <file>..." to unstage)
	new file:   hola.rb
```

## Guardar: git commit

`git commit` cierra la caja y le pone una etiqueta. El mensaje, después de
`-m`, dice lo que has hecho:

```sh
git commit -m "Primer programa"
```

```text
[main (root-commit) 499728c] Primer programa
 1 file changed, 1 insertion(+)
 create mode 100644 hola.rb
```

`499728c` es el comienzo del **identificador** del commit: su número
único.

Escribe mensajes cortos y claros, que digan **qué cambia**: «Añade el
cálculo de la media», «Corrige la división entre cero». Evita «cambios» o
«ya funciona».

## Ver el historial: git log

`git log` muestra los commits, del más reciente al más antiguo:

```sh
git log
```

```text
commit 499728c1890cc53a9d89212eaccef8917f0d2eda
Author: Alicia <alicia@example.com>
Date:   Sun Oct 18 19:55:35 2026 +0000

    Primer programa
```

Para ver una línea por commit:

```sh
git log --oneline
```

```text
499728c Primer programa
```

Para salir de un historial largo, pulsa **q**.

## Ignorar archivos: .gitignore

Algunos archivos no deben ir al repositorio: los archivos que crea el
compilador, las contraseñas, los archivos del sistema. Escribe sus nombres
en un archivo `.gitignore`, en la raíz del proyecto:

```text
# Lo que compila Rust (cargo build)
target/
# Los archivos de macOS
.DS_Store
# Los secretos
.env
```

Git ya no verá esos archivos. Añade el propio `.gitignore` al repositorio:

```sh
git add .gitignore
git commit -m "Añade el .gitignore"
```

> **Nota:** `cargo new` ya crea un `.gitignore` para tus proyectos Rust.

> **Atención:** nunca pongas una contraseña en un commit. Aunque la borres
> después, se queda en el historial.

## GitHub: guardar en línea

Por ahora, tu repositorio está **en tu ordenador**. **GitHub** es un sitio
web que aloja repositorios Git en línea. Hay otros, como GitLab o
Codeberg; funcionan de la misma manera.

### Crear un repositorio en GitHub

1. Crea una cuenta gratuita en `github.com`.
2. Haz clic en **+** arriba a la derecha y después en **New repository**.
3. Ponle un nombre, por ejemplo `code`.
4. Déjalo **vacío**: no marques «Add a README».
5. Haz clic en **Create repository**.

GitHub muestra entonces la dirección del repositorio, por ejemplo
`https://github.com/alicia/code.git`.

### Enviar tu código: git push

Dile a tu repositorio local dónde está el repositorio en línea. El nombre
`origin` es la costumbre:

```sh
git remote add origin https://github.com/alicia/code.git
```

Después envía tus commits:

```sh
git push -u origin main
```

La primera vez, GitHub te pide que inicies sesión. Después, basta con un
simple `git push`. Recarga la página de GitHub: tu código está en línea.

### Recuperar un repositorio: git clone

En otro ordenador, o para recuperar el proyecto de otra persona,
`git clone` descarga todo el repositorio, con su historial:

```sh
git clone https://github.com/alicia/code.git
cd code
```

Si el repositorio ha cambiado en línea desde entonces, `git pull`
recupera los commits nuevos.

## La rutina de cada día

Una vez creado el repositorio, el trabajo diario cabe en unos pocos
comandos:

| Cuándo                            | Comando                     |
|-----------------------------------|-----------------------------|
| Al empezar                        | `git pull`                  |
| Para ver dónde estás              | `git status`                |
| Después de un pequeño paso que funciona | `git add .`           |
| Para guardar ese paso             | `git commit -m "Mensaje"`   |
| Para ver el historial             | `git log --oneline`         |
| Al terminar                       | `git push`                  |

```sh
git pull
git status
git add .
git commit -m "Añade la función media"
git push
```

> **Consejo:** haz commits pequeños y a menudo: cada vez que algo
> funcione. Si después lo rompes todo, volverás a una versión que
> funcionaba.

## Errores frecuentes

### Usar git fuera de un repositorio

```sh
git status
```

```text
fatal: not a git repository (or any of the parent directories): .git
```

No estás en la carpeta del proyecto, o no has hecho `git init`.
Compruébalo con `pwd` y después haz `cd code` o `git init`.

### Hacer un commit sin git add

```sh
git commit -m "Primer programa"
```

```text
On branch main

Initial commit

Untracked files:
  (use "git add <file>..." to include in what will be committed)
	hola.rb

nothing added to commit but untracked files present (use "git add" to track)
```

La caja está vacía. Haz primero `git add hola.rb` (o `git add .`).

### Olvidar presentarte

```sh
git commit -m "Primer programa"
```

```text
Author identity unknown

*** Please tell me who you are.

Run

  git config --global user.email "you@example.com"
  git config --global user.name "Your Name"

to set your account's default identity.
Omit --global to set the identity only in this repository.
```

Escribe los dos comandos `git config` con tu nombre y tu correo, y vuelve
a hacer el commit.

### Hacer push sin repositorio en línea

```sh
git push
```

```text
fatal: No configured push destination.
Either specify the URL from the command-line or configure a remote repository using

    git remote add <name> <url>

and then push using the remote name

    git push <name>
```

Git no sabe adónde enviar. Añade la dirección con
`git remote add origin …` y después `git push -u origin main`.

## Ponte a prueba

**1.** ¿Cuál es la diferencia entre `git add` y `git commit`?

> **Respuesta**
>
> `git add` elige los cambios que se van a guardar (llenas la caja).
> `git commit` los guarda de verdad, con un mensaje (cierras la caja y le
> pones una etiqueta).

**2.** Tu proyecto Rust contiene una carpeta `target/` muy pesada. ¿Cómo
evitas enviarla a GitHub?

> **Respuesta**
>
> Escribe `target/` en el archivo `.gitignore`, en la raíz del proyecto.

**3.** Cambias de ordenador. ¿Cómo recuperas tu proyecto desde GitHub?

> **Respuesta**
>
> Con `git clone` seguido de la dirección del repositorio, por ejemplo
> `git clone https://github.com/alicia/code.git`.

## En resumen

- Git guarda cada versión de tu código; una carpeta seguida es un repositorio, una versión es un commit.
- `git init` crea el repositorio, una sola vez.
- `git add` prepara, `git commit -m "…"` guarda, `git status` y `git log` muestran dónde estás.
- `.gitignore` enumera los archivos que no se deben seguir.
- `git push` envía a GitHub, `git clone` recupera un repositorio, `git pull` lo actualiza.
- Haz commits pequeños y a menudo.

Siguiente: [Cómo funciona la web](web.md)
