use csv::Writer;
use ironworks::Ironworks;
use ironworks::sestring::format::Input;
use ironworks::sestring::format::Player;
use ironworks::sestring::format::Gender;
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::path::Path;

use ironworks::excel::{Excel, Field, Language};
use ironworks::file::exh::{ColumnDefinition, SheetKind};

use crate::exd_schema::field_names;
use crate::formatter::format_string;

/// Generates a CSV extract for the given sheet and language
pub fn sheet(excel: &Excel, language: Language, sheet_name: &str) -> Result<(), Box<dyn Error>> {
    // Set up the Input for parsing sestrings
    let input = Input::new().with_global_parameter(1, String::from("Player Player")); // Player name

    // Fetch the sheet data
    let sheet = excel.sheet(sheet_name)?;
    let has_subrows = sheet.kind()? == SheetKind::Subrows;

    // Sort by offset to align with EXDSchema column order
    let mut columns = sheet.columns()?;
    columns.sort_by_key(|column| column.offset);

    // Set up the output file
    let language_code = language_code(&language);
    let path = format!("output/{}/{}.csv", language_code, sheet_name);
    if let Some(parent) = Path::new(&path).parent() {
        fs::create_dir_all(parent)?;
    }
    let mut writer =
        Writer::from_path(&path).expect(format!("Failed to open output file: {}", &path).as_str());

    // Write the field header
    match field_names(sheet_name)? {
        Some(names) => writer.serialize(&names)?,
        None => (),
    };

    // Write the file data
    for row in sheet.into_iter() {
        let row = &row?;
        let mut data: Vec<String> = Vec::new();

        let id = match has_subrows {
            true => format!("{}.{}", row.row_id(), row.subrow_id()),
            false => row.row_id().to_string(),
        };

        data.push(id);

        for column in columns.iter() {
            let specifier = ColumnDefinition {
                kind: column.kind,
                offset: column.offset,
            };
            let field = row.field(&specifier)?;

            data.push(field_to_string(&field, &input));
        }

        match writer.serialize(data) {
            Ok(_) => (),
            Err(err) => {
                return Err(format!(
                    "{err}. For differing field counts, try adding Unknown columns to the schema.",
                )
                .into());
            }
        }
    }

    writer
        .flush()
        .expect(format!("Failed to write output file: {}", &path).as_str());

    return Ok(());
}

/// Generates a CSV extract for the given sheet and language
pub fn quest_sheet(excel: &Excel, language: Language, sheet_name: &str) -> Result<(), Box<dyn Error>> {
    // Set up the Input for parsing sestrings
    let input = Input::new();

    // Fetch the sheet data
    let sheet = excel.sheet(sheet_name)?;

    // Sort by offset to align with EXDSchema column order
    let mut columns = sheet.columns()?;
    columns.sort_by_key(|column| column.offset);

    // Set up the output file
    let language_code = language_code(&language);
    let path = format!("output/{}/{}_actor.csv", language_code, sheet_name);
    if let Some(parent) = Path::new(&path).parent() {
        fs::create_dir_all(parent)?;
    }
    let mut writer =
        Writer::from_path(&path).expect(format!("Failed to open output file: {}", &path).as_str());
    
    let field_name = field_names(sheet_name)?.expect("canno find fieldnames for Quest");
    let quest_id_index = field_name
        .iter()
        .position(|name| name == "Id")
        .ok_or("Quest Id column not found")? -1;


    // Write the file data
    for row in sheet.into_iter() {
        let row = &row?;
        let mut npc_ids: Vec<String> = Vec::new();
        let mut quest_id: String = "".to_string();

        for (column_index,column) in columns.iter().enumerate() {
            let specifier = ColumnDefinition {
                kind: column.kind,
                offset: column.offset,
            };
            let field = row.field(&specifier)?;
            if column_index == quest_id_index {
    
                quest_id = field_to_string(&field, &input);
            } else {
                let field_value = field_to_string(&field, &input);
                if field_value.starts_with("ACTOR") {
                    let npc_column = columns.get(column_index +1).expect("No npc_id here");
                    let npc_specifier = ColumnDefinition {
                        kind: npc_column.kind,
                        offset: npc_column.offset,
                    };
                    let npc_field = row.field(&npc_specifier)?;
                    let npc_id = field_to_string(&npc_field, &input);
                    npc_ids.push(npc_id);
                }
            }
        }

        for npc_id in npc_ids {
            let mut data: Vec<String> = Vec::new();
            data.push(npc_id);
            data.push(quest_id.clone());
            match writer.serialize(data) {
                Ok(_) => (),
                Err(err) => {
                    return Err(format!(
                        "{err}. For differing field counts, try adding Unknown columns to the schema.",
                    )
                    .into());
                }
            }
        }

    }

    writer
        .flush()
        .expect(format!("Failed to write output file: {}", &path).as_str());

    return Ok(());
}

