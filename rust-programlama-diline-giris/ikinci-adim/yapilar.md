# Yapılar
Birbiriyle ilgili özellikleri bileşik veri tipi halinde bir araya getiren, kullanıcı tanımlı özel türlerdir. Yapılar tanımlanırken seçilecek isimler geleneksel olarak `CamelCase` şeklinde tercih edilirler. Rust'ta `struct` anahtar kelimesi kullanılarak tanımlanabilecek üç değişik yapı biçimi vardır.
- **Kurallı yapılar**
- **Çokuzlu yapıları**
- **Birim yapıları**

### Kurallı yapılar
En yaygın kullanılan yapı türlerindendir. Parantez içine alınmış listelere, araları virgülle ayrılmış bir veya daha fazla `isim:deger` çiftlerine sahiptirler. Metodları bulunmayan sınıflara benzerler, veri alanları isimlendirilmiş olduğundan üyelerine `nesne.isim` şeklinde erişilebilirler. 

```Rust
struct Yapı {
    alan1: f32,
    alan2: String,
    pub alan3: bool
}
````
C tipi olarak da adlandırılabilen bu yapıların içinde tanımlanan her alanın bir adı ve türü vardır. Bu alanlara `ornek_yapi.alan1` söz dizimi kullanılarak erişilir. Tanımları tür için genel bir şablon gibi olduğundan elemanlarını belirtirken herhangi bir sıraya uyulmasına gerek yoktur. 
```Rust
struct Uye {
    adi: String,
    eposta: String,
    yasi: u64,
    aktif: bool,
}

fn main () {
    let uye1 = Uye {
        eposta: String::from("kullanici@github.com"),
        adi: String::from("rust-lang-tr"),
        aktif: true,
        yasi: 33
    };
    println!("Üye adı: {},", uye1.adi)
}
````
Yapı örnekleri alanlarına değer atanarak oluşturulur ve bu alanların tuttuğu değerlere nokta gösterimi kullanılarak erişilebilir.
```Rust
struct Renk {
    kizil: u8,
    yesil: u8,
    mavi: u8,
}

fn main() {
    let siyah = Renk {kizil: 0, yesil: 0, mavi: 0};
    println!("Siyah = ({}, {}, {})", siyah.kizil, siyah.yesil, siyah.mavi);  // Siyah = (0, 0, 0)
}
````
Yapılar varsayılan olarak değişmez kabul edilirler. Bir yapıyı değişken hale getirmek için mut anahtar kelimesi kullanılır. Ancak mut anahtar kelimesinin kullanılması sadece örneğin değişmesine neden olur, yapının alanları düzeyinde değiştirilebilirliği sağlamaz.
```Rust
let mut ornek = Renk {kizil: 0, yesil: 0, mavi: 255};
ornek.mavi = 238;
println!("Örnek = ({}, {}, {})", ornek.kizil, ornek.yesil, ornek.mavi);  // Örnek = (0, 0, 238)
````

Bir yapı nesnesinin üyeleri `{..kopyalanacak_ornek}` şeklinde başka bir örnekten kopyalanabilir. Kopyalama yapılırken yeni üyenin bazı üyelerine değer verilebilir. Bu durumda değer ataması yapılmamış kalan üyeler örneklenen kopyanın üye değerlerini edinirler.
```Rust
let mavi = Renk {mavi:255, ..ornek};
println!("Mavi = ({}, {}, {})", mavi.kizil, mavi.yesil, mavi.mavi);  // Mavi = (0, 0, 255) 
````
Yapı örnekleri let ile bağlanarak destructure edelebilirler. Bu yapıldığında üyelere takma isimlerle erişmek mümkün olur.
```Rust
let Renk {kizil: k, yesil: y, mavi: m} = mavi;
println!("Destructure mavi = ({}, {}, {})", k, y, m); // Destructure mavi = (0, 0, 255)
````
İşlevler üzerinden yapı alanlarına erişilerek bir örneği elde edilebilir.
```Rust
fn koyu_mavi_yap() -> Renk {
    Renk {kizil: 25, yesil: 25, mavi:112}
}

let koyu_mavi = koyu_mavi_yap();
println!("Koyu mavi = ({}, {}, {})", koyu_mavi.kizil, koyu_mavi.yesil, koyu_mavi.mavi); // Koyu mavi = (25, 25, 112)
````
işlev yoluyla oluşturulan örnekler de destructure edilebildiğinden örneğin  üyelerine takma isimlerle erişmek mümkün olur.
```Rust
let Renk {kizil: k, yesil: y, mavi: m} = koyu_mavi;
println!("Destructure Koyu mavi = ({}, {}, {})", k, y, m);  // Destructure Koyu mavi = (25, 25, 112)
````

### Çokuzlu yapıları
Normal yapılara benzemekle beraber isimlendirilmiş alanlar yerine `struct Tuple(u32, String);` söz diziminde olduğu gibi üyelerinin türleri bildirilir. Çokuzlular gibi kullanılan bu türün üyelerine 0'dan başlayan index değerleri ile ulaşılır. 
```Rust
struct Renk(u8, u8, u8);

fn main() {
    let siyah = Renk(0, 0, 0);
    println!("Siyah = {}, {}, {}", siyah.0, siyah.1, siyah.2);  // Siyah = 0, 0, 0
}
````
Kurallı yapılarda olduğu gibi çokuzlu yapılarının da örnekleri let ile bağlanarak destructure edelebilirler. Bu tercih edildiğinde üyelerine index numarası yerine takma isimler kullanarak  erişmek mümkün olur.

```Rust
let Renk(k, y, m) = siyah;
println!("Destructure siyah = {}, {}, {}", k, y, m);    // Destructure siyah = 0, 0, 0
````

### Birim yapıları
Herhangi bir üyeye sahip olmayan bu yapı türü boş bir çokuzluya benzer. Sıfır baytlık boyuta sahip olduklarından genellikle marker olarak veya Jenerik türler oluştururken faydalıdırlar. İçlerinde veri bulundurmadıklarından uygulamalara özellikler eklemek için de kullanışlıdırlar.
```Rust
struct Mesafe;

fn main() {
    let x = Mesafe;
}
````
