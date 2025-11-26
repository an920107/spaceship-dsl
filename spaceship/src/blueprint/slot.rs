pub const TOTAL_SLOTS: usize = 10;

#[diagnostic::on_unimplemented(message = "cannot add component: slot limit exceeded")]
trait IsTrue {}

struct Check<const B: bool>;
impl IsTrue for Check<true> {}

pub trait SlotIsAvailable<const N: usize, const C: usize> {}
impl<const N: usize, const C: usize> SlotIsAvailable<N, C> for ()
where
    Check<{ N + C <= TOTAL_SLOTS }>: IsTrue,
{
}
