# Solana Instruction Data: Fixed vs Dynamic Types

This document explains how **instruction data** in Solana programs is serialized, with a focus on **which types have length prefixes**.

---

## 1. Overview

- `instruction_data` is the raw byte array sent to a Solana program.  
- The structure depends on the **types of arguments** in the instruction.  
- Each instruction often starts with an **instruction discriminator** (8 bytes in Anchor), followed by the serialized arguments.

---

## 2. Fixed-Size Types (No Prefix)

- Fixed-size types occupy a known number of bytes.  
- **No length prefix** is needed.  
- Examples:

| Type             | Size (bytes) |
|-----------------|--------------|
| `u8`             | 1            |
| `i8`             | 1            |
| `u16`            | 2            |
| `i16`            | 2            |
| `u32`            | 4            |
| `i32`            | 4            |
| `u64`            | 8            |
| `i64`            | 8            |
| `u128`           | 16           |
| `i128`           | 16           |
| `bool`           | 1            |
| `[T; N]`         | `size_of(T) * N` |
| `Pubkey` (32-byte array) | 32     |
| `Option<Fixed>`  | 1 byte (for `Some/None`) + size of inner type if `Some` |

> **Rule:** Program knows the size → no length prefix.

---

## 3. Dynamic (Variable-Length) Types (4-Byte Prefix)

- Variable-length types can change size per instruction.  
- **Always prefixed with a 4-byte little-endian length (`u32`)**.  
- Examples:

| Type                  | Serialization |
|-----------------------|---------------|
| `String`              | `[u32 length][UTF-8 bytes]` |
| `Vec<T>`              | `[u32 length][elements]`    |
| `Option<Vec<T>>`      | `[1 byte Some/None][u32 length + elements if Some]` |
| Structs with dynamic fields | Only dynamic fields get the length prefix |

> **Rule:** Program does not know the size → 4-byte length prefix tells how many bytes to read.

---

## 4. Examples

**String `"TestToken"` serialization:**

- **UTF-8 bytes:**  
  `[84, 101, 115, 116, 84, 111, 107, 101, 110]`  
  *(Raw bytes of the string)*

- **Length prefix (`u32`, little-endian): uses 4-bytes to represent the length**  
  `[9, 0, 0, 0]`  
  *(Specifies the string length: 9 bytes)*

- **Serialized (full representation):**  
  `[9, 0, 0, 0, 84, 101, 115, 116, 84, 111, 107, 101, 110]`  
  *(Prefix + UTF-8 bytes; this is what the Solana VM expects)*


**Vector `[1, 2, 3, 4]` serialization:**

- **Length prefix (`u32`, little-endian):**  
  `[4, 0, 0, 0]`  
  *(Specifies the vector length: 4 elements)*

- **Elements:**  
  `[1, 2, 3, 4]`  
  *(Raw bytes of the vector elements)*

- **Serialized (full representation):**  
  `[4, 0, 0, 0, 1, 2, 3, 4]`  
  *(Prefix + elements; this is what the Solana VM expects)*


**Fixed-size `u64 = 42` serialization:**

- **Serialized:**  
  `[42, 0, 0, 0, 0, 0, 0, 0]`  
  *(8 bytes, little-endian; no length prefix)*


## 5. Quick Rule of Thumb

- **Fixed-size → no length prefix**  
- **Variable-size → 4-byte little-endian length prefix**  


## 6. Using Borsh to automate this

```rust
// Define a Struct representing your input params
#[derive(BorshSerialize, BorshDeserialize)]
struct InitializeMintAccountArgs {
    name: String,
    symbol: String,
    uri: String,
    supply: u64,
}

// Then use it in your tests 

// Params
let name = "TestToken";
let symbol = "TTK";
let uri = "https://cdn-icons-png.flaticon.com/512/17978/17978725.png";
let supply: u64 = 100000;

// Instruction data consist of [Instruction_discriminator][serialized instruction data, in other words params]
// Since i need every argument to have 4-byte length prefix for every dynamic arg i must use Borsh for simplification
let args = InitializeMintAccountArgs {
    name: name.to_string(),
    symbol: symbol.to_string(),
    uri: uri.to_string(),
    supply,
};

// This is the full instruction data
let mut instruction_data = discriminator;
let args_bytes = borsh::to_vec(&args).expect("Failed to serialize args");
instruction_data.extend_from_slice(&args_bytes); 
```