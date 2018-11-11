macro_rules! wrapper {
    ( $newtype:ident, $type: ty) => {
        #[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Debug)]
        struct $newtype($type);

        operator! ( impl (::std::ops::Add) add + for $newtype($type) );
        operator! ( impl (::std::ops::Sub) sub - for $newtype($type) );
        operator! ( impl (::std::ops::Mul) mul * for $newtype($type) );
        operator! ( impl (::std::ops::Div) div / for $newtype($type) );

        operator_assign! ( impl (::std::ops::AddAssign)/add_assign/+= for $newtype($type) );
        operator_assign! ( impl (::std::ops::SubAssign)/sub_assign/-= for $newtype($type) );
        operator_assign! ( impl (::std::ops::MulAssign)/mul_assign/-= for $newtype($type) );
        operator_assign! ( impl (::std::ops::DivAssign)/div_assign/-= for $newtype($type) );
    };
}

macro_rules! operator {
    ( impl ($($trait_:tt)*) $fn_:ident $op:tt for $newtype:ident($type: ty)) => {
        impl $($trait_)*<$newtype> for $newtype {
            type Output = $newtype;

            fn $fn_(self, other: $newtype) -> $newtype {
                $newtype(self.0 $op other.0)
            }
        }
    };
}

macro_rules! operator_assign {
    ( impl($($trait_:tt)*)/$fn_:ident/$op:tt for $newtype:ident($type: ty)) => {
        impl $($trait_)*<$newtype> for $newtype {
            fn $fn_(&mut self, other: $newtype) {
                self.0 $op other.0;
            }
        }
    };
}
