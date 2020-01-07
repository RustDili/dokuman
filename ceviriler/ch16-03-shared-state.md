## Durum paylaşımlı eş zamanlılık
İş parçaları arasında mesajlaşarak iletişim kurmak, eş zamanlılık yönetiminin güzel bir yolu olmakla birlikte tek seçeneği bu değildir. Hatırlanacağı gibi Go dili dökümanlarında yer alan sloganın bir parçası "Hafızanın paylaştırılarak iletişim kurulması" tavsiyesinde bulunuyordu. 
Peki ama, iş parçaları arasında bellek paylaşımıyla veri aktarmak nedir ve neden bunun yerine mesajlaşma yöntemiyle veri paylaşımı daha fazla tercih edilmektedir?

Herhangi bir programlama dilinde kanallar, bir bakıma tek mülkiyetli gibidirler; yani bir kanala herhangi bir değer aktarıldığında bu değer bir daha kullanılmaz. Oysa bellek paylaşımlı eş zamanlılık, birden fazla mülkiyetin olması demektir ve bu da haliyle, çok sayıda iş parçası aynı anda aynı bellek konumuna erişmesi anlamına gelmektedir. Fakat mülkiyetin çok sayıda paydaşının olması, bu farklı paydaşların her birinin ayrı ayrı yönetilmesini gerektirdiğinden, karmaşıklığı da beraberinde getirir. Rust’ın tür sistemi ve mülkiyet kuralları, bu durumla başa çıkabilmeyi oldukça kolaylaştırır. 
Paylaşılan hafıza yönetiminde en yaygın eşzamanlılık ilkelerinden biri olan `mutex`' lere bakalım.

### Mutex kullanarak iş parçası verilerine aynı anda erişmek
Durum paylaşımlı eş zamanlılık
