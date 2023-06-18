use std::io;

fn main() -> io::Result<()> {
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buf = [0u8; 1504];
    loop {
        let nbytes = nic.recv(&mut buf[..])?;
        let _eth_flags = u16::from_be_bytes([buf[0], buf[1]]);
        let _eth_proto = u16::from_be_bytes([buf[2], buf[3]]);
        if _eth_proto != 0x0800 {
            //not ipv4
            continue;
        }
        match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..nbytes])  {
            Ok(p) => {
                let src = p.source_addr();
                let dst = p.destination_addr();
                let proto = p.protocol();
                if proto != 0x06 {
                    //not tcp
                    continue;
                }
                match etherparse::TcpHeaderSlice::from_slice(&buf[4+p.slice().len()..]) {
                    Ok(p) => {
                        eprintln!(
                            "{} => {} {}b of tcp to port {}",
                            src,
                            dst,
                            p.slice().len(),
                            p.destination_port()
                        );
                    }
                    Err(e) => {
                        eprintln!("ignoring weird tcp packet : {:?}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("ignoring weird packet : {:?}", e);
            }
        }
        // match etherparse::Ipv4HeaderSlice::from_slice(&buf[..nbytes]) {
        //     Ok(header) => {
        //         eprintln!("header : {:?}", header);
        //         match header.protocol() {
        //             6 => {
        //                 match etherparse::TcpHeaderSlice::from_slice(&buf[header.slice().len()..nbytes]) {
        //                     Ok(tcp) => {
        //                         eprintln!("tcp : {:?}", tcp);
        //                         let datai = header.slice().len() + tcp.slice().len();
        //                         eprintln!("data : {:x?}", &buf[datai..nbytes]);
        //                     },
        //                     Err(e) => {
        //                         eprintln!("tcp error : {:?}", e);
        //                     }
        //                 }
        //             },
        //             _ => {
        //                 eprintln!("protocol : {:?}", header.protocol());
        //             }
        //         }
        //     },
        //     Err(e) => {
        //         eprintln!("ipv4 error : {:?}", e);
        //     }
        // }
    }
}
