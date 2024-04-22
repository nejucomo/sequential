A [Sequential] trait abstraction for generating/consuming sequences of values with an explicit termination value

The fundamental method is [Sequential::into_next]:

```rust,ignore
fn into_next(self) -> Either<(Self, Self::Item), Self::Terminal>;
```

This either produces a means of contuing (via `Self`) and an [Item](Sequential::Item), or else a [Terminal](Sequential::Terminal) value. Because this method consumes `self`, it ensures the [Sequential] state is dropped upon termination.

# Example

```
use sequential::Sequential;
use std::io::{BufRead, BufReader, Read};

fn count_lines_and_chars<R>(r: R) -> std::io::Result<(usize, usize)>
where
    R: Read,
{
    let mut lines = 0;
    let mut chars = 0;
    let seq = BufReader::new(r).lines().terminate_on_err();
    seq.for_each(|line| {
        lines += 1;
        chars += line.chars().count();
    })?; // Notice the '?' propagation.

    Ok((lines, chars))
}
```

# [Iterator] name collisions

Because [Sequential] provides a lot of similar functionality to [Iterator], the same method names are used where it makes sense. Meanwhile, there is a blanket impl for [Sequential] for every [Iterator], which is convenient for enabling any [Sequential] consuming API to be passed an [Iterator] directly (or likelywise for [IntoSequential]).

This means in some cases there is method name ambiguity:

## Example: Method Name Ambiguity
```rust,compile_fail
use sequential::Sequential;

let it = 0..5;
let mut acc = 0;

// Do we mean `Iterator::for_each` or `Sequential::for_each`?
it.for_each(it, |inc| acc += inc);

assert_eq!(acc, 10);
```

The most direct solution is to use [Fully Qualified Syntax for Disambiguation](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#fully-qualified-syntax-for-disambiguation-calling-methods-with-the-same-name):

```rust
use sequential::Sequential;

let it = 0..5;
let mut acc = 0;
Sequential::for_each(it, |inc| acc += inc);
assert_eq!(acc, 10);
```

However, this can be avoided wherever variable bounds can disambiguate:

```rust
use sequential::Sequential;

fn sum_elements<S>(seq: S) -> u64
where
    S: Sequential<Item = u64>
{
    let mut acc = 0;
    seq.for_each(|inc| acc += inc);
    acc
}

let sum = sum_elements(0..5);
assert_eq!(sum, 10);
```

