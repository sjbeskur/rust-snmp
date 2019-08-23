use std::net::{Ipv4Addr };
use std::time::Duration;

mod lib;
use lib::*;

fn main(){

    let community = b"ggc_ro";
    let ip: Ipv4Addr = "10.80.4.110".parse().unwrap();
    let sync = SyncSession::new(format!{"{}:161", ip}, community, Some(Duration::from_secs(3)),0);

    if let Ok(mut sess) = sync{
        /*
        match sess.get(&[1,3,6,1,2,1,1,1,0]){
            Ok(result) =>{
                print_varbinds(result.varbinds);
            },
            _ => {}
        }
        match sess.getnext(&[1,3,6,1,2,1,1,1]){
            Ok(result) =>{
                print_varbinds(result.varbinds);
            },
            _ => {}
        }
        match sess.getbulk(&oids ,0,30){
            Ok(result) => {
                print_varbinds(result.varbinds);
            },
            Err(e) =>{
                println!("{:?}",e)
            }
        }
        */

        let snmp_oids =    ["1.3.6.1.2.1.2.2.1.2".to_owned()];
        let snmp_oids_111 =    ["1.3.6.1.2.1.2.2.1.2.111".to_owned()];
        let snmp_oids_3525 =    ["1.3.6.1.2.1.2.2.1.2.3525".to_owned()];
        let snmp_oids_4356 =    ["1.3.6.1.2.1.2.2.1.2.4356".to_owned()];

//        let oid = [1,3,6,1,2,1,2,2,1,2];
        let oids = oid_strings_to_uint_vecs(&snmp_oids);
        let oids = oid_uints_to_ary(&oids);
        snmpbulkwalk(&mut sess, &oids);

        let oids = oid_strings_to_uint_vecs(&snmp_oids_111);
        let oids = oid_uints_to_ary(&oids);
        snmpbulkwalk(&mut sess, &oids);

        let oids = oid_strings_to_uint_vecs(&snmp_oids_3525);
        let oids = oid_uints_to_ary(&oids);
        snmpbulkwalk(&mut sess, &oids);

        let oids = oid_strings_to_uint_vecs(&snmp_oids_4356);
        let oids = oid_uints_to_ary(&oids);
        snmpbulkwalk(&mut sess, &oids);


    }
}

fn snmpbulkwalk(sess: &mut SyncSession, oid_list: &[&[u32]] ) {
    match sess.getbulk(&oid_list ,0,30){
        Ok(result) => {
            print_varbinds(result.varbinds);
        },
        Err(e) =>{
            println!("{:?}",e)
        }
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
            println!("{} : {:?}", oid, val);
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

fn oid_string_to_uint( snmp_oid: String) -> Vec<u32>{ 
    let z = snmp_oid.split('.')
        .filter(|z| !z.is_empty() )
        .map(|z| z.parse::<u32>().unwrap()).collect::<Vec<u32>>();

    z
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
