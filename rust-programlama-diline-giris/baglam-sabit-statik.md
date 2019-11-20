# Değişken bağlamı, sabit ve statik öğeler
Rust programlama dilinde değişkenler aksi belirtilmediği sürece değişmez olarak kabul edildiğinden, değişken kavramıyla aslında **ilklendirilme bağlamına** işaret edilir. Eğer bu bağlamın değişebilir olması isteniyorsa değişken bildirilirken `mut` anahtar sözcüğünün kullanılması gereklidir.
Rust statik olarak tasarlanmış bir dil olduğundan, veri türlerini derleme zamanında kontrol eder. Bu nedenle bağlam tanımlarında türün bildirimi zorunlu tutulmaz. Türün bildirilmediğinde, derleyici ifede ve kullanımı kontrol ederek en olası veri türünü belirlemeye çalışır. 
Bununla birlikte eğer Sabit ya da Statik bir bağlam tanımlanmak isteniyorsa o bağlamın türü mutlaka bildirilmek zorundadır. Türler `(:)` iki noktanın ardından bildirilir. 

### Değişken bağlamı
Rust dilinde bir bağlam ifade edilirken, bağlayıcı olarak genellikle `let` anahtar sözcüğü kullanılır. Bu yolla programda kullanılan isimleri bir değer yada işleve bağlamak mümkün olur. 

```Rust
fn main() {
  let a = true; 
  println!("a değişmezi: {}", a);
  
  let b: bool = false; 
  println!("b değişmezi: {}", b);
  
  let (x, y) = (1, 5); 
  println!("x: {}, y: {}", x, y);
  
  let mut z = 5; 
  println!("z değişkeni: {}", z);
  z = 6;
  println!("z değişimi: {}", b);
}
````

Bağlayıcı ifadenin sol tarafı bir model olduğundan `let` anahtar sözcüğü yardımıyla çok sayıda isim bir dizi değere veya işlev değerine bağlanabilir.

```Rust
let (x, y,z ) = (1, 5, 10); 
  println!("x: {}, y: {}, z: {}", x, y, z);
````

Sabitleri tanımlamak için `const` anahtar sözcüğü kullanılır. Ancak bellekte sabit bir adresi bulunmayan bu türlerin ömrü programın yaşam süresi kadardır.

```Rust
fn main() {
  const N: i32 = 5;
  println!("N sabitinin değeri: {}," N);
}
````

Bir global değişken türü tanımımlanırken `static` anahtar sözcüğü kullanılır. Bu türlerin her değeri için sadece bir örnek olabilir ve bu örnekler bellekte sabit bir adreste bulunurlar.

```Rust
fn main() {
  static S: i32 = 10;
  println!("S global değeri: {}," S);
}
````

Statik öğeler genellikle kod dosyasının en üstünde işlevlerin dışında bulundurulurlar. Sabit bir türün hafıza adresine nadiren başvurulduğundan, zorunlu olmadıkça statik tür yerine sabit öğelerin kullanılması yeğlenir. Bu tercih programın optimizasyonu için de önemlidir.

