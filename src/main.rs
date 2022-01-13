use data_encoding::{BASE32, BASE64};
use oath::{totp_raw_custom_time, HashType};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let secret = get_secret();
    let rhex: Vec<u8> = if secret.contains("=") {
        BASE64.decode(secret.as_bytes()).unwrap()
    } else {
        BASE32.decode(secret.as_bytes()).unwrap()
    };
    let current_time: u64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Clock may have gone backwards")
        .as_secs();
    let rstring: u64 = totp_raw_custom_time(&rhex, 10, 0, 30, current_time, &HashType::SHA1);
    let passcode: String = base26(&rstring);

    let exe_path = std::env::current_exe().unwrap();
    let passcode_path = {
        let target_dir_path = exe_path.parent().unwrap();
        target_dir_path.join(std::path::Path::new("passcode.txt"))
    };

    match std::fs::write(passcode_path, passcode) {
        Ok(_) => println!("{}", "created passcode file successfully!"),
        Err(e) => eprintln!("{}", e),
    }
}

#[cfg(not(feature = "predefined_secret"))]
fn get_secret() -> String {
    match std::env::var("SECRET") {
        Ok(result) => return result.to_string(),
        Err(err) => {
            let args: Vec<String> = std::env::args().collect();
            if args.len() > 1 {
                return args[1].clone().to_string();
            } else {
                eprintln!(
                    "Please pass args for secret or set `SECRET` environment variable!\n{}",
                    err
                );
                std::process::exit(1);
            };
        }
    };
}

fn base26(num: &u64) -> String {
    let mut encode = num.clone();
    let mut decode = String::new();
    let chars: Vec<char> = "23456789BCDFGHJKMNPQRTVWXY".chars().collect();
    for _ in 0..5 {
        let pchar = chars[(encode as usize).wrapping_rem(chars.len())];
        decode.push(pchar);
        encode = encode.wrapping_div(chars.len() as u64);
    }
    decode
}
