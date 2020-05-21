## Birleştiriciler
## Birleştirici nedir?
  - "Birleştirici" nin bir anlamı, **birleştirici modeline** atıfta bulunan, daha dolaylı anlam ifade eden, bir şeyleri birleştirme fikri etrafında merkezlenmiş kütüphaneleri düzenleme tarzıdır.
Genellikle **bazı `T` türleri**, **`T` türünün "temel" değerlerini oluşturmak için bir takım işlevler** ve **`T` türünün daha karmaşık değerlerini oluşturmak** için **`T` türü değerlerini çeşitli şekillerde birleştirebilen** bazı “birleştiriciler” vardır. Diğer bir tanım ise "serbest değişkensiz işlev" anlamındadır. __ [wiki.haskell.org](https://wiki.haskell.org/Combinator)
  - Birleştirici, program parçalarından yeni program parçaları oluşturan bir işlevdir. Birleştiricileri kullanan bir programcı bir anlamda her ayrıntıyı elle yazmaktan ziyade otomatik olarak olşturmayı tercih etmektedir. __ John Hughes — [Monad'ların Oklarla genelleştirilmesi](http://www.cse.chalmers.se/~rjmh/Papers/arrows.pdf) [İşlevsel Programlama Kavramları](https://github.com/caiorss/Functional-Programming/blob/master/haskell/Functional_Programming_Concepts.org)'ndan.
  
Rust ekosistemindeki "birleştiriciler" in tam tanımı ise biraz belirsizdir.

- `or()`, `and()`, `or_else()`, `and_then()` 
  ▸ **`T` türündeki iki değişkeni birleştirerek** ▸ **aynı `T` türünü döndürür**.
 
- `Option` türleri için `filter()`
  ▸ Koşullu bir işlevi bir kapama gibi kullanarak şarta göre **`T` türünü Filtreler** ▸ **aynı `T` türünü döndürür**.
  
- `map()`, `map_err()` 
  ▸ **`T` türünü bir kapama işlevi uygulayarak dönüştürür**. 
  ▸ Bu işlem sırasında  **`T` içindeki değerin veri türü değişebilir**. 
  Örneğin: `Some<&str>` `Some<usize>` dönüştürülebileceği gibi `Err<&str>` de `Err<isize>` türüne dönüştürülebilir.
  
- `map_or()`, `map_or_else()`
  ▸ **Bir kapama işlevi uygulayarak dönüştürdüğü `T` türü ve içindeki değerleri döndürür**.
  ▸ **`None` ve `Err`, için varsayılan bir değer ya da başka bir kapama işlevi** uygulanır.
  
- `Option` türleri için `ok_or()`, `ok_or_else()` 
  ▸ **`Option` türünü bir `Result` türüne dönüştürür**.
  
- `as_ref()`, `as_mut()` 
  ▸ **`T` türünü bir referansa veya değişebilir bir referansa dönüştürür**.
  
## `or()` ve `and()` metodları
İki ifadeyi birleştirerek, `Option` veya `Result` türlerinden birini döndürür.

  - **`or()` metodu:** `Some` ya da `Ok` değerlerinden birisi varsa bu değer hemen döner.
  - **`and()` metodu:** Her iki ifadede de `Some` veya `Ok` değerlerinden biri bulunuyorsa, ikinci ifadedeki değer geriye döner. Bunlardan herhangi birinde `None` veya `Err` bulunuyorsa bu değer hızlıca döner.
  
```Rust
fn main() {
  let s1 = Some("some1");
  let s2 = Some("some2");
  let n: Option<&str> = None;

  let o1: Result<&str, &str> = Ok("ok1");
  let o2: Result<&str, &str> = Ok("ok2");
  let e1: Result<&str, &str> = Err("error1");
  let e2: Result<&str, &str> = Err("error2");

  assert_eq!(s1.or(s2), s1); // Some1 veya Some2 = Some1
  assert_eq!(s1.or(n), s1);  // Some veya None = Some
  assert_eq!(n.or(s1), s1);  // None veya Some = Some
  assert_eq!(n.or(n), n);    // None1 veya None2 = None2

  assert_eq!(o1.or(o2), o1); // Ok1 veya Ok2 = Ok1
  assert_eq!(o1.or(e1), o1); // Ok veya Err = Ok
  assert_eq!(e1.or(o1), o1); // Err veya Ok = Ok
  assert_eq!(e1.or(e2), e2); // Err1 veya Err2 = Err2

  assert_eq!(s1.and(s2), s2); // Some1 ve Some2 = Some2
  assert_eq!(s1.and(n), n);   // Some ve None = None
  assert_eq!(n.and(s1), n);   // None ve Some = None
  assert_eq!(n.and(n), n);    // None1 ve None2 = None1
  
  assert_eq!(o1.and(o2), o2); // Ok1 ve Ok2 = Ok2
  assert_eq!(o1.and(e1), e1); // Ok ve Err = Err
  assert_eq!(e1.and(o1), e1); // Err ve Ok = Err
  assert_eq!(e1.and(e2), e1); // Err1 ve Err2 = Err1
}
````
> 🔎 Rust'ın nightly sürümü `Option` türleri için `xor()` türünü desteklerken, yalnızca ifade bir tanesi `Some` almışsayi `Some` döndürür, ancak ikisini birden döndürmez.

## `or_else()` metodu
`or()` metoduna benzer. Farklı olarak, ikinci ifadenin aynı türden bir `T` döndüren [kapama işlevi](https://github.com/rust-lang-tr/dokuman/blob/master/rust-programlama-diline-giris/ilk-adim/islev.md#i%CC%87simsiz-i%C5%9Flevler-kapamalar) olmasını şart koşar.

```rust
fn main() {
    // `Option` için or_else()
    let s1 = Some("some1");
    let s2 = Some("some2");
    // let fn_some = || -> Option<&str> { Some("some2") }; ifadesine benzer
    let fn_some = || Some("some2");

    let n: Option<&str> = None;
    let fn_none = || None;

    assert_eq!(s1.or_else(fn_some), s1);  // Some1 or_else Some2 = Some1
    assert_eq!(s1.or_else(fn_none), s1);  // Some or_else None = Some
    assert_eq!(n.or_else(fn_some), s2);   // None or_else Some = Some
    assert_eq!(n.or_else(fn_none), None); // None1 or_else None2 = None2

    // `Result` için or_else()
    let o1: Result<&str, &str> = Ok("ok1");
    let o2: Result<&str, &str> = Ok("ok2");
    // let fn_ok = |_| -> Result<&str, &str> { Ok("ok2") }; ifadesine benzer
    let fn_ok = |_| Ok("ok2"); 

    let e1: Result<&str, &str> = Err("error1");
    let e2: Result<&str, &str> = Err("error2");
    let fn_err = |_| Err("error2");

    assert_eq!(o1.or_else(fn_ok), o1);  // Ok1 or_else Ok2 = Ok1
    assert_eq!(o1.or_else(fn_err), o1); // Ok or_else Err = Ok
    assert_eq!(e1.or_else(fn_ok), o2);  // Err or_else Ok = Ok
    assert_eq!(e1.or_else(fn_err), e2); // Err1 or_else Err2 = Err2
}
````

## `and_then()` metodu
`and()` metoduna benzer. Tek fark, ikinci ifadenin aynı türden `T` döndüren bir [kapama işlevi](https://github.com/rust-lang-tr/dokuman/blob/master/rust-programlama-diline-giris/ilk-adim/islev.md#i%CC%87simsiz-i%C5%9Flevler-kapamalar) olmasını şart koşar.

```rust
fn main() {
    // `Option` için and_then()
    let s1 = Some("some1");
    let s2 = Some("some2");
    // let fn_some = |_| -> Option<&str> { Some("some2") }; ifadesine benzer
    let fn_some = |_| Some("some2");

    let n: Option<&str> = None;
    let fn_none = |_| None;

    assert_eq!(s1.and_then(fn_some), s2); // Some1 and_then Some2 = Some2
    assert_eq!(s1.and_then(fn_none), n);  // Some and_then None = None
    assert_eq!(n.and_then(fn_some), n);   // None and_then Some = None
    assert_eq!(n.and_then(fn_none), n);   // None1 and_then None2 = None1

    // // `Result` için and_then()
    let o1: Result<&str, &str> = Ok("ok1");
    let o2: Result<&str, &str> = Ok("ok2");
    // let fn_ok = |_| -> Result<&str, &str> { Ok("ok2") }; ifadesine benzer
    let fn_ok = |_| Ok("ok2");

    let e1: Result<&str, &str> = Err("error1");
    let e2: Result<&str, &str> = Err("error2");
    let fn_err = |_| Err("error2");

    assert_eq!(o1.and_then(fn_ok), o2);  // Ok1 and_then Ok2 = Ok2
    assert_eq!(o1.and_then(fn_err), e2); // Ok and_then Err = Err
    assert_eq!(e1.and_then(fn_ok), e1);  // Err and_then Ok = Err
    assert_eq!(e1.and_then(fn_err), e1); // Err1 and_then Err2 = Err1
}
````

## `filter()` metodu
