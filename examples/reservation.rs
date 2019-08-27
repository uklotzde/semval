use semval::prelude::*;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct UnexpectedValue<T> {
    pub expected: T,
    pub actual: T,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Email(String);

impl Email {
    const fn min_len() -> usize {
        // a@b.c = 5 chars
        5
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum EmailInvalidity {
    MinLen(UnexpectedValue<usize>),
    Format,
}

impl Validate for Email {
    type Invalidity = EmailInvalidity;

    fn validate(&self) -> ValidationResult<Self::Invalidity> {
        let mut context = ValidationContext::valid();
        context.invalidate_if(
            self.0.len() < Self::min_len(),
            EmailInvalidity::MinLen(UnexpectedValue {
                expected: Self::min_len(),
                actual: self.0.len(),
            }),
        );
        context.invalidate_if(
            self.0.chars().filter(|c| *c == '@').count() != 1,
            EmailInvalidity::Format,
        );
        context.into()
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Phone(String);

impl Phone {
    const fn min_len() -> usize {
        6
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum PhoneInvalidity {
    MinLen(UnexpectedValue<usize>),
}

impl Validate for Phone {
    type Invalidity = PhoneInvalidity;

    fn validate(&self) -> ValidationResult<Self::Invalidity> {
        let mut context = ValidationContext::valid();
        let len = self.0.chars().filter(|c| !c.is_whitespace()).count();
        context.invalidate_if(
            len < Self::min_len(),
            PhoneInvalidity::MinLen(UnexpectedValue {
                expected: Self::min_len(),
                actual: len,
            }),
        );
        context.into()
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct ContactData {
    email: Option<Email>,
    phone: Option<Phone>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum ContactDataInvalidity {
    Phone(PhoneInvalidity),
    Email(EmailInvalidity),
    Incomplete,
}

impl Validate for ContactData {
    type Invalidity = ContactDataInvalidity;

    fn validate(&self) -> ValidationResult<Self::Invalidity> {
        let mut context = ValidationContext::valid();
        if let Some(ref email) = self.email {
            context.validate_and_map(email, ContactDataInvalidity::Email)
        }
        if let Some(ref phone) = self.phone {
            context.validate_and_map(phone, ContactDataInvalidity::Phone)
        }
        // Either email or phone must be present
        context.invalidate_if(
            self.email.is_none() && self.phone.is_none(),
            ContactDataInvalidity::Incomplete,
        );
        context.into()
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Customer {
    name: String,
    contact_data: ContactData,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum CustomerInvalidity {
    NameEmpty,
    ContactData(ContactDataInvalidity),
}

impl Validate for Customer {
    type Invalidity = CustomerInvalidity;

    fn validate(&self) -> ValidationResult<Self::Invalidity> {
        let mut context = ValidationContext::valid();
        context.invalidate_if(self.name.is_empty(), CustomerInvalidity::NameEmpty);
        context.validate_and_map(
            &self.contact_data,
            CustomerInvalidity::ContactData,
        );
        context.into()
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
struct Quantity(usize);

impl Quantity {
    const fn min() -> Self {
        Self(1)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum QuantityInvalidity {
    Min(UnexpectedValue<Quantity>),
}

impl Validate for Quantity {
    type Invalidity = QuantityInvalidity;

    fn validate(&self) -> ValidationResult<Self::Invalidity> {
        let mut context = ValidationContext::valid();
        context.invalidate_if(
            *self < Self::min(),
            QuantityInvalidity::Min(UnexpectedValue {
                expected: Self::min(),
                actual: *self,
            }),
        );
        context.into()
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Reservation {
    customer: Customer,
    quantity: Quantity,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum ReservationInvalidity {
    Customer(CustomerInvalidity),
    Quantity(QuantityInvalidity),
}

impl Validate for Reservation {
    type Invalidity = ReservationInvalidity;

    fn validate(&self) -> ValidationResult<Self::Invalidity> {
        let mut context = ValidationContext::valid();
        context.validate_and_map(&self.customer, ReservationInvalidity::Customer);
        context.validate_and_map(&self.quantity, ReservationInvalidity::Quantity);
        context.into()
    }
}

fn main() {
    let mut reservation = Reservation::default();
    println!("{:?}: {:?}", reservation, reservation.validate());

    reservation.customer.contact_data.email = Some(Email("a@b@c".to_string()));
    println!("{:?}: {:?}", reservation, reservation.validate());

    reservation.customer.name = "Mr X".to_string();
    reservation.customer.contact_data.phone = Some(Phone("0 123".to_string()));
    reservation.customer.contact_data.email = None;
    reservation.quantity = Quantity(4);
    println!("{:?}: {:?}", reservation, reservation.validate());

    reservation.customer.contact_data.phone = None;
    reservation.customer.contact_data.email = Some(Email("a@b.c".to_string()));
    println!("{:?}: {:?}", reservation, reservation.validate());
}
