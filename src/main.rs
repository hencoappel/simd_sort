pub fn simd_sort(slice: &mut [i64]) {
    unsafe {
        simd_sort_impl(slice, 0, slice.len() - 1);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn simd_sort_impl(slice: &mut [i64], left: usize, right: usize) {
    if left == right {
        return;
    }
    let pivot = pick_pivot(slice, left, right);
    let pivot_pos = partition(slice, pivot, left, right);

    // println!("({}, {}, {})", left, pivot_pos, right);
    simd_sort_impl(slice, left, pivot_pos);
    simd_sort_impl(slice, pivot_pos + 1, right);
}

pub fn quick_sort(slice: &mut [i64]) {
    quick_sort_impl(slice, 0, slice.len() - 1);
}

fn quick_sort_impl(slice: &mut [i64], left: usize, right: usize) {
    if left == right {
        return;
    }
    let pivot = pick_pivot(slice, left, right);
    let pivot_pos = partition(slice, pivot, left, right);

    // println!("({}, {}, {})", left, pivot_pos, right);
    quick_sort_impl(slice, left, pivot_pos);
    quick_sort_impl(slice, pivot_pos + 1, right);
}

fn pick_pivot(slice: &mut [i64], left: usize, right: usize) -> i64 {
    // pre-sort the low, middle (pivot), and high values in place.
    // this improves performance in the face of already sorted data, or
    // data that is made up of multiple sorted runs appended together.
    let middle = left + ((right - left) >> 1); // (i+j)/2 == i+(j-i)/2
    swap_if_greater_with_items(slice, left, middle); // swap the low with the mid point
    swap_if_greater_with_items(slice, left, right); // swap the low with the high
    swap_if_greater_with_items(slice, middle, right); // swap the middle with the high
    slice[middle]
}

fn partition(slice: &mut [i64], pivot: i64, mut left: usize, mut right: usize) -> usize {
    // println!("Partition {:?}", slice);
    loop {
        while slice[left] < pivot {
            left += 1;
        }
        while slice[right] > pivot {
            right -= 1;
        }

        if left >= right {
            break;
        }
        swap(slice, left, right);
    }
    right
}

fn swap(slice: &mut [i64], a: usize, b: usize) {
    let val = slice[a];
    slice[a] = slice[b];
    slice[b] = val;
}

fn swap_if_greater_with_items(slice: &mut [i64], a: usize, b: usize) {
    if a != b {
        if slice[a] > slice[b] {
            swap(slice, a, b);
        }
    }
}

fn main() {
    // println!("Hello, world!");
    let mut arr = vec![2, 5, 12, 33, 1, 6, 10, 3, 4];
    println!("{:?}", arr);
    quick_sort(&mut arr);
    println!("{:?}", arr);
}
