#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub enum BlockKind {
    Air,
    FullCube,
    Water,
    Lava,
}