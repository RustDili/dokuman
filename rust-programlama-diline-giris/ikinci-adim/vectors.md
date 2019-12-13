# Vektörler
Diziler aynı türden oluşan verileri liste halinde bir arada tutan sabit uzunluktaki değişmezlerdir. Onların bu değişmezliği `mut` anahtar sözcüğüyle tanımlanıyor olsalar bile boyutlarının değiştirlmesine yetmez. Dilimler ise dizinin belli bir bölümüne referans verdiklerinden boyutları pıda değişebilen biryutları değiştirilebilen listeler oluşturulabilmesini sağlayan veri yapılarına vektör denir.

Söz dizimi Vec<T> olarak ifade edilen öbekte T herhangi bir veri türünü temsil eder. Eğer i32 türünde bir vektör ifade edilecekse bu basitçe Vec <i32> olarak gösterilir. 
Ayrıca vektöre ait veriler, o vektör için heap üzerinde özel olarak ayrılmış dinamik bir alanda tutulurlar.  
