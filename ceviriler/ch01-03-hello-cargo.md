## Merhaba, Cargo

Cargo Rust'ın derleme sistemi ve paket yöneticisidir. Bu araç sizin için; kod oluşturmak, kodun bağımlı olduğu kütüphaneleri *(kodunuzun ihtiyaç duyduğu kitaplıklara bağımlılık adını veriyoruz)* indirmek ve bunları derlemek gibi pek çok görevi yerine getirdiğinden çoğu Rustacean bu aracı Rust projelerini yönetmek için kullanır.

Daha önce yazdığımıza benzeyen basit Rust programlarının herhangi bir kütüphaneye bağımlılığı bulunmadığından "Merhaba dünya!" gibi basit bir projeyi Cargo ile derlemeye çalışsaydık, Cargo'nun sadece kodu derlemekle ilgili olan bölümü kullanılmış olacaktı. Daha karmaşık ve farklı kütüphanelerle çalışmaya ihtiyaç duyan Rust programları yazmaya niyetlendiğinizde, projenin Cargo aracılığıyla oluşturulması bağımlılıkları yönetmenize epey yardımcı olacaktır.

Rust projelerinin büyük çoğunluğu Cargo aracını kullandığından, bu kitabın geri kalan bölümlerinde sizin de bu aracı kullandığınız varsayılacaktır. Eğer ["Kurulum"](ch01-01-installation.md) bölümünde açıklanan resmi yükleyicileri kullandıysanız Cargo aracı Rust ile birlikte gelmiş demektir. Rust'ı başka yollar ile kurduysanız sisteminizde Cargo'nun kurulu olup olmadığını terminalinize aşağıdaki kodları girerek kontrol edebilirsiniz.

```console
$ cargo --version
```

Terminalinizde çıktı olarak bir sürüm numarası görünüyorsa Cargo aracınız Rust kurulumuyla birlikte yüklenmiş demektir. Eğer `Command not found` şeklinde bir hata ile karşılaşıyorsanız Cargo'nun nasıl kurulacağına dair kullandığınız kurulum yönteminin belgelerine bakmalısınız.

### Cargo ile Proje Oluşturmak
