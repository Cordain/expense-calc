import init, { Calculator } from "./pkg/expense_calc.js";
let wasm = await init();

//get website elements
const expenses_list = document.getElementById("expenses_list");
const add_expense = document.getElementById("add_expense");
const calculate = document.getElementById("calculate");
const buyer = document.getElementById("buyer");
const amount = document.getElementById("amount");
const owers = document.getElementById("owers");

const ext_calculator = Calculator.new();
init_site()

add_expense.addEventListener("click", event => {
    const buyer_name = buyer.value;
    const amount_val = amount.value;
    const owers_names = owers.value;
    if(buyer_name == "" || amount_val == "" || owers_names == ""){
        alert("Please fill the form");
    }
    else{
        const idx = ext_calculator.add_expense(buyer_name,amount_val);

        owers.value.split(new RegExp(", ?")).forEach(name => {
            if (false == ext_calculator.add_ower_to_expense(name,idx)) {
                alert("ERROR adding "+name);
            }
        });
        expenses_list.textContent = ext_calculator.print_expenses();
    }
})

calculate.addEventListener("click", event => {
    const table = document.getElementById("report");
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

function init_site(){
    expenses_list.textContent = ext_calculator.print_expenses();
    buyer.value = "";
    amount.value = "";
    owers.value = "";
}