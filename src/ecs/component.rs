use crate::render::drawble::*;
use crate::transform::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Component {
    Transform(Transform),
    Drawble(Drawble),
}
