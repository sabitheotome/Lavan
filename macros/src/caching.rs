use proc_macro::TokenStream;
use std::{
    hash::{Hash, Hasher},
    io::{BufRead, Write},
    os::unix::fs::MetadataExt,
    str::FromStr,
};

pub(crate) fn cached(
    attr: TokenStream,
    ts: TokenStream,
    f: impl FnOnce(TokenStream, TokenStream) -> TokenStream,
) -> std::io::Result<TokenStream> {
    const CACHE_PATH: &str = "target/cache/lavan_proc_macro";
    let ref latest = format!("{CACHE_PATH}/latest");

    let string = attr.to_string() + &ts.to_string();
    let mut hasher = std::hash::DefaultHasher::new();
    string.hash(&mut hasher);
    let hash = hasher.finish();
    let file_content = std::fs::read(format!("{CACHE_PATH}/{hash}"));

    Ok(match file_content {
        Ok(content) => {
            TokenStream::from_str(&String::from_utf8(content).expect("erro0")).expect("error1")
        }
        Err(_) => {
            let output = f(attr, ts);
            std::fs::create_dir_all(CACHE_PATH)?;

            let now = std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs();

            std::fs::File::create(latest)?;
            let ctime = std::fs::metadata(latest)?.mtime();

            if now - ctime as u64 >= 60 {
                if let Ok(mut file) = std::fs::File::open(latest) {
                    let allowed = std::io::BufReader::new(&mut file)
                        .lines()
                        .map(|e| e.expect("error2"))
                        .collect::<Vec<_>>();
                    for file in std::fs::read_dir(CACHE_PATH)? {
                        let file = file
                            .expect("error3")
                            .file_name()
                            .to_string_lossy()
                            .into_owned();
                        if !allowed.contains(&file) {
                            std::fs::remove_file(format!("{CACHE_PATH}/{file}"))?;
                            std::fs::File::create(latest)?;
                        }
                    }
                    drop(file);
                }
            }

            let mut file = std::fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open(latest)?;
            file.write(hash.to_string().as_bytes())?;
            file.write(b"\n")?;
            file.flush()?;

            std::fs::write(format!("{CACHE_PATH}/{hash}"), output.to_string())?;
            output
        }
    })
}
