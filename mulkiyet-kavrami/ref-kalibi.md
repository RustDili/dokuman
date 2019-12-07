### Ref Kalıbı
Rust’ ta bir değişkene referans vermenin diğer yolu ise `ref` anahtar kelimesini kullanmaktır.

```rust
fn main() {
    let sayi = 10;

    let ref ref_sayi1 = sayi;
    let ref_sayi2 = &sayi;
}
```

Ampersand **&** sembolü kullanılarak alınan referans ile yukarıdaki örnekte kullanılan referans kalıbı arasında hiçbir fark yoktur. Genellikle `ref` kalıbı yapısal programlamada işleri biraz daha kolaylaştırmak için kullanılır.

```rust
struct Point { x: i32, y: i32 }

fn main() {
    let nokta = Point { x: 0, y: 0 };

    let x_in_kopyasi = {
        // 'x_referansi' değişkeni, 'nokta' adlı struct'ın x değerini tutuyor.
        let Point { x: ref x_referansi, y: _ } = nokta;

        // 'x_referansi' değerini döndürerek x değişkeninin kopyası oluşturulur.
        *x_referansi
    };
}
```

Yukarıdaki örnekte `Point` adlı yapıdan elde edilen nesnenin `x` adlı üye değeri, kapama işlevi yardımıyla `ref` kalıbı kullanılarak  `x_in_kopyasi` değişkenine kopyalanıyor. Bu Kalıp yapılarla çalışırken olduğu gibi, **Tuple**'larla çalışırken öğelerine referans vermek için oldukça kullanışlıdır.

```rust
fn main() {
    let mut mutable_tuple = (5, 3);

    {
        // İkinci elemanını mutable referans olarak almak için tuple'ı parçalıyoruz.
        let (_, ref mut ikinci) = mutable_tuple;
        *ikinci = 2; // İkinci elemanının değerini değiştiriyoruz.
    }

    println!("Tuple: {:?}", mutable_tuple); // Çıktı "Tuple: (5, 2)" olacaktır.
}
```

Bu tür işlemler yapılırken `ref` kalıbı yerine `&` sembolü ile referans vermek, hata yapılmasını kolaylaştırabileceğinden `ref` kalıbını kullanmak oldukça yararlıdır.

Rust’ın bellek yönetim mekanizması, referansları takip ederken değişkenlerin yaşam sürelerini de hesaba kattığından, bazı durumlarda değişkenlerin yaşam sürelerinin açıkça belirtilmesi gerekir.

```rust
struct Product;

struct Config {
    debug_mode: bool
}

struct ProductService<'a> {
    config: &'a Config
}
struct BasketService<'a> {
    config: &'a Config
}

impl<'a> ProductService<'a> {

    fn new (config: &Config) -> ProductService {
        ProductService {
            config: config
        }
    }
}

impl<'a> BasketService<'a> {

    fn new (config: &Config) -> BasketService {
        BasketService {
            config: config
        }
    }
}

fn main() {
    let config = Config { debug_mode: true };
    let product_service = ProductService::new(&config);
    let basket_service = BasketService::new(&config);
}
```

Yukarıdaki örnekte temel olarak `'a` nın yaşamının `ProductService'` e ait olan `Config` referansından daha uzun sürmeyeceği bildirilmektedir. Yapılarda bu tür sınırlamalar derleyici tarafından çıkarsanamadıkları için yaşam sürelerinin bu şekilde açıkça belirtilmesi zorunludur. Yapılarda nesnelerin yaşam alanları ve referanslar konusuna ileride ayrıca değinilecektir.
