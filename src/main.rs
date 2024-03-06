
mod items;
mod cli;
mod file;

fn main() {

    let (method, arg1, arg2) = cli::start();
    match method.as_str() {

        "help" => {
            println!("here is the list of command you can do!\n");
            println!("FI (Find Item) will try to get the item that you typed, example: TMMX FI \"Volcano\"");
            println!("FWT (Find With Type) will try to get all the items with that type (have to be in caps), example: TMMX FWT \"BASIC\"");
            println!("FITV (Find Item Total Value) will try to get item value (have to be in caps), example: TMMX FITV \"Volcano\" 10");
        },

        "FI" => {
            let maybe_item = items::get_item_info(arg1.as_ref().unwrap().as_str());
            if maybe_item.is_some() {
                println!("The value is: {:#?}", maybe_item.unwrap());
            } else {
                println!("could not find: {:#?}", arg1.as_ref().unwrap())
            }
        },

        "FITV" => {
            let maybe_total_value = items::get_item_total_value(
                arg1.as_ref().unwrap().as_str(),
                arg2.unwrap().parse().expect("need an number")
            );
            if maybe_total_value.is_some() {
                println!("Total value is {}", maybe_total_value.unwrap())
            } else {
                println!("Could not find {}", arg1.unwrap())
            }

        }

        "FWT" => {
            let maybe_all_items = items::get_all_items_with(arg1.as_ref().unwrap().clone());

            if maybe_all_items.is_some() {

                println!("Item's with the tag {}", arg1.as_ref().unwrap());

                for item in maybe_all_items.unwrap().iter() {
                    println!("{:#?}", *item);
                }

            } else {

                println!("could not find with type: {:#?}", arg1.unwrap());

            }

        }

        _ => println!("{} is not a valid command! if you need help type TMMX help.", method)
    }

}
