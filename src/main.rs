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
    std::hint::black_box(());
    if args.len() < 2 {
        panic!(
            "You must provide a game path. For example: cargo run -- \"C:\\Program Files (x86)\\Square Enix\\FINAL FANTASY XIV - A Realm Reborn\""
        );
    }

    let path = Path::new(&args[1]);

    let ironworks = Ironworks::new().with_resource(SqPack::new(Install::at(path)));
    let mut excel = Excel::new(ironworks);

    let language = args
        .get(2)
        .map(String::as_str)
        .map(parse_language)
        .transpose()?
        .unwrap_or(Language::English);
    excel.set_default_language(language);
    let sheets = excel.list().expect("Could not retrieve sheet list.");

    println!(
        "Exporting {} sheets",
        export::language_code(&language).to_uppercase()
    );

    for sheet in sheets.iter() {

        if  sheet.starts_with("quest/") ||
            sheet.starts_with("custom/") ||
            sheet.starts_with("wrap/") ||
            sheet.starts_with("shop/") ||
            sheet.starts_with("system/ExitRange") ||
            sheet.starts_with("raid/") ||
            sheet.starts_with("transport/") ||
            sheet.starts_with("warp/") ||
            sheet.starts_with("opening/") ||
            sheet.starts_with("leve/") ||
            sheet.starts_with("guild_order/") ||
            sheet.starts_with("dungeon/") ||
            sheet.starts_with("cut_scene/") ||
            sheet.starts_with("content/") {
            // println!(
            //     "Processing {} quest sheet...", sheet
            // );
            match export::quest_line_sheet(&excel, language, &sheet) {
                Ok(_) => (),
                // Log failed sheets and continue
                Err(err) => eprintln!("Failed to export {}. {}", sheet, err),
            }
        } else if sheet == "Quest" {
            // println!(
            //     "Processing {} quest map sheet...", sheet
            // );
            
            match export::quest_sheet(&excel, language, &sheet) {
                 Ok(_) => (),
                 // Log failed sheets and continue
                 Err(err) => eprintln!("Failed to export {}. {}", sheet, err),
            }
            match export::sheet(&excel, language, &sheet) {
                Ok(_) => (),
                // Log failed sheets and continue
                Err(err) => eprintln!("Failed to export {}. {}", sheet, err),
            }
        } else {
            // println!(
            //     "Processing {} sheet...", sheet
            // );
            match export::sheet(&excel, language, &sheet) {
                Ok(_) => (),
                // Log failed sheets and continue
                Err(err) => eprintln!("Failed to export {}. {}", sheet, err),
            }
        }
        
    }

    Ok(())
}

fn parse_language(code: &str) -> Result<Language, String> {
    match code.to_ascii_lowercase().as_str() {
        "en" => Ok(Language::English),
        "de" => Ok(Language::German),
        "fr" => Ok(Language::French),
        "ja" => Ok(Language::Japanese),
        "chs" => Ok(Language::ChineseSimplified),
        "ko" => Ok(Language::Korean),
        "tc" => Ok(Language::ChineseTraditional),
        _ => Err(format!(
            "Unsupported language code '{code}'. Use one of: en, de, fr, ja, chs, ko, tc"
        )),
    }
}
