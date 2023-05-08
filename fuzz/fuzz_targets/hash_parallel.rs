#![no_main]
use std::collections::HashSet;

use libfuzzer_sys::fuzz_target;
use boomphf::Mphf;

fuzz_target!(|value: &[u8]| {
    let data: Vec<_> = value.into_iter().collect::<HashSet<_>>().into_iter().collect();
    let phf = Mphf::new_parallel(2.0, &data, None);
    
    for val in data {
        phf.hash(&val);
    }
});
