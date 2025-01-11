use std::cmp::Ordering;
use crate::objects::fields::Fields;


#[derive(Clone)]
pub struct Entry {
    name: String,
    beds: i8,
    baths: i8,
    deposit: f32,
    pet_deposit: f32,
    pet_monthly: f32,
    parking_monthly: f32,
    monthly_rent: f32,
    total_rent: f32,
    rent_for_2: f32,
    rent_for_3: f32,
    rent_for_4: f32,
    link: String,
}

impl Default for Entry {
    fn default() -> Self {
        Entry::new(String::from("Enter name here"), 0, 0, 0.0, 0.0, 0.0, 0.0, 0.0, String::new())
    }
}

impl Entry {
    pub fn new(name: String,
        beds: i8,
        baths: i8, 
        deposit: f32, 
        pet_deposit: f32,
        pet_monthly: f32,
        parking_monthly: f32,
        monthly_rent: f32,
        link: String,) -> Self {
            let mut object = Entry {
                name,
                beds,
                baths,
                deposit,
                pet_deposit,
                pet_monthly,
                parking_monthly,
                monthly_rent,
                total_rent: 0.0,
                rent_for_2: 0.0,
                rent_for_3: 0.0,
                rent_for_4: 0.0,
                link
            };
            object.calculate(0);
            object
    }

    pub fn calculate(&mut self, pet_count: i8) {
        self.total_rent = self.monthly_rent + (self.pet_monthly * pet_count as f32) + self.parking_monthly;
        self.rent_for_2 = self.total_rent/2.0 + self.parking_monthly;
        self.rent_for_3 = self.total_rent/3.0 + self.parking_monthly * 2.0;
        self.rent_for_4 = self.total_rent/4.0 + self.parking_monthly * 3.0;
    }

    pub fn cmp(&self, other: &Entry, sort_field : Fields) -> Ordering {
        match sort_field {
            Fields::Name => {
                let result = self.name.to_ascii_lowercase().cmp(&other.name.to_ascii_lowercase());
                return result;
            },
            Fields::Beds => { return self.beds.cmp(&other.beds); },
            Fields::Baths => { return self.baths.cmp(&other.baths); },
            Fields::Deposit => { return self.deposit.total_cmp(&other.deposit); },
            Fields::PetDeposit => { return self.pet_deposit.total_cmp(&other.pet_deposit); },
            Fields::PetMonthly => { return self.pet_monthly.total_cmp(&other.pet_monthly); },
            Fields::ParkingMonthly => { return self.parking_monthly.total_cmp(&other.parking_monthly); },
            Fields::MonthlyRent => { return self.monthly_rent.total_cmp(&other.monthly_rent); },
            Fields::TotalRent => { return self.total_rent.total_cmp(&other.total_rent); },
            Fields::RentFor2 => { return self.rent_for_2.total_cmp(&other.rent_for_2); },
            Fields::RentFor3 => { return self.rent_for_3.total_cmp(&other.rent_for_3); },
            Fields::RentFor4 => { return self.rent_for_4.total_cmp(&other.rent_for_4); },
        }
    }

    pub fn get_name(&self) -> String {
        return self.name.clone();
    }

    pub fn get_link(&self) -> String {
        return self.link.clone();
    }

    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }

    pub fn set_link(&mut self, new_link: String) {
        self.link = new_link;
    }

    pub fn is(&self, name: &String) -> bool {
        return self.name.to_ascii_lowercase().cmp(&name.to_ascii_lowercase()).is_eq();
    }

    pub fn get_i8(&self, field: Fields) -> Option<i8> {
        match field {
            Fields::Name => { None },
            Fields::Beds => { Some(self.beds) },
            Fields::Baths => { Some(self.baths) },
            Fields::Deposit => { None },
            Fields::PetDeposit => { None },
            Fields::PetMonthly => { None },
            Fields::ParkingMonthly => { None },
            Fields::MonthlyRent => { None },
            Fields::TotalRent => { None },
            Fields::RentFor2 => { None },
            Fields::RentFor3 => { None },
            Fields::RentFor4 => { None },
        }
    }

    pub fn get_f32(&self, field: Fields) -> Option<f32> {
        match field {
            Fields::Name => { None},
            Fields::Beds => { None },
            Fields::Baths => { None },
            Fields::Deposit => { Some(self.deposit) },
            Fields::PetDeposit => { Some(self.pet_deposit) },
            Fields::PetMonthly => { Some(self.pet_monthly) },
            Fields::ParkingMonthly => { Some(self.parking_monthly) },
            Fields::MonthlyRent => { Some(self.monthly_rent) },
            Fields::TotalRent => { Some(self.total_rent) },
            Fields::RentFor2 => { Some(self.rent_for_2) },
            Fields::RentFor3 => { Some(self.rent_for_3) },
            Fields::RentFor4 => { Some(self.rent_for_4) },
        }
    }

    pub fn set_i8(&mut self, field: Fields, new_value: i8) {
        match field {
            Fields::Name => {},
            Fields::Beds => { self.beds = new_value; },
            Fields::Baths => { self.baths = new_value; },
            Fields::Deposit => {},
            Fields::PetDeposit => {},
            Fields::PetMonthly => {},
            Fields::ParkingMonthly => {},
            Fields::MonthlyRent => {},
            Fields::TotalRent => {},
            Fields::RentFor2 => {},
            Fields::RentFor3 => {},
            Fields::RentFor4 => {},
        }
    }

    pub fn set_f32(&mut self, field: Fields, new_value: f32) {
        match field {
            Fields::Name => {},
            Fields::Beds => {},
            Fields::Baths => {},
            Fields::Deposit => {self.deposit = new_value; },
            Fields::PetDeposit => {self.pet_deposit = new_value;},
            Fields::PetMonthly => {self.pet_monthly = new_value;},
            Fields::ParkingMonthly => {self.parking_monthly = new_value;},
            Fields::MonthlyRent => {self.monthly_rent = new_value;},
            Fields::TotalRent => {},
            Fields::RentFor2 => {},
            Fields::RentFor3 => {},
            Fields::RentFor4 => {},
        }
    }
}