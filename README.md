# Decentralized Library Book Loan 

This is a smart contract built on the Stellar network using Soroban/Stellar CLI. It allows users to manage a simple library book loan system directly on the blockchain.

## Features (CRUD)
- **Get Loans**: View all borrowed books.
- **Borrow Book**: Record a new book loan.
- **Update Loan**: Update the title of the borrowed book.
- **Return Book**: Delete the loan record once the book is returned.

## Smart Contract Testnet ID
**Contract ID:** `CBIMSH26JAFLKATBFOOPNVSKEM7WHZJMLTM6HWQTGB6YZG4AE47MANZN`

## How to Interact (via CLI)
You can use the following command to borrow a book:
```bash
stellar contract invoke --id CBIMSH26JAFLKATBFOOPNVSKEM7WHZJMLTM6HWQTGB6YZG4AE47MANZN --source perpus --network testnet -- borrow_book --borrower_name "Your Name" --book_title "Book Title"