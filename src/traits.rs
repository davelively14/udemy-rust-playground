struct Visa {
    verification: u32,
}

impl CreditCharge for Visa {
    fn charge_with_id(&self, id: u32) -> bool {
        id % 3 == self.verification % 4
    }
}

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

fn transact<Q: CreditCharge>(card: Q) {
    // get verif code
    let id = 4096;
    if card.charge_with_id(id) {
        println!("Success!!");
    } else {
        panic!("Invalid code!");
    }
}

pub fn run_examples() {
    let bit_credit = BitCredit { btcnumber: 1024 };
    let visa = Visa { verification: 1024 };

    transact(bit_credit);
    transact(visa);
}
