# Freshes pro
**Language:** Rust  
**Basic action:** Send POST requests to a form submission endpoint 10,000 times, with randomly generated data.

## How to use
1. Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed on your machine
2. Run `cargo build --release` to build the app & run the executable, **or** run the app in one command:
```sh
cargo run
```

### Scam reference
In summer 2024, many Israeli people received this scam message via SMS:

> לקוח יקר,
>
> המשכנתא שברשותך צפויה להתייקר בקרוב.  
> בדיקה ללא עלות זמינה עבורך !  
>
> לפרטים : https://example.com/  
>
> הסרה : https://example.com/delete/  
>
> *ההודעה מנוסחת בלשון זכר אך מיועדת לשני המינים.

> Replace `example.com` with `freshes.pro`

The website in freshes.pro contains a form with two visible fields; name and phone number. When the user fill out the form and clicks the submit button, the data is sent to the server and the success/failure message is displayed in the page.  
The server is vulnerable to a simple POST request flood attack on the form submission endpoint.  

#### Why Rust?
No reason. Why not?
