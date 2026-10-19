# Fechas y horas

Esta página muestra cómo crear, leer, calcular y mostrar fechas en TypeScript, evitando la trampa de los meses y la de las zonas horarias.

> **Objetivos:**
>
> - sabrás qué es un objeto `Date` y cómo crear una fecha precisa;
> - sabrás evitar la trampa de los meses numerados desde 0;
> - sabrás trabajar en UTC para obtener el mismo resultado en todas partes;
> - sabrás mostrar una fecha en español con `Intl.DateTimeFormat`;
> - sabrás calcular una diferencia en días y sumar días;
> - sabrás guardar estos cálculos en funciones bien tipadas.

TypeScript usa el objeto `Date` de JavaScript: todo lo que dice
[el capítulo de JavaScript sobre las fechas](../javascript/dates.md) sigue siendo
cierto aquí. Esta página repasa lo esencial, paso a paso, y muestra lo que añaden los
**tipos**.

## Una fecha es un número

Para el ordenador, una fecha es **un solo número**: los milisegundos transcurridos
desde el 1 de enero de 1970 a medianoche (tiempo universal). Imagina un cronómetro
puesto en marcha ese día y que nunca se ha detenido. Una fecha es lo que el cronómetro
mostraba en un momento dado.

```text
   1 ene. 1970                                     5 mar. 2026, 14:30
   00:00 UTC
       │─────────────── 1 772 721 000 000 ms ─────────────────│
       0                                                  getTime()
```

```typescript
const zero: Date = new Date(0);
console.log(zero.toISOString());

const cita: Date = new Date(Date.UTC(2026, 2, 5, 14, 30));
const milisegundos: number = cita.getTime();
console.log(milisegundos);
console.log(cita.toISOString());
```

```sortie
1970-01-01T00:00:00.000Z
1772721000000
2026-03-05T14:30:00.000Z
```

- `Date` es un **tipo**: una variable de tipo `Date` contiene un objeto fecha;
- `getTime()` devuelve el número del cronómetro, de tipo `number`;
- `toISOString()` escribe la fecha en formato **ISO 8601**: año-mes-día, una `T`,
  la hora, y `Z`, que significa «tiempo universal».

> **Nota:** `new Date()` **sin argumentos** da el instante actual. Es útil en un programa
> real, pero el resultado cambia en cada ejecución. Por eso todos los ejemplos de esta
> página usan fechas fijas.

> **Recuerda:** un objeto `Date` envuelve un número de milisegundos. Todo lo demás
> (día, mes, hora) se calcula a partir de ese número.

## Crear una fecha precisa

### Con Date.UTC

`Date.UTC(año, mes, día, hora, minuto)` calcula el número del cronómetro para una
fecha dada en tiempo universal. Después se lo pasas a `new Date`.

```typescript
const navidad = new Date(Date.UTC(2026, 11, 25));
const mediodia = new Date(Date.UTC(2026, 6, 14, 12, 0));
console.log(navidad.toISOString());
console.log(mediodia.toISOString());
```

```sortie
2026-12-25T00:00:00.000Z
2026-07-14T12:00:00.000Z
```

Fíjate bien: Navidad se escribe con `11`, y el 14 de julio con `6`. No es una errata.

### La trampa de los meses

En `Date.UTC` y `new Date(año, mes, …)`, **los meses empiezan en 0**. Enero vale 0,
diciembre vale 11. Los días, en cambio, sí empiezan en 1.

Es como los huecos de una caja de huevos numerados desde 0: el primer hueco es el
hueco 0.

```text
   mes real:    ene   feb   mar  abr  may  jun   jul   ago   sep  oct  nov  dic
   número JS:     0     1     2    3    4     5     6     7     8    9   10   11
```

```typescript
const meses = [0, 1, 11, 12];
for (const m of meses) {
  const d = new Date(Date.UTC(2026, m, 1));
  console.log(`número ${m} -> ${d.toISOString().slice(0, 10)}`);
}
```

```sortie
número 0 -> 2026-01-01
número 1 -> 2026-02-01
número 11 -> 2026-12-01
número 12 -> 2027-01-01
```

| Paso | `m` | Mes real | Fecha obtenida |
|------|-----|----------|----------------|
| 1 | 0 | enero | 2026-01-01 |
| 2 | 1 | febrero | 2026-02-01 |
| 3 | 11 | diciembre | 2026-12-01 |
| 4 | 12 | (no hay mes 13) | 2027-01-01 |

