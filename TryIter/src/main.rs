fn main() {
    println!("Hello, world!");
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();

    for item in v1_iter {
        println!("{:?}", item);
    }

    let v1:Vec<i32> = vec![1,2,3];
    let v2:Vec<i32> = v1.iter().map(|x| x+1).collect();
    println!("{:?}",v2);

    let ctr = Counter::new().filter(|x| x%3 ==0);
    for i in ctr{
        println!("{}",i);
    }
}

struct Counter{
    count:u32,
}

impl Counter{
    pub fn new()->Counter{
        Counter{count:0}
    }
}

impl Iterator for Counter{
    type Item = u32;
    fn next(&mut self) ->Option<Self::Item>{
        self.count+=1;
        if self.count <6 {
            Some(self.count)
        }
        else    {
            None
        }
    }
}

