## Merhaba, Dünya

Artık Rust'ı yüklediğimize göre ilk Rust programımızı yazabiliriz. Yeni bir programlama dilini öğrenme aşamasında `Merhaba, dünya!` çıktısını ekrana yazdırmak neredeyse gelenek haline geldiğinden biz de burada bu geleneğe uyacağız.

> Not: Bu kitap komut satırı hakkında temel düzeyde bilgi sahibi olduğunuzu varsayar. Bununla birlikte Rust,
> kodlarınızı nasıl düzenleyeceğinize, hangi araçları kullanacağınıza veya onları nereye kaydedeceğinize 
> karışmadığından, dilerseniz komut satırı yerine aşina olduğunuz entegre geliştirme ortamınızı (IDE) kullanabilirsiniz. 
> Son zamanlarda Rust ekibi farklı IDE'ler ile entegrasyonu iyileştirmeye odaklandığından artık birçok IDE belli
> düzeylerde dil desteği sağlıyor. Tercih ettiğiniz IDE'nin yeterli dil desteği sağlayıp sağlamadığını IDE belgelerinden 
> kontrol edebilirsiniz.

### Bir Proje Dizini Oluşturmak

Öncelikle işe Rust kodlarımızı kaydedeceğimiz bir proje dizini oluşturarak başlayalım. Rust için kodunuzu nerede sakladığınız önemli olmamakla beraber, bu kitapta yer alan alıştırma ve projeler için ana dizininizde (linux için Home) dizininizde yeni bir proje dizini oluşturup tüm çalışmalarınızı orada depolamanızı öneririz.

Ana dizin üzerinde "Merhaba, dünya" projesinin saklanacağı dizi *projeler* dizinin oluşturabilmek için bir terminal penceresi açarak sırasıyla aşağıdaki komutları girin.

Bu komutlar Linux, macOS ve Windows üzerindeki PowerShell için çalışmaktadır:

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

Artık *merhaba_dunya* dizini içinde bulunduğumuza göre, programın kaynak kodunu kaydedeceğimiz ve *main.rs* olarak adlandıracağımız yeni bir dosya oluşturabiliriz. Rust dosya adlarının birden fazla kelime içermesi durumunda alt çizgi ile ayrılması gerektiğini ve daima .rs uzantısıyla sonlandığını unutmayın. Dizin veya dosya isimlerinin bitişik olarak yazılması mümkünse de bu yaklaşımın pek önerilmediğini, *merhabadunya.rs* yerine *merhaba_dunya.rs* biçimindeki adlandırmaların tercih edildiğini aklınızda bulundurun. 

Şimdi biraz önce oluşturduğumuz *main.rs* dosyasını açarak Örnek 1-1'de yer alan kod satırlarını dosyamıza ekleyelim:

<span class="filename">Dosya adı: main.rs</span>

```rust
fn main() {
	println!("Merhaba, dünya!");
}
```

<span class="caption"> Örnek 1-1: Ekrana "Merhaba, dünya!" yazdıran bir program.</span>

Dosyayı kaydedip yeniden terminal penceresine dönerek programınızı Linux veya macOS'ta derleyip çalıştırabilmek için aşağıdaki komutları girin:

```console
$ rustc main.rs
$ ./main
Merhaba, dünya!
```

Windows kullanıyorsanız `./main` yerine `./main.exe` komutunu kullanın:

```powershell
> rustc main.rs
> .\main.exe
Merhaba, dünya!
```

