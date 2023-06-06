# Bolt Scripting Language

## Todo
-   [ ] Parse While, Fncs
-   [ ] Add Test Cases
-   [ ] Proper Error Parsing
-   [ ] Refactor code using better standards

# Bolt specification

High level features -
-   Dynamic typing
-   Automatic Memory Management

## Data Types
-   Boolean - true, false

-   Number - integer or decimal
```
let a = 10;
//Or
let b = 10.2;
```
Bolt only has one type which is Number and handles decimals and integers both as double-precision floating point

-   String - "Hello world"

-   NULL - No value

## Comparison and equality

less < than;
lessThan <= orEqual;
greater > than;
greaterThan >= orEqual;

```
1 == 2; // false.
"a" != "b"; // true.
```

Different types can be compared.
```
1 == "a"; // false.
```


Values of different types are never equivalent in Bolt.
```
123 == "123"; // false.
```

## Logical operators

The not operator, a prefix !, returns false if its operand is true, and vice
versa similar to Javascript.

```
!true; // false.
!false; // true.
```

## Control Flow

```if (condition) {
print "yes";
} else {
print "no";
}

While -

while (condition) {
    // Run the code
}

For loops -

for (var a = 1; a < 10; a = a + 1) {
    print a;
}
```

## Functions

```
printName(firstName, lastName);
```

You can also call a function without passing anything to it.
```
printName();
```

Bolt supports Closures as well!

"// or /* */" can be used for comments

```
// Comments in Bolt
let a = 10;
```