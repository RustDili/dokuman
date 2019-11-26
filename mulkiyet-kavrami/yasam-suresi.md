### Yaşam Alanı ve Süresi
Rust dilinde bazı değişkenler tanımlandıkları sırada hiçbir kaynağa sahip değillerken, sistemden alınmış bulunan her kaynağın sadece bir sahibi bulunur. Benzer şekilde bir değerin sahibine mülkiyeti bıraktırıldığında Rust jargonuyla düşürüldüğünde sahip olunan değer de düşürülür. Daha da ilginci değişkenin bildirildiği bir blok kapsamı, bir işlev, bir if ifadesi veya süslü parantezler arasında sunulan bir kod bloğu vb. Sonlandırıldığı anda hem değişken hem de içeriğindeki değer düşürülür.

```Rust
fn selamla() {
  let baglam = "İyi günler dilerim".to_string();
  println!("{}", baglam); // `baglam` değişkeni de burada düşürülür.
}
```

Yukarıdaki örneğe bakıldığında, işlev bloğunun sonuna gelindiğinde s değişkeni değerinin düşürüleceğini bildiğimizden, değişkenin yaşam alanını net biçimde gözlemleyebiliyoruz. Bu durum daha karmaşık veri yapılarında da aynen geçerlidir.

```rust
let names = vec!["Pascal".to_string(), "Christoph".to_string()];
```

Bu örnek kod ile names adında bir vektör oluşturulmuştur. Rust’ın vektörleri dizi veya listelere benzemekle beraber boyutları dinamik olarak değişebildiğinden push() işlevi yardımıyla çalışma zamanında değer kabul edebilirler. Örneğin kullandığı hafıza aşağıdaki gösterime benzer.

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

Daha önceki string nesnesine benzer şekilde kapasitesi, uzunluğu ve vektöre ait ham verilerinin bulunduğu heap konumu gösteren işaretçisi ile vektör nesnesinin kendisi stack üzerinde depolanırken, vektöre ait ham verilerin oluşturduğu string nesnelerinin kendi tampon bellekleri ile ve sırasıyla heap üzerinde depolanır. Bu şekilde her değerin tek bir değişkene ait olduğu veri ağacı yapısı oluşturularak, names vektöründeki her bir öğenin kapsam dışına çıkıldıkça değerlerinin ve kullandıkları ara belleklerinin düşmesi sağlanmış olur.
