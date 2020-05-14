# Yapılar
Birbiriyle ilgili özellikleri bileşik veri tipi halinde bir araya getiren, kullanıcı tanımlı özel türlerdir. Yapılar tanımlanırken seçilecek isimler geleneksel olarak `PascalCase` şeklinde tercih edilirler. Rust'ta `struct` anahtar kelimesi kullanılarak tanımlanabilecek üç değişik yapı biçimi vardır.
1. **C benzeri kurallı yapılar**
    - Virgülle ayrılmış bir veya daha fazla *alan:değer* çiftleri
    - Süslü parantez ile sarmalanmış öğe listesi
    - Nesne yönelimli programlama dillerindeki sınıflarla benzerlik
    - İsimlendirilmiş alanlardan oluştuklarından, nokta gösterimi ile erişim
2. **Çokuzlu yapıları**
    - Virgülle ayrılmış bir veya daha fazla eleman
    - Çokuzlu koleksiyonları gibi parantez ile sarmalanmış öğe listesi
    - İsimlendirilmiş çokuzlu şeklinde kullanım kolaylığı
3. **Birim yapıları**
    - Hiç bir üyeye sahip değil
    - Boş bir çokuzluya benzer biçimde **`()`** yeni tür tanımlama
    - Nadiren tercih edilse de, jenerik türlerde oldukça kullanışlı

⭐️ Rust'ın nesne yönelimli programlama yaklaşımında öznitelikler ve metodlar, yapılara ve özelliklere -`traits`- ayrı ayrı yerleştirilir. Yapılar yalnızca öznitelikler içerirken, özelliklerde ise yalnızca metodlar bulunabilir. Bunlar birbirlerine `impls` adını verdiğimiz uygulamalar ile bağlanır.

