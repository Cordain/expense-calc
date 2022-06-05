import init, { Calculator } from "./pkg/expense_calc.js";
let wasm = await init();

//get website elements
const expenses_list = document.getElementById("expenses_list");
const add_expense = document.getElementById("add_expense");
const calculate = document.getElementById("calculate");
const clear_data = document.getElementById("clear_data");
const buyer = document.getElementById("buyer");
const is_included = document.getElementById("included");
const amount = document.getElementById("amount");
const owers = document.getElementById("owers");
const table = document.getElementById("report");
const expense_currency = document.getElementById("expense_currency");
const target_currency = document.getElementById("target_currency");

var ext_calculator;
init_site();

add_expense.addEventListener("click", event => {
    const buyer_name = buyer.value.match(/[\p{L}a-zA-Z]+(?: [\p{L}a-zA-Z]+)?/u);
    const amount_val = amount.value.match(/\d*\.?\d*/);
    const owers_names = owers.value.match(/[\p{L}a-zA-Z]+(?: [\p{L}a-zA-Z]+)?/gu);
    const owers_count = (owers.value.match(/,/g) || []).length;
    const currency = ext_calculator.confirm_currency(expense_currency.value);
    if(
        buyer_name !== null && 
        amount_val !== null && 
        owers_names !== null && 
        buyer_name[0] === buyer.value && 
        amount_val[0] === amount.value && 
        owers_names.length-1 === owers_count &&
        currency !== null
    ){
        if(false == ext_calculator.add_expense(buyer_name[0], amount_val[0], currency, is_included.checked, owers_names)){
            alert("ERROR adding expense, check name duplicates!");
        }

        expenses_list.textContent = ext_calculator.print_expenses();
    }
    else{
        alert("Please fill the form");
    }
})

calculate.addEventListener("click", calc_on_click)

async function calc_on_click(event){
    const report = document.createElement("tbody");
    const currency = ext_calculator.confirm_currency(target_currency.value);
    table.innerHTML = "";

    const calc_report = await ext_calculator.calculate(currency);
    
    
    if(calc_report !== undefined){
        const first_line_end = calc_report.indexOf('\n');
        const trh = document.createElement("tr");
        calc_report.substring(0,first_line_end).split(";").forEach(column_name => {
            const th = document.createElement("th");
            th.innerHTML = column_name;
            trh.appendChild(th);
        })
        report.appendChild(trh);


        const owe_report = calc_report.substring(first_line_end+1);
        owe_report.split("\n").forEach(row => {
            const tr = document.createElement("tr");
            row.split(";").forEach(cell => {
                const td = document.createElement("td");
                td.innerHTML = cell;
                tr.appendChild(td);
            })
            report.appendChild(tr);
        })
        table.appendChild(report);
    }
}

clear_data.addEventListener("click", event =>{
    init_site();
    alert("Site has been cleared!");
})

function init_site(){
    ext_calculator = Calculator.new();
    const currencies = JSON.parse(ext_calculator.get_currencies());
    expenses_list.textContent = ext_calculator.print_expenses();
    buyer.value = "";
    is_included.checked = true;
    amount.value = "";
    owers.value = "";
    table.innerHTML = "";
    expense_currency.innerHTML = "";
    target_currency.innerHTML = "";

    for (let currency in currencies){
        var expense_curr = document.createElement('option');
        var target_curr = document.createElement('option');

        expense_curr.value = currency;
        expense_curr.innerHTML = currencies[currency];
        target_curr.value = currency;
        target_curr.innerHTML = currencies[currency];

        expense_currency.appendChild(expense_curr);
        target_currency.appendChild(target_curr);
    }
    return ext_calculator;
}