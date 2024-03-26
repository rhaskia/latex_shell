use markdown::*;

pub fn render_md(file: &String) {
    let options = ParseOptions::default();
    let tree = to_mdast(file, &options);
    println!("{tree:?}");
}
