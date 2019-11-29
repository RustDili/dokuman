### Mülkiyet Kavramını Anlamak
Çöp toplayıcısına sahip olmayan düşük seviyeli dillerde, değişkenler için ayrılan kaynakların hafızaya iade edilmesinden programcının kendisi sorumludur. Programın akışı içerisinde programcı; işlemlerinde kullandığı kaynaklarla işini bitirdiğinde ya da programın o kaynağa ihtiyacı olmadığını düşündüğünde, kaynağı serbest bırakarak kullanılan bellek alanını iade eder. Böyle bir örneğin **C** dilindeki karşılığı işaretçi kullanılarak aşağıdaki şekilde yazılabilir.

```c
    int main(int argc, const char * argv[]) {
        char *str;
        str = (char *) malloc(10);  // Bellekte 10 birimlik yer ayırıyoruz.
                                    // str ile bir takım işlemler...

        free(str);                  // str pointer'ını ve kullandığı kaynağı serbest bırakıyoruz.

        return 0;
    }
```

Bu basit örnekte; `str` adında bir `char` işaretçisine hafızadan 10 birimlik bellek alanı ayrılmış, kodun işaretçiye ihtiyaç duyulmayan bölümüne gelindiğinde `free()` işlevi yardımıyla ayrılan kaynak sisteme iade edilmiştir.

Normal görünen bu kodda yer alan iade işleminin herhangi bir nedenle unutulması, ayrılan kaynağın işletim sistemine geri verilmemesine neden olur ve bu kaynak sistem yeniden başlatılana kadar hafızada kalmaya devam eder. Bu bir durumda hafızanın kullanılan o bölümü süreç boyunca sistem ihtiyaçları için kullanılamayacağından, program **bellek sızıntısı** olarak bilinen soruna neden olacaktır.

Bellek sızıntısı, iade edilmesi gereken bellek miktarı arttıkça ciddi oranda kaynak savurganlığına neden olacak ve programın performans kaybından donmasına kadar giden daha büyük sorunları da beraberinde getirecektir.
Çöp toplayıcılı dillerde bu problemler otomatik süreçler tarafından yönetilip yürütüldüğünden, programlama süreci büyük bir dikkat ve hassasiyet gerektirmez.

Bazı programlama dilleri, sistemden ödünç alınan kaynakların geri verilmesi sorumluluğunu programcıya bırakırken, bazıları da otomatik olarak kendi sorumluluğunda bulunan çöp toplayıcı mekanizmalarına devrederler. Rust’ta ise bellek kaynaklarının geri verilmesi sorumluluğu mülkiyet kavramı üzerinden **hafızayı elinde tutanın sorumluluğuna** bırakılmıştır. Bu da demektir ki; bir nesneye sahip olan, o nesnenin yaşam hakkınına da sahiptir: İster bu hakkı devreder, ister nesneyi öldürür. 

Rust bir çöp toplama mekanizmasına sahip olmadığı gibi, bellek yönetimi konusunda da programcısını zorlamaz. Bellek yönetimi ve güvenliği ise **mülkiyet kavramı** üzerinden sağlanır.
