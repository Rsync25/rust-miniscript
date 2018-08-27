extern crate script_descriptor;

use script_descriptor::{Descriptor, Policy, DummyKey};

use std::str::FromStr;

fn do_test(data: &[u8]) {
    let data_str = String::from_utf8_lossy(data);
    if let Ok(pol) = &Policy::<DummyKey>::from_str(&data_str) {
        // Compile
        let desc = pol.compile();
        // Try to roundtrip the output of the compiler
        let output = desc.to_string();
        if let Ok(desc) = &Descriptor::<DummyKey>::from_str(&output) {
            let rtt = desc.to_string();
            assert_eq!(output, rtt);
        } else {
            panic!("compiler output something unparseable: {}", output)
        }
    }
}

#[cfg(feature = "afl")]
extern crate afl;
#[cfg(feature = "afl")]
fn main() {
    afl::read_stdio_bytes(|data| {
        do_test(&data);
    });
}

#[cfg(feature = "honggfuzz")]
#[macro_use] extern crate honggfuzz;
#[cfg(feature = "honggfuzz")]
fn main() {
    loop {
        fuzz!(|data| {
            do_test(data);
        });
    }
}

#[cfg(test)]
mod tests {
    fn extend_vec_from_hex(hex: &str, out: &mut Vec<u8>) {
        let mut b = 0;
        for (idx, c) in hex.as_bytes().iter().enumerate() {
            b <<= 4;
            match *c {
                b'A'...b'F' => b |= c - b'A' + 10,
                b'a'...b'f' => b |= c - b'a' + 10,
                b'0'...b'9' => b |= c - b'0',
                _ => panic!("Bad hex"),
            }
            if (idx & 1) == 1 {
                out.push(b);
                b = 0;
            }
        }
    }

    #[test]
    fn duplicate_crash() {
        let mut a = Vec::new();
        extend_vec_from_hex("706b286b172829f1", &mut a);
        super::do_test(&a);
    }
}
