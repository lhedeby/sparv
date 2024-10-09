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

There are 2 different kind of comments. Single line comments starts with ```//``` and last until the end of the line.
Block comments starts and end with ```#``` and everything inside the block is a comment that will be ignore by the
parser:
```ts
    // This is a comment on the whole line
    print("This is not a comment"); // you can also end lines with comments
    // They will be ignored by the interpreter

    #
        This is a block comment.
        It works across multiple
        lines!
    #
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

The ``0:10`` syntax you saw earlier is actually just a list. All for loop are just for-each loops:
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

## Objects

Objects can be declared with any number of comma-separated key/value pairs. And can later be accessed with '.' operator:
```ts
    let obj = {
        key = "value",
        number = 123,
        list = [1,2,3],
    };

    print(obj.key); // prints 'value'
```

## Functions

Functions can be declared using the ``fun`` keyword:
```ts
    fun square(number) {
        return number * number;
    }

    let result = square(2); // = 4
```
If they only have 1 line of code no function body is needed. Also they're first class, meaning they are treated like any other variable:
```ts
    fun square(number) return number * number;

    // they can also be assigned to a variable directly
    let map = fun(list, fn) {
        let res = [];
        for item in list {
            // append to the list
            res = res + [fn(item)];
        }
        return res;
    };

    let res = map([1,2,3], square); // = [1, 4, 9]
```

Fucntions can also be chained with the ``->`` operator. It passes the value of the evaluated expression to the next function:
```ts
    // these 2 function will always be defined in the following 'function' examples
    fun square(number) return number * number;
    fun double(number) return number + number;

    1->double; // == double(1);
    1->double->square; // == square(double(1));
```
There's no need to to invoke the function with ``()``. It's called with a single parameter.  


You can also assign the result of an arrow expression
```ts
    ...

    let i = 1->double; // = 2
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
You can also inline functions with arrows:
```ts
    2->fun(x) x * 5->print; // prints: 10
```

These are the basics for sparv. Next step is to get familiar with the [native functions](https://github.com/lhedeby/sparv/blob/main/docs/native-functions.md).
