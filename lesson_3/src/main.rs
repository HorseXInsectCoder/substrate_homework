
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{self, Read, Write};
use std::thread::JoinHandle;

// 512字节的缓冲buffer
const BUF_SIZE: usize = 512;

// 处理具体的连接
fn handle_stream(mut stream: TcpStream) -> io::Result<()> {
    // 绑定一个512字节的buffer
    let mut buf = [0; BUF_SIZE];

    // 循环处理连接
    loop {
        // 把流读到buffer中
        let bytes_read = stream.read(&mut buf)?;
        // 如果没有数据，直接返回
        if bytes_read == 0 {
            return Ok(())
        }
        // 把数据写到buffer中
        stream.write(&buf[..bytes_read]);
    }
}

fn main()  {
    // 监听127.0.0.1:8081，并用match进行错误处理
    let listener = match TcpListener::bind("127.0.0.1:8081") {
        // 正确的情况
        Ok(listener) => listener,
        // 错误的情况
        Err(e) => {
            // 打印错误
            eprintln!("bind Err: {}", e);
            // 程序异常退出
            panic!();
        },
    };

    // 绑定一个向量，里面放的是线程句柄
    let mut thread_vec: Vec<JoinHandle<()>> = vec![];

    // 接收流
    for stream in listener.incoming() {
        // 重新绑定(shadowing)流
        let stream = stream.unwrap();
        // 生成线程处理连接
        let handle = thread::spawn(move || {
            // 具体处理连接的函数，
            handle_stream(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
        });
        // 把线程句柄放到向量中
        thread_vec.push(handle);
    };

    // 对每个向量里面的元素进行等待
    for handle in thread_vec {
        handle.join().unwrap();
    }
}
