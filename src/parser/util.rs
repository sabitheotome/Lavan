pub(crate) mod assoc {
    macro_rules! val {
        ($ident:ident) => {
            <$ident::Output as $crate::response::traits::Response>::Value
        };
        ($ident:ident<$param:ty>) => {
			<$ident::Output as $crate::response::traits::Response>::WithVal<$param>
		};
    }

    macro_rules! err {
        ($ident:ident) => {
            <$ident::Output as $crate::response::traits::Response>::Error
        };
		($ident:ident<$param:ty>) => {
			<$ident::Output as $crate::response::traits::Response>::WithErr<$param>
		};
    }

    pub(crate) use err;
    pub(crate) use val;
}

pub(crate) mod gen {
    macro_rules! parser {
        ($name:ident) => {};
    }
}
