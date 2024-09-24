# noproto

No-`std`, no-`alloc` protocol buffers (protobuf) implementation in Rust, for embedded systems.
Optimized for binary size and memory usage, not for performance.

Status: **very experimental,** :radioactive: do not use in production yet. In particular, it doesn't
handle many protobuf types well (see below).

## Features

Implemented:

- Derive macros.
- `heapless::Vec`, `heapless::String` impls.
- `optional`
- `repeated`
- `oneof`
- `enum`

Not implemented (yet?):

- Support multiple protobuf encodings. Protobuf "types" are more like "type + wire encoding" all in one,
  so one Rust type can be encoded multiple ways on the wire. `noproto` currently assumes the Rust type is enough
  to deduce how it should be encoded on the wire, which is not true.
- Support more types (see below)
- Impls for `alloc` containers (goal is to be `no-alloc`, but we could still have them optionally).
- Impls for `&[T]` for repeated fields (only doable for writing, not reading)
- Tool to compile `.proto` files into Rust code.
- Maps
- Deprecated field groups.

## Type mapping

| Protobuf | Rust | 
|-|-|
| `bool` | `bool` |
| `int32` | `i32` |
| `uint32` | `u32` |
| `sint32` | TODO |
| `fixed32` | `noproto::Fixed32` |
| `sfixed32` | `noprooto::SFixed32` |
| `int64` | `i64` |
| `uint64` | `u64` |
| `sint64` | TODO |
| `fixed64` | `noproto::Fixed64`  |
| `sfixed64` | `noproto::SFixed64`  |
| `float` | `f32` |
| `double` | `f64` |
| `string` | `heapless::String<N>` |
| `bytes` | `heapless::Vec<u8, N>` |

## Minimum supported Rust version (MSRV)

`noproto` is guaranteed to compile on the latest stable Rust version at the time of release. It might compile with older versions but that may change in any new patch release.

## License

This work is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
