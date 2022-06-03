import init, { Calculator } from "./pkg/expense_calc.js";
let wasm = await init();

//get website elements
const expenses_list = document.getElementById("expenses_list");
const add_expense = document.getElementById("add_expense");
const calculate = document.getElementById("calculate");
const clear_data = document.getElementById("clear_data");
const buyer = document.getElementById("buyer");
const amount = document.getElementById("amount");
const owers = document.getElementById("owers");
const table = document.getElementById("report");

let ext_calculator;
init_site()

add_expense.addEventListener("click", event => {
    const buyer_name = buyer.value.match(/[\p{L}a-zA-Z]+(?: [\p{L}a-zA-Z]+)?/u);
    const amount_val = amount.value.match(/\d*\.?\d*/);
    const owers_names = owers.value.match(/[\p{L}a-zA-Z]+(?: [\p{L}a-zA-Z]+)?/gu);
    const owers_count = (owers.value.match(/,/g) || []).length;
    if((buyer_name !== null && amount_val !== null && owers_names !== null) && (buyer_name[0] === buyer.value) && (amount_val[0] === amount.value) && (owers_names.length-1 === owers_count)){
        const idx = ext_calculator.add_expense(buyer_name[0],amount_val[0]);

        owers_names.forEach(name => {
            if (false == ext_calculator.add_ower_to_expense(name,idx)) {
                alert("ERROR adding "+name);
                ext_calculator.revert_expense();
            }
        });
        expenses_list.textContent = ext_calculator.print_expenses();
    }
    else{
        alert("Please fill the form");
    }
})

calculate.addEventListener("click", event => {
    const report = document.createElement("tbody");
    table.innerHTML = "";

    const calc_report = ext_calculator.calculate();

    
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
})

clear_data.addEventListener("click", event =>{
    init_site();
    alert("Site has been cleared!");
})

function init_site(){
    ext_calculator = Calculator.new();
    expenses_list.textContent = ext_calculator.print_expenses();
    buyer.value = "";
    amount.value = "";
    owers.value = "";
    table.innerHTML = "";
}