use packed_simd::*;

type i32_vec = i32x8;
const vec_size = 8;


pub fn simd_sort(slice: &mut [i32]) {
    unsafe {
        simd_sort_impl(slice, 0, slice.len() - 1);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn simd_sort_impl(slice: &mut [i32], left: usize, right: usize) {
    if left == right {
        return;
    }
    let pivot = pick_pivot(slice, left, right);
    let pivot_vec = i32_vec::splat(pivot);
    let pivot_pos = simd_partition(slice, pivot_vec, left, right);

    // println!("({}, {}, {})", left, pivot_pos, right);
    simd_sort_impl(slice, left, pivot_pos);
    simd_sort_impl(slice, pivot_pos + 1, right);
}

unsafe fn simd_partition(slice: &mut [i32],  pivot: i32_vec,
                         write_left: usize, write_right:usize ) {
    let data = i32_vec::from_slice_unaligned(&slice[write_left..(write_left+vec_size)])
    var data = Avx2.LoadDquVector256(slice);
    var mask = (uint) Avx.MoveMask(
        Avx2.CompareGreaterThan(data, P).AsSingle());
    data = Avx2.PermuteVar8x32(data,
                               Avx2.LoadDquVector256(PermTablePtr + mask * 8)));
    Avx.Store(write_left,  data);
    Avx.Store(write_right, data);
    var popCount = PopCnt.PopCount(mask);
    write_right -= pc;
    write_left  += 8 - pc;
}

pub fn quick_sort(slice: &mut [i32]) {
    quick_sort_impl(slice, 0, slice.len() - 1);
}

fn quick_sort_impl(slice: &mut [i32], left: usize, right: usize) {
    if left == right {
        return;
    }
    let pivot = pick_pivot(slice, left, right);
    let pivot_pos = partition(slice, pivot, left, right);

    // println!("({}, {}, {})", left, pivot_pos, right);
    quick_sort_impl(slice, left, pivot_pos);
    quick_sort_impl(slice, pivot_pos + 1, right);
}

fn pick_pivot(slice: &mut [i32], left: usize, right: usize) -> i32 {
    // pre-sort the low, middle (pivot), and high values in place.
    // this improves performance in the face of already sorted data, or
    // data that is made up of multiple sorted runs appended together.
    let middle = left + ((right - left) >> 1); // (i+j)/2 == i+(j-i)/2
    swap_if_greater_with_items(slice, left, middle); // swap the low with the mid point
    swap_if_greater_with_items(slice, left, right); // swap the low with the high
    swap_if_greater_with_items(slice, middle, right); // swap the middle with the high
    slice[middle]
}

fn partition(slice: &mut [i32], pivot: i32, mut left: usize, mut right: usize) -> usize {
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

fn swap(slice: &mut [i32], a: usize, b: usize) {
    let val = slice[a];
    slice[a] = slice[b];
    slice[b] = val;
}

fn swap_if_greater_with_items(slice: &mut [i32], a: usize, b: usize) {
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
