# Dibujar en el navegador con canvas

Esta página muestra cómo dibujar formas y animar una pelota en una página web con el elemento `<canvas>`, separando el dibujo (en el navegador) de los cálculos (que se pueden probar con Node).

> **Objetivos:**
>
> - sabrás colocar un `<canvas>` en una página y obtener su «pincel»;
> - entenderás las coordenadas del canvas, donde el eje `y` baja;
> - sabrás dibujar rectángulos, círculos, líneas y texto;
> - sabrás calcular posiciones con funciones puras, probadas en Node;
> - sabrás animar una pelota que rebota, un paso a la vez.

## Un lienzo de píxeles

Un `<canvas>` (en inglés, «lienzo») es un rectángulo vacío en la página. Dibujas en él
con JavaScript, píxel a píxel. Es como una hoja de papel cuadriculado: cada cuadrito
es un **píxel**, un punto de color de la pantalla.

El canvas no forma parte del [DOM](../debuter/glossaire.md#dom) como un título o un
botón: lo que dibujas en él no es un elemento. Es pintura. Una vez puesta, se queda
ahí, hasta que pintes encima o la borres.

```html norun
<!doctype html>
<html lang="es">
  <head>
    <meta charset="utf-8">
    <title>Mi lienzo</title>
  </head>
  <body>
    <canvas id="lienzo" width="300" height="150"></canvas>
    <script type="module" src="dibujo.js"></script>
  </body>
</html>
```

- `width="300"`: el lienzo mide 300 píxeles de ancho;
- `height="150"`: y 150 píxeles de alto;
- el script `dibujo.js` hará el dibujo (ver [el DOM](dom.md) para enlazar un script).

Para dibujar, le pides al canvas su **contexto 2D**: es el pincel, el objeto que sabe
trazar formas.

```javascript norun
// dibujo.js
const lienzo = document.querySelector("#lienzo");
const ctx = lienzo.getContext("2d");   // el pincel

ctx.fillStyle = "tomato";              // el color de relleno
ctx.fillRect(10, 20, 100, 50);         // un rectángulo relleno
```

Casi siempre este pincel se llama `ctx` (por *context*).

> **Recuerda:** `<canvas>` es un lienzo vacío; `getContext("2d")` te da el pincel
> `ctx`, y es él quien dibuja.

## Las coordenadas: y baja

Para decir dónde dibujar, das dos números: `x` e `y`.

- `x` cuenta los píxeles **hacia la derecha**, desde el borde izquierdo;
- `y` cuenta los píxeles **hacia abajo**, desde el borde de arriba.

El punto `(0, 0)` está por tanto **arriba a la izquierda**. Es lo contrario de las
matemáticas del colegio, donde `y` sube. Piensa en una página de un libro: se lee de
arriba abajo, la línea 1 está arriba.

```text
 (0,0) ─────── x aumenta ─────────▶
   │ ┌──────────────────────────┐
   │ │                          │
   │ │    ■ (60,30)             │
   y │                          │
   │ │               ● (240,110)│
   │ │                          │
   ▼ └──────────────────────────┘
  y aumenta             (300,150) = abajo a la derecha
```

El punto `(240, 110)` está más **abajo** que `(60, 30)`, porque su `y` es mayor.

| Punto | Dónde en el lienzo de 300 × 150 |
|---|---|
| `(0, 0)` | esquina de arriba a la izquierda |
| `(300, 0)` | esquina de arriba a la derecha |
| `(0, 150)` | esquina de abajo a la izquierda |
| `(150, 75)` | el centro |

Si tienes coordenadas «al derecho», como en matemáticas, una pequeña función les da la
vuelta. No toca el navegador: Node puede ejecutarla.

```javascript
function aCanvas(x, y, alto) {
  return { x, y: alto - y };
}

console.log(aCanvas(0, 0, 150));    // abajo a la izquierda en matemáticas
console.log(aCanvas(0, 150, 150));  // arriba a la izquierda en matemáticas
console.log(aCanvas(100, 40, 150));
```

```sortie
{ x: 0, y: 150 }
{ x: 0, y: 0 }
{ x: 100, y: 110 }
```

| Llamada | `alto - y` | `y` en el canvas |
|---|---|---|
| `aCanvas(0, 0, 150)` | `150 - 0` | `150` (abajo) |
| `aCanvas(0, 150, 150)` | `150 - 150` | `0` (arriba) |
| `aCanvas(100, 40, 150)` | `150 - 40` | `110` |

Prueba: llama a `aCanvas(150, 75, 150)`. El centro sigue siendo el centro:
`{ x: 150, y: 75 }`.

> **Recuerda:** `(0, 0)` está arriba a la izquierda. Cuanto mayor es `y`, más **abajo**
> está el punto.

## Dibujar formas

Estas son las órdenes principales del pincel. Este código solo funciona en un
navegador (`document` no existe en Node).

### Rectángulos

```javascript norun
ctx.fillStyle = "steelblue";
ctx.fillRect(10, 10, 80, 40);     // relleno: x, y, ancho, alto

ctx.strokeStyle = "black";
ctx.lineWidth = 3;
ctx.strokeRect(110, 10, 80, 40);  // solo el contorno

ctx.clearRect(0, 0, 300, 150);    // borra todo el lienzo
```

`x` e `y` indican la esquina **de arriba a la izquierda** del rectángulo. El ancho va
hacia la derecha, el alto va **hacia abajo**.

```text
 (10,10)
    ┌────────── 80 ──────────┐
    │                        │ 40
    └────────────────────────┘
                         (90,50)
```

### Líneas y trazados

Una línea se dibuja en tres tiempos, como con un lápiz: levantas el lápiz, lo apoyas,
trazas.

```javascript norun
ctx.beginPath();          // nuevo dibujo: olvidamos el anterior
ctx.moveTo(20, 130);      // apoyar el lápiz (sin trazar)
ctx.lineTo(150, 20);      // trazar hasta aquí
ctx.lineTo(280, 130);     // luego hasta aquí
ctx.closePath();          // volver al punto de partida
ctx.stroke();             // dibujar el contorno (un triángulo)
```

### Círculos

Un círculo es un **arco** que da la vuelta completa: del ángulo `0` al ángulo
`2 * Math.PI` (los ángulos están en radianes; `2 * Math.PI` es una vuelta).

```javascript norun
ctx.beginPath();
ctx.arc(150, 75, 30, 0, 2 * Math.PI);  // centro (150, 75), radio 30
ctx.fillStyle = "gold";
ctx.fill();
```

Atención: para un círculo, `(150, 75)` es el **centro**, no la esquina.

### Texto

```javascript norun
ctx.font = "20px sans-serif";
ctx.fillStyle = "black";
ctx.fillText("Hola", 10, 140);  // x, y de la línea base del texto
```

| Orden | Qué hace |
|---|---|
| `fillRect(x, y, an, al)` | un rectángulo relleno |
| `strokeRect(x, y, an, al)` | el contorno de un rectángulo |
| `clearRect(x, y, an, al)` | borra una zona |
| `beginPath()` | empieza un trazado nuevo |
| `moveTo(x, y)` / `lineTo(x, y)` | mueve el lápiz / traza una línea |
| `arc(x, y, r, inicio, fin)` | un arco o un círculo |
| `fill()` / `stroke()` | rellena / traza el trazado |
| `fillText(texto, x, y)` | escribe un texto |

> **Recuerda:** primero eliges el color (`fillStyle`, `strokeStyle`) y luego dibujas.
> Para las líneas y los círculos: `beginPath`, el trazado y luego `fill` o `stroke`.

## Ver un dibujo sin navegador

Para entender lo que hace un dibujo, no hace falta abrir un navegador. Puedes fabricar
un **lienzo de caracteres**: una cuadrícula de puntos `.`, donde escribes `#` en lugar
de un píxel pintado. Cada carácter hace el papel de un píxel.

```javascript
class LienzoTexto {
  constructor(ancho, alto) {
    this.ancho = ancho;
    this.alto = alto;
    // una fila por cada valor de y, una casilla por cada valor de x
    this.filas = Array.from({ length: alto }, () => Array(ancho).fill("."));
  }

  punto(x, y, car = "#") {
    if (x >= 0 && x < this.ancho && y >= 0 && y < this.alto) {
      this.filas[y][x] = car;
    }
  }

  fillRect(x, y, an, al, car = "#") {
    for (let dy = 0; dy < al; dy++) {
      for (let dx = 0; dx < an; dx++) {
        this.punto(x + dx, y + dy, car);
      }
    }
  }

  mostrar() {
    console.log(this.filas.map((fila) => fila.join("")).join("\n"));
  }
}

const lienzo = new LienzoTexto(12, 5);
lienzo.punto(0, 0, "O");       // el origen
lienzo.fillRect(3, 1, 4, 2);   // un rectángulo de 4 por 2
lienzo.punto(11, 4, "X");      // el último píxel
lienzo.mostrar();
```

```sortie
O...........
...####.....
...####.....
............
...........X
```

La `O` está arriba a la izquierda, y la `X` abajo a la derecha. El rectángulo empieza
en `x = 3`, `y = 1`, y baja.

Sigamos `fillRect(3, 1, 4, 2)` paso a paso. El bucle de `dy` (las filas) contiene el
bucle de `dx` (las columnas):

| Paso | dy | dx | Casilla pintada `(x + dx, y + dy)` |
|---|---|---|---|
| 1 | 0 | 0 | (3, 1) |
| 2 | 0 | 1 | (4, 1) |
| 3 | 0 | 2 | (5, 1) |
| 4 | 0 | 3 | (6, 1) |
| 5 | 1 | 0 | (3, 2) |
| 6 | 1 | 1 | (4, 2) |
| 7 | 1 | 2 | (5, 2) |
| 8 | 1 | 3 | (6, 2) |

Ocho casillas: 4 de ancho por 2 de alto.

Prueba: cambia `lienzo.fillRect(3, 1, 4, 2)` por `lienzo.fillRect(10, 3, 4, 4)`.
El rectángulo se sale del lienzo: solo se pintan las casillas de dentro (ese es el
papel del `if` de `punto`). Un canvas de verdad hace lo mismo, sin error.

> **Recuerda:** una cuadrícula de caracteres es un «canvas de los pobres». Permite
> comprobar la lógica de un dibujo con un simple `console.log`.

## Calcular posiciones

Lo más difícil de un dibujo son los cálculos: dónde colocar cada forma. Guárdalos en
**funciones puras** (reciben números, devuelven números). Funcionan en todas partes, y
puedes comprobarlas en Node.

### Centrar un rectángulo

Para centrar, partes del medio del lienzo y retrocedes la mitad del rectángulo.

```javascript
function centrar(anchoLienzo, altoLienzo, an, al) {
  return {
    x: (anchoLienzo - an) / 2,
    y: (altoLienzo - al) / 2,
  };
}

console.log(centrar(300, 150, 100, 50));
console.log(centrar(12, 5, 4, 1));
```

```sortie
{ x: 100, y: 50 }
{ x: 4, y: 2 }
```

| Cálculo | Valor |
|---|---|
| `(300 - 100) / 2` | `100` |
| `(150 - 50) / 2` | `50` |

Así que el rectángulo va de `x = 100` a `x = 200`: quedan 100 píxeles a cada lado.

### Una fila de cuadrados

Quieres 4 cuadrados de 20 píxeles, separados por 10 píxeles. El cuadrado número `i`
empieza en `margen + i * (tamano + espacio)`.

```javascript
function fila(cantidad, tamano, espacio, margen) {
  const posiciones = [];
  for (let i = 0; i < cantidad; i++) {
    posiciones.push(margen + i * (tamano + espacio));
  }
  return posiciones;
}

console.log(fila(4, 20, 10, 5));
```

```sortie
[ 5, 35, 65, 95 ]
```

| i | `i * (20 + 10)` | `+ 5` | x del cuadrado |
|---|---|---|---|
| 0 | 0 | 5 | 5 |
| 1 | 30 | 35 | 35 |
| 2 | 60 | 65 | 65 |
| 3 | 90 | 95 | 95 |

En el navegador, solo queda dibujar:

```javascript norun
for (const x of fila(4, 20, 10, 5)) {
  ctx.fillRect(x, 10, 20, 20);
}
```

Prueba: llama a `fila(4, 20, 0, 5)`. El espacio es cero: `[ 5, 25, 45, 65 ]`, los
cuadrados se tocan y forman una sola banda.

> **Recuerda:** separa los cálculos (funciones puras, que se pueden probar) de las
> órdenes de dibujo (`ctx.fillRect`…). El navegador solo sirve para pintar.

## Animar: un paso a la vez

Un dibujo animado es una serie de imágenes fijas. Para animar una pelota, repites sin
parar tres cosas:

```text
 ┌──────────────────────────────────────────┐
 │ 1. borrar el lienzo         (clearRect)  │
 │ 2. calcular la posición siguiente (paso) │◀──┐
 │ 3. dibujar la pelota        (arc, fill)  │   │
 └──────────────────────┬───────────────────┘   │
                        └── imagen siguiente ───┘
```

En el navegador, `requestAnimationFrame` llama a tu función justo antes de cada nueva
imagen de la pantalla (unas 60 veces por segundo):

```javascript norun
let pelota = { x: 20, y: 20, vx: 3, vy: 2 };

function imagen() {
  ctx.clearRect(0, 0, lienzo.width, lienzo.height);     // 1. borrar
  pelota = paso(pelota, lienzo.width, lienzo.height);   // 2. calcular
  ctx.beginPath();                                       // 3. dibujar
  ctx.arc(pelota.x, pelota.y, 5, 0, 2 * Math.PI);
  ctx.fill();
  requestAnimationFrame(imagen);                         // y volvemos a empezar
}

requestAnimationFrame(imagen);
```

### La función paso

La pelota tiene una posición (`x`, `y`) y una **velocidad** (`vx`, `vy`): cuánto se
mueve en cada imagen. Si el próximo movimiento la sacara del lienzo, inviertes la
velocidad: rebota, como una pelota de ping-pong contra una pared.

```javascript
function paso(pelota, ancho, alto) {
  let { x, y, vx, vy } = pelota;
  if (x + vx < 0 || x + vx > ancho - 1) vx = -vx;   // pared izquierda o derecha
  if (y + vy < 0 || y + vy > alto - 1) vy = -vy;    // techo o suelo
  return { x: x + vx, y: y + vy, vx, vy };
}

let pelota = { x: 0, y: 0, vx: 3, vy: 1 };
for (let i = 1; i <= 7; i++) {
  pelota = paso(pelota, 10, 3);
  console.log(i, pelota.x, pelota.y, pelota.vx, pelota.vy);
}
```

```sortie
1 3 1 3 1
2 6 2 3 1
3 9 1 3 -1
4 6 0 -3 -1
5 3 1 -3 1
6 0 2 -3 1
7 3 1 3 -1
```

El lienzo mide 10 casillas de ancho (`x` de 0 a 9) y 3 de alto (`y` de 0 a 2).
Sigamos cada llamada:

| Paso | x | y | vx | vy | Qué pasa |
|---|---|---|---|---|---|
| inicio | 0 | 0 | 3 | 1 | |
| 1 | 3 | 1 | 3 | 1 | nada especial |
| 2 | 6 | 2 | 3 | 1 | la pelota toca el suelo (`y = 2`) |
| 3 | 9 | 1 | 3 | -1 | `2 + 1 = 3 > 2`: rebote en el suelo, `vy` pasa a `-1` |
| 4 | 6 | 0 | -3 | -1 | `9 + 3 = 12 > 9`: rebote en la pared derecha |
| 5 | 3 | 1 | -3 | 1 | `0 - 1 < 0`: rebote en el techo |
| 6 | 0 | 2 | -3 | 1 | la pelota toca la pared izquierda |
| 7 | 3 | 1 | 3 | -1 | dos rebotes a la vez: pared izquierda y suelo |

En cada paso, compruebas **antes** de moverte. Así la pelota nunca sale.

### Ver la trayectoria

En un lienzo de caracteres, escribes el número de cada paso en lugar de la pelota:

```javascript
function paso(pelota, ancho, alto) {
  let { x, y, vx, vy } = pelota;
  if (x + vx < 0 || x + vx > ancho - 1) vx = -vx;
  if (y + vy < 0 || y + vy > alto - 1) vy = -vy;
  return { x: x + vx, y: y + vy, vx, vy };
}

const filas = Array.from({ length: 3 }, () => Array(10).fill("."));
let pelota = { x: 0, y: 0, vx: 3, vy: 1 };
filas[pelota.y][pelota.x] = "0";
for (let i = 1; i <= 6; i++) {
  pelota = paso(pelota, 10, 3);
  filas[pelota.y][pelota.x] = String(i);
}
console.log(filas.map((f) => f.join("")).join("\n"));
```

```sortie
0.....4...
...5.....3
6.....2...
```

Lee las cifras en orden: `0`, `2`, `3`… Falta el `1`: la casilla del paso 1 quedó
tapada por el `5` (misma posición `(3, 1)`), como la pintura fresca tapa la vieja.

Prueba: empieza con `vx: 2`. La pelota ya no rebota en el mismo sitio, y el camino
dibujado cambia.

> **Recuerda:** una animación = borrar, calcular, dibujar, volver a empezar. La función
> `paso` es pura: puedes probarla en Node, paso a paso.

## Reaccionar a un clic

Un clic da coordenadas en la **ventana**. Para saber dónde cae en el lienzo, restas la
posición del lienzo en la página.

```javascript norun
lienzo.addEventListener("click", (event) => {
  const marco = lienzo.getBoundingClientRect();  // dónde está el lienzo en pantalla
  const x = event.clientX - marco.left;
  const y = event.clientY - marco.top;
  console.log(casillaClicada(x, y, 20));
});
```

Encontrar la casilla de un tablero es un cálculo puro: divides por el tamaño de una
casilla y te quedas con la parte entera con `Math.floor`.

```javascript
function casillaClicada(x, y, tamano) {
  return { columna: Math.floor(x / tamano), fila: Math.floor(y / tamano) };
}

console.log(casillaClicada(5, 5, 20));
console.log(casillaClicada(45, 12, 20));
console.log(casillaClicada(60, 99, 20));
```

```sortie
{ columna: 0, fila: 0 }
{ columna: 2, fila: 0 }
{ columna: 3, fila: 4 }
```

| Clic | `x / 20` | `y / 20` | Casilla |
|---|---|---|---|
| (5, 5) | 0.25 → 0 | 0.25 → 0 | columna 0, fila 0 |
| (45, 12) | 2.25 → 2 | 0.6 → 0 | columna 2, fila 0 |
| (60, 99) | 3 → 3 | 4.95 → 4 | columna 3, fila 4 |

> **Recuerda:** primero convierte el clic en coordenadas del lienzo, y luego deja que
> una función pura haga el resto.

## Errores frecuentes

### No se encuentra el lienzo

```javascript norun
const lienzo = document.querySelector("#leinzo");
const ctx = lienzo.getContext("2d");
```

```text
Uncaught TypeError: Cannot read properties of null (reading 'getContext')
```

`querySelector` no encontró ningún elemento con `id="leinzo"` (una errata) y devolvió
`null`. Comprueba el `id` en el HTML, y carga el script después del lienzo
(`type="module"`).

### Un contexto mal escrito

```javascript norun
const ctx = lienzo.getContext("2D");
ctx.fillRect(0, 0, 10, 10);
```

```text
Uncaught TypeError: Cannot read properties of null (reading 'fillRect')
```

El nombre del contexto se escribe en minúsculas: `"2d"`. Con un nombre desconocido,
`getContext` devuelve `null`, sin error. El error solo llega en la línea siguiente.

### Olvidar beginPath

```javascript norun
ctx.arc(50, 50, 10, 0, 2 * Math.PI);
ctx.fill();
ctx.arc(150, 50, 10, 0, 2 * Math.PI);   // sin beginPath antes
ctx.fill();
```

No hay mensaje de error. Pero una línea une los dos círculos y la forma queda rara: el
segundo `arc` se añadió al **mismo** trazado. Empieza cada forma con
`ctx.beginPath()`.

### Agrandar el lienzo con CSS

```html norun
<canvas id="lienzo" style="width: 600px; height: 300px"></canvas>
```

Tampoco hay error, pero el dibujo sale borroso y deformado. Sin `width` ni `height`,
el lienzo mide 300 × 150 píxeles; el CSS lo estira como una foto ampliada. Da el
tamaño con los atributos `width="600" height="300"`.

## Ponte a prueba

**Pregunta 1:** en un lienzo de 300 × 150, ¿dónde está el punto `(0, 150)`?

> **Respuesta**
>
> Abajo a la izquierda. `x = 0` es el borde izquierdo, e `y = 150` está abajo del
> todo, porque `y` baja.

**Pregunta 2:** una pelota está en `x = 8` con `vx = 3`, en un lienzo de 10 de ancho
(`x` de 0 a 9). ¿Qué da `paso` para `x` y `vx`?

> **Respuesta**
>
> `8 + 3 = 11`, mayor que `9`: la pelota rebota. `vx` pasa a `-3`, y `x` pasa a
> `8 - 3 = 5`.

**Pregunta 3:** escribe una función `columna(x, tamano, espacio, margen)` que dé la
posición horizontal de la columna número `x` de una cuadrícula, y luego muestra las 3
primeras para un tamaño de 10, un espacio de 2 y un margen de 4.

> **Respuesta**
>
> ```javascript
> function columna(x, tamano, espacio, margen) {
>   return margen + x * (tamano + espacio);
> }
>
> console.log([0, 1, 2].map((x) => columna(x, 10, 2, 4)));
> ```
>
> ```sortie
> [ 4, 16, 28 ]
> ```

## En resumen

- `<canvas width height>` es un lienzo; `getContext("2d")` da el pincel `ctx`.
- `(0, 0)` está arriba a la izquierda, e `y` aumenta hacia abajo.
- `fillRect`, `arc`, `lineTo`, `fillText` dibujan; `beginPath` empieza una forma.
- Guarda los cálculos en funciones puras y compruébalos en Node, si hace falta con un
  lienzo de caracteres.
- Una animación: borrar, calcular (`paso`), dibujar, con `requestAnimationFrame`.

Siguiente: [guardar datos en el navegador](stockage.md)
