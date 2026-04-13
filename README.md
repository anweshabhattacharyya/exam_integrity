# 🧠 Exam Integrity Smart Contract (Soroban)
![Screenshot 2026-04-13 at 1 38 47 PM](https://github.com/user-attachments/assets/b03af40a-5587-4025-80b5-f7b014b03a96)


![Stellar](https://img.shields.io/badge/Blockchain-Stellar-blue)
![Soroban](https://img.shields.io/badge/Smart%20Contracts-Soroban-green)
![Rust](https://img.shields.io/badge/Language-Rust-orange)
![Status](https://img.shields.io/badge/Status-Active-success)

---

## 📌 Project Description

**Exam Integrity** is a decentralized application built on the Stellar blockchain using Soroban smart contracts.
It aims to eliminate cheating, data tampering, and unfair practices in digital examinations by leveraging blockchain immutability.

By storing exam data and submissions securely on-chain, the system ensures complete transparency, trust, and verifiability for both educators and students.

---

## ⚙️ What It Does

* 📄 Registers exams with cryptographic hashes of questions
* 🧾 Accepts secure student submissions using hashed answers
* 🚫 Prevents duplicate submissions automatically
* 🔍 Enables verification of exam integrity without revealing actual content
* 📦 Stores all records permanently on-chain

---

## 🚀 Key Features

* 🔐 **Tamper-Proof Data** – Once stored, exam data cannot be altered
* 👨‍🎓 **Single Submission Enforcement** – One attempt per student
* 🌐 **Decentralized System** – No central authority required
* 📊 **Transparent Records** – Fully auditable on blockchain
* ⚡ **High Efficiency** – Powered by Soroban smart contracts

---

## 🧩 How It Works

1. **Exam Creation**
   Instructor uploads exam → system stores hashed questions

2. **Student Submission**
   Student submits hashed answers → stored on-chain

3. **Validation**
   Smart contract checks for duplicate submissions

4. **Verification**
   Anyone can verify integrity using hashes without exposing content

---

## 🔗 Deployed Smart Contract

**Contract Address:**
`CBIBWAQLSZHNSLVHLHIND6UDFZM256QEWNQ6C5CNCEZSVRSBZATEQQBK`

---

## 🛠 Tech Stack

* **Rust** (Soroban SDK)
* **Stellar Blockchain**
* **Soroban Smart Contracts**

---

## 📦 Project Structure

```
├── contracts/
│   └── exam_integrity.rs
├── README.md
└── Cargo.toml
```

---

## 🚧 Future Enhancements

* 🔑 Wallet-based authentication (Stellar wallet integration)
* ⏳ Time-bound exams with automatic locking
* 🤖 AI-powered answer validation for objective tests
* 🏫 Admin dashboard for institutions
* 📱 Frontend UI (React + Web3 integration)

---

## 🤝 Contributing

Contributions are welcome!
Feel free to fork this repository, open issues, or submit pull requests to improve the project.

---

## 📄 License

This project is licensed under the **MIT License**.

---

## 💡 Vision

To build a **trustless, transparent, and secure examination ecosystem** where integrity is guaranteed by design—not enforced by authority.
