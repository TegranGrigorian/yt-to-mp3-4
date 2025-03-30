use num_cpus;
pub struct MultiThreadUtils {

}
impl MultiThreadUtils {
    // Function to get the number of available CPU cores
    pub fn get_num_cpus() -> usize {
        // Use the num_cpus crate to get the number of logical CPUs
        num_cpus::get()
    }
    
    // You can add more utility functions related to multithreading here if needed
}