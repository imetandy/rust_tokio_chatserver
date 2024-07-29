use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, 
    net::TcpListener
};

// future is a thing with no known value at the moment
// Rust doesn't do async by default
#[tokio::main]
async fn main() {
    // tcp echo server
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    loop {
        let (mut socket, _addr) = listener.accept().await.unwrap();
        
        tokio::spawn( async move {

            let (reader, mut writer) = socket.split();

            let mut reader = BufReader::new(reader);
    
            let mut line = String::new();
    
            loop {
                    
                let bytes_read = reader.read_line(&mut line).await.unwrap();
    
                if bytes_read == 0 {
                    break;
                }
                writer.write_all(line.as_bytes()).await.unwrap();
                line.clear();
    
            }
        });

    }

}
