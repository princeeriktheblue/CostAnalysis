use std::{env, fs::{read_to_string, write, File}, io::Write};

use eframe::{egui, NativeOptions};
use egui::Ui;
use egui_extras::{Column, TableBuilder};
use jzon::JsonValue;

mod objects;

use crate::objects::entry::Entry;
use crate::objects::fields::Fields;

const BED_KEY: &str = "beds";
const BATH_KEY: &str = "baths";
const DEPOSIT_KEY: &str = "deposit";
const PET_DEPOSIT_KEY: &str = "petdeposit";
const PET_MONTHLY_KEY: &str = "petmonthly";
const PARKING_MONTHLY_KEY: &str = "parkingmonthly";
const MONTHLY_RENT_KEY: &str = "monthlyrent";
const LINK_KEY: &str = "link";
const RENT_DATA_KEY: &str = "rentdata";
const PROPERTIES_KEY: &str = "properties";

const PET_COUNT_KEY: &str = "petcount";
const ZOOM: f32 = 1.5;

fn main() {
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0,480.0]),
        ..Default::default()
    };
    let result = eframe::run_native(
        "Rental Cost Tracker",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    );
    if result.is_err() {
        println!("Error: {}", result.err().unwrap());
    }
}

fn fetch_i8(data: &JsonValue, key: &str) -> Option<i8> {
    let data_key = data.get(key);
    if data_key.is_none() { return None; }

    data_key.unwrap().as_i8()
}

fn fetch_f32(data: &JsonValue, key: &str) -> Option<f32> {
    let data_key = data.get(key);
    if data_key.is_none() { return None; }

    data_key.unwrap().as_f32()
}

fn build(name: &str, data: &JsonValue) -> Option<Entry> {
    let beds = fetch_i8(data, BED_KEY);
    if beds.is_none() { return None; }

    let baths = fetch_i8(data, BATH_KEY);
    if baths.is_none()  { return None; }

    let deposit = fetch_f32(data, DEPOSIT_KEY);
    if deposit.is_none()  { return None; }

    let pet_deposit = fetch_f32(data, PET_DEPOSIT_KEY);
    if pet_deposit.is_none()  { return None; }

    let pet_monthly= fetch_f32(data, PET_MONTHLY_KEY);
    if pet_monthly.is_none()  { return None; }

    let parking_monthly= fetch_f32(data, PARKING_MONTHLY_KEY);
    if parking_monthly.is_none()  { return None; }

    let monthly_rent= fetch_f32(data, MONTHLY_RENT_KEY);
    if monthly_rent.is_none()  { return None; }

    let link = data.get(LINK_KEY);
    if link.is_none() { return None; }

    Some(Entry::new(
        String::from(name), 
        data.get(BED_KEY).unwrap().as_i8().unwrap(), 
        data.get(BATH_KEY).unwrap().as_i8().unwrap(), 
        data.get(DEPOSIT_KEY).unwrap().as_f32().unwrap(), 
        data.get(PET_DEPOSIT_KEY).unwrap().as_f32().unwrap(), 
        data.get(PET_MONTHLY_KEY).unwrap().as_f32().unwrap(), 
        data.get(PARKING_MONTHLY_KEY).unwrap().as_f32().unwrap(), 
        data.get(MONTHLY_RENT_KEY).unwrap().as_f32().unwrap(), 
        data.get(LINK_KEY).unwrap().to_string()))
}

fn save(data: &Entry) -> Result<JsonValue, &str> {
    let mut entry_value = JsonValue::new_object();

    if entry_value.insert(BED_KEY, data.get_i8(Fields::Beds)).is_err() { return Err("Failed to save bed count!"); }
    if entry_value.insert(BATH_KEY, data.get_i8(Fields::Baths)).is_err() { return Err("Failed to save baths count!"); }
    if entry_value.insert(DEPOSIT_KEY, data.get_f32(Fields::Deposit)).is_err() { return Err("Failed to save deposit!"); }
    if entry_value.insert(PET_DEPOSIT_KEY, data.get_f32(Fields::PetDeposit)).is_err() { return Err("Failed to save pet deposit!"); }
    if entry_value.insert(PET_MONTHLY_KEY, data.get_f32(Fields::PetMonthly)).is_err() { return Err("Failed to save pet monthly!"); }
    if entry_value.insert(PARKING_MONTHLY_KEY, data.get_f32(Fields::ParkingMonthly)).is_err() { return Err("Failed to save parking monthly!"); }
    if entry_value.insert(MONTHLY_RENT_KEY, data.get_f32(Fields::MonthlyRent)).is_err() { return Err("Failed to save monthly rent!"); }
    if entry_value.insert(LINK_KEY, data.get_link()).is_err() { return Err("Failed to save the link!"); }

    Ok(entry_value)
}

