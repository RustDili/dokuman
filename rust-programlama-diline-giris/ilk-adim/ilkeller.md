# Basit veri türleri
Rust çok sayıda basit veri türüne erişim sağlar.

### i. Boolean
**bool:** yalnızca `true` ya da `false` olabilecek bir değeri temsil eden mantıksal veri türüdür. 

```Rust
let x = true;         // Tür çıkarsamalı kısa bildirim 
let y: bool = false;  // Açık tür bildirimi
````

Bir tamsayı türüne dönüştürüldüğünde `true` için **1** `false` için **0** atandığı görülür. `!`, `&` , `|` operatörleri aracılığıyla  BitAnd, BitOr, Not vb. gibi çeşitli özelliklerin uygulanmasını sağlar. Uygulamada TRUE, FALSE, 1, 0 gibi olası karşılık değerleri kullanılmaz.

```Rust
fn main() {
   let bugun_hava_yagmurlu = true;
   
   if bugun_hava_yagmurlu {
       println!("Dışarıya çıkıyorsan şemsiye almalısın.");
   } else {
       println!("Kahve içerek güneşlenmek için ne güzel bir gün.");
   }
}
````

### ii. Char
**char:**  `'a'`, `'@'`,  `'~'` gibi her biri 4 byte (unicode desteği nedeniyle 32 bit)'a sığabilen ve tek bir karakteri temsil eden karakterleri tutar ve tek tırnak gösterimiyle temsil edilir.

```Rust
fn main() {
    let a = '😎'; 
    let b = '♫';
    let c = '@';
    println!("a: {}, b: {}, c: {}", a, b, c);
}
````

### iii. i8, i16, i32, i64, i128
8, 16, 32, 64, 128 bit sabit boyutlu ve `(+/-)` işaretli tam sayı değerlerini tutarlar.

| VERİ TÜRÜ | MIN                                      | MAX                                     |
| --------- | ---------------------------------------- | --------------------------------------- |
| i8        | -128                                     | 127                                     |
| i16       | -32768                                   | 32767                                   |
| i32       | -2147483648                              | 2147483647                              |
| i64       | -9223372036854775808                     | 9223372036854775807                     |
| i128      | -170141183460469231731687303715884105728 | 170141183460469231731687303715884105727 |

💡 **Min ve Max değerleri;** min değerler için **-(2ⁿ⁻¹)** ve max değerler için **2ⁿ⁻¹-1** formüllerine dayanmaktadır. Türün alt sınır değerlerini öğrenmek için `min_value()`, üst sınır değerlerini öğrenmek içinse `max_value()` işlevleri kullanılır.

```Rust
fn main() {
    println!("i16 min değeri: {} ve max değeri: {}", i16::min_value(), i16::max_value());
}
````

### iv. u8, u16, u32, u64, u128
8, 16, 32, 64, 128 bit sabit boyutlu ve `(0/+)` işaretli tam sayı değerlerini tutarlar.

| VERİ TÜRÜ | MIN | MAX                                     |
| --------- | --- | --------------------------------------- |
| u8        | 0   | 255                                     |
| u16       | 0   | 65535                                   |
| u32       | 0   | 4294967295                              |
| u64       | 0   | 18446744073709551615                    |
| u128      | 0   | 340282366920938463463374607431768211455 |

💡 **Min ve Max değerleri;** min değerler için **0** ve max değerler için **2ⁿ-1** formüllerine dayanmaktadır. Her tam sayı türünün alt sınır değerlerine `min_value()`, üst sınır değerlerine ise `max_value()` işlevleri aracılığıyla ulaşılır.  

```Rust
fn main() {
    println!("u32 min değeri: {} ve max değeri: {}", u32::min_value(), u32::max_value());
}
````

### v. isize ve usize
İşaretçi boyutunda işaretli ve işaretsiz tam sayı türlerini tutarlar. Programın derlenmekte olduğu bilgisayar mimarisinin sunduğu en yüksek bit değerine eşittirler. Varsayılan olarak bu değerler 32 bit platformlarda **32 bit**, 64 bit platformlarda da **64 bit** kabul edilir.

> 🔎 [Çapraz derleme](https://github.com/rust-lang/rustup.rs#cross-compilation) ve [desteklenen katmanlar](https://forge.rust-lang.org/release/platform-support.html) hakkında daha fazla bilgi edinebilirsiniz

```Rust
fn main() {
    println!("isize max değeri: {} usize max değeri: {}", 
            isize::max_value(), usize::max_value());
}
````

