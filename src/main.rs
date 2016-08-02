extern crate hyper;

use std::env;
use std::io::Read;

fn main() {
    println!("Going to make get request");
    let client = hyper::Client::new();
    if let Some(arg1) = env::args().nth(1) {
        let hostname = arg1;
        let mut res = client.get(&hostname).send().unwrap();
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        assert_eq!(res.status, hyper::Ok);
        println!("Status: {}", res.status);
        println!("Body: {}", body);
    }
}