struct MyApp {
    sortorder: Fields,
    list: Vec<Entry>,
    write_flag: bool,
    read_flag: bool,
    pet_count : i8,
}

impl Default for MyApp {
    fn default() -> Self {
        let mut obj = Self {
            sortorder: Fields::Name,
            list: Vec::new(),
            write_flag: false,
            read_flag: true,
            pet_count: 2,
        };

        obj.read();

        obj
    }
}

impl MyApp {
    fn name_unique(&mut self, new_name: String) -> bool {
        for entry in self.list.iter_mut() {
            if entry.get_name().eq_ignore_ascii_case(&new_name) {
                return false;
            }
        }
        true
    }

    fn remove(&mut self, name: String) {
        let mut index: usize = 0;
        loop {
            if self.list[index].is(&name) {
                self.list.remove(index);
                self.write_flag = true;
                break;
            }
            
            index = index + 1;
            self.write_flag = true;
            if index >= self.list.len() { return; }
        }
    }

    fn update_name(&mut self, old : String, new : String) {
        for entry in self.list.iter_mut() {
            if entry.is(&old) {
                entry.set_name(new);
                self.write_flag = true;
                break;
            }
        } 
    }

    fn update_link(&mut self, name: String, link: String) {
        for entry in self.list.iter_mut() {
            if entry.is(&name) {
                entry.set_link(link);
                self.write_flag = true;
                break;
            }
        } 
    }

    fn update_i8(&mut self, name: String, field: Fields, new_value: i8) {
        let mut index: usize = 0;
        loop {
            if self.list[index].is(&name) {
                self.list[index].set_i8(field, new_value);
                self.write_flag = true;
                break;
            }
            index = index + 1;
            if index >= self.list.len() { return; }
        }
    }

    fn update_f32(&mut self, name: String, field: Fields, new_value: f32) {
        let mut index: usize = 0;
        loop {
            if self.list[index].is(&name) {
                self.list[index].set_f32(field, new_value);
                self.list[index].calculate(self.pet_count);
                self.write_flag = true;
                break;
            }
            index = index + 1;
            if index >= self.list.len() { return; }
        }
    }

    fn show_error(&mut self, message: String) {
        println!("Error: {}", message);
    }

    fn build_i8_field(&mut self, name: &String, field: Fields, entry: &Entry, ui: &mut Ui) {
        let original_count = entry.get_i8(field).unwrap();
        let mut value_string = original_count.to_string();
        if original_count == 0 {
            value_string = String::new();
        }
        let response = ui.add(egui::TextEdit::singleline(&mut value_string));

        if response.lost_focus() || response.changed() {
            if value_string.is_empty() { 
                self.update_i8(name.clone(), field, 0); 
            } 
            else {
                match value_string.parse::<i8>() {
                    Ok(value) => {
                        if original_count != value {
                            self.update_i8(name.clone(), field, value);
                        }
                    },
                    Err(_) => {self.show_error(String::from("Invalid data for bed count"));},
                }
            }
        }
    }

    fn build_f32_field(&mut self, name: &String, field: Fields, entry: &Entry, ui: &mut Ui) {
        let original_count = entry.get_f32(field).unwrap();
        let mut value_string = original_count.to_string();
        if original_count == 0.0 {
            value_string = String::new();
        }
        let response = ui.add(egui::TextEdit::singleline(&mut value_string));

        if response.lost_focus() || response.changed() {
            if value_string.is_empty() {
                self.update_f32(name.clone(), field, 0.0);
            }
            else {
                match value_string.parse::<f32>() {
                    Ok(value) => {
                        if original_count != value {
                            self.update_f32(name.clone(), field, value);
                        }
                    },
                    Err(_) => {self.show_error(String::from("Invalid data for bed count"));},
                }
            }
        }
    }

    fn load_list(&mut self) -> Vec<Entry> {
        if self.write_flag { self.write() }

        if self.read_flag { self.read(); }
        
        let mut clone: Vec<Entry> = Vec::new();

        for entry in &self.list {
            clone.push(entry.clone());
        }        

        clone.sort_by(|a,b| { a.cmp(b, self.sortorder)});

        clone
    }

    fn insert_new_entry(&mut self) {
        let mut new_entry = Entry::default();
        let mut len = self.list.len();
        loop {
            let mut new_name = String::from("New_");
            new_name.push_str(len.to_string().as_str());
            if self.name_unique(new_name.clone()) {
                new_entry.set_name(new_name);
                break;
            }
            else {
                len = len + 1;
            }

            if len > 100 {
                let mut new_name = String::from("New_");
                new_name.push_str(len.to_string().as_str());
                new_entry.set_name(new_name);
            }
        }

        self.list.push(new_entry);
        self.write_flag = true;
    }

