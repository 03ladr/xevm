use std::time::Instant;


macro_rules! arith_instructor {
    ( $machine:expr, $op:tt, $inc:expr ) => {
        let now = Instant::now();
        let val2 = ($machine).stack.pop()?;
        let val1 = ($machine).stack.pop()?;
        let evaluation = val1 $op val2;
        $machine.stack.push(evaluation)?;
        $machine.pc_increment($inc);
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
    }
}

macro_rules! bool_arith_instructor {
    ( $machine:expr, $op:tt, $inc:expr ) => {
        let mut result: u8 = 0;
        let val2 = $machine.stack.pop()?;
        let val1 = $machine.stack.pop()?;
        let evaluation = val1 $op val2;
        if evaluation == true {
            result = 1;
        };
        $machine.stack.push(U256::from(result))?;
        $machine.pc_increment($inc);
    }
}

macro_rules! polynomial_arith_instructor {
    ( $machine:expr, $op1:tt, $op2: tt, $inc: expr ) => {
        let val3 = $machine.stack.pop()?;
        let val2 = $machine.stack.pop()?;
        let val1 = $machine.stack.pop()?;
        let evaluation = (val1 $op1 val2) $op2 val3;
        $machine.stack.push(evaluation)?;
        $machine.pc_increment($inc);
    }
}

macro_rules! dupn {
    ( $machine:expr, $idx:expr ) => {
        let val = $machine.stack.storage[$machine.stack.storage.len() - $idx];
        $machine.stack.push(val)?;
        $machine.pc_increment(1);
    }
}
