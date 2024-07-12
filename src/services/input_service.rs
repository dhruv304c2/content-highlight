use std::io::{self, Write};
use regex::Regex;

pub struct InputService;

impl InputService{

    pub fn get_input<T>(
        prompt: String, 
        default_val : T) -> T 
    where T : std::str::FromStr {

        let mut parsed = false;
        let mut inp = String::new();
        let mut paresed_val = default_val;

        print!("{}", prompt);
        let mut retry_count = 0;
        while !parsed {
            if retry_count > 0 {
                println!("in-valid try again");
            }
            retry_count += 1;
            _ = io::stdout().flush();
            inp.clear();
            let _ = io::stdin().read_line(&mut inp);
            inp = inp.trim().to_string();

            let parse_result = inp.parse::<T>();

            match parse_result {
                Ok(val) => {
                    paresed_val = val;
                    break;
                }
                Err(_) => {
                    parsed = false;
                }
            }
        }
        return paresed_val;
    }

    pub fn get_string(prompt: String,default_val : String, validation : Validation) ->  String{
        let mut success = false;
        let mut mod_prompt = prompt.clone();
        while !success {
            let inp = Self::get_input(mod_prompt.clone(), default_val.clone());
            match validation {
                Validation::None => {
                    success = true;
                    if success {
                        return inp;
                    }
                }
                Validation::VideoLink => todo!(),
                Validation::YtLink => {
                    success = Self::is_valid_yt_link(inp.clone());
                    if !success {
                        mod_prompt = "Not a valid youtube link, try again".to_string();
                    }else {
                        return inp.clone();
                    }
                }
            }
        }

        return default_val;
    }

    pub fn is_valid_yt_link(link: String) -> bool{
        let re = Regex::new(r"^(https?://)?(www\.)?(youtube\.com|youtu\.?be)/.+$").unwrap();
        re.is_match(&link)
    }
}

pub enum Validation{
        None,
        VideoLink,
        YtLink,
}
