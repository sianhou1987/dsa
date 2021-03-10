use Vector::RawVec;

#[test]
fn allocate_in() {
    let raw_vec = RawVec::<i32>::with_capacity_in(2);
    unsafe {
        *(raw_vec.ptr().add(0)) = 0;
        *(raw_vec.ptr().add(1)) = 1;
        let a = core::slice::from_raw_parts_mut(raw_vec.ptr(), 2);
        assert!(a == [0, 1]);
    }

    let raw_vec = RawVec::<i32>::with_capacity_zeroed_in(2);
    unsafe {
        let a = core::slice::from_raw_parts_mut(raw_vec.ptr(), 2);
        assert!(a == [0, 0]);
    }
}
