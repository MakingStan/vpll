use std::fs;
use std::thread::current;
use json::JsonValue;

pub fn convert_vpll_to_json(code: String) -> JsonValue
{
    let mut json_value = JsonValue::new_object();
    let lines = code.split("\n");

    json_value["elements"] = JsonValue::new_array();

    let config_file_contents = fs::read_to_string("config.json").unwrap();
    let mut config_json = json::parse(config_file_contents.as_str()).unwrap();

    /*
        Add all of the permanent json types
    */
    for i in config_json["permanent_types"].members()
    {
        let index_value = i.as_str().unwrap();
        json_value[index_value] = config_json[index_value].clone();
    }

    for (current_line, line) in lines.enumerate()
    {
        //Split the line with spaces
        let line_splitted = line.split(" ").collect::<Vec<&str>>();

        if line != "" {
            if !line.starts_with("//")
            {
                let element = line_splitted[0];

                if config_json[element].is_null()
                {
                    //it is null, notify the user of it
                    println!("🟨 WARNING: Element called \"{}\" could not be found in the config file. It has been ignored. 🟨", element);

                    /*
                        Handle for loops, sleeps etc..
                     */
                }
                else
                {
                    if config_json["permanent_types"].contains(element)
                    {
                        //handle the permanent types
                        let mut json_element = JsonValue::new_object();
                        let json_type = line_splitted[0];

                        //Give the element all of the required fields
                        json_element = config_json[json_type].clone().into();

                        for attribute_index in (1..line_splitted.len()-1).rev().step_by(2)
                        {
                            let int_value = line_splitted[attribute_index + 1];

                            json_element[line_splitted[attribute_index]] = int_value.into();
                        }

                        json_value[json_type] = json_element;
                    }
                    else
                    {
                        let mut json_element = JsonValue::new_object();
                        let json_type = line_splitted[0];
                        json_element["type"] = json_type.into();

                        //Give the element all of the required fields
                        json_element = config_json[json_type].clone().into();

                        //update the fields
                        for attribute_index in (1..line_splitted.len()-1).rev().step_by(2)
                        {
                            let int_value = line_splitted[attribute_index + 1];

                            json_element[line_splitted[attribute_index]] = int_value.into();
                        }
                        json_value["elements"].push(json_element).expect("Could not push element to the elements json array.");
                    }

                }
            }
            //else it's a blank line or a comment, so we don't want to interfere with it
        }
    }


    return json_value;
}