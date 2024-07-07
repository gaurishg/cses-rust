struct SqrtVec {
    n: usize,
    k: usize,
    r: usize,
    c: usize,
    data: Vec<Vec<i32>>,
}

impl SqrtVec {
    pub fn new(n: usize, k: usize) -> Self {
        let mut data = Vec::new();
        let sqrt_n = (n as f64).sqrt() as usize;
        let mut i = 1;
        for x in 0..sqrt_n {
            data.push(vec![]);
            for _ in 0..sqrt_n {
                data[x as usize].push(i);
                i += 1;
            }
        }
        if sqrt_n * sqrt_n != n {
            data.push(vec![]);
            for _ in 0..(n - sqrt_n * sqrt_n) {
                data[sqrt_n as usize].push(i);
                i += 1;
            }
        }
        Self {
            n,
            k,
            r: 0,
            c: 0,
            data,
        }
    }
    
    pub fn pop(&mut self) -> i32 {
        if self.n == 1 {
            self.n -= 1;
            return self.data[self.r][self.c];
        }
        self.skip();
        self.remove()
    }
    
    pub fn pop_all(mut self) -> Vec<i32> {
        let mut v = vec![];
        while self.n > 0 {
            v.push(self.pop());
        }
        v
    }

    fn skip(&mut self) {
        let mut to_skip = self.k % self.n;
        while to_skip != 0 {
            let ele_available_to_skip = self.data[self.r].len() - self.c;
            if ele_available_to_skip > to_skip {
                self.c += to_skip;
                to_skip = 0;
            } else {
                self.r = (self.r + 1) % self.data.len();
                self.c = 0;
                to_skip -= ele_available_to_skip;
            }
        }
    }
    
    fn remove(&mut self) -> i32 {
        let result = self.data[self.r].remove(self.c);
        self.n -= 1;
        self.move_to_valid_position();
        result
    }
    

    fn move_to_valid_position(&mut self) {
        while self.c >= self.data[self.r].len() {
            if self.data[self.r].is_empty() {
                self.data.remove(self.r);
                self.r %= self.data.len();
                self.c = 0;
            } else {
                self.r = (self.r + 1) % self.data.len();
                self.c = 0;
            }
        }
    }
}


pub fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    let mut reader = buffer.split_whitespace().map(|s|s.parse().unwrap());
    let n = reader.next().unwrap();
    let k = reader.next().unwrap();

    let data = SqrtVec::new(n, k);
    for x in data.pop_all() {
        print!("{} ", x);
    }
}

#[cfg(test)]
mod test {
    use super::SqrtVec;
    
    #[test]
    fn test_1() {
        assert_eq!(vec![3, 6, 2, 7, 5, 1, 4], SqrtVec::new(7, 2).pop_all());
    }
}