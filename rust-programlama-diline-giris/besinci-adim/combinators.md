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

````
