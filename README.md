# Passive

 <span style="color:red;">Warning:</span> This tool was developed for educational purposes only.


## Instructions

[Install Rust](https://www.rust-lang.org/tools/install)

Clone repo and run 
```
cargo build
```    

## Usage
```
 cargo run --release -- --<flag> <input> 
```
```
fn
Full name: This flag is for searching people based on their full name. 
This function works only for people residing in the United States.
```
```
ip
IP: This flag is for searching IP addresses. 
This function is designed to check the location of a given IP address.

```
```
u
Username: This flag is for checking if a username exists on 
GitHub, TikTok, Codecademy, Reddit, and Instagram.

```

#### Example
``` rs
cargo run --release -- --u Mainm0e

Searching username: Mainm0e
github: yes
tiktok: no
codecademy: no
reddit: no
instagram: yes

```


## What is OSINT?
- Open Source Intelligence (OSINT) is data collected from publicly available sources to be used in an intelligence context. In the intelligence community, the term "open" refers to overt, publicly available sources (as opposed to covert or clandestine sources). It involves collecting information from online sources like social media, forums, websites, blogs, etc.




## Educational Purpose

- This tool is intended for educational purposes only. It is designed to demonstrate the principles of passive reconnaissance and OSINT, and to raise awareness about the privacy, security, ethical, and legal considerations associated with using such tools.
