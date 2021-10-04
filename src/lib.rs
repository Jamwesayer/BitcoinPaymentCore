#[macro_use]
extern crate diesel;
extern crate dotenv;

extern crate qrcode_generator;

pub mod db_models;
pub mod schema;

pub mod business {
    pub mod irepository;
    pub mod model;
    pub mod usecase;
}

pub mod presentation {
    pub mod controller;
    pub mod controller_service;
    pub mod item;
}

pub mod data {
    pub mod entity;
    pub mod idatasource;
    pub mod repository;
    pub mod blockchain_util;
    pub mod database_util;
}
