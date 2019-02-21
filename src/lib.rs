mod dep;

#[no_mangle]
pub unsafe extern "C" fn get_x(dep_struct: *const dep::otherdep::dep_struct) -> u32 {
    dep_struct.read().x
}
