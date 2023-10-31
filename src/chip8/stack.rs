pub struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
 
    
    pub fn new() -> Self {
        Stack { stack: Vec::new() }
    }


    pub fn push(&mut self, value: T) -> Result<(), &'static str> {
        if self.stack.len() >= 16 {
            return Err("Error: Stack maximum size exceeded");
        }
        self.stack.push(value);
        Ok(())
    }

    
    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }
}