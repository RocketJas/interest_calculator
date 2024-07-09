use anyhow::{anyhow, Error};
use chrono::{NaiveDate};
use std::collections::BTreeMap;
use std::io::{self, Write};

#[derive(Clone, Debug)]
struct Loan {
    start_date: NaiveDate,
    end_date: NaiveDate,
    loan_amount: f64,
    loan_currency: String,
    base_interest_rate: f64,
    margin: f64,
    // figure out how to add id here and if it is the right thing to do
    // id: u32,
    // not including total_interest in the struct as it is calculated and not set by the user, meaning if another field is changed it could get out of sync
    // total_interest: Option<f64>,
}

/// Create a method new() for the Loan struct that takes in no values and returns a Loan with default values.
impl Loan {
    fn new(
        start_date: NaiveDate,
        end_date: NaiveDate,
        loan_amount: f64,
        loan_currency: String,
        base_interest_rate: f64,
        margin: f64,
        // id: u32,
    ) -> Self {
        Loan {
            start_date,
            end_date,
            loan_amount,
            loan_currency,
            base_interest_rate,
            margin,
            // id,
        }
    }
    fn calculate_interest(&self) -> f64 {
        let days = self
            .end_date
            .signed_duration_since(self.start_date)
            .num_days();
        let total_interest_rate = self.base_interest_rate + self.margin;
        let daily_interest_rate = total_interest_rate / 365.0;

        self.loan_amount * daily_interest_rate * days as f64
    }
    fn display(&self) {
        println!("Loan Information");
        println!("----------------");
        println!("Start Date: {}", self.start_date);
        println!("End Date: {}", self.end_date);
        println!("Loan Amount: {} {}", self.loan_amount, self.loan_currency);
        println!("Loan Currency: {}", self.loan_currency);
        println!("Base Interest Rate: {}%", self.base_interest_rate * 100.0);
        println!("Margin: {}%", self.margin * 100.0);
        println!(
            "Total Interest: {:.2} {}",
            self.calculate_interest(),
            self.loan_currency
        );
    }
        
}

#[derive(Debug)]
struct LoanCalculator {
    // BTreeMap is used as it is ordered by key and efficient for lookups.
    // HashMap could be used for faster lookups but it is unordered so we do not use it here.
    loans: BTreeMap<u32, Loan>,
    next_loan_id: u32,
}

impl LoanCalculator {
    fn new() -> Self {
        LoanCalculator {
            loans: BTreeMap::new(),
            next_loan_id: 1,
        }
    }

    fn add_loan(&mut self, loan: Loan) -> u32 {
        let loan_id = self.next_loan_id;
        self.loans.insert(loan_id, loan);
        self.next_loan_id += 1;
        loan_id
    }

    fn update_loan(&mut self, loan_id: u32, updated_loan: Loan) -> Result<(), Error> {
        if self.loans.contains_key(&loan_id) {
            self.loans.insert(loan_id, updated_loan);
            println!("Loan with ID {} updated successfully!\n", loan_id);
            Ok(())
        } else {
            Err(anyhow!("Loan with ID {} not found.\n", loan_id))
        }
    }
     
    fn display(&self) {
        println!("Loan IDs:");
        println!("----------------");
        for (loan_id, _) in self.loans.iter() {
            println!("Loan ID: {}", loan_id);
        }
    }
}

fn main() -> Result<(), Error> {
    println!("Loan Interest Calculator");

    let mut calculator = LoanCalculator::new();

    loop {
        println!("------------------------");
        println!("1. Add Loan");
        println!("2. Update Loan");
        println!("3. Show Loan Information");
        println!("4. Show All Loans");
        println!("5. Create Mock Data");
        println!("6. Exit");
        print!("Please enter your choice: ");

        let choice = get_input();
        // print 4 new lines
        println!("\n\n\n\n");
        // quick error handling for non integer input
        let choice: u32 = choice.trim().parse::<u32>().or::<u32>(Ok(999)).unwrap();
        let result = match choice {
            1 => add_loan(&mut calculator),
            2 => update_loan(&mut calculator),
            3 => show_loan_information(&mut calculator),
            4 => show_all_loans(&mut calculator),
            5 => {
                create_mock_data(&mut calculator)
            }
            6 => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("\nInvalid choice! Please enter an integer from 1-5.");
                Ok(())
            }
        };
        if let Err(e) = result {
            println!("Error: {}", e);
        }
    }
    Ok(())
}