/// Generates a CSV extract for the given sheet and language
pub fn quest_line_sheet(excel: &Excel, language: Language, sheet_name: &str) -> Result<(), Box<dyn Error>> {
    // Set up the Input for parsing sestrings
    let input_female_day = Input::new()
        .with_global_parameter(4, 1u32)
        .with_global_parameter(11, 9u32)
        .with_global_parameter(1, String::from("_FIRSTNAME_ _LASTNAME_"))
        .with_player(
            0,
            Player {
                name: "_FIRSTNAME_ _LASTNAME_".into(),
                gender: Gender::Female,
            }
        
        )
        .with_local_player_id(0);
    let input_female_night = Input::new()
        .with_global_parameter(4, 1u32)
        .with_global_parameter(11, 2u32)
        .with_global_parameter(1, String::from("_FIRSTNAME_ _LASTNAME_"))
        .with_player(
            0,
            Player {
                name: "_FIRSTNAME_ _LASTNAME_".into(),
                gender: Gender::Female,
            }
        
        )
        .with_local_player_id(0);
    let input_male_day = Input::new()
        .with_global_parameter(4, 0u32)
        .with_global_parameter(11, 9u32)
        .with_global_parameter(1, String::from("_FIRSTNAME_ _LASTNAME_"))
        .with_player(
            0,
            Player {
                name: "_FIRSTNAME_ _LASTNAME_".into(),
                gender: Gender::Male,
            }
        
        )
        .with_local_player_id(0);
    let input_male_night = Input::new()
        .with_global_parameter(4, 0u32)
        .with_global_parameter(11, 2u32)
        .with_global_parameter(1, String::from("_FIRSTNAME_ _LASTNAME_"))
        .with_player(
            0,
            Player {
                name: "_FIRSTNAME_ _LASTNAME_".into(),
                gender: Gender::Male,
            }
        
        )
        .with_local_player_id(0);

    // Fetch the sheet data
    let sheet = excel.sheet(sheet_name)?;
    let has_subrows = sheet.kind()? == SheetKind::Subrows;

    // Sort by offset to align with EXDSchema column order
    let mut columns = sheet.columns()?;
    columns.sort_by_key(|column| column.offset);

    // Set up the output file
    let language_code = language_code(&language);
    let path = format!("output/{}/{}.csv", language_code, sheet_name);
    if let Some(parent) = Path::new(&path).parent() {
        fs::create_dir_all(parent)?;
    }
    let mut writer =
        Writer::from_path(&path).expect(format!("Failed to open output file: {}", &path).as_str());

    let inputs = [
        ("", &input_female_day),
        ("_M_D", &input_male_day),
        ("_F_N", &input_female_night),
        ("_M_N", &input_male_night),
    ];
    let mut written_rows: HashSet<Vec<String>> = HashSet::new();

    // Write the field header
    match field_names(sheet_name)? {
        Some(names) => writer.serialize(&names)?,
        None => (),
    };

    // Write the file data
    for row in sheet.into_iter() {
        let row = row?;
        let id = match has_subrows {
            true => format!("{}.{}", row.row_id(), row.subrow_id()),
            false => row.row_id().to_string(),
        };

        let mut output_rows = [
            vec![id.clone()],
            vec![id.clone()],
            vec![id.clone()],
            vec![id],
        ];

        for column in &columns {
            let specifier = ColumnDefinition {
                kind: column.kind,
                offset: column.offset,
            };
            let field = row.field(&specifier)?;
            let values: Vec<String> = inputs
                .iter()
                .map(|(_, input)| field_to_string(&field, input))
                .collect();

            for (output, value) in output_rows.iter_mut().zip(values) {
                output.push(value);
            }
        }

        let mut seen_contents: HashSet<Vec<String>> = HashSet::new();

        for (variant, mut output) in output_rows.into_iter().enumerate() {
            let contents = output[1..].to_vec();

            if seen_contents.insert(contents) {
                output[0].push_str(inputs[variant].0);

                if written_rows.insert(output.clone()) {
                    writer.serialize(output)?;
                }
            }
        }

    }

    writer
        .flush()
        .expect(format!("Failed to write output file: {}", &path).as_str());

    return Ok(());
}

// Workaround borrowed from boilmaster to check for file existence
// See: https://github.com/ackwell/boilmaster/blob/main/crates/bm_asset/src/service.rs
struct FileExists;
impl ironworks::file::File for FileExists {
    fn read(_stream: impl ironworks::FileStream) -> Result<Self, ironworks::Error> {
        Ok(Self)
    }
}

/// Returns the languages available to export
pub fn available_languages(ironworks: &Ironworks) -> Vec<Language> {
    return ironworks
        .file::<ironworks::file::exh::ExcelHeader>("exd/Item.exh") // Read the headers from an arbitrary sheet
        .expect("Could not read available languages from: exd/Item.exh")
        .languages
        .into_iter()
        .map(Language::from)
        .filter(|language| {
            // Check if the sheet exists for a given language. The Global version's `EXcelHeader`s indicates all
            // languages that are supported, even though the files aren't present besides EN, DE, FR, and JA.
            ironworks
                .file::<FileExists>(&format!("exd/Item_0_{}.exd", self::language_code(language)))
                .is_ok()
        })
        .collect();
}

/// Returns a short code for the given language
pub fn language_code(language: &Language) -> &str {
    return match language {
        Language::English => "en",
        Language::German => "de",
        Language::French => "fr",
        Language::Japanese => "ja",
        Language::ChineseSimplified => "chs",
        Language::Korean => "ko",
        Language::ChineseTraditional => "tc",
        _ => "??",
    };
}

/// Transforms the given field to a string
fn field_to_string(field: &Field, input: &Input) -> String {
    return match field {
        Field::String(value) => format_string(value, input),
        Field::Bool(value) => {
            if *value {
                String::from("True")
            } else {
                String::from("False")
            }
        }
        Field::I8(value) => value.to_string(),
        Field::I16(value) => value.to_string(),
        Field::I32(value) => value.to_string(),
        Field::I64(value) => value.to_string(),
        Field::U8(value) => value.to_string(),
        Field::U16(value) => value.to_string(),
        Field::U32(value) => value.to_string(),
        Field::U64(value) => value.to_string(),
        Field::F32(value) => value.to_string(),
    };
}
