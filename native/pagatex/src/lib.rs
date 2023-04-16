use rustler::{Error, NifStruct};

#[derive(NifStruct)]
#[module = "Pagatex.Payment"]
struct Payment {
    pub from: String,
    pub to: Vec<String>,
    pub amount: i32,
}

#[derive(NifStruct)]
#[module = "Pagatex.Obligation"]
struct Obligation {
    pub from: String,
    pub to: String,
    pub amount: i32,
}

#[rustler::nif]
fn who_pays_whom(payments: Vec<Payment>) -> Result<Vec<Obligation>, Error> {
    use pagat::{Money, Payment, Payments, Person};

    let mut payments_builder = Payments::builder();

    payments.iter().for_each(|payment| {
        let to = payment.to
            .iter()
            .map(|to| Person::new(to)).collect::<Vec<Person>>();

        let payment = Payment::builder()
            .from(Person::new(&payment.from))
            .amount(Money::new(payment.amount))
            .to(&to)
            .build();

        payments_builder.record(payment);
    });

    match payments_builder.build().who_pays_whom() {
        Ok(wpw) => Ok(wpw.raw().iter().map(|obligation| Obligation {
            from: obligation.from.raw().to_string(),
            to: obligation.to.raw().to_string(),
            amount: obligation.amount.raw(),
        }).collect()),
        Err(ref e) => Err(Error::Term(Box::new(e.to_string()))),
    }
}

rustler::init!("Elixir.Pagatex", [who_pays_whom]);
