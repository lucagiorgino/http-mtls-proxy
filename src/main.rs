use std::convert::Infallible;
use std::net::SocketAddr;

use bytes::Bytes;
use http_body_util::Full;
use tokio::net::{TcpListener, TcpStream};

// An async function that consumes a request, does nothing with it and returns a
// response.
// async fn hello(_: Request<impl hyper::body::Body>) -> Result<Response<Full<Bytes>>, Infallible> {
//     Ok(Response::new(Full::new(Bytes::from("Hello World!"))))
// }

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();

    // This address is localhost
    let addr: SocketAddr = ([127, 0, 0, 1], 3000).into();

    // Bind to the port and listen for incoming TCP connections
    // Bind the listener to the address
    let listener = TcpListener::bind(addr).await?;

    println!("Listening on http://{}", addr);
    loop {
        // When an incoming TCP connection is received grab a TCP stream for
        // client<->server communication.
        //
        // Note, this is a .await point, this loop will loop forever but is not a busy loop. The
        // .await point allows the Tokio runtime to pull the task off of the thread until the task
        // has work to do. In this case, a connection arrives on the port we are listening on and
        // the task is woken up, at which point the task is then put back on a thread, and is
        // driven forward by the runtime, eventually yielding a TCP stream.
        let (socket, _) = listener.accept().await?;
        // Use an adapter to access something implementing `tokio::io` traits as if they implement
        // `hyper::rt` IO traits.
        // let io = TokioIo::new(tcp);

        println!("{:?}", socket);
        
        tokio::spawn(handle_client(socket));

        // Spin up a new task in Tokio so we can continue to listen for new TCP connection on the
        // current task without waiting for the processing of the HTTP1 connection we just received
        // to finish
        // tokio::task::spawn(async move {
        //     // Handle the connection from the client using HTTP1 and pass any
        //     // HTTP requests received on that connection to the `hello` function
        //     if let Err(err) = http1::Builder::new()
        //         .timer(TokioTimer::new())
        //         .serve_connection(io, service_fn(hello))
        //         .await
        //     {
        //         println!("Error serving connection: {:?}", err);
        //     }
        // });
    }
}

// Handle client connection
async fn handle_client(mut socket: TcpStream) {
    todo!();
}