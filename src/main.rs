use {
    hyper::{
        service::{make_service_fn, service_fn},
        Body, Error, Request, Response, Server,
    },
    std::{env::args, net::ToSocketAddrs},
};

const MSG: &[u8] = b"I'm a teapot";

async fn handle(_: Request<Body>) -> Result<Response<Body>, Error> {
    Ok(Response::new(MSG.into()))
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let addr = args()
        .nth(1)
        .ok_or("Missing argument")?
        .to_socket_addrs()
        .map_err(|e| e.to_string())?
        .next()
        .ok_or("No available address")?;

    let make_service = make_service_fn(|_| async move { Ok::<_, Error>(service_fn(handle)) });
    let builder = Server::bind(&addr);

    println!("Listening on {}", addr);
    let _ = builder.serve(make_service).await;
    Ok(())
}
