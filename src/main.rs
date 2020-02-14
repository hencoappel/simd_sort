fn simd_sort<T: Ord + Copy>(slice: &mut [T]) {
    unsafe {
        simd_sort_impl(slice);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn simd_sort_impl<T: Ord + Copy>(slice: &mut [T]) {
    if slice.len() <= 1 {
        return;
    }
    let mut left: usize = 0;
    let mut right: usize = slice.len() - 1;
    loop {
        let mut i = left;
        let mut j = right;

        // pre-sort the low, middle (pivot), and high values in place.
        // this improves performance in the face of already sorted data, or
        // data that is made up of multiple sorted runs appended together.
        let middle = i + ((j - i) >> 1); // (i+j)/2 == i+(j-i)/2
        swap_if_greater_with_items(slice, i, middle); // swap the low with the mid point
        swap_if_greater_with_items(slice, i, j); // swap the low with the high
        swap_if_greater_with_items(slice, middle, j); // swap the middle with the high

        let x = slice[middle];
        loop {
            while slice[i] < x {
                i += 1;
            }
            while x < slice[j] {
                j -= 1;
            }
            if i > j {
                break;
            }
            if i < j {
                let key = slice[i];
                slice[i] = slice[j];
                slice[j] = key;
            }
            i += 1;
            j -= 1;
            if i > j {
                break;
            }
        }
        println!("{:?}, {:?}", j, left);
        if j - left <= right - i {
            if left < j {
                simd_sort_impl(&mut slice[left..j]);
            }
            left = i;
        } else {
            if i < right {
                simd_sort_impl(&mut slice[i..right]);
            }
            right = j;
        }
        if left >= right {
            break;
        }
    }
}

fn swap_if_greater_with_items<T: Ord + Copy>(slice: &mut [T], a: usize, b: usize) {
    if a != b {
        if slice[a] > slice[b] {
            let val = slice[a];
            slice[a] = slice[b];
            slice[b] = val;
        }
    }
}

fn main() {
    println!("Hello, world!");
    let mut arr = vec![2, 5, 1, 6];
    simd_sort(&mut arr);
    println!("{:?}", arr);
}
