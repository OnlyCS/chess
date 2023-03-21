pub trait ToVec<T> {
    fn to_vec(&self) -> Vec<&T>;
    fn to_vec_mut(&mut self) -> Vec<&mut T>;
}
