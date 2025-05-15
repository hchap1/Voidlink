use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::net::TcpListener;
use std::net::TcpStream;

use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use std::thread::JoinHandle;
use std::thread::spawn;

enum Message {
    End,
    NewClient((TcpStream, SocketAddr)),
}

pub struct ReceiveServer {
    listener: TcpListener,
    clients: Vec<TcpStream>,
    listen_thread: Option<JoinHandle<()>>,
    
    listen_sender: Option<Sender<Message>>,
    client_receiver: Option<Receiver<Message>>,

    client_exchange_thread: Option<JoinHandle<()>>
}

impl ReceiveServer {
    fn new(port: u16) -> Result<ReceiveServer, String> {
        match TcpListener::bind(
            SocketAddr::new(
                IpAddr::V4(
                    Ipv4Addr::new(0, 0, 0, 0)
                ),
                port
            )
        ) {
            Ok(listener) => Ok(ReceiveServer {
                listener, clients: Vec::new(), listen_thread: None,
                listen_sender: None, client_receiver: None
            }),
            Err(e) => Err(format!("Failed to create listener: {e:?}")),
        }
    }

    fn start(&mut self) -> Result<(), String> {
        let (client_sender, client_receiver) = channel::<Message>();
        let (listen_sender, listen_receiver) = channel::<Message>();
        
        self.client_receiver = Some(client_receiver);
        self.listen_sender = Some(listen_sender);

        let listener = match self.listener.try_clone() {
            Ok(listener) => listener,
            Err(e) => return Err(format!("Failed to clone listener: {e:?}"))
        };

        self.listen_thread = Some(spawn(move || listen(listener, client_sender, listen_receiver)));
        Ok(())
    }
}

fn listen(listener: TcpListener, client_sender: Sender<Message>, listen_receiver: Receiver<Message>) {
    let nonblocking = listener.set_nonblocking(true).is_ok();
    loop {
        if nonblocking {
            let (stream, addr) = match listener.accept() {
                Ok(data) => data,
                Err(e) => {
                    println!("Failure to accept client: {e:?}");
                    continue;
                }
            };

            // Err means the data will never arrive. Abandon the server in this case. This is bad.
            if client_sender.send(Message::NewClient((stream, addr))).is_err() {
                println!("Failed to send client between threads. Exiting server.");
                break;
            }

            match listen_receiver.try_recv() {
                Ok(message) => match message {
                    Message::End => break,
                    _ => break
                },
                Err(std::sync::mpsc::TryRecvError::Empty) => {},
                Err(std::sync::mpsc::TryRecvError::Disconnected) => break
            }
        } else {
            println!("Failure to set nonblocking. TODO: Implement blocking");
            break;
        }
    }
}
