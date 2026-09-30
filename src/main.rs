
use spindle::network::Graph;

fn main() {

    ///! We index the vertices simply by their number
    type V = u32;
    let _ = Graph::<V>::new(false);
}
