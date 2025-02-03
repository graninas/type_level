# Tools for type-level domain modeling in Rust

These packages contain various tools for type-level domain modeling. Create powerful type-level eDSLs with easiness.

## Example of usage

Type-level eDSL that describes the Conway's Game of Life rule:

```rust
type A = State<tl_str!("Alive"), 1>;
type D = State<tl_str!("Dead"), 0>;

type Neighbors3  = NeighborsCount<A, tl_i32![3]>;
type Neighbors23 = NeighborsCount<A, tl_i32![2, 3]>;

type GolTransitions = tl_list![IStateTransition,
   StateTransition<D, A, Neighbors3>,
   StateTransition<A, A, Neighbors23>];

type GoLStep = Step<D, GolTransitions>;

type GoLRule = Rule<
   tl_str!("Game of Life"),
   tl_str!("gol"),
   GoLStep>;
```

For more examples, see this repo: [type-level-interfaces-in-rust](https://github.com/graninas/type-level-interfaces-in-rust).

## Crates

### `tl_interface`
Type-level interface and the universal evaluator.

### `tl_list_lib`
Type-level kinded list.

### `tl_str_list`
Type-level string (type-level list of char constants).

### `tl_str_macro`
Helper macros for better UX of type-level strings.

