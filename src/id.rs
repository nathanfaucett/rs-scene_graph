use core::intrinsics;
use core::marker::Reflect;
use core::any::TypeId;
use core::cmp::Ordering;


#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Id {
    t: u64,
    type_id: TypeId,
}

impl Id {

    pub fn of<T: ?Sized + Reflect + 'static>() -> Id {
        Id {
            t: unsafe { intrinsics::type_id::<T>() },
            type_id: TypeId::of::<T>(),
        }
    }

    pub fn get_id(&self) -> u64 {
        self.t
    }
    pub fn get_type_id(&self) -> TypeId {
        self.type_id
    }
}

impl PartialOrd for Id {
    fn partial_cmp(&self, other: &Id) -> Option<Ordering> {
        self.t.partial_cmp(&other.t)
    }
    fn lt(&self, other: &Id) -> bool {
        self.t.lt(&other.t)
    }
    fn le(&self, other: &Id) -> bool {
        self.t.le(&other.t)
    }
    fn gt(&self, other: &Id) -> bool {
        self.t.gt(&other.t)
    }
    fn ge(&self, other: &Id) -> bool {
        self.t.ge(&other.t)
    }
}

impl Ord for Id {
    fn cmp(&self, other: &Self) -> Ordering {
        self.t.cmp(&other.t)
    }
}
