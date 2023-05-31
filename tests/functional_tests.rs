#[cfg(test)]
mod tests {
    use pfp::Config;
    use pfp::PFP;
    use pfp::rabin_karp::RabinKarp;
    use std::str;

    #[test]
    fn create_pfp_config() {
        let config = Config::new(5, 7);
        assert_eq!(config.w, 5);
        assert_eq!(config.p, 7);
    }

    #[test]
    fn create_pfp_struct() {
        let config = Config::new(5, 7);
        let filename = "/home/gogis/rust_projects/pfp/tests/poem.txt";
        let PFP: PFP<u32> = PFP::new(&config, filename).unwrap();
        assert_eq!(PFP.config.w, 5);
        assert_eq!(PFP.config.p, 7);
    }

    #[test]
    fn rb_create_struct() {
        let rabin_karp = RabinKarp::new(5);
        assert_eq!(rabin_karp.wsize, 5);
    }

    #[test]
    fn rb_some_add_chars() {
        let mut rabin_karp = RabinKarp::new(5);
        let small_string = "abcdefgijklmn";
        for c in small_string.chars() {
            rabin_karp.add_char(&(c as u8));
        }
        let window = &rabin_karp.get_window();
        let res = str::from_utf8(window).unwrap();
        assert_eq!(res, "jklmn");
    }

    #[test]
    fn get_triggers_yeast() {
        // TODO: make the test smaller and check the triggers somehow
        let config = Config::new(2, 10);
        let filename = "/home/gogis/rust_projects/pfp/tests/yeast.fasta";
        let mut pfp: PFP<u32> = PFP::new(&config, filename).unwrap();
    }

    #[test]
    fn parse_small_file() {
        let config = Config::new(1, 10);
        let filename = "/home/gogis/rust_projects/pfp/tests/poem.txt";
        let mut PFP: PFP<u32> = PFP::new(&config, filename).unwrap();
    }
}
