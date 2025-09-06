fn main(){
    let v = vec![1,3,21,8];
    
    let mut v2 = Vec::new();
    
    for val in v {
        if val%2==0 {
            v2.push(val);
        }
    }
    
    let  iter = v2.iter_mut();
    for val in iter {
       *val = *val * 2;
    }
    
    println!("{:?}",v2);
}