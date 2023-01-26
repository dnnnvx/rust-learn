use crate::{
    model::{CrtShEntry, Subdomain},
    Error,
};
use reqwest::blocking::Client;
use std::{collections::HashSet, time::Duration};
use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    Resolver,
};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn enumerate(http_client &Client, target: &str) -> Result<Vec<Subdomain>>{
    
}