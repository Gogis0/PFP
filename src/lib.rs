use num_integer;
use std::fs;
use std::io;
use std::io::BufReader;
use std::io::Read;
use std::str;

pub mod rabin_karp {
    pub struct RabinKarp {
        i: usize,
        pub wsize: usize,
        pub window: Vec<u8>,
        pub hash: i64,
        prime: i64,
        asize: i64,
        asize_pot: i64,
        total_chars: u64,
    }

    impl RabinKarp {
        pub fn new(wsize: usize) -> RabinKarp {
            let asize = 256;
            let prime = 1999999973;
            let mut asize_pot = 1;

            for i in 1..wsize {
                asize_pot = (asize_pot * asize) % prime;
            }

            Self {
                i: 0,
                wsize: wsize,
                window: vec![0; wsize as usize],
                hash: 0,
                prime: prime,
                asize: asize,
                asize_pot: asize_pot,
                total_chars: 0,
            }
        }

        pub fn add_char(&mut self, c: &u8) -> i64 {
            let last_char = self.window[self.i] as i64;
            self.hash += self.prime - (last_char * self.asize_pot) % self.prime;
            self.hash = (self.asize * self.hash + (*c as i64)) % self.prime;
            self.window[self.i as usize] = *c;
            self.i = (self.i + 1) % self.wsize;
            self.total_chars = self.total_chars + 1;
            self.hash
        }

        pub fn get_window(&self) -> Vec<u8> {
            [&self.window[self.i..], &self.window[..self.i]].concat()
        }
    }
}

pub struct Config {
    pub w: u16,
    pub p: u16,
}

impl Config {
    pub fn new(w: u16, p: u16) -> Config {
        Config { w, p }
    }
}

pub struct Phrase<T> {
    pub id: T,
    pub size: T,
}

pub struct PFP<'a, T: num_integer::Integer> {
    pub config: &'a Config,
    pub parse: Vec<Phrase<T>>,
    pub dictionary: String,
}

impl<'a, T: num_integer::Integer> PFP<'a, T>
where
    T: num_integer::Integer,
{
    pub fn get_triggers(
        &self,
        config: &'a Config,
        filename: &'a str,
    ) -> Result<Vec<Vec<u8>>, io::Error> {
        //TODO
        Ok(Vec::new())
    }

    pub fn new(config: &'a Config, filename: &'a str) -> Result<PFP<'a, T>, io::Error> {
        let mut RK = rabin_karp::RabinKarp::new(config.w as usize);

        let mut parse = Vec::new();
        let mut dictionary = String::new();

        let mut phrases = Vec::<Vec<u8>>::new();
        let mut current_phrase = (0..config.w).map(|_| 0).collect::<Vec<u8>>();
        let mut buffer = [0; 4096]; // some initial buffer size
        let mut f = fs::File::open(filename)?;
        loop {
            // TODO: separate trigger finding from the actual parsing
            let n = f.read(&mut buffer[..])?;
            if n == 0 {
                println!("OK, the file has been read");
                break;
            }
            for i in 0..n {
                RK.add_char(&buffer[i]);
                current_phrase.push(buffer[i]);
                if (RK.hash % (config.p as i64)) == 0 {
                    phrases.push(current_phrase);
                    current_phrase = RK.get_window();
                    //println!("phrase: {:?}", std::str::from_utf8(&current_phrase).unwrap());
                }
            }
        }

        Ok(PFP {
            config,
            parse,
            dictionary,
        })
    }
}
