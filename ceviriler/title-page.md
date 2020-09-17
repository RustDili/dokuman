# Rust Programlama Dili

*Steve Klabnik ve Carol Nichols'tan, Rust Topluluğu'nun katkılarıyla...*

Metnin bu sürümü Rust 2018 baskısının deyimlerinden yararlanabilmek için, tüm projelerin *Cargo.toml* belgelerinde `edition = 2018` ifadesini içerdiğini ve Rust 1.41.0 veya daha üst bir sürümünü kullandığınızı varsayar. Rust'ı yüklemek veya güncellemek için kitabın [Bölüm 1 "Kurulum"](https://rustdili.github.io/ch01-01-installation.html)  ayracını, sürümler hakkında bilgilere ulaşmak içinse [Ek E](https://rustdili.github.io/appendix-05-editions.html) belgesini takip edin.

Rust dili kitabının 2018 Sürümü, Rust'ı daha ergonomik ve öğrenilmesi kolay bir dil haline getiren iyileştirmeleri içerir. Kitabın bu baskısında aşağıdaki iyileştirmeleri yansıtan bir dizi değişiklik yer almaktadır:

- "Büyüyen Projeleri Paketler, Sandıklar ve Modüller ile Yönetmek" başlıklı 7. Bölüm neredeyse yeniden yazıldı. 2018 Sürümü'ndeki modül sistemi ve yolların çalışma şekli daha tutarlı hale getirildi.
- Bölüm 10'da ise, yeni `impl Trait` söz dizimini açıklayan "Parametre Olarak Özellikler" ve "Özellikleri Uygulayan Geri Dönüş Türleri" başlıklı yeni konulara yer verildi.
- Bölüm 11'e, testlerin `?` işlecini kullanarak nasıl yazılacağını gösteren, "Testlerde `Result<T, E>` Kullanımı" başlıklı yeni bir konu eklendi.
- Derleyicide sağlanan iyileştirmeler yaşam süresi belirtimlerine daha az ihtiyaç duyduğundan, Bölüm 19'daki "Gelişmiş Yaşam Süreleri" bölümü kaldırıldı.
- Daha önce Ek D belgesinde yer alan "Makrolar" konusu, prosedür makrolarını da içerecek şekilde genişletilerek, Bölüm 19'da yer alan "Makrolar" başlıklı müstakil alana taşındı.
- Ek A'da yer alan "Anahtar Kelimeler" başlığında, 2015 ve 2018 Sürümü için yazılmış kodların birlikte çalışmasını sağlayan yeni "ham tanımlayıcılar" özelliğine yer verildi.
- Ek D artık "Faydalı Geliştirme Araçları" olarak adlandırılıyor ve yakın zamanda yayınlanmış olan Rust kodu yazmanıza katkı sağlayacak araçlara yer veriyor.
- Kitap boyunca bir dizi küçük hatayı ve kesin olmayan ifadeleri düzelttik. Bu sorunları bize bildiren okuyuculara teşekkür ederiz!

*Rust Programlama Dili* kitabının önceki sürümlerinde yer alan kodların, halihazırda kullandığınız Rust derleyicisinin sürümünü yükseltmiş olsanız bile, Cargo.toml belgelerinde `edition = "2018"` bildirimi olmadan da derlenmeye devam edeceğini hatırlatırız. İşte bu Rust’un geriye dönük uyumluluğunun bir garantisidir!

Bu kitabın çevrimiçi HTML formatı [https://rustdili.github.io/](https://rustdili.github.io/) adresinde yer almaktadır. Kitabın çevrimdışı çalışan Orijinal dildeki kopyası ise, `rustup` ile gerçekleştirilen Rust kurulumla ile gelir. Bu kitabı okumak için terminalinizde `rustup docs --book` komutunu çalıştırmanız yeterli olacaktır.

Bu kitabın orijinal dilindeki sürümünü, kapaksız ve e-kitap biçiminde [No Starch Press](https://nostarch.com/rust) adresinden temin edebilirsiniz.
