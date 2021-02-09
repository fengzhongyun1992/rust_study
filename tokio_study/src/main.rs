/// dyn
/// 返回值： Rust 要求必须返回一个具体的类型而不是一个抽象，因为抽象对于Rust是一个模糊的不具名的信息，
/// 无法在编译期确定很多细节（这其实是可以解决的，不过由于 Rust 目前对于 DST 即动态大小类型的支持还在未来特性中，为保证 Rust 的稳定推进，目前只能这样）
/// 可以通过装修来实现 Box,但使用装箱意味着这一过程属于运行时的动态分配，无法再将对象定于栈上
///
///  Rust 之所以要求函数不能返回多种类型是因为 Rust 在需要在 编译期确定返回值占用的内存大小, 显然不同类型的返回值其内存大小不一定相同.
/// 尝试一：既然如此, 把返回值装箱, 返回一个胖指针, 这样我们的返回值大小可以确定了 （Box）
///  Box<dyn Trait>
/// impl Trait 和 dyn Trait 在Rust分别被称为静态分发和动态分发
///
/// 通过单态化, 编译器消除了泛型, 而且没有性能损耗, 这也是 Rust 提倡的形式, 缺点是过多展开可能会导致编译生成的二级制文件体积过大, 这时候可能需要重构代码.
/// 静态分发虽然有很高的性能, 但在文章开头其另一个缺点也有所体现, 那就是无法让函数返回多种类型, 因此 Rust 也支持通过 trait object 实现动态分发. 既然 Trait 是具有某种特性的类型的集合, 那我们可以把 Trait 也看作某种类型, 但它是"抽象的", 就像 OOP 中的抽象类或基类, 不能直接实例化.
///
/// RFC-2113 明确说明了引入 dyn 的原因, 即语义模糊, 令人困惑, 原因在于没有 dyn 让 Trait 和 trait objects 看起来完全一样
///
/// impl trait 和 dyn trait 区别在于静态分发于动态分发,
/// 静态分发性能 好, 但大量使用有可能造成二进制文件膨胀;
/// 动态分发以 trait object 的概念通过虚表实现, 会带来一些运行时开销.
/// 又因 trait object 与 Trait 在不引入 dyn 的情况下经常导致语义混淆,
/// 所以 Rust 特地引入 dyn 关键字, 在 Rust 2018 中已经稳定.
///


use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// #[tokio::main]
/// main 好像是一个过程宏
/// https://github.com/tokio-rs/tokio/blob/master/tokio-macros/src/lib.rs#L179

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>> {
    let mut listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket,_) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = [0;1024];
            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(n) if n==0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}",e);
                        return ;
                    }
                };
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket;err = {:?}",e);
                    return ;
                }
            }
        });
    }
}
