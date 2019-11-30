### Borçlanma
Başka programlama dillerine aşina olanlar aşağıdakine benzer bir kod tasarımıyla ilk karşılaştıklarında `name`, `a` ve `b` değişkenlerinin her birinin **Pascal** çıktısını üreteceğini tahmin edeceklerdir.

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

Derleyicinin ürettiği hatanın açıklamasında, `name` adlı string türündeki değişken değerinin `a` değişkenine taşındıktan sonra `b` değişkenine de atanmaya çalışıldığını ve bu durumun hataya neden olduğunu söyler. Buradaki sorun; `b` değişkenine atanmak istenen `name` değişkenine ait tüm kaynakların önceden `a` değişkene aktarılması ve `name` değişkeninin artık herhangi bir değerle ilişkisinin olmamasından kaynaklanır. Neler olup bittiğini daha iyi kavrayabilmek için bellekte neler olduğunu anlamakta yarar var. `name` değişkeni ilklendiğinde hafızadaki durumu aşağıdaki gibi temsil edilmektedir.

```bash
            +–––+–––+–––+
stack frame │ • │ 8 │ 6 │ <– name
            +–│–+–––+–––+
              │
            +–V–+–––+–––+–––+–––+–––+–––+–––+
       heap │ P │ a │ s │ c │ a │ l │   │   │
            +–––+–––+–––+–––+–––+–––+–––+–––+
````

Ancak `name` değişken değeri `a` değişkenine atandığında, mülkiyeti de `a` değişkenine aktarılmış olduğundan `name` değişkeni ilklendirilmeden bırakılmış olur ve ait olduğu kapsam boyunca yeni bir değer ile bağlanmazsa kapsamdan çıkıldığında otomatik olarak yok edilir.

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

Derleyicinin bu kodu daha çalıştırılmadan önce statik olarak analiz etmesi ve güvensiz kod üretimini engelemesi önemlidir. Ancak böyle bir derleyici denetimi, birden fazla noktadan aynı verilere ulaşmak gerektiğinde oldukça maliyetlidir. Bu durumla başa çıkmanın kolay fakat pahalı bir yolu; değişkene ait değeri kopyalamak ya da klonlamaktan geçer, ancak bu tercih bellekteki verilerin de artmasına neden olur.

```rust
let name = "Pascal".to_string();
let a = name;
let b = a.clone();
```

Yukarıdaki örnekte görüleceğ gibi `name` değişkeni `a` değişkenine atandığında, artık kendisinden değer elde edilemeyeceğinden, klonlama yeni atanan `a` değişkeni üzerinden yapılır.  Program çalıştığında bellekteki veriler düşürülmediği sürece hafızada aşağıdakine benzer şekilde temsil edilirler.

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

Açıkçası verileri klonlamak, uğraşılan verilere bağlı olarak fazla bellek gerektiren pahalı bir işlemdir. Oysa bu senaryoda gerekli olan tek şey ilgili değere **referans** vermekten başka bir şey değildir. **Referans vermek**; özellikle bir değerin mülkiyeti gerekmeyen işlev tasarımlarında oldukça yararlıdır. Kendisine verilen adı kullanarak selamlama işlevini yerine getiren aşağıdaki gibi bir örnek hayal edelim:

```rust
fn greet(name: String) {
  println!("Hello, {}!", name);
}
```

Bu işlev kendisinin aynı değişkenle defalarca çağrılmasını önlediği gibi, kendisine geçirilen değerin mülkiyetine de ihtiyaç duymaz:

```rust
let name = "Pascal".to_string();
greet(name);
greet(name); // name değişkeni daha önce taşınmış olduğundan derlenmeyecek
```

Bir verinin referansını almak için **`&`** sembolünü kullanılır. Ve bu sembol kullanıldığında, bir değer üzerinden referans beklendiği açıkça anlaşılır.

*Bu tür API’ler farklı nedenlerden dolayı genellikle `&str` şeklinde daha genel amaçlara yönelik tasarlanmakla birlikte şimdilik sadece `&String`’ e ihtiyaç duyulduğundan aşağıdaki şekilde örneklenmiştir.*

```rust
fn greet(name: &String) {
  println!("Hello, {}!", name);
}
```

Bu düzenlemeyle `greet()` işlevi artık bir `string` referansı bekleyecek ve defalarca çağrılabilecektir. 

```rust
let name = "Pascal".to_string();
greet(&name);
greet(&name);
```

Bir değerin referansını bekleyen işlevler, kendilerine aktarılan değerleri sadece **ödünç almış olduklarından** o değerlere hiçbir zaman sahip olmazlar. Bu nedenle referans yoluyla ödünç alınmış değerler kullanıldığında, programın farklı noktalarından aynı verilere ulaşmak da sorun olmaktan çıkar.

```rust
let name = "Pascal".to_string();
let a = &name;
let b = &name;
```

Yukarıdaki örnekte yer alan `a` ve `b` değişkenleri, `name` değişkenini referans yoluyla borç alarak kullandıklarından `name` değişkeni sahip olduğu kaynakların mülkiyetini hiçbir zaman kaybetmez. Böylelikle `a` değişkenine ödünç olarak verilen `name` değeri, `b` değişkeni tarafından da kullanılabilir. 

```rust
let name = "Pascal".to_string();
let a = &name;
let b = a;
```

Bu bilgiler yardımıyla gerekli düzenlemeleri gerçekleştirip `greet()` işlevini çağırmak artık bir soruna yol açmaz.

```rust
fn greet(name: &String) {
  println!("Hello, {}!", name);
}

fn main() {
 let name = "Pascal".to_string();
 let a = &name;
 greet(a);

 let b = a;
 greet(a);
}
```
