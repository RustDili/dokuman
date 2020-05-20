## Standart kütüphane, temel türler ve ön kütüphaneler
⭐️ Rust dilinin bileşenleri sadece `std` kütüphane sandığı tarafından değil, derleyici tarafından da uygulanır. Bunlar: 
  
  - **[İlkel türler](https://doc.rust-lang.org/std/#primitives)**: Derleyici tarafından tanımlanan metodlar temel türlere doğrudan `std` kütüphanesi tarafından uygulanır.
  - **[Standard Makrolar](https://doc.rust-lang.org/std/#macros)**: Hem derleyici hem de `std` tarafından tanımlanır.
  
**`std`** kütüphanesi, kapsadıkları alanlara göre **[modüllere](https://doc.rust-lang.org/std/#modules)** ayrılmıştır.

⭐️ Temel türler **derleyici** tarafından uygulanırken, temel türlere için **en yararlı metodları** standart kütüphane tarafından doğrudan  uygulanır. Ancak, bazı temel türlerin **daha az kullanılan dil öğeleri**, ilgili **std** modüllerinde bulunduğundan **std** modüllerinde hem `char`, `str` hem de tamsayı türlerini bir arada görmeniz doğaldır.

## Temel türler
Derleyici tarafından tanımlanan ve doğrudan `std` kütüphanesi tarafından uygulanan metodlar aşağıda yer almaktadır:

```Rust
bool, char, slice, str

i8, i16, i32, i64, i128, isize
u8, u16, u32, u64, u128, usize

f32, f64

array, tuple

pointer, fn, reference
````

## Standard Makrolar
Hem derleyici hem de `std` kütüphanesi tarafından tanımlanan makrolar:

```Rust
print, println, eprint, eprintln
format, format_args
write, writeln

// concat_idents: yalnızca `nightly` sürümü için deneysel API
concat, concat_idents, stringify 

include, include_bytes, include_str

assert, assert_eq, assert_ne
debug_assert, debug_assert_eq, debug_assert_ne

try, panic, compile_error, unreachable, unimplemented

file, line, column, module_path
env, option_env
cfg

// select: yalnızca `nightly` sürümü için deneysel API
select, thread_local

vec
````

## Std modülleri
```rust
char, str

i8, i16, i32, i64, i128, isize
u8, u16, u32 ,u64, u128, usize
f32, f64
num

// heap: yalnızca `nightly` sürümü için deneysel API
vec, slice, hash, heap, collections

string, ascii, fmt

default

marker, clone, convert, cmp, iter

ops, ffi

option, result, panic, error

io
fs, path
mem, thread, sync
process, env
net
time
os

ptr, boxed, borrow, cell, any, rc

prelude

// intrinsics: yalnızca `nightly` sürümü için deneysel API
intrinsics
// raw: yalnızca `nightly` sürümü için deneysel API
raw 
````

> 🔎 [Rust’un kaynak kodları](https://github.com/rust-lang/rust)nı incelediğinizde, [src dizini](https://github.com/rust-lang/rust/tree/master/src)nin bir **çalışma alanı** olduğunu görebilirsiniz. Çok sayıda kütüphane sandığına sahip olmasına rağmen, [kök Cargo.toml](https://github.com/rust-lang/rust/blob/master/src/Cargo.toml) dosyasını incelediğinizde, temel sandıkların [rustc](https://github.com/rust-lang/rust/tree/master/src/rustc) *(derleyici)* ve [libstd](https://github.com/rust-lang/rust/tree/master/src/libstd) *(std)* olduğunu rahatlıkla fark edersiniz. **Std modülleri**nin çoğunun orijinal konumu `src/libcore`'dur. Bu modüllerin `use pub` kullanımı yoluyla yeniden dışa aktarılmış olduğunu `Libstd/lib.rs` dosyasından görebilirsiniz.

**Oldukça önemli `std` modüllerinden birkaçı** aşağıda sıralanmıştır.
- `std::io` - Çekirdek **I/O** işlevselliği
- `std::fs` - **Filesystem** Dosya sistemine özgü işlevsellik
- `std::path` - **Cross-platform path** platformlar arası yol işlevselliği
- `std::env` - **Process’s environment** Sürecin/işlemin çevresel işlevselliği
- `std::mem` - **Memory** Hafıza ile ilgili işlevler
- `std::net` - **TCP/UDP** TCP/UDP iletişimi
- `std::os` - **OS** İşletim sistemine özgü işlevsellik
- `std::thread` -  Yerel **İşliklere"" özgü işlevsellik
- `std::collections` - Çekirdek **koleksiyon türleri**

> 💯 Daha fazla detay için [Rust Standard Kütüphane Belgeleri](https://doc.rust-lang.org/std/)ni inceleyebilirsiniz.

## Ön kütüphaneler
