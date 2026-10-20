# Inheritance and prototypes

This page shows how a class can reuse another class with `extends` and `super`, what a prototype is, and when to prefer composition.

> **Goals:**
>
> - you will be able to create a subclass with `extends`;
> - you will be able to call the parent with `super`;
> - you will be able to override a method and take advantage of polymorphism;
> - you will understand, in simple terms, what a prototype is;
> - you will be able to choose between inheritance, composition and mixins.

## Creating a subclass with extends

[Inheritance](../debuter/glossaire.md#heritage) lets you create a class from another
one. The new class, the **subclass** (or child class), receives everything the
**parent** class has: its properties and its methods.

Think of a family of vehicles: a car *is a* vehicle, a bike *is a* vehicle. They all
move; each one adds its own features.

```javascript
class Animal {
  constructor(name) {
    this.name = name;
  }

  eat() {
    console.log(`${this.name} is eating.`);
  }
}

class Dog extends Animal {
  bark() {
    console.log(`${this.name} barks: Woof!`);
  }
}

const rex = new Dog("Rex");
rex.eat();
rex.bark();
console.log(rex instanceof Dog, rex instanceof Animal);
```

```sortie
Rex is eating.
Rex barks: Woof!
true true
```

`Dog` did not write a `constructor` or an `eat` method: it inherits them from `Animal`.
A dog is also an animal, so `instanceof Animal` answers `true`.

## The constructor and super

If the subclass needs its own constructor, it must first call the parent's constructor
with `super(...)`. The parent prepares the base of the object; the child then completes
it.

```javascript
class Animal {
  constructor(name) {
    this.name = name;
  }
}

class Dog extends Animal {
  constructor(name, breed) {
    super(name);
    this.breed = breed;
  }
}

const rex = new Dog("Rex", "labrador");
console.log(rex);
```

```sortie
Dog { name: 'Rex', breed: 'labrador' }
```

> **Remember:** in a subclass constructor, call `super(...)` **before** you use
> `this`.

## Overriding a method

A subclass can **override** a method of the parent: it writes a method with the same
name, which replaces the parent's one for its objects.

```javascript
class Animal {
  constructor(name) {
    this.name = name;
  }

  speak() {
    return "...";
  }
}

class Cat extends Animal {
  speak() {
    return "Meow";
  }
}

console.log(new Animal("Creature").speak());
console.log(new Cat("Kitty").speak());
```

```sortie
...
Meow
```

### Extending the parent's method with super

Sometimes you do not want to replace the parent's method, but to **enrich** it.
`super.method()` calls the parent's version.

```javascript
class Employee {
  constructor(name, salary) {
    this.name = name;
    this.salary = salary;
  }

  describe() {
    return `${this.name}, salary ${this.salary} €`;
  }
}

class Manager extends Employee {
  constructor(name, salary, team) {
    super(name, salary);
    this.team = team;
  }

  describe() {
    return `${super.describe()}, manages ${this.team.length} people`;
  }
}

console.log(new Employee("Léa", 2000).describe());
console.log(new Manager("Sam", 3000, ["Léa", "Tom"]).describe());
```

```sortie
Léa, salary 2000 €
Sam, salary 3000 €, manages 2 people
```

## Polymorphism

"Polymorphism" means "many forms". Objects of different classes answer the **same**
method **call**, each in its own way. The code that uses them does not need to know which
is which.

Here is a list of shapes. Each one knows how to compute its area; the loop simply calls
`area()`.

```javascript
class Shape {
  constructor(name) {
    this.name = name;
  }

  area() {
    throw new Error("area() must be overridden");
  }

  toString() {
    return `${this.name}: ${this.area().toFixed(2)}`;
  }
}

class Rectangle extends Shape {
  constructor(width, height) {
    super("Rectangle");
    this.width = width;
    this.height = height;
  }

  area() {
    return this.width * this.height;
  }
}

class Square extends Rectangle {
  constructor(side) {
    super(side, side);
    this.name = "Square";
  }
}

class Circle extends Shape {
  constructor(radius) {
    super("Circle");
    this.radius = radius;
  }

  area() {
    return Math.PI * this.radius ** 2;
  }
}

const shapes = [new Rectangle(2, 3), new Square(4), new Circle(1)];
for (const shape of shapes) {
  console.log(String(shape));
}

const total = shapes.reduce((sum, s) => sum + s.area(), 0);
console.log(`Total area: ${total.toFixed(2)}`);
```

```sortie
Rectangle: 6.00
Square: 16.00
Circle: 3.14
Total area: 25.14
```

Three ideas in this example:

- `Shape.area` throws an error: it is a way of saying "each subclass **must** write its
  own version";
- `toString` is written only once, in `Shape`, and calls `this.area()`: the right version
  is chosen based on the object;
- `Square` inherits from `Rectangle`, which inherits from `Shape`: inheritance can have
  several levels.

To add a triangle, you only need to write a `Triangle` class: the loop does not change.

## Prototypes, simply

Under the hood, JavaScript does not really have "classes" like other languages. It has
**prototypes**. The `class` keyword is a nicer way to write this mechanism.

Every object has a hidden link to another object: its **prototype**. When you ask for a
property, JavaScript first looks for it in the object. If it does not find it, it looks
in the prototype, then in the prototype's prototype, and so on. This is the **prototype
chain**.

Think of a child who does not know the answer: they ask their parent, who asks the
grandparent, and so on.

`Object.getPrototypeOf(object)` shows this link.

```javascript
class Animal {
  constructor(name) {
    this.name = name;
  }

  speak() {
    return "...";
  }
}

class Dog extends Animal {}

const rex = new Dog("Rex");

console.log(Object.getPrototypeOf(rex) === Dog.prototype);
console.log(Object.getPrototypeOf(Dog.prototype) === Animal.prototype);
console.log(Object.hasOwn(rex, "name"));
console.log(Object.hasOwn(rex, "speak"));
console.log(Object.getOwnPropertyNames(Animal.prototype));
```

```sortie
true
true
true
false
[ 'constructor', 'speak' ]
```

What happens when you write `rex.speak()`?

```text
rex                 → name: "Rex"            no speak
  ↓ prototype
Dog.prototype       → (nothing more)         no speak
  ↓ prototype
Animal.prototype    → speak()                found!
  ↓ prototype
Object.prototype    → toString(), hasOwnProperty()…
  ↓ prototype
null                → end of the chain
```

- The **data** (`name`) is stored in each object: `Object.hasOwn(rex, "name")` answers
  `true`.
- The **methods** are stored only once, in the class's prototype: all objects share
  them.

This is also why every object has `toString`: the chain always ends with
`Object.prototype`.

> **Note:** you almost never need to handle prototypes by hand. Write classes; just
> remember that methods are shared through the prototype chain.

## Prefer composition

Inheritance expresses an "**is a**" relationship: a dog *is an* animal. But often, the
relationship is "**has a**": a car *has an* engine. A car is not an engine! In that
case, you store the object in a property: this is **composition**.

```javascript
class Engine {
  constructor(horsepower) {
    this.horsepower = horsepower;
  }

  start() {
    return `Vroom (${this.horsepower} hp)`;
  }
}

class Car {
  constructor(brand, engine) {
    this.brand = brand;
    this.engine = engine;
  }

  start() {
    console.log(`${this.brand}: ${this.engine.start()}`);
  }
}

const small = new Car("City car", new Engine(70));
const big = new Car("Truck", new Engine(400));
small.start();
big.start();
```

```sortie
City car: Vroom (70 hp)
Truck: Vroom (400 hp)
```

Composition is more flexible: you can change the engine without touching `Car`. Long
inheritance chains (`A` → `B` → `C` → `D`) quickly become hard to understand.

> **Tip:** when in doubt, ask yourself "is X **a** Y?". If the sentence sounds wrong,
> choose composition.

## Mixins with Object.assign

A class has only **one** parent. How can you give the same ability (for example "knowing
how to swim") to classes that are not related? With a
[mixin](../debuter/glossaire.md#mixin): a simple object that contains methods, which you
copy into a class's prototype.

`Object.assign(target, source)` copies the properties of `source` into `target`.

```javascript
const Swimmer = {
  swim() {
    return `${this.name} swims.`;
  },
};

const Flyer = {
  fly() {
    return `${this.name} flies.`;
  },
};

class Duck {
  constructor(name) {
    this.name = name;
  }
}

class Fish {
  constructor(name) {
    this.name = name;
  }
}

Object.assign(Duck.prototype, Swimmer, Flyer);
Object.assign(Fish.prototype, Swimmer);

const donald = new Duck("Donald");
const nemo = new Fish("Nemo");
console.log(donald.swim());
console.log(donald.fly());
console.log(nemo.swim());
console.log(typeof nemo.fly);
```

```sortie
Donald swims.
Donald flies.
Nemo swims.
undefined
```

The duck can swim and fly; the fish can only swim. The mixin's methods use `this.name`:
they work with any object that has a `name`.

## Common mistakes

### Using this before super

```javascript norun
class Animal {
  constructor(name) { this.name = name; }
}
class Dog extends Animal {
  constructor(name, breed) {
    this.breed = breed;
    super(name);
  }
}
new Dog("Rex", "labrador");
```

```text
file:///home/vous/projet/main.mjs:6
    this.breed = breed;
    ^

ReferenceError: Must call super constructor in derived class before accessing 'this' or returning from derived constructor
    at new Dog (file:///home/vous/projet/main.mjs:6:5)
    at file:///home/vous/projet/main.mjs:10:1
```

The message says: "call the parent constructor before using `this`". Put `super(name);`
on the **first** line of the constructor. The same message appears if you forget
`super` completely.

### Forgetting super. when extending a method

```javascript norun
class Animal {
  describe() { return "an animal"; }
}
class Dog extends Animal {
  describe() { return this.describe() + " that barks"; }
}
console.log(new Dog().describe());
```

```text
file:///home/vous/projet/main.mjs:5
  describe() { return this.describe() + " that barks"; }
                           ^

RangeError: Maximum call stack size exceeded
    at Dog.describe (file:///home/vous/projet/main.mjs:5:28)
    at Dog.describe (file:///home/vous/projet/main.mjs:5:28)
    at Dog.describe (file:///home/vous/projet/main.mjs:5:28)
```

`this.describe()` calls the **dog**'s method, that is, itself, forever. The call stack
overflows. To call the parent's version, write `super.describe()`.

### Misspelling an overridden method

```javascript norun
class Shape { area() { throw new Error("area() must be overridden"); } }
class Square extends Shape { constructor(s){super(); this.s=s;} aera(){ return this.s**2; } }
console.log(new Square(2).area());
```

```text
file:///home/vous/projet/main.mjs:1
class Shape { area() { throw new Error("area() must be overridden"); } }
                             ^

Error: area() must be overridden
    at Square.area (file:///home/vous/projet/main.mjs:1:30)
    at file:///home/vous/projet/main.mjs:3:27
```

`aera` is not `area`: the method is not overridden, so the parent's one runs. Check that
the name is **exactly** the same as in the parent class.

## Test yourself

**Question 1:** what does this code print?

```javascript norun
class A { name() { return "A"; } }
class B extends A { name() { return "B" + super.name(); } }
class C extends B {}
console.log(new C().name());
```

> **Answer**
>
> `BA`. `C` has no `name` method: JavaScript finds it in `B`, which adds `"B"` in front
> of the result of `A`'s version.

**Question 2:** a `House` has `Room`s. Should you write `class House extends Room`?

> **Answer**
>
> No. A house **has** rooms, it **is not** a room. Use composition:
> `this.rooms = [new Room(...), ...]`.

**Question 3:** where is the `speak` method of a `new Cat()` object stored: in the object
itself, or somewhere else?

> **Answer**
>
> In `Cat.prototype` (or higher up in the chain). The object only contains its data;
> `Object.hasOwn(object, "speak")` answers `false`. All instances share the same method.

## Summary

- `class B extends A`: `B` inherits the properties and methods of `A`.
- In the child constructor, `super(...)` comes before any `this`.
- A method with the same name overrides the parent's; `super.method()` calls the parent.
- Polymorphism: different objects answer the same call, each in its own way.
- Methods live in prototypes, linked in a chain up to `Object.prototype`.
- "Is a" → inheritance; "has a" → composition; shared ability → mixin.

Next: [scope and closures](closures.md). Compare with
[inheritance in Ruby](../ruby/heritage.md).
