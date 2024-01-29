struct Bank {
    corpus: u64,         // Fixed supply of money
    min_duration: u64,   
    max_duration: u64,   
    prepayment_fee: f64, // Pre-payment fee as a percentage
}

struct Loan {
    amount: u64,
    interest_rate: f64,
    duration: u64,
    remaining_balance: u64,
    installment_amount: u64,
}

impl Bank {
    const INIT_CORPUS: u64 = 1_000_000; // Initial corpus amount for the bank

    fn new() -> Self {
        Bank {
            corpus: Self::INIT_CORPUS,
            min_duration: 6, //  in months
            max_duration: 36, //  in months
            prepayment_fee: 0.02, // Pre-payment fee as 2%
        }
    }

    fn apply_for_loan(&mut self, amount: u64, duration: u64) -> Loan {
        // Calculate interest rate inversely proportional to the duration
        let interest_rate = 1.0 / duration as f64;

        // Calculate installment amount using simple interest formula
        let installment_amount = (amount as f64 * (1.0 + interest_rate)) / duration as f64;

        Loan {
            amount,
            interest_rate,
            duration,
            remaining_balance: amount,
            installment_amount: installment_amount as u64,
        }
    }

    fn process_installment(&mut self, loan: &mut Loan, payment: u64) {
        // Check if installment is late
        if payment < loan.installment_amount {
            let penalty = loan.installment_amount - payment;
            loan.remaining_balance += penalty;
        }

        // Reduce debt from principal and interest component
        let interest_payment = loan.remaining_balance as f64 * loan.interest_rate;
        loan.remaining_balance -= (loan.installment_amount as f64 - interest_payment) as u64;

        // Update bank's corpus
        self.corpus += payment;
    }

    fn process_prepayment(&mut self, loan: &mut Loan, prepayment_amount: u64) {
        // Calculate pre-payment fee
        let prepayment_fee = prepayment_amount as f64 * self.prepayment_fee;
        loan.remaining_balance -= prepayment_fee as u64;

        // Update bank's corpus
        self.corpus += prepayment_fee;
    }
}

fn main() {
    // Create a new bank
    let mut my_bank = Bank::new();

    // Apply for a loan
    let mut my_loan = my_bank.apply_for_loan(5000, 12);

    // Process installments and pre-payment
    my_bank.process_installment(&mut my_loan, 1000);
    my_bank.process_installment(&mut my_loan, 800);
    my_bank.process_prepayment(&mut my_loan, 2000);

    // Display remaining loan balance and bank's corpus
    println!("Remaining Loan Balance: {}", my_loan.remaining_balance);
    println!("Bank's Corpus: {}", my_bank.corpus);
}
