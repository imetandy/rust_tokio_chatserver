use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, 
    net::TcpListener, sync::broadcast
};


// turbofish example: 
fn give_me_default<T>() -> T where T: Default,{
    Default::default()
}


// future is a thing with no known value at the moment
// Rust doesn't do async by default
#[tokio::main]
async fn main() {
    let _value = give_me_default::<i32>();
    
    // tcp echo server
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    let (tx, _rx) = broadcast::channel(10);


    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn( async move {

            let (reader, mut writer) = socket.split();

            let mut reader = BufReader::new(reader);
    
            let mut line = String::new();
    
            loop {
                // useful when things need to use shared state with a finite number of things. 
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            break;
                        }

                        tx.send((line.clone(), addr)).unwrap();
                        line.clear();
                    }
                    result = rx.recv() => {
                        let (msg, other_addr) = result.unwrap();
                        if addr != other_addr {
                            writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }

                }
                
    
            }
        });

    }

}
