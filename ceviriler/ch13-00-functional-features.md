# İşlevsel dil özellikleri: Yineleyiciler ve kapamalar
Rust'un tasarımında mevcut birçok dil ve teknikten esinlenilmesinin dile en önemli katkısı işlevsel programlama olmuştur. İşlevsel programlamanın temelindeyse; işlevleri bağımsız değişkenlere geçirirerek değer olarak kullanmak, başka işlevlerden değer döndürmek, elde edilen değerleri daha sonra kullanmak üzere değişkenlere atamak gibi işlemler bulunur.   

Bu bölümde, işlevsel programlamanın ne olup olmadığını tartışmak yerine Rust'ın, işlevsel olarak adlandırılan birçok dildeki özelliklere benzer bazı özelliklerini tartışacağız.

Dahası özellikle şu konuları ele alacağız:

* *Kapamalar*, bir değişkene depolanabilen işlev benzeri yapılar
* *Yineleyiciler*, dizi öğelerini seri olarak işlemenin yolları
* Bu iki özeliğin, Bölüm 12'deki I/O projesini geliştirmek için kullanılması
* Ve bu özelliklerin performansı (Spoiler uyarısı: düşündüğünüzden daha hızlı!)

Diğer bölümlerde ele aldığımız **sıralamalar** ve **örüntü eşleme** gibi diğer Rust özellikleri de işlevsel programlama tekniklerinden etkilenirler. Kapamalar ve yineleyicilerde uzmanlaşmak, hızlı ve deyimsel Rust kodları üretmenin önemli bir parçası olduğundan bu bölümün tamamını bu konulara ayıracağız.
