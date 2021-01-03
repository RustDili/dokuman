## Merhaba, Dünya

Artık Rust'ı yüklediğimize göre ilk Rust programımızı yazabiliriz. Yeni bir programlama dilini öğrenme aşamasında `Merhaba, dünya!` çıktısını ekrana yazdırmak neredeyse gelenek haline geldiğinden biz de burada bu geleneğe uyacağız.

> Not: Bu kitap komut satırı hakkında temel düzeyde bilgi sahibi olduğunuzu varsayar. Bununla birlikte Rust,
> kodlarınızı nasıl düzenleyeceğinize, hangi araçları kullanacağınıza veya onları nereye kaydedeceğinize 
> karışmadığından, dilerseniz komut satırı yerine aşina olduğunuz entegre geliştirme ortamınızı (IDE) kullanabilirsiniz. 
> Son zamanlarda Rust ekibi farklı IDE'ler ile entegrasyonu iyileştirmeye odaklandığından artık birçok IDE belli
> düzeylerde dil desteği sağlıyor. Tercih ettiğiniz IDE'nin yeterli dil desteği sağlayıp sağlamadığını IDE belgelerinden 
> kontrol edebilirsiniz.

### Bir Proje Dizini Oluşturmak

Öncelikle işe Rust kodlarımızı saklayacağımız bir proje dizini oluşturarak başlayalım. Rust için kodunuzu nerede sakladığınız önemli olmamakla beraber, bu kitapta yer alan alıştırma ve projeler için ana dizininizde (linux için Home)yeni bir *projeler* dizini oluşturup tüm çalışmalarınızı orada depolamanızı öneririz.

Ana dizinde "Merhaba, dünya" projesinin saklanacağı *projeler* dizinin oluşturabilmek için bir terminal penceresi açarak sırasıyla aşağıdaki komutları uygulayalım.

Linux, macOS ve Windows PowerShell için aşağıdaki komutları girin:

```console
$ mkdir ~/projeler
$ cd ~/projeler
$ mkdir merhaba_dunya
$ cd merhaba_dunya
```

Windows CMD içinse şu komutları girin:

```cmd
> mkdir "%USERPROFILE%\projeler"
> cd /d "%USERPROFILE%\projeler"
> mkdir merhaba_dunya
> cd merhaba_dunya
```

### Bir Rust Programı Yazmak ve Çalıştırmak

Artık *merhaba_dunya* dizini içinde bulunduğumuza göre, programın kaynak kodunu kaydedeceğimiz ve *main.rs* olarak adlandıracağımız yeni bir dosya oluşturabiliriz. Rust'ta dosya adlarının birden fazla kelime içermesi durumunda alt çizgi ile ayrılması gerektiğini ve daima .rs uzantısıyla sonlandığını unutmayın. Dizin veya dosya isimlerinin bitişik olarak yazılması mümkünse de bu yaklaşımın pek önerilmediğini, *merhabadunya.rs* yerine *merhaba_dunya.rs* biçimindeki adlandırmaların tercih edildiğini aklınızda bulundurun. 

Şimdi biraz önce oluşturduğumuz *main.rs* dosyasını açarak Örnek 1-1'de yer alan kod satırlarını dosyamıza ekleyelim:

<span class="filename">Dosya adı: main.rs</span>

```rust
fn main() {
	println!("Merhaba, dünya!");
}
```

<span class="caption"> Örnek 1-1: Ekrana "Merhaba, dünya!" yazdıran bir program.</span>

Dosyayı kaydedip yeniden terminal penceresine dönelim. Programımızı Linux veya macOS üzerinde derleyip çalıştırabilmek için aşağıdaki komutları uygulayalım:

```console
$ rustc main.rs
$ ./main
Merhaba, dünya!
```

Windows kullanıyorsanız `./main` yerine `./main.exe` komutunu kullanmanız gerekir:

```powershell
> rustc main.rs
> .\main.exe
Merhaba, dünya!
```

