openrpc-stub-gen
================

!WIP!

JSON-roundtrip and JSON-aware comparison with the input file:

```
cargo run --release -- ./api/input.openrpc JSON 2>/dev/null | jq . > debug.json
diff <(jq --sort-keys . ./api/input.openrpc) <(jq --sort-keys . debug.json)
```

Dump the AST:

```
cargo run --release -- ./api/input.openrpc TREE > tree.txt 2> debug.txt
```

Generate the code into example and then run it (must print `OK`):

```
rm -rf examples && mkdir examples
cargo run --release -- ./api/input.openrpc CODE > examples/gen.rs
echo '\nfn main() { println!("OK"); }' >> examples/gen.rs
cargo run --example gen 2> debug.txt
```
