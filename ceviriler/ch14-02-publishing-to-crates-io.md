## Bir Sandığı Crates.io Üzerinde Yayınlamak

Önceki bölümlerde gerçekleştirdiğimiz bazı örnek projeler, çalışabilmek için [crates.io](https://crates.io/)'daki bazı paketlere bağımlı olduklarından, bu paketleri projelerimize dahil etmeyi öğrenmiştik. Halbuki sizler de kendi paketlerinizi yayınlayarak kodlarınızı başkalarıyla paylaşabilirsiniz. [Crates.io](https://crates.io/) sitesinde bulunan sandık kayıt defteri, paketlerinizin kaynak kodunu dağıtacağından, öncelikle projenizin açık kaynak kodunu barındırmak zorundadır.

Rust ve Cargo, yayınlanan paketleri başka geliştiricilerin kolaylıkla bulup kullanabilmelerini sağlayan özelliklere sahiptir. Az sonra bu özelliklerin bazılarından bahsedecek ve ardından bir paketin nasıl yayınlanacağını anlatacağız.

### Kullanışlı Belgeleme Yorumları Oluşturmak

Paketlerinizin doğru biçimde belgelenmesi, bu paketlerin başka kullanıcılar tarafından nasıl ve ne zaman kullanılabileceğine ışık tutacağından, belgeleme sürecine zaman ayırmak önemlidir. Bölüm 3'te, Rust kodlarını iki eğik çizgi `//` kullanarak nasıl yorumlayacağımızı tartışmıştık. Bu normal yorumların yanısıra Rust, *belgeleme yorumları* olarak bilinen ve kod içinde yapılan açıklamaları, HTML belgelerine çevirmeye yarayan özel bir yorumlama biçimine sahiptir. Bu HTML belgeleri, sandığınızın nasıl *uygulandığını* anlatmaktan ziyade, nasıl *kullanılacağını* öğrenmek isteyen programcılara yol gösteren genel API öğelerinin belgelenmiş içeriğinden oluşur.

Belgeledikleri öğeden hemen önce yerleştirilen ve iki yerine `///` üç eğik çizgi ile ifade edilen belgeleme yorumları, metni biçimlendirmek için Markdown gösterimini destekler. Örnek 14-1, `sandigim` adlı sandıkta yer alan `bir_ekle` işlevi için belgeleme yorumlarını göstermektedir:

Dosya: src/lib.rs
```Rust

/// Kendisine iletilen sayıya bir ekler
///
/// # Örnekler
///
/// ```
/// let deger = 5;
/// let yanit = sandigim::bir_ekle(deger);
///
/// assert_eq!(6, yanit);
/// ```

pub fn bir_ekle(x: i32) -> i32 {
    x + 1
}

````
Örnek 14-1: Bir işlevin belgelenmesi

Örnekteki `bir_ekle` işlevinin görevini anlatıp, `Örnekler` etiketli bir bölüm başlatarak `bir_ekle` işlevinin nasıl kullanılacağını gösteren kodlarla başlıyoruz. Bu işlemleri tamamladıktan sonra `cargo doc` komutunu çalıştırarak bu yorum satırlarının işlenmesiyle oluşturulan bir HTML belgesine sahip oluruz. Bu komutla oluşturulan HTML belgeleri `rustdoc` araç seti çalıştırılarak `target/doc` dizinine yerleştirilecektir.

Biraz daha rahatlık sağlayan `cargo doc --open` komutu ise, hem sandığınıza ait tüm bağımlılıkların HTML belgelerini oluşturacak, hem de oluşturduğu belgeleri web tarayıcınızda açarak kullanımınıza sunacaktır. Şimdi `bir_ekle` işlevine giderek, Resim 14-1'de gösterilen belgeleme yorumlarının metne nasıl dönüştürüldüğünü inceleyebilirsiniz:

![resim trpl14-01](https://github.com/RustDili/dokuman/blob/master/ceviriler/img/trpl14-01.png)
Resim 14-1: `bir_ekle` işlevinin HTML belgeleri

#### Yaygın Olarak Kullanılan Bölümler

HTML belgesinde `# Örnekler` başlıklı bir bölüm oluşturabilmek için Örnek 14-1'de `# Örnek` şeklinde bir Markdown başlığı sözdizimi kullandık. Pek çok sandık yazarının belgelerinde yaygın olarak kullandığı bazı bölümler ise aşağıda sıralanmaktadır:
 
* **Panikler:** Belgelenen işlevin panik üretebileceği senaryolar. İşlevin çağrıldığı programlarda panik üretmesi istenmiyorsa, kullanıcıların bu senaryoların gerçekleşebileceği durumlarda işlevi çağırmadıklarından emin olunmalıdır. 
* **Hatalar:** İşlev bir `Result` türü döndürdüğünde, oluşması muhtemel hata çeşitlerinin ve bu hataların döndürülme neden ve koşullarının tanımlanması, işlevi çağıran tarafların farklı türden hataları farklı şekillerde işlemelerini sağlayacak şekilde kod üretmelerine yardımcı olabilir.
* **Güvenlik:** Eğer işlev çağrısı `unsafe` yani emniyetsiz bir çağrı ise (Rust'ın emniyetsiz kullanım seçeneğini Bölüm 19'da tartışacağız), işlevin güvensiz olma nedenlerini açıklayan ve çağıran tarafların desteklemesi gereken değişmezleri kapsayan bir bölüm olmalıdır.

Çoğu belgelendirme çalışmasında bu bölümlerin her birinin yorumlanmasına ihtiyaç duyulmaz. Bununla birlikte bu bölümler kodlarınızı çağıran tarafların bilmek isteyeceği yönleri hatırlamanız amacıyla tavsiye edilen bir kontrol listesidir.

#### Test Amaçlı Belgeleme Yorumları

Belgeleme yorumlarınıza örnek kod blokları eklemek, kütüphanenizin nasıl kullanılacağını göstermenize yardımcı olabileceği gibi ek bir avantaj olarak `cargo test` komutu çalıştırıldığında kod örneklerinizin test edilmesine olanak sağlar. Hiçbir şey test edilebilecek örnekler içeren belgelerden daha iyi olamaz. Ancak, belgeleme sonrası gerçekleşen kod değişiklikleri yüzünden işe yaramayan örneklerden daha kötü bir şey de yoktur. Örnek 14-1'deki `bir_ekle` işlevi için oluşturduğumuz belgeleme koduna `cargo test` komutunu uyguladığımızda, aşağıdakine benzer bir test sonucu göreceğiz:

```bash

   Doc-tests sandigim

running 1 test
test src/lib.rs - bir_ekle (line 5) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

````

Bu aşamada işlev ya da örnek değiştirilecek olursa, örnekteki `assert_eq!` ifadesi panik üreteceğinden `cargo test` komutu tekrar çalıştırıldığında, test sürecinin örnek ve kod bölümlerinin uyumsuzluğunu fark ettiğini gözlemleyeceğiz.

#### İçerilen Öğelerin Yorumlanması

Bir başka belgeleme biçimi olan `//!` ise, yorum satırlarının hemen altına eklenenen öğeleri belgelemek yerine, yorumun ait olduğu, yani içerildiği öğeyi belgelemek için kullanılır. Bu tarz yorum satırlarını genellikle sandık veya modülün tamamını bir bütün olarak belgelemek amacıyla, kök dosyasının içinde (kural gereği src/lib.src) ya da bir modül içerisinde kullanırız. 

Örnek 14-2'de gösterildiği gibi, eğer daha önce oluşturduğumuz ve `bir_ekle` işlevini içeren `sandigim` için, bu sandığın amacını açıklayan belgeler eklemek istiyorsak, bunları *src/lib.rs* dosyasının en başına `//!` işaretini kullanarak eklememiz gerekir.

Dosya: src/lib.rs
```Rust

//! # Sandigim
//!
//! `Sandigim`, bazı hesaplamaların daha kolay yapılmasını
//! sağlayan araçlar koleksiyonudur.

/// Kendisine iletilen rakama 1 ekler
// --snip--

````

Örnek 14-2: Bir bütün olarak `Sandigim` belgeleri

`//!` işaretiyle başlayan son satırın altında herhangi bir kod satırının bulunmadığına ve bir satırın boş bırakılmış olduğuna dikkat edin! Bunun sebebi, içerilen belge yorumlarını `///` yerine, `//!` işaretiyle başlattığımızdan sonraki satırlarda bulunan öğeler yerine, işaretin bulunduğu satırdaki öğelerin belgelenecek olmasıdır. Bu durumda bu yorumu içeren öğe, sandık kökümüz olan *src/lib.rs* dosyası olacağından, bu yorumlar da sandığın tamamı için yapılan açıklamaları içerecektir.

Eğer `cargo doc --open` komutunu çalıştırırsak, işaretlemiş olduğumuz bu yorum satırları, tıpkı Şekil 14-2'de gösterildiği gibi `sandigim` belgesinin ön sayfasında, sandıktaki genel öğeler listesinin üstünde görüntülenecektir:

![resim trpl14-02](https://github.com/RustDili/dokuman/blob/master/ceviriler/img/trpl14-02.png)
Resim 14-2: `Sandigim`'ın tamamını içeren yorumlarla oluşturulmuş HTML belgeleri

Öğelerdeki belge yorumları, özellikle sandık ve modülleri tanımlamak için kullanışlıdır. Bu yorumları, paketlerinizi kullanacak olan kişilerin paket düzeninizi anlamalarına yardımcı olmak ve paket kapsamının genel amacını açıklamak için kullanmanız önemlidir.

### Uygun Bir Genel API'yi `pub use` ile Dışa Aktarmak

Bölüm 7'de kodlarımız, `mod` anahtar kelimesini kullanarak modüller halinde nasıl düzenleyeceğinizi, `pub` anahtar sözcüğüyle öğelerin nasıl genelleştirileceğini ve `use` anahtar kelimesiyle de öğelerin kapsama nasıl dahil edileceğini incelemiştik. Ancak, bir sandığın geliştirilme sürecinde sizin için anlamlı olan organizasyon yapısı, kullanıcılarınız için çok uygun olmayabilir. Sandığınızı çok katmanlı ve hiyerarşik bir yapıda düzenlediğinizde, bu hiyerarşinin alt katmanlarında tanımlanmış bir türü kullanmak isteyen kişiler, bu türe erişmekte sorun yaşayabilirler. Hem ayrıca bir türe `use sandigim::KullanisliBirTur;` şeklinde bir söz dizimiyle ulaşmak yerine,  `sandigim::bir_modul::baska_bir_modul::KullanisliBirTur;` şeklinde bir söz dizimiyle ulaşmak oldukça rahatsız edici olabilir.

Bir sandık yayınlarken herkese açık olarak tasarlanmış olan API'nizin yapısı oldukça önemlidir. Sandığınızı kullanan kişiler bu yapıya sizin kadar aşina olmadıklarından, sandığınız büyüyüp karmaşık bir *modüller hiyerarşisine* dönüştüğünde, kullanmak istedikleri API parçalarına ulaşmakta zorluk çekebilirler. 

İyi haber şu ki, eğer organizasyon yapınız başkaları tarafından farklı kütüphaneler ile kullanılamayacak gibiyse, API hiyerarşisini veya tasarımını baştan sona yeniden düzenlemek yerine, `pub use` anahtar kelimesini kullanarak, bu yapının genel kullanıma uygun bir sürümünü tüm öğeleriyle birlikte yeniden ihraç edebilirsiniz. Yeniden ihraç işleminde, bir konumda bulunan genel bir öğe yerinden alınarak, sanki başka bir yerde ve başka bir konumda tanımlanmış gibi herkese açık hale getirilir.  

 Örnek 14-3'te de görüleceği gibi, sanatsal kavramları modellemek için `sanat` adında bir kütüphane tasarladığımızı varsayalım. Ve bu kütüphanenin içinde `BirincilRenk` ve `IkincilRenk` olarak isimlendirilmiş iki sıralamadan *(enum)* oluşan `turler` modülü ve `karisim` adında bir işlev içeren `araclar` modülü bulunsun:

Dosya: src/lib.rs
```Rust
//! # Sanat
//!
//! Sanatsal kavramları modellemek için bir kütüphane.

pub mod turler {
    /// RYB renk modeline göre ana renkler.
    pub enum BirincilRenk {
        Kizil,
        Sari,
        Mavi,
    }

    /// RYB renk modeline göre ikincil renkler.
    pub enum IkincilRenk {
        Portakal,
        Yesil,
        Mor,
    }
}

pub mod araclar {
    use crate::turler::*;

    /// İkincil bir renk oluşturmak için iki ana rengi
    /// eşit miktarda birleştirir.
    pub fn karisim(c1: BirincilRenk, c2: BirincilRenk) -> IkincilRenk {
        // --snip--
    }
}
````
Örnek 14-3: `turler` ve `araclar` modülleri halinde düzenlenmiş öğeler içeren bir `sanat` kütüphanesi

Resim 14-3, Bu sandık için`cargo doc` tarafından üretilen belgenin ön yüzünü göstermektedir:

![resim trpl14-03](https://github.com/RustDili/dokuman/blob/master/ceviriler/img/trpl14-03.png)
Resim 14-3: `turler` ve `araclar` modüllerini örnekleyen `sanat` sandığının ön yüzü

Belgenin ön sayfasında `BirincilRenk` ve `IkincilRenk` türleriyle `karisim` işlevinin listelenmediğine dikkat edin. Onların görüntülenebilmesi için `turler` ve `araclar` bağlantılarının açılması gerekir.

Bu kütüphaneye bağımlı olan başka bir sandığın, halihazırda tanımlanmış olan `sanat` modül yapısına ait öğeleri kendi kapsamına alabilmesi için `use` ifadesini kullanması gerekir. Örnek 14-4, `sanat` sandığındaki `BirincilRenk` ve `karisim` öğelerini kullanan başka bir sandık örneğini göstermektedir:

Dosya: src/main.rs
```Rust

use sanat::turler::BirincilRenk;
use sanat::araclar::karisim;

fn main() {
    let kizil = BirincilRenk::Kizil;
    let sari = BirincilRenk::Sari;
    karistir(kizil, sari);
}

````
Örnek 14-4: Örnek 14-4: İç yapısı dışa aktarılan sanat sandığının öğelerini kullanan başka bir sandık 

Örnek 14-4'te yer alan `sanat` sandığını kullanan kodun programcısı, `BirincilRenk` türünün `turler` modülünde ve `karisim` işlevinin de  `araclar` modülünde olduğunu anlayabilmelidir. Sandığın modül yapısı, `sanat` sandığını geliştiren programcılardan çok, bu sandığı kullanan  programcılar için önemlidir. Sandığın parçalarını `turler` ve `sanat` modülleri olarak düzenleyen iç yapı, bu sandığın nasıl kullanılacağını öğrenmek isteyenler için herhangi bir yararlı bilgi içermediği gibi, `sanat` sandığı modül yapısının yarattığı karmaşa, `use` ifadelerinde modül adlarını belirtmek isteyen kullanıcıların nereye bakacaklarını karıştırmalarına neden olacağından kullanışsız bir yapıdır. 

Herkesin kullanacağı bu API'nin sorunlu iç düzenlemesini `pub use` kullanarak kaldırıp, öğeleri en üst düzeyde yeniden dışa aktarabilmek için, Örnek 14-3'te yer alan `sanat` sandığının kodlarını, Örnek 14-5'te gösterildiği şekilde yeniden düzenleyebiliriz. 

Dosya: src/lib.rs
```Rust
//! # Sanat
//!
//! Sanatsal kavramları modellemek için bir kütüphane.

pub use self::araclar::karisim;
pub use self::turler::BirincilRenk;
pub use self::turler::IkincilRenk;

pub mod turler {
    // --snip--
}

pub mod araclar {
    // --snip--
}
````
Örnek 14-5: Öğeleri yeniden dışa aktarmak için `pub use` ifadesi eklemek

Şekil 14-4'te görebileceğiniz gibi `cargo doc` komutunun bu sandık için oluşturduğu API belgeleri, dışa aktarımları yeniden ön sayfada listeleyerek bağlayacak, `BirincilRenk` ve `IkincilRenk` türleriyle `karisim` işlevinin bulunmasını oldukça kolaylaştıracaktır.

![resim trpl14-04](https://github.com/RustDili/dokuman/blob/master/ceviriler/img/trpl14-04.png)
Resim 14-4: Yeniden dışa aktarımı örnekleyen `sanat` sandığının ön yüzü

Artk `sanat` sandığını kullanmak isteyen programcılar, ister hâlâ kullanılmaya uygun durumdaki Örnek 14-3 ve Örnek 14-4'ün iç yapılarını inceleyerek kodlarına bunları dahil edebilirler, isterlerse Örnek 14-5 ve 14-6'da yenilenerek kullanıma daha uygun hale getirilen yapıyı tercih edebilirler.

Dosya src/main.rs
```Rust
use sanat::karisim;
use sanat::BirincilRenk;

fn main() {
    // --snip--    
}
````
Örnek 14-6: `sanat` sandığının yeniden dışa aktarılan öğelerini kullanan bir program

İç içe geçmiş çok sayıda modülü, türleri `pub use` ifadesiyle en üst düzeyde yeniden dışa aktarmak, sandığı kullanacak kişilerin deneyimlerinde önemli farklar yaratabilir. 

Kullanışlı bir genel API tasarımı oluşturmak bilimden çok sanat olarak kabul edildiğinden, kullanıcılarınız için en iyi çalışacak uygun bir düzenleme için defalarca tekrar yapmanız gerekebilir. Bununla birlikte `pub use` kullanımını seçmek, sandığınızın iç düzenlemesinde size esneklik sağlarken, bu iç düzeni kullanıcılarınıza sunduğunuz arayüzden ayırır. Kurduğunuz bazı sandıkların iç düzenlemelerinin genel API katmanlarına göre farklarını incelemek için bu sandıkların kodlarına bakmanız öğretici olacaktır.

### Crates.io Hesabı Oluşturmak

Herhangi bir sandığı yayınlayabilmeniz için öncelikle [crates.io](https://crates.io/) üzerinde bir hesap oluşturmanız ve bir API anahtarı almanız gerekir. Bunun için *(Her ne kadar gelecekte siteye başka yöntemlerle üye olunması planlanmış olsa da şu an için yalnızca GitHub hesaplarımız ile giriş kabul edildiğinden)* [crates.io](https://crates.io/) adresini ziyaret ederek GitHub hesabınız ile giriş yapın. Ardından, [https://crates.io/me/](https://crates.io/me/) adresindeki hesap ayarlarınızı gözden geçirerek API anahtarınızı alın. Aldığınız bu API anahtarını `cargo login` komutuna ekleyerek tıpkı aşağıda örneklendiği gibi çalıştırın:

```bash
$ cargo login abcdefghijklmnopqrstuvwxyz012345
````

Bu komut API anahtarınızı Cargo'ya bildirecek ve yerel olarak onu *~/.cargo/credentials* içinde depolayacaktır. Anahtarınızın *size özel* olduğunu, *gizli* kalması ve kimseyle paylaşılmaması gerektiğini unutmayın. Eğer herhangi bir sebeple anahtarınızı birileriyle paylaşmak zorunda kalırsanız, eskisini derhal iptal ederek yeni bir anahtar oluşturun.

### Yeni Oluşturulmuş Sandığa Meta Veri Eklemek

Artık hesabınızı oluşturduğunuza göre, yayınlamak istediğiniz bir sandığınız olduğunu düşünebiliriz. Ancak sandığınızı yayınlamadan önce, *Cargo.toml* dosyasının `[package]` bölümüne sandığınıza ait meta veriler eklemeniz gerekir.

Bir sandık üzerinde yerel olarak çalışırken onu istediğiniz gibi adlandırabilirsiniz. Ancak sandığınız yayın aşamasına geldiğinde benzersiz bir isme ihtiyacı olacak. [crates.io](https://crates.io/)'daki sandık isimlerinde öncelik ilk gelene verildiğinden, bir isim bir sandığa tahsis edildikten sonra başka bir sandığa tahsis edilemez. Başka bir ifadeyle, sandık adı bir kez alındığında aynı isimde başka bir sandık yayınlanamaz. O yüzden sandığınızı [crates.io](https://crates.io/) üzerinde yayınlanmadan önce, sandık adınızı kullanan başka bir sandık olup olmadığını araştırmalısınız. Eğer sandık adınız halihazırda başka bir sandık tarafından kullanılmakta ise, yenileyeceğiniz sandık adını, paketinizin *Cargo.toml* dosyasında bulunan `[package]` bölümüne, isim alanıyla birlikte aşağıda gösterildiği gibi girmeniz gerekir.

<!-- Anlam bozukluklarının düzeltilmesi -->

Dosya: Cargo.toml

```Rust

[package]
name = "tahmin_oyunu"

````
Her ne kadar benzersiz bir ad seçmiş olsanız bile, sandığı yayınlamak için `cargo publish` komutunu çalıştırdığınızda aşağıdakine benzer bir uyarı ve hata alabilirsiniz:

```bash

$ cargo publish
    Updating crates.io index
warning: manifest has no description, license, license-file, documentation, homepage or repository.
See https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata for more info.
--snip--
error: api errors (status 200 OK): missing or empty metadata fields: description, license. Please see https://doc.rust-lang.org/cargo/reference/manifest.html for how to upload metadata

````

Bunun sebebi, sandığınızı kullanmak isteyecek programcılar için hazırlamanız gereken, sandığınızın neler yaptığını ve hangi koşullar altında kullanılabileceğini düzenleyen açıklama ve lisans bilgileri gibi önemli detayları atlamış olmanızdır. Bu hatayı düzeltmek için gerekli olan bilgileri paketinizin *Cargo.toml* dosyasına işlemeniz gerekir. 

Girdiğiniz açıklamalar arama sonuçlarında görüntüleneceğinden, en azından bir iki cümle açıklamak eklemeniz yerinde olur. Lisans alanı içinse bir `license` tanımlayıcı değeri vermeniz gereklidir. [Linux Vakfı'nın Yazılım Paketi veri değişimi (SPDX)](http://spdx.org/licenses/), bu alan için kullanabileceğiniz tanımlayıcıları listeler. Örneğin, sandığınızı MIT Lisansı ile lisansladığınızı belirtmek için `MIT` tanımlayıcısını eklemeniz gerekir:

```bash
[package]
name = "tahmin_oyunu"
license = "MIT"
````

SPDX'te listelenmemiş bir lisans kullanmak istiyorsanız, söz konusu lisansın metnini bir dosyaya yerleştirmeniz, dosyayı projenize eklemeniz ve ardından `license` alanındaki tanımı `license-file` şeklinde dosya adını tanımlayacak şekilde belirtmeniz gerekmektedir.

Projeniz için hangi lisansın daha uygun olacağına dair rehberlik bu kitabın kapsamı dışındadır. Rust topluluğunun pek çok üyesi projelerini, Rust'ın tercih ettiği gibi `MIT VEYA Apache-2.0` olarak çifte lisans kullanarak sunar. Bu uygulama biçimi projenizi `OR` ekiyle birden fazla lisansa sahip olacak şekilde lisans tanımlayıcısıyla ilişkilendirebileceğinizi gösterir.

Benzersiz bir isim seçtiğiniz sandığınıza; yazar adı, sürüm bilgisi, paket açıklaması ve lisans bilgileri eklendikten sonra, yayına hazır hale gelen projenizin `Cargo.toml` dosyası aşağıdaki dosya gibi görünecektir:

```bash

[package]
name = "tahmin_oyunu""
version = "0.1.0"
authors = ["Rust Dili <rustdili@gmail.com>"]
edition = "2018"
description = "Bilgisayarın seçtiği sayıyı tahmin ederken eğleneceğiniz keyifli bir oyun."
license = "MIT OR Apache-2.0"

[dependencies]

````

Sandıklarınızın kolaylıkla fark edilip kullanabilmesi için değerlendirebileceğiniz meta verilere [Cargo Belgeleri](https://doc.rust-lang.org/cargo/) üzerinden kolaylıkla ulaşabilirisiniz.

### Bir sandığı Crates.io'da Yayınlamak

Artık bir hesabınız, API anahtarınız, benzersiz ada sahip bir sandığınız ve bu sandığa gerekli meta verileri eklediğinize göre sandığızı yayınlamaya hazırsınız demektir. Bir sandığı yayınlamak demek, sandığınızın belirli bir sürümünü başka kullanıcılar için [crates.io](https://crates.io/) sitesine yüklemek anlamına gelmektedir. 

[Crates.io](https://crates.io/)'nun asıl hedeflerinden biri yayınlanan sandıklara bağımlı tüm projelerin çalışmaya devam edebilmesi için kalıcı bir kod arşivi oluşturmaktır. Bu nedenle bir sandık yayınlandıktan sonra, yayınlanan sürümün üzerinde ekleme, düzenleme yahut silme gibi işlemler artık yapılamayacağından, sandık yayınlarken dikkatli olunması gerekir. Sürüm silme işlemine izin vermek bu hedefi imkansızlaştıracağından sürüm silmek yerine sandığın başka sürümlerinin yayınlaması yeğlenir. O nedenle bir sandığa ait sürümlerin yayın sınırı yoktur. 

Şimdi `cargo publish` komutunu tekrar çalıştırdığınızda paketinizin başarıyla yayınlandığını göreceksiniz:

```bash

$ cargo publish
    Updating crates.io index
   Packaging tahmin_oyunu v0.1.0 (file:///projem/tahmin_oyunu)
   Verifying tahmin_oyunu v0.1.0 (file:///projem/tahmin_oyunu)
   Compiling tahmin_oyunu v0.1.0
(file:///projem/tahmin_oyunu/target/package/tahmin_oyunu-0.1.0)
    Finished dev [unoptimized + debuginfo] target(s) in 0.19s
   Uploading tahmin_oyunu v0.1.0 (file:///projem/tahmin_oyunu)

````

Tebrikler! Artık kodunuzu Rust topluluğuyla paylaştığınıza göre, herkes sizin sandığınızı kendi projesine bağımlılık olarak kolayca ekleyebilir.

### Mevcut Sandığın Yeni Sürümünü Yayınlamak

Daha önce yayınladığınız bir sandık üzerinde tüm kullanıcıları etkileyebilecek değişiklikler yaptığınızda ya da sandığınızın yeni sürümünü yayınlamak istediğinizde, *Cargo.toml* dosyasında belirtilen sürüm değerini değiştirerek yeniden yayınlayabilirsiniz. Yaptığınız değişiklik türlerine bağlı olarak, sonraki sürümün numarasına karar verirken [Anlamsal Sürüm Oluşturma Kuralları](http://semver.org/) sitesini kullanabilirsiniz. Yeni sürümünüzü düzenlemelerinizi gerçekleştirdikten sonra `cargo publish` komutunu kullanarak yayınlayabilirsiniz.

### Sürümleri Crates.io'dan `cargo yank` Komutu Kullanarak Kaldırmak

Yayımlanmış bir sandığın önceki sürümlerini kaldıramasanız dahi, sandık sürümünüz bir nedenle bozulduğunda, yeni projelerin bu sürümleri kullanmasını ve bağımlılık olarak eklemesini engelleyebilirsiniz. Cargo'nun böyle durumlarda kullanılan `yank` komutu ilgili sürümün geri çekilmesini sağlar. 

Bir sürümün geri çekilmesi, yeni projelerin bu sürüme bağlanmasını önlerken, halihazırda kendisine bağımlı olan projelerin bu sürümü indirmesine ve bağımlı olarak çalışmaya devam etmesine izin verir. Temel olarak `yank` yani geri çekme işlemi *Cargo.lock* dosyasına işlenmiş projelerin bozulmadan kullanılmaya devam edeileceğini, ama yeni bağımlılık taleplerinde bu sürümün kullanılmasına izin verilmeyeceği anlamına gelmektedir.

Geri çekilmek istenen sandık sürümü için `cargo yank` komutunun aşağıda örneğe benzer biçimde kullanılması gerekir:

```bash
$ cargo yank --vers 1.0.1
````

Bu geri çekme işlemini `cargo yank` komutuna `--undo` ekleyerek geçersiz hale getirebilir ve projelerin bu sürüme yeniden bağlanmasına izin verebilirsiniz:

```bash
$ cargo yank --vers 1.0.1 --undo
````

Geri çekme işlemi halihazırda var olan kodları silemez. Eğer projenize yanlışlıkla eklediğiniz bazı özel veya sır olarak kalması gereken kodlar var ve bu kodları `yank` kullanarak silmek istiyorsanız, bunu yapmanız mümkün olamayacağından, sürümünüzü yayınlamadan önce bu kodları sıfırlamanız veya kaldırmanız gerekir.
