This repository contains a benchmark to compare different way of passing arguments with a type unknown at compile time to a function.

Two types of arguments are compared: generics and trait objects. Generics have three different syntaxes so those three methods are present in the benchmark method, although they are all equivalent.
To see how the argument passing performs, the methods are tested against different parameter types:
- A concrete type object
- A dynamic trait
- Boxed concrete type and dynamic objects

# Benchmark results

Times are given for my setup, and shouldn't be considered as absolute but only relatively.

|                                       | Generics: `print_area_impl` | Trait objects: `print_area_dyn` |
| ------------------------------------- | --------------------------- | ------------------------------- |
| **Reference to concrete object**: `&` | 0.5 ns                      | 2.5 ns                          |
| **Reference to dyn object**: `&dyn`   | 1.0 ns                      | 2.9 ns                          |

Using `print_area_where` and `print_area_simplified` lead to the same result as `print_area_impl` as the syntax may differ but all methods still use generics. Similarly, passing a Boxed object has the same performances as passing a reference as `Box` is a *zero cost abstraction* in Rust.