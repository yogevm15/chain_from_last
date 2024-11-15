# Chain from last
An iterator adaptor that chains the iterator with an iterator built from the last item.

## Example
```rust
fn it_works(){
    let words: Vec<_> = "lorem ipsum dolor;sit;amet"
        .split(" ")
        .chain_from_last(|l| l.split(";"))
        .collect();

    assert_eq!(words, vec!["lorem", "ipsum", "dolor", "sit", "amet"]);
}
```