#![feature(generic_const_exprs)]

pub mod blueprint;

#[macro_export]
macro_rules! create_spaceship {
    (
        core {
            $( $core_key:ident $core_type:path ),* $(,)?
        }
        $(optional {
            $( $opt_key:ident $opt_type:path ),* $(,)?
        })?
    ) => {
        create_spaceship!(
            @core ( $crate::blueprint::Blueprint::new().set_frame() )
            $( $core_key $core_type, )*
            @@
            $( $( $opt_key $opt_type, )* )?
        )
    };

    (@core ($expr:expr) reactor $t:path, $($tail:tt)*) => {
        create_spaceship!(@core ($expr.add_reactor(<$t>::new())) $($tail)*)
    };

    (@core ($expr:expr) engine $t:path, $($tail:tt)*) => {
        create_spaceship!(@core ($expr.add_engine(<$t>::new())) $($tail)*)
    };

    (@core ($expr:expr) life_support $t:path, $($tail:tt)*) => {
        create_spaceship!(@core ($expr.add_life_support(<$t>::new())) $($tail)*)
    };

    (@core ($expr:expr) bridge $t:path, $($tail:tt)*) => {
        create_spaceship!(@core ($expr.add_bridge(<$t>::new())) $($tail)*)
    };

    (@core ($expr:expr) @@ $($tail:tt)*) => {
        create_spaceship!(@optional ($expr.lock_core_modules()) $($tail)*)
    };

    (@optional ($expr:expr) shield $t:path, $($tail:tt)*) => {
        create_spaceship!(@optional ($expr.add_shield(<$t>::new())) $($tail)*)
    };

    (@optional ($expr:expr) sensors $t:path, $($tail:tt)*) => {
        create_spaceship!(@optional ($expr.add_sensors(<$t>::new())) $($tail)*)
    };

    (@optional ($expr:expr)) => {
        $expr.finalize() as $crate::blueprint::FinalizedBlueprint<_, _, _>
    };
}
