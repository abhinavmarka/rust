enum Action {
    Deposit,
    Withdraw,
}

struct Account {
    name: String,
    accountBalance: f32,
    active: bool,
    action: Action,
}
fn show_result(account: &Account) {
    println!("Name: {}", account.name);
    println!("Account balance: {}", account.accountBalance);
    println!("Active Status: {}", account.active);

    match account.action {
        Action::Deposit => println!("--150000 was depodited into account\n--updated balance: 1150000"),
        Action::Withdraw => println!("--100000 was withdrawn from account\n--updated balance: 900000"),
    }
}



fn main() {
    let Account1 = Account {
        name: String::from("charan"),
        accountBalance: 1000000.0,
        active: true,
        action: Action::Deposit,
    };
    let Account2 = Account {
        name: String::from("charan"),
        accountBalance: 1000000.0,
        active: true,
        action: Action::Withdraw,
    };
   
    show_result(&Account1);
    println!("------");
    show_result(&Account2);
}