### Hafıza Güvenliği
Rust'ın bir programlama dili olarak öne çıkmasının önemi tartışılırken, bellek güvenliğinin aslında ne
anlama geldiğini anlamak gereklidir. Özellikle, sistem programcılığına uygun olmayan veya
çoğunlukla çöp toplama mekanizmasına sahip dillerden edinilen tecrübeler ışığında Rust'un bu
temel özelliklerinin ayırdına varmak biraz zor olabilir.

Will Crichton'ın, **Rust' ta Bellek Güvenliği: C ile Bir Örnek Çalışma​** adlı önemli makalesinde
belirtildiği gibi: “Hafıza güvenliği, kullanılan işaretçilerin daima doğru tür/boyutta tahsis edilen geçerli
hafızaya işaret ettiği bir programlama özelliğidir. Güvensiz hafızaya sahip bir program hatalarına
bağlı olarak teknik olmayan çıktılar üretebileceği ya da kendiliğinden çökebileceğinden, hafıza
güvenliği bir doğruluk sorunudur.”
Haliyle bu ifadeden, uygulamada "hafıza güvenliği sunmadan" kod yazılmasına izin veren
programlama dillerinin varlığını öğreniyor ve güvensiz hafıza kullanımına yol açan hataları
tanımanın da kolay olduğunu anlıyoruz.

**Dangling Pointers:​** Geçersiz ya da silinmiş verileri gösteren işaretçiler. (Bu türden sorunlarla
karşılaşıldığında verinin bellekte nasıl depolandığına bakılması mantıklı olacaktır. [Daha fazla bilgi için](https://stackoverflow.com/questions/17997228/what-is-a-dangling-pointer)

**Double frees:** Aynı hafıza bölgesini iki kere boşaltmaya çalışarak ​ tanımsız davranış​ lara yol açmak
[Daha fazla bilgi için​](https://stackoverflow.com/questions/21057393/what-does-double-free-mean)

Güvensiz hafızaya yol açan hatalardan bazılarıdır. ​ Dangling pointer​ kavramını izah edebilmek için
aşağıda bulunan D kodunun hafızada nasıl temsil edildiğini inceleyelim.

```d
string s = "Have a nice day";
```

Dizgi olarak ilklendirilen bir değişkenin bellekte kullandığı stack ve heap bölümleri aşağıdakine
benzer biçimde gösterilir.

```bash
                      buffer
                    /   capacity
                  /   /    length
                /   /    /
            +–––+––––+––––+
stack frame │ • │ 16 │ 15 │ <– s
            +–│–+––––+––––+
              │
            [–│––––––––––––––––––––––––– capacity ––––––––––––––––––––––––––]
              │
            +–V–+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+
        heap │ H │ a │ v │ e │   │ a │   │ n │ i │ c │ e │   │ d │ a │ y │   │
            +–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+

            [––––––––––––––––––––––––– length ––––––––––––––––––––––––––]
```

Heap ve stack kavramlarının ne olduğuna gelmeden önce stack üzerinde depolanan verinin üç
kelimeden oluşan sabit boyutlu bir string nesnesi olduğunu bilmek önemlidir. Aşağıdaki alanlar ise
asıl verileri, arabellek kapasitesini, metnin uzunluğunu tutan ve heap tarafından ayrılan arabellek
için bir işaretçidir. Başka bir deyişle arabelleğinin sahibi **s adındaki**​ string nesnesidir. Bu nedenle
nesne program tarafından yok edildiğinde, string boyutu kadar tamponlanmış olan belleği de
türünün kendi yıkıcısı tarafından serbest bırakılacaktır.
Bununla birlikte, aynı tampon belleğe referans veren başka işaretçiler de olabilir. Nesnenin
yıkılmasıyla boşaltılan ve artık hafızada bulunmayan bu tampon belleğe halen işaret etmekte olan
bu işaretçilere **dangling pointers**​ adı verilmektedir.

Bu tür sorunlar bünyelerinde bulundurdukları çöp toplayıcı mekanizmaları sayesinde D ve go gibi yeni nesil dillerde ciddi sorunlara yol açmazlar. Bu tür programların çöp toplama mekanizmaları, programın işleyişi boyunca artık kullanılmayan ve hafızaya geri verilmesi gereken her şeyi çalışma zamanında otomatik olarak tespit edip sisteme geri iade ederler.  
Bir programlama dilinin bu tür bir mekanizmaya sahip olması şirin görünse bile, çöp toplama süreçlerinin derleme zamanında gerçekleştiriliyor olması, programın çalışma zamanı ve performansını etkiler.

Rust hafıza güvenliğini garanti altına almak için çöp toplama mekanizması kullanmak yerine  mülkiyet ve borçlanma sistematiği üzerinden bu sorunu çözüyor. O nedenle Rust dilinde hafıza güvenliğinden söz edildiğinde, bellek korumalı olmayan bir kodun yazılmasına derleyici tarafından izin verilmediği anlatılmak istenir.
