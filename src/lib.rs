mod utils;

use std::{fmt::format, collections::{HashSet, HashMap}, hash::Hash};

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub struct Expense {
    buyer: String,
    amount: f32,
    owers: Vec<String>
}

impl Expense{
    pub fn new(buyer: String, amount: f32) -> Expense{
        Expense { buyer: (buyer), amount: (amount), owers: (Vec::new()) }
    }

    pub fn add_ower(&mut self, ower: String) -> bool{
        match self.buyer.contains(&ower){
            true => false,
            false =>{
                self.owers.push(ower);
                true
            }
        }
    }

    pub fn print_me(&self) -> String{
        //todo!("Format message with last name to be ' and ' separated."); 
        format!("{} has to receive {:.2} from {}",
            self.buyer,
            self.amount,
            self.owers.join(", ")
        )
    }

    pub fn get_names(&self) -> Vec<String>{
        let mut output = self.owers.clone();
        output.push(self.buyer.clone());
        output
    }

    pub fn get_buyer(&self) -> String{
        self.buyer.clone()
    }

    pub fn calculate(&self) -> HashMap<String,f32>{
        let mut report: HashMap<String,f32> = HashMap::new();
        for ower in self.owers.iter(){
            report.insert(ower.to_owned(), self.amount/self.owers.len() as f32);
        }
        report
    }
}

#[wasm_bindgen]
struct Calculator{
    expenses: Vec<Expense>    
}

#[wasm_bindgen]
impl Calculator{
    pub fn new() -> Calculator{
        Calculator { expenses: (Vec::new()) }
    }

    // Add expense and return its reference as index
    pub fn add_expense(&mut self, buyer: String, amount: f32) -> usize{
        self.expenses.push(Expense::new(buyer,amount));
        self.expenses.len() -1
    }

    pub fn add_ower_to_expense(&mut self, ower: String, expense_idx: usize) -> bool{
        match self.expenses.get_mut(expense_idx){
            Some(expense) => {
                expense.add_ower(ower)
            }
            None => false
        }
    }

    pub fn print_expenses(&self) -> String{
        let mut printer = String::from("Current Expenses:");
        for expense in self.expenses.iter(){
            printer.push('\n');
            printer.push_str(&expense.print_me());
        };
        printer
    }

    pub fn calculate(&self) -> Option<String>{
        //aquire names
        let names: Vec<String> = {
            let mut names: HashSet<String> = HashSet::new();
            for expense in self.expenses.iter(){
                let names_from_expense = expense.get_names();
                for name in names_from_expense.iter(){
                    names.insert(name.clone());
                }
            }
            names.into_iter().collect()
        };
        //init cost matrix
        let mut cost_matrix: HashMap<&String, HashMap<&String,f32>>;
        cost_matrix = HashMap::new();
        for name in names.iter(){
            let mut cost_row: HashMap<&String,f32> = HashMap::new();
            for name in names.iter(){
                cost_row.insert(name, 0f32);
            }

            cost_matrix.insert(name, cost_row);
        }
        //calculate cost matrix
        for expense in self.expenses.iter(){
            let expense_report = expense.calculate();
            let buyer_ref = match names.binary_search(&expense.get_buyer()){
                Ok(idx) => {
                    names.get(idx)
                }
                Err(_) => None
            }?;
            for (ower, amount) in expense_report{
                //get reference from names vec
                let ower_ref = match names.binary_search(&ower){
                    Ok(idx) => {
                        names.get(idx)
                    }
                    Err(_) => None
                }?;
                
                *cost_matrix.get_mut(buyer_ref)?.get_mut(ower_ref)? += amount;
            }
        }
        //stringify cost matrix to csv
        let mut csv: String = format!("Buyer\\Ower;{}",names.join(";"));
        
        for (buyer,owes) in cost_matrix.iter(){
            csv.push('\n');
            let mut amounts: Vec<String> = Vec::new();
            for name in names.iter(){
                amounts.push(owes.get(name)?.to_string());
            }
            csv.push_str(buyer.clone());
            csv.push(';');
            csv.push_str(&amounts.join(";"));
        }
        Some(csv)
    }
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, expense-calc!");
}