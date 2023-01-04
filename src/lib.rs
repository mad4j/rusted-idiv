mod checked {
    use num::{traits::CheckedRem, CheckedDiv};

    pub trait CheckedIdiv: Sized + CheckedDiv + CheckedRem {
        fn checked_idiv(&self, v: &Self) -> Option<Self>;
    }

    impl CheckedIdiv for i64 {
        fn checked_idiv(&self, lsh: &Self) -> Option<Self> {
            match self.checked_rem(lsh) {
                Some(v) => {
                    if v == 0 {
                        self.checked_div(lsh)
                    } else {
                        None
                    }
                }
                // occurs when lsh == 0
                None => None,
            }
        }
    }
}

mod unchecked {

    
}
