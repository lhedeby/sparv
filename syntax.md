# Syntax

## print
``
    print "hello";
``

## for-loop
``
    for i in 24 -> 13 {
        ...
    }
``

## while-loop
``
    while 5 > 2 {
        ...
    }
``

## variable declaration
``
    let foo = "bar";
``

## if statement
``
    if 25 == 23 {
        ...
    }
``

## conditials
``
    let true_var = 25 == 25 and 42 == 42;
    let false_var = 25 != 25 or 42 != 42;
``

## functions
``
    fun foo(p1, p2) {
        print (p1 + p2);
    }
``
### functions are first class
``
    fun exec(f) {
        f(2, 5);
    }
``

## shorthand functions
``
    fun square(x) return x*x;
``


## square every number in list
``
    fun square(x) return x*x;
    fun map_list(list, f) {
        let res = list;
        // TODO: implement built-in 'len'
        for l in [0, 1, 2] {
            res[l] = f(res[l]);
        }
        return res;
    }
    let res = map_list([1,2,3], square);
``

## TODO SYNTAX
``
    let r = 0:5;
    // expands to [0, 1, 2, 3, 4]
``
