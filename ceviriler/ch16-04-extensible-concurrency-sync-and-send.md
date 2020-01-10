## `Sync` ve `Send` özellikleri ile genişletilebilir eş zamanlılık
İlginçtir ki, Rust dili çok az eş zamanlılık özelliğine sahiptir. Bu bölümde şimdiye kadar bahsettiğimiz hemen hemen her eş zamanlılık özelliği, dilin değil, standart kütüphanenin bir parçasıydı. Rust’ta eş zamanlılığı yönetme seçenekleri, dil veya standart kitaplıkla sınırlanmadığından, kendi eş zamanlılık özelliklerinizi yazabilir veya başkaları tarafından yazılanları kullanabilirsiniz.

Bununla birlikte, `std::marker` kütüphanesi içinde gömülü olan iki eş zamanlılık kavramı: `Sync` ve `Send` özelliklerinden bahsetmek gerek.

### `Send` kullanarak iş parçaları arasında mülkiyet aktarımı
Standart kütüphanenin marker özelliği, `Send` kullanılarak mülkiyetin iş parçaları arasında aktarılabileceğini gösterir. Hemen hemen her Rust türü birer `Send` türü olmakla birlikte `Rc<T>` gibi bazı istisnalar; kendi değerleri klonlanıp, başka bir iş parçasına aktarıldıklarında, her iki iş parçasını da referans sayılarıyla birlikte güncelleyebilirler. `Rc<T>` getirdiği bu ek maliyet nedeniyle genellikle tek iş parçasının kullanıldığı durumlar için uygun olur. 
Bu nedenle, Rust’ın tür sistemi ve özellik sınırları, yanlışlıkla da olsa `Rc<T>` değerinin parçalarına gönderilmesini engeller. Bu girişimi örnek 16-14’ te denediğimizde **`Send` özelliğinin `Rc<Mutex<i32>>` için uygulanmadığını** ancak aynı mutex’in örnek 16-15’ te olduğu gibi `Arc<T>` ile sarmalandığında derlenerek çalıştığını görmüştük.

`Send` türlerinden oluşan her tür, bölüm 19’da tartışılacak olan ham işaretçiler dışında kalan neredeyse tüm ilkel türler, otomatik olarak `Send` olarak işaretlenir. 

### `Sync` ile çok sayıda iş parçasından erişime izin vermek
`Sync` marker özelliği, bu özelliği uygulayan türün çok sayıda iş parçası tarafından referans alınmasının güvenli olduğunu gösterir. Başka bir ifadeyle, herhangi bir `T` türü `&T` ise, yani `T` için bir referans ise, o tür hem bir `Send` hem de `Sync`’tir; yani referans iş parçasına güvenle gönderilebilir. `Send`’e benzer şekilde, hemen hemen tüm ilkel türler de birer `Sync`’tir. Haliyle `Sync` türlerinden oluşan türler de birer `Sync`’dir. 

Fakat akıllı işaretçilerden olan `Rc<T>`, hem gönderme hem de alma niteliklerine sahip olduğundan `Sync` değildir. 15. Bölümde bahsetmiş olduğumuz `RefCell<T>`, çalışma zamanında gerçekleştirdiği borç kontrolü işlemleri nedeniyle iş parçaları için güvenli sayılamayacağından; bu ve ilgili tüm `Cell<T>` türleri de `Sync` değillerdir. Ancak yine akıllı bir işaretçi olan `Mutex<T>` ise `Sync`’tir ve çok sayıda iş parçası arasında erişim sağlamak için güvenle kullanılabilir.

### `Send` ve `Sync` özelliklerini elle uygulamak güvenli değildir
Çünkü `Send` ve `Sync` özelliklerinden oluşan türler, zaten otomatik olarak `Send` ve `Sync` kabul edildiklerinden bu özellikleri elle uygulamak gereksizdir. Hatta bu ikilinin işaretçi özellikleri olarak uygulayacakları bir yöntem bile yoktur. Sadece eş zamanlı uygulamalarda, ilgili değişmezleri zorlamak için kullanılışlıdırlar.

Yine de bu özellikleri elle uygulamaya çalışmak, güvensiz Rust kodlarının kullanılmasını gerektirir ve bu konu bölüm 19’ da ele alınacaktır. Şimdilik üzerinde düşülmesi gereken en önemli şey; `Send` ve `Sync` özellikleri taşımayan yeni eş zamanlı türler oluştururken, güvenlik garantilerini sağlamak için dikkatli düşünmektir. Bu garantiler ve bunların nasıl sağlanacağı hakkında daha fazla bilgi için [The Rustonomicon’a başvurabilirsiniz.](https://doc.rust-lang.org/stable/nomicon/)

## Bölüm özeti
Kitabın eş zamanlılık konusunu inceleyen kısmının sadece burası olmadığını, 20. Bölümde gerçekleştireceğimiz projede, bu bölümde anlatılan kavramları, buradaki küçük örneklerden daha gerçekçi şekilde kullanacağımızı hatırlatmak istiyoruz.

Önceden de belirtildiği gibi Rust’ın eş zamanlılığı dilin bir parçası olarak ele almasından dolayı buradaki kısıtlılığın aksine, sandıklar üzerinden çok sayıda eş zamanlılık çözümü sunulmaktadır. Bu sandıklardaki eş zamanlılık süreçleri standart kütüphanedeki çözümlerden çok daha hızlı çalışırlar. Bu nedenle çoklu iş parçaları kullanmanız gerektiğinde çevrimiçi sunulan modern ve güncellenmiş sandıklara  ulaşmanız önemlidir.

Rust’ın standart kütüphanesi mesajlaşarak eş zamanlı programlama ve eş zamanlı bağlamlarda güvenle kullanılabilen `Mutex<T>` ve `Arc<T>` gibi akıllı işaretçi türleri için kanallar sağlar. Tür sistemi ve borçlanma denetçisi bu çözümleri kullanan kodun, veri yarışları veya geçersiz referanslarla sonuçlanmayacağından emin olur. Kodunuz derlendikten sonra, diğer dillerde yaygın olarak görülebilen karmaşık hatalarla karşılaşmadan çok sayıda iş parçası üzerinde rahatlıkla çalışacağından emin olabilirsiniz. Eş zamanlı programlama artık korkulacak bir kavram değil: öne çıkın ve programlarınızı korkusuzca eş zamanlı yapın!

Bir sonraki bölümde Rust programlarınız büyüdükçe sorunları modellemenin ve çözümleri yapılandırmanın deyimsel çözüm yollarını tartışacak, Rust dilinin nesne yönelimli programlama ailesiyle ilişkilerinden bahsedeceğiz.
