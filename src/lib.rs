use std::net::*;
use std::io::*;

pub struct BoincClient {
    stream: TcpStream,
}

impl BoincClient {
    /// Connect to a BOINC client running on the standard port.
    pub fn connect(address: IpAddr) -> Result<BoincClient> {
        let socket_addr = SocketAddr::new(address, 31416);
        BoincClient::connect_socket(socket_addr)
    }
    /// Connect to a BOINC client running on the local host.
    pub fn connect_local() -> Result<BoincClient> {
        BoincClient::connect_socket("localhost:31416")
    }
    /// Connect to a BOINC client running on a non standard port.
    pub fn connect_socket<A: ToSocketAddrs>(address: A) -> Result<BoincClient> {
        let stream = TcpStream::connect(address)?;
        stream.set_read_timeout(Some(std::time::Duration::from_secs(3)))?;
        stream.set_nodelay(true)?;
        Ok(BoincClient { stream })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connect() {
        // Local BOINC server, standard port configuration.
        assert!(BoincClient::connect_local().is_ok());

        // Local or remote BOINC server, standard port configuration.
        assert!(BoincClient::connect("127.0.0.1".parse().unwrap()).is_ok());

        // Local or remote BOINC server, custom port configuration.
        assert!(BoincClient::connect_socket("localhost:31416").is_ok());
        assert!(BoincClient::connect_socket("slartybartfast").is_err());
    }
}
