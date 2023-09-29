fn cocktail_shaker_sort(arr: &mut [i32]) {
    let length = arr.len();
    let mut swapped;
    let mut start = 0;
    let mut end = length - 1;

    loop {
        swapped = false;

        // Forward pass
        for i in start..end {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }

        end -= 1;

        if !swapped || start > end {
            break;
        }

        swapped = false;

        // Backward pass
        for i in (start..end).rev() {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }
        start += 1;
        if !swapped || start > end {
            break;
        }
    }
}

fn main() {
    let mut array = vec![64, 34, 25, 12, 22, 11, 90,1];
    cocktail_shaker_sort(&mut array);
    println!("Sorted array: {:?}", array);
}