    fn recalculate(&mut self) {
        for entry in self.list.iter_mut() {
            entry.calculate(self.pet_count);
        }
    }

    fn write(&mut self) {
        self.write_flag = false;

        let mut properties = JsonValue::new_object();
        properties[PET_COUNT_KEY] = JsonValue::from(self.pet_count);

        let mut saveable = JsonValue::new_object();
        for entry in &self.list {
            match save(&entry) {
                Ok(data) => {
                    match saveable.insert(&entry.get_name(), data) {
                        Ok(_) => {},
                        Err(error) => {println!("Failed to push save file. {}", error)},
                    }
                },
                Err(error) => {
                    println!("Failed to parse into save file. {}", error)
                }
            }
        }

        let mut total = JsonValue::new_object();
        total[PROPERTIES_KEY] = properties;
        total[RENT_DATA_KEY] = saveable;
    
        let str_data = total.dump();
        let data = str_data.as_bytes();
    
        let mut data_path = env::current_dir().unwrap();
        data_path.push("rentdata");
        data_path.set_extension("json");
    
        let data_path_literal = data_path.as_path();
    
        if !data_path.exists() {
            let mut file = File::create(data_path_literal).unwrap();
            file.write_all(data).unwrap();
        }
        else {
            if !data_path.exists() { println!("Error in opening the file!"); return; }
    
            let write_result = write(data_path_literal, data);
            if write_result.is_err() {
                println!("Failed to write to file! {}", write_result.unwrap_err());
            }
        }
    }

    fn read(&mut self) {
        self.read_flag = false;
        let mut new_list: Vec<Entry> = Vec::new();

        let mut data_path = env::current_dir().unwrap();
        data_path.push("rentdata");
        data_path.set_extension("json");

        let data_path_literal = data_path.as_path();

        if !data_path.exists() {
            let mut file = File::create(data_path_literal).unwrap();
            file.write_all(String::from("{}").as_bytes()).unwrap();
        }

        //println!("Opening!");
        let data = jzon::parse(read_to_string(data_path_literal).unwrap().as_str()).unwrap();

        if !data.is_empty() {
            match &data[PROPERTIES_KEY][PET_COUNT_KEY].as_i8() {
                Some(value) => { self.pet_count = value.clone(); },
                None => { println!("Failed to parse pet count from file"); }
            }

            let rental_data = &data[RENT_DATA_KEY];
            for (name, data ) in rental_data.entries() {
                match build(name, data) {
                    Some(entry) => { new_list.push(entry);},
                    None => { println!("Failed to build from data!") }
                }
            }
        }
        self.list = new_list; 
        self.recalculate();
    }
}

impl eframe::App for MyApp {
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.write();
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.set_pixels_per_point(ZOOM);

            //ctx.set_fonts(fonts);

            ui.horizontal(|ui| {
                ui.label("Pet count: ");

                let mut pet_str = self.pet_count.to_string();
                let response = ui.text_edit_singleline(&mut pet_str);
                if response.changed() || response.lost_focus() {
                    if pet_str.is_empty() {
                        self.pet_count = 0;
                    }
                    else {
                        match pet_str.parse::<i8>() {
                            Ok(new) => {
                                if new != self.pet_count {
                                    self.pet_count = new;
                                    self.recalculate();
                                }
                            },
                            Err(_) => {},
                        }
                    }
                }
            });

