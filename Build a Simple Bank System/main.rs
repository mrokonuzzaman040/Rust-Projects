struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn new(owner: String, balance: f64) -> BankAccount {
        BankAccount { owner, balance }
    }

    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        println!(
            "Deposited: ${}, and current account balance ${}",
            amount, self.balance
        );
    }

    fn withdraw(&mut self, amount: f64) {
        if self.balance >= amount {
            self.balance -= amount;
            println!(
                "Withdrawn: ${}, and current account balance ${}",
                amount, self.balance
            );
        } else {
            println!("Insufficient funds");
        }
    }

    fn display(&self) {
        print!("Account owner: {}, ", self.owner);
        println!("Account balance: ${}", self.balance);
    }
}

fn main() {
    let mut account = BankAccount::new("Your Name".to_string(), 100.0);
    account.deposit(50.0);
    account.withdraw(10.0);
    account.display();
}
