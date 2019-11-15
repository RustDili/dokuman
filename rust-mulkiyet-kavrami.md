![Image of Yaktocat](resimler/mulkiyet-kavrami-kapak.png)

# Rust'ın Mülkiyet Kavramı

Rust programlama dilini diğer dillerden ayıran özelliklerin başında ownership, borrowing ve lifetime
yani dilimizdeki karşılıklarıyla **mülkiyet**​
, **borçlanma​** ve **yaşam**​ olan bu üç kavram geliyor.
Dilin temelini oluşturan bu üç kavramının bu dili kullanan programcılar için geleneksel programlama
yaklaşımını tamamen değiştirdiğini ifade edebiliriz. Öyle ki; diğer programlama dillerinin bakış
açılarına hakim olan programcıların mantıklı bulduğu kodlar Rust ile denendiğinde genellikle
çalışmaz.

Rust dilinde mülkiyet, öylesine önemli bir kavramdır ki, bu kavramı mümkün olduğunca erken
anlamak, derleyici hatalarını değerlendirmek ve dili daha çabuk kavramak açısından oldukça
yararlıdır. Ancak Rust’ın bu kavramları verilerin hafızada depolanma şekli ve hafıza güvenliği ile
doğrudan ilişkili olduğundan kavramların doğru ve tutarlı anlaşılabilmesi için öncelikle bu konuları
incelenmesi oldukça önemlidir.

## Hafıza Güvenliği

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

```d
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

## Hafızanın Bölümleri: Stack ve Heap

Rust' un mülkiyet kavramını ele alış şekline girmeden önce, hafızanın stack ve heap olarak adlandırılan bölümlerinin ne olduğuna ve bu bölümlerde ne tur verilerin nerede depolandığına hızlı bir şekilde göz atalım. Aynı belleğin parçaları olmalarına rağmen stack ve heap farklı veri yapılarıyla temsil edilirler. Hafızada verilerin girildiği sırayla depolandığı ancak ters sırada kaldırıldığı (lifo:  last in first out) ve çok hızlı çalışan bölümüne stack adı verilirken, okuma ve yazma için daha fazla hesaplama gerektiren  ve bir ağaç yapısını andıran bölümüne de heap adı verilir.

Hafızanın bu bölümlerinde neler olduğu ise üzerinde çalışılan verilerle ilgilidir. Rust’ ta tam sayılar, kayan noktalı sayılar, işaretçi tipleri gibi boyutları sabit ya da derleme zamanında bilinen türler stack bölümünde depolanır. Derleme zamanında boyutları dinamik olarak değişebilen ya da hiç bilinmeyen türler ise silinirlerken özel bir temizlik çalışması gerektirdiğinden heap bölümüne konulur.
O nedenle önceki örnekte bulunan string nesnesinin kendisi; kapasite, uzunluk ve ara bellek işaretçi değerleri değişmez ve sabit boyutta olduğundan stack üzerinde, sahip olduğu ham verileri için kendisine ayrılan ara bellek ise heap üzerinde depolanır. D gibi programlama dillerinin çoğunda hafıza işlemleri bu şekilde yürütülürken Rust heap bölümünde veri depolamaktan kaçınır ve mülkiyet kavramı konusunda da ele alınacağı gibi Box gibi belirli işaretçi türlerini kullanıma sokar.

## Mülkiyet Kavramını Anlamak

Çöp toplayıcısına sahip olmayan düşük seviyeli dillerde değişkenler için ayrılan kaynakların hafızaya iade edilmesinden programcının kendisi sorumludur. Programın akışı içerisinde programcı, işlemlerinde kullandığı kaynaklarla işini bitirdiğinde  ya da programın o kaynağa ihtiyacı olmadığını düşündüğünde kaynağı serbest bırakarak kullanılan bellek alanını iade etme yolunu seçer. C dilinde işaretçi kullanılarak böyle bir örnek verilmek istendiğinde şu şekilde bir kodun yazılması gerekir.

```c
    int main(int argc, const char * argv[]) {
        char *str;
        str = (char *) malloc(10); // Bellekte 10 birimlik yer ayırıyoruz.
                              // str ile bir takım işlemler...

        free(str); // str pointer'ını ve kullandığı kaynağı serbest bırakıyoruz.

        return 0;
    }
```

Bu basit örnekte; str adında bir char işaretçisi tanımlanarak ona hafızadan 10 birimlik bellek alanı ayrılmış, kodun str işaretçisine ihtiyaç duyulmayan bölümüne gelindiğinde free() işlevi yardımıyla ayrılan o kaynak sisteme geri verilmiştir.

Normal görünen bu kodda yer alan iade işleminin herhangi bir nedenle unutulması ayrılan kaynağın programı işleten sisteme geri verilmemesine neden olur ve bu kaynak sistem yeniden başlatılana kadar hafızada kalmaya devam eder. Böyle bir durumda hafızanın kullanılan o bölümü süreç boyunca sistem ihtiyaçları için kullanılamayacağından program bellek sızıntısı olarak tanımlanan soruna neden olacaktır.

Bu sorun iade edilmesi gereken bellek miktarı arttıkça ciddi oranda kaynak savurganlığına neden olacak, performans kaybından programın donmasına kadar giden sorunları beraberinde getirecektir.
Çöp toplayıcısı olan dillerde ise bu sorun otomatik süreçler tarafından yönetilip yürütüldüğünden programlama sürecinde büyük bir dikkat ve hassasiyet gerektirmez.

Toparlarsak programlama dillerinin bazıları çalışabilmeleri için sistemden aldıkları kaynakların geri verilmesi sorumluluğunu programcıya, bazıları otomatik mekanizmaları aracılığıyla kendi sorumluluğunda bulunan çöp toplayıcı mekanizmalarına verirken; Rust bu sorumluluğu mülkiyet kavramı üzerinden hafızayı elinde tutanın sorumluluğuna bırakılmıştır.

Yani bu Rust’ın bir çöp toplama mekanizmasının bulunmadığı gibi, bellek yönetimi konusunda da programcısını zorlamadığı anlamına gelir.  Rust bellek yönetimi ve güvenliğini hiç bir çöp toplama mekanizmasına ihtiyaç duymadan mülkiyet kavramı içinde kümelediği kurallar aracılığıyla sağlar.

Bir nesnenin sahibi olmak, o nesnenin yaşam hakkının yalnızca sahibinin elinde olduğu anlamına gelir.

## Yaşam Alanı ve Süresi