Hangi işletim sistemini kullanıyor olursanız olun, komutları uyguladıktan sonra terminalinizde `Merhaba, dünya!` çıktısını görüyor olmalısınız. Bu çıktıyı görmüyorsanız, yardım için kurulum bölümündeki ["Sorun Giderme"](ch01-01-installation.html#troubleshooting) başlığına göz atın.

Eğer ekranınızda `Merhaba, dünya!` yazısını görüyorsanız bu resmi olarak bir Rust porgramı yazdığınızı gösterir, tebrikler! Siz de artık bir Rust programcısısınız, aramıza hoş geldiniz!

### Bir Rust Programının Anatomisi

Ekrana `Merhaba, dünya!` yazdıran programınızda neler olup bittiğine daha yakından bakalım. İşte bulmacanın ilk parçası:

```rust
fn main() {
#	println!("Merhaba, dünya!");	

}
```

Bu satırlar Rust'ta `main` adındaki bir işlevi tanımlar. Ve `main` işlevi tüm çalıştırılabilir Rust programlarında işletilen ilk kod olduğundan özeldir. Bu işlevin ilk satırında, parametre almayan ve hiçbir şey döndürmeyen `main` adında bir işlevi bildirilir. Eğer bu işleve bir veya birden fazla parametre iletilmiş olsaydı, bu parametreler `()` parantezin içinde yerlerini almış olacaklardı.

Eğer dikkat ederseniz işlev gövdesinin süslü parantezler `{}` içine alınmış olduğunu göreceksiniz. Rust'ta işlev gövdeleri bu süslü parantezler içine alınmak zorundadır. İşlev gövdesini saran ilk süslü parantezi, işlev bildirimiyle aynı satıra yerleştirirken arada bir boşluk bırakmak iyi bir kod yazım tekniğidir. Rust projelerinde standart kod yazım tekniğine  bağlı kalmak ve kodlarınızı belirli bir şekilde biçimlendirmek için `rustfmt` adındaki otomatik biçimlendirme aracını kullanabilirsiniz. Bu araç Rust ekibi tarafından tıpkı `rustc` gibi standart Rust dağıtımına dahil edildiğinden halihazırda bilgisayarınızda kurulu durumda olmalıdır. Daha fazla ayrıntı için çevrimiçi belgelere başvurabilirsiniz. 

Gövdesi süslü parantezler ile sarmalanmış olan `main` işlevinin içinde aşağıdaki kod satırı bulunur.

```rust
#fn main() {
	println!("Merhaba, dünya!");
#}
```
Bu küçük programdaki tüm işi yaparak metni ekrana yazdıran bu satır olmakla birlikte burada dikkat edilmesi gereken dört önemli ayrıntı yer almaktadır.

Bunlardan birincisi ve en az dikkat edileni, Rust stili girintilerde bir sekme (tab) yerine dört boşluk (space)kullanılmasıdır.

İkincisi `println!` ile bir Rust makrosu çağırılmaktadır. Eğer kodda Rust makrolarından yararlanmak yerine bir işlev çağrısı yapılmak istenseydi, yazdırma işlevi `!` olmadan sadece `println` şeklinde kullanılmış olacaktı. Rust makrolarını 19. bölümde ayrıntılarıyla inceleyeceğimizden şu an için `!` işaretini gördüğünüz her yerde bunun bir işlev çağrısı değil bir Rust makrosu olduğunu bilmeniz yeterlidir.

Üçüncüsü `"Merhaba, dünya!"` olarak gördüğünüz dizgiyi `println!` makrosuna argüman olarak geçirdiğimizde bu makro sayesinde ekrana yazdırılır.

Dördüncüsü ve son olarak bu satırın noktalı virgül `;` ile bittiğine dikkat edin. Satırın bununla bitirilmesi, ifadenin artık sona erdiğini ve sonraki ifadenin yeniden başlatılabileceği anlamına gelmektedir. Rust kodlarındaki satırların çoğu noktalı virgül ile sonlandırılır.

### Derlemek ve Çalıştırmak Ayrı Birer Adımdır

Yeni oluşturulan bir programın çalıştırılma sürecindeki adımları birlikte inceleyelim.

Rust programları çalıştırılmadan önce Rust derleyicisi tarafından ve rustc komutuna aşağıdaki gibi kaynak dosyası adı ileterek derlenmelidir:

```console
$ rustc main.rs
```

Eğer C veya C++ dillerine aşina iseniz, bu işlemin `gcc` veya `clang` ile oldukça benzeştiğini fark edeceksiniz. Başarıyla gerçekleşen bir derlemenin ardından Rust çalıştırılabilir ikili bir dosya üretecektir.

Bu çalıştırılabilir dosyayı, Linux, macOS veya Windows PowerShell üzerinde görüntüleyebilmek için `ls` komutunu kullanabilirsiniz. Linux ve macOS sistemlerinde aynı dizinde iki adet dosya gösterilirken, Windows'ta PowerShell veya CMD ile üç tane dosya görüntülenecektir.

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

Her iki durumdada .rs uzantılı bir kaynak kodu dosyası, windows'ta main.exe olarak ancak diğer platformlarda sadece main olarak görünen çalıştırılabilir ikili dosya, Windows için ek olarak hata ayıklama bilgilerini içeren .pdb uzantılı birer dosya gösterilecektir, 

Bu dizinde çalıştırılabilir halde bulunan main ya da main.exe dosyasını aşağıdaki gibi çalıştırabilirsiniz:

```console
$ ./main # ya da windows için .\main.exe
```

Eğer programınız "Merhaba, dünya!" metnini içeriyorsa bu komutu girdiğinizde terminalinizde `"Merhaba, dünya!"` yazdırılacaktır.

Programlama tecrübeniz Ruby, Python veya JavaScript gibi dinamik dillerden oluşuyorsa bu programın ayrı adımlar halinde derleyip çalıştırılmasına alışkın olmayabilirsiniz. Ancak Rust *derlemeli bir dil*dir. Bu da bir Rust programının derlenmesiyle oluşturulan çalıştırılabilir dosyanın paylaşabilmesi ve Rust'ın kurulu olmadığı ortamlarda bile çalışması demektir. Oysa dinamik dillerden herhangi birinin kaynağını *.py, .rb veya .js* dosyası olarak biriyle paylaştığınızda bu dosyaların çalıştırılabilmesi, için o ortamda Python, Ruby ya da JavaScript uygulamalarının kurulu olması gerekmektedir.  Bununla birlikte bu dillerde de programın çalıştırılması için yalnızca tek bir komut yeterlidir. Dil tasarımlarında bulunan her şey bir uzlaşma meselesinden başka bir şey değildir.  

Programları `rustc` ile derlemek basit programlar için yeterli olmakla birlikte projeniz büyüyüp geliştikçe seçenekleri yönetmek, kod paylaşımını kolaylaştırmak isteyeceksiniz. Sonraki bölümde daha gerçekçi ve karmaşık Rust programlarını yazmanıza yardımcı olacak Cargo aracını keşfedeceğiz. 
