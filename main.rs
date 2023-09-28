fn cocktail_shaker_sort(arr: &mut [i32]) {
    let length = arr.len();
    let mut swapped;

    loop {
        swapped = false;

        for i in 0..length - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }

        swapped = false;

        for i in (0..length - 2).rev() {
            if arr[i] > arr[i + 1] {
                
                arr.swap(i, i + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
}

fn main() {
    let _arr = vec![64, 34, 25, 12, 22, 11, 90,1];
    let mut array = vec![64, 34, 25, 12, 22, 11, 90,1];
    cocktail_shaker_sort(&mut array);
    println!("Sorted array: {:?}", array);
}

