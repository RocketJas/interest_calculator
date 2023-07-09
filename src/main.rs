use chrono::{Duration, NaiveDate};
use std::collections::BTreeMap;
use std::io::{self, Write};
use anyhow::{anyhow, Error};

#[derive(Clone, Debug)]
struct Loan {
    start_date: NaiveDate,
    end_date: NaiveDate,
    loan_amount: f64,
    loan_currency: String,
    base_interest_rate: f64,
    margin: f64,
    total_interest: f64,
    // This could be a vector but we may want to access daily information by date in the future.
    // BTreeMap is used as it is sorted by key and efficient for lookups.
    daily_information: BTreeMap<NaiveDate, Daily_Information>,
}

#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
struct Daily_Information {
    day_interest: f64,
    day_interest_no_margin: f64,
    days_elapsed: i64,
}

/// Create a method new() for the Loan struct that takes in no values and returns a Loan with default values.
impl Loan {
    fn new() -> Self {
        Loan {
            loan_amount: 1000.0,
            start_date: NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2020, 1, 5).unwrap(),
            loan_currency: "USD".to_string(),
            base_interest_rate: 0.05,
            margin: 0.01,
            total_interest: 0.0,
            daily_information: BTreeMap::new(),
        }
    }
    fn calculate_interest(&mut self) -> () {
        let days = self.end_date.signed_duration_since(self.start_date).num_days();
        let total_interest_rate = self.base_interest_rate + self.margin;
        let daily_interest_rate_no_margin = self.base_interest_rate / 365.0;
        let daily_interest_rate = total_interest_rate / 365.0;

        // This could be done more concisely but having it structured like this allows the interest to be changed to a more complex type in the future.
        for day in 1..days+1 {
            let current_date = self.start_date + Duration::days(day);
            let daily_interest_amount_no_margin = self.loan_amount * daily_interest_rate_no_margin;
            let daily_interest_amount = self.loan_amount * daily_interest_rate;
            let daily_information = Daily_Information {
                day_interest: daily_interest_amount,
                day_interest_no_margin: daily_interest_amount_no_margin,
                days_elapsed: day,
            };
            self.daily_information.insert(current_date, daily_information);
        }
        let total_interest: f64 = self.loan_amount * daily_interest_rate * days as f64;
        self.total_interest = total_interest;
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
}

fn main() -> Result<(), Error>{
    println!("Loan Interest Calculator");

    let mut calculator = LoanCalculator::new();

    loop {
        println!("------------------------");
        println!("1. Add Loan");
        println!("2. Update Loan");
        println!("3. Show Loan Information");
        println!("4. Show All Loans");
        println!("5. Exit");
        print!("Please enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        // quick error handling for non integer input
        let choice: u32 = choice.trim().parse::<u32>().or::<u32>(Ok(999)).unwrap();
        let result = match choice {
            1 => {
                add_loan(&mut calculator)
            }
            2 => {
                update_loan(&mut calculator)   
            }
            3 => {
                show_loan_information(&mut calculator)
            }
            4 => {
                show_all_loans(&mut calculator)
            }
            5 => {
                println!("Exiting...");
                break
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

fn show_all_loans(calculator: &mut LoanCalculator) -> Result<(), Error> {
    println!("All Loans:");
    for (loan_id, loan) in calculator.loans.iter() {
        println!("Loan ID: {}", loan_id);
        println!("{:#?}\n", loan);
    }
    Ok(())
}

fn update_loan(calculator: &mut LoanCalculator) -> Result<(), Error> {
    print!("Enter the Loan ID to update: ");
    io::stdout().flush().unwrap();
    let mut loan_id_input = String::new();
    io::stdin().read_line(&mut loan_id_input).unwrap();
    let loan_id: u32 = loan_id_input.trim().parse()?;
    if let Some(mut loan) = calculator.loans.get(&loan_id).cloned() {
        // reset total_interest to 0 so it can be recalculated
        loan.total_interest = 0.0;
        // reset daily_information to empty so it can be recalculated
        loan.daily_information = BTreeMap::new();
        loan = update_loan_parameters(loan)?;
        calculator.update_loan(loan_id, loan)?;
        let new_loan = calculator.loans.get_mut(&loan_id).unwrap();
        new_loan.calculate_interest();
        Ok(())
    } else {
        Err(anyhow!("Loan with ID {} not found.\n", loan_id))
    }
}

fn show_loan_information(calculator: &mut LoanCalculator) -> Result<(), Error>{
    print!("Enter the Loan ID: ");
    io::stdout().flush().unwrap();
    let mut loan_id_input = String::new();
    io::stdin().read_line(&mut loan_id_input).unwrap();
    let loan_id: u32 = loan_id_input.trim().parse()?;

    let loan = calculator.loans.get_mut(&loan_id).ok_or(anyhow!("Loan with ID {} not found.\n", loan_id))?;
    print_interest_results(loan.clone());
    Ok(())
}

fn add_loan(calculator: &mut LoanCalculator) -> Result<(), Error>{
    let loan = update_loan_parameters(Loan::new())?;
    let loan_id = calculator.add_loan(loan);
    let added_loan = calculator.loans.get_mut(&loan_id).unwrap();
    println!("Loan added with ID: {}\n", loan_id);
    added_loan.calculate_interest();
    Ok(())
}

fn print_interest_results(loan: Loan) {
    println!("Loan Interest Calculation Results");
    println!("--------------------------------");
    // printing could be prettier but this is just a demo
    // it is more important that the calculations are correct and we do not round too early
    println!("{:#?}\n", loan);
}

fn update_loan_parameters(mut loan: Loan) -> Result<Loan, Error> {
    println!("Update Loan Parameters");
    println!("----------------------");

    print!("Start Date (YYYY-MM-DD): ");
    io::stdout().flush().unwrap();
    let mut start_date = String::new();
    io::stdin().read_line(&mut start_date).unwrap();
    loan.start_date = NaiveDate::parse_from_str(&start_date.trim(), "%Y-%m-%d")?;

    print!("End Date (YYYY-MM-DD): ");
    io::stdout().flush().unwrap();
    let mut end_date = String::new();
    io::stdin().read_line(&mut end_date).unwrap();
    loan.end_date = NaiveDate::parse_from_str(&end_date.trim(), "%Y-%m-%d")?;

    print!("Loan Amount: ");
    io::stdout().flush().unwrap();
    let mut loan_amount = String::new();
    io::stdin().read_line(&mut loan_amount).unwrap();
    loan.loan_amount = loan_amount.trim().parse()?;

    print!("Loan Currency: ");
    io::stdout().flush().unwrap();
    let mut loan_currency = String::new();
    io::stdin().read_line(&mut loan_currency).unwrap();
    loan.loan_currency = loan_currency.trim().to_string();

    print!("Base Interest Rate (%): ");
    io::stdout().flush().unwrap();
    let mut base_interest_rate = String::new();
    io::stdin().read_line(&mut base_interest_rate).unwrap();
    // divide by 100 to convert to %
    loan.base_interest_rate = base_interest_rate.trim().parse::<f64>()?/100.0;

    print!("Margin (%): ");
    io::stdout().flush().unwrap();
    let mut margin = String::new();
    io::stdin().read_line(&mut margin).unwrap();
    // divide by 100 to convert to %
    loan.margin = margin.trim().parse::<f64>()?/100.0;

    println!("Loan parameters updated successfully!\n");

    Ok(loan)
}