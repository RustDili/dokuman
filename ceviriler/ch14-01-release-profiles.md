# Sürüm Profilleriyle Derlemeleri Özelleştirme
Rust'ta *yayın profilleri*; bir programcının kodu derleme aşamasında çeşitli seçenekler üzerinde daha fazla denetime sahip olmasına izin veren, farklı yapılandırmalara sahip önceden tanımlanmış ve özelleştirilebilir profillerdir. Her profil diğerlerinden bağımsız olarak yapılandırılır.

Cargo'nun iki ana profili fardır: Bunlardan `dev` olanı programın `cargo build` komutuyla işletilirken kullanılan profil, release ise `cargo build --release` komutuyla işletilirken kullanılan profildir. Geliştirici profili *(dev)*, geliştirme için iyi varsayılanlarla tanımlanırken, yayın profili *(release)*, yayın derlemeleri için iyi varsayılanlara sahiptir.

Bu profil adları, yapılarınızın çıktısından tanıdık gelebilir:
```bash
$ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
$ cargo build --release
    Finished release [optimized] target(s) in 0.0s
````
Yukarıdaki derleme çıktısında gösterilen `dev` ve `release` ifadeleri, derleyicinin farklı profiller kullandığını gösterir.

Eğer projenizin *Cargo.toml* dosyasında `[profil.*]` bölümü yoksa Cargo geçerli olan profillerin her biri için varsayılan ayarlara sahip demektir. Özelleştirmek istediğiniz herhangi bir profili `[profil.*]` bölüm başlığının altına ekleyerek, varsayılan ayarların alt kümelerini geçersiz kılabilirsiniz. Örneğin, `dev` ve `yayın` profilleri için `opt-level` *(tercih düzeyi)* ayarları için varsayılan değerler şunlardır:

Dosya: Cargo.toml
```rust
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
````
`Opt-level` ayarı, Rust'ın kodunuza uygulayacağı 0-3 aralığındaki optimizasyon değerini kontrol eder. Daha fazla optimizasyon uygulamak derleme süresini uzatacağından, geliştirme aşamasındaysanız ve kodunuzu sık sık derliyorsanız, elde edilen kod daha yavaş çalışsa bile hızlı bir derleme süresi istersiniz. O nedenle `dev` profili için `opt-level` düzeyi varsayılanı `0` olarak atanmıştır. Derleme için daha fazla zaman harcamak ancak kodunuz yayınlanmaya hazır olduğunda tercih edeceğiniz şeydir. Çünkü yayın modunda yalnızca bir defa derleme yaparken, sonrasında derlenen programı birçok kez çalıştırırsınız. Bu nedenle yayın modu, daha hızlı çalışan kodlar için daha uzun derleme süresi kullanır. İşte bu sebepten yayın profili olarak varsayılan `opt-level` düzeyi `3` olarak ayarlanmıştır.

Varsayılan ayarların herhangi birini projenizin Cargo.toml dosyasına farklı bir değer ekleyerek geçersiz kılabilirsiniz. Örneğin, `dev` profilinin optimizasyon düzeyini 1 olarak ayarlamak istiyorsak, bu iki satırı projemizin Cargo.toml dosyasına ekleyebiliriz:

Dosya: Cargo.toml
```Rust
[profile.dev]
opt-level = 1
````
Bu kod varsayılan `opt-level = 0` düzey ayarını geçersiz kılacağından `cargo build` komutunu çalıştırdığımızda, Cargo `dev` profili için varsayılan değerlerle birlikte `opt-level` düzeyinde gerçekleştirdiğimiz özelleştirme seviyemizi tercih edecektir. Bununla birlikte `opt-level` düzeyini `1` olarak ayarladığımızdan derleyici, yayın profilinde olduğu kadar olmasa da, varsayılandan daha fazla optimizasyon uygular. 

Profillerin yapılandırma seçenekleri ve varsayılan değerlerinin tam listesi için [Cargo belgeleri](https://doc.rust-lang.org/cargo/reference/manifest.html#the-profile-sections)ni inceleyebilirsiniz.
