use anyhow::{Context, Result};
use clap::Parser;
use console::Term;
use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

#[derive(clap::ValueEnum, Clone)]
enum ProjectLanguage {
    /// specialized for the rust language
    Rust,
    /// generic
    Any,
}

/// Create a new Geany project in the current directory
#[derive(Parser)]
struct Cli {
    /// The programming language of the project
    #[clap(value_enum)]
    language: ProjectLanguage,
    /// Force overwrite if a project file already exists
    #[arg(short, long)]
    force: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let cw = env::current_dir().with_context(|| "could not read current directory.")?;
    let prj_name = cw.file_name().unwrap().to_str().unwrap();
    let prj_filename = format!("./{}.geany", prj_name);
    let prj_base_path = cw.to_str().unwrap();

    let path = Path::new(&prj_filename);

    if !args.force && path.exists() {
        print!(
            "geany-init: '{}' already exists, overwrite ? (y,N) ",
            prj_filename
        );
        io::stdout().flush()?;
        let term = Term::stdout();
        let ans = term.read_char().unwrap();
        println!();
        if ans != 'y' && ans != 'Y' {
            return Ok(());
        }
    }

    let mut prj_template = String::from(PROJECT_TEMPLATE);

    prj_template = prj_template.replace("%PRJ_NAME%", prj_name);
    prj_template = prj_template.replace("%PRJ_BASE_PATH%", prj_base_path);

    match args.language {
        ProjectLanguage::Rust => {
            let prj_build_menu = RUST_BUILD_MENU.replace("%PRJ_BASE_PATH%", prj_base_path);
            prj_template += &prj_build_menu;
            prj_template += RUST_PROJORG;
        }
        ProjectLanguage::Any => {}
    };

    fs::write(path, prj_template)
        .with_context(|| format!("could not write file '{}'", prj_filename))?;

    Ok(())
}

const PROJECT_TEMPLATE: &str = "
[editor]
line_wrapping=false
line_break_column=80
auto_continue_multiline=true

[file_prefs]
final_new_line=true
ensure_convert_new_lines=false
strip_trailing_spaces=false
replace_tabs=false

[indentation]
indent_width=4
indent_type=1
indent_hard_tab_width=8
detect_indent=false
detect_indent_width=false
indent_mode=2

[project]
name=%PRJ_NAME%
base_path=%PRJ_BASE_PATH%

[long line marker]
long_line_behaviour=1
long_line_column=80

[files]
current_page=0

";

const RUST_BUILD_MENU: &str = "
[build-menu]
NF_00_LB=Cargo Build
NF_00_CM=cargo build
NF_00_WD=
EX_00_LB=Cargo Run
EX_00_CM=cargo run
EX_00_WD=%PRJ_BASE_PATH%
EX_01_LB=Run
EX_01_CM=\"./%e\"
EX_01_WD=
RustFT_00_LB=Cargo Check
RustFT_00_CM=cargo check
RustFT_00_WD=
RustFT_01_LB=Cargo Build
RustFT_01_CM=cargo build
RustFT_01_WD=
RustFT_02_LB=Cargo Test
RustFT_02_CM=cargo test
RustFT_02_WD=
filetypes=Rust;
NF_03_LB=Compile
NF_03_CM=rustc \"%f\"
NF_03_WD=";

const RUST_PROJORG: &str = "
[prjorg]
source_patterns=
header_patterns=
ignored_dirs_patterns=.*;target;
ignored_file_patterns=Cargo.lock;
generate_tag_prefs=0
external_dirs=";