Hangi işletim sistemini kullanıyor olursanız olun, bu komutları uyguladıktan sonra terminalinizde `Merhaba, dünya!` çıktısını görüyor olmalısınız. Bu çıktıyı görmüyorsanız, yardım için kurulum bölümündeki ["Sorun Giderme"](ch01-01-installation.html#troubleshooting) başlığına göz atın.

Eğer ekranınızda `Merhaba, dünya!` yazısı ışıldıyorsa bu resmi olarak bir Rust programı yazdığınıza işarettir, tebrikler! Siz de artık bir Rust programcısı sayılırsınız ve aramıza hoş geldiniz!

### Bir Rust Programının Anatomisi

Ekranımıza `Merhaba, dünya!` yazısını bastıran programda neler olup bittiğine daha yakından bakalım. Bulmacanın ilk parçası aşağıdadır:

```rust
fn main() {
#    println!("Merhaba, dünya!");	

}
```

Bu satırlar Rust'ta `main` adındaki bir işlevi tanımlar. Ve `main` işlevi tüm çalıştırılabilir Rust programlarında işletilen ilk kod olduğundan özeldir. Bu işlevin ilk satırında, parametre almayan ve hiçbir şey döndürmeyen `main` adında bir işlevi bildirilir. Eğer bu işleve bir veya birden fazla parametre iletilmiş olsaydı, bu parametreler `()` parantezin içinde yerlerini almış olacaklardı.

Rust'ta bu satırlar bir işlevi tanımlar. Çalıştırılabilir tüm rust programlarında bulunan `main` işlevi, programın işletilen ilk kodu olması bakımından özel bir konumdadır. İlk satır parametre almayan ve hiçbir şey döndürmeyen işlev adını `main` olarak bildirir. Elimizde işleve iletilecek parametreler olsaydı onlar da yerlerini bu `()` parantezin içine konumlandırılmakla bulmuş olacaklardı.

Dikkat ederseniz işlev gövdesinin süslü parantezler `{}` içine alınmış olduğunu göreceksiniz. Rust'ta işlev gövdeleri bu süslü parantezler içine alınmak zorundadır. İşlev gövdesini saran ilk süslü parantezi, işlev bildirimiyle aynı satıra yerleştirirken arada bir boşluk bırakmak iyi bir kod yazım tekniğidir. Rust projelerinde standart kod yazım tekniğine  bağlı kalmak ve kodlarınızı belirli bir şekilde biçimlendirmek için `rustfmt` adındaki otomatik biçimlendirme aracını kullanabilirsiniz. Bu araç Rust ekibi tarafından tıpkı `rustc` gibi standart Rust dağıtımına dahil edildiğinden halihazırda bilgisayarınızda kurulu durumda olmalıdır. Daha fazla ayrıntı için çevrimiçi belgelere başvurabilirsiniz.

Gövdesi süslü parantezler ile sarmalanmış olan `main` işlevinin içinde aşağıdaki kod satırı bulunur.

```rust
#fn main() {
	println!("Merhaba, dünya!");
#}
```
Bu küçük programdaki tüm işi üstlenerek metni ekrana yazdıran bu satırda dikkat edilmesi gereken dört önemli ayrıntı yer almaktadır.

İlki ve en az dikkat edileni, Rust stili girintilerde bir sekme *(tab)* yerine dört boşluk *(space)* kullanılır.

İkincisi, `println!` bir Rust makrosu çağırır. Eğer kodda bir işlev çağrısı yapılsaydı, `println` (`!` olmadan) yazılmış olacaktı. Rust makrolarını 19. bölümde ayrıntılarıyla inceleyeceğimizden şu an için `!` işaretini gördüğünüz her yerde bunun bir işlev yerine, bir makroya yapılan çağrı olduğunu bilmeniz yeterlidir.


Üçüncüsü, `"Merhaba, dünya!"` olarak gördüğünüz dizgi, `println!` makrosuna argüman olarak geçirildiğinde ekrana yazdırılır.

Dördüncüsü, satırı noktalı virgül (`;`) ile bitiriyor olmamız ifadenin bittiğini ve bir sonrakinin başlayabileceğini bildirir. Rust kodlarındaki pek çok satır noktalı virgül ile sonlandırılır.

### Derlemek ve Çalıştırmak Ayrı Birer Adımdır

Yeni oluşturulan bir programın çalıştırılma süreci adımlarını birlikte inceleyelim.

Rust programları çalıştırılmadan önce Rust derleyicisi tarafından ve `rustc` komutuna aşağıdaki gibi kaynak dosyası adı ileterek derlenmelidir:

```console
$ rustc main.rs
```

Eğer C veya C++ dillerine aşina iseniz, bu işlemin `gcc` veya `clang` ile oldukça benzeştiğini fark edeceksiniz. Başarıyla gerçekleşen bir derlemenin ardından Rust çalıştırılabilir ikili (binary) bir dosya üretecektir.

Bu çalıştırılabilir dosyaya, Linux, macOS veya Windows PowerShell sistemlerinde, dizin içindeyken terminalinize `ls` komutu girerek ulaşabilirsiniz. Linux ve macOS sistemlerinde aynı dizinde iki adet dosya gösterilirken, Windows'ta PowerShell veya CMD ile üç tane dosya görüntülenecektir.

```text
$ ls
main main.rs
```

Eğer Windows üzerinde CMD kullanıyorsanız aşağıdaki komutu girmeniz gereklidir:

```cmd
> dir /B %= Buradaki /B seçeneği yalnızca dosya isimlerinin görüntülenmesini sağlar =%
main.exe
main.pdb
main.rs
```

Her iki durumda da *.rs* uzantılı bir kaynak kodu dosyası, (windows'ta *main.exe* olarak ancak diğer platformlarda sadece *main* olarak görünen) çalıştırılabilir ikili dosya, Windows için ek olarak hata ayıklama bilgilerini içeren *.pdb* uzantılı birer dosya gösterilecektir.

Bu dizinde çalıştırılabilir halde bulunan *main* ya da *main.exe* dosyasını aşağıdaki gibi kullanarak işletebilirsiniz:

```console
$ ./main # ya da windows için .\main.exe
```

Eğer programınız "Merhaba, dünya!" metnini içeriyorsa bu komutu girdiğinizde terminalinizde `"Merhaba, dünya!"` yazdırılacaktır.

Programlama tecrübeniz Ruby, Python veya JavaScript gibi dinamik dillerden oluşuyorsa bu programın ayrı adımlar halinde derleyip çalıştırılmasına alışkın olmayabilirsiniz. Ancak Rust *derlemeli bir dil*dir. Bu da bir Rust programının derlenmesiyle oluşturulan çalıştırılabilir dosyanın paylaşabilmesi ve Rust'ın kurulu olmadığı ortamlarda bile çalışması demektir. Oysa dinamik dillerden herhangi birinin kaynağını *.py, .rb veya .js* dosyası olarak biriyle paylaştığınızda bu dosyaların çalıştırılabilmesi, için o ortamda Python, Ruby ya da JavaScript uygulamalarının kurulu olması gerekmektedir.  Bununla birlikte bu dillerde de programın çalıştırılması için yalnızca tek bir komut yeterlidir. Dil tasarımlarında bulunan her şey bir uzlaşma meselesinden başka bir şey değildir.  

Programları `rustc` ile derlemek basit programlar için yeterli olmakla birlikte projeniz büyüyüp geliştikçe seçenekleri yönetmek ve kod paylaşımını kolaylaştırmanın yollarını arayacaksınız. Sonraki bölümde gerçek dünyada daha sık kullanılan ve daha karmaşık Rust programları yazmanıza yardım edecek olacak Cargo aracını keşfedeceğiz. 
