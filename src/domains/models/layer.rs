#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Layer {
    Meta,
    Abstract,
    SpecDetail,
    Implementation,
}
