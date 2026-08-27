use std::path::Path;
use std::{env, error::Error};

use ironworks::excel::Language;
use ironworks::{
    Ironworks,
    excel::Excel,
    sqpack::{Install, SqPack},
};
mod exd_schema;
mod export;
mod formatter;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!(
            "You must provide a game path. For example: cargo run -- \"C:\\Program Files (x86)\\Square Enix\\FINAL FANTASY XIV - A Realm Reborn\""
        );
    }

    let path = Path::new(&args[1]);

    let ironworks = Ironworks::new().with_resource(SqPack::new(Install::at(path)));
    let mut excel = Excel::new(ironworks);

    let language: Language = Language::French;
    excel.set_default_language(language);
    let sheets = excel.list().expect("Could not retrieve sheet list.");

    println!(
        "Exporting {} sheets",
        export::language_code(&language).to_uppercase()
    );

    for sheet in sheets.iter() {

        if sheet.starts_with("quest/")  && sheet.contains("04920") {
            println!(
                "Processing {} sheet...", sheet
            );
            match export::quest_line_sheet(&excel, language, &sheet) {
                Ok(_) => (),
                // Log failed sheets and continue
                Err(err) => eprintln!("Failed to export {}. {}", sheet, err),
            }
        } else if sheet == "Quest" {
            println!(
                "Processing {} sheet...", sheet
            );
            match export::quest_sheet(&excel, language, &sheet) {
                Ok(_) => (),
                // Log failed sheets and continue
                Err(err) => eprintln!("Failed to export {}. {}", sheet, err),
            }
        }
        
    }

    // Quick debugging for schema updates

    // for language in languages {
    //     excel.set_default_language(language);
    //     export::sheet(&excel, language, &String::from("Mount"))?;
    // }

    // let language = Language::English;
    // excel.set_default_language(language);
    // export::sheet(&excel, language, "Mount")?;

    Ok(())
}
