macro_rules! wrapper {
    ( $newtype:ident, $type: ty) => {
        #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Debug)]
        struct $newtype($type);

        operator! ( impl Add/add/+ for $newtype($type) );
        operator! ( impl Sub/sub/- for $newtype($type) );

        operator_assign! ( impl AddAssign/add_assign/+= for $newtype($type) );
        operator_assign! ( impl SubAssign/sub_assign/-= for $newtype($type) );
    };
}

macro_rules! operator {
    ( impl $trait_:ident/$fn_:ident/$op:tt for $newtype:ident($type: ty)) => {
        use std::ops::$trait_;
        impl $trait_<$newtype> for $newtype {
            type Output = $newtype;

            fn $fn_(self, other: $newtype) -> $newtype {
                $newtype(self.0 $op other.0)
            }
        }
    };
}

macro_rules! operator_assign {
    ( impl $trait_:ident/$fn_:ident/$op:tt for $newtype:ident($type: ty)) => {
        use std::ops::$trait_;
        impl<'a> $trait_<&'a $newtype> for $newtype {
            fn $fn_(&mut self, other: &'a $newtype) {
                self.0 $op other.0;
            }
        }
    };
}
