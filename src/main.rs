mod view;

use view::View;

fn main() {
    println!("Starlit!");

    let v = View::new();
   
    loop {
        v.render();
    }
}
