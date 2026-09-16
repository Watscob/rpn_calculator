A simple CLI Reverse Polish Notation (RPN) calculator.

## Run
```sh
cargo run
```

## Unit tests
```sh
cargo test
```

## Available commands
| Command | Description                                             |
| ------- | ------------------------------------------------------- |
| `_`     | -x                                                      |
| `+`     | x + y                                                   |
| `-`     | x - y                                                   |
| `*`     | x * y                                                   |
| `/`     | x / y                                                   |
| `%`     | x % y                                                   |
| `~`     | x / y then x % y                                        |
| `^`     | x exp y                                                 |
| `v`     | sqrt(x)                                                 |
| `p`     | Print the last value of the stack                       |
| `n`     | Pop and print the last value of the stack (no newline)  |
| `f`     | Print the full stack                                    |
| `c`     | Clear the stack                                         |
| `d`     | Duplicate the last value of the stack                   |
| `r`     | Reverse the last 2 values of the stack                  |
| `k`     | Set the precision to the last value of the stack        |
| `K`     | Push the precision to the stack                         |

## Example
```sh
$ cargo run
9 4 /
p
2
4 k
f
2
3 /
p
0.7500
```
