use slint::{include_modules, PlatformError, Weak};


include_modules!();

const TAXPER: f64 = 0.30;
const OWNERPER: f64 = 0.55;
const PROFITPER: f64 = 0.05;
const OPEXPER: f64 = 0.10;

fn main() -> Result<(), PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle: Weak<AppWindow> = ui.as_weak();

    ui.on_divide_income(move |string| {
        if let Some(ui) = ui_handle.upgrade() {
            let num: f64 = string.trim().parse().unwrap();
            let tax: f64 = num * TAXPER;
            let owner: f64 = num * OWNERPER;
            let profit: f64 = num * PROFITPER;
            let opex: f64 = num * OPEXPER;

            let result: String = format!(
                "Taxes: {:.2}\nOwner: {:.2}\nProfit: {:.2}\nOperating Expenses: {:.2}\n",
                tax, owner, profit, opex
            );
            ui.set_results(result.into());
        }
    });

    ui.run()
}
