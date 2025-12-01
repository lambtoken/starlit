mod render;

use render::View;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 3 {
        println!("Usage: {} or {} <file_path>", args[0], args[0]);
        return;
    }

    let mut v = View::new(800, 600);
    v.render();
}
