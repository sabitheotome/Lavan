pub(crate) mod assoc {
    macro_rules! val {
        ($ident:ident) => {
            <$ident::Output as $crate::output::traits::Response>::Value
        };
        ($ident:ident<$param:ty>) => {
			<$ident::Output as $crate::output::traits::Response>::WithVal<$param>
		};
        ($ident:ident :: $assoc:ident) => {
            <$ident::$assoc as $crate::output::traits::Response>::Value
        };
        ($ident:ident<$param:ty> :: $assoc:ident) => {
			<$ident::$assoc as $crate::output::traits::Response>::WithVal<$param>
		};
    }

    macro_rules! err {
        ($ident:ident) => {
            <$ident::Output as $crate::output::traits::Response>::Error
        };
		($ident:ident<$param:ty>) => {
			<$ident::Output as $crate::output::traits::Response>::WithErr<$param>
		};
        ($ident:ident :: $assoc:ident) => {
            <$ident::$assoc as $crate::output::traits::Response>::Error
        };
		($ident:ident<$param:ty> :: $assoc:ident) => {
			<$ident::$assoc as $crate::output::traits::Response>::WithErr<$param>
		};
    }

    pub(crate) use err;
    pub(crate) use val;
}
