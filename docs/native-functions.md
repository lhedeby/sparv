# Native functions

Native functions are functions built into sparv and are always there for you to use.
If you followed the [getting started](https://github.com/lhedeby/sparv/blob/main/docs/getting-started.md)
you already used one of them:
```ts
    print("Print is a native function");
```
## read_file

Reads a file and returns the content of the file as a string:
```ts
let file = read_file("some_file");
```

## read_input

Waits for user input. Takes a string as prompt:
```ts
    let input = read_input("your name: ");
    print("Hello " + input + "!");
```

## len

Returns the length of a list or a string. Errors on all other types:
```ts
    let s_len = len("text");
    print(s_len); // prints: 4

    [1, 2, 3, 4]->len->print; // also prints: 4
```
## split

Splits a string and with a specified delimiter and returns a list with the substrings:
```ts
    split("Badger Mushroom Snake", " ")->print; // prints: [Badger, Mushroom, Snake]
```

## split_lines

Splits a text at the line breaks. Sparv supports multiline strings so its easy to test:
```ts
    "first line
    second line
    third line"-> split_lines->print;
```
## parse

Tries to parse a string to a number. If the string cannot be converted to a number it 
returns null. Runtime expection if anything other than a string is passed as argument:
```ts
    "1"->parse->fun(x) x == 1->print; // prints: true
    "hello"->parse->print; // prints: null
```
## typeof

Returns the type of the variable:
```ts
   typeof([]) // = <list>
   typeof({}) // = <object>
   typeof("hello") // = <string>
   typeof(42) // = <number>
```
The return type is a string that you can compare if you want to check the type:
```ts
    if typeof([]) == "<list>" {
        print("Do something...");
    }
```
## random

Random takes a list as argument and returns a random value from the list:
```ts
    let foo_or_bar = random(["foo", "bar"]); // evaluates to either "foo" or "bar"
    let number = random(0:100); // evaluates to a number from 0 to 99 (':' is a non-inclusive range)
```
