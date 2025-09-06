use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Debug)]   // <-- traits derived
enum Fruit {
    Apple,
    Banana,
    Mango,
    Lychee,
    Pineapple,
}

fn fruit_basket(basket: &mut HashMap<Fruit, u32>) {
    let fruit_kinds = [
        Fruit::Apple,
        Fruit::Banana,
        Fruit::Mango,
        Fruit::Lychee,
        Fruit::Pineapple,
    ];

    

    for fruit in fruit_kinds {

        // TODO: Insert new fruits if they are not already present in the
        // basket. Note that you are not allowed to put any type of fruit that's
        // already present!


         
     match basket.get(&fruit){
        Some(_) =>{},
        None => {
            basket.insert(fruit,2);

        }
        


     };
    }
}

fn main() {
    // You can optionally experiment here.

    let mut basket = HashMap::new();
    basket.insert(Fruit:: Apple ,4);
    basket.insert(Fruit:: Mango , 2);
    basket.insert(Fruit:: Lychee , 5);
    fruit_basket(&mut basket);
    println!("{:?}",basket);

 let iter = basket.values();
 let sum :u32 = iter.sum();
 println!("No of fruits in basket after all the operations are {}",sum);
    
}
