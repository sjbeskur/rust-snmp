use std::net::{Ipv4Addr};
use std::time::Duration;

use super::{SnmpResult, SyncSession, Varbinds, ObjectIdentifier, Value};


#[test]
fn sam_test() {
    //let community = b"ggc_ro";
    let community = b"public";
    //let ip: Ipv4Addr = "10.10.1.254".parse().unwrap();
    let ip: Ipv4Addr = "192.168.17.148".parse().unwrap(); //soleinik, ruckus

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
        // hp test 
        /*
        let snmp_oids =    ["1.3.6.1.2.1.2.2.1.2".to_owned()];
        let snmp_oids_111 =    ["1.3.6.1.2.1.2.2.1.2.111".to_owned()];
        let snmp_oids_3525 =    ["1.3.6.1.2.1.2.2.1.2.3525".to_owned()];
        let snmp_oids_4356 =    ["1.3.6.1.2.1.2.2.1.2.4356".to_owned()];

        n = 0
        m = 30
        */

        //let snmp_oids =    ["1.3.6.1.2.1.2.2.1.2".to_owned()];
        let snmp_oids =    ["1.3.6.1.2.1.2.2.1.2".to_owned()];

        // let snmp_oids_5 =    ["1.3.6.1.2.1.2.2.1.2.5".to_owned()];
        // let snmp_oids_10 =    ["1.3.6.1.2.1.2.2.1.2.10".to_owned()];
        // let snmp_oids_15 =    ["1.3.6.1.2.1.2.2.1.2.15".to_owned()];

        
        let n = 0;
        let m = 5;

//        let oid = [1,3,6,1,2,1,2,2,1,2];
        let oids = oid_strings_to_uint_vecs(&snmp_oids);
        let oids = oid_uints_to_ary(&oids);
        snmpbulkwalk(&mut sess, &oids, n, m);

        println!("");
/*
        let oids = oid_strings_to_uint_vecs(&snmp_oids_5);
        let oids = oid_uints_to_ary(&oids);
        snmpbulkwalk(&mut sess, &oids, n, m);
        println!("");

        let oids = oid_strings_to_uint_vecs(&snmp_oids_10);
        let oids = oid_uints_to_ary(&oids);
        snmpbulkwalk(&mut sess, &oids, n, m);
        println!("");

        let oids = oid_strings_to_uint_vecs(&snmp_oids_15);
        let oids = oid_uints_to_ary(&oids);
        snmpbulkwalk(&mut sess, &oids, n, m);
*/

    }
}

// fn snmpbulkget(sess: &mut SyncSession, oid_list: &[&[u32]], non_repeaters: u32, max_repeats: u32 ) {
//     match sess.getbulk(&oid_list , non_repeaters,max_repeats){
//         Ok(result) => {
//             print_varbinds(result.varbinds);
//         },
//         Err(e) =>{
//             println!("{:?}",e)
//         }
//     }
// }

//sess: &'a mut SyncSession 
fn snmpbulkget_r<'a>(sess: &'a mut SyncSession, oid: &[u32], non_repeaters: u32, max_repeats: u32 ) -> SnmpResult<Varbinds<'a>>{
    
    let mut oid_list: Vec<&[u32]> = Vec::new();
    oid_list.push(oid);

    match sess.getbulk(&oid_list , non_repeaters, max_repeats){
        Ok(result) => {
            return Ok(result.varbinds)
        },
        Err(e) =>{
            return Err(e) //SnmpError::SendError)
        }
    }

}


fn snmpbulkwalk(sess: &mut SyncSession, oid_list: &[&[u32]], non_repeaters: u32, max_repeats: u32 ) {
    let prefix = oid_list[0];
    let prefix_len = prefix.len();
    let mut oid = prefix;

    //let mut list: Vec<(ObjectIdentifier, Value)> = Vec::new();
    
    let mut mem_buff: [u32; 128] = [0; 128];

    loop {
        let bulkquery = snmpbulkget_r(sess, oid.clone(), non_repeaters, max_repeats);
                
        match bulkquery{

            Ok(vbs) => {
                let filtered : Vec<(ObjectIdentifier, Value)> = vbs
                    .filter(|(curr_oid, _val)| {  
                        let mut oid_buff: [u32; 128] = [0; 128];

                        //&[u32]
                        let coid = curr_oid.read_name(&mut oid_buff).unwrap();


                        let r = coid[0..prefix_len] == (*prefix)[..];
                        r
                }).collect();











                if filtered.len() > 0{
                    let last = filtered.last().unwrap();
                    let last_oid = last.0.clone();
                    let new_oid = last_oid.read_name(&mut mem_buff).unwrap();
                    oid = new_oid;
                    
                    for (i , j ) in filtered{
                        println!("oid={:?}   val={:?}", i, j);
                    }                    
                }else{
                    break;
                }

            },
            _ => {}
        };

    };
}

//        let loid = last_oid.read_name(&mut buff).unwrap();
//        _oid = &loid.to_vec()[0 .. pl].to_vec();

//        if 
//        println!("{} : {:?}", last_oid, vb);
//        println!("{:?} : ", voids);
//       print_varbinds(vbs);

// fn print_varbinds(varbinds: Varbinds){

//     let prefix = [43,6,1,2,1,2,2,1,2];
//     let l = prefix.len();
//     //
//     for (oid, val) in varbinds{
//         let o = oid.raw();
//         let pr = &o[0 .. l];
        
//         if pr == prefix {
//             println!("{} = {:?}", oid, val);
//         }
//     }
// }


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

// fn oid_string_to_uint( snmp_oid: String) -> Vec<u32>{ 
//     let z = snmp_oid.split('.')
//         .filter(|z| !z.is_empty() )
//         .map(|z| z.parse::<u32>().unwrap()).collect::<Vec<u32>>();

//     z
// }


pub fn oid_uints_to_ary(oid_vec: &Vec<Vec<u32>>) -> Vec<&[u32]> {
    let mut oids = Vec::new();
    for i in 0..oid_vec.len() {
        let v = oid_vec[i].as_slice();
        oids.push(v);
    }
    oids
}

// <----  /optimize this  ----->
