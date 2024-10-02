// main.rs
use tfhe::{ConfigBuilder, generate_keys, set_server_key, FheUint8};
use tfhe::prelude::*;

fn main() {
    let config = ConfigBuilder::default().build();
    let (client_key, server_key) = generate_keys(config);

    let clear_v: Vec<u8> = vec![8, 2, 3, 21, 5, 1, 13]; // input vector
    println!("input vector = {:?}", clear_v);

    // encrypt clear_v
    let mut v: Vec<_> = clear_v.into_iter()
        .map(|x| FheUint8::encrypt(x, &client_key)) // encrypt each element of clear_v using client_key
        .collect(); // obtained encrypted vector of clear_v

    // server-side computation: insertion sort
    set_server_key(server_key);
    for i in 0..v.len() { // i <- 0, 1, ..., v.len()-1
        for j in (0..i).rev() { // j <- i-1, i-2, ..., 0
            let gt = v[j].gt(&v[j+1]); // 'gt' to be the encrypted bool for v[j] > v[j+1]
            let tmp = v[j].clone(); // clone v[j]
            v[j] = gt.select(&v[j+1], &v[j]); // using 'gt', the smaller value is selected and cloned to v[j] (secretly)
            v[j+1] = gt.select(&tmp, &v[j+1]); // using 'gt', the larger value is selected and cloned to v[j+1] (secretly)
        }
    }

    // decrypt the sorted vector
    let clear_v: Vec<u8> = v.iter()
        .map(|x| FheUint8::decrypt(x, &client_key)) // decrypt each element of v using client_key
        .collect(); // obtained decrypted vector
    println!("sorted vector = {:?}", clear_v);
}
