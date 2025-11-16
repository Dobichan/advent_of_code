use std::path::Path;
use std::{env, fs};

fn main() {
    println!("cargo:rerun-if-changed=src/solutions");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("solution_registry.rs");
    let mut content = String::new();
    const MIN_YEAR: u16 = 2015;
    const MAX_YEAR: u16 = 2025;

    // content.push_str("use std::collections::HashMap;\n");
    // content.push_str("use crate::AoCSolution;\n");
    // for y in MIN_YEAR..=MAX_YEAR {
    //     content.push_str(&format!("use crate::solutions::y{};\n", y));
    // }
    content.push_str("type SolutionConstructor = fn() -> Box<dyn AoCSolution>;\n\n");
    content.push_str("pub fn get_solutions() -> HashMap<(u16, u8), SolutionConstructor> {\n");
    content.push_str("    let mut map = HashMap::new();\n\n");

    // for y in MIN_YEAR..=MAX_YEAR {
    //     let dir_str = format!("src/solutions/y{}", y);
    //     let solution_dir = Path::new(&dir_str);
    //     for entry in
    //         fs::read_dir(solution_dir).expect(&format!("Failed to read solutions directory {}", y))
    //     {
    //         let entry = entry.unwrap();
    //         let path = entry.path();

    //         if path.is_file() {
    //             let file = path.file_name().unwrap().to_string_lossy();
    //             let stem = path.file_stem().unwrap().to_string_lossy();

    //             if file.ends_with(".rs") && stem.starts_with("y") {
    //                 if let Ok(day) = stem[6..].parse::<u8>() {
    //                     let entry = format!(
    //                         "    map.insert(({}, {}), || {{ Box::new(solutions::y{}::y{}d{:02}::Solution {{}}) }});\n",
    //                         y, day, y, y, day
    //                     );

    //                     content.push_str(&entry);
    //                 }
    //             }
    //         }
    //     }
    // }

    for y in MIN_YEAR..=MAX_YEAR {
        for d in 1..=25 {
            let entry = format!(
                "    map.insert(({}, {}), || {{ Box::new(solutions::y{}::y{}d{:02}::Solution {{}}) }});\n",
                y, d, y, y, d
            );
            content.push_str(&entry);
        }
    }

    content.push_str("\n    map\n");
    content.push_str("}\n");

    fs::write(&dest_path, content).unwrap();
}
