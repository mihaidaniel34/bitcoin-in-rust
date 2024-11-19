use crate::core::Core;
use anyhow::Result;
use cursive::event::{Event, Key};
use cursive::traits::*;
use cursive::views::{
    Button, Dialog, EditView, LinearLayout, Panel, ResizedView, TextContent, TextView,
};
use cursive::Cursive;
use std::sync::{Arc, Mutex};
use tracing::*;
use tracing::field::debug;

#[derive(Clone, Copy)]
enum Unit {
    Btc,
    Sats,
}
/// Convert an amount between BTC and Satoshi units.
fn convert_amount(amount: f64, from: Unit, to: Unit) -> f64 {
    match (from, to) {
        (Unit::Btc, Unit::Sats) => amount * 100_000_000.0,
        (Unit::Sats, Unit::Btc) => amount / 100_000_000.0,
        _ => amount,
    }
}

pub fn run_ui(core: Arc<Core>, balance_content: TextContent) -> Result<()> {
    info!("Initializing UI");
    let mut siv = cursive::default();
    setup_siv(&mut siv, core.clone(), balance_content);
    info!("Starting UI event loop");
    siv.run();
    info!("UI event loop ended");
    Ok(())
}
fn setup_siv(siv: &mut Cursive, core: Arc<Core>, balance_content: TextContent) {
    siv.set_autorefresh(true);
    siv.set_window_title("BTC wallet");
    siv.add_global_callback('q', |s| {
        info!("Quit command received");
        s.quit()
    });
    setup_menubar(siv, core.clone());
    setup_layout(siv, core, balance_content);
    siv.add_global_callback(Event::Key(Key::Esc), |siv| {
        siv.select_menubar()
    });
    siv.select_menubar();
}

fn setup_menubar(siv: &mut Cursive, core: Arc<Core>) {
    siv.menubar()
        .add_leaf("Send", move |siv| {
            show_send_transaction(siv, core.clone());
        })
        .add_leaf("Quit", |siv| siv.quit());
    siv.set_autohide_menu(false);
}
fn setup_layout(siv: &mut Cursive, core: Arc<Core>, balance_content: TextContent) {
    let instruction = TextView::new("Press Escape to select the top menu");
    let balance_panel = Panel::new(TextView::new_with_content(balance_content))
        .title("Balance");
    let info_layout = create_info_layout(&core);
    let layout = LinearLayout::vertical()
        .child(instruction)
        .child(balance_panel)
        .child(info_layout);
    siv.add_layer(layout);
    
}
fn create_info_layout(core: &Arc<Core>) -> LinearLayout {
    let mut info_layout = LinearLayout::horizontal();
    let keys_content = core
        .config
        .my_keys
        .iter()
        .map(|key| format!("{}", key.private.display()))
        .collect::<Vec<String>>()
        .join("\n");
    info_layout.add_child(ResizedView::with_full_width(
        Panel::new(TextView::new(keys_content))
            .title("Your keys"),
    ));
    let contacts_content = core
        .config
        .contacts
        .iter()
        .map(|contact| contact.name.clone())
        .collect::<Vec<String>>()
        .join("\n");
    info_layout.add_child(ResizedView::with_full_width(
        Panel::new(TextView::new(contacts_content))
            .title("Contacts"),
    ));
    info_layout
}

fn show_send_transaction(siv: &mut Cursive, core: Arc<Core>) {
    info!("Showing send transaction dialog");
    let unit = Arc::new(Mutex::new(Unit::Btc));
    siv.add_layer(
        Dialog::around(create_transaction_layout(unit.clone()))
            .title("Send Transaction")
            .button("Send", move |siv| {
                send_transaction(siv, core.clone(), *unit.lock().unwrap());
            })
            .button("Cancel", |siv| {
                debug!("Transaction cancelled");
                siv.pop_layer();
            })
    )
    
}

fn create_transaction_layout(unit: Arc<Mutex<Unit>>) -> LinearLayout {
    LinearLayout::vertical()
        .child(TextView::new("Recipient"))
        .child(EditView::new().with_name("recipient"))
        .child(TextView::new("Amount"))
        .child(EditView::new().with_name("amount"))
        .child(create_unit_layout(unit))
}

fn create_unit_layout(unit: Arc<Mutex<Unit>>) -> LinearLayout {
    LinearLayout::horizontal()
        .child(TextView::new("Unit: "))
        .child(
            TextView::new_with_content(TextContent::new("BTC"))
                .with_name("unit_display")
        )
        .child(Button::new("Switch", move |siv| {
            switch_unit(siv, unit.clone());
        }))
}

fn switch_unit(siv: &mut Cursive, unit: Arc<Mutex<Unit>>) {
    let mut unit = unit.lock().unwrap();
    *unit = match *unit {
        Unit::Btc => Unit::Sats,
        Unit::Sats => Unit::Btc,
    };
    siv.call_on_name("unit_display", |view: &mut TextView| {
        view.set_content(match *unit {
            Unit::Btc => "BTC",
            Unit::Sats => "Sats",
        });
    });
}

fn send_transaction(siv: &mut Cursive, core: Arc<Core>, unit: Unit) {
    debug!("Send button pressed");
    let recipient = siv
        .call_on_name("recipient", |view: &mut EditView| view.get_content())
        .unwrap();
    let amount: f64 = siv
        .call_on_name("amount", |view: &mut EditView| view.get_content())
        .unwrap()
        .parse()
        .unwrap_or(0.0);
    let amount_sats = convert_amount(amount, unit, Unit::Sats) as u64;
    info!("Attempting to send transaction to {} for {} satoshis", recipient, amount_sats);
    match core
        .send_transaction_async(recipient.as_str(), amount_sats) {
        Ok(_) => show_success_dialog(siv),
        Err(e) => show_error_dialog(siv, e),
    }
    
}

fn show_success_dialog(siv: &mut Cursive) {
    info!("Transaction sent successfully");
    siv.add_layer(
        Dialog::text("Transaction sent successfully")
            .title("Success")
            .button("OK", |s| {
            debug("Closing success dialog");
            s.pop_layer();
            s.pop_layer();
        }),
    );
}

fn show_error_dialog(siv: &mut Cursive, error: impl std::fmt::Display) {
    error!("Failed to send transaction: {}", error);
    
    siv.add_layer(
        Dialog::text(format!("Transaction failed to send: {}", error))
            .title("Error")
            .button("OK", |s| {
                debug!("Closing error dialog");
                s.pop_layer();
            }),
    )
}
