### Hafıza Güvenliği
Bir programlama dili olarak Rust'ın önemini tartışmadan önce, bellek güvenliğinin aslında ne anlama geldiğini anlamak gereklidir. Tecrübelerini; sistem programcılığına uygun olmayan veya çöp toplama mekanizmasına sahip diller aracılığıyla edinmiş programcılar için Rust'un temel özelliklerinin ayırdına varmak biraz zor olabilir.

Will Crichton'ın, **Rust' ta Bellek Güvenliği: C ile Bir Örnek Çalışma​** adlı önemli makalesinde belirtildiği gibi: *“Hafıza güvenliği, kullanılan işaretçilerin daima doğru tür/boyutta tahsis edilen geçerli hafızaya işaret ettiği bir programlama özelliğidir. Güvensiz hafızaya sahip bir program, hatalarına bağlı olarak teknik olmayan çıktılar üretebileceği ya da kendiliğinden çökebileceğinden, hafıza
güvenliği bir doğruluk sorunudur.”* 
Haliyle bu ifadeden, uygulamada **hafıza güvenliği sağlamadan** kod yazılmasına izin veren programlama dillerinin varlığını keçfediyor ve bunların aşağıdaki türden sorunlara neden olabileceğini öğreniyoruz.

**Dangling Pointers:​** Geçersiz ya da silinmiş verileri gösteren işaretçiler. (Bu türden sorunlarla karşılaşıldığında verinin bellekte nasıl depolandığına bakılması mantıklı olacaktır. [Daha fazla bilgi için](https://stackoverflow.com/questions/17997228/what-is-a-dangling-pointer)

**Double frees:** Aynı hafıza bölgesini iki kere boşaltmaya çalışarak *tanımsız davranışlara* yol açmak [Daha fazla bilgi için​](https://stackoverflow.com/questions/21057393/what-does-double-free-mean)

**Dangling pointer** kavramını izah edebilmek için aşağıda bulunan D kodunun hafızada nasıl temsil edildiğini inceleyelim.

```d
string s = "Have a nice day";
```

Dizgi olarak ilklendirilen bir değişkenin bellekte kullandığı `stack` ve `heap` bölümleri aşağıdakine benzer biçimde gösterilir.

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

`Heap` ve `stack` kavramlarının ne olduğuna gelmeden önce, `stack` üzerinde depolanan verinin üç kelimeden oluşan sabit boyutlu bir `string` nesnesi olduğunu söylememiz gerekir. Altındaki alanlar ise asıl verileri, arabellek kapasitesini, metnin uzunluğunu tutan ve heap tarafından ayrılan arabellek için bir işaretçidir. Başka bir deyişle arabelleğinin sahibi **s adındaki** `string` nesnesidir. Bu nedenle nesne program tarafından yok edildiğinde, string boyutu kadar tamponlanmış olan belleği de türünün kendi yıkıcısı tarafından serbest bırakılacaktır. Ara belleğin geri verilmesine rağmen, aynı tampon belleğe referans veren başka işaretçiler de olabilir. Nesnenin yıkılmasıyla boşaltılan ve artık hafızada bulunmayan bu tampon belleğe halen işaret etmekte olan bu işaretçilere **dangling pointers**  adı verilmektedir.

D ve go gibi yeni nesil diller bünyelerinde bulundurdukları çöp toplayıcı mekanizması sayesinde Bu tür sorunların üstesinden gelebilmektedirler. Çöp toplama mekanizmaları; artık kullanılmayan ve hafızaya geri verilmesi gereken her şeyi, çalışma zamanında tespit edip sisteme geri verirler. Çöp toplama mekanizmalarına sahip programlama dilleri her ne kadar şirin görünseler de çöp toplama işleri programın çalışma zamanı ve performansını etkiler.

Rust çöp toplama mekanizması kullanmak yerine, hafıza güvenliğini garanti altına alan **mülkiyet** ve **borçlanma** sistematiği üzerinden bu sorunları çözüyor. O nedenle **Rust’ta hafıza güvenliği** denildiğinde; *derleyicinin güvensiz kod yazılmasına izin vermeyeceği* anlatılmak istenir.
