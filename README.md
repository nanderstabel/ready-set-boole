# ready-set-boole

This repository consists of a collection of exercises related to **Boolean Algebra** and **Set Theory**, both of which are very important theories in both mathematics and computer science.

The following binaries can be build and run:

```
adder
multiplier
gray_code
boolean_evaluation
truth_table
negation_normal_form
conjunctive_normal_form
SAT
powerset
set_evaluation
curve
inverse_function
```

Example:
```
cargo run --release --bin truth_table
```

To run tests for a specific binary:
```
cargo test --bin set_evaluation
```

To run all tests at once:
```
cargo test
```

More information on the different binaries can be found [here](https://github.com/nanderstabel/ready-set-boole/tree/main/src/bin)
