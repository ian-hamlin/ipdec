# ipdec

Playing with rust.  Pass an IP to show the type and decimal.

# Example

```
cargo run 216.147.214.045
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/ipdec 216.147.214.045`
V4(216.147.214.45) 3633567277
```

```
cargo run 2405:0205:8481:84DB:76A9:4824:523E:917C
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/ipdec '2405:0205:8481:84DB:76A9:4824:523E:917C'`
V6(2405:205:8481:84db:76a9:4824:523e:917c) 47878210334518265565253214373104488828
```
