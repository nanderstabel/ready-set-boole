## Adder
A function that takes as parameters two natural numbers `a` and `b` and returns one natural number that equals `a + b`.
```
Maximum time complexity : O(1)
Maximum space complexity : O(1)
```

## Multiplier
A function that multiplies `a` and `b` and returns the result.
```
Maximum time complexity : O(1)
Maximum space complexity : O(1)
```

## Gray code
Takes an integer `n` and returns its equivalent in [**gray code**](https://en.wikipedia.org/wiki/Gray_code).


## Boolean evaluation
A function that takes as input a string that contains a propositional formula in [**reverse polish notation**](https://en.wikipedia.org/wiki/Reverse_Polish_notation), evaluates this formula, then returns the result. if the formula is invalid, the behaviour of the return value is undefined.

```
Maximum time complexity : O(n)
```

## Truth table
A function that takes as input a string that contains a propositional formula in reverse polish notation, and writes its [**truth table**](https://en.wikipedia.org/wiki/Truth_table) on the standard output. If the formula is invalid, the behaviour is undefined.

```
Maximum time complexity : O(2^n)
```

## Negation Normal Form
A function that takes as input a string that contains a propositional formula in reverse polish notation, and returns an equivalent formula in [**Negation Normal Form**](https://en.wikipedia.org/wiki/Negation_normal_form), meaning that every negation operator must be located right after a variable. The result must only contain variables and the following symbols: `!`, `&` and `|` (even if the input contains other operations). If the formula is invalid, the behaviour is undefined.


## Conjunctive Normal Form
A function that takes as input a string that contains a propositional formula in reverse polish notation, and returns an equivalent formula in [**Conjunctive Normal Form**](https://en.wikipedia.org/wiki/Conjunctive_normal_form). this means that in the ouput, every negation must be located right after a variable and every conjunction must be located at the end of the formula. The result must only contain variables and the following symbols: `!`, `&` and `|` (even if the input contains other operations). If the formula is invalid, the behaviour is undefined.


## SAT
A function that takes as input a string that contains a propositional formula in reverse polish notation and tells whether it is satisfiable. The format of the propositional formulas is the same as usual. The function determines if there is at least one vombination of values for each variable of the given formula that makes the result be `true`. If such a combination exists, the function returns `true`, otherwise, it returns `false`. If the formula is invalid, the behaviour is undefined.


## Powerset
A function that takes as input a set of integers, and returns its [**powerset**](https://en.wikipedia.org/wiki/Power_set).
```
Maximum space complexity : O(2^n)
```

## Set evaluation
A function that takes as input a string that contains a propositional formula in reverse polish notation, and a list of sets (each containing numbers), then evaluates this list and returns the resulting set. Each character represents a symbol. Each letter represents a set that is passed to the function. The set A is the first set, the set B is the second set, etc... The globally encompassing set is considered to be the union of all the sets given as parameters. If the formula is invalid, or if the amount of sets provided in the list is not equal to the amount of variables in the formula, the behaviour is undefined.


## Curve
A function (the inverse of a space-filling curve, used to encode spatial data into a line) that takes a pair of coordinates in two dimensions and assigns a unique value in the closed interval [0; 1] ∈ R. Let f be a function:
```
f : (x,y) ∈ [[0;216 −1]]^2 ⊂ N2 → [0;1] ∈ R
```
The above function f is bijective and represents the function to implement. You’re
free to use the method you want as long as it stays bijective. If the input is out of range, the behaviour is undefined.


## Inverse function
The inverse function f−1 of the function f, so this time, this is a [**space-filling curve**](https://en.wikipedia.org/wiki/Z-order_curve). If the input is out of range, the behaviour is undefined.

For more detailed information about the requirements of this project check [this](https://github.com/nanderstabel/ready-set-boole/blob/main/ready-set-boole.pdf).
