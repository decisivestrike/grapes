pub mod concurrent;
pub mod local;

pub trait State<T> {
    type Value<'a>
    where
        Self: 'a;

    fn get(&self) -> Self::Value<'_>;

    fn get_untracked(&self) -> Self::Value<'_>;

    fn set(&self, value: T);

    fn update<U>(&self, updater: U)
    where
        U: FnOnce(&mut T);
}
