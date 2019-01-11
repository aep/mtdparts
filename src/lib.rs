use std::io::BufReader;
use std::io::BufRead;
use std::io::Read;
use std::collections::HashMap;

pub fn parse_mtd<R: Read> (r: R) -> Result<HashMap<String, String>, std::io::Error> {
    let mut ret = HashMap::new();
    let reader = BufReader::new(r);
    for line in  reader.lines().skip(1) {
        if let Ok(line) = line {
            let line : Vec<&str> = line.split_whitespace().collect();
            if line.len() < 4 {
                continue;
            }
            let mtd = match line[0].trim().split(":").next() {
                Some(v) => v,
                None => continue,
            }.to_string();
            let name = line[3].trim().replace("\"","").to_string();
            ret.insert(name, mtd);
        }
    }
    Ok(ret)
}


#[test]
fn example() {
let parts = br#"dev:    size   erasesize  name
mtd0: 00020000 00010000 "factory-boot"
mtd1: 00020000 00010000 "fs-uboot"
mtd2: 00a00000 00010000 "firmware"
mtd3: 00152ba1 00010000 "kernel"
mtd4: 008ad45f 00010000 "rootfs"
mtd5: 004c0000 00010000 "genesis"
mtd6: 00000200 00010000 "mac"
mtd7: 00000200 00010000 "pin"
mtd8: 00000100 00010000 "device-id"
mtd9: 0000fb00 00010000 "product-info"
mtd10: 000b0000 00010000 "sysconf"
mtd11: 00010000 00010000 "partition-table"
mtd12: 0000a000 00010000 "support-list"
mtd13: 00000100 00010000 "soft-version"
mtd14: 00001000 00010000 "extra-para"
mtd15: 00000400 00010000 "identity"
mtd16: 00010000 00010000 "art"
"#;
let r = parse_mtd(parts as &[u8]).unwrap();
assert!(r["device-id"] == "mtd8");
assert!(r["identity"] == "mtd15");
assert!(r["art"] == "mtd16");
assert!(r.get("mtd13") == None);
}