### vi. f32 ve f64
32 ve 64 bit boyutlarında ondalık basamaklı sayıları tutabilen türlerdir. Rust, kayan noktalı sayılar aritmetiğini ifade ederken **IEEE** standartlarına uyar. Bu nedenle `f32` türü, diğer dillerdeki tek duyarlıklı float türüne benzerken, `f64` çift duyarlıklı double türüne benzer.
Hedeflenen donanım çift hassasiyeti desteklemiyor; yahut tek hassasiyetli türün çift hassasiyetli türden daha hızlı olabileceği durumlarda `f32`, diğer durumlarda daima `f64` kullanılması tavsiye edilir. Aksi belirtilmedikçe Rust, tam sayılar için `i32`, kayan noktalı sayılar için `f64` türlerinin kullanılacağını varsayar. Her ne kadar derleme anında türler çıkarsanıyor olsalar da, eğer varsayılan türler haricinde bir tür kullanılmak isteniyorsa, bu türün açıkça bildirilmesi gerekir.

```Rust
fn main() {
    let a = 4.7_f32;
    let b = 5.2_f32;
    let c = -3.3_f32;
    
    println!("a: {}, b: {}, c: {}", a, b, c);
    println!("a: {}, b: {}, c: {}", a.floor(), b.ceil(), c.round());
}
````

### vii. Diziler
Aynı veri türündeki öğelerin sabit boyutlu listesini tutarlar. Köşeli parantez içinde tanımlanırlar ve bellekte bitişik olarak depolanırlar. 
Tür ve uzunluk bilgileri derleme zamanında bilinmesi gerektiğinden `[T; size]` söz dizimiyle kullanılırlar. Köşeli parantez içinde bildirilen birinci bölüm olan `T` dizi elemanlarının türünü, ikinci bölüm olan `size` dizinin büyüklüğünü bildirir.

```Rust
let a = [1, 2, 3];                                // a[0] = 1, a[1] = 2, a[2] = 3 
let mut b = [5, 10, 15];                          // Elemanları değişebilen dizi 
let c: [i32; 0] = [];                             // [Tür tanımı; öğesiz] -> [] / boş bir dizi
let d: [i32; 3] = [1, 2, 3];                      // Türü ve eleman sayısı bildirilmiş dizi 
let e = ["değer"; 3];                             // ["değer", "değer", "değer"]; 

println!("{:?}", a);                              // [1, 2, 3]; 
println!("{:#?}", a);                             // [ // 1, // 2, // 3 // ];

let xs: [i32; 5] = [1, 2, 3, 4, 5];               // 5 elemanlı statik bir dizi
let ys: [i32; 500] = [0; 500];                    // Tüm üyeleri 0 ile ilklenen 500 elemanlı dizi
let ab: [i32; 5] = [-2, 5];                       // Tüm üyeleri -2 ile ilklenen 5 elemanlı dizi

println!("Dizinin ilk elemanı: {:?}", xs[0]);     // Dizilerde index 0 dan başlar. 1 yazdırılır.
println!("Dizinin {} tane üyesi var", ys.len());  // len işlevi dizinin uzunluğunu verir
println!("Dizi elemanları: {:?}", ab);            // [-2, -2, -2, -2, -2];

// use std::mem;
println!("ys dizisi belleğin stack bölgesinde: {} byte yer kaplıyor.", mem::size_of_val(&ys));
````

Diziler varsayılan olarak değişmez kabul edildiklerinden, tanımlandıktan sonra eleman adetleri ve türleri değiştirilemez. Tanımlandıkları sırada aldıkları mut anahtar sözcüğü ile sadece eleman değerleri değiştirebilir.
Eğer eleman adetlerinin otomatik olarak arttırılabildiği bir dizi türü gerekiyorsa bunun için **Vektör**ler tercih edilmelidir. Vektörler türleri aynı olmak kaydıyla istenilen sayıda elemanı kabul ederler.

