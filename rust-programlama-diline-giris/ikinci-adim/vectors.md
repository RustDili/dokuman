# Vektörler
Diziler aynı türden oluşan verileri liste halinde bir arada tutan sabit uzunluktaki değişmezlerdir. Onların bu değişmezliği `mut` anahtar sözcüğüyle tanımlanıyor olsalar bile boyutlarının değiştirlmesine yetmez. Dilimler ise dizinin belli bir bölümüne referans verdiklerinden boyutları dinamik olarak değişebilen müstakil veri yapıları değildirler. Rust'ta, aynı türden öğelere sahip ve boyutları değiştirilebilen listeler oluşturulabilmesini sağlayan veri yapılarına vektör denir.

`Vec<T>` söz dizimiyle ifade edilirler. Söz diziminde yer alan `T` depolanacak veri türünü temsil eder. Örneğin `i32` türünde bir vektör basitçe `Vec<i32>` olarak ifade edilir. 
Vektörler boyutları dinamik olarak değişebilen veri türleri olduklarından depoladıkları öğelere ait veriler, `heap` üzerinde kendisi için  özel olarak ayrılmış dinamik bir alanda tutulurlar.
### Boş bir vektör oluşturmak
