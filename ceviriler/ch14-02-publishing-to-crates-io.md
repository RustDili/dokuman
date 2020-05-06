# Bir Sandığı Crates.io Üzerinde Yayınlamak
Önceki bölümlerde gerçekleştirdiğimiz bazı örnek projeler çalışabilmek için [crates.io](https://crates.io/)'daki bazı paketlere bağımlı olduklarından onları projelerimize dahil etmeyi öğrenmiştik. Halbuki sizler de kendi paketlerinizi yayınlayarak kodlarınızı başkalarıyla paylaşıma açabilirsiniz. [Crates.io](https://crates.io/)'daki sandık kayıt defteri, paketlerinizin kaynak kodunu dağıtacağından, öncelikle projenizin açık kaynak kodunu barındırmak zorundadır.

Rust ve Cargo, yayınlanan paketleri başkalarının da kolaylıkla bularak ve kullanabilmesini sağlayan özelliklere sahiptir. Bu özelliklerin bazılarından az sonra bahsedecekve ardından bir paketin nasıl yayınlanacağını anlatacağız.

## Kullanışlı Dökümantasyon Yorumları Oluşturmak
Paketlerinizi doğru bir şekilde belgelemek, diğer kullanıcıların bunları nasıl ve ne zaman kullanacaklarını bilmelerine yardımcı olacağından belge yazmak için harcadığınız zamana değecektir. Bölüm 3'te iki eğik çizgi `//` kullanarak Rust kodlarını nasıl yorumlayacağımızı tartışmıştık. Normal yorumların yanısıra Rust ayrıca dökümantasyon yorumları olarak bilinen ve kodlar içerisind eyapılan açıklamaları HTML belgelerine çevirmeye yarayan belirli bir yorumlama şekline de sahiptir. Bu HTML belgesi, sandığınızın nasıl uygulandığını anlatmaktan ziyade, sandığınızın nasıl kullanılacağını öğrenmek isteyen programcılara yol gösteren genel API öğelerinin Dokümantasyon yorumlarını gösterir. 

Belge yorumları iki yerine `///` üç eğik çizgi kullanır ve metni biçimlendirmek için Markdown gösterimini destekler. Doküman yorumlarını belgeledikleri öğeden hemen önce yerleştirin. Örnek 14-1, `my_crate` adlı sandıkta yer alan `add_one` işlevi için belge yorumlarını göstermektedir:

Dosya: src/lib.rs
```Rust
/// Kendisine iletilen sayıya bir ekler
///
/// # Örnekler
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
````
Örnek 14-1: Bir işlevin belgelenmesi

Bu örnekte, `add_one` işlevinin ne yaptığını açıklıyoruz, Örnekler başlığıyla bir bölüm başlatarak `add_one` işlevinin nasıl kullanılacağını gösteren kodları ekliyoruz. Ardından `cargo doc` komutunu çalıştırarak bu belge yorumlarından HTML belgesi oluşturabiliriz. Bu komut `rustdoc` araç setini çalıştırarak oluşturulan HTML belgelerini `target/doc` dizinine yerleştir.

Daha da kolayı `cargo doc --open` komutu hem mevcut sandığınız ve onun tüm bağımlılıklarının HTML belgelerini oluşturacak hem de oluşturduğu belgelerin web tarayıcınızda açılmasını sağlayacaktır. Şimdi `add_one` işlevine giderek, Örnek 14-1'de gösterilen belge yorumlarının metne nasıl dönüştürüldüğünü izleyebilirsiniz: 

![resim trpl14-01](https://github.com/rust-lang-tr/dokuman/blob/master/ceviriler/trpl14-01.png)
Şekil 14-1: `add_one` işlevinin HTML belgeleri

## Yaygın Olarak Kullanılan Bölümler
HTML belgesinde “Örnekler” şeklinde bir bölüm başlığı oluşturabilmek için Örnek 14-1'de `# Örnek` şeklinde işaretleme kullandık. Aşağıda ise, pek çok sandık yazarının belgelerinde yaygın olarak kullandıkları bazı bölümler verilmiştir:
  * **Panikler:** Belgelenen işlevin panik yapabileceği senaryolar. İşlevin çağrıldığı programlarda panik oluşması istenmiyorsa, kullanıcıların işlevi çağırmadıklarından emin olunmalıdır. 
  * **Hatalar** İşlev bir `Result` döndürürse, oluşabilecek hataların türleri ve bu hataların döndürülmesine neden olabilecek koşulların tanımlanması, işlevi çağıran tarafların farklı türdeki hataları farklı şekillerde işlemek için kod yazabilmeleri için yardımcı olabilir.
  * **Güvenlik** İşlevin çağrılması güvenli değilse (Bölüm 19'da güvensizliği tartışacağız), işlevin neden güvensiz olduğunu açıklayan ve işlevi çağıran tarafların desteklemesini beklediği sabitleri kapsayan bir bölüm olmalıdır.

Çoğu belgelendirme çalışmasında bu bölümlerin her birinin yorumlanmasına ihtiyaç duyulmaz, bununla birlikte bu bölümler size kodlarınızı çağıran tarafların bilmek isteyeceği yönlerini hatırlatmak için tavsiye edilen bir kontrol listesidir.

## Belgelendirme Yorumlarını Test Olarak Değerlendirmek 
Belge yorumlarınıza örnek kod blokları eklemek, kütüphanenizin nasıl kullanılacağını göstermenize yardımcı olabilir ve bunun ek bir avantajı da `cargo test` komutunu çalıştırarak dokümanlarınızdaki kod örneklerinin test edilmesi sağlanır! Hiçbir şey test edilebilecek örnekler içeren belgelerden daha iyi olamaz. Ancak, belgelendirme yapıldıktan sonra gerçekleşen kod değişiklikleri yüzünden işe yaramayan örneklerden daha kötü bir şey de yoktur. Örnek 14-1'deki `add_one` işlevi için gerçekleştirdiğimiz belgelendirmeye `cargo test` komutu uygularsak, aşağıdaki gibi bir test sonucu göreceğiz.

```bash
   Doc-tests my_crate

running 1 test
test src/lib.rs - add_one (line 5) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
````
İşlev veya örnekteki `assert_eq!` değerini değiştirdiğimizde örnek panikler. Bu durumdayken `cargo test` komutunu yeniden  çalıştırılırsa belge testinde örnek ve kodun birbiriyle senkronize olmadığının tespit edildiğini gözlemleriz.

## İçerilen Öğelerin Yorumlanması
Bir başka belgelendirme biçimi olan `//!` ise yorum satırlarının hemen altına eklenenen öğeleri belgelemek yerine, yorumun ait olduğu, yani içerildiği öğeyi belgelendirirmek için kullanılır. Genellikle bu tarz yorum etiketlerini sandık veya modülün tamamını bir bütün olarak belgelemek amacıyla kök dosyasının içinde (kural gereği src/lib.src) ya da bir modül içerisinde kullanırız. 

Örneğin `add_one` işlevini içeren `my_crate` adlı sandığın amacını açıklayan belgeler eklemek istiyorsak src/lib.rs dosyasının en başına örnek 14-2' de gösterileceği gibi  `//!` işaretini belge yorumu olarak ekleyebiliriz.

Dosya: src/lib.rs
```Rust
//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
// --snip--
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
````
Örnek 14-2: Bir bütün olarak `my_crate` sandığı belgeleri

Dikkat ederseniz `//!` işaretiyle başlayan son satırdan sonra herhangi bir kod bulunmuyor. Bunun nedeni yorum satırını `///` yerine `//!` işaretiyle başlattığımız için satırın bitiminden sonra gelen satırlardaki öğeler yerine işaretin bulunduğu satırdaki öğeyi belgelenecek olmasıdır. O nedenle, bu yorumu içeren öğe, sandık kökü olan src/lib.rs dosyası olacağından bu yorum satırları tüm sandık için yapılan açıklamaları içerir. 

Artık `cargo doc --open` komutu çalıştırıldığında işaretlenmiş bu satırlar aşağıda yer alan Şekil 14-2'de gösterildiği gibi `my_crate` belgesinin ön sayfasında, sandıktaki genel öğeler listesinin üstünde görüntülenir: 

![Şekil 14-2](https://doc.rust-lang.org/book/img/trpl14-02.png)
Şekil 14-2: Yorumlar dahil olmak üzere `my_crate` sandığını bir bütün olarak tanımlayan belgeler

Öğeler içindeki belge yorumları, özellikle sandık ve modülleri tanımlamak için kullanışlıdır. Bu yorumları, paketlerinizi kullanacak olan kişilerin paket düzenini anlamalarına yardımcı olmak ve paket kapsamının genel amacını açıklamak için kullanın.

## [pub use](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html#exporting-a-convenient-public-api-with-pub-use) Ön Ekiyle Bir API'yi Genel Kullanıma Uyarlamak
Bölüm 7'de, `mod` anahtar kelimesini kullanarak kodumuzu modüller halinde nasıl düzenleyeceğinizi, `pub` anahtar sözcüğüyle öğelerin nasıl genelleştirileceğini ve `use` anahtar kelimesiyle de öğelerin kapsama nasıl dahil edileceğini ele almıştık. Ancak, bir sandık geliştirirken sizin için anlamlı olan yapı, kullanıcılarınız için çok uygun olmayabilir. Yapılarınızı çok katmanlı hiyerarşik bir yapıda düzenlemek isteyebilirsiniz, ancak daha sonra bu hiyerarşinin derinlerinde tanımlamış bulunduğunuz herhangi bir türü kullanmak isteyen kişiler, bu türe erişmekte sorun yaşayabilirler. Ayrıca bu türe `use my_crate::UsefulType;` şeklinde bir söz dizimiyle ulaşmak yerine `my_crate::some_module::another_module::UsefulType;` şeklinde bir söz dizimi kullanmak oldukça rahatsız edici olabilir. 

Bir sandık yayınlarken herkese açık olarak tasarlanmış olan API'nizin yapısı oldukça önemlidir. Sandığınızı kullanan kişiler bu yapıya sizin kadar aşina olmadıklarından, sandığınız büyüyüp karmaşık bir modül hiyerarşisine dönüştüğünde, kullanmak istedikleri API parçalarına ulaşmakta zorluk çekebilirler. İyi haber şu ki, eğer yapınız başkalarının kütüphanelerinde kullanılmaya uygun değilse, API Hiyararşisini ve belki tasarımını baştan başa yeniden düzenlemek yerine, `pub use` anahtar kelimelerini kullanarak, tüm öeğeleriyle bu yapının genel kullanıma uygun bir sürümünü yeniden ihraç edebilirsiniz. Yeniden ihraç herkese açık bir öğeyi bir konumda alır ve başka bir yerde, başka bir konumda tanımlanmış gibi herkese açık hale getirir.

Örneğin, sanatsal kavramları modellemek için `art` adında bir kütüphane tasarladığımızı varsayalım. Örnek 14-3'te de görüleceği gibi; bu kütüphanenin içinde `PrimaryColor` ve `SecondaryColor` adında iki enum içeren bir `kinds` modülü ve `mix` adındaki işlevi içeren `utils` modülü bulunmaktadır:

Dosya: src/lib.rs
```Rust
//! # Art
//!
//! A library for modeling artistic concepts.

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --snip--
    }
}
````
Örnek 14-3: `kinds` ve `utils` modüllerinde düzenlenmiş öğeleri içeren bir `art` kütüphanesi
![resim 14-3](https://doc.rust-lang.org/book/img/trpl14-03.png)
Şekil 14-3, `cargo doc` tarafından üretilen bu sandık için dokümantasyonun ön sayfasının nasıl görüneceğini göstermektedir:

Ön sayfada `PrimaryColor` ve `SecondaryColor` türleri ve mix() işlevinin listelenmediğine dikkat edin. Onları görmek için `kind` ve `utils` bağlantılarına tıklamak gereklidir. Bu kütüphaneye bağımlı başka bir sandık, halihazırda tanımlanmış olan `art` modül yapısına ait öğeleri kendi kapsamına alabilmek için `use` ifadesini kullanmak zorunda kalacaktır. Örnek 14-4, `art` sandığındaki `PrimaryColor` ve `mix` öğelerini kullanan başka bir sandık örneğini gösterir:

Dosya: src/main.rs
```Rust
use art::kinds::PrimaryColor;
use art::utils::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
````
Örnek 14-4: İç yapısının dışa aktarılmasıyla öğeleri kullanılabilir hale gelen `art` sandığı 

Örnek 14-4'te yer alan `art` sandığını kullanan kodun programcısı, `PrimaryColor` türünün `kinds` modülünde ve `mix()` işlevinin de  `utils` modülünde olduğunu bulabilmelidir. Sandığın modül yapısı, `art` sandığını geliştiren programcılardan çok, bu sandığı kullanan  programcılar için önemlidir. Sandığın parçalarını `kinds` ve `art` modülleri olarak düzenleyen iç yapı, bu sandığın nasıl kullanılacağını öğrenmek isteyenler için herhangi bir yararlı bilgiler içermediği gibi `art` sandığı modül yapısının yarattığı karmaşa, use ifadelerinde modül adlarını belirtmek isteyen kullanıcıların nereye bakacaklarını bilememelerine neden olacağından kullanışsız bir yapıdır. 

Herkesin kullanımına açık olan API'den iç düzenlemeyi kaldırıp, pub use ifadelerini kullanarak öğeleri en üst düzeyde yeniden dışa aktarabilmek için Örnek 14-3'te yer alan `art` sandığının kodlarını, Örnek 14-5'te görğleceği şekilde yeniden düzenleyebiliriz. 

Dosya: src/lib.rs
```Rust
//! # Art
//!
//! A library for modeling artistic concepts.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    // --snip--
}

pub mod utils {
    // --snip--
}
````
Örnek 14-5: Öğeleri yeniden dışa aktarmak için pub use ifadeleri eklemek

`cargo doc` komutunun bu sandık için oluşturduğu API belgeleri, Şekil 14-4'te görebileceğiniz gibi ön sayfada yeniden dışa aktarımları listeleyerek bağlayacak ve `PrimaryColor` ve `SecondaryColor` türlerini ve `mix` işlevini bulmayı kolaylaştıracaktır.
![Şekil 14-4](https://doc.rust-lang.org/book/img/trpl14-04.png)
Şekil 14-4: Yenilenen dışa aktarımı listeleyen `art` belgelendirmesinin ön sayfası

Artk `art` sandığını kullanacak olan programcılar, ister Örnek 14-5 ve 14-6'da yenilenen daha uygun yapıyı, isterlerse hâlâ kullanılmaya uygun durumda bulunan Örnek 14-4'te ve 14-3'teki iç yapıyı inceleyip kodlarına dahil edebilirler.

Dosya src/main.rs
```Rust
use art::mix;
use art::PrimaryColor;

fn main() {
    // --snip--
}
````
Örnek 16-4: Yeniden dışa aktarılan `art` sandığı öğelerini kullanan bir program

Çok sayıda iç içe modülün olduğu durumlarda, türleri en üst düzeyde yeniden ihraç etmek pub use, sandığı kullanan kişilerin deneyiminde önemli bir fark yaratabilir.  Kullanışlı bir genel amaçlı API oluşturmak bir sanat olduğundan kullanılarınıza en uygun olabilecek tasarım için defalarca tekrar yapmanız gerekebilir. ancak `pub use` kullanımı paket iç yapınızı kullanıcılara sunulduğu halinden ayırabildiği için esneklik sağlar. sandık iç yapılarının genel kullanıma açılmış hallerinden farklı olup olmadığını anlamak için çalışmalarına dahil ettiğiniz başka sandıkları incelemeniz yararlı olacaktır. 

## Bir Crates.io Hesabı Oluşturma
Herhangi bir sandığı yayınlayabilmeniz için öncelikle [crates.io](https://crates.io/) üzerinde bir hesap oluşturmanız ve bir API anahtarı almanız gereklidir. Bunun için [crates.io](https://crates.io/) adresini ziyaret ederek GitHub hesabınız ile giriş yapın. (Her ne kadar gelecekte siteye başka yöntemlerle üye olunacağı planlanmış olsa da şu an için yalnızca GitHub hesaplarımız ile giriş kabul edilmektedir.) Sisteme girişinizi yaptıktan sonra, [https://crates.io/me/](https://crates.io/me/) adresindeki hesap ayarlarınızı ziyaret ederek ve API anahtarınızı alın. Ardından, aşağıdaki gibi API anahtarınızla `cargo login` komutunu çalıştırın:

```bash
$ cargo login abcdefghijklmnopqrstuvwxyz012345
````

Bu komut Cargo'ya API anahtarınızı bildirecek ve onu yerel olarak `~/.cargo/credentials` içinde depolayacaktır. Anahtarınızı size özel olduğunu, gizli kalması ve kimseyle paylaşılmaması gerektiğini unutmayın. Eğer anahtarınızı herhangi bir sebeple birileriyle  paylaşmışsanız, geçerli olanı deerhal iptal ederek yeni bir anahtar oluşturmalısınız.

## Yeni Oluşturulmuş Sandığa Meta Veri Eklemek
