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
