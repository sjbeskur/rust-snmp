//run this with: cargo test serg_test -- --nocapture
use std::net::{Ipv4Addr};
use std::time::Duration;

use super::{SyncSession, SnmpPdu, SnmpMessageType, ObjectIdentifier, Value, ObjIdBuf};

use pnet::datalink;
use pnet::datalink::Channel::Ethernet;
use pnet::packet::Packet;
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};

use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::udp::UdpPacket;
use pnet::packet::ip::IpNextHeaderProtocols;

#[test]
fn serg_test() {
    //let community = b"ggc_ro";
    let community = b"public";
    //let ip: Ipv4Addr = "10.10.1.254".parse().unwrap();
    let ip: Ipv4Addr = "192.168.17.148".parse().unwrap(); //soleinik, ruckus

    let sender = SyncSession::new(format!{"{}:161", ip}, community, None, 0);

    let snmp_oids:[u32; 10] = [
            1,3,6,1,2,1,2,2,1,2
        ];


    //let oid_list = oid_strings_to_uint_vecs(snmp_oids);
    let non_repeaters = 0;
    let max_repeats = 5;

    if let Ok(mut sender) = sender{
        std::thread::spawn(move||{
            let sleep = Duration::from_secs(3);
            loop{
                let res = sender.send_getbulk(&[&snmp_oids], non_repeaters, max_repeats);
                //let res = sender.send_get(&snmp_oids);
                if let Ok(bytes) = res{
                    println!("SNMP sent {} bytes...", bytes);
                }
                std::thread::sleep(sleep);
            }
        });
    }


    //listener part...
    let nic = "enx00e04c68004a";

    // Find the network interface with the provided name
    let interfaces = datalink::interfaces();
    let interface = interfaces.into_iter()
                              .find(|iface|iface.name == nic)
                              .expect(&format!("Unable to find interface[{}]!", nic));


    // Create a new channel, dealing with layer 2 packets
    let mut rx = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(_, rx)) => rx,
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("An error occurred when creating the datalink channel: {}", e)
    };

    //never ending loop...
    loop {
        match rx.next() {
            Ok(packet) => {

                let ether = EthernetPacket::new(packet).unwrap();
                let typ = ether.get_ethertype();
                if typ != EtherTypes::Ipv4 {continue}

                let ipv4 = Ipv4Packet::new(ether.payload());
                if let Some(ipv4) = ipv4 {
                    let proto = ipv4.get_next_level_protocol();

                    if proto == IpNextHeaderProtocols::Udp{

                        let udp = UdpPacket::new(ipv4.payload());
                        if let Some(udp) = udp {


                            match SnmpPdu::from_bytes(udp.payload()){
                                Ok(snmp) =>{

                                    let hw_src = ether.get_source();
                                    let hw_dst = ether.get_destination();

                                    let ip_src = ipv4.get_source();
                                    let ip_dst = ipv4.get_destination();

                                    

                                    if snmp.message_type == SnmpMessageType::Response {
                                        //println!("RESPONSE[id:{}]: [{}]{} -> {}[{}]\n[{:#?}]\n",snmp.req_id, hw_src,ip_src,ip_dst,hw_dst,snmp.varbinds);
                                        println!("RESPONSE[id:{}]: [{}]{} -> {}[{}]\n",snmp.req_id, hw_src,ip_src,ip_dst,hw_dst);

                                        let vbs = snmp.varbinds;

                                        let size = vbs.clone().count();
                                        let fvbs = vbs.filter(|(oi, _)|oi.eq(&snmp_oids[..])).collect::<Vec<(ObjectIdentifier, Value)>>();

                                        fvbs.iter().for_each(|(oid,v)|println!("{} -> {:?}", oid, v));
                                        
                                        if fvbs.len() == size{


                                            if let Ok(mut sender)  = SyncSession::new(format!{"{}:161", ip}, community, None, 0){

                                                let (oid, _) = &fvbs[fvbs.len()-1];

                                                use std::mem;

                                                let mut buf: ObjIdBuf = unsafe { mem::uninitialized() };
                                                if let Ok(name) = oid.read_name(&mut buf) {
                                                    let res = sender.send_getbulk(&[&name], non_repeaters, max_repeats);
                                                    if let Ok(bytes) = res{
                                                        println!("----->SNMP sent more {} bytes...", bytes);
                                                    }
                                                }
                                            }
                                        }else{
                                            println!("========== end ================");
                                        }
                                    }else{
                                        println!("REQUEST[id:{}]: [{}]{} -> {}[{}]\n[{:#?}]\n",snmp.req_id, hw_src,ip_src, ip_dst,hw_dst,snmp.varbinds);
                                    }

                                },
                                Err(_) => ()
                            }




                        }
                    }
                }
            }
            Err(e) => panic!("packetdump: unable to receive packet: {}", e)
        }
    }



}