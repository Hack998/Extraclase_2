use std::fmt::Debug;
use std::marker::PhantomData;

#[derive(Debug, Default)]
pub struct Yes;
#[derive(Debug, Default)]
pub struct No;

pub trait ToAssign: Debug {}
pub trait Assigned: ToAssign {}
pub trait NotAssigned: ToAssign {}

impl ToAssign for Yes {}
impl ToAssign for No {}

impl Assigned for Yes {}
impl NotAssigned for No {}

pub fn cook_pasta(
        pasta_type: String,
pasta_name: Option<String>,
pasta_length: u64,
altitude: u64,
water_type: Option<String>,
) {
// your code here
println!(
"Cocinando pasta! -> {:?}, {:?}, {:?}, {:?}, {:?}",
pasta_type, pasta_name, pasta_length, altitude, water_type
);
}

#[derive(Debug, Clone, Default)]
pub struct CookPastaBuilder<PASTA_TYPE_SET, PASTA_LENGTH_SET, ALTITUDE_SET>
        where
PASTA_TYPE_SET: ToAssign,
PASTA_LENGTH_SET: ToAssign,
ALTITUDE_SET: ToAssign,
{
pasta_type_set: PhantomData<PASTA_TYPE_SET>,
pasta_length_set: PhantomData<PASTA_LENGTH_SET>,
altitude_set: PhantomData<ALTITUDE_SET>,

pasta_type: String,
pasta_name: Option<String>,
pasta_length: u64,
altitude: u64,
water_type: Option<String>,
}

impl<PASTA_TYPE_SET, PASTA_LENGTH_SET, ALTITUDE_SET>
        CookPastaBuilder<PASTA_TYPE_SET, PASTA_LENGTH_SET, ALTITUDE_SET>
where
        PASTA_TYPE_SET: ToAssign,
PASTA_LENGTH_SET: ToAssign,
ALTITUDE_SET: ToAssign,
{
pub fn with_pasta_type(
        self,
        pasta_type: String,
) -> CookPastaBuilder<Yes, PASTA_LENGTH_SET, ALTITUDE_SET> {
CookPastaBuilder {
pasta_type_set: PhantomData {},
pasta_length_set: PhantomData {},
altitude_set: PhantomData {},
pasta_type,
pasta_name: self.pasta_name,
pasta_length: self.pasta_length,
altitude: self.altitude,
water_type: self.water_type,
}
}

pub fn with_pasta_name(
        self,
        pasta_name: String,
) -> CookPastaBuilder<PASTA_TYPE_SET, PASTA_LENGTH_SET, ALTITUDE_SET> {
CookPastaBuilder {
pasta_type_set: PhantomData {},
pasta_length_set: PhantomData {},
altitude_set: PhantomData {},
pasta_type: self.pasta_type,
pasta_name: Some(pasta_name),
        pasta_length: self.pasta_length,
altitude: self.altitude,
water_type: self.water_type,
}
}

pub fn with_pasta_length(
        self,
        pasta_length: u64,
) -> CookPastaBuilder<PASTA_TYPE_SET, Yes, ALTITUDE_SET> {
CookPastaBuilder {
pasta_type_set: PhantomData {},
pasta_length_set: PhantomData {},
altitude_set: PhantomData {},
pasta_type: self.pasta_type,
pasta_name: self.pasta_name,
pasta_length,
altitude: self.altitude,
water_type: self.water_type,
}
}

pub fn with_altitude(
        self,
        altitude: u64,
) -> CookPastaBuilder<PASTA_TYPE_SET, PASTA_LENGTH_SET, Yes> {
CookPastaBuilder {
pasta_type_set: PhantomData {},
pasta_length_set: PhantomData {},
altitude_set: PhantomData {},
pasta_type: self.pasta_type,
pasta_name: self.pasta_name,
pasta_length: self.pasta_length,
altitude,
water_type: self.water_type,
}
}

pub fn with_water_type(
        self,
        water_type: String,
) -> CookPastaBuilder<PASTA_TYPE_SET, PASTA_LENGTH_SET, ALTITUDE_SET> {
CookPastaBuilder {
pasta_type_set: PhantomData {},
pasta_length_set: PhantomData {},
altitude_set: PhantomData {},
pasta_type: self.pasta_type,
pasta_name: self.pasta_name,
pasta_length: self.pasta_length,
altitude: self.altitude,
water_type: Some(water_type),
}
}
}

impl CookPastaBuilder<Yes, Yes, Yes> {
        pub fn execute(&self) {
            //  Código más amplio, en este caso un print solo para ejemplificar
            println!("Cocinando pasta! -> {:?}", self);
        }
}

pub fn cook_pasta2() -> CookPastaBuilder<No, No, No> {
CookPastaBuilder::default()
}

fn main() {
    cook_pasta("Lasaña".to_owned(), None, 100, 300, Some("Con aceite y sal".to_owned()));

    cook_pasta2()
            .with_pasta_type("Lasaña".to_owned())
            .with_pasta_length(100)
            .with_water_type("Con aceite y sal".to_owned())
            .with_altitude(300)
            .execute();
}