### viii. Çokuzlular
Aynı ya da farklı veri türlerinden oluşan elemanların sabit büyüklükteki listelerini oluşturmak için kullanılan değer topluluklarıdırlar. Her elemanı `(T1, T2, T3...Tn)` kendi türünün imzalı değeri olduğundan, işlevlerden çok sayıda değer döndürürken oldukça yararlıdır.

```Rust
fn main() {
    let a = (1, 1.5, true, 'a', "Merhaba Dünya!");
    println!("a: {:?}", a);                       // a: (1, 1.5, true, 'a', "Merhaba Dünya!")
    
    let b: (i32, f64) = (10, 3.5);
    println!("b: {:?}", b);                       // b: (10, 3.5)
    
    let (c, d) = b;
    println!("c: {:?}, d: {:?}", c, d);           // c: 10, d: 3.5
    
    let (e, _, f, _, g) = a;
    println!("e: {:?}, f: {}, g: {}", e, f, g);   // e: 1, f: true, g: Merhaba Dünya! _, ilgilenmediğiniz öğeleri temsile der
    
    let h = (0,);
    println!("h: {:?}", h);                       // h: (0,) -> Tek elemanlı Çokuzlu
    
    let i = (b, (20, 50), -3.2);
    println!("i: {:?}", i);                       // i: ((10, 3.5), (20, 50), -3.2)
}
````

Bu türün de tıpkı diziler gibi değiştirilemeyeceği varsayılır. `mut` anahtar kelimesiyle tanımlanmaları sadece öğelerinin değiştirilebilmesini sağlar. Eleman sayılarının değiştirilmesine izin verilmez.

### ix. Dilimler
Dizilere benzer ancak boyutları derleme zamanında belli değildir. Dilim `&[T];` söz dizimiyle ifade edilebilen ve iki parçadan oluşan bir nesne olarak düşünülmelidir. İfadenin ilk parçası erişilen verinin göstergesi olurken, ikinci parçası elde edilecek olan dilimin uzunluğunu gösterir.

```Rust
fn main() {
    let a: [i32; 4] = [1, 2, 3, 4]; // Referans verilecek ana dizi 
    
    let b: &[i32] = &a;
    println!("b: {:?}", b);            // Bütün dizi dilimlenir.

    let c = &a[0..4]; 
    println!("c: {:?}", c);            // index 0 dan başlayarak ilk 4 eleman

    let d = &a[..];
    println!("d: {:?}", d);            // Bütün dizi dilimlenir.
    
    let e = &a[1..3];
    println!("e: {:?}", e);            // [2, 3]

    let f = &a[1..];
    println!("f: {:?}", f);            // [2, 3, 4]
    
    let g = &a[..3];
    println!("g: {:?}", g);            // [1, 2, 3]
}
````

Diziler dilimler tarafından otomatik olarak ödünç alınırlar.

```Rust
fn dilimle(dilim: &[i32]) {
    println!("İlk elemanı {} olan {} elemanlı dilim", dilim[0], dilim.len())
}

fn main() {
    let dil: [i32; 5] = [1, 2, 3, 4, 5];
    dilimle(&dil);
    
    let dilim: &[i32] = &dil;
    dilimle(&dilim);
    
    dilimle(&dil[1..3]);
}
````

### x. Str
Unicode dizgilerinin bir parçasını UTF-8 formatında tutan boyutu olmayan dilimlerdir. Dizgi dilimleri olarak adlandırılırlar. Genellikle bir dizgiyi ödünç almak için `&str` şeklinde kullanılırlar.

```Rust
fn main() {
    let dizgi = String::from("Merhaba Dünya!");
    let merhaba = &dizgi[0..7];
    println!("Dizginin ilk parçası: {}", merhaba);
    
    let dunya = &dizgi[8..15];
    println!("Dizginin son parçası: {}", dunya);
}
````
