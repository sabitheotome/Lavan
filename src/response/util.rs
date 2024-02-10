macro_rules! try_op {
    ($expr:expr) => {
        match $expr.branch() {
            std::ops::ControlFlow::Continue(it) => it,
            std::ops::ControlFlow::Break(err) => {
                return $crate::response::traits::Pseudotriable::from_residual(err)
            }
        }
    };
}
pub(crate) use try_op;
