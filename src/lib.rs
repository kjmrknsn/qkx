extern crate getopts;

use std::collections::HashMap;
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use getopts::Options;

#[derive(Debug)]
struct Config {
    file_path: String,
}

impl Config {
    fn from(args: Vec<String>) -> Result<Config, String> {
        let mut opts = Options::new();

        opts.optopt("f", "file", "file path", "/file/path");

        let matches = match opts.parse(&args[1..]) {
            Ok(m) => m,
            Err(f) => { return Err(opts.usage(&f.to_string())); },
        };

        let mut conf = Config {
            file_path: String::new(),
        };

        match matches.opt_str("f") {
            Some(f) => { conf.file_path = f; },
            None => { return Err(opts.usage("file path is not specified")); },
        }

        Ok(conf)
    }
}

pub struct Extractor {
    conf: Config,
}

impl Extractor {
    pub fn from(args: Vec<String>) -> Result<Extractor, String> {
        let conf = match Config::from(args) {
            Ok(c) => c,
            Err(e) => {
                return Err(e);
            },
        };

        Ok(Extractor {
            conf: conf,
        })
    }

    pub fn run(&self) -> Result<(), Box<Error>> {
        let mut f = File::open(&self.conf.file_path)?;

        let mut queries = String::new();
        f.read_to_string(&mut queries)?;

        let query_num = queries.len();

        let words: Vec<&str> = queries.split(&[' ', '\t', '\r', '\n', '(', ')', ',', '\'', '"', '*', '/'][..]).filter(|s| s.len() > 0).collect();

        let word_num = words.len();

        let mut word_counts: HashMap<String, usize> = HashMap::new();

        for word in words {
            *word_counts.entry(word.to_lowercase()).or_insert(0) += 1;
        }

        let mut word_counts_vec: Vec<(&String, &usize)> = word_counts.iter().collect();
        word_counts_vec.sort_by_key(|k| k.1);
        word_counts_vec.reverse();

        println!("query_num: {}, word_num: {}", query_num, word_num);
        println!("rank word count");
        for (i, &(word, count)) in word_counts_vec.iter().enumerate() {
            println!("{} {} {} ({:.4}%)", i, word, count, (*count as f64 / word_num as f64) * 100f64);
        }

        Ok(())
    }
}
