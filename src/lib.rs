pub struct Config {
    pub file_name: String,
    pub compress_level: u32,
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next();

        let file_name = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let compress_level = match args.next() {
            Some(arg) => match arg.parse::<u32>() {
                Ok(n) => n,
                Err(_) => return Err("Couldn't parse the compression level")
            },
            None => 10 // дефолт уровень сжатия
        };

        Ok(Config {
            file_name,
            compress_level
        })
    }
}