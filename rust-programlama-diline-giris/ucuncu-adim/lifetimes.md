# Yaşam süreleri
Referanslarla çalışırken onları kullanmayı bırakana kadar referans verisinin canlı kaldığından emin olmalıyız. 

Şöyle ki; 
  - Bir değerle bağladığımız 'a' adında bir değişkenimiz var.
  - Bu `a` değişkenini `x` adındaki başka bir değişkene referans toluyla bağladığımız takdirde `x` değişkeniyle işimiz bitene kadar `a` değişkeninin yaşadığından emin olmamız gerekir. 
  
>🔎 Bellek yönetimi, bilgisayar belleğine uygulanan bir kaynak yönetimi şeklidir. Programlama dillerinin çoğu 1990'ların ortasına kadar, programcının kullanılmayan nesneleri tanımlamak veya serbest bırakabilmesi için talimatlar vermesini gerektiren **Elle Bellek Yönetimi**'ni kullanıyordu. 1959'da John McCarthy, bir **Otomatik Bellek Yönetimi** (AMM) şekli olan **Çöp Toplayıcı** sistemini *(GC)* icat etti. Bu sistem programlayıcıya güvenmek yerine hangi belleğin artık kullanılmayacağını belirler ve o belleği otomatik olarak serbest bırakır. **Objective-C ve Swift** dilleri de, **Otomatik Referans Sayma** (ARC) adı verilen benzer bir işilevsellik sunarlar.

## Yaşam Süresi Nedir?
Rust dilinde,

  * Aynı anda bir kaynağın yalnızca **tek sahibi** bulunur. Bu durum **kapsam dışına çıkıldığında** biter ve kaynak bellekten kaldırılır.
  * Aynı kaynağı tekrar kullanmak istediğimizde onu içeriğinden **referans** yoluyla **ödünç** alıyoruz.
  * **Referanslar** ile çalışırken **derleyicinin** başvurulan kaynakları **ne zamana kadar** kullanılabileceğini anlayabilmesi için bu kaynakların **yaşam sürelerini** bildiren **ek talimatlar** verilmesi gerekir. 
  * ⭐ Ancak bu ek talimatlar **kodu daha ayrıntılı** hale getireceğinden **ortak kalıpları** daha ergonomik hale getirmek için yaşam sürelerinin `fn` tanımlarında **elenmesine veya ihmal edilmesine** izin verilir. Bu durumda, derleyici ömür boyu ek açıklamaları dolaylı olarak atar.

Yaşam süreleri ek açıklamaları **derleme zamanında** denetlenir. Hafıza ise, verinin ilk ve son olarak ne zaman kullanıldığı derleyici tarafından kontrol edildikten sonra **çalışma zamanında** yönetilir. Rust derlemelerinin yavaş olma nedeni budur.

> * C ve C++ 'dan farklı olarak, Rust **genelde** değerleri açıkça düşürmez.
> * GC'den farklı olarak Rust, referans alınmayan veriler için hafızadan yer ayırma çağrısında bulunmaz.
> * Rust, kapsam dışına çıkmak üzere olan verilerin bellek ayırma çağrılarını değerlendirerek bu kaynağa daha fazla başvuru yapılmasını engeller.

## Kullanım
Yaşam süreleri bir **kesme `'`** işareti ile gösterilir ve kurallara göre adlandırılırken küçük harf kullanılır. Bu kullanım genellikle `'a` ile başlar ve **çok sayıda yaşam süresi** belirtilmesi gerektiğinde sırasıyla **alfabenin diğer harfleri** kullanılır. 

Referansları kullanılırken, aşağıdaki başlıklar altında incelenmiş konulara dikkat edilmesi gereklidir.

### 01. İşlevlerde bildirim
  * Referanslı giriş ve çıkış parametreleri kullanıldığında **`&`** işaretinden sonra parametrelerin yaşam süreleri bildirilmelidir.
  Örneğin: `..(x: &'a str)` , `..(x: &'a mut str)`
  * Genelleştirilen işlevlerde yaşam sürelerinin genellenen türler için olduğu, işlev adından sonra bildirilmelidir.
  Örneğin: `fn foo<'a>(..)` , `fn foo<'a, 'b>(..)`
