use std::{fs::File, io::Write, process::Command};

use crate::{objects::svg_to_vector::svg_to_vector, objects::vector_object::VectorFeatures};

fn latex_to_svg(latex: &str, template: Option<&str>, file_name: &str) ->  String {
    let mut template_str = r#"\documentclass[preview]{standalone}
\usepackage[spanish]{babel}
\usepackage{amsmath}
\usepackage{amssymb}
\usepackage{xcolor}

\begin{document}
\textcolor[HTML]{FFFFFF}{
{}
}
\end{document}
"#;
    if template.is_some() {
        template_str = template.unwrap();
    }
    let latex = template_str.replace("{}", latex);
    let mut file = File::create(file_name).unwrap();
    file.write_all(latex.as_bytes()).unwrap();
    let mut child = Command::new("latex")
        .args([
            "-interaction=nonstopmode",
            "--shell-escape",
            "-halt-on-error",
            file_name
        ])
        .spawn()
        .unwrap();
    child.wait().unwrap();
    let mut child = Command::new("dvisvgm")
        .args([
            file_name[0..file_name.len() - 4].to_string() + ".dvi",
            "-n".to_string(),
            file_name[0..file_name.len() - 4].to_string() + ".svg"
        ])
        .spawn()
        .unwrap();
    child.wait().unwrap();
    let svg = std::fs::read_to_string(file_name[0..file_name.len() - 4].to_string() + ".svg").unwrap();
    return svg;
}


pub fn latex_to_vector(latex: &str, template: Option<&str>, file_name: &str) -> VectorFeatures {
    let svg = latex_to_svg(latex, template, file_name);
    let vec_obj = svg_to_vector(svg.as_str());
    return vec_obj;
}