>💡 Daha karmaşık örneklere, [Uygulamalar ve Özellikler](ikinci-adim/impl-and-traits.md), [Yaşam süreleri](#) ve [Modüller](#) bölümlerinden ulaşabilirsiniz.

## C benzeri kurallı yapılar
En yaygın kullanılan yapı türlerindendir. Parantez içine alınmış listelere, araları virgülle ayrılmış bir veya daha fazla `isim:deger` çiftlerine sahiptirler. Metodları bulunmayan sınıflara benzerler, veri alanları isimlendirilmiş olduğundan üyelerine `nesne.isim` şeklinde erişilebilirler: 

```Rust
struct Yapi {
    kesirli : f32,
    dizge   : String,
    boolean : bool,
}

fn main() {
    let ornek   = Yapi {
        kesirli : 7.05,
        dizge   : String::from("Merhaba!"),
        boolean : false,
    };
    
    print!("{}, {}, {}", ornek.kesirli, ornek.dizge, ornek.boolean);
}
// 7.05, Merhaba!, false
````

C tipi olarak da adlandırılabilen bu yapıların içinde tanımlanan her alanın bir adı ve türü vardır. Bu alanlara `yapi_ornegi.alan_adi` söz dizimi kullanılarak erişilir. Tanımları tür için genel bir şablon gibi olduğundan elemanlarını belirtirken herhangi bir sıraya uyulmasına gerek yoktur: 

```Rust
struct Uye  {
    no      :   u64,
    adi     :   String,
    eposta  :   String,
    yasi    :   u8,
    aktif   :   bool,
}

fn main ()  {
    // Yapinin örneğini almak
    let ali =   Uye {
        no      : 1,
        adi     : String::from("Ahmet"),
        eposta  : String::from("abc@xyz.com"),
        yasi    : 37,
        aktif   : true,
    };
    
    println!("no: {}, eposta: {}", ali.no, ali.eposta);
    // no: 1, eposta: abc@xyz.com

    // destructure edelim
    let Uye {no: no, adi: ad, eposta: ep, yasi: ys, aktif: ak} = ali;
    println!("No: {}, Adi: {}, eposta: {}, Yaşı: {}, Aktif: {}", no ,ad, ep, ys, ak);
    // No: 1, Adi: Ahmet, eposta: abc@xyz.com, Yaşı: 37, Aktif: true
}
````

Yapı örnekleri alanlarına değer atanarak oluşturulur ve bu alanların tuttuğu değerlere nokta gösterimi kullanılarak erişilebilir:

```Rust
struct Renk {
    kirmizi : u8,
    yesil   : u8,
    mavi    : u8,
}

fn main()   {
    // Yapının siyah adlı örneği
    let siyah = Renk {
        kirmizi :   0,
        yesil   :   0,
        mavi    :   0,
    };
    // Nokta gösterimi kullanarak yapı alanlarına erişmek
    println!("Siyah = ({}, {}, {})", siyah.kirmizi, siyah.yesil, siyah.mavi);
    // Siyah = (0, 0, 0)
    
    // destructure edilişi
    let Renk {kirmizi: k, yesil:y, mavi:m} = siyah;
    println!("Siyah = ({}, {}, {})", k, y, m);
    // Siyah = (0, 0, 0)
}
````

Yapılar varsayılan olarak değişmez kabul edilirler. Bir yapıyı değişken hale getirmek için `mut` anahtar kelimesi kullanılır. Ancak `mut` anahtar kelimesinin kullanılması sadece örneğin değişmesine neden olur, yapı alanları düzeyinde değişkenlik sağlamaz:

```Rust
// Yapı örneği
let mut ornek = Renk {kizil: 0, yesil: 0, mavi: 255};
// Sadece örneğin içeriği değişir
ornek.mavi = 238;
println!("Örnek = ({}, {}, {})", ornek.kizil, ornek.yesil, ornek.mavi);  // Örnek = (0, 0, 238)
````

Bir yapı örneğinin üyeleri `{..kopyalanacak_ornek}` şeklinde başka bir örnekten kopyalanabilir. Kopyalama yapılırken yeni üyenin bazı üyelerine değer verilebilir. Bu durumda değer ataması yapılmayan üyeler örneklenen kopyanın üye değerlerini edinirler:

```Rust
let mavi = Renk {mavi:255, ..ornek};
println!("Mavi = ({}, {}, {})", mavi.kizil, mavi.yesil, mavi.mavi);  // Mavi = (0, 0, 255) 
````

Yapı örnekleri `let` ile bağlanarak destructure edelebilirler. Bu yapıldığında üyelere takma isimlerle erişmek mümkün olur:

```Rust
let Renk {kizil: k, yesil: y, mavi: m} = mavi;
println!("Destructure mavi = ({}, {}, {})", k, y, m); // Destructure mavi = (0, 0, 255)
````

İşlevler üzerinden yapı alanlarına erişilerek bir örneği elde edilebilir:

```Rust
fn koyu_mavi_yap() -> Renk {
    Renk {kizil: 25, yesil: 25, mavi:112}
}

let koyu_mavi = koyu_mavi_yap();
println!("Koyu mavi = ({}, {}, {})", koyu_mavi.kizil, koyu_mavi.yesil, koyu_mavi.mavi); // Koyu mavi = (25, 25, 112)
````

işlev yoluyla oluşturulan örnekler de destructure edilebildiğinden örneğin  üyelerine takma isimlerle erişmek mümkün olur:

```Rust
let Renk {kizil: k, yesil: y, mavi: m} = koyu_mavi;
println!("Destructure Koyu mavi = ({}, {}, {})", k, y, m);  // Destructure Koyu mavi = (25, 25, 112)
````

## Çokuzlu yapıları
Normal yapılara benzemekle beraber isimlendirilmiş alanlar yerine `struct Tuple(u32, String);` söz diziminde olduğu gibi üyelerinin türleri bildirilir. Çokuzlular gibi kullanılan bu türün üyelerine 0' dan başlayan index değerleri ile ulaşılır.

⭐️ Çokuzlu yapılarında yalnızca bir öğe bulunduğunda, buna newtype örneği denir ve bu örnek yeni bir tür oluşturmaya yardımcı olur:

```Rust
struct Renk(u8, u8, u8);
struct Mesafe(i32);

fn main() {
    // Bir örnek oluşturma
    let siyah = Renk(0, 0, 0);
    println!("Siyah = {}, {}, {}", siyah.0, siyah.1, siyah.2);  // Siyah = 0, 0, 0
    
    // Newtype pattern ile tür örneğini almak
    let uzaklik = Mesafe(20);
    println!("Uzaklık: {}", uzaklik.0);
    
    // Örneğin `let` ile bağlanarak destructure edilmesi
    let Mesafe(hedef) = uzaklik;
    println!("Hedef: {} mesafede", hedef);
}
````

Kurallı yapılarda olduğu gibi çokuzlu yapılarının da örnekleri `let` ile bağlanarak destructure edelebilirler. Bu tercih edildiğinde üyelerine dizin numarası yerine takma isimler kullanarak erişmek mümkün olur:

```Rust
let Renk(k, y, m) = siyah;
println!("Destructure siyah = {}, {}, {}", k, y, m);    // Destructure siyah = 0, 0, 0
````

## Birim yapıları
Herhangi bir üyeye sahip olmayan bu yapı türü boş bir çokuzluya benzer. Sıfır baytlık boyuta sahip olduklarından genellikle marker olarak veya Jenerik türler oluştururken faydalıdırlar. İçlerinde saklanacak veri bulundurmadıklarından genellikle uygulamalara özellikler eklemek için de tercih edilirler:

```Rust
struct Mesafe;

fn main() {
    let x = Mesafe;
}
````
