pub fn plus_array(mut list_1: Vec<i32>, b: i32) -> Vec<i32> {
    let len_list = list_1.len();
    for i in 0..len_list {
        list_1[i] += b;
    }
    list_1
}