extern crate hyper;

use std::env;

fn main() {
    println!("Going to make get request");
    let client = hyper::Client::new();
    if let Some(arg1) = env::args().nth(1) {
        let hostname = arg1;
        let res = client.get(&hostname).send().unwrap();
        assert_eq!(res.status, hyper::Ok);
        println!("Status: {}", res.status);
    }
}
