use num_cpus;
pub struct MultiThreadUtils {

}
impl MultiThreadUtils {
    pub fn get_num_cpus() -> usize {
        num_cpus::get()
    }
}