fn main() {
    {
        let json = include_str!("../api/0x10165f5.json");
        let mut res: iamgroot::jsonrpc::Response = serde_json::from_str(json).unwrap();
        let res: gen::Block = serde_json::from_value(res.result.take().unwrap()).unwrap();
        println!("{res:#?}");
    }
}
