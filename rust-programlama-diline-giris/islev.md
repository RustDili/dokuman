# İşlevler
Girilen **n** adet parametreyi alıp, bir mantık çerçevesinde sarmalanmış kod bloğuna aktararak bir veya birden fazla sonucun üretilmesini ve çağrıldığı noktaya döndürülmesini sağlayan yapısal bloklardır.

### İsimlendirilmiş İşlevler
- İşlevin İngilizce karşılığı function'ı simgeleyen `fn` anahtar sözcüğü ile bildirilirler.
- Argüman ile kullanıldığında bu argümanların veri türleri parametre listesinde bildirilir.
- İşlevlerin boş bir **Tuple ()** döndüreceği varsayılır. İşlevin sonuç döndürmesi isteniyorsa, dönüş türleri **`->`** işaretinden sonra belirtilir.

#### i. Merhaba dünya!

```Rust
fn main() { 
  println!("Merhaba dünya!"); 
}
````

#### ii. Argüman geçirmek

```Rust
fn topla(a: i8, b: i8) { 
  println!("toplam: {}", a + b); 
}

fn main() {
  topla(10, 20);
}
````

#### iii. Dönen değerler

```Rust
// 01. Return anahtar sözcüğü kullanılmayan örnek. Sadece son ifade döner.
fn birle_topla(a: i32) -> i32 { 
  a + 1	// Son ifade bu satır olduğundan dönecek olan ifade bu satırdadır. Son satırda noktalı virgül aranmaz. 
        // Ve bu bildirimin return a + 1; ifadesine eşit olduğu anlaşılır. 
} 

// 02. Return anahtar sözcüğü kullanan örnek. 
fn ikiyle_topla(a: i32) -> i32 { 
  return a + 2; // a+2 döndürülür. Ancak bu kötü bir uygulama olarak kabul edilir. Bu tür kullanımlar genellikle  
                // koşullu ifadelerde yer alırlar. Diğer hallerde son ifade yöntemi tercih edilmelidir
}
````

#### iv. Veri türü olarak kullanılan işlev işaretçisi

```Rust
// 01. Tür bildirimsiz kullanım 
  let b = birle_topla; 
  let c = b(5); // 6 

// 02. Tür bildirimli kullanım 
  let d: fn(i32) -> i32 = ikiyle_topla; 
  let e = d(5); // 7
````

### İsimsiz işlevler
Kapama ya da lambda işlevler olarak bilinirler; argümanlarının veri türlerini bildirmek veya işlevden sonuç döndürmek isteğe bağlıdır.

Standart biçimde tasarlanmış bir işlev aşağıdaki gibi ifade edelebilir.

```Rust
fn get_square_value(x: i32) -> i32 { 
  x * x	
}

fn main() { 
  let x = 2; 
  println!("{}", get_square_value(x)); // 4 yazdırır.
} 
````

Aynı örneği isimsiz işlev olarak tasarlanamak çok daha pratiktir. Böylece işlevin giriş ve çıkış türlerini bildirmek isteğe bağlı kalır.

```Rust
fn main() {
    let x = 3;
    
    let karesi = |x: i32|-> i32 {
        x * x
    };
    
    println!("{}'in karesi: {}", x, karesi(x));
}
````
Giriş ve dönüş türlerini belirtmenin isteğe bağlı olduğu bu isimsiz işlevde, giriş parametreleri işleve `||` kullanılarak geçirilir ve ifade gövdesi `{};` köşeli parantezler ile sarılır.

Eğer giriş ve dönüş türleri belirtilmeyen isimsiz işlev tek bir satırdan oluşuyorsa `{};` köşeli parantez kullanmak zorunlu değildir.

```Rust
fn main() {
    let a = 3;
    let b = 5;
    
    let carp = |a, b| a * b;
    println!("a: {} ve b: {} çarpımı: {}'dır.", a, b, carp(a, b));
}
````
