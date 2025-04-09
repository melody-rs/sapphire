macro_rules! def_val_prop {
    ($field:ident -> $value_ty:ty) => {
        fn $field(&self) -> $value_ty {
            let arenas = $crate::arenas::get().read();
            arenas[self.as_key()].$field
        }

        paste::paste! {
            fn [<set_ $field>](&self, val: $value_ty) {
                let mut arenas = $crate::arenas::get().write();
                arenas[self.as_key()].$field = val;
            }
        }
    };
}
pub(crate) use def_val_prop;

macro_rules! def_stubbed_prop {
    ($field:ident -> $value_ty:ty) => {
        fn $field(&self) -> $value_ty {
            <$value_ty as Default>::default()
        }

        paste::paste! {
            fn [<set_ $field>](&self, _: $value_ty) {
            }
        }
    };

    ($field:ident: $default:literal -> $value_ty:ty) => {
        fn $field(&self) -> $value_ty {
            $default
        }

        paste::paste! {
            fn [<set_ $field>](&self, _: $value_ty) {
            }
        }
    };
}
pub(crate) use def_stubbed_prop;

macro_rules! bind_prop {
    ($class:ident.$field:ident = $get:path, $set:path) => {
        $class.define_method(stringify!($field), magnus::method!($get, 0))?;
        $class.define_method(concat!(stringify!($field), "="), magnus::method!($set, 1))?;
    };
}
pub(crate) use bind_prop;

macro_rules! def_stubbed_class_prop {
    ($field:ident -> $value_ty:ty) => {
        fn $field() -> $value_ty {
            <$value_ty as Default>::default()
        }

        paste::paste! {
            fn [<set_ $field>](_: $value_ty) {
            }
        }
    };
    ($field:ident: $default:literal -> $value_ty:ty) => {
        fn $field() -> $value_ty {
            $default
        }

        paste::paste! {
            fn [<set_ $field>](_: $value_ty) {
            }
        }
    };
}
pub(crate) use def_stubbed_class_prop;

macro_rules! bind_class_prop {
    ($class:ident.$field:ident = $get:path, $set:path) => {
        $class.define_singleton_method(stringify!($field), magnus::function!($get, 0))?;
        $class.define_singleton_method(
            concat!(stringify!($field), "="),
            magnus::function!($set, 1),
        )?;
    };
}
pub(crate) use bind_class_prop;

macro_rules! bind_module_prop {
    ($class:ident.$field:ident = $get:path, $set:path) => {
        $class.define_module_function(stringify!($field), magnus::function!($get, 0))?;
        $class
            .define_module_function(concat!(stringify!($field), "="), magnus::function!($set, 1))?;
    };
}
pub(crate) use bind_module_prop;
