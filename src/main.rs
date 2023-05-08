use std::collections::HashMap;

fn main() {
    
    //into_iter provides an owned iterator:
    //the collection is moved and you can't use the original variable
    let sum: u32 = vec![1, 2, 3].into_iter().map(|x| x * x).sum();
    println!("The result of (1*1)+(2*2)+(3*3) = {}", sum);

    fn vector() {
        let v = vec![1, 2, 3];

        for x in v.into_iter() {
            println!("{}", x);
        }
    }
    //into_iter moved v's values and v can no longer be used
    vector();

    //iter provides borrowed iterator
    fn hashmap() {
        let mut h = HashMap::new();
        h.insert("Hello".to_string(), "World".to_string());

        //iterator is consumed with for x in
        for (key, value) in h.iter() {
            println!("{}: {}", key, value);
        }
    }

    hashmap();

    //using for_each() instead of for x in
    fn for_each() {
        let v = vec!["Hello", "World", "!"].into_iter();

        v.for_each(|word| {
            println!("{}", word);
        });
    }

    for_each();

    fn collect() {
        let x = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10].into_iter();
        
        let y: Vec<u64> = x.clone().collect();

        println!("{:?} {:?}", x, y);
    }
    
    collect();

    fn filter() {
        let v = vec![-1, 0, 1, 2, 3, -5, -10].into_iter();

        let positive_num: Vec<i32> = v.filter(|x: &i32| x.is_positive()).collect();

        println!("{:?}", positive_num);
    }

    filter();
}
