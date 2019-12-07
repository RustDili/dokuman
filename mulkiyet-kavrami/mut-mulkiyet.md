### Mutability ve Mülkiyet
Aşağıdaki örnekten de hatırlanacağı gibi Rust derleyicisine bir değişkenin değişebilir olduğu bilgisi `mut` anahtar kelimesi ile bildiriliyordu.

```rust
let x = 10;
x = 20;             // Değişken bağlamları Rust' da varsayılan olarak 
                    // değişmez kabul edildiğinden derleyici hata üretecektir.

let mut y = 15;
y = 25;             // oysa 'mut' anahtar kelimesi kullanılarak tanımlanan değişken
                    // değişebilir(mutable) olduğundan bu kod ise sorunsuz çalışacaktır.
```

Bir değişkenin değişmezlik durumu mülkiyet transferi gerçekleştirilirken değiştirilebilir. Aşağıdaki örnek, değişmez olarak bildirilmiş bir değere ait mülkiyetin, başka bir değişkene aktarılırken değiştirilebileceğini gösterir.

```rust
fn main() {
    let immutable_vec = vec![1, 2, 3];
    println!("immutable_vec değişkeni: {:?}", immutable_vec);

    // Değiştirilemez bir değeri değiştirilebilir olarak başka bir değişkene taşıyoruz.
    let mut mutable_vec = immutable_vec;
    println!("mutable_vec değişkeni: {:?}", mutable_vec);

    // Artık bu değişkeni istediğimiz gibi değiştirebiliriz.
    mutable_vec.push(4);
    println!("şimdi de mutable_vec değişkeni: {:?}", mutable_vec);
}
```
