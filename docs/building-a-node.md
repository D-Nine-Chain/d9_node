# building a node

if compiles successfully with cargo build, then can do
on an EC2 Ubuntu instance it is imperative to use `cargo build --release` if memory is less than 8GB. otherwise you will get an

`error: linking with `cc` failed: exit code: 1` error.

```
cargo build --release
```

after completion can run:

```
./target/release/d9-node -h
```

to explore the parameters and subcommands of the node.
