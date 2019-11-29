### Yaşam Alanı ve Süresi
Rust programlarında bazı değişkenler tanımlandıkları esnada hiçbir kaynağa sahip olmayabilirler. Bununla birlikte sistemden alınmış bulunan her kaynağın sadece bir sahibi bulunur. 
Benzer şekilde bir değerin sahibine o değerin mülkiyeti bıraktırıldığında, *-ki buna Rust programcıları mülkiyetin düşürülmesi der-* sahip olunan değer de düşürülür. Daha da ilginci, değişkenin bildirildiği **blok kapsamı**, **işlev**, **if ifadesi** veya süslü parantezler arasında sunulan bir **kod bloğu** vb. sonlandırıldığı anda; hem değişken hem de içerdiği değer düşürülür

```Rust
fn selamla() {
  let baglam = "İyi günler dilerim".to_string();
  println!("{}", baglam); // `baglam` değişkeni de burada düşürülür.
}
```

Yukarıdaki örneğe bakıldığında, işlev bloğunun sonunda `s` değişken değerinin düşürüleceğini bildiğimizden, değişkenin yaşam alanını net biçimde gözlemleyebiliyoruz. Bu durum daha karmaşık veri yapılarında da aynen geçerlidir.

```rust
let names = vec!["Pascal".to_string(), "Christoph".to_string()];
```

Üstteki örnekte `names` adında ve `string` türünde bir vektör oluşturulmuştur. Rust’ın vektörleri dizi veya listelere benzemekle beraber boyutları dinamik olarak değişebildiğinden `push()` işlevi yardımıyla çalışma zamanında değer kabul edebilirler. Örneğin kullandığı hafıza aşağıdaki gösterime benzer.

```bash
            [–– names ––]
            +–––+–––+–––+
stack frame │ • │ 3 │ 2 │
            +–│–+–––+–––+
              │
            [–│–– 0 –––] [–––– 1 ––––]
            +–V–+–––+–––+–––+––––+–––+–––+–––+
       heap │ • │ 8 │ 6 │ • │ 12 │ 9 │       │
            +–│–+–––+–––+–│–+––––+–––+–––+–––+
              │\   \   \  │
              │ \   \    length
              │  \    capacity
              │    buffer │
              │           │
            +–V–+–––+–––+–––+–––+–––+–––+–––+
            │ P │ a │ s │ c │ a │ l │   │   │
            +–––+–––+–––+–––+–––+–––+–––+–––+
                          │
                          │
                        +–V–+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+
                        │ C │ h │ r │ i │ s │ t │ o │ p │ h │   │   │   │
                        +–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+
```

Daha önceki `string` nesnesine benzer şekilde kapasitesi, uzunluğu ve vektöre ait ham verilerinin bulunduğu `heap` konumunu gösteren işaretçisi ile vektör nesnesinin kendisi `stack` üzerinde depolanırken, vektöre ait ham verilerin oluşturduğu `string` nesneleri, kendi tampon bellekleri ile ve sırasıyla `heap` üzerinde depolanır. Bu şekilde her değerin tek bir değişkene ait olduğu veri ağacı yapısı oluşturularak, `names` vektöründeki her bir öğenin kapsam dışına çıkıldıkça değerlerinin ve kullandıkları ara belleklerinin düşürülmesi sağlanır.
