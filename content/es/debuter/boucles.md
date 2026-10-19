# Los bucles, paso a paso

Un bucle hace que el ordenador repita una acción sin copiar el código: esta página muestra, vuelta a vuelta, cómo avanza.

> **Objetivos:**
>
> - entender por qué se usa un bucle;
> - seguir un bucle «para cada» vuelta a vuelta;
> - contar con `range` y saber dónde se detiene;
> - calcular una suma, contar, encontrar el mayor;
> - escribir un bucle «mientras» sin que gire para siempre;
> - detener un bucle antes del final con `break`.

## Repetir sin copiar y pegar

Para mostrar «Hola» tres veces, puedes escribir tres veces
`print("Hola")`. Pero ¿y para 1000 veces? Copiar no es una solución.

Un [bucle](glossaire.md#bucle) le dice al ordenador: «vuelve a hacer
esto varias veces». Escribes la acción **una sola vez**.

```python
for vuelta in range(3):
    print("Hola")
```

```sortie
Hola
Hola
Hola
```

Para 1000 veces, solo cambias `3` por `1000`.

Piensa en una lavadora: no lavas cada calcetín a mano. La máquina repite
el mismo gesto por ti.

> **Recuerda:** un bucle evita copiar el mismo código. Escribes la acción
> una vez y el ordenador la repite.

## El bucle «para cada»

El bucle más sencillo toma una [lista](glossaire.md#lista) y pasa por
**cada elemento**, uno a uno. En Python, es `for`.

```python
frutas = ["manzana", "pera", "banana"]
for fruta in frutas:
    print("Me gusta la", fruta)
```

```sortie
Me gusta la manzana
Me gusta la pera
Me gusta la banana
```

Léelo en voz alta: «**para cada** fruta **en** frutas, muestra…».

### Lo que pasa, vuelta a vuelta

La variable `fruta` es una caja. En cada vuelta, el bucle mete **un
elemento nuevo** dentro y luego ejecuta el código con sangría.

| Vuelta | fruta | Lo que se muestra |
|--------|-------|-------------------|
| 1 | `"manzana"` | Me gusta la manzana |
| 2 | `"pera"` | Me gusta la pera |
| 3 | `"banana"` | Me gusta la banana |
| fin | – | no quedan elementos: el bucle se detiene |

Una vuelta también se llama [iteración](glossaire.md#iteracion). La
palabra da miedo, pero solo quiere decir «una vuelta».

### La sangría importa

En Python, las líneas **con sangría** bajo el `for` están dentro del
bucle. Las demás vienen después.

```python
for fruta in ["manzana", "pera"]:
    print("Dentro del bucle:", fruta)
print("Después del bucle")
```

```sortie
Dentro del bucle: manzana
Dentro del bucle: pera
Después del bucle
```

«Después del bucle» se muestra **una sola vez**: la línea no tiene
sangría. Este desplazamiento se llama [sangría](glossaire.md#indentacion).

Prueba: añade 4 espacios delante de la última línea. Entonces se muestra
dos veces, una por vuelta.

> **Recuerda:** «para cada» pasa por todos los elementos, uno a uno, y se
> detiene solo al final de la lista.

### El mismo bucle en otros lenguajes

La idea es la misma en todas partes. Solo cambian las palabras.

```javascript
for (const fruta of ["manzana", "pera"]) {
  console.log("Me gusta la", fruta);
}
```

```sortie
Me gusta la manzana
Me gusta la pera
```

```ruby
["manzana", "pera"].each do |fruta|
  puts "Me gusta la #{fruta}"
end
```

```sortie
Me gusta la manzana
Me gusta la pera
```

```go
package main

import "fmt"

func main() {
	for _, fruta := range []string{"manzana", "pera"} {
		fmt.Println("Me gusta la", fruta)
	}
}
```

```sortie
Me gusta la manzana
Me gusta la pera
```

En Go, `range` da dos cosas: la posición y el elemento. El `_` quiere
decir «la posición no la necesito».

## Contar con range

A menudo no tienes una lista: solo quieres contar. `range` fabrica una
serie de números, un [intervalo](glossaire.md#rango).

```python
for i in range(4):
    print(i)
```

```sortie
0
1
2
3
```

Sorpresa: obtienes 4 números, pero de **0 a 3**. No de 1 a 4.

### Por qué se detiene uno antes del final

`range(4)` quiere decir «4 números, empezando en 0». El 4 es el
**límite**: te detienes **antes** de él.

Piensa en una regla graduada. De 0 a 4 hay 4 centímetros. La marca «4»
indica el final: no es un centímetro más.

```text
 0    1    2    3    4
 |----|----|----|----|
  1.º  2.º  3.º  4.º     ← 4 casillas; el límite 4 no es una de ellas
```

También puedes dar un inicio: `range(inicio, límite)`.

```python
for i in range(1, 4):
    print(i)
```

```sortie
1
2
3
```

Así que para llegar **hasta** 3, escribes `4` como límite. Un tercer
número da el paso: `range(0, 10, 2)` da 0, 2, 4, 6, 8.

Prueba: escribe `range(1, 3)`. Solo tendrás 1 y 2.

> **Recuerda:** `range(1, 4)` da 1, 2, 3. El límite nunca está
> incluido.

### Contar en otros lenguajes

En JavaScript y en Go, escribes tres partes: el inicio, la condición para
continuar y el paso. `for (let i = 1; i < 4; i++)` también da 1, 2, 3
(`i++` quiere decir «suma 1 a `i`»).

En Ruby, `(1..3)` va de 1 a 3, final **incluido** (dos puntos). Con tres
puntos, `(1...3)`, se detiene antes, como Python.

## Acumular: la suma

Este es el uso más común de un bucle: **sumar** números. Guardas una caja
`total` y le añades cada número.

Es como una hucha. Al principio está vacía. Con cada moneda, el contenido
crece.

```python
precios = [3, 5, 2]
total = 0
for p in precios:
    total = total + p
print(total)
```

```sortie
10
```

`total = total + p` se lee: «toma el total anterior, súmale `p` y guarda
el resultado en `total`».

| Paso | p | total antes | total después |
|------|---|-------------|---------------|
| inicio | – | – | 0 |
| vuelta 1 | 3 | 0 | 3 |
| vuelta 2 | 5 | 3 | 8 |
| vuelta 3 | 2 | 8 | 10 |

A menudo se escribe `total += p`: es lo mismo, pero más corto.

Prueba: añade `print(total)` **dentro** del bucle (con sangría). Verás
cómo crece la hucha: 3, luego 8, luego 10.

> **Recuerda:** para una suma, pones `total = 0` **antes** del bucle,
> sumas **dentro** del bucle y muestras el resultado **después**.

## Acumular: el contador y el mayor

Un **contador** es una suma en la que siempre se añade 1. Contemos las
notas por encima de 10.

```python
notas = [12, 8, 15]
contador = 0
for nota in notas:
    if nota > 10:
        contador += 1
print(contador)
```

```sortie
2
```

| Paso | nota | ¿nota > 10? | contador |
|------|------|-------------|----------|
| inicio | – | – | 0 |
| vuelta 1 | 12 | sí | 1 |
| vuelta 2 | 8 | no | 1 |
| vuelta 3 | 15 | sí | 2 |

El `if` está **dentro** del bucle: se hace la pregunta en cada vuelta.
Las condiciones se explican en [la lógica](logique.md).

Para el **mayor**, guardas «el mayor visto hasta ahora». Empiezas por el
**primer** elemento, no por 0.

```python
notas = [12, 8, 15]
mayor = notas[0]
for nota in notas:
    if nota > mayor:
        mayor = nota
print(mayor)
```

```sortie
15
```

| Paso | nota | ¿nota > mayor? | mayor |
|------|------|----------------|-------|
| inicio | – | – | 12 |
| vuelta 1 | 12 | no | 12 |
| vuelta 2 | 8 | no | 12 |
| vuelta 3 | 15 | sí | 15 |

Este ejemplo se explica en detalle, en cinco lenguajes, en [pensar como
un programador](algorithmes.md).

## El bucle «mientras»

A veces no sabes **cuántas** vueltas dar. Solo sabes **cuándo parar**. Es
el bucle «mientras»: `while`.

«**Mientras** el agua no esté caliente, espera». No sabes cuántos minutos:
lo compruebas cada vez.

```python
contador = 1
while contador <= 3:
    print(contador)
    contador = contador + 1
print("Terminado")
```

```sortie
1
2
3
Terminado
```

Antes de cada vuelta, Python hace la pregunta `contador <= 3`. Sí: da una
vuelta. No: sale.

| Paso | contador | ¿contador <= 3? | Acción |
|------|----------|-----------------|--------|
| 1 | 1 | sí | muestra 1, contador pasa a 2 |
| 2 | 2 | sí | muestra 2, contador pasa a 3 |
| 3 | 3 | sí | muestra 3, contador pasa a 4 |
| 4 | 4 | no | sale del bucle |

### El bucle infinito

Si nada cambia en el bucle, la pregunta siempre tiene la misma respuesta.
El bucle no se detiene **nunca**: es un **bucle infinito**.

Que no cunda el pánico: pulsa **Ctrl + C** en el [terminal](terminal.md)
y el programa se detiene. Hay un ejemplo en los errores frecuentes, más
abajo.

> **Recuerda:** en un bucle `while`, algo tiene que cambiar en cada
> vuelta; si no, nunca se detiene.

### «Mientras» en otros lenguajes

```javascript
let contador = 1;
while (contador <= 2) {
  console.log(contador);
  contador++;
}
```

```sortie
1
2
```

Go no tiene la palabra `while`: escribe `for contador <= 2 { … }`.

## Parar antes con break

Buscas tus llaves en unos cajones. En cuanto las encuentras, paras. No
hace falta abrir los cajones siguientes.

Eso hace `break`: **sale del bucle enseguida**.

```python
cajones = ["bolígrafos", "llaves", "pilas"]
for cajon in cajones:
    print("Abro:", cajon)
    if cajon == "llaves":
        print("¡Encontradas!")
        break
print("Me voy.")
```

```sortie
Abro: bolígrafos
Abro: llaves
¡Encontradas!
Me voy.
```

El cajón «pilas» no se abre nunca. `break` se escribe igual en
JavaScript, en Ruby y en Go.

Prueba: cambia `"llaves"` por `"pilas"` en el `if`. Se abren los tres
cajones.

> **Recuerda:** `break` detiene el bucle inmediatamente. El programa
> sigue después del bucle.

## Errores frecuentes

### El bucle infinito

Te olvidas de hacer avanzar el contador.

```python norun
contador = 1
while contador <= 3:
    print(contador)
```

El programa muestra `1` sin fin. Pulsa **Ctrl + C**. Python muestra
entonces:

```text
1
1
Traceback (most recent call last):
  File "bucle.py", line 3, in <module>
    print(contador)
    ~~~~~^^^^^^^^^^
KeyboardInterrupt
```

`KeyboardInterrupt` quiere decir «detenido desde el teclado»: eres tú.
Para corregirlo, añade `contador = contador + 1` dentro del bucle (con
sangría).

### Una vuelta de menos

Quieres los números del 1 al 3 y escribes `range(1, 3)`.

```python
for i in range(1, 3):
    print(i)
```

```sortie
1
2
```

No hay mensaje de error, pero falta el 3: el límite nunca está incluido.
Corrección: `range(1, 4)`. Este error de una vuelta es tan común que
tiene un nombre en inglés: *off-by-one* («desfasado en uno»).

### Volver a poner el total a cero dentro del bucle

El `total = 0` tiene sangría: está **dentro** del bucle.

```python
precios = [3, 5, 2]
for p in precios:
    total = 0
    total = total + p
print(total)
```

```sortie
2
```

Esperabas 10. En cada vuelta, se vacía la hucha y luego se añade una
sola moneda. Al final, solo queda el último precio.

| Vuelta | p | después de `total = 0` | después de `total + p` |
|--------|---|------------------------|------------------------|
| 1 | 3 | 0 | 3 |
| 2 | 5 | 0 | 5 |
| 3 | 2 | 0 | 2 |

Corrección: pon `total = 0` **antes** del `for`, sin sangría.

## Ponte a prueba

**1.** ¿Qué muestra `for i in range(3): print(i)`?

> **Respuesta**
>
> `0`, luego `1`, luego `2`, cada uno en una línea. `range(3)` empieza en
> 0 y se detiene antes de 3.

**2.** ¿Cuándo conviene preferir `while` a `for`?

> **Respuesta**
>
> Cuando no sabes de antemano cuántas vueltas dar, pero sabes cuándo
> parar. Por ejemplo: «mientras el número sea menor que 100».

**3.** Con `numeros = [4, 1, 6]`, `total = 0` antes del bucle y
`total += n` dentro, ¿qué valores toma `total`?

> **Respuesta**
>
> 0 al principio, luego 4, luego 5, luego 11. El resultado es 11.

## En resumen

- Un bucle repite una acción sin copiar el código.
- `for` pasa por cada elemento de una lista, uno a uno.
- `range(1, 4)` da 1, 2, 3: el límite nunca está incluido.
- Para una suma: `total = 0` antes, `total += x` dentro, `print` después.
- `while` repite mientras una condición sea verdadera: algo tiene que cambiar; si no, Ctrl + C.
- `break` sale del bucle enseguida.

Siguiente: [las funciones, sin jerga](fonctions.md).
