### **Exercise: Library Management System (Structs & Vectors in Rust)**
---
### **Objective:**
Build a **Library Management System** where users can:
- **Add books** to a library.
- **Borrow books** (only if available).
- **Return books** to the library.
- **Display all available books.**

---

### **Requirements**
1. Define a **`Book` struct** with:
   - `title: String`
   - `author: String`
   - `is_borrowed: bool`

2. Define a **`Library` struct** with:
   - `books: Vec<Book>`

3. Implement methods for `Library`:
   - `add_book(&mut self, title: String, author: String)`
   - `borrow_book(&mut self, title: &str) -> Result<(), String>`
   - `return_book(&mut self, title: &str) -> Result<(), String>`
   - `display_books(&self)`

4. Use **ownership and borrowing properly**:
   - `&mut self` for modifying books.
   - Use **`Result<(), String>`** for error handling.

---

### **Expected Output**
```rust
Available Books:
1. The Rust Programming Language by Steve Klabnik
2. Clean Code by Robert C. Martin

Borrowing "The Rust Programming Language"...
Book borrowed successfully!

Available Books:
1. Clean Code by Robert C. Martin

Returning "The Rust Programming Language"...
Book returned successfully!

Available Books:
1. The Rust Programming Language by Steve Klabnik
2. Clean Code by Robert C. Martin
```

---

### **Hints**
1. Store books in a `Vec<Book>`.
2. When borrowing a book, **search by title**, and if `is_borrowed == false`, mark it as `true`.
3. When returning a book, mark `is_borrowed` as `false`.

---

### **Bonus Challenge (Optional)**
- Add a **search feature** for books.
- Allow **removing books** from the library.
