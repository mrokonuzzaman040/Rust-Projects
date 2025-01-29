## **Exercise: Build a Simple Bank System**

### **Objective:**

Create a Rust program that simulates a basic banking system with the following functionality:

- A `BankAccount` struct that holds:
  - `owner` (String)
  - `balance` (f64)
- Methods to:
  - **Deposit money** into the account.
  - **Withdraw money** from the account (ensure insufficient balance is handled).
  - **Display account details**.

---

### **Requirements**

1. Define a **struct** named `BankAccount`.
2. Implement the following **methods** using `impl`:
   - `new(owner: String, initial_balance: f64) -> BankAccount`
   - `deposit(&mut self, amount: f64)`
   - `withdraw(&mut self, amount: f64) -> Result<f64, String>`
   - `display(&self)`
3. Use **ownership and borrowing properly**:
   - `&mut self` where needed for modifying balance.
   - Return **Result<T, String>** for handling errors.

---

### **Expected Output Example**

```rust
Account Owner: Alice
Balance: $1000.0

Depositing $500...
New Balance: $1500.0

Withdrawing $200...
New Balance: $1300.0

Withdrawing $2000...
Error: Insufficient funds!
```

---

### **Hints**

1. Use `struct` for the `BankAccount`.
2. Use `self.balance += amount;` for deposits.
3. Use an `if` condition inside `withdraw()`:
   - If balance is sufficient, subtract and return `Ok(new_balance)`.
   - If insufficient, return `Err("Insufficient funds!".to_string())`.
4. Call these methods in `main()` and test with different values.

---

### **Bonus Challenge (Optional)**

- Add a **`transfer` method** that allows transferring money between two accounts.
- Add **user input** using `std::io::stdin()` to interact with the bank.
