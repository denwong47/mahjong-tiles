use base64::Engine;
use std::{fs, io, path};

static SVG_MOD: &str = include_str!("src/_templates/svg_mod.rs");
static SVG_TEMPLATE: &str = include_str!("src/_templates/svg_template.rs");

/// Get the constant name for the given name.
fn get_constant_name(name: &str) -> String {
    name.to_uppercase().replace("-", "_")
}

struct SvgDeclaration {
    path: path::PathBuf,
    svg_base64: String,
}

impl SvgDeclaration {
    /// Create a new SVG declaration.
    pub fn new(path: path::PathBuf) -> Self {
        let svg_base64 = base64::engine::general_purpose::STANDARD.encode(
            std::fs::read_to_string(&path)
                .unwrap_or_else(|_| panic!("Could not read the contents of the file {path:?}.")),
        );
        Self { path, svg_base64 }
    }

    /// Convert a name to a constant format, which is uppercase and uses underscores.
    pub fn constant_name(&self) -> io::Result<String> {
        let path = &self.path;
        Ok(get_constant_name(
            &path
                .file_stem()
                .ok_or(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("{path:?} does not have a valid stem."),
                ))?
                .to_string_lossy(),
        ))
    }

    /// Get the declaration of the SVG.
    pub fn declaration(&self) -> io::Result<String> {
        let path = &self.path;
        let file_name = path
            .file_name()
            .ok_or(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("{path:?} does not have a valid file name."),
            ))?
            .to_string_lossy();
        let constant_name = self.constant_name()?;
        let svg_base64 = &self.svg_base64;
        Ok(format!(
            "/// Base64 encoded SVG contents, generated from {file_name}.\npub const {constant_name}: &str = \"data:image/svg+xml;base64,{svg_base64}\";",
        ))
    }

    /// Match the SVG name stem to a declaration.
    pub fn match_to_declaration(&self) -> io::Result<String> {
        let path = &self.path;
        let file_stem = path
            .file_stem()
            .ok_or(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("{path:?} does not have a valid file stem."),
            ))?
            .to_string_lossy();
        let constant_name = self.constant_name()?;

        Ok(format!("        {file_stem:?} => Some({constant_name}),",))
    }
}

/// Generate a svg template.
fn generate_svg_template(source: path::PathBuf, output_dir: &path::Path) -> io::Result<String> {
    let canonicalized_source = source.canonicalize()?;
    let source_path = canonicalized_source.to_string_lossy();
    let mod_name = source.file_stem().unwrap().to_string_lossy().to_string();

    let mut declarations = fs::read_dir(source)?
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.is_file() && path.extension()?.to_string_lossy().to_ascii_lowercase() == "svg" {
                Some(path)
            } else {
                None
            }
        })
        .map(SvgDeclaration::new)
        .collect::<Vec<SvgDeclaration>>();

    assert_eq!(declarations.len(), 37, "There should be 37 SVG files.");

    declarations.sort_by_key(|declaration| {
        declaration
            .constant_name()
            .expect("Could not get the constant name.")
    });

    let template = SVG_TEMPLATE
        .replace("{{SOURCE_PATH}}", &source_path)
        .replace(
            "{{DECLARATIONS}}",
            &declarations
                .iter()
                .map(|declaration| declaration.declaration())
                .collect::<io::Result<Vec<_>>>()?
                .as_slice()
                .join("\n\n"),
        )
        .replace(
            "{{MATCHES}}",
            &declarations
                .iter()
                .map(|declaration| declaration.match_to_declaration())
                .collect::<io::Result<Vec<_>>>()?
                .as_slice()
                .join("\n"),
        );

    std::fs::write(output_dir.join(format!("{mod_name}.rs")), template)?;

    Ok(mod_name)
}

// Example custom build script.
fn main() -> io::Result<()> {
    // Tell Cargo that if the SVG file change, to rerun this build script.
    println!("cargo::rerun-if-changed=svg/dark/*.svg");
    println!("cargo::rerun-if-changed=svg/light/*.svg");
    println!("cargo::rerun-if-changed=src/_templates/svg_template.rs");
    println!("cargo::rerun-if-changed=build.rs");

    let out_dir = std::env::var("OUT_DIR").expect(
        "Failed to get the output directory. Please make sure that the environment variable `OUT_DIR` is set.",
    );

    let svg_dir = path::PathBuf::from(&out_dir).join("svg");
    if !svg_dir.exists() {
        fs::create_dir(&svg_dir)?;
    }
    let module_names = ["dark", "light"]
        .iter()
        .map(|theme| {
            let source = path::PathBuf::from(format!("svg/{}", theme));
            generate_svg_template(source, &svg_dir)
        })
        .collect::<io::Result<Vec<String>>>()?;

    let template = SVG_MOD
        .replace(
            "{{MODULES}}",
            &module_names
                .iter()
                .map(|name| format!("pub mod {name};"))
                .collect::<Vec<String>>()
                .join("\n"),
        )
        .replace(
            "{{STYLES}}",
            &module_names
                .iter()
                .map(|name| {
                    let constant_name = get_constant_name(name);
                    format!("    /// SVG style for the {name} theme.\n    {constant_name},")
                })
                .collect::<Vec<String>>()
                .join(""),
        )
        .replace(
            "{{MATCH}}",
            &module_names
                .iter()
                .map(|name| {
                    let constant_name = get_constant_name(name);
                    format!(
                        "            SvgStyle::{} => {name}::get_svg_by_name(name),",
                        constant_name
                    )
                })
                .collect::<Vec<String>>()
                .join("\n"),
        );

    std::fs::write(svg_dir.join("mod.rs"), template)?;

    Ok(())
}
