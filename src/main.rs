use clap::{Parser, ArgAction};
use filepath::FilePath;
use glob::glob;
use log::error;
use parser::find_between;
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf}, error::Error,
};

use arboard::Clipboard;

#[derive(Parser)]
struct Args {
    #[arg(short = 'f', long = "file", help = "File to compile")]
    file_path: String,
    #[arg(short = 'o', long = "output", help = "Output file", required = false)]
    output_path: Option<String>,
    #[arg(short = 'c', long = "clip", action = ArgAction::SetTrue, help = "Redirect output to a clipboard (On Linux, use xclip)")]
    use_clipboard: bool,
}

fn process_includes(mut code: String, file_path: PathBuf) -> Result<String, Box<dyn Error>> {
    let includes: Vec<String> = find_between(&code, "//include<", ">");

    for include in includes {
        let mut include_absolute: PathBuf = Path::new(include.as_str()).to_path_buf();

        if !include_absolute.is_absolute() {
            include_absolute = Path::new(&file_path).join(&include);
        }

        let mut include_content: String = String::new();
        let mut glob = glob(include_absolute.to_str().unwrap())?.peekable();

        if glob.peek().is_none() {
            error!("Cannot find: {}", include);
            panic!();
        };

        for entry in glob {
            let mut file = OpenOptions::new()
                .read(true)
                .open( &entry? )?;

            let mut include_content_per_glob: String = String::new();

            file.read_to_string(&mut include_content_per_glob)?;

            include_content_per_glob.push('\n');

            include_content.push_str(include_content_per_glob.as_str());
        }

        code = code.replace(format!("//include<{}>", include).as_str(), &include_content);
    }

    Ok(code)
}



fn main() {
    let args: Args = Args::parse();
    pretty_env_logger::init();

    if !Path::new(&args.file_path).exists() {
        error!("Invalid file path: {}", args.file_path);
    }

    if Path::new(&args.file_path).is_dir() {
        error!("{}: Is a directory", args.file_path);
    }

    let mut file: File = OpenOptions::new().read(true).open(&args.file_path).unwrap();

    let mut source = String::new();
    file.read_to_string(&mut source).unwrap();
    let mut file_full_path: PathBuf = file.path().unwrap();
    file_full_path.pop();

    let processed = process_includes(source, file_full_path)
            .unwrap_or_else(|err| {
                error!("Got error while parsing: {}", err);
                panic!()
    });

    if args.use_clipboard {
        if cfg!(target_os = "linux") {
            error!("On Linux, use xclip instead");
            panic!();
        } else {
            Clipboard::new().unwrap().set_text(processed).unwrap();
        }
    } else {
        let path: &String = &args.output_path.unwrap_or_else(|| {
            error!("No path provided!");
            panic!();
        });
        let mut output: File = OpenOptions::new()
            .create(true)
            .write(true)
            .append(false)
            .open(path)
            .unwrap();

        output.write_all(processed.as_bytes()).unwrap();
    }
}
