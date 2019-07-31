struct BitCredit {
    btcnumber: u32,
}

impl CreditCharge for BitCredit {
    fn charge_with_id(&self, id: u32) -> bool {
        id % 2 == self.btcnumber % 2
    }
}

trait CreditCharge {
    fn charge_with_id(&self, id: u32) -> bool;
}

pub fn run_examples() {
    let card = BitCredit { btcnumber: 1024 };
    let code = 4096;

    if card.charge_with_id(code) {
        println!("Success!!");
    } else {
        println!("Failure!!");
    }
}
