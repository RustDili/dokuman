### Borçlanma
Başka programlama dillerine aşina olanlar böyle bir kod tasarımıyla ilk karşılaştıklarında name, a ve b değişkenlerinin her birinin "Pascal" çıktısını üreteceğini tahmin edeceklerdir.

```rust
let name = "Pascal".to_string();
let a = name;
let b = name;
```

Ancak bu kod Rust derleyicisine gönderildiğinde durumun böyle olmadığı anlaşılır.  

```bash
error[E0382]: use of moved value: `name`
 --> src/main.rs:4:11
  |
2 |   let name = "Pascal".to_string();
  |       ---- move occurs because `name` has type `std::string::String`, which does not implement the `Copy` trait
3 |   let a = name;
  |           ---- value moved here
4 |   let b = name;
  |           ^^^^ value used here after move
```

Ürettiği oldukça faydalı hata açıklamasında derleyici, `name` adlı string türündeki değişken içeriğinin `a` değişkenine taşındıktan sonra `b` değişkenine de atanmaya çalışıldığını ve bu durumun hataya neden olduğunu söyler. Buradaki sorun `b` değişkenine atanmak istenen `name` değişkeninin mülkiyetinin başka bir değişkene geçmesi ve artık bir değere sahip olmamasından kaynaklanır. Neler olup bittiğini daha iyi kavrayabilmek için bellekte neler olduğunu anlamakta yarar var. `name` değişkeni ilklendiğinde hafızadaki durumu aşağıdaki gibi temsil edilmektedir.

```bash
            +–––+–––+–––+
stack frame │ • │ 8 │ 6 │ <– name
            +–│–+–––+–––+
              │
            +–V–+–––+–––+–––+–––+–––+–––+–––+
       heap │ P │ a │ s │ c │ a │ l │   │   │
            +–––+–––+–––+–––+–––+–––+–––+–––+

Ancak `name` değişken değeri `a` değişkenine atandığında mülkiyeti de `a` değişkenine aktarılmış olduğundan `name` değişkeni ilklendirilmeden bırakılmış olur. 

```bash
            [–– name ––] [––– a –––]
            +–––+–––+–––+–––+–––+–––+
stack frame │   │   │   │ • │ 8 │ 6 │
            +–––+–––+–––+–│–+–––+–––+
                          │
              +–––––––––––+
              │
            +–V–+–––+–––+–––+–––+–––+–––+–––+
       heap │ P │ a │ s │ c │ a │ l │   │   │
            +–––+–––+–––+–––+–––+–––+–––+–––+
```

Böyle bir durumda `b = name;` ifadesinin sürpriz şekilde hatayla sonuçlanmasına şaşırmak yerine, derleyicinin bu kodu daha çalıştırılmadan önce statik olarak analiz etmesine odaklanmak önemlidir. Rust derleyicisi güvensiz kod yazılmasına izin veren derleyicilerden değildir:-)

Ancak derleyicinin bu tutumu değişkenler aracılığıyla birden fazla noktada aynı verilere ulaşılması gerektiğinde maliyetli çözümleri de beraberinde getirir. Böyle bir durumla başa çıkmanın kolay ama pahalı bir yolu değişkene ait olan değeri kopyalamak ya da klonlamak olacak, ancak bu tercih bellekteki verilerin de çoğalacağı anlamına gelecektir.

```rust
let name = "Pascal".to_string();
let a = name;
let b = a.clone();
```

Yukarıdaki örnekte isim değişkeni a değişkenine atandıktan sonra kendisinden bir değer elde edilemeyeceğinden klonlamanın yeni atanan a değişkeni üzerinden yapıldığına dikkat edilmelidir.  Program çalıştığında bellekteki veriler düşürülmediği sürece aşağıdakine benzer şekilde temsil edilirler.

```bash
            [–– name ––] [––– a –––][–––– b ––––]
            +–––+–––+–––+–––+–––+–––+–––+–––+–––+
stack frame │   │   │   │ • │ 8 │ 6 │ • │ 8 │ 6 │
            +–––+–––+–––+–│–+–––+–––+–│–+–––+–––+
                          │           │
              +–––––––––––+           +–––––––+
              │                               │
            +–V–+–––+–––+–––+–––+–––+–––+–––+–V–+–––+–––+–––+–––+–––+–––+–––+
       heap │ P │ a │ s │ c │ a │ l │   │   │ P │ a │ s │ c │ a │ l │   │   │
            +–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+
```

Açıkçası verileri klonlamak, uğraşılan verilere bağlı olarak fazla bellek gerektiren pahalı bir işlemdir. Oysa bu senaryoda gerekli olan tek şey bir değere referans vermektir. Bu tür bir yönelim ise özellikle bir değere sahip olmanız gerekmeyen işlevleri yazarken oldukça kullanışlıdır. Kendisine verilen ismi kullanarak selamlama işlevini yerine getiren aşağıdaki gibi bir örnek hayal edelim:

```rust
fn greet(name: String) {
  println!("Hello, {}!", name);
}
```

Bu işlev kendisinin aynı değişkenle defalarca çağrılmasını önlediği gibi, kendisine geçirilen değerin mülkiyetine de gerek duymaz:

```rust
let name = "Pascal".to_string();
greet(name);
greet(name); // name değişkeni daha önce taşınmış olduğundan derlenmeyecek
```

Bir değişkenin referansını almak için & sembolünü kullanılır. Ve bu sembol kullanıldığında bir değer üzerinden referans beklendiği açıkça anlaşılır.

Genellikle bu türden API’ler farklı nedenlerden dolayı &str şeklinde daha genel amaçlara yönelik tasarlanmakla birlikte bu aşamada sadece &String’ e ihtiyaç duyulduğundan aşağıdaki şekilde tasarlanmıştır.

```rust
fn greet(name: &String) {
  println!("Hello, {}!", name);
}
```

İşlev şimdi bir string referansı beklediğinden yapılan bu değişiklik onun defalarca çağrılabilmesine neden olur. 

```rust
let name = "Pascal".to_string();
greet(&name);
greet(&name);
```

Örnekteki işlev gibi, bir değerin referansını bekleyen işlevler kendilerine aktarılan değerleri sadece *ödünç almış olduklarından o değerlere hiçbir zaman sahip olmazlar.

Artık değişkenleri birden fazla noktada aynı verilere ulaşabilecek şekilde değerlendirebiliriz.

```rust
let name = "Pascal".to_string();
let a = &name;
let b = &name;
```

Yukarıdaki kodda a ve b değişkenleri name değişkeni değerini referans gösterirken name değişkeni değerinin mülkiyetini hiçbir zaman kaybetmez. Aşağıdaki kod da aynı şekilde değerlendirilir. 

```rust
let name = "Pascal".to_string();
let a = &name;
let b = a;
```

Artık aşağıdaki atamaları gerçekleştirerek greet() işlevini çağırmak bir soruna yol açmaz.

```rust
let name = "Pascal".to_string();
let a = &name;
greet(a);

let b = a;
greet(a);
```
