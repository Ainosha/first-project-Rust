use std::io;
use std::time::{Duration, Instant};
use rand::Rng;

fn swap(a: &mut i8, b: &mut i8){
    let t = *a;
    *a = *b;
    *b = t;
}

fn merge(a:&mut [i8],l:usize,r:usize,n:usize){ //void merge(int a[], int l, int r, int n)
    
    //int *b = (int *)malloc(n * sizeof(int)); /* dynamic memory must be freed */
    // if (b == NULL)
    // {
    //     printf("Can't Malloc! Please try again.");
    //     exit(EXIT_FAILURE);
    // }
    let mut b: Vec<i8> = Vec::with_capacity(n);
    b.resize(n, 0); // Initialize the vector with zeros

    let mut c = l;
    let mut p1 = l;
    let mut p2 = ((l + r) / 2) + 1;
    while p1 < ((l + r) / 2) + 1 && p2 < r + 1
    {
        if a[p1] <= a[p2] //if (a[p1] <= a[p2])
        {
            b[c] = a[p1]; //b[c++] = a[p1];
            p1+=1;
        }
        else
        {
            b[c] = a[p2]; //b[c++] = a[p2];
            p2+=1;
        }
        c+=1; // c++;
    }

    if p2 == r + 1
    {
        while p1 < ((l + r) / 2) + 1
        {
            b[c] = a[p1]; //b[c++] = a[p1];
            p1+=1; // p1++;
            c += 1;
        }
    }
    else
    {
        while p2 < r + 1
        {
            b[c] = a[p2]; //b[c++] = a[p2];
            p2+=1; // p2++;
            c += 1;
        }
    }

    for c in l..r {
        a[c] = b[c];
    }
    //for c = l, c < r + 1, c+1 a[c] = b[c];

    //free(b); //not needed in rust
}
fn merge_sort(a: &mut [i8], n: usize, l: usize, r: usize) {
    if r - l == 1 {
        if a[l] > a[r] {
            //swap(&a[l], &a[r]); Rust's borrow checker does not allow multiple mutable references to the same data at the same time.
            let (left, right) = a.split_at_mut(r);
            swap(&mut left[l], &mut right[0]);
        }
    } else if l != r {
        merge_sort(a, n, l, (l + r) / 2);
        merge_sort(a, n, ((l + r) / 2) + 1, r);
        merge(a, l, r, n);
    }
}

fn main() {
    loop {
        let mut input = String::new();
        println!("Enter Array size: ");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let n: usize = match input.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Ce n'est pas un nombre valide. Essaie encore!");
                continue;
            }
        };
        if n <= 0 {
            println!("Array size must be greater than 0!");
            continue;
        }
        let mut a: Vec<i8> = Vec::with_capacity(n);
        for i in 0..n {
            println!("Enter number[{}]: ", i);
            input.clear();
            io::stdin()
                .read_line(&mut input).expect("Failed to read line");
            let num: i8 = input.trim().parse()
                .expect("Please enter a valid number");
            a.push(num);
        }
        let start = Instant::now();
        merge_sort(&mut a, n, 0, n - 1);
        let duration = start.elapsed();

        println!("Sorted Array: {:?}", a);
        println!("Time taken to sort: {:?}", duration);
        return;
    }
}

fn test_with_100_values() {
    let mut rng = rand::thread_rng();
    let mut total_duration = Duration::new(0, 0);

    for _ in 0..100 {
        let mut a: Vec<i8> = (0..100).map(|_| rng.gen_range(0..100)).collect();
        let start = Instant::now();
        let len = a.len();
        merge_sort(&mut a, len, 0, len - 1);
        let duration = start.elapsed();
        total_duration += duration;
    }

    let average_duration = total_duration / 100;
    println!("Average time taken to sort 100 arrays: {:?}", average_duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorting() {
        test_with_100_values();
    }
}