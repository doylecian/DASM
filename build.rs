use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;

// Used to generate code for mapping from opcodes to instructions. Much easier than manually defining a function or static map with all the match arms.
// http://ref.x86asm.net/coder32.html used as a reference for opcodes
fn main() {
    let content = fs::read_to_string("opcodes.csv").expect("Unable to read file");

    let arms: String = content
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split(",").map(|l| l.chars().filter(|c| !c.is_whitespace()).collect::<String>()).collect::<Vec<_>>();
            let (key, value) = (parts[0].clone(), parts[1].clone());
            format!(
                r#"0x{:02X} => X86Instruction::{},"#,
                u8::from_str_radix(&key, 16).unwrap_or(0),
                value
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let opcodes = content
        .lines()
        .map(|line| {
            line.split(",")
                .map(|l| l.chars().filter(|c| !c.is_whitespace()).collect::<String>())
                .collect::<Vec<_>>()[1]
                .clone()
        })
        .collect::<HashSet<_>>()
        .iter()
        .map(|f| format!(r#"{},"#, f))
        .collect::<Vec<_>>()
        .join("\n");

    let generated_code = format!(
        r#"

        #[derive(Debug, PartialEq)]
        pub enum X86Instruction {{
            {}
        }}


        pub fn _x86_byte_to_instruction(input: u8) -> X86Instruction {{
            match input {{
                {}
                _ => panic!("Unknown value!"),
            }}
        }}
        "#,
        opcodes, arms
    );

    // Write the generated code to a file
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("x86_generated.rs");
    fs::write(dest_path, generated_code).expect("Unable to write file");
}
