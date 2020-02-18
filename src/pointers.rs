/**
 * pointers point to a resource in memory
*/
pub fn run() {
    // primitive array
    let arr1 = [1,2,3];
    let arr2 = arr1;

    // with non primitives if you assign another variable to a piece of data, the first variable will no longer hold that value. you'll need to use a reference (&) to point to the resource.
    let vec1 = vec![1,2,3];
    let vec2 = &vec1; // without an '&' vec1 throws an error in this situation

    println!("values: {:?}", (arr1, arr2));
    println!("values: {:?}", (&vec1, vec2));


}