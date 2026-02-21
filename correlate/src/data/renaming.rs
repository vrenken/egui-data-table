#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RenamingTarget {
    Project(usize),
    DataSource(usize),
    Sheet(usize, usize),
    Row(usize),
    Column(usize),
}
