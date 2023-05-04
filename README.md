# kumiawase-onesan
お姉さんが実行したシンプルな数え上げプログラム。

```
$ cargo run -- 4
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/kumiawase-onesan 4`
result = 8512

$ cargo run --release -- 5
    Finished release [optimized] target(s) in 0.00s
     Running `target/release/kumiawase-onesan 5`
result = 1262816
```

```
% time cargo run --release -- 6
    Finished release [optimized] target(s) in 0.00s
     Running `target/release/kumiawase-onesan 6`
result = 575780564
cargo run --release -- 6  2175.08s user 8.26s system 99% cpu 36:27.15 total
```
