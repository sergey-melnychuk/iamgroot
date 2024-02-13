use crate::gen::Rpc;

fn main() {
    let client = gen::client::Client::new("https://eth.llamarpc.com");
    let block = client
        .getBlockByNumber(gen::GetBlockByNumberIncludetransactions(false))
        .unwrap();
    println!("{block:#?}");
}
