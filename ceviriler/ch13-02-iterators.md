## Yineleyiciler ile bir dizi öğeyi işlemek
Yineleyici kalıbı, bazı işlemleri bir dizi öğe üzerinde sırayla gerçekleştirmenizi sağlarken, her bir öğe üzerinde yineleme yapmak ve işlediği dizinin ne zaman biteceğini belirleyen mantıktan sorumludur. Yineleyicileri kullandığınızda bu mantığı kendi başınıza yeniden uygulamak zorunda kalmazsınız.

Rust'ta yineleyiciler tembeldir, bu da demektir ki; yineleyiciler kendilerini tüketerek kullanan yöntemler çağırılana kadar programlarınıza hiçbir etkisi yapmazlar. Örneğin aşağıdaki kod; `Vec<T>` üzerinde tanımlanan `iter` yöntemini çağırarak `v1` vektöründeki öğeler üzerinde bir yineleyici oluşturur. Bu kod tek başına anlamlı bir şey gerçekleştirmez.

Dosya adı: src/main.rs
```Rust
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();
````
[Örnek 13-13:](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=52bd47f729da130719a194242eb2dbb2) Bir yineleyici oluşturmak

Bir yineleyici oluşturduktan sonra artık bunu çeşitli şekillerde kullanabiliriz. Bölüm 3'ün sonlarında yer alan örnek 3-5'te, her eleman üzerinde bir takım işlemleri gerçekleştirmek amacıyla `for` döngüsünün tüketimine verilen `iter` tanımlamasıyla yineleyici kullanmış ancak  yineleyicilere şu ana kadar derinlemesine odaklanamamıştık.

Aşağıdaki örnekte, yineleyicinin oluşturulması ile `for` döngüsünde kullanımı birbirinden bağımsız olarak sunulmaktadır. Kendi başına `v1_iter` değişkeninde saklanan yineleyicinin tanımlandığı satırda hiç bir işlem gerçekleşmez. Ancak `v1_iter`'deki yineleyiciyi kullanan bir `for` döngüsü ile çağrıldığında, kendisine geçirilmiş olan her öğenin değerini ekrana yazdıran bir döngü yineleyicisi olarak kullanışlı  hale gelir.

Dosya adı: src/main.rs
```Rust
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();

for val in v1_iter {
    println!("Got: {}", val);
}
````
[Örnek 13-14:](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=329f8b44fc0e94dccaca707c0ca98574) Bir `for` döngüsünde yineleyici kullanmak

Standart kitaplıklarında yineleyici bulunmayan dillerde bu tarz bir işlev, olasılıkla dizinin ilk elemanıyla değerinin arttırıldığı bir sayaç değişkeniyle başlatılacak, dizinin sonuna erişilene kadar her eleman için değişken birer birer güncellenerek işletilecektir.

Oysa yineleyiciler bütün bu karmaşık sayım sürecindeki mantığı sizin için üstlenebilir, muhtemel kod tekrarlarını azaltarak potansiyel karışıklıkların üstesinden gelebilirler. Yineleyiciler sadece vektörler gibi indekslenebilir veri yapılarıyle değil, aynı mantığın uygulandığı pekçok farklı koleksiyon türüyle kullanılılırken de fazlasıyla esnektirler. Haydi yineleyicilerin bunu nasıl yaptığını birlikte inceleyelim.

### `Iterator` özelliği ve `next` Metodu
