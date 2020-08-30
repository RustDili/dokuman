# Rust Programlama Dili

*Steve Klabnik ve Carol Nichols'tan, Rust Topluluğu'nun katkılarıyla...*

Metnin bu sürümü Rust 2018 baskısının deyimlerinden yararlanabilmek için, tüm projelerin *Cargo.toml* belgelerinde `edition = 2018` ifadesini içerdiğini ve Rust 1.41.0 veya daha üst bir sürümünü kullandığınızı varsayar. Rust'ı yüklemek veya güncellemek için kitabın [Bölüm 1 "Kurulum"](https://rustdili.github.io/ch01-01-installation.html)  bölümünü, sürümler hakkında bilgilere ulaşmak içinse [Ek E](https://rustdili.github.io/appendix-05-editions.html) belgesini takip edin.

Rust dili kitabının 2018 Sürümü, Rust'u daha ergonomik ve öğrenilmesi kolay bir dil haline getiren iyileştirmeleri içerir. Kitabın bu baskısı aşağıdaki iyileştirmeleri yansıtan bir dizi değişiklik yer almaktadır:

- Bölüm 7, "Büyüyen Projeleri Paketler, Sandıklar ve Modüller ile Yönetmek" neredeyse yeniden yazıldı. 2018 Sürümü'ndeki modül sistemi ve yolların çalışma şekli daha tutarlı hale getirildi.
- Bölüm 10, yeni `impl Trait` sözdizimini açıklayan "Parametre Olarak Özellikler" ve "Özellikleri Uygulayan Geri Dönüş Türleri" başlıklı yeni bölümlere yer vermektedir.
- Bölüm 11, `?` operatörünü kullanan testlerin nasıl yazılacağını gösteren, "Testlerde `Result<T, E>` Kullanımı" başlıklı yeni bir bölüme sahiptir.
- Derleyicide sağlanan iyileştirmeler yaşam süresi belirtimlerine daha az ihtiyaç duyduğundan, Bölüm 19'daki "Gelişmiş Yaşam Süreleri" bölümü kaldırıldı.
- Daha önce Ek D belgesinde yer alan "Makrolar" konusu, prosedür makrolarını da içerecek şekilde genişletilerek, Bölüm 19'da yer alan "Makrolar" başlıklı müstakil alana taşındı.
- Ek A'da yer alan "Anahtar Kelimeler" başlığında, 2015 ve 2018 Sürümü için yazılmış kodların birlikte çalışmasını sağlayan yeni "ham tanımlayıcılar" özelliğine yer verilmiştir.
- Ek D artık "Faydalı Geliştirme Araçları" olarak adlandırılıyor ve yakın zamanda yayınlanmış bulunan Rust kodu yazmanıza yardımcı olacak araçları kapsıyor.
- Kitap boyunca bir dizi küçük hatayı ve kesin olmayan ifadeleri düzelttik. Bu sorunları bize bildiren okuyuculara teşekkür ederiz!

*Rust Programlama Dili* kitabının önceki sürümlerinde yer alan kodların, halihazırda kullandığınız Rust derleyicisinin sürümünü yükseltmiş olsanız bile, Cargo.toml belgelerinde edition = "2018" bildirimi olmadan da derlenmeye devam edeceğini hatırlatırız. İşte bu Rust’un geriye dönük uyumluluğunun bir garantisidir!

Bu kitabın çevrimiçi HTML formatı [https://rustdili.github.io/](https://rustdili.github.io/) adresinde yer almaktadır. Kitabın çevrimdışı çalışan Orijinal dildeki kopyası ise, rustup ile yapılan Rust kurulumlarında mevcut olup, bu kitabı okumak için terminalinizde `rustup docs --book` komutunu çalıştırmanız yeterlidir.

Bu kitabı kapaksız ve e-kitap biçiminde [No Starch Press](https://nostarch.com/rust) adresinden temin edebilirsiniz.