            if ui.button("Add Entry").clicked() {
                self.insert_new_entry();
                //Create a popup to fill in the data!
            }
            TableBuilder::new(ui)
                .column(Column::auto().resizable(true))
                .column(Column::auto().resizable(true))
                .column(Column::auto().resizable(true))
                .column(Column::auto().resizable(true))
                .column(Column::auto().resizable(true))
                .column(Column::auto().resizable(true))
                .column(Column::auto().resizable(true))
                .column(Column::auto().resizable(true))
                .column(Column::auto().resizable(true))
                .column(Column::auto().resizable(true))
                .column(Column::auto().resizable(true))
                .column(Column::auto().resizable(true))
                .column(Column::auto().resizable(true))
                .column(Column::auto().resizable(true))
                .column(Column::remainder())
                .header(30.0, |mut header| {
                    header.col(|ui| {
                        if ui.button("Name").clicked() {
                            self.sortorder = Fields::Name;
                        };
                    });
                    header.col(|ui| {
                        if ui.button("Number of Beds").clicked() {
                            self.sortorder = Fields::Beds;
                        };
                    });
                    header.col(|ui| {
                        if ui.button("Number of Baths").clicked() {
                            self.sortorder = Fields::Baths;
                        };
                    });
                    header.col(|ui| {
                        if ui.button("Deposit").clicked() {
                            self.sortorder = Fields::Deposit;
                        };
                    });
                    header.col(|ui| {
                        if ui.button("Pet Deposit").clicked() {
                            self.sortorder = Fields::PetDeposit;
                        };
                    });
                    header.col(|ui| {
                        if ui.button("Pet Monthly").clicked() {
                            self.sortorder = Fields::PetMonthly;
                        };
                    });
                    header.col(|ui| {
                        if ui.button("Parking Monthly").clicked() {
                            self.sortorder = Fields::ParkingMonthly;
                        };
                    });
                    header.col(|ui| {
                        if ui.button("Monthly Rent").clicked() {
                            self.sortorder = Fields::MonthlyRent;
                        };
                    });
                    header.col(|ui| {
                        if ui.button("Total Rent").clicked() {
                            self.sortorder = Fields::TotalRent;
                        };
                    });
                    header.col(|ui| {
                        if ui.button("Rent for 2").clicked() {
                            self.sortorder = Fields::RentFor2;
                        };
                    });
                    header.col(|ui| {
                        if ui.button("Rent for 3").clicked() {
                            self.sortorder = Fields::RentFor3;
                        };
                    });
                    header.col(|ui| {
                        if ui.button("Rent for 4").clicked() {
                            self.sortorder = Fields::RentFor4;
                        };
                    });
                    header.col(|ui| {
                        ui.heading("Link");
                    });
                    header.col(|ui| {
                        ui.heading("Delete");
                    });
                })
                .body(|mut body| {
                    let list = self.load_list();
                    for entry in &list {
                        let cloned = entry.clone();
                        let name = entry.get_name();
                        body.row(30.0, |mut row: egui_extras::TableRow<'_, '_>| {
                            row.col(|ui| {
                                let mut name = String::from(&cloned.get_name());
                                let response = ui.add(egui::TextEdit::singleline(&mut name));
                                if response.changed() || response.lost_focus() {
                                    if self.name_unique(name.clone()) && !&cloned.is(&name) {
                                        self.update_name(entry.get_name().clone(), name);
                                    }
                                }
                            });
                            row.col(|ui: &mut egui::Ui| {
                                self.build_i8_field(&name, Fields::Beds, entry, ui);
                            });
                            row.col(|ui: &mut egui::Ui| {
                                self.build_i8_field(&name, Fields::Baths, entry, ui);
                            });
                            row.col(|ui: &mut egui::Ui| {
                                self.build_f32_field(&name, Fields::Deposit, entry, ui);
                            });
                            row.col(|ui: &mut egui::Ui| {
                                self.build_f32_field(&name, Fields::PetDeposit, entry, ui);
                            });
                            row.col(|ui: &mut egui::Ui| {
                                self.build_f32_field(&name, Fields::PetMonthly, entry, ui);
                            });
                            row.col(|ui: &mut egui::Ui| {
                                self.build_f32_field(&name, Fields::ParkingMonthly, entry, ui);
                            });
                            row.col(|ui: &mut egui::Ui| {
                                self.build_f32_field(&name, Fields::MonthlyRent, entry, ui);
                            });
                            row.col(|ui: &mut egui::Ui| {
                                ui.add(egui::Label::new(entry.get_f32(Fields::TotalRent).unwrap().to_string()));
                            });
                            row.col(|ui: &mut egui::Ui| {
                                ui.add(egui::Label::new(entry.get_f32(Fields::RentFor2).unwrap().to_string()));
                            });
                            row.col(|ui: &mut egui::Ui| {
                                ui.add(egui::Label::new(entry.get_f32(Fields::RentFor3).unwrap().to_string()));
                            });
                            row.col(|ui: &mut egui::Ui| {
                                ui.add(egui::Label::new(entry.get_f32(Fields::RentFor4).unwrap().to_string()));
                            });
                            row.col(|ui: &mut egui::Ui| {
                                let mut link = String::from(&cloned.get_link());
                                let response = ui.add(egui::TextEdit::singleline(&mut link));
                                if response.changed() || response.lost_focus() {
                                    if !&cloned.get_link().eq_ignore_ascii_case(&link) {
                                        self.update_link(name.clone(), link);
                                    }
                                }
                            });
                            row.col(|ui: &mut egui::Ui| {
                                let response = ui.add(egui::Button::new("Delete"));
                                if response.clicked() {
                                    self.remove(name.clone());
                                }
                            });
                        });
                    }
                });
        });
    }
}