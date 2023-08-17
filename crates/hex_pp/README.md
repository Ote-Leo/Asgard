# Hex Pretty Printer

A library providing a pretty hex print functionality.

- A `simple_hex()` way renders one-line hex dump.
- A `pretty_hex()` way renders columned multi-line hex dump with addressing and ASCII
  representation.
- A `config_hex()` way renders hex dump in specified format.

## Example of `simple_hex()`

Input

```rust
use pretty_hex::*;

let v = vec![222, 173, 190, 239, 202, 254, 32, 24];
assert_eq!(simple_hex(&v), format!("{}", v.hex_dump()));

println!("{}", v.hex_dump());
```

Output

```txt
de ad be ef  ca fe 20 18
```

## Example of `pretty_hex()`

Input

```rust
use pretty_hex::*;

let v: &[u8] = &random::<[u8;30]>();
assert_eq!(pretty_hex(&v), format!("{:?}", v.hex_dump()));

println!("{:?}", v.hex_dump());
```

Output

```txt
Length: 30 (0x1e) bytes
0000:   6b 4e 1a c3  af 03 d2 1e  7e 73 ba c8  bd 84 0f 83   kN......~s......
0010:   89 d5 cf 90  23 67 4b 48  db b1 bc 35  bf ee         ....#gKH...5..
```

## Example of `config_hex()`

Input

```rust
use pretty_hex::*;

let cfg = HexConfig {title: false, width: 8, group: 0, ..HexConfig::default() };

let v = &include_bytes!("data");
assert_eq!(config_hex(&v, cfg), format!("{:?}", v.hex_conf(cfg)));

println!("{:?}", v.hex_conf(cfg));
```

Output

```txt
0000:   6b 4e 1a c3 af 03 d2 1e   kN......
0008:   7e 73 ba c8 bd 84 0f 83   ~s......
0010:   89 d5 cf 90 23 67 4b 48   ....#gKH
0018:   db b1 bc 35 bf ee         ...5..
```

inspired by [haskell's pretty hex](https://hackage.haskell.org/package/pretty-hex-1.0).
