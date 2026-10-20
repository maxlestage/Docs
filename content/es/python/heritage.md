# Herencia y polimorfismo

Esta página muestra cómo una clase puede heredar de otra, cómo objetos distintos pueden
responder a la misma llamada y cómo hacer que `==`, `<` o `len` funcionen con tus propias
clases.

> **Objetivos:**
>
> - sabrás escribir una subclase y llamar al padre con `super()`;
> - sabrás redefinir un método y comprobar un tipo con `isinstance`;
> - sabrás qué son el polimorfismo y el *duck typing*;
> - sabrás escribir una clase abstracta con `abc`;
> - sabrás elegir entre herencia y composición, y escribir `__eq__`, `__lt__` y
>   `__len__`.

## Una subclase

La [herencia](../debuter/glossaire.md#heritage) permite crear una clase a partir de
otra. La nueva clase, la **subclase** (o clase hija), recibe todo lo que tiene la
**clase padre**: sus atributos y sus métodos. Se escribe poniendo el padre entre
paréntesis.

```python
class Animal:
    def __init__(self, nombre):
        self.nombre = nombre

    def comer(self):
        print(f"{self.nombre} come.")

class Perro(Animal):
    def ladrar(self):
        print(f"{self.nombre} ladra: ¡Guau!")

rex = Perro("Rex")
rex.comer()
rex.ladrar()
```

```sortie
Rex come.
Rex ladra: ¡Guau!
```

`Perro` no ha escrito `__init__` ni `comer`: los ha heredado de `Animal`. Solo añade lo
que le es propio.

> **Recuerda:** la herencia expresa una relación «**es un**». Un perro **es un**
> animal. Si la frase suena mal, la herencia probablemente no es la herramienta adecuada.

## super(): llamar al padre

Si la subclase necesita atributos adicionales, escribe su propio `__init__`. Para no
copiar el trabajo del padre, lo llama con `super()`.

```python
class Animal:
    def __init__(self, nombre):
        self.nombre = nombre

class Gato(Animal):
    def __init__(self, nombre, color):
        super().__init__(nombre)
        self.color = color

michi = Gato("Michi", "gris")
print(michi.nombre, michi.color)
```

```sortie
Michi gris
```

`super().__init__(nombre)` quiere decir: «haz primero lo que haría el padre con
`nombre`». Después, `Gato` añade `color`.

## Redefinir un método

Una subclase puede **redefinir** un método del padre: escribe un método con el mismo
nombre, que sustituye al del padre para sus objetos.

```python
class Animal:
    def __init__(self, nombre):
        self.nombre = nombre

    def hablar(self):
        return "..."

class Perro(Animal):
    def hablar(self):
        return "Guau"

class Gato(Animal):
    def hablar(self):
        return "Miau"

print(Animal("Bicho").hablar())
print(Perro("Rex").hablar())
print(Gato("Michi").hablar())
```

```sortie
...
Guau
Miau
```

### Completar en lugar de sustituir

Con `super()`, un método redefinido también puede **reutilizar** el del padre y
ampliarlo.

```python
class Empleado:
    def __init__(self, nombre, salario):
        self.nombre = nombre
        self.salario = salario

    def descripcion(self):
        return f"{self.nombre}, {self.salario} €"

class Jefe(Empleado):
    def __init__(self, nombre, salario, equipo):
        super().__init__(nombre, salario)
        self.equipo = equipo

    def descripcion(self):
        return super().descripcion() + f", equipo de {len(self.equipo)}"

print(Jefe("Alice", 4000, ["Bob", "Chloé"]).descripcion())
```

```sortie
Alice, 4000 €, equipo de 2
```

## isinstance e issubclass

`isinstance(objeto, Clase)` responde a «¿este objeto es de esta clase, **o de una de sus
subclases**?». `issubclass` hace la misma pregunta para dos clases.

```python
class Animal:
    pass

class Perro(Animal):
    pass

rex = Perro()
print(isinstance(rex, Perro))
print(isinstance(rex, Animal))
print(isinstance(rex, str))
print(issubclass(Perro, Animal))
print(type(rex) == Animal)
```

```sortie
True
True
False
True
False
```

`type(rex) == Animal` es falso: `type` da la clase exacta, sin mirar a los padres.
Es mejor usar `isinstance`.

> **Nota:** todas las clases heredan de `object`, la clase base de Python. Es ella la que
> proporciona el `__repr__` y el `==` por defecto.

## El polimorfismo

«Polimorfismo» quiere decir «muchas formas». La misma llamada, `forma.area()`, da un
resultado distinto según el objeto. El código que llama no necesita saber de qué clase
se trata.

```python
import math

class Circulo:
    def __init__(self, radio):
        self.radio = radio

    def area(self):
        return math.pi * self.radio ** 2

class Rectangulo:
    def __init__(self, ancho, alto):
        self.ancho = ancho
        self.alto = alto

    def area(self):
        return self.ancho * self.alto

formas = [Circulo(1), Rectangulo(2, 3), Circulo(2)]
for forma in formas:
    print(type(forma).__name__, round(forma.area(), 2))
```

```sortie
Circulo 3.14
Rectangulo 6
Circulo 12.57
```

Es como un mando a distancia: el botón «encender» enciende la televisión, la radio o el
ventilador. Cada aparato sabe lo que «encender» significa para él.

### El duck typing

En el ejemplo anterior, `Circulo` y `Rectangulo` no tienen **ningún padre común**: solo
tienen los dos un método `area`. Para Python, eso basta.

Es el *duck typing* («tipado de pato»): «si camina como un pato y grazna como un pato,
entonces es un pato». Python no comprueba el tipo de un objeto, solo que sabe hacer lo
que se le pide.

```python
class Pato:
    def graznar(self):
        return "Cuac cuac"

class Robot:
    def graznar(self):
        return "BIP cuac BIP"

def hacer_graznar(cosa):
    print(cosa.graznar())

hacer_graznar(Pato())
hacer_graznar(Robot())
```

```sortie
Cuac cuac
BIP cuac BIP
```

## Las clases abstractas

Una **clase abstracta** es un modelo incompleto: anuncia métodos que cada subclase
**debe** escribir, pero no se puede crear un objeto directamente a partir de ella. Se
escribe con el módulo `abc` (*abstract base classes*, «clases base abstractas»).

```python
from abc import ABC, abstractmethod

class Forma(ABC):
    @abstractmethod
    def area(self):
        pass

    def describir(self):
        return f"{type(self).__name__} de área {self.area()}"

class Cuadrado(Forma):
    def __init__(self, lado):
        self.lado = lado

    def area(self):
        return self.lado ** 2

print(Cuadrado(3).describir())
try:
    Forma()
except TypeError as e:
    print(e)
```

```sortie
Cuadrado de área 9
Can't instantiate abstract class Forma without an implementation for abstract method 'area'
```

`describir` se escribe una sola vez, en `Forma`, y llama a `area`, que cada subclase
proporciona. La clase abstracta garantiza que ninguna subclase olvide `area`.

## ¿Composición o herencia?

La herencia no es la única forma de reutilizar código. La **composición** consiste en
**guardar un objeto dentro de otro**. Expresa una relación «**tiene un**»: un coche
**tiene un** motor; **no es** un motor.

```python
class Motor:
    def __init__(self, potencia):
        self.potencia = potencia

    def arrancar(self):
        return f"Brum ({self.potencia} CV)"

class Coche:
    def __init__(self, marca, potencia):
        self.marca = marca
        self.motor = Motor(potencia)

    def arrancar(self):
        return f"{self.marca}: {self.motor.arrancar()}"

print(Coche("Zoé", 90).arrancar())
```

```sortie
Zoé: Brum (90 CV)
```

| Pregunta | Elección |
|----------|----------|
| ¿«X es un Y» es verdad? | herencia |
| ¿«X tiene un Y» es verdad? | composición |
| ¿dudas? | composición, más flexible |

La composición permite cambiar una pieza sin tocar el resto: se le puede poner al coche
un motor eléctrico sin crear una nueva clase de coche.

## Los métodos especiales para tus clases

Los métodos rodeados de dobles guiones bajos, como `__init__`, se llaman
**métodos especiales** (o *dunder*, por *double underscore*). Python los llama solo
cuando usas un operador o una función integrada.

| Escribes | Python llama a |
|----------|----------------|
| `a == b` | `a.__eq__(b)` |
| `a < b` | `a.__lt__(b)` |
| `len(a)` | `a.__len__()` |
| `str(a)`, `print(a)` | `a.__str__()` |
| `x in a` | `a.__contains__(x)` |

### Comparar con __eq__

Por defecto, `==` entre dos objetos mira si se trata del **mismo** objeto. Con `__eq__`,
tú decides lo que significa «igual».

```python
class Punto:
    def __init__(self, x, y):
        self.x = x
        self.y = y

    def __eq__(self, otro):
        if not isinstance(otro, Punto):
            return NotImplemented
        return self.x == otro.x and self.y == otro.y

print(Punto(1, 2) == Punto(1, 2))
print(Punto(1, 2) == Punto(3, 4))
print(Punto(1, 2) == "texto")
```

```sortie
True
False
False
```

`NotImplemented` le dice a Python: «no sé comparar con eso». Python prueba entonces otra
cosa, y acaba respondiendo `False`.

### Ordenar con __lt__

`sorted`, `min` y `max` usan `<`. Basta con escribir `__lt__` (*less than*,
«menor que»).

```python
class Carta:
    def __init__(self, valor, palo):
        self.valor = valor
        self.palo = palo

    def __lt__(self, otra):
        return self.valor < otra.valor

    def __repr__(self):
        return f"{self.valor}{self.palo}"

mano = [Carta(10, "♥"), Carta(3, "♠"), Carta(7, "♦")]
print(sorted(mano))
print(max(mano))
```

```sortie
[3♠, 7♦, 10♥]
10♥
```

### Medir con __len__

`__len__` permite usar `len()`. Además, un objeto de longitud 0 se considera falso en un
`if`.

```python
class Cesta:
    def __init__(self):
        self.articulos = []

    def agregar(self, articulo):
        self.articulos.append(articulo)

    def __len__(self):
        return len(self.articulos)

    def __contains__(self, articulo):
        return articulo in self.articulos

cesta = Cesta()
if not cesta:
    print("Cesta vacía")
cesta.agregar("pan")
cesta.agregar("leche")
print(len(cesta))
print("pan" in cesta)
```

```sortie
Cesta vacía
2
True
```

> **Consejo:** escribir todos estos métodos a mano se vuelve repetitivo enseguida. Las
> *dataclasses* los generan por ti: mira [las anotaciones de tipo](typage.md).

## Errores frecuentes

### Olvidar super().__init__

```python norun
class Animal:
    def __init__(self, nombre):
        self.nombre = nombre

class Gato(Animal):
    def __init__(self, nombre, color):
        self.color = color

michi = Gato("Michi", "gris")
print(michi.nombre)
```

```text
Traceback (most recent call last):
  File "main.py", line 10, in <module>
    print(michi.nombre)
          ^^^^^^^^^^^^
AttributeError: 'Gato' object has no attribute 'nombre'
```

El `__init__` de `Gato` sustituye al de `Animal`, que por tanto nunca se llama.
Añade `super().__init__(nombre)` al principio del `__init__` de `Gato`.

### Olvidar un método abstracto

```python norun
from abc import ABC, abstractmethod

class Forma(ABC):
    @abstractmethod
    def area(self):
        pass

class Cuadrado(Forma):
    def __init__(self, lado):
        self.lado = lado

c = Cuadrado(3)
```

```text
Traceback (most recent call last):
  File "main.py", line 12, in <module>
    c = Cuadrado(3)
TypeError: Can't instantiate abstract class Cuadrado without an implementation for abstract method 'area'
```

`Cuadrado` no ha escrito `area`: sigue siendo abstracta. Añade el método en `Cuadrado`.

### Ordenar objetos sin __lt__

```python norun
class Carta:
    def __init__(self, valor):
        self.valor = valor

cartas = [Carta(7), Carta(2)]
print(sorted(cartas))
```

```text
Traceback (most recent call last):
  File "main.py", line 6, in <module>
    print(sorted(cartas))
          ~~~~~~^^^^^^^^
TypeError: '<' not supported between instances of 'Carta' and 'Carta'
```

Python no sabe qué carta es la más pequeña. Escribe `__lt__`, o indica una clave de
ordenación: `sorted(cartas, key=lambda c: c.valor)`.

### Llamar a len sin __len__

```python norun
class Cesta:
    def __init__(self):
        self.articulos = ["pan", "leche"]

print(len(Cesta()))
```

```text
Traceback (most recent call last):
  File "main.py", line 5, in <module>
    print(len(Cesta()))
          ~~~^^^^^^^^^
TypeError: object of type 'Cesta' has no len()
```

Añade un método `__len__` que devuelva `len(self.articulos)`.

## Ponte a prueba

**Pregunta 1:** una `Bici` tiene dos `Rueda`. ¿Herencia o composición?

> **Respuesta**
>
> Composición: una bici **tiene** ruedas, no **es** una rueda. Escribiremos
> `self.ruedas = [Rueda(), Rueda()]` en `Bici`.

**Pregunta 2:** ¿qué devuelve `isinstance(True, int)`? (Pista: `bool` es una subclase
de `int`.)

> **Respuesta**
>
> `True`. `isinstance` también acepta las subclases, y `bool` hereda de `int`.

**Pregunta 3:** escribe una clase `Nota` con un atributo `valor`, que se pueda ordenar
con `sorted`.

> **Respuesta**
>
> ```python
> class Nota:
>     def __init__(self, valor):
>         self.valor = valor
>
>     def __lt__(self, otra):
>         return self.valor < otra.valor
>
>     def __repr__(self):
>         return f"Nota({self.valor})"
>
> print(sorted([Nota(15), Nota(8), Nota(12)]))
> ```
>
> ```sortie
> [Nota(8), Nota(12), Nota(15)]
> ```

## En resumen

- `class Hija(Padre):` hereda los atributos y los métodos del padre.
- `super()` llama a la versión del padre, sobre todo en `__init__`.
- El polimorfismo: la misma llamada da un comportamiento adaptado a cada objeto; con el
  *duck typing*, solo cuenta que el método exista.
- `ABC` y `@abstractmethod` obligan a las subclases a escribir ciertos métodos.
- Prefiere la composición («tiene un») cuando «es un» no suena bien.
- `__eq__`, `__lt__` y `__len__` hacen que `==`, `sorted` y `len` funcionen con tus
  objetos.

Siguiente: [las anotaciones de tipo](typage.md). Compara con
[la herencia en Ruby](../ruby/heritage.md).
