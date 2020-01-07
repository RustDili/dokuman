## Durum paylaşımlı eş zamanlılık
İş parçaları arasında mesajlaşarak iletişim kurmak, eş zamanlılık yönetiminin güzel bir yolu olmakla birlikte tek seçeneği bu değildir. Hatırlanacağı gibi Go dili dökümanlarında yer alan sloganın bir parçası "Hafızanın paylaştırılarak iletişim kurulması" tavsiyesinde bulunuyordu. 
Peki ama, iş parçaları arasında bellek paylaşımıyla veri aktarmak nedir ve neden bunun yerine mesajlaşma yöntemiyle veri paylaşımı daha fazla tercih edilmektedir?

Herhangi bir programlama dilinde kanallar, bir bakıma tek mülkiyetli gibidirler; yani bir kanala herhangi bir değer aktarıldığında bu değer bir daha kullanılmaz. Oysa bellek paylaşımlı eş zamanlılık, birden fazla mülkiyetin olması demektir ve bu da haliyle, çok sayıda iş parçası aynı anda aynı bellek konumuna erişmesi anlamına gelmektedir. Fakat mülkiyetin çok sayıda paydaşının olması, bu farklı paydaşların her birinin ayrı ayrı yönetilmesini gerektirdiğinden, karmaşıklığı da beraberinde getirir. Rust’ın tür sistemi ve mülkiyet kuralları, bu durumla başa çıkabilmeyi oldukça kolaylaştırır. 
Paylaşılan hafıza yönetiminde en yaygın eşzamanlılık ilkelerinden biri olan mutex' lere bakalım.

### Mutex kullanarak iş parçası verilerine aynı anda erişmek
Mutex; tuttuğu verilere aynı anda sadece bir iş parçasının erişmesine izin veren, -İngilizce'deki "Mutual exclusion" sözcüklerinin- karşılıklı dışlama mekanizmasının kısaltılmış adıdır. Herhangi bir iş parçasının mutex içindeki verilere erişebilmesi için, erişim isteğinde bulunarak mutex kilidini talep etmesi gerekir. Talep edilen kilit, o an için verilere özel erişim izni olan ve iş parçalarını takip eden mutex mekanizmasına ait olan veri parçasıdır. Kısaca "mutex, içindeki verileri kilitleme mekanizması aracılığıyla korur" diyebiliriz.

* Mutex kullanımı, aşağıdaki kuralların hatırlanması gerekli olduğundan kullanılması oldukça zor olarak bilinir:
  * Verileri kullanmaya başlamadan önce kilidi alınmalıdır.
  * Mutex tarafından korunan verilerin kulanımı sona erdiğinde, kilidin diğer iş parçalarının kullanımına açılması gereklidir.

Mutex’lerin gerçek dünyada neye benzediğini kavrayabilmek için, kalabalık bir grup ve yalnızca bir mikrofon ile gerçekleştirilen bir panel hayal edin: Paneldeki her tartışmacı, konuşmaya başlamadan önce, mikrofonu kullanmak istediklerini bildirmeli veya işaret etmelidir. Ve tartışmacılar sadece mikrofonu aldıklarında konuşabilir, bitirdiklerinde mikrofonu konuşmak isteyen bir sonraki tartışmacıya verebilirler. Ancak konuşanlardan biri işi bittiğinde mikrofonu elinde unutur ya da başka bir tartışmacıya geri vermezse artık hiç kimse konuşamaz. Mikrofon paylaşımı aksadığındaysa tartışma planlandığı gibi ilerlemez. 
