use std::net::{Ipv4Addr };
use std::time::Duration;
use std::env;


mod lib;
use lib::*;
//use super::oid::ObjIdBuf;
//use super::{Varbinds, Value, SyncSession, SnmpResult};


//#[test]
fn main(){

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let community = b"ggc_ro";
    let ip: Ipv4Addr = "10.80.4.110".parse().unwrap();
    let sync = SyncSession::new(format!{"{}:161", ip}, community, Some(Duration::from_secs(3)),0);

    if let Ok(mut sess) = sync{

        let snmp_oids =    ["1.3.6.1.2.1.2.2.1.2".to_owned()];
        
        let n = 0;
        let m = 100;

        let oids = oid_strings_to_uint_vecs(&snmp_oids);
        let oids = oid_uints_to_ary(&oids);



        let list = snmpbulkwalk(&mut sess, &oids, n, m);

        for item in list{
            println!("{}", item );
        }


    }
}


fn snmpbulkget<'a>(sess: &'a mut SyncSession, oid: &[u32], non_repeaters: u32, max_repeats: u32 ) -> lib::SnmpResult<Varbinds<'a>>{    
    let mut oid_list: Vec<&[u32]> = Vec::new();
    oid_list.push(oid);
    match sess.getbulk(&oid_list , non_repeaters, max_repeats){
        Ok(result) => {
            return Ok(result.varbinds)
        },
        Err(e) =>{
            return Err(SnmpError::SendError)
        }
    }
}


fn snmpbulkwalk(sess: &mut SyncSession, oid_list: &[&[u32]], non_repeaters: u32, max_repeats: u32 ) -> Vec<String>{
    let prefix = oid_list[0];
    let mut oid = prefix;

    let mut list: Vec<String> = Vec::new();
    let mut mem_buff: [u32; 128] = [0; 128];
    loop {
        let bulkquery = snmpbulkget(sess, oid.clone(), non_repeaters, max_repeats);
                
        match bulkquery{
            Ok(vbs) => {
                for (curr_oid, value) in vbs{
                    let mut oid_buff: [u32; 128] = [0; 128];
                    let coid = curr_oid.read_name(&mut oid_buff).unwrap();
                    if !coid.starts_with(prefix) { //if(coid[0..prefix_len] != (*prefix)[..]) {
                        return list;
                    }else{
                        let new_oid = curr_oid.read_name(&mut mem_buff).unwrap();
                        oid = new_oid;
                        list.push( format!("{} {:?}", curr_oid, value) );
                    }
                }                    
            },
            Err(e) => {
                println!("{:?}",e);
            }
        };

    }
}


fn print_varbinds(varbinds: Varbinds){

    let prefix = [43,6,1,2,1,2,2,1,2];
    let l = prefix.len();
    //
    for (oid, val) in varbinds{
        let o = oid.raw();
        let pr = &o[0 .. l];
        
        if pr == prefix {
            println!("{} = {:?}", oid, val);
        }
    }
}


// <----  optimize this  -----
fn oid_strings_to_uint_vecs( snmp_oids: &[String]) -> Vec<Vec<u32>>{ 
    let mut oid_buf = Vec::new();
    for s in snmp_oids.into_iter() {
        let z = s.split('.')
        .filter(|z| !z.is_empty() )
        .map(|z| z.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        oid_buf.push(z);
    }
    oid_buf
}

pub fn oid_uints_to_ary(oid_vec: &Vec<Vec<u32>>) -> Vec<&[u32]> {
    let mut oids = Vec::new();
    for i in 0..oid_vec.len() {
        let v = oid_vec[i].as_slice();
        oids.push(v);
    }
    oids
}

// <----  /optimize this  ----->
