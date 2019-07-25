//! A custom storage type that wraps another storage with `Arc`

use specs::storage::{UnprotectedStorage, TryDefault};
use specs::world::Index;
use hibitset;

use std::marker::PhantomData;
use std::sync::Arc;

pub struct SharedStorage<T, S>(Arc<S>, PhantomData<T>);

impl<T, S: UnprotectedStorage<T>> TryDefault for SharedStorage<T, S> {
    fn try_default() -> Result<Self, String> {
        Ok(SharedStorage(Arc::new(TryDefault::try_default()?), Default::default()))
    }
}

impl<T, S: UnprotectedStorage<T>> UnprotectedStorage<T> for SharedStorage<T, S> {
    unsafe fn clean<B>(&mut self, has: B)
    where
        B: hibitset::BitSetLike
    {
        self.0.write().unwrap().clean(has);
    }

    unsafe fn get(&self, id: Index) -> &T {
        self.0.get(id)
    }

    unsafe fn get_mut(&mut self, id: Index) -> &mut T {
        unimplemented!()
    }

    unsafe fn insert(&mut self, id: Index, value: T) {
        self.0.write().unwrap().insert(id, value)
    }

    unsafe fn remove(&mut self, id: Index) -> T {
        self.0.write().unwrap().remove(id)
    }
}
