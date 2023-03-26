use anyhow::Result;
use rand::Rng;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct IPAddress<'a> {
    urls: Vec<(&'a str, &'a str)>,
    rng: rand::rngs::ThreadRng,
}

impl<'a> IPAddress<'a> {
    pub fn new() -> IPAddress<'a> {
        IPAddress {
            urls: vec![
                ("https://httpbin.org/ip", "origin"),
                ("https://api.ipify.org/?format=json", "ip"),
                ("https://ip4.seeip.org/json", "ip"),
                ("https://api4.my-ip.io/ip.json", "ip"),
            ],
            rng: rand::thread_rng(),
        }
    }

    pub fn fetch_ip_address(&mut self) -> Result<String> {
        let index = &self.rng.gen_range(0..self.urls.len());

        let value = self.urls[index.clone()];

        let resp = reqwest::blocking::get(value.0)?.json::<HashMap<String, String>>()?;

        Ok(resp.get(value.1).unwrap().clone())
    }
}
