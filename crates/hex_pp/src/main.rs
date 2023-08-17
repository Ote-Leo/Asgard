use hex_pp::*;


fn main() {
    let msg_2 = String::from("Hello, World!\nThis is a long ass message!\nWow!!");
    let hex_conf = HexConfig {
        title: true,
        ascii: true,
        width: 16,
        group: 4,
        chunk: 1,
        display_offset: 0,
        ..HexConfig::default()
    };

    println!("{:?}", msg_2.hex_conf(hex_conf));
}