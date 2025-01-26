u32 UInt uint32
```rust
4_294_967_295    4_294_967_295u 
0xFF_FF_FF_FF    0xFFFF_FFFFu
0b11111111_11111111_11111111_11111111    0b11111111_11111111_11111111_11111111
0o37_777_777_777
"4294967295".parse().unwrap(); // Mengonversi string ke u32
(2u32.pow(32) - 1) as u32; // Menghasilkan nilai maksimum u32
std::u32::MAX
std::u32::MAX.wrapping_sub()
std::u32::from(4_294_967_295u64 as u32)
(0..32).map(|_| 1u32).reduce(|a, b| a + b).unwrap() * 2_147_483_648; // 4_294_967_295
[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1].iter().sum();
std::u32::MAX.checked_add(0).unwrap();
// No underscores allowed
// Octal literal without underscores

```
u16
0xFFFF
