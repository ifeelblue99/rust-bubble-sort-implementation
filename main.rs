fn main() {
  let mut data = [88usize, 55, 11, 66, 99, 0, 77];
  println!("sorted arr. {:?}",bubble_sort(&mut data));
}
fn bubble_sort(arr: &mut [usize]) ->  &mut [usize] {

  let mut last_point = arr.len()-1;

  while last_point > 1 {

    for indx in 0..last_point {
      if arr[indx] > arr[indx+1] {
        let temp = arr[indx+1];
        arr[indx+1] = arr[indx];
        arr[indx] = temp;
      }
    }
    last_point = last_point - 1 ;
  }
  arr
}
