use std::cmp;

fn main() {
    let initial = [66, 70, 52, 93, 44, 67, 47, 10, 11, 13, 94, 99, 51, 12];
    let mut to_sort;
    println!("initial:          {:?}",initial);

    to_sort  = initial.clone();
    bubble_sort(&mut to_sort);
    println!("bubble-sorted:    {:?}",to_sort);

    to_sort  = initial.clone();
    sel_sort(&mut to_sort);
    println!("selection-sorted: {:?}",to_sort);

    to_sort  = initial.clone();
    insert_sort(&mut to_sort);
    println!("insertion-sorted: {:?}",to_sort);

    println!();
    println!("unordered search:");
    report_search(44,unordered_search(44,&initial[..]));
    report_search(43,unordered_search(43,&initial[..]));

    println!();
    println!("binary search:");
    report_search(44,binary_search(44,&to_sort[..]));
    report_search(43,binary_search(43,&to_sort[..]));

    println!();
    println!("the min and max of initial are {:?}",
             min_max(&initial[..]));
}

/*
// NOTE!! The following will not compile: It needs lifetime annotations.
// We'll fix this later on.
fn swap(x :&mut u32, y : &mut u32) {
    let t = x;
    x = y;
    y = t;
}
*/

fn bubble_sort(a : &mut [u32]) {
    let len = a.len();
    for i in 0..len {
        for j in 0..(len-i-1) {
            if a[j]>a[j+1] {
                // swap the values of a[j] and a[j+1]
                let t = a[j];
                a[j] = a[j+1];
                a[j+1] = t;
            }
        }
    }
}

fn report_search(x : u32, r : Option<usize>) {
    print!("\t {} ",x);
    match r {
        None    => { println!("not found"); },
        Some(i) => { println!("found at index {}",i); },
    }
}

fn unordered_search(x : u32, a : &[u32]) -> Option<usize> {
    for i in 0..a.len() {
        if x==a[i] { return Some(i); }
    }
    None
}


fn sel_sort(a : &mut [u32]) { //check 
   let len = a.len();
  
    for i in 0..(len-1){
       let mut  min = i;
        for j in (i+1)..len{
            if a[j] < a[min]
            {
                min=j;
            } 
        }
        if min != i
           {
            let t = a[i];
            a[i] = a[min];
            a[min] = t;   
            }
        }
}

fn insert_sort(a : &mut [u32]) {
    let mut i=1;
    let len = a.len();
    while i < len
    {   
      let mut j = i;
      while  j > 0  && a[j-1]> a[j]
      {
          let x = a[j];
          a[j]=a[j-1];
          a[j-1]=x;
          j = j-1;
      }
        i = i+1;
    }
}

fn binary_search(x : u32, a : &[u32]) -> Option<usize> {
    let mut low = 0;
    let mut high = a.len()-1;
    

    while low<=high 
    {
        let mid = (low+high)/2;
        if a[mid] < x
        {
            low=mid+1;
        }
        else if a[mid] > x
        {
            high=mid-1;
        }
        else
        {
            return Some(mid); 
        }
    }
    return None;


}

fn min_max(a : &[u32]) -> (u32,u32) {
    let len = a.len();
    assert!(len>0);
   
    let mut min = cmp::min(a[0], a[1]);
    let mut max = cmp::max(a[0], a[1]);

    for i in 1..len-1
    {
        min = cmp::min(min, a[i]);
        max = cmp::max(max, a[i]);

    }

    return (min,max);
}
    
// NOTE:
// cmp::min(a,b) returns the minimum of a and b
// cmp::max(a,b) returns the maximum of a and b

