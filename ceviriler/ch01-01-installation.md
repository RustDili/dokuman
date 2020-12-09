## Kurulum

İlk adım Rust'ı kurmaktır. Rust sürümlerini ve bunlarla ilişkili araçları yönetmek için tasarlanmış bir komut satırı aracı olan `rustup` aracılığıyla Rust'ı indireceğiz. İndirme işlemini gerçekleştirebilmek için internet bağlantısına ihtiyacınız olacak.

> Herhangi bir nedenle `rustup` aracını kullanmak istemiyorsanız 
> diğer seçenekler için lütfen [kurulum sayfasını](https://www.rust-lang.org/tools/install) inceleyiniz.

Sonraki adım Rust derleyicisinin en son kararlı sürümünü kurmaktır. Rust'ın kararlılık garantileri kitapta derlenen örneklerin, Rust'ın daha yeni sürümleriyle de derlenmeye devam etmesini sağlayacaktır. Rust genellikle hata mesaj ve uyarılarını iyileştirdiğinden, derleyici çıktıları sürümden sürüme biraz değişebilir. Başka bir ifadeyle, buradaki kurulum adımlarına uyarak yüklediğiniz daha yeni ve kararlı Rust sürümleri, bu kitabın içeriğiyle uyumlu çalışmalıdır.

> ### Komut Satırı Gösterimi
>
> Bu bölümde ve kitap boyunca, terminalde kullanılan bazı komutları göstereceğiz.
> Bir terminale girmeniz gereken satırların her biri `$` karakteri ile başlar. 
> Ancak bu karakter her komutun başlangıcını gösterdiğinden ayrıca elle yazılmasına gerek yoktur.
> `$` karakteri ile başlamayan satırlar genellikle önceki komutun çıktısını gösterir.
> Buna ek olarak PowerShell'e özgü örneklerde `$` yerine `>` karakteri kullanılır.

### Linux ya da macOS için `rustup` Kurulumu

Rust'ı Linux ya da macOS bir sistemde kullanacaksanız bir terminal açarak aşağıdaki komutu giriniz:

```console
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

Bu komut bir komut dosyasını indirerek Rust'ın en son kararlı sürümünü sisteminize kuracak olan `rustup` aracının kurulumunu başlatır. Kurulum esnasında sistem parolanızın gerekebileceğini hatırlatırız. Kurulumuz başarılı olduğu takdirde aşağıdaki satır görünecektir.

```text
Rust is installed now. Great!
```
Ek olarak muhtemelen daha önceden sisteminize yüklenmiş olan bir tür bağlayıcıya da ihtiyacınız olacak. Eğer bir Rust programını derlemeye çalışırken bir bağlayıcının çalıştırılamadığını bildiren hatalar alıyorsanız bu, gerekli olan bağlayıcının sisteminizde yüklü olmadığını ve elle yüklemeniz gerektiği anlamına gelir. C derleyicileri genellikle doğru bağlayıcılarla birlikte gelir. C derleyicisinin kurulumunu öğrenmek için platformunuzun belgelerine göz atmanız gerekir. Ayrıca, bazı yaygın Rust paketleri C kodlarına bağımlı olduğundan bir C derleyicisine ihtiyaç duyacaktır. Bu nedenle şimdiden bir C derleyicisi edinmeniz yararlı olabilir.

### Windows için `rustup` kurulumu

Rust'ı Windows işletim sisteminize kurabilmeniz için [Windows için yükle](https://www.rust-lang.org/tools/install) adresine giderek yükleme talimatlarını uygulamanız gerekir. Kurulumun bir aşamasında Visual Studio 2013 veya sonrası için C++ derleme araçlarına da ihtiyacınız olacağını bildiren bir mesaj alacaksınız. Derleme araçlarını edinmenin en kolay yolu [Visual Studio 2019 için Derleme Araçları](https://visualstudio.microsoft.com/tr/visual-cpp-build-tools/)'nı yüklemektir. Bu yükleme esnasında yüklenecek bileşenleri seçmeniz gerektiğinde "C++ Derleme Araçları"nı seçtiğinizden ve Windows 10 SDK ile ingilizce dil paketi bileşenlerinin dahil edildiğinden emin olun.

Bu kitabın geri kalanı, hem cmd.exe hem de PowerShell'de çalışan komutları kullanır. Bunların arasında belirgin farklılıklar olursa hangisinin kullanılacağını belirteceğiz.

### Güncelleme ve Kaldırma

Rust'ı `rustup` ile kurduktan sonra en son sürümüne güncelleme yapmanız oldukça kolaydır. Bunun için terminalinizde aşağıdaki komut satırını çalıştırmanız yeterlidir: 

```console
$ rustup update
```

Eğer Rust ve `rustup` aracını kaldırmak isterseniz terminalinizde aşağıdaki satırı çalıştırmanız yeterlidir.

```console
$ rustup self uninstall
```

### Sorun Giderme

Rust'ın sisteminize doğru şekilde kurulup kurulmadığını kontrol etmek için terminalinizde aşağıdaki satırı çalıştırmanız yeterlidir:

```console
$ rustc --version 
```

Yayınlanan son kararlı sürümün numarasını, kayıt değeri ve işlem tarihini aşağıdaki biçimde görmelisiniz:

```text
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

Çıktınız bu şekildeyse Rust'ı başarıyla yüklemişsiniz demektir. Eğer Windows kullanıcısıysanız ve çıktınızı göremiyorsanız Rust'ın %PATH% sistem değişkeninizde olup olmadığını kontrol edin. Bunların her biri doğru uygulanmış,  yerli yerinde ve Rust halen çalışmıyorsa yardım alacağınız birkaç yer bulunmakta. Bunlardan en erişilebilir olanı [Rust'ın Discord resmi kanalı](https://discord.com/invite/rust-lang) olan #beginners kanalıdır. Orada size yardımcı olabilecek diğer Rustaceans'larla (evet kendimizden bu saçma adla bahsediyoruz) çevrimiçi sohbet edip sorununuza çözüm bulabilirsiniz. Diğer harika kaynaklar arasında ise [Rust Kullanıcıları Forumu](https://users.rust-lang.org/) ile [Stack Overflow](https://stackoverflow.com/questions/tagged/rust) bulunmaktadır.

### Yerel Belgeler

Rust kurulumu, çevrimdışı okuyabilmeniz için belgelerin yerel bir kopyasını da içerir. Bu yerel belgeleri tarayıcınızda okuyabilmek için terminalinizde `rustup doc` komutunu çalıştırabilirsiniz. 

Standart kütüphane tarafından sağlanan bir tür veya işlev hakkında bilgi almak ve nasıl kullanılacağını öğrenmek istiyorsanız uygulama programlama arabirimi (API) belgelerini inceleyebilirsiniz. 
