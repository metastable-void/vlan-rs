# VLAN ID Rust library
Clean VLAN ID representation with transparent u16 support, an invalid VLAN Error type, and a value that represents a native VLAN.

## Usage
```rust
use vlan::MaybeVlanId;

let native = MaybeVlanId::NATIVE;
assert_eq!(native, MaybeVlanId::try_new(0u16).unwrap());
assert!(MaybeVlanId::try_new(4095).is_err());

// memory-level compatibility with u16!
let zero: u16 = 0u16;
let should_be_zero: u16 = unsafe { std::mem::transmute(MaybeVlanId::NATIVE) };
assert_eq!(zero, should_be_zero);
```

## License
Apache 2.0 or MPL 2.0.
