macro_rules! try_op {
    ($expr:expr) => {
        match $expr.control_flow() {
            std::ops::ControlFlow::Continue(it) => it,
            std::ops::ControlFlow::Break(err) => {
                return $crate::response::traits::Response::from_error(err)
            }
        }
    };
}
pub(crate) use try_op;
