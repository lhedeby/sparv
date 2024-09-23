# Syntax


## print
``ts
    print "hello";
``

## for-loop
``ts
    for i in 24 -> 13 {
        ...
    }
``

## while-loop
``ts
    while 5 > 2 {
        ...
    }
``

## variable declaration
``ts
    let foo = "bar";
``

## if statement
``ts
    if 25 == 23 {
        ...
    }
``

## conditials
``ts
    let true_var = 25 == 25 and 42 == 42;
    let false_var = 25 != 25 or 42 != 42;
``

## functions
``ts
    fun foo(p1, p2) {
        print (p1 + p2);
    }
``
### functions are first class
``ts
    fun exec(f) {
        f(2, 5);
    }
``
