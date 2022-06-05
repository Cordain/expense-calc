mod utils;
mod expense;
mod currency;

use std::{collections::{HashSet, HashMap}, rc::Rc};
use expense::Expense;
use currency::get_currency_factor;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use js_sys::Promise;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

type RcString = Rc<String>;


#[wasm_bindgen]
#[derive(Debug)]
pub struct Calculator{
    people: HashSet<RcString>,
    currency_handle: currency::CurrencyHandle,
    currencies: HashSet<RcString>,
    expenses: Vec<Expense>,
    result: Option<String>
}

#[wasm_bindgen]
impl Calculator{

    pub fn new() -> Calculator{
        Calculator { 
            expenses: (Vec::new()), 
            currencies: (HashSet::new()), 
            people: (HashSet::new()), 
            result: (None),
            currency_handle: (currency::CurrencyHandle::new())
        }
    }

    // Add expense and return its reference as index
    pub fn add_expense(&mut self, buyer: String, amount: f32, currency: String, buyer_included: bool, owers: Box<[JsValue]>) -> bool{
        let buyer: RcString = Rc::new(buyer);
        self.people.insert(buyer.clone());

        let currency: RcString = Rc::new(currency);
        self.currencies.insert(currency.clone());

        let mut expense = Expense::new(buyer.clone(), amount,currency.clone(),buyer_included);

        for ower in owers.to_vec().iter(){
            match ower.as_string(){
                Some(o) => {
                    let ower = Rc::new(o);
                    match expense.add_ower(ower.clone()){
                        true => {
                            self.people.insert(ower.clone());
                        }
                        false => {return false;}
                    }
                }
                None => { return false;}
            }
        }

        self.expenses.push(expense);
        true
    }

    pub fn add_ower_to_expense(&mut self, ower: String, expense_idx: usize) -> bool{
        match self.expenses.get_mut(expense_idx){
            Some(expense) => {
                let ower: RcString = Rc::new(ower);
                self.people.insert(ower.clone());
                expense.add_ower(ower.clone())
            }
            None => false
        }
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

    pub fn get_result(&self) -> String{
        match &self.result{
            Some(report) => report.to_owned(),
            None => {
                alert("Calculation went unsuccesfully");
                String::from("DATA INVALID")
            }
        }
    }

    pub fn get_currencies(&self) -> String{
        self.currency_handle.get_currency_map()
    }

    pub fn confirm_currency(&self, currency: String) -> Option<String> {
        self.currency_handle.is_currency_in_list(currency)
    }
    //https://github.com/rustwasm/wasm-bindgen/issues/1858
    #[wasm_bindgen]
    pub fn calculate(&self, target_currency: String) -> Promise{
        let people = self.people.clone();
        let expenses = self.expenses.clone();
        let currencies = self.currencies.clone();

        future_to_promise(async move {
            //init cost matrix
            let mut cost_matrix: HashMap<RcString, HashMap<RcString,f32>>;
            cost_matrix = HashMap::new();
            for name in people.iter(){
                let mut cost_row: HashMap<RcString,f32> = HashMap::new();
                for name in people.iter(){
                    cost_row.insert(name.clone(), 0f32);
                }

                cost_matrix.insert(name.clone(), cost_row);
            }

            //init currency conversion factors
            let currency_factors = {
                let mut currency_factors: HashMap<RcString,f32> = HashMap::new();
                for currency in currencies.iter(){
                    let currency_factor = get_currency_factor(currency.to_owned(),&target_currency).await;
                    currency_factors.insert(currency.to_owned(),currency_factor);
                }
                currency_factors
            };

            //calculate cost matrix
            for expense in expenses{
                let currency_factor = *currency_factors.get(&expense.get_currency()).unwrap_or(&f32::NAN);
                let expense_report = expense.calculate(currency_factor);
                let buyer = expense.get_buyer();
                let buyer_ref = match people.get(&buyer){
                    Some(idx) => {
                        Ok(idx)
                    }
                    None => {
                        alert("Could not find buyer.");
                        Err(JsValue::from(""))
                    }
                }?;
                for (ower, amount) in expense_report{
                    //get reference from names vec
                    let ower_ref = match people.get(&ower){
                        Some(idx) => {
                            Ok(idx)
                        }
                        None => {
                            alert("Could not find ower.");
                            Err(JsValue::from(""))
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
            for person in people.iter(){
                csv.push_str(person);
                csv.push(';');
            }
        
            for (buyer,owes) in cost_matrix.iter(){
                csv.push('\n');
                let mut amounts: Vec<String> = Vec::new();
                for name in people.iter(){
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
            Ok(JsValue::from(csv))
        })
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