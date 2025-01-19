# Getting started

Welcome to Sparv, a lightweight scripting language.
This guide will introduce you to the basics and help you start coding right away.

## Hello world

Sparv is a scripting language designed to minimize boilerplate code, making it quick and easy to get started.
Print is a built-in function and can be called in a couple different ways:

```ts
    print("Hello, World");
    "Hello, World"->print;
```

## Variables

Variables are declared using the ```var``` keyword:
```ts
    var foo = "bar";
    var i = 42;
    var b = true;
    var list = [1, 2, 3];
```
To reassign a variable you just skip the ```var``` keyword. Sparv is dynamically typed so you are allowed to change the type
of a variable during reassignment:
```ts
    var number = 81;
    number = "not a number anymore";
    number = false;
```

## Control flow

The normal control flow statements should look familiar to you if you got previous experience with any c-style language:
```ts
    // if-statement
    var lemons = true;
    if lemons {
        print("Let's make lemonade!");
    } else {
        print("I'd rather just drink water anyway");
    }

    // loop
    loop 5 {
        print("Printing this 5 times");
    }

    // for-loop
    for i in 0:10 {
        print(i + ". They're taking the hobbits to Isengard!");
    }

    // while-loop
    var i = 0;
    while i < 10 {
        print("Badger");
        i = i + 1;
    }
```

The ``0:10`` syntax you saw earlier is actually just a list. All for loop are just for-each loops:
```ts
    var list = 0:4; // [0, 1, 2, 3]
    var same = [0, 1, 2, 3];
```

You can use ``+`` to concatenate lists:
```ts
    var list1 = [1, 2, 3];
    var list2 = [4, 5, 6];
    var list3 = list1 + list2; // [1,2,3,4,5,6]
```
## Objects

Objects can be declared with any number of comma-separated key/value pairs. And can later be accessed with '.' operator.
They look just like json objects:
```ts
    var obj = {
        key: "value",
        number: 123,
        list: [1,2,3],
    };

    print(obj.key); // prints 'value'
```

Functions can be declared using the ``fun`` keyword:
```ts
    fun square(number) {
        return number * number;
    }

    var result = square(2); // = 4
```
If the function only has 1 statement the braces around the body can be omitted.
Just 1 line with an expression, no need for the ```return``` keyword either. Also 
they're first class, meaning they are treated like any other variable and can be 
assigned to variables and passed around as arguments:
```ts
    fun square(number) number * number;

    // they can also be assigned to a variable directly
    var map = fun(list, fn) {
        for i in i:len(list) {
            // append to the list
            list[i] = fn(list[i]);
        }
        return res;
    };

    var res = map([1,2,3], square); // = [1, 4, 9]
```

You have seen the 2 ways of declaring functions and there are some slight differences.
The biggest difference is that the function declaration hoists the function. Allowing you to use the function before its been
declared:

```ts
    f1();
    fun f1() { }
```

Functions can also be chained with the ``->`` operator. It passes the value of the evaluated expression to the next function:
```ts
    // these 2 function will always be defined in the following 'function' examples
    fun square(number) number * number;
    fun double(number) number + number;

    1->double; // == double(1);
    1->double->square; // == square(double(1));
```
There's no need to to invoke the function with ``()``. It's called with a single parameter.  


You can also assign the result of an arrow expression
```ts
    ...

    var i = 1->double; // = 2
    print(i); // prints 2
```
But the easier way would be to just chain the print expression:
```ts
    ...

    1->double->print;
```
The native function print conveniently returns the printed value. So you can keep on chaining:
```ts
    ...

    // Let's add some line breaks for readability!
    1->
        double-> // 2
        print-> // prints '2'
        square-> // 4
        square-> // 16
        print; // prints '16'

```
The arrow operator can only pass 1 parameter to the next function. But sometimes you want
a function that takes more arguments. [Currying](https://en.wikipedia.org/wiki/Currying#) to the rescue!
(Sadly not called [Schönfinkelisation](https://en.wikipedia.org/wiki/Moses_Sch%C3%B6nfinkel))
```ts
    fun add(x) { // add returns a new function that uses 'x' in its evaluation
        return fun(list) {
            for i in 0:len(list) {
                list[i] = list[i] + x; // add 'x' to all elements
            }
            return list; // returns the list with all the elements incremented with 'x'
        };
    }

    [1,2,3]->
        add(3)-> // call to 'add' and passes 3 as the 'x' param. This then returns the inner function of add.
                // Which is the function that gets called with the list as the parameter.
        print; // prints '[4, 5, 6]'
```
Inline functions can be defined directly within arrow expressions:
```ts
    2->fun(x) x * 5->print; // prints: 10
```

These are the basics for sparv. Next step is to get familiar with the [native functions]().
If you want more examples you can also checkout my solutions for [Advent of code 2024](https://github.com/lhedeby/aoc-2024-sparv)

