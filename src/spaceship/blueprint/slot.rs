pub const TOTAL_SLOTS: usize = 10;

pub struct SlotCheck<const N: usize, const C: usize>;

struct Check<const B: bool>;
trait IsTrue {}
impl IsTrue for Check<true> {}

pub trait SlotIsAvailable {}
impl<const N: usize, const C: usize> SlotIsAvailable for SlotCheck<N, C> where
    Check<{ N + C <= TOTAL_SLOTS }>: IsTrue
{
}
