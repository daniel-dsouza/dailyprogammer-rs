use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct Packet {
    remaining: usize,
    contents: HashMap<usize, String>,
}

fn main() {
    let mut pkts:HashMap<String, Packet> = HashMap::new();

    loop {
        let mut raw = String::new();
        io::stdin().read_line(&mut raw).expect("Failed to read line");

        let name = raw[0..4].trim_right().to_owned();
        // let name2 = raw[0..4].trim_right().to_owned();

        let packet = pkts.entry(String::from(name)).or_insert({
            Packet {
                remaining: raw[12..16].trim_right().parse::<usize>().unwrap().to_owned(),
                contents: HashMap::new()
            }
        });

        // not really happy with using 2 hashmaps... maybe use a Vec?
        packet.contents.insert(raw[8..12].trim_right().parse::<usize>().unwrap().to_owned(), raw[16..].trim_right().to_owned());
        packet.remaining -= 1;

        if packet.remaining == 0 {
            // println!("finished packet {}", name2);
            packet.contents.len();
            for number in 0.. packet.contents.len() {
                println!("{}", packet.contents.get(&number).unwrap())
            }
        }
    }
    
    //     let hms = packets.entry(packet.clone()).or_insert((0..length).map(|x| String::new()).collect::<Vec<_>>() as Vec<String>);
}
