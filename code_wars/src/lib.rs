// It should remove all values from list a, which are present in list b keeping their order.
pub fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    // a.into_iter()
    //     .filter(|item| {
    //     !b.contains(item)
    // })
    //     .collect()

    let mut a = a;
    a.retain(|x| !b.contains(x));

    a
}