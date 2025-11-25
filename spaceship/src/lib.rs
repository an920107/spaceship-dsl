#![feature(generic_const_exprs)]

pub mod blueprint;

#[macro_export]
macro_rules! create_spaceship {
    (
        core {
            $( $core_ops:ident ( $($core_args:expr),* ) );* $(;)?
        }
        optional {
            $( $opt_ops:ident ( $($opt_args:expr),* ) );* $(;)?
        }
    ) => {
        $crate::blueprint::Blueprint::new()
            .set_frame()
            $(
                .$core_ops( $($core_args),* )
            )*
            .lock_core_modules()
            $(
                .$opt_ops( $($opt_args),* )
            )*
            .finalize() as $crate::blueprint::FinalizedBlueprint<_, _, _>
    };
}
