# Getting started

This is an introduction to the programming language sparv. This will help you get started and explain the basic concepts.

## Hello world

There's no boiler plate to get started. So to get started you just need to use the native function ``print``:

```ts
    print("Hello, World!");
```

## Variables

Variables are declared using the ``let`` keyword:
```ts
    let foo = "bar";
    let i = 42;
    let b = true;
    let list = [1,2,3];
```
Sparv is dynamically typed so you are allowed to reassign a variable to another type:
```ts
    let number = 81;
    number = "not a number anymore";
    number = false;
```
## Comments

The only supported comments are ``//`` and the last untill the end of the line:
```ts
    // This is a comment on the whole line
    print("This is not a comment"); // you can also end lines with comments
    // They will be ignored by the interpreter
```

## Control flow

The normal control flow statement should look familiar to you if you got previous experience with any c-style language:
```ts
    // if-statement
    let lemons = true;
    if lemons {
        print("Let's make lemonade!");
    } else {
        print("I'd rather just drink water anyway");
    }

    // for-loop
    for i in 0:10 {
        print(i + ". They're taking the hobbits to Isengard!");
    }

    // while-loop
    let i = 0;
    while i < 10 {
        print("Badger");
        i = i + 1;
    }
```

## Lists

The ``0:10`` syntax you saw earlier is actually just a list. All for loop are actually for-each loops:
```ts
    let list = 0:4; // [0, 1, 2, 3]
    let same = [0, 1, 2, 3];
```

You can use ``+`` to concatenate lists:
```ts
    let list1 = [1, 2, 3];
    let list2 = [4, 5, 6];
    let list3 = list1 + list2; // [1,2,3,4,5,6]
```

## Functions

Functions can be declared using the ``fun`` keyword:
```ts
    fun square(number) {
        return number * number;
    }

    let result = square(10); // == 100
```
