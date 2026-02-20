#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RenamingTarget {
    DataSource(usize),
    Sheet(usize, usize),
    Row(usize),
    Column(usize),
}