fn create_mock_data(calculator: &mut LoanCalculator) -> Result<(), Error> {

    let loan1 = Loan::new(
        NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
        NaiveDate::from_ymd_opt(2020, 1, 31).unwrap(),
        1000.0,
        "USD".to_string(),
        0.05,
        0.01,
    );
    let loan2 = Loan::new(
        NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
        NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
        2000.0,
        "USD".to_string(),
        0.055,
        0.02,
    );

    let loan3 = Loan::new(
        NaiveDate::from_ymd_opt(2020, 4, 3).unwrap(),
        NaiveDate::from_ymd_opt(2020, 7, 21).unwrap(),
        10000.0,
        "USD".to_string(),
        0.10,
        0.01,
    );

    calculator.add_loan(loan1);
    calculator.add_loan(loan2);
    calculator.add_loan(loan3);

    show_all_loans(calculator)
}

fn show_all_loans(calculator: &mut LoanCalculator) -> Result<(), Error> {
    println!("All Loans:");
    // println!("{:?}", calculator);
    calculator.display();
    println!("\n\n\n\n");
    Ok(())
}

fn get_input() -> String {
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn update_loan(calculator: &mut LoanCalculator) -> Result<(), Error> {
    print!("Enter the Loan ID to update: ");
    let loan_id_input = get_input();

    let loan_id: u32 = loan_id_input.trim().parse()?;
    if let Some(mut loan) = calculator.loans.get(&loan_id).cloned() {
        // need to change this
        loan = create_loan()?;
        calculator.update_loan(loan_id, loan)?;
        let new_loan = calculator.loans.get_mut(&loan_id).unwrap();
        new_loan.calculate_interest();
        Ok(())
    } else {
        Err(anyhow!("Loan with ID {} not found.\n", loan_id))
    }
}

fn show_loan_with_id(calculator: &LoanCalculator, loan_id: &u32) -> Result<(), Error> {
    let loan = calculator
        .loans
        .get(&loan_id)
        .ok_or(anyhow!("Loan with ID {} not found.\n", loan_id))?;
    loan.display();
    Ok(())
}

fn show_loan_information(calculator: &mut LoanCalculator) -> Result<(), Error> {
    print!("Enter the Loan ID: ");
    let loan_id_input = get_input();
    // improve error message below, "could not get loan ID form input"
    let loan_id: u32 = loan_id_input.trim().parse()?;

    show_loan_with_id(calculator, &loan_id)?;
    Ok(())
}

fn add_loan(calculator: &mut LoanCalculator) -> Result<(), Error> {
    let loan = create_loan()?;
    let loan_id = calculator.add_loan(loan);
    let added_loan = calculator.loans.get_mut(&loan_id).unwrap();
    println!("Loan added with ID: {}\n", loan_id);
    added_loan.calculate_interest();
    Ok(())
}

/// Can improve this function to allow a single input to fail and retry not all inputs
fn create_loan() -> Result<Loan, Error> {

    print!("Start Date (YYYY-MM-DD): ");
    let start_date = NaiveDate::parse_from_str(get_input().as_str(), "%Y-%m-%d")?;

    print!("End Date (YYYY-MM-DD): ");
    let end_date = NaiveDate::parse_from_str(get_input().as_str(), "%Y-%m-%d")?;

    print!("Loan Amount: ");
    let loan_amount = get_input().trim().parse()?;

    print!("Loan Currency: ");
    let loan_currency = get_input().trim().parse()?;

    print!("Base Interest Rate (%): ");
    // divide by 100 to convert to %
    let base_interest_rate = get_input().trim().parse::<f64>()? / 100.0;

    print!("Margin (%): ");
    // divide by 100 to convert to %
    let margin = get_input().trim().parse::<f64>()? / 100.0;

    Ok(Loan::new(
        start_date,
        end_date,
        loan_amount,
        loan_currency,
        base_interest_rate,
        margin,
    ))
}
