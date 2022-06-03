mod utils;

use std::{collections::{HashSet, HashMap}, rc::Rc};

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Debug)]
pub struct Expense {
    buyer: Rc<String>,
    amount: f32,
    buyer_included: bool,
    owers: Vec<Rc<String>>
}

impl Expense{
    pub fn new(buyer: Rc<String>, amount: f32, buyer_included: bool) -> Expense{
        Expense { buyer: (buyer), amount: (amount), buyer_included: (buyer_included), owers: (Vec::new()) }
    }

    pub fn add_ower(&mut self, ower: Rc<String>) -> bool{
        match self.buyer == ower{
            true => false,
            false =>{
                self.owers.push(ower);
                true
            }
        }
    }

    pub fn print_me(&self) -> String{
        let owers_str = 
            match self.owers.len(){
                1 => String::from(match self.owers.get(0){
                    Some(ower) => ower,
                    None => ""
                }), 
                els => {
                    let mut owers_str = String::from(match self.owers.get(0){
                        Some(ower) => ower,
                        None => ""
                    });
                    match self.owers.get(1..els-1){
                        Some(inner_owers) => for ower in inner_owers.into_iter(){
                            owers_str.push_str(", ");
                            owers_str.push_str(ower);
                        }
                        None => {}
                    }
                    owers_str.push_str(" and ");
                    owers_str.push_str(match self.owers.get(els-1){
                        Some(ower) => ower,
                        None => ""
                    });
                    owers_str
                }
            };
        format!("{} has to receive {:.2} from {}",
            self.buyer,
            self.amount,
            owers_str
        )
    }

    pub fn get_buyer(&self) -> Rc<String>{
        self.buyer.clone()
    }

    pub fn calculate(&self) -> HashMap<Rc<String>,f32>{
        let mut report: HashMap<Rc<String>,f32> = HashMap::new();
        let divided_amount = match self.buyer_included{
            false => self.amount/self.owers.len() as f32,
            true => self.amount/(self.owers.len()+1) as f32
        };
        for ower in self.owers.iter(){
            report.insert(ower.to_owned(), divided_amount);
        }
        report
    }
}

#[wasm_bindgen]
#[allow(dead_code)]
pub struct Calculator{
    people: HashSet<Rc<String>>,
    expenses: Vec<Expense>    
}

#[wasm_bindgen]
#[allow(dead_code)]
impl Calculator{
    pub fn new() -> Calculator{
        Calculator { expenses: (Vec::new()), people: (HashSet::new()) }
    }

    // Add expense and return its reference as index
    pub fn add_expense(&mut self, buyer: String, amount: f32, buyer_included: bool) -> usize{
        let buyer: Rc<String> = Rc::new(buyer);
        self.people.insert(buyer.clone());
        self.expenses.push(Expense::new(buyer.clone(), amount,buyer_included));
        self.expenses.len() -1
    }

    pub fn add_ower_to_expense(&mut self, ower: String, expense_idx: usize) -> bool{
        match self.expenses.get_mut(expense_idx){
            Some(expense) => {
                let ower: Rc<String> = Rc::new(ower);
                self.people.insert(ower.clone());
                expense.add_ower(ower.clone())
            }
            None => false
        }
    }

    pub fn revert_expense(&mut self){
        self.expenses.pop();
    }

    pub fn print_expenses(&self) -> String{
        let mut printer = String::from("Current Expenses:");
        for expense in self.expenses.iter(){
            let expense_prt = expense.print_me();
            printer.push('\n');
            printer.push_str(&expense_prt);
        };
        printer
    }

    pub fn calculate(&self) -> Option<String>{
        //init cost matrix
        let mut cost_matrix: HashMap<Rc<String>, HashMap<Rc<String>,f32>>;
        cost_matrix = HashMap::new();
        for name in self.people.iter(){
            let mut cost_row: HashMap<Rc<String>,f32> = HashMap::new();
            for name in self.people.iter(){
                cost_row.insert(name.clone(), 0f32);
            }

            cost_matrix.insert(name.clone(), cost_row);
        }
        //calculate cost matrix
        for expense in self.expenses.iter(){
            let expense_report = expense.calculate();
            let buyer = expense.get_buyer();
            let buyer_ref = match self.people.get(&buyer){
                Some(idx) => {
                    Some(idx)
                }
                None => {
                    alert("Could not find buyer.");
                    None
                }
            }?;
            for (ower, amount) in expense_report{
                //get reference from names vec
                let ower_ref = match self.people.get(&ower){
                    Some(idx) => {
                        Some(idx)
                    }
                    None => {
                        alert("Could not find ower.");
                        None
                    }
                }?;
                
                match cost_matrix.get_mut(buyer_ref){
                    Some(buyer) => {
                        match buyer.get_mut(ower_ref){
                            Some(ower) => {
                                *ower += amount;
                            }
                            None => {
                                alert("Problem getting owe in cost matrix.");
                            }
                        }
                    }
                    None => {
                        alert("Problem getting buyer in cost matrix.");
                    }
                };
            }
        }
        //stringify cost matrix to csv
        let mut csv: String = format!("Buyer\\Ower;");
        for person in self.people.iter(){
            csv.push_str(person);
            csv.push(';');
        }
        
        for (buyer,owes) in cost_matrix.iter(){
            csv.push('\n');
            let mut amounts: Vec<String> = Vec::new();
            for name in self.people.iter(){
                amounts.push(match owes.get(name){
                    Some(owe) => format!("{:.2}",owe),
                    None => {
                        alert("Error: no owe in cost matrix!");
                        String::from("0.00")
                    }
                });
            };
            csv.push_str(buyer);
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