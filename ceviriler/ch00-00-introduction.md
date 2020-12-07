# Giriş
> Not: Kitabın orijinal dilindeki sürümü, [basılı](https://nostarch.com/rust) ve [e-kitap](https://nostarch.com/) biçiminde 
> No Starch Press üzerinde ve orijinal dilinde sunulmaktadır.

Rust üzerine bir giriş kitabı olan Rust Programlama Dili'ne hoş geldiniz. Rust programlama dili, daha hızlı ve daha güvenilir programlar yazmanıza yardımcı olur. Programlama dili tasarımlarında üst düzey ergonomi ile düşük seviyeli kontrol, genel bir anlaşmazlık halindedir ve Rust bu çatışmaya meydan okur. Bu olağanüstü tasarım bir yandan bellek kullanımı gibi geleneksel olarak düşük seviyeli kontrol ile ilişkilendirilen tüm zorlukları ortadan kaldırırken, diğer yandan sağladığı güçlü teknik kapasite ve olağanüstü geliştirici deneyimini dengeleyerek bu ayrıntıları rahatlıkla kontrol etmenizi sağlar.

## Rust Kimler İçin

Rust, çeşitli nedenlerden dolayı pek çok insan için idealdir. Bu insanların ait olduğu önemli üretim gruplardan birkaçına bakalım.

### Geliştirici Ekipleri

Rust, sistem programlama bilgileri farklı düzeylerde olan kalabalık geliştirici ekipleri arasında işbirliğini tesis eden verimli bir araç olduğunu kanıtlıyor. Düşük seviyeli kod, pek çok dilde kapsamlı testler ve deneyimli geliştiriciler tarafından, kodun dikkatle incelenmesiyle yakalanabilen çözümü zor hatalara eğilimlidir. Rust derleyicisi, eşzamanlılık hataları dahil bu türden hatalı kodların derlenmesini reddederek adeta bir bekçi rolü oynar. Böylelikle derleyiciyi takımın bir üyesi olarak gören geliştirici ekibi, değerli zamanlarını hataları takip etmek yerine, programın mantığına odaklanarak geçirebilirler.

Rust ayrıca sistem programlama dünyasına çağdaş geliştirici araçları da sunar:

* Rust ile birlikte gelen bağımlılık yöneticisi ve derleme aracı olan Cargo, Rust ekosisteminde bağımlılıkları ekleme, derleme ve yönetmeyi sancısız ve tutarlı hale getirir.
* Rustfmt ise geliştiriciler arasında tutarlı bir kodlama tarzı oluşturur.
* Rust Dil Sunucusu, kod tamamlama ve satır içi hata mesajları için *Entegre Geliştirme Ortamı (IDE)*  entegrasyonunu destekler.

Bunlar ve Rust ekosistemindeki diğer araçları kullanan geliştiriciler sistem düzeyinde kod yazarken daha üretken olabilirler.

### Öğrenciler

Rust, öğrenciler ve sistem kavramlarını öğrenmekle ilgilenenler için tasarlanmıştır. Pek çok kişi Rust kullanarak işletim sistemleri geliştirme gibi alanları öğrenmiştir. Rust topluluğu oldukça misafirperver olup öğrencilerin sorularını heves ve heyacanla yanıtlamaktan çekinmezler. Bu kitap gibi girişimler aracılığıyla Rust ekipleri, sistem konseptlerini mümkün olduğunca çok kişi için, özellikle de programlamaya yeni başlayanlar için erişebilir hale getirmek istiyorlar.

### Şirketler

Büyüklü küçüklü yüzlerce şirket üretimlerinde çeşitli görevler için Rust'ı kullanıyorlar. Bu görevler arasında komut satırı araçları, web hizmetleri, DevOps araçları, gömülü cihazlar, ses, video analizi ve kod dönüştürme, kripto para birimleri, biyoinformatik, arama motorları, IOT uygulamaları, makine öğrenimi ve hatta Firefox web tarayıcısının önemli bölümleri bile bulunmakta.

### Açık Kaynak Geliştiricileri

Rust, Rust programlama dili, topluluğu, geliştirici araçları ve kütüphanelerinin oluşumuna katkı sağlamak isteyen kişiler içindir. Rust diline katkıda bulunmanızı çok isteriz.

### Hız ve İstikrara Değer Verenler

Biz hızdan bahsederken, hem Rust ile oluşturabileceğiniz programların hızından, hem de Rust'ın kodlama sürecinde sağladığı hızdan bahsediyoruz. Bu sebepten Rust, bir dilden hız bekleyenler içindir.

Rust derleyicisinin kontrolleri özellik ekleme ve yeniden düzenleme aşamalarında kararlılık sağladığından, bu kontrollerin yapılamadığı dillerde geliştiricilerin eski ve kırılgan kodları değiştirmekten korktukları sürecin tam tersidir. Bu nedenle Rust, bir dilden istikrar bekleyenler içindir,

Sıfır maliyetli soyutlamalar için çabalayan Rust, elle yazılan kodlar kadar çabuk düşük seviyeli kodlara derlenen üst düzey özellikler için gayret ederken, güvenli çalışan kodu hızlı çalışan bir kod haline getirmeye uğraşır. Dolayısıyla Rust, hız ve istikrar bekleyenler içindir!

<!-- Çeviride kaldığım yer -->

