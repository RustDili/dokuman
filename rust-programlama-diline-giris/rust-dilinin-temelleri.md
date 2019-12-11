# Rust Dilinin Temelleri
## Başlarken

Rust'ın İngilizce’de kelime anlamı pas'a karşılık gelir. Ancak dilin yaratıcısı, asla yok edilemeyen bir çeşit küf mantarından esinlenerek bu adı seçmiş. Günümüzün pek çok programlama dilinin olduğu gibi Rust’ın da **Ferris** adlı bir maskot yengeci var. 

Rust güvenli, eş zamanlı tasarlanabilen ve hızlı kod yazmayı sağlayan bir sistem programlama dilidir ve bütün bu hedeflerini çöp toplayıcı sistemi kullanmadan hafızayı güvende tutarak gerçekleştirir.  Dil bu hedeflere yönelik tasarlandığından kolaylıkla öğrenilebilme ve hemen proje üretebilmek gibi hedefleri gözetmiyor. Söz diziminin karışık olması kullanıcısının öğrenmesini zorlaştırırken, alt seviye programlama mantığına yakınlaştırıyor. 
Tasarımcısı Graydon Hoare’ye göre Rust dilinin hedef kitlesi: Sistem seviyesinde iş yaparken performans ve dağıtım özelliklerinden dolayı C++ seçmek zorunda kalan ancak daha güvenilir ve daha az yorucu bir seçenek arayan "Bıkmış usanmış C++ geliştiricileri" dir. 

Stack overflow’un 4 yıldır yaptığı anketlerde en sevilen programlama dili olarak seçilen Rust, **Swift** ve **Golang** ile birlikte yıldızı parıldayan dillerden. 

Ekosistemi dost canlısı ve yardımsever olmakla birlikte tepeden kontrollü bir dil olmak yerine demokratik yapıda; yani kullanıcılar RFC (Request For Comment) olarak herhangi bir değişiklik talep edebiliyorlar. Döküman içinde verilen alıştırma ve pratikleri **Documentation tests** aracılığıyla test edilebiliyor. Böylece dışarıya ürün verilirken dökümanların içindeki örnek kod bloklarının çalışıp çalışmadığı denetlenmiş oluyor. 

Fonksiyonel, prosedürel ve nesne yönelimli olması Rust’ı oldukça esnek yapıyor bu da kullancının programlama aşamasında daha az kısıtlanması demek. 

Üretilen binary kodlar olabildiğince küçük, debug modunda hazırlandığından olabildiğince ayrıştırılabilir. 

Dilin olumlu yanları dışında, yeni başlayanlar için kafa karıştırıcı bir söz dizimine sahip olması; **ownership**, **borrowing**, **lifetime** gibi yenilikçi kavramların anlaşılabilmesi için epey pratik gerektirmesi, standardizasyonunun dağınık olması ve derleme süresi olumsuz tarafları da var. Ancak bu dezavantajlar bazı hallerde avantaja dönebiliyor.

**Çöp toplayıcı** özelliğinin olmaması ve **Null Reference**'bulunmaması dilin en iyi yanlarından. **Rust** sistem programcılığının: **Bellek taşması**, **Hafıza sızıntısı**, **Tanımsız davranışlar**, **Dangling Pointer**, **Double Free**, **Format String Error**, **Array Bound Error** gibi dertleriyle de uğraştırmıyor.
