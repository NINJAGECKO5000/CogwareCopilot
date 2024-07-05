#[macro_export]
macro_rules! gpio_pins {
    ($($PXi:ident),+) => {
        $(
        #[derive(Debug)]
        pub struct $PXi {
            reg: *const crate::pac::gpio::RegisterBlock,
        }

        impl core::ops::Deref for $PXi {
            type Target = crate::pac::gpio::RegisterBlock;
            fn deref(&self) -> &Self::Target {
                unsafe { &*(self.reg as *const Self::Target) }
            }
        })+
    };
}

#[macro_export]
macro_rules! gpfsel {
    ($gpfseln:ident, [$($PXi:ident: {$FSELnA:ident,$fseln:ident,$alt0:ident,$alt1:ident,$alt2:ident,$alt3:ident,$alt4:ident,$alt5:ident}),+]) => {
        $(
        paste! {
            impl $PXi {
                pub fn into_input(&self) {
                    self.$gpfseln().modify(|_, w| w.$fseln().input());
                }

                pub fn into_output(&self) {
                    self.$gpfseln().modify(|_, w| w.$fseln().output());
                }

                pub fn [<into_ $alt0>](&self) {
                    self.$gpfseln().modify(|_, w| w.$fseln().$alt0());
                }

                pub fn [<into_ $alt1>](&self) {
                    self.$gpfseln().modify(|_, w| w.$fseln().$alt1());
                }

                pub fn [<into_ $alt2>](&self) {
                    self.$gpfseln().modify(|_, w| w.$fseln().$alt2());
                }

                pub fn [<into_ $alt3>](&self) {
                    self.$gpfseln().modify(|_, w| w.$fseln().$alt3());
                }

                pub fn [<into_ $alt4>](&self) {
                    self.$gpfseln().modify(|_, w| w.$fseln().$alt4());
                }

                pub fn [<into_ $alt5>](&self) {
                    self.$gpfseln().modify(|_, w| w.$fseln().$alt5());
                }

                pub fn variant(&self) -> crate::pac::gpio::$gpfseln::$FSELnA {
                    self.$gpfseln().read().$fseln().variant()
                }

                pub fn is_input(&self) -> bool {
                    self.$gpfseln().read().$fseln().is_input()
                }

                pub fn is_output(&self) -> bool {
                    self.$gpfseln().read().$fseln().is_output()
                }

                pub fn [<is_ $alt0>](&self) -> bool {
                    self.$gpfseln().read().$fseln().[<is_ $alt0>]()
                }

                pub fn [<is_ $alt1>](&self) -> bool {
                    self.$gpfseln().read().$fseln().[<is_ $alt1>]()
                }

                pub fn [<is_ $alt2>](&self) -> bool {
                    self.$gpfseln().read().$fseln().[<is_ $alt2>]()
                }

                pub fn [<is_ $alt3>](&self) -> bool {
                    self.$gpfseln().read().$fseln().[<is_ $alt3>]()
                }

                pub fn [<is_ $alt4>](&self) -> bool {
                    self.$gpfseln().read().$fseln().[<is_ $alt4>]()
                }

                pub fn [<is_ $alt5>](&self) -> bool {
                    self.$gpfseln().read().$fseln().[<is_ $alt5>]()
                }
            }
        })+
    };
}

#[macro_export]
macro_rules! gpset {
    ($gpsetn:ident, $gpclrn:ident, [$($PXi:ident: {$setn:ident,$clrn:ident}),+]) => {
    $(
        impl $PXi {
            pub fn set(&self) {
                unsafe { self.$gpsetn().write_with_zero(|w| w.$setn().set_bit()) }
            }

            pub fn clr(&self) {
                unsafe { self.$gpclrn().write_with_zero(|w| w.$clrn().clear_bit_by_one()) }
            }
        })+
    };
}

#[macro_export]
macro_rules! gpio_pup_pdn {
    ($gppupdnn:ident, [$($PXi:ident: {$ctrln:ident}),+]) => {
        $(
        impl $PXi {
            pub fn pup_pdn_none(&self) {
                self.$gppupdnn().modify(|_, w| w.$ctrln().none());
            }

            pub fn pup_pdn_up(&self) {
                self.$gppupdnn().modify(|_, w| w.$ctrln().up());
            }

            pub fn pup_pdn_down(&self) {
                self.$gppupdnn().modify(|_, w| w.$ctrln().down());
            }
        })+
    };
}
