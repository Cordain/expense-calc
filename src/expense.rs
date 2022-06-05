
use super::RcString;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Expense {
    buyer: RcString,
    amount: f32,
    currency: RcString,
    buyer_included: bool,
    owers: Vec<RcString>
}
impl Clone for Expense{
    fn clone(&self) -> Self {
        Self { buyer: self.buyer.clone(), amount: self.amount.clone(), currency: self.currency.clone(), buyer_included: self.buyer_included.clone(), owers: self.owers.clone() }
    }
}

impl Expense{
    pub fn new(buyer: RcString, amount: f32, currency: RcString, buyer_included: bool) -> Expense{
        Expense { buyer: (buyer), amount: (amount), currency: (currency), buyer_included: (buyer_included), owers: (Vec::new()) }
    }

    pub fn add_ower(&mut self, ower: RcString) -> bool{
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
        format!("{} has to receive {:.2} {} from {}",
            self.buyer,
            self.amount,
            self.currency,
            owers_str
        )
    }

    pub fn get_buyer(&self) -> RcString{
        self.buyer.clone()
    }

    pub fn get_currency(&self) -> RcString{
        self.currency.clone()
    }

    pub fn calculate(&self, currency_factor: f32) -> HashMap<RcString,f32>{
        let mut report: HashMap<RcString,f32> = HashMap::new();
        let divided_amount = match self.buyer_included{
            false => self.amount/self.owers.len() as f32,
            true => self.amount/(self.owers.len()+1) as f32
        };
        for ower in self.owers.iter(){
            report.insert(ower.to_owned(), divided_amount*currency_factor);
        }
        report
    }
}