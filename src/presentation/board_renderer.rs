use crate::domain::cell::Cell;

struct BoardRenderer {
    cells: Vec<Cell>,
}

trait Renderable {
    fn render(&self) -> String;
}

// impl Renderable for BoardRenderer {
//     fn render(&self) -> String {
//         self.cells
//             .iter()
//             .map(|cell| match cell.is_alive() {
//                 true => 'O',
//                 false => '.',
//             })
//             .collect()
//     }
// }
