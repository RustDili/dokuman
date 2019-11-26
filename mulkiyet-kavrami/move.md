### Move
Stack üzerinde depolanan `integer`, `bool`, `char` gibi ilkel türler haricinde; heap üzerinde depolanan kaynaklara sahip olan her nesne, bir diğerine atama yoluyla ya da bir işleve parametre olarak aktarıldığında o kaynağın mülkiyeti de aktarılmış sayılır. Bu duruma Rust terminolojisinde **move** yani taşıma adı verilir. Ve bir kaynak taşındığında kaynağın bir önceki sahibi artık kullanılamaz hale geleceğinden silinir. Böylelikle halihazırda silinmiş olan bir kaynağın referans verilerek kullanılmasının önüne geçilmiş olunur. 
Bu noktada nesnenin atanma ya da parametre yoluyla işleve geçirilmesi gibi işlemlerde stack üzerinde depolanan türler kopyalanırken heap üzerinde depolanan türler ise taşınır. Bu durum Rust diliyle kod üretimini etkilediği için oldukça önemlidir.

```rust
fn islev(x: i32, y: i32) {
    println!(“İşlevden dönen değerler x: {}, y: {} “, x, y);
}

fn main() {
    // Stack allocated integer değeri
    let x = 5i32;

    // kaynağı taşınmadan x değişkeni y değişkenine kopyalanıyor.
    let y = x;

    islev(x, y);

    // İki değişken de birbirinden bağımsız olarak halen kullanılabilir.
    println!("x değişkeni: {}, ve y değişkeni {}", x, y);
}
```

Yukarıdaki örnek x değişkeninde tutulan ilkel bir tür olan integer değeri diğer programlama dillerine benzer şekilde kopyalanırken aşağıdaki örnekte yer alan a değişkeni heap üzerinde depolanan bir değere sahip olduğu için atama işlemin gerçekleştiğinde mülkiyeti de taşınacak, mülkiyetinde bulunan hafiza kaynağı ise, mülkiyeti devralan b değişkenine aktarılacağından, a değeri silinerek kullanılamaz duruma gelecektir.

```rust
fn main() {
    // a bir heap allocated integer değeri işaret eder.
    let a = Box::new(5i32);
    println!("a: {}", a);
// a değişkenini b'ye taşıyoruz. Bu andan sonra a kullanım dışı olacak. 
// Eğer bu noktadan sonra a'yı kullanmaya çalışırsak derleyici 
// use of moved value hatası üretecek.
    let b = a;    // a değişkeninin sahip olduğu değer için ayrılmış kaynak
// artık b’ye taşındığından bu noktadan sonra a artık kullanılamaz

   println!("b'nin taşıdığı değer: {}", b); // Sorunsuz çalışacak
   println!("a'nın taşıdığı değer: {}", a); // Bu satır hata üretilmesine neden olur
}
```

Aşağıdaki örnek main işlevi içinde farklı yapı türleriyle kullanılmak istenilen bir değişkenin yaşamını mülkiyet aktarımı nedeniyle nerelerde sürdüğünü ve değişkenin hangi noktada kullanılamaz hale gelerek kullandığı hafıza kaynağını sisteme geri iade ettiğini anlatır.

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

Yorum satırları dikkatle okunduğunda akıllara config değişkeninin neden main işlevinin son satırına kadar kullanılamadığı sorusu gelebilir.  Bunun yanıtı config değişkeni değeri için bellekte ayrılan tüm kaynağı ProductService yapısının new işlevine işlev parametresi olarak geçirilmesinde gizlidir.

Mülkiyetin new işlevine geçirilmesiyle artık kaynağın yeni sahibi olan new işlevinin o mülkiyetle ne yapacağı tamamen kendisini ilgilendirir. Mülkiyeti elinde bulunduranın sahip olduğu şeyi imha etme hakkına da sahip olduğunu bildiğimizden, ancak yeni sahibin hafızayı boşaltma kararı verip vermediği bilgisine sahip olmadığımızdan dolayı böyledir.

Mülkiyetin ve üzerindeki tüm tasarrufun başka bir servise aktarıldığı bir bellek alanının başına neyin geldiği bilinmeden başka servislerce de kullanılmasına izin verilmesi korkunç veri kayıplarına neden olabileceğinden Rust derleyici olası bir çalışma zamanını önlemiş olur  
Heap üzerinde depolanan kaynaklara sahip olan değişkenler parametreleri yoluyla işlevlere geçirildiklerinde sadece o işlevin kod blokları arasında yaşarlar. Doğal olarak taşıdığı değer için ayrılmış kaynakları işleve aktarılan değişken de bir daha kullanılamayacağından silinir.

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
