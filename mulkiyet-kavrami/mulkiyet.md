### Mülkiyet Kavramını Anlamak
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
