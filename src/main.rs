extern crate unicode_normalization;

use unicode_normalization::UnicodeNormalization;

fn main() {
    let v: Vec<u16> = (0xff66..0xff9e).collect();
    if let Ok(s) = String::from_utf16(&v) {
        for c in s.chars() {
            let ns: String = c.to_string().nfkc().collect();
            if let Some(nc) = ns.chars().next() {
                println!("hanzen[{:>2}] = {}; /* {} => {} */",
                         c as u16 - 0xff66, nc as u16, c, ns);
            }
        }
    }
}
