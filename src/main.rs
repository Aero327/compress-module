use std::{env, fs, process, io::prelude::*};
use std::fs::File;
use brotli2::read::{BrotliDecoder, BrotliEncoder};
use compress_module::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(args.into_iter()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    };
}

// урааа чат гпт рулит
fn run(config: Config) -> std::io::Result<()> {
    let file_contents = fs::read_to_string(config.file_name.clone())?;
    let data = file_contents.as_bytes();

    // Создаем буфер для хранения сжатых данных
    let mut compressed_data = Vec::new();
    {
        // Создаем компрессор и записываем данные в буфер
        let mut compressor = BrotliEncoder::new(data, config.compress_level);
        compressor.read_to_end(&mut compressed_data)?;
    }

    // Записываем сжатые данные в новый файл
    let new_file_name = config.file_name.clone() + ".br";
    let mut file_write = File::create(&new_file_name)?;
    file_write.write_all(&compressed_data)?;

    // Декомпрессируем данные и проверяем их эквивалентность
    let mut decompressor = BrotliDecoder::new(&compressed_data[..]);
    let mut contents = String::new();
    decompressor.read_to_string(&mut contents)?;

    assert_eq!(contents, file_contents);

    Ok(())
}
