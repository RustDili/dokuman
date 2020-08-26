## Sürüm Profilleriyle Derlemeleri Özelleştirme

Rust'ın *sürüm profilleri* kodun derleme aşamasında programcıya, çeşitli seçenekler üzerinde daha fazla denetim izni veren, değişik yapılandırmalara sahip önceden tanımlanmış ve özelleştirilebilir profillerdir. Her profil diğerlerinden bağımsız olarak yapılandırılır.

Cargo'nun iki ana profili vardır: Bunlardan `dev` seçeneği, programın `cargo build` komutuyla işletilmesi sırasında kullanılan profil, release seçeneğiyse `cargo build --release` komutuyla işletilirken kullanılan profildir. Geliştirici profili *(dev)*, geliştirme için iyileştirilen donanımlara sahipken, sürüm profili *(release)*, sürüm derlemeleri için iyileştirilen donanımlarla gelir.

Bu profil adları size sürüm çıktılarınızdan tanıdık geliyor olabilir:

```bash
$ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
$ cargo build --release
    Finished release [optimized] target(s) in 0.0s
````
Yukarıdaki derleme çıktısında gösterilen `dev` ve `release` ifadeleri, derleyicinin farklı profiller kullandığını gösterir.

Eğer projenizin *Cargo.toml* dosyasında `[profil.*]` bölümü yoksa Cargo geçerli olan profillerin her biri için varsayılan ayarlara sahip demektir. Özelleştirmek istediğiniz herhangi bir profili `[profil.*]` bölüm başlığının altına ekleyerek, varsayılan ayarların alt kümelerini geçersiz kılabilirsiniz. Örneğin, `dev` ve `release` profilleri için `opt-level` *(tercih düzeyi)* ayarları için varsayılan değerler şunlardır:

Dosya: Cargo.toml

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
````

`Opt-level` ayarı, Rust'ın kodunuza uygulayacağı 0-3 aralığındaki optimizasyon değerini kontrol eder. Henüz geliştirme aşamasındaysanız ve kodunuzu sık sık derliyorsanız, daha fazla optimizasyon uygulamak derleme süresini uzatacağından, kodunuz yavaş çalışsa bile derleme sürenizin hızlı olmasını istersiniz. O nedenle `dev` profili için `opt-level` düzeyi varsayılanı `0` olarak atanmıştır. Oysa sürüm modunda, programınızı bir kere derleyeceğiniz ve sonrasında çok kere çalıştıracağınızdan, derleme süresi için fazla zaman ayırmayı sadece kodunuzu yayınlanmaya hazır olduğunuzda tercih edeceksiniz. Yani daha hızlı çalışan kod üretimi daha fazla zaman gerektireceğinden sürüm modunda profili düzeyi düzeyi varysayılanı `3` olarak ayarlanmıştır.

Varsayılan ayarların herhangi birini projenizin *Cargo.toml* dosyasına farklı bir değer ekleyerek geçersiz kılabilirsiniz. Örneğin, `dev` profilinin optimizasyon düzeyini 1 olarak ayarlamak istiyorsanız, projenizin *Cargo.toml* dosyasına bu iki satırı eklemeniz yeterlidir:

Dosya: Cargo.toml

```toml
[profile.dev]
opt-level = 1
````

Bu kod varsayılan `opt-level = 0` düzey ayarını geçersiz kılacağından, `cargo build` komutunu çalıştırdığımızda, Cargo tarafından `dev` profili için varsayılan değerlerle birlikte, `opt-level` düzeyinde gerçekleştirdiğimiz özelleştirme seviyesi değerlendirilecektir. Bununla birlikte derleyici, `opt-level` düzeyini `1` olarak ayarladığımızdan, sürüm profilinde olduğu kadar olmasa da, varsayılandan daha fazla iyileştirme uygular. 

Profillerin yapılandırma seçenekleri ve varsayılan değerlerinin tam listesi için [Cargo belgeleri](https://doc.rust-lang.org/cargo/reference/profiles.html)'ni inceleyebilirsiniz.
