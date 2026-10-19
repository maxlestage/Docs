# Cómo funciona un ordenador

Un ordenador lo guarda todo en forma de números y los transforma muy rápido: esta página explica el procesador, la memoria, los archivos, los bytes y el papel del sistema operativo, para entender mejor lo que hacen tus programas.

> **Objetivos:**
>
> - saber para qué sirven el procesador, la memoria RAM y el disco;
> - leer una ruta de archivo y entender el árbol de carpetas;
> - entender los bits y los bytes, y contar en binario;
> - ver cómo se guarda un texto en forma de números (ASCII, Unicode);
> - distinguir el sistema operativo, un programa y un proceso.

## Las grandes piezas del ordenador

Imagina una cocina:

- el **procesador** es el cocinero: hace el trabajo;
- la **memoria RAM** es la encimera: rápida, al alcance de la mano,
  pero se vacía al final del día;
- el **disco** es el armario: más lento, pero todo se queda guardado, incluso
  con la luz apagada.

Estas tres piezas trabajan juntas cada vez que ejecutas un
programa.

## El procesador

El [procesador](glossaire.md#procesador) (en inglés *CPU*) ejecuta
instrucciones muy simples: sumar dos números, comparar dos
números, copiar un número de una casilla a otra, saltar a otra
instrucción.

Solo hace cosas muy pequeñas, pero **miles de millones por segundo**. Un
programa que muestra una página entera son millones de esas pequeñas
instrucciones puestas una tras otra.

> **Recuerda:** el procesador no entiende ni Ruby, ni Rust, ni
> Rythme. Solo entiende su propio lenguaje, hecho de números: el
> **lenguaje máquina**.

## La memoria RAM y el disco

La [memoria RAM](glossaire.md#memoria-ram) (en inglés *RAM*, memoria de acceso aleatorio) guarda los
datos de los programas **mientras se ejecutan**: tus variables, tus
listas, el texto que escribes. Es muy rápida, pero se borra
al apagar el ordenador.

El **disco** (disco duro o SSD) guarda los datos **para siempre**:
tus fotos, tus documentos, tus programas. Es más lento, pero nada
desaparece al apagarlo.

| Pieza          | Velocidad     | ¿Guarda los datos al apagar?         | Tamaño habitual |
|----------------|---------------|--------------------------------------|-----------------|
| Memoria RAM    | muy rápida    | no                                   | 8 a 32 GB       |
| Disco          | más lento     | sí                                   | 256 GB a 2 TB   |

Por eso hay que **guardar** un archivo: mientras no lo hayas
hecho, tu trabajo solo está en la memoria RAM.

## Los archivos y las carpetas

En el disco, los datos se guardan en **archivos**: un archivo
es una secuencia de bytes con un nombre, como `hola.rb` o `foto.jpg`.

Los archivos se guardan en **carpetas**, que pueden contener
otras carpetas. Eso forma un árbol:

```text
/home/alice
├── Documentos
│   └── carta.txt
└── codigo
    ├── hola.rb
    └── calculo.ry
```

### Las rutas

Una [ruta](glossaire.md#ruta) es la dirección de un archivo: la lista de
carpetas que hay que atravesar, separadas por `/` (o `\` en Windows).

- Una ruta **absoluta** parte de la raíz: `/home/alice/codigo/hola.rb`,
  o `C:\Users\Alice\codigo\hola.rb` en Windows.
- Una ruta **relativa** parte de la carpeta donde estás: si estás en
  `/home/alice`, el mismo archivo se escribe `codigo/hola.rb`.
- `..` significa «la carpeta padre»: desde `codigo`, el archivo
  `carta.txt` se escribe `../Documentos/carta.txt`.

La [terminal](terminal.md) te hace usar estas rutas todos los días.

## Los bits y los bytes

En el fondo, el ordenador solo conoce dos estados: hay corriente o no hay
corriente. Se escriben `0` y `1`. Cada una de esas cifras se llama
[bit](glossaire.md#bit).

Un bit solo no dice gran cosa. Por eso se agrupan de ocho en ocho: ocho bits
forman un [byte](glossaire.md#byte) (u *octeto*). Un byte puede
tomar 2 × 2 × 2 × 2 × 2 × 2 × 2 × 2 = 256 valores, de 0 a 255.

| Unidad  | Tamaño                  | Para hacerse una idea     |
|---------|-------------------------|---------------------------|
| 1 byte  | 8 bits                  | una letra simple          |
| 1 KB    | unos 1000 bytes         | un archivo de texto pequeño |
| 1 MB    | alrededor de un millón  | una foto                  |
| 1 GB    | unos mil millones       | una película              |

## Contar en binario

Nosotros contamos con diez cifras (0 a 9): es la base 10. El ordenador
cuenta con dos cifras (0 y 1): es la base 2, el
[binario](glossaire.md#binario).

En base 10, cada posición vale diez veces más que la anterior: 1, 10,
100… En binario, cada posición vale **el doble**: 1, 2, 4, 8, 16,
32, 64, 128.

```text
  64  32  16   8   4   2   1
   1   0   0   0   0   0   1    →  64 + 1 = 65
```

Los tres lenguajes saben escribir un número en binario. En Ruby:

```ruby
puts 65.to_s(2)
puts 0b1000001
```

```sortie
1000001
65
```

En Rust, `{:b}` muestra en binario, y `{:08b}` rellena con ceros
hasta tener ocho cifras, es decir, un byte:

```rust
println!("{:b}", 65);
println!("{:08b}", 5);
println!("Un byte llega hasta {}", u8::MAX);
```

```sortie
1000001
00000101
Un byte llega hasta 255
```

Rythme no tiene una función ya hecha para eso, pero se puede escribir
con una [función recursiva](glossaire.md#recursion): se divide entre dos
y se guarda el resto.

```rythme
fn binario(0) = "0"
fn binario(1) = "1"
fn binario(n) = binario(n // 2) + text(n % 2)

print(binario(65))
print(binario(5))
```

```sortie
1000001
101
```

## El texto también son números

El ordenador solo guarda números. Para guardar texto, se le dio
un **número a cada carácter**. Es un código que comparte todo el
mundo.

### ASCII: los primeros 128 caracteres

El código [ASCII](glossaire.md#ascii), creado en los años 60, numera las
letras inglesas, las cifras y la puntuación. `A` vale 65, `B` vale
66, `a` vale 97, el espacio vale 32.

En Ruby, `ord` da el número de un carácter, y `chr` hace lo contrario:

```ruby
puts "A".ord
puts 66.chr
puts "Ada".bytes.inspect
```

```sortie
65
B
[65, 100, 97]
```

En Rust, un carácter (`char`) se convierte en número con `as u32`:

```rust
println!("{}", 'A' as u32);
println!("{}", char::from_u32(66).unwrap());
```

```sortie
65
B
```

En Rythme, `char_code` y `char` hacen el mismo trabajo:

```rythme
print(char_code("A"), char(66))
print(map(chars("Ada"), char_code))
```

```sortie
65 B
[65, 100, 97]
```

### Unicode: todos los caracteres del mundo

ASCII no tiene ni `é`, ni `ñ`, ni ideogramas, ni emojis. [Unicode](glossaire.md#unicode)
lo numeró todo: más de 150 000 caracteres, de todas las
escrituras. `é` vale 233, `€` vale 8364.

Para guardar esos números en bytes, casi siempre se usa
**UTF-8**: los caracteres ASCII ocupan 1 byte, `é` ocupa 2, `€`
ocupa 3.

```ruby
puts "é".ord
puts "é".bytesize
puts "Aé".bytes.inspect
```

```sortie
233
2
[65, 195, 169]
```

En Rust, `.len()` cuenta los **bytes**, no los caracteres:

```rust
println!("{}", "é".len());
println!("{}", "é".chars().count());
```

```sortie
2
1
```

> **Nota:** por eso hay que guardar tus archivos en UTF-8.
> Tu editor lo hace por defecto.

## El sistema operativo

El [sistema operativo](glossaire.md#sistema-operativo) (Windows,
macOS, Linux, Android…) es el programa que arranca primero. Es el
director de orquesta:

- reparte el procesador y la memoria entre los programas;
- guarda los archivos en el disco;
- gestiona la pantalla, el teclado, la red;
- inicia y detiene los programas.

Tus programas nunca hablan directamente con el disco: se lo piden al
sistema operativo, por ejemplo «abre el archivo `notas.txt`».

## Programas y procesos

Un **programa** es un archivo en el disco: una receta guardada en un
libro. Un [proceso](glossaire.md#proceso) es ese programa **mientras
se ejecuta**: la receta mientras se cocina.

Cuando escribes `ruby hola.rb`, el sistema operativo crea un
proceso: copia el programa en la memoria RAM, le da un
número, y el procesador empieza a ejecutar sus instrucciones. Cuando el
programa termina, el proceso desaparece y su memoria se libera.

Puedes ejecutar dos veces el mismo programa: eso da dos procesos.
El Administrador de tareas (Windows), el Monitor de Actividad (macOS) o el
comando `ps` (Linux y macOS) los muestran.

## Por qué traducir los programas

El procesador solo entiende el lenguaje máquina. Tu código, en cambio, es un
texto escrito para personas. Por eso hay que **traducirlo**. Hay dos
maneras de hacerlo, explicadas en
[Interpretado o compilado](programmer.md#interpretado-o-compilado):

- un [compilador](glossaire.md#compilador) traduce todo el programa una
  vez, por adelantado, a un archivo de lenguaje máquina. Es el caso de Rust;
- un [intérprete](glossaire.md#interprete) lee el código y lo ejecuta sobre
  la marcha. Es el caso de Ruby y de Rythme.

Es como un libro traducido de antemano (compilado), o un intérprete que
traduce mientras la persona habla (interpretado).

## Errores frecuentes

### Sumar un texto y un número

Para ti, `"A"` y `65` se parecen. Para el programa, son dos
[tipos](glossaire.md#tipo) distintos:

```ruby norun
letra = "A"
puts letra + 1
```

```text
main.rb:2:in `+': no implicit conversion of Integer into String (TypeError)

puts letra + 1
             ^
	from main.rb:2:in `<main>'
```

Convierte primero: `letra.ord + 1` da el número `66`, y
`(letra.ord + 1).chr` da la letra `"B"`.

### Poner un número en un char en Rust

```rust norun
fn main() {
    let letra: char = 65;
    println!("{letra}");
}
```

```text
error[E0308]: mismatched types
 --> main.rs:2:23
  |
2 |     let letra: char = 65;
  |                ----   ^^ expected `char`, found `u8`
  |                |
  |                expected due to this
```

Rust nunca convierte un número en carácter por su cuenta. Escribe la
letra `'A'`, o convierte: `let letra = 65u8 as char;`.

### Darle varios caracteres a char_code

```rythme norun
print(char_code("AB"))
```

```text
error[E023]: `char_code` necesita un solo carácter, pero "AB" tiene 2
 --> main.ry:1:7
  |
1 | print(char_code("AB"))
  |       ^^^^^^^^^^^^^^^
```

Un número corresponde a **un** carácter. Para una palabra entera, divídela
primero: `map(chars("AB"), char_code)` da `[65, 66]`.

### Equivocarse de carpeta

```ruby norun
texto = File.read("notas.txt")
```

```text
main.rb:1:in `read': No such file or directory @ rb_sysopen - notas.txt (Errno::ENOENT)
	from main.rb:1:in `<main>'
```

Una ruta relativa parte de la carpeta desde donde **ejecutas** el programa. Compruébalo
con `pwd` y `ls` en la terminal, o da la ruta correcta.

## Ponte a prueba

**1.** Escribes un texto largo sin guardarlo, y la batería se agota.
¿Por qué se pierde el texto?

> **Respuesta**
>
> Solo estaba en la memoria RAM, que se borra al apagarse. Solo
> se conserva lo que está guardado en el disco.

**2.** ¿Cuánto vale el número binario `101`?

> **Respuesta**
>
> 5: se leen las posiciones de derecha a izquierda, 1 + 0 × 2 + 1 × 4 = 5.

**3.** ¿Cuál es la diferencia entre un programa y un proceso?

> **Respuesta**
>
> El programa es un archivo en el disco. El proceso es ese programa
> mientras se ejecuta, en la memoria RAM, con su propio número.

## En resumen

- El procesador calcula, la memoria RAM guarda los datos durante el trabajo, el disco los guarda para siempre.
- Una ruta da la dirección de un archivo: absoluta desde la raíz, relativa desde la carpeta actual.
- Todo se guarda en bits (0 o 1), agrupados de ocho en ocho en bytes.
- Un texto es una secuencia de números: ASCII para el inglés, Unicode (en UTF-8) para todo lo demás.
- El sistema operativo reparte la máquina; un proceso es un programa mientras se ejecuta.
- El código debe traducirse a lenguaje máquina, con un compilador o un intérprete.

Siguiente: [pensar como un programador](algorithmes.md).
