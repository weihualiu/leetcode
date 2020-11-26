struct OrderedStream {
    vec: Vec<String>,
    idx : usize
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl OrderedStream {

    fn new(n: i32) -> Self {
        let mut v = Vec::new();
        for i in 0..n {
            v.push(String::from(""));
        }
        OrderedStream{
            vec: v,
            idx: 0
        }
    }

    fn insert(&mut self, id: i32, value: String) -> Vec<String> {
        self.vec[id as usize -1] = value;
        let mut v: Vec<String> = Vec::new();
        let mut temp = 0;
        for i in self.idx..self.vec.len() {
            let val = self.vec.get(i).unwrap();
            if val.eq("") {
                break;
            }else{
                v.push(val.clone());
                temp+=1;
            }
        }
        self.idx+=temp;

        v
    }
}

/**
 * Your OrderedStream object will be instantiated and called as such:
 * let obj = OrderedStream::new(n);
 * let ret_1: Vec<String> = obj.insert(id, value);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test() {
        let mut obj = OrderedStream::new(5);
        let ret_1: Vec<String> = obj.insert(3, "ccccc".to_string());
        println!("{:?}", ret_1);
        let ret_1: Vec<String> = obj.insert(1, "aaaaa".to_string());
        println!("{:?}", ret_1);
        let ret_1: Vec<String> = obj.insert(2, "bbbbb".to_string());
        println!("{:?}", ret_1);
        let ret_1: Vec<String> = obj.insert(5, "eeeee".to_string());
        println!("{:?}", ret_1);
        let ret_1: Vec<String> = obj.insert(4, "ddddd".to_string());
        println!("{:?}", ret_1);
    }
}

/*
创建一个Vector，并初始化为n个空字符串。
每次插入新值后，从偏移指针idx开始遍历，遇到空字符串时中断，将遍历的值插入一个新vector中返回
遍历的次数累加到idx上，用于偏移。

 */

/*
// 20ms耗时的实现。区别在于我使用了vector来存储值，他使用来HashMap来存储。
use std::collections::HashMap;
struct OrderedStream {
    m: HashMap<i32,String>,
    n: i32,
    next: i32,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl OrderedStream {


    fn new(n: i32) -> Self {
        Self {
            n:n,
            next:1,
            m: HashMap::default()
        }
    }

    fn insert(&mut self, id: i32, value: String) -> Vec<String> {
        self.m.insert(id, value);
        let mut ans = Vec::new();
        let begin = self.next;
        for i in begin..=self.n {
            if self.m.contains_key(&i) {
                ans.push(self.m.remove(&i).unwrap());
                self.next = i+1;
            } else {
                break;
            }
        }
        ans
    }
}
 */