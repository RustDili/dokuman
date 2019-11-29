### Move
Stack üzerinde depolanan `integer`, `bool`, `char` gibi ilkel türler haricinde; `heap` üzerinde depolanan kaynaklara sahip olan her nesne; bir diğerine atama yoluyla ya da bir işleve parametre olarak aktarıldığında, o kaynağın mülkiyeti de aktarılmış sayılır. Bu duruma Rust terminolojisinde **`move`** yani taşıma adı verilir. Ve bir kaynak taşındığında, kaynağın bir önceki sahibi artık kullanılamaz hale geleceğinden silinir. Böylelikle halihazırda silinmiş olan bir kaynağın referans verilerek kullanılmasının önüne geçilmiş olunur. 
Bu noktada; nesnenin atama ya da parametre yoluyla işleve geçirilmesi gibi işlemlerde `stack` üzerinde depolanan türler kopyalanırken, `heap` üzerinde depolanan türler ise taşınır. Bu durum Rust'ta kod üretimini etkilediği için oldukça önemlidir.

```rust
fn islev(x: i32, y: i32) {
    println!(“İşlevden dönen değerler x: {}, y: {} “, x, y);
}

fn main() {
    // Stack üzerinde depolanan integer değeri
    let x = 5i32;

    // kaynağı taşınmadan x değişkeni y değişkenine kopyalanıyor.
    let y = x;

    islev(x, y);

    // İki değişken de birbirinden bağımsız olarak halen kullanılabilir.
    println!("x değişkeni: {}, ve y değişkeni {}", x, y);
}
```

Örnekteki `x` değişkeninde tutulan ve ilkel bir tür olan integer değeri, diğer programlama dillerine benzer şekilde kopyalanırken, aşağıdaki kodda yer alan `a` değişkeni `heap` üzerinde depolandığından, atama işlemi gerçekleştiğinde mülkiyeti de taşınacak, mülkiyetinde bulunan hafiza kaynağı ise, mülkiyeti devralan `b` değişkenine aktarılacak ve `a` değeri silinecektir.

```rust
fn main() {
    // a heap'te depolanan integer değeri işaret eder.
    
    let a = Box::new(5i32);
    println!("a: {}", a);

// a değişkeni b'ye taşınıyor ve bu andan sonra a değişkeni kullanımaz olacak.
// Bu noktadan sonra a değişkeni kullanılmak istenirse derleyici 
// `use of moved value` yani `taşınan değer kullanımı` hatası üretecek.
    
    let b = a;

// bu noktada `a` değişkeninin sahip olduğu değer için ayrılmış kaynak
// b değişkenine taşındığından bu noktadan sonra a artık kullanılamaz

   println!("b'nin taşıdığı değer: {}", b); // Sorunsuz çalışacak
   println!("a'nın taşıdığı değer: {}", a); // Bu satır hata üretilmesine neden olur
}
```

Aşağıdaki kodun `main()` işlevinde farklı yapı türleriyle kullanılan değişkenin, mülkiyet aktarımları neticesinde yaşamının nerelerde sürdüğünü ve değişkenin hangi noktada kullanılamaz hale gelerek hafıza kaynağını sisteme iade ettiği açıkça anlaşılır.

```rust
struct Config {……}
struct ProductService {……}
struct BasketService {……}

fn main() {
    let config = Config { debug_mode: true };
    // Bu noktada config `main` işlevine aittir.
    // Bu main işlevinin sonunda hafızanın 
    // serbest bırakılacağı anlamına gelir

    let product_service = ProductService::new(config);
    // Bu noktada config `new` işlevinin mülkiyetine girer
    // Yani main işlevi artık `config` değişkenine sahip olmadığından
    // `config` değişkeninin kullanılmasına artık izin verilmez. 

    let basket_service = BasketService::new(config);
    // config değişkeni `main` işlevinin mülkiyetinde olmadığından 
    // BasketService’e ait olan new işlevinde de kullanılamaz.
}
```

Yorum satırları dikkatle okunduğunda akıllara `config` değişkeninin neden `main()` işlevinin son satırına kadar kullanılamadığı sorusu gelebilir. Bunun yanıtı `config` değişkeni değeri için bellekte ayrılan tüm kaynağın, `ProductService` nesnesinin `new()` metoduna işlev parametresi olarak geçirilmesinde gizlidir: Mülkiyetin `new()` işlevine geçirilmesiyle, artık kaynağın yeni sahibi olan `new()` işlevinin o mülkiyetle ne yapacağı tamamen kendisini ilgilendirir. İster `config` değişkenini kullanıp, başka bir yere transfer eder, isterse yaşamına son verip ona ayrılan kaynakların hafızaya iade edilmesini sağlar. Bu bilginin halihazırda `main()` işlevi içinde bilinemiyor olması bu soruyu cevaplar.
Mülkiyet ve kullanım hakları başka bir servise devredilmiş bellek alanlarının başlarına neyin geldiği bilinmediği müddetçe, başka servisler tarafından kullanılmasına izin verilemesi, telafi edilemez veri kayıplarına neden olabileceğinden, mülkiyet hakları Rust derleyicisi tarafından dikkatlice takip edilerek, olası veri kayıpları ve çalışma zamanı hatalarının önüne geçilmiş olunur.  
 
`Heap` üzerinde depolanan kaynaklara sahip olan değişkenler, parametreleri yoluyla işlevlere geçirildiklerinde sadece o işlevin kod blokları arasında yaşarlar. Doğal olarak, taşıdığı değer için ayrılmış kaynakları işleve aktarılan değişken de bir daha kullanılamayacağından silinir.

```rust
// Aşağıdaki işlev parametre yoluyla kendisine geçirilen a değişkeni için
// ayrılan kaynakların mülkiyetini de alacağından işlevin bitiminde
// c’nin kaynakları da sisteme iade edilecek

fn box_sil(c: Box<i32>) {
    // <-mülkiyet devralan C değişkeni yaşamaya başlar

println!("İşlevin kapsamı boyunca c istenildiği gibi kullanılır: {}", c);

}    // <-C değişkeninin yaşam alanı bu noktada sonlandığından
// c değişkeni silinerek taşıdığı kaynak sisteme iade edilir.

fn main() {
    let a = Box::new(5i32);
    println!("a: {}", a);

    box_sil(a);    // <- a değişkeninin tuttuğu değer için hafızada ayrılan
    
    // kaynağın mülkiyeti işleve devredildiğinden
    // a değişkeni bu noktadan sonra kullanılamaz
    
    println!("a’nın sahip olduğu kaynak taşındığından hata üretilir: {}", a); 
}    // <- Eğer a değişkenin sahip olduğu kaynak  box_sil işlevine 
    // aktarılmamış olsaydı a değişkeni yaşam alanı bu noktada sona erecekti
```