El número 12 no provoca **ningún error**: la fecha se desborda hacia enero del año
siguiente. TypeScript no puede protegerte aquí, porque `12` es un `number` válido.

Prueba: cambia `12` por `13`. Obtienes `2027-02-01`.

> **Consejo:** cuando tengas un mes «humano» (de 1 a 12), escribe siempre `mes - 1` al
> crear la fecha, y `+ 1` al leerlo.

### A partir de un texto ISO

Un texto en formato ISO se lee directamente, con los meses **normales** (03 = marzo).
Termínalo con `Z` para decir «tiempo universal»:

```typescript
const d = new Date("2026-03-05T14:30:00Z");
console.log(d.toISOString());
console.log(d.getTime() === Date.UTC(2026, 2, 5, 14, 30));
```

```sortie
2026-03-05T14:30:00.000Z
true
```

> **Atención:** un texto en otro formato, como `"05/03/2026"`, da una fecha **inválida**
> (o peor, una fecha equivocada). Usa el formato ISO, o divide el texto tú mismo (mira
> [las expresiones regulares](regex.md#devolver-t-o-null)).

> **Recuerda:** con números, los meses van de 0 a 11. Con un texto ISO, van de 01 a 12.

## UTC u hora local

**UTC** es la hora de referencia mundial, la misma para todo el mundo, sin horario de
verano. La **hora local** depende del lugar donde se ejecuta el programa: en Madrid,
va una o dos horas por delante de UTC.

```text
   el mismo instante, visto desde tres ciudades

   UTC         ──────●──────  14:30
   Madrid      ──────●──────  15:30
   Nueva York  ──────●──────  09:30
```

Por eso el objeto `Date` tiene **dos familias** de métodos:

| Hora local (a evitar aquí) | UTC (usada en esta página) |
|----------------------------|----------------------------|
| `getHours()` | `getUTCHours()` |
| `getDate()` | `getUTCDate()` |
| `setDate()` | `setUTCDate()` |
| `new Date(2026, 2, 5)` | `new Date(Date.UTC(2026, 2, 5))` |

Los métodos «locales» dan un resultado distinto según el ordenador. Un programa que
funciona en Madrid puede equivocarse de día en un servidor de Tokio. Usa siempre la
familia **UTC** para calcular, y pasa a una zona horaria concreta solo en el momento de
**mostrar**.

> **Recuerda:** calcula en UTC (`getUTC…`, `setUTC…`, `Date.UTC`). Así el resultado es
> el mismo en todas las máquinas.

## Leer las partes de una fecha

Cada método `getUTC…` devuelve un `number`:

```typescript
const d = new Date(Date.UTC(2026, 2, 5, 14, 30));

console.log("año:", d.getUTCFullYear());
console.log("mes:", d.getUTCMonth(), "(o sea, marzo)");
console.log("día:", d.getUTCDate());
console.log("hora:", d.getUTCHours());
console.log("minuto:", d.getUTCMinutes());
console.log("día de la semana:", d.getUTCDay());
```

```sortie
año: 2026
mes: 2 (o sea, marzo)
día: 5
hora: 14
minuto: 30
día de la semana: 4
```

`getUTCDay()` da el día de la semana: **0 para el domingo**, 1 para el lunes… y 4 para
el jueves. El 5 de marzo de 2026 es jueves.

Para obtener el nombre del día, basta con un array:

```typescript
const DIAS = ["domingo", "lunes", "martes", "miércoles", "jueves", "viernes", "sábado"] as const;

const d = new Date(Date.UTC(2026, 2, 5));
const nombre = DIAS[d.getUTCDay()];
console.log(nombre);
```

```sortie
jueves
```

Prueba: cambia el `5` por `8`. Obtienes `domingo`.

> **Recuerda:** `getUTCMonth()` devuelve de 0 a 11, `getUTCDay()` devuelve de 0
> (domingo) a 6 (sábado).

## Mostrar una fecha en español

`Intl.DateTimeFormat` da formato a una fecha según un idioma. Indica `"es-ES"` para el
español de España, y **siempre** `timeZone: "UTC"` para obtener la misma presentación
en todas partes.

```typescript
const d = new Date(Date.UTC(2026, 2, 5, 14, 30));

const corto = new Intl.DateTimeFormat("es-ES", { timeZone: "UTC" });
const largo = new Intl.DateTimeFormat("es-ES", { timeZone: "UTC", dateStyle: "long" });
const completo = new Intl.DateTimeFormat("es-ES", {
  timeZone: "UTC",
  dateStyle: "full",
  timeStyle: "short",
});

console.log(corto.format(d));
console.log(largo.format(d));
console.log(completo.format(d));
```

```sortie
5/3/2026
5 de marzo de 2026
jueves, 5 de marzo de 2026, 14:30
```

### Opciones comprobadas por TypeScript

Las opciones tienen un tipo: `Intl.DateTimeFormatOptions`. Así que TypeScript conoce
los valores permitidos. Una errata, como `dateStyle: "Long"`, se rechaza incluso
antes de la ejecución (mira los [errores frecuentes](#errores-frecuentes)).

Puedes guardar tus ajustes en una variable tipada y elegir cada parte:

```typescript
const opciones: Intl.DateTimeFormatOptions = {
  timeZone: "UTC",
  weekday: "long",
  day: "numeric",
  month: "long",
};

const formato = new Intl.DateTimeFormat("es-ES", opciones);
console.log(formato.format(new Date(Date.UTC(2026, 6, 14))));
console.log(formato.format(new Date(Date.UTC(2026, 11, 25))));
```

```sortie
martes, 14 de julio
viernes, 25 de diciembre
```

Prueba: añade `year: "numeric"` a las opciones. Las dos líneas terminan entonces con
`2026`.

> **Recuerda:** `Intl.DateTimeFormat("es-ES", { timeZone: "UTC", … })` muestra en
> español; el tipo `Intl.DateTimeFormatOptions` comprueba tus opciones.

## Calcular con fechas

### Comparar dos fechas

Para comparar, compara los números del cronómetro, con `getTime()`:

```typescript
const salida = new Date(Date.UTC(2026, 0, 10));
const llegada = new Date(Date.UTC(2026, 0, 24));

console.log(salida.getTime() < llegada.getTime());
console.log(salida.getTime() === new Date("2026-01-10T00:00:00Z").getTime());
console.log(salida === new Date("2026-01-10T00:00:00Z"));
```

```sortie
true
true
false
```

La última línea muestra `false`: `===` compara dos **objetos**, no su contenido. Son
dos cajas distintas, aunque contengan el mismo número.

### La diferencia en días

Un día dura 24 × 60 × 60 × 1000 = 86 400 000 milisegundos. Para obtener una diferencia
en días, se restan los dos números y luego se divide.

```typescript
const MS_POR_DIA = 24 * 60 * 60 * 1000;

const salida = new Date(Date.UTC(2026, 0, 10));
const llegada = new Date(Date.UTC(2026, 0, 24));

const diferenciaMs = llegada.getTime() - salida.getTime();
const dias = Math.round(diferenciaMs / MS_POR_DIA);
console.log(diferenciaMs);
console.log(`${dias} días`);
```

```sortie
1209600000
14 días
```

| Paso | Cálculo | Valor |
|------|---------|-------|
| 1 | `salida.getTime()` | 1 768 003 200 000 |
| 2 | `llegada.getTime()` | 1 769 212 800 000 |
| 3 | `diferenciaMs` = 2 − 1 | 1 209 600 000 |
| 4 | `diferenciaMs / MS_POR_DIA` | 14 |

`Math.round` redondea al día más cercano: útil si las horas no son exactamente las
mismas.

> **Nota:** en JavaScript, se ve a menudo `llegada - salida`. TypeScript lo
> **rechaza**: no se restan dos objetos. Pasa por `getTime()` (mira los
> [errores frecuentes](#errores-frecuentes)).

### Sumar días

Para sumar días, se **copia** la fecha y luego se cambia el día de la copia con
`setUTCDate`. Si el día pasa del final del mes, la fecha pasa sola al mes siguiente.

```typescript
const factura = new Date(Date.UTC(2026, 0, 20));

const vencimiento = new Date(factura.getTime());
vencimiento.setUTCDate(vencimiento.getUTCDate() + 30);

console.log(factura.toISOString().slice(0, 10));
console.log(vencimiento.toISOString().slice(0, 10));
```

```sortie
2026-01-20
2026-02-19
```

| Paso | Instrucción | `vencimiento` |
|------|-------------|---------------|
| 1 | copia de `factura` | 2026-01-20 |
| 2 | `getUTCDate()` | 20 |
| 3 | `20 + 30` | 50 |
| 4 | `setUTCDate(50)`: enero solo tiene 31 días, 50 − 31 = 19 | 2026-02-19 |

¿Por qué copiar? Porque `setUTCDate` **modifica** el objeto. Sin copia, la fecha de la
factura también cambiaría. Es como escribir sobre el original de un documento en lugar
de escribir sobre una fotocopia.

Prueba: cambia `+ 30` por `- 25`. Obtienes `2025-12-26`: el cálculo retrocede incluso
un año.

> **Recuerda:** compara y resta con `getTime()`. Para sumar días, copia la fecha y
> luego usa `setUTCDate`.

## Funciones de fechas bien tipadas

Los cálculos con fechas son fáciles de estropear. El buen reflejo: escribirlos **una
vez**, en pequeñas funciones tipadas, y no volver a tocarlos. El resto del programa
llama a esas funciones.

```typescript
const MS_POR_DIA = 24 * 60 * 60 * 1000;

// Lee "2026-03-05"; devuelve null si el texto no es una fecha real.
function leerDia(texto: string): Date | null {
  if (!/^\d{4}-\d{2}-\d{2}$/.test(texto)) {
    return null;
  }
  const d = new Date(`${texto}T00:00:00Z`);
  if (Number.isNaN(d.getTime()) || d.toISOString().slice(0, 10) !== texto) {
    return null;
  }
  return d;
}

function sumarDias(fecha: Date, dias: number): Date {
  const copia = new Date(fecha.getTime());
  copia.setUTCDate(copia.getUTCDate() + dias);
  return copia;
}

function diferenciaEnDias(inicio: Date, fin: Date): number {
  return Math.round((fin.getTime() - inicio.getTime()) / MS_POR_DIA);
}

const enEspanol = new Intl.DateTimeFormat("es-ES", { timeZone: "UTC", dateStyle: "long" });

function mostrar(fecha: Date): string {
  return enEspanol.format(fecha);
}

for (const texto of ["2026-01-20", "2026-02-30", "20/01/2026"]) {
  const dia = leerDia(texto);
  if (dia === null) {
    console.log(`${texto}: fecha inválida`);
    continue;
  }
  const vencimiento = sumarDias(dia, 30);
  console.log(`${mostrar(dia)} -> pagar antes del ${mostrar(vencimiento)} (${diferenciaEnDias(dia, vencimiento)} días)`);
}
```

```sortie
20 de enero de 2026 -> pagar antes del 19 de febrero de 2026 (30 días)
2026-02-30: fecha inválida
20/01/2026: fecha inválida
```

Lo que garantiza cada función:

| Función | Recibe | Devuelve | Lo que evita |
|---------|--------|----------|--------------|
| `leerDia` | `string` | `Date \| null` | los textos mal formados y las fechas imposibles |
| `sumarDias` | `Date`, `number` | una **nueva** `Date` | modificar el original |
| `diferenciaEnDias` | `Date`, `Date` | `number` | la resta de objetos |
| `mostrar` | `Date` | `string` | olvidar `timeZone: "UTC"` |

¿Por qué se rechaza `2026-02-30`? `new Date` acepta este texto y lo convierte
discretamente en el 2 de marzo. `leerDia` vuelve a leer la fecha obtenida con
`toISOString()`: como ya no es el mismo texto, devuelve `null`.

Gracias al tipo `Date | null`, TypeScript **obliga** a quien llama a tratar el caso
inválido antes de usar la fecha (es el
[estrechamiento](../debuter/glossaire.md#estrechamiento)).

Prueba: borra el bloque `if (dia === null) { … }`. TypeScript rechaza el programa:
`dia` quizá sea `null`.

> **Recuerda:** encierra los cálculos con fechas en pequeñas funciones tipadas. Una
> función que lee un texto devuelve `Date | null`.

## ¿Y Temporal?

Una nueva API de JavaScript, **Temporal**, corrige los defectos de `Date`: meses que
empiezan en 1, objetos que no se pueden modificar y verdaderas zonas horarias. Todavía
no está disponible en Node 22 ni en los tipos de TypeScript 5.8: mientras tanto, usa
`Date` con las precauciones de esta página.

## Errores frecuentes

### Restar dos fechas directamente

```typescript norun
const salida = new Date(Date.UTC(2026, 0, 10));
const llegada = new Date(Date.UTC(2026, 0, 24));
const diferencia = llegada - salida;
console.log(diferencia);
```

```text
main.ts(3,20): error TS2362: The left-hand side of an arithmetic operation must be of type 'any', 'number', 'bigint' or an enum type.
main.ts(3,30): error TS2363: The right-hand side of an arithmetic operation must be of type 'any', 'number', 'bigint' or an enum type.
```

JavaScript acepta esta resta, TypeScript no: `llegada` y `salida` son objetos, no
números. Escribe `llegada.getTime() - salida.getTime()`.

### Equivocarse en una opción de presentación

```typescript norun
const formato = new Intl.DateTimeFormat("es-ES", { timeZone: "UTC", dateStyle: "Long" });
```

```text
main.ts(1,69): error TS2769: No overload matches this call.
  Overload 1 of 2, '(locales?: LocalesArgument, options?: DateTimeFormatOptions | undefined): DateTimeFormat', gave the following error.
    Type '"Long"' is not assignable to type '"long" | "short" | "medium" | "full" | undefined'. Did you mean '"long"'?
  Overload 2 of 2, '(locales?: string | string[] | undefined, options?: DateTimeFormatOptions | undefined): DateTimeFormat', gave the following error.
    Type '"Long"' is not assignable to type '"long" | "short" | "medium" | "full" | undefined'. Did you mean '"long"'?
```

El mensaje es largo, pero el final lo dice todo: `Did you mean '"long"'?`. Los valores
de las opciones están en inglés y en minúsculas: `"short"`, `"medium"`, `"long"`,
`"full"`.

### Pasar un texto en lugar de una fecha

```typescript norun
function sumarDias(fecha: Date, dias: number): Date {
  const copia = new Date(fecha.getTime());
  copia.setUTCDate(copia.getUTCDate() + dias);
  return copia;
}

console.log(sumarDias("2026-03-01", 7));
```

```text
main.ts(7,23): error TS2345: Argument of type 'string' is not assignable to parameter of type 'Date'.
```

Un texto no es una `Date`. Conviértelo primero, y comprueba el resultado:
`const d = leerDia("2026-03-01"); if (d) { sumarDias(d, 7); }`.

### Usar una fecha inválida

```typescript norun
const d = new Date("31/12/2026");
console.log(d.toISOString());
```

```text
RangeError: Invalid time value
```

**No** es un error de TypeScript: `new Date` siempre devuelve una `Date`, aunque sea
inválida. El error llega durante la ejecución. Usa el formato ISO (`"2026-12-31"`) y
comprueba con `Number.isNaN(d.getTime())`, como en `leerDia`.

## Ponte a prueba

**Pregunta 1:** ¿qué fecha muestra este programa?

```typescript
const d = new Date(Date.UTC(2026, 9, 1));
console.log(d.toISOString().slice(0, 10));
```

> **Respuesta**
>
> El 1 de **octubre**, porque los meses empiezan en 0: 9 es el décimo mes.
>
> ```sortie
> 2026-10-01
> ```

**Pregunta 2:** ¿por qué usar `getUTCDate()` en lugar de `getDate()`?

> **Respuesta**
>
> `getDate()` lee la fecha en la zona horaria del ordenador: el resultado puede cambiar
> de una máquina a otra. `getUTCDate()` da el mismo resultado en todas partes.

**Pregunta 3:** escribe una función `esAntes(a: Date, b: Date): boolean` y pruébala
con el 1 y el 2 de marzo de 2026.

> **Respuesta**
>
> ```typescript
> function esAntes(a: Date, b: Date): boolean {
>   return a.getTime() < b.getTime();
> }
>
> const uno = new Date(Date.UTC(2026, 2, 1));
> const dos = new Date(Date.UTC(2026, 2, 2));
> console.log(esAntes(uno, dos), esAntes(dos, uno));
> ```
>
> ```sortie
> true false
> ```

## En resumen

- Un objeto `Date` envuelve un número de milisegundos desde el 1 de enero de 1970 (UTC).
- Con `Date.UTC` y `new Date(año, mes, …)`, los meses van de **0 a 11**.
- Calcula en UTC (`getUTC…`, `setUTC…`) y muestra con
  `Intl.DateTimeFormat("es-ES", { timeZone: "UTC" })`.
- TypeScript rechaza `fecha1 - fecha2`: pasa por `getTime()`.
- Para sumar días, copia la fecha y luego usa `setUTCDate`.
- Guarda estos cálculos en pequeñas funciones tipadas, que devuelvan `Date | null`
  cuando la lectura puede fallar.

Siguiente: [una herramienta de línea de comandos tipada](ligne-de-commande.md